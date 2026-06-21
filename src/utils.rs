use std::{
    error::Error,
    ffi::OsStr,
    fs,
    io::{self, Read, Write},
    path::Path,
    sync::{Arc, RwLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose, Engine as _};
use log::error;

use zip::ZipWriter;

use crate::{
    constant::{BACKUP_BLACK_LIST, GAME_SAVE_LOCAL_DIR, SAVE_CLOUD_DIR},
    ime::get_current_format_time,
    tai::{change_psv_account_id, get_psv_account_id},
    ui::ui_loading::Loading,
    vita2d::rgba,
};

pub fn current_time() -> u128 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

pub fn normalize_path(path: &str) -> String {
    let invalid_chars = ['\\', '/', ':', '*', '?', '"', '\'', '<', '>', '|'];
    let mut path = path.to_string();
    for c in invalid_chars.iter() {
        path = path.replace(*c, "_");
    }
    path.trim().to_string()
}

pub fn str_to_c_str(data: &str) -> Vec<u8> {
    format!("{}\0", data).into_bytes()
}

pub fn ease_out_expo(elapsed: Duration, duration: Duration, start: f32, end: f32) -> f32 {
    if elapsed >= duration {
        return end;
    }
    start
        + (end - start)
            * (1.0 - 2.0_f32.powf(-10.0 * elapsed.as_millis() as f32 / duration.as_millis() as f32))
}

pub fn get_active_color() -> u32 {
    let from = (168, 254, 255) as (i32, i32, i32);
    let to = (0, 168, 255) as (i32, i32, i32);
    let mut current = (0, 0, 0) as (i32, i32, i32);
    let p = (current_time() % 1000) as i32;

    if p < 400 {
        current.0 = from.0 + (to.0 - from.0) * p / 400;
        current.1 = from.1 + (to.1 - from.1) * p / 400;
        current.2 = from.2 + (to.2 - from.2) * p / 400;
    } else {
        current.0 = from.0 + (to.0 - from.0) * (1000 - p) / 600;
        current.1 = from.1 + (to.1 - from.1) * (1000 - p) / 600;
        current.2 = from.2 + (to.2 - from.2) * (1000 - p) / 600;
    }

    rgba(current.0, current.1, current.2, 0xff)
}

pub fn create_save_cloud_dir_if_not_exists() -> Result<(), Box<dyn Error>> {
    let path = Path::new(SAVE_CLOUD_DIR);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// # get game save list of local dir
pub fn get_local_game_saves(local_dir: String, items: Arc<RwLock<Vec<String>>>) {
    let game_save_dir = Path::new(&local_dir);
    let mut list = vec![];
    for entry in game_save_dir.read_dir().expect("read game save dir") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().unwrap().to_str().unwrap();
                if name.ends_with(".zip") {
                    list.push(name.to_string());
                }
            }
        }
    }
    list.sort_by(|a, b| b.cmp(&a));
    *items.write().expect("write game saves") = list;
}

pub fn zip_dir_with(
    zip: &mut ZipWriter<fs::File>,
    input_path: &Path,
    prefix: &str,
    back_list: &[&str],
) -> Result<(), Box<dyn Error>> {
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut buffer = vec![0; 1024 * 512];
    for entry in input_path.read_dir()? {
        if let Ok(entry) = entry {
            let path = entry.path();
            let name = path.strip_prefix(Path::new(prefix)).unwrap();
            if let Some(_) = back_list.iter().find(|&&x| x == name.to_str().unwrap()) {
                continue;
            }
            Loading::notify_desc(entry.file_name().to_string_lossy().to_string());
            // Write file or directory explicitly
            // Some unzip tools unzip files with directory paths correctly, some do not!
            if path.is_file() {
                #[allow(deprecated)]
                zip.start_file_from_path(name, options)?;
                let mut input_file = fs::File::open(path)?;
                loop {
                    let size = input_file.read(&mut buffer)?;
                    if size == 0 {
                        break;
                    }
                    zip.write_all(&buffer[0..size])?;
                }
            } else if !name.as_os_str().is_empty() {
                // Only if not root! Avoids path spec / warning
                // and mapname conversion failed error on unzip
                #[allow(deprecated)]
                zip.add_directory_from_path(name, options)?;
                zip_dir_with(zip, path.as_path(), prefix, back_list)?;
            }
        }
    }

    Ok(())
}

