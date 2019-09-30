use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let old_uid = unsafe { libc::getuid() };
    let unshare_ret = unsafe { libc::unshare(libc::CLONE_NEWUSER | libc::CLONE_NEWNS) };
    if unshare_ret != 0 {
        panic!("unshare returned {}", std::io::Error::last_os_error());
    }
    let my_pid = std::process::id();
    let proc_path = format!("/proc/{}", my_pid);
    let proc_path = Path::new(&proc_path);
    let uid_map = format!("0 {} 1", old_uid);
    File::create(proc_path.join("uid_map")).unwrap().write_all(uid_map.as_bytes()).unwrap();
    let setuid_ret = unsafe { libc::setuid(0) };
    if setuid_ret != 0 {
        panic!("setuid returned {}", std::io::Error::last_os_error());
    }
    let my_uid = unsafe { libc::getuid() };
    println!("uid: {}", my_uid);
    println!("username: {}", whoami::username());
    println!("pwd: {}", std::env::current_dir().unwrap().to_string_lossy());
}
