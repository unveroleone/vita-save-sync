use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use zip::ZipWriter;

pub fn sha256_hex(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    format!("sha256:{}", hash.iter().map(|b| format!("{:02x}", b)).collect::<String>())
}

pub fn sha256_file(path: &str) -> Result<String, String> {
    let data = fs::read(path).map_err(|e| format!("Read file failed: {}", e))?;
    Ok(sha256_hex(&data))
}

pub fn zip_dir(from: &str, to: &str) -> Result<(), String> {
    let from = if from.ends_with('/') {
        from.to_string()
    } else {
        format!("{}/", from)
    };
    let output_path = Path::new(to);
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Create dir failed: {}", e))?;
        }
    }
    let file = fs::File::create(output_path).map_err(|e| format!("Create zip failed: {}", e))?;
    let mut zip = ZipWriter::new(file);
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    zip_inner(&mut zip, Path::new(&from), &from, &options)?;
    zip.finish().map_err(|e| format!("Finish zip failed: {}", e))?;
    Ok(())
}

fn zip_inner(
    zip: &mut ZipWriter<fs::File>,
    dir: &Path,
    prefix: &str,
    options: &zip::write::FileOptions,
) -> Result<(), String> {
    let mut buffer = vec![0u8; 1024 * 512];
    for entry in dir.read_dir().map_err(|e| format!("Read dir failed: {}", e))? {
        let entry = entry.map_err(|e| format!("Dir entry failed: {}", e))?;
        let path = entry.path();
        let name = path
            .strip_prefix(Path::new(prefix))
            .map_err(|e| format!("Strip prefix failed: {}", e))?;
        if path.is_file() {
            zip.start_file(
                name.to_string_lossy(),
                *options,
            )
            .map_err(|e| format!("Zip start file failed: {}", e))?;
            let mut f = fs::File::open(&path).map_err(|e| format!("Open file failed: {}", e))?;
            loop {
                let n = f.read(&mut buffer).map_err(|e| format!("Read failed: {}", e))?;
                if n == 0 {
                    break;
                }
                zip.write_all(&buffer[..n])
                    .map_err(|e| format!("Zip write failed: {}", e))?;
            }
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(
                name.to_string_lossy(),
                *options,
            )
            .map_err(|e| format!("Zip add dir failed: {}", e))?;
            zip_inner(zip, &path, prefix, options)?;
        }
    }
    Ok(())
}

pub fn zip_extract(from: &str, to: &str) -> Result<(), String> {
    let file = fs::File::open(from).map_err(|e| format!("Open zip failed: {}", e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Read zip failed: {}", e))?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| format!("Zip entry failed: {}", e))?;
        let out_path = match entry.enclosed_name() {
            Some(name) => Path::new(to).join(name),
            None => continue,
        };
        if entry.name().ends_with('/') {
            fs::create_dir_all(&out_path)
                .map_err(|e| format!("Create dir failed: {}", e))?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Create parent dir failed: {}", e))?;
            }
            let mut out =
                fs::File::create(&out_path).map_err(|e| format!("Create file failed: {}", e))?;
            io::copy(&mut entry, &mut out)
                .map_err(|e| format!("Extract failed: {}", e))?;
        }
    }
    Ok(())
}