pub fn zip_dir(from: &str, to: &str, back_list: &[&str]) -> Result<(), Box<dyn Error>> {
    let from = if from.ends_with("/") {
        from.to_string()
    } else {
        format!("{}/", from)
    };
    let output_path = Path::new(to);
    if !output_path.parent().unwrap().exists() {
        fs::create_dir_all(output_path.parent().unwrap())?;
    }
    let mut zip = zip::ZipWriter::new(fs::File::create(output_path)?);
    zip_dir_with(&mut zip, Path::new(&from), &from, back_list)?;
    zip.finish()?;
    Ok(())
}

pub fn zip_file(from: &str, name: &str, to: &str) -> Result<(), Box<dyn Error>> {
    let from_path = Path::new(from).join(name);
    let mut zip = zip::ZipWriter::new(fs::File::create(to)?);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut buffer = vec![0; 1024 * 512];
    #[allow(deprecated)]
    zip.start_file_from_path(Path::new(name), options)?;
    let mut input_file = fs::File::open(from_path)?;
    loop {
        let size = input_file.read(&mut buffer)?;
        if size == 0 {
            break;
        }
        zip.write_all(&buffer[0..size])?;
    }
    zip.finish()?;
    Ok(())
}

pub fn zip_extract(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    back_list: Option<&[&str]>,
) -> Result<(), Box<dyn Error>> {
    let mut zip = zip::ZipArchive::new(fs::File::open(from)?)?;
    for i in 0..zip.len() {
        Loading::notify_title(format!("Extracting {}/{}", i + 1, zip.len()));
        let mut file_name = zip.by_index(i)?;
        let output_path = match file_name.enclosed_name() {
            Some(file_name) => {
                Loading::notify_desc(file_name.to_string_lossy().to_string());
                to.as_ref().join(file_name).to_owned()
            }
            None => continue,
        };

        if (*file_name.name()).ends_with('/') {
            if !output_path.exists() {
                fs::create_dir_all(&output_path)?;
            }
        } else {
            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            if back_list.is_some_and(|list| list.iter().find(|&&x| x == file_name.name()).is_some())
                && output_path.exists()
            {
                continue;
            }
            let mut output_file = fs::File::create(&output_path)?;
            io::copy(&mut file_name, &mut output_file)?;
        }
    }

    Ok(())
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<u64> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(0)
}

pub fn join_path(base: &str, path: &str) -> String {
    if base == "" || base.ends_with("/") {
        format!("{}{}", base, path)
    } else {
        format!("{}/{}", base, path)
    }
}

pub fn update_sfo_file_with_current_account_id(sfo_path: &str) -> Result<(), Box<dyn Error>> {
    if Path::new(&sfo_path).exists() {
        let account_id = get_psv_account_id();
        if account_id > 0 {
            if change_psv_account_id(&sfo_path, account_id) < 0 {
                let msg = format!("change psv account id failed: {}", account_id);
                error!("{}", msg);
                return Err(msg.into());
            }
        } else {
            error!("get psv account id failed");
            return Err("get psv account id failed".into());
        }
    }

    Ok(())
}

pub fn backup_game_save(from: &str, to: &str) -> Result<(), Box<dyn Error>> {
    zip_dir(from, to, &BACKUP_BLACK_LIST)
}

