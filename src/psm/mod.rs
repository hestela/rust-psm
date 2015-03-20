// TODO: import rust uuid lib
extern crate error;
extern crate ep;

use self::error::*;
use self::ep::*;

pub struct Version{
  major: usize,
  minor: usize
}

enum ComponentType {
  CORE, MQ, AM, IB
}

pub fn init(version: Version) -> Result<(), Error> {
  return Ok(())
}

pub fn finalize() -> Result<(), Error> {
  return Ok(())
}

// TODO: add error handler fns

// TODO: determine if we want to make a "device" mod
pub fn num_ipath_units() -> Result<u32, Error> {
  return Ok(1)
}
