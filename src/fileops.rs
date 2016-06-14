extern crate libc;

use std::ffi::CString;
use libc::c_int;

pub fn open<T: Into<String>>(path: T, mode: c_int) -> c_int {
  unsafe { libc::open(CString::new(path.into())
                      .unwrap_or(CString::new("").unwrap()).as_ptr(), mode) }
}

pub fn close(fd: c_int) -> c_int {
  unsafe { libc::close(fd) }
}

#[test]
fn open_close_devnull() -> () {
  let fd = open("/dev/null", libc::O_RDONLY);
  assert!(fd >= 0);
  let ret = close(fd);
  assert!(ret != -1);
}