pub fn restore_game_save(from: &str, to: &str) -> Result<(), Box<dyn Error>> {
    if let Some(from_parent) = Path::new(from).parent() {
        if let Some(auto_backup_path) = from_parent
            .join(&format!("{} auto.zip", get_current_format_time()))
            .to_str()
        {
            Loading::notify_title("Auto-backing up...".to_string());
            let _ = backup_game_save(to, auto_backup_path);
        }
    }
    Loading::notify_title("Restoring save...".to_string());
    let mut res = zip_extract(from, to, Some(&BACKUP_BLACK_LIST));
    if res.is_ok() {
        let sfo_path = format!("{}/sce_sys/param.sfo", to);
        res = update_sfo_file_with_current_account_id(&sfo_path);
    }
    res
}

pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

pub fn base64_decode(data: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(general_purpose::STANDARD.decode(data)?)
}

pub fn get_str_md5(data: &[u8]) -> String {
    format!("{:x}", md5::compute(data))
}

pub fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(data);
    let hex: String = hash.iter().map(|b| format!("{:02x}", b)).collect();
    format!("sha256:{}", hex)
}

pub fn sha256_file(path: &str) -> Result<String, Box<dyn Error>> {
    let data = fs::read(path)?;
    Ok(sha256_hex(&data))
}

pub fn delete_dir_if_empty(path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() && path.read_dir()?.next().is_none() {
        fs::remove_dir(path)?;
    }
    Ok(())
}

pub fn get_game_local_backup_dir(title_id: &str, name: &str) -> String {
    let default_dir_path = format!(
        "{}/{} {}",
        GAME_SAVE_LOCAL_DIR,
        title_id,
        normalize_path(name)
    )
    .trim()
    .to_string();

    let path = Path::new(GAME_SAVE_LOCAL_DIR);
    if !path.exists() {
        return default_dir_path;
    }

    if let Ok(dir_iter) = path.read_dir() {
        for entry in dir_iter {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let name = path
                        .file_name()
                        .unwrap_or(OsStr::new(""))
                        .to_str()
                        .unwrap_or("");
                    if !name.is_empty() && name.starts_with(title_id) {
                        return path.to_str().unwrap_or(&default_dir_path).to_string();
                    }
                }
            }
        }
    }

    default_dir_path
}

pub fn create_parent_if_not_exists(path: &str) -> Result<(), Box<dyn Error>> {
    match Path::new(path).parent() {
        Some(parent) => {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        None => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::utils::{base64_decode, base64_encode};

    use super::ease_out_expo;

    #[test]
    pub fn test_md5() {
        let mut context = md5::Context::new();
        context.consume("123");
        context.consume("456");
        let data = "123456";
        let res = md5::compute(data);
        assert_eq!("e10adc3949ba59abbe56e057f20f883e", format!("{:x}", res));
        assert_eq!(
            "e10adc3949ba59abbe56e057f20f883e",
            format!("{:x}", context.compute())
        );
    }

    #[test]
    pub fn test_base64() -> Result<(), Box<dyn std::error::Error>> {
        let data = String::from("hello world");
        let data = data.as_bytes();
        let key = base64_encode(data);
        let res = String::from_utf8_lossy(&base64_decode(&key)?).to_string();
        assert_eq!(res, "hello world");

        Ok(())
    }

    #[test]
    pub fn test_ease_out_expo() {
        assert_eq!(
            ease_out_expo(
                Duration::from_millis(1),
                Duration::from_millis(10),
                0.0,
                10.0,
            ),
            5.0
        );

        assert_eq!(
            ease_out_expo(
                Duration::from_millis(2),
                Duration::from_millis(10),
                0.0,
                10.0,
            ),
            7.5
        );
        assert_eq!(
            ease_out_expo(
                Duration::from_millis(3),
                Duration::from_millis(10),
                0.0,
                10.0,
            ),
            8.75
        );
        assert_eq!(
            ease_out_expo(
                Duration::from_millis(10),
                Duration::from_millis(10),
                0.0,
                10.0,
            ),
            10.0
        );
    }

    #[test]
    fn test_normalize_path() {
        let path = "a\\a/a:a*a?a\"a\'a<a>a|a";
        let path = super::normalize_path(path);
        assert_eq!(
            "a_a_a_a_a_a_a_a_a_a_a",
            path
        );
    }
}
