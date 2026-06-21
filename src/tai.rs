use core::slice;
use std::{cell::RefCell, ffi::CStr, fs, os::raw::*, path::Path, ptr::null};

use crate::utils::{create_parent_if_not_exists, str_to_c_str};

const APP_KERNEL_PATH: &str = "app0:sce_sys/resources/kernel.skprx";
const PLUGIN_KERNEL_PATHS: [&str; 3] = [
    "ux0:data/SAVSYNC01/sce_sys/resources/kernel.skprx",
    "ux0:VitaShell/module/kernel.skprx",
    "ux0:data/save-sync/kernel.skprx",
];

extern "C" {
    fn sceAppUtilLoad();
    fn sceAppUtilExit();
    fn taiLoad(path: *const c_char) -> i32;
    fn sceLoad() -> i32;
    fn pfs_mount(path: *const c_char) -> i32;
    fn pfs_unmount() -> i32;
    fn applist_init();
    fn applist_get() -> *const TitleList;
    fn applist_free(appList: *const TitleList);
    // return 0 if failed
    fn get_account_id() -> c_ulonglong;
    // return < 0 if failed
    fn change_account_id(fs_path: *const c_char, account_id: c_ulonglong) -> c_char;
    fn prevent_to_sleep();
    fn launch_app_by_title_id(title_id: *const c_char);
}

#[repr(C)]
pub struct Title {
    title_id: *const c_char,
    real_id: *const c_char,
    name: *const c_char,
    iconpath: *const c_char,
}

impl Title {
    pub fn title_id(&self) -> &str {
        unsafe {
            let c_str = CStr::from_ptr(self.title_id);
            c_str.to_str().unwrap()
        }
    }

    pub fn real_id(&self) -> &str {
        unsafe {
            let c_str = CStr::from_ptr(self.real_id);
            c_str.to_str().unwrap()
        }
    }

    pub fn name(&self) -> &str {
        unsafe {
            let c_str = CStr::from_ptr(self.name);
            c_str.to_str().unwrap()
        }
    }

    pub fn iconpath(&self) -> &str {
        unsafe {
            let c_str = CStr::from_ptr(self.iconpath);
            c_str.to_str().unwrap()
        }
    }
}

#[repr(C)]
pub struct TitleList {
    size: c_int,
    list: *const Title,
}

pub struct Titles {
    data: RefCell<Option<*const TitleList>>,
}

impl Titles {
    /// get all list
    pub fn new() -> Titles {
        unsafe {
            applist_init();
        }
        Titles {
            data: RefCell::new(None),
        }
    }

    pub fn data(&self) -> *const TitleList {
        if self.data.borrow().is_none() {
            unsafe {
                let ptr = applist_get();
                if !ptr.is_null() {
                    (*self.data.borrow_mut()) = Some(ptr);
                    return ptr;
                }
            }
            null()
        } else {
            self.data.borrow().expect("get data ptr of titles")
        }
    }

    pub fn size(&self) -> usize {
        if self.data().is_null() {
            return 0;
        }
        unsafe { (*self.data()).size as usize }
    }

    pub fn get_title_by_idx(&self, idx: i32) -> Option<&Title> {
        let size = self.size() as i32;
        if idx < size {
            unsafe {
                let list = slice::from_raw_parts((*self.data()).list, size as usize);
                return Some(&list[idx as usize]);
            }
        }
        None
    }

    pub fn iter(&self) -> TitlesIterator {
        TitlesIterator {
            index: 0,
            titles: self,
        }
    }
}

impl Drop for Titles {
    fn drop(&mut self) {
        let ptr = self.data();
        unsafe {
            applist_free(ptr);
        }
    }
}

pub struct TitlesIterator<'a> {
    index: usize,
    titles: &'a Titles,
}

impl<'a> Iterator for TitlesIterator<'a> {
    type Item = &'a Title;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.titles.data().is_null() {
            unsafe {
                let size = self.titles.size();
                if self.index < size {
                    let list = slice::from_raw_parts((*self.titles.data()).list, size);
                    let item = Some(&list[self.index]);
                    self.index += 1;
                    return item;
                }
            }
        }
        None
    }
}

pub fn sce_app_util_load() {
    unsafe {
        sceAppUtilLoad();
    }
}

pub fn sce_app_util_exit() {
    unsafe {
        sceAppUtilExit();
    }
}

/// # tai_load_start_kernel_module
///
/// load skprx module
pub fn tai_load_start_kernel_module() -> Result<(), String> {
    unsafe {
        let mut loaded = false;
        for path in PLUGIN_KERNEL_PATHS {
            let path = str_to_c_str(path);
            let id = taiLoad(path.as_slice().as_ptr() as *const c_char);
            if id >= 0 || id == -2147299309 {
                loaded = true;
                break;
            }
        }
        if !loaded {
            if Path::new(APP_KERNEL_PATH).exists() {
                let to_path = PLUGIN_KERNEL_PATHS[2];
                create_parent_if_not_exists(to_path).ok();
                fs::copy(APP_KERNEL_PATH, to_path).ok();
                let path = str_to_c_str(to_path);
                let id = taiLoad(path.as_slice().as_ptr() as *const c_char);
                if id < 0 && id != -2147299309 {
                    return Err(format!("cannot find kernel module: {:#x}\n", id));
                }
            } else {
                return Err("cannot find kernel module\n".to_string());
            }
        }
    }

    Ok(())
}

/// # sce_kernel_load_start_module
///
/// load suprx module
pub fn sce_kernel_load_start_module() -> Result<(), String> {
    unsafe {
        let id = sceLoad();
        if id < 0 {
            return Err(format!("cannot find user module: {:#x}\n", id));
        }
    }

    Ok(())
}

pub struct Tai;

impl Drop for Tai {
    fn drop(&mut self) {
        sce_app_util_exit();
    }
}

/// init vitashell module
pub fn tai_init() -> Result<Tai, String> {
    // load kernel module
    tai_load_start_kernel_module()?;

    // load user module
    sce_kernel_load_start_module()?;

    // sce app util
    sce_app_util_load();

    Ok(Tai)
}

/// mount save data
pub fn mount_pfs(path: &str) -> i32 {
    let c_str = str_to_c_str(path);
    unsafe {
        // unmount first
        pfs_unmount();
        pfs_mount(c_str.as_slice().as_ptr() as *const c_char)
    }
}

/// unmount save date
pub fn unmount_pfs() -> i32 {
    unsafe { pfs_unmount() }
}

pub fn get_psv_account_id() -> u64 {
    unsafe { get_account_id() }
}

pub fn change_psv_account_id(sfo_path: &str, account_id: u64) -> i8 {
    unsafe {
        let c_str = str_to_c_str(sfo_path);
        change_account_id(c_str.as_slice().as_ptr() as *const c_char, account_id)
    }
}

pub fn psv_prevent_sleep() {
    unsafe {
        prevent_to_sleep();
    }
}

pub fn psv_launch_app_by_title_id(title_id: &str) {
    unsafe {
        let c_str = str_to_c_str(title_id);
        launch_app_by_title_id(c_str.as_slice().as_ptr() as *const c_char);
    }
}
