use fileops::Fd;
use ipath;
use ipath::{user_info, base_info, control_data, cmd, cmd_data};
use std::mem;
use std::error::Error;

// TODO: to borrow or to not to borrow.
// For the sake of brevity, assume the latest driver/device is in use
pub fn userint(fd: Fd, u_info: user_info, b_info: base_info) ->
    (control_data, user_info, base_info) {
  let mut b_info_clone = b_info.clone();
  let cmd_union = cmd_data { user_info : u_info.clone() };

  let mut driver_cmd = cmd { cmd_type : ipath::CMD_ASSIGN_CONTEXT,
    cmd_data : cmd_union };
  unsafe {
    driver_cmd.cmd_data.user_info.base_info_size =
      mem::size_of::<user_info>() as u32;
    // XXX: safety is kill
    driver_cmd.cmd_data.user_info.base_info_addr =
      &mut b_info_clone as *mut base_info as u64;

    let bytes = fd.write(&driver_cmd);
    match bytes {
      Ok(res) => println!("Got {} bytes written!", res),
      Err(err) => println!("Got an error {}", err.description().to_owned())
    }
  }
  unimplemented!();
}

