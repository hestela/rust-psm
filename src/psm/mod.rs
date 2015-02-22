// TODO: import rust uuid lib
pub use self::error::*;
pub use self::ep::*;
//pub use self::epid::*;

pub mod ep;
pub mod error;
pub mod epid;

pub struct Version{
  major: usize,
  minor: usize
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

//TODO: need a uuid mod with uuid import
//pub fn generate_uuid()
