
#![allow(dead_code, unused)]

use std::ffi::{
    c_int,
    c_uint,
    CStr,
};

fn mkdir(path: &[u8], mode: c_uint) -> Result<(), ()> {
    let path = CStr::from_bytes_with_nul(path).map_err(|_| ())?;
    let res = unsafe { libc::mkdir(path.as_ptr(), mode) };
    if res < 0 {
        return Err(())
    }
    Ok(())
}

fn mount(src: &[u8], target: &[u8], fstype: &[u8], flags: u64) -> Result<(), ()> {
    let src = CStr::from_bytes_with_nul(src).map_err(|_| ())?;
    let target = CStr::from_bytes_with_nul(target).map_err(|_| ())?;
    let fstype = CStr::from_bytes_with_nul(fstype).map_err(|_| ())?;
    let res = unsafe { libc::mount(
        src.as_ptr(),
        target.as_ptr(),
        fstype.as_ptr(),
        flags,
        std::ptr::null(),
    ) };
    if res < 0 {
        return Err(())
    }
    Ok(())
}

pub(crate) fn prepare_fs() -> Result<(), ()> {
    mkdir(b"/proc\0", 0o0755).ok();
    mkdir(b"/sys\0", 0o0755).ok();
    mkdir(b"/dev\0", 0o0755).ok();
    mkdir(b"/tmp\0", 0o1777).ok();
    mount(b"none\0", b"/proc\0", b"proc\0", 0)?;
    mount(b"none\0", b"/sys\0", b"sysfs\0", 0)?;
    mount(b"none\0", b"/dev\0", b"devtmpfs\0", 0)?;
    mount(b"none\0", b"/tmp\0", b"tmpfs\0", 0)?;

    Ok(())
}
