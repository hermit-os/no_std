#![no_std] // don't link the Rust standard library
#![no_main]

#[macro_use]
extern crate log;
extern crate alloc;
extern crate hermit;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::ffi::CStr;
use hermit::fs::{self, readdir, File};
use hermit::io::{Read, Write};
use hermit::{sys_close, sys_getdents64, sys_opendir, sys_shutdown, Dirent64};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main(_argc: i32, _argv: *const *const u8, _env: *const *const u8) {
    info!("Enter main function");

    info!("Read content of / with");
    let fd = sys_opendir("/\0".as_ptr());
    let mut v: Vec<u8> = vec![0; 0x1000];
    let readlen = sys_getdents64(fd, v.as_mut_ptr() as *mut Dirent64, 0x1000);
    let mut i = 0;
    loop {
        if i >= readlen {
            break;
        }

        let dir = unsafe { &*(v.as_ptr().offset(i.try_into().unwrap()) as *const Dirent64) };
        let name = unsafe {
            CStr::from_ptr(&dir.d_name as *const _ as *const i8)
                .to_str()
                .unwrap()
        };
        info!("{}", name);
        i += dir.d_off;
    }
    sys_close(fd);

    info!("Read content of /proc with");
    for i in readdir("/proc").expect("Unable to read /proc").iter() {
        info!("{:?}", *i);
    }

    if let Ok(attr) = fs::file_attributes("/proc/version") {
        info!("Attributes of /etc/version {:?}", attr);
    } else {
        error!("Unable to get file attributes");
    }

    if let Ok(mut file) = File::open("/proc/version") {
        let mut version: String = String::new();
        if file.read_to_string(&mut version).is_err() {
            error!("Unable to read /proc/version");
        }
        info!("version: {}", version);
    } else {
        error!("Unable to open file");
    };

    info!("create file /tmp/test.txt");
    if let Ok(mut file) = File::create("/tmp/test.txt") {
        write!(file, "Hello from HermitOS!").expect("Unable to write into /tmp/test.txt");
    } else {
        error!("Unable to create file");
    }

    info!("read file /tmp/test.txt");
    if let Ok(mut file) = File::open("/tmp/test.txt") {
        let mut content: String = String::new();
        if file.read_to_string(&mut content).is_err() {
            error!("Unable to read /tmp/test.txt");
        }
        info!("File content {}", content);
    } else {
        error!("Unable to open file");
    }

    sys_shutdown(0);
}
