//! Contains operations related to the /dev/ipath character files.

extern crate libc;

use std::ffi::CString;
use libc::{c_int, size_t, c_void};
use std::os::unix::io::{RawFd, AsRawFd};
use std::ops::Drop;
use std::io::Error;
use std::mem::size_of;

pub struct Fd(RawFd);

impl Fd {
  pub fn open<T: Into<String>>(path: T, mode: c_int) -> Result<Fd, Error> {
    let fd = unsafe { libc::open(CString::new(path.into())
                .unwrap_or(CString::new("").unwrap()).as_ptr(), mode) };
    match fd {
      -1 => Err(Error::last_os_error()),
      _ => Ok(Fd(fd))
    }
  }

  fn close(&self) -> c_int {
    unsafe { libc::close(self.0) }
  }

  // With F_SETFD we only care if fcntl failed
  pub fn try_set_flag(&self, flag: c_int ) -> Result<c_int, Error> {
    match unsafe { libc::fcntl(self.0, libc::F_SETFD, flag) } {
      -1 => Err(Error::last_os_error()),
      ret @ _ => Ok(ret)
    }
  }

  pub fn write<T>(&self, t: T) -> Result<size_t, Error> where T: Sized {
    match unsafe { libc::write(self.0, &t as *const T as *const c_void, size_of::<T>() as size_t) } {
      -1 => Err(Error::last_os_error()),
      ret @ _ => Ok(ret as size_t)
    }
  }
}

impl AsRawFd for Fd {
  fn as_raw_fd(&self) -> RawFd {
    self.0
  }
}

impl Drop for Fd {
  // XXX: Do we need to check result of close?
  fn drop(&mut self) {
    if self.0 != -1 {
      self.close();
    }
  }
}

#[test]
// Check open/close on a file that should exist in most linux based OS.
fn open_close_devnull() {
  use std::error::Error as std_error;
  match Fd::open("/dev/null", libc::O_RDONLY) {
    Err(e) => panic!(e.description().to_owned()),
    _ => ()
  }
}
