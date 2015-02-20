// TODO: import rust uuid lib
use error::*;
use epid::*;

pub struct psm_version {
  major: usize,
  minor: usize
}

pub fn init(version: psm_version) -> Result<(), psm_error> {
  return Ok(())
}

pub fn finalize() -> Result<(), psm_error> {
  return Ok(())
}

// TODO: add error handler fns

// TODO: determine if we want to make a "device" mod
pub fn num_ipath_units() -> Result<u32, psm_error> {
  return Ok(1)
}

//TODO: need a uuid mod with uuid import
//pub fn generate_uuid()
