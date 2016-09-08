#![crate_type="lib"]
#![crate_name="rust_psm"]
#![feature(untagged_unions)]

extern crate libc;
extern crate uuid;

pub mod error;
pub mod fileops;
pub mod ep;
pub mod ipath;
pub mod ptl_am;
pub mod ptl_ips;
pub mod ptl_self;

use error::Error;

pub struct Version {
  major: usize,
  minor: usize
}

enum ComponentType {
  CORE, MQ, AM, IB
}

/// Can only be called once, using ONCE_INIT as a way arround global state.
/// C version allows for init to be called multiple times, but after the
/// first time, it seems like nothing useful is done.
/// This should also prevent calling init after finalize.
pub fn init(version: Version) -> Error {
  use std::sync::{Once, ONCE_INIT};
  // TODO: not sure if this needs to be mutable
  static INIT: Once = ONCE_INIT;
  let mut result = Error::AlreadyInitialized;
  INIT.call_once(|| {
    result = Error::Ok
  });
  return result;
}

pub fn finalize() -> () {
}

// TODO: add error handler fns

// TODO: determine if we want to make a "device" mod
pub fn num_ipath_units() -> Result<u32, Error> {
  return Ok(1)
}
