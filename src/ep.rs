use epid::*;
// TODO: determine more appropriate types instead of passing -1/NULL, psm.h:449
// TODO: when rust supports conditional compilation add PSM_VERNO conditional fields
pub struct ep_options {
  timeout: u64,
  unit: isize,
  affinity: isize,
  shm_mbytes: isize,
  num_send_buffers: isize,
  network_pkey: u64,
  port: isize,
  ib_service_id: u64,
  // TODO: make psm_path_res type
  send_descriptor: isize,
  ep_imm_size: isize
}

// TODO: change uuid to rust uuid, add psm_ep
pub fn ep_open(uuid: u64, ep: u64, epid: psm_epid) -> Result<*ep_options, psm_error> {
  Err(psm_err { error: PSM_ERROR_LAST, error_str: "send help"})
}
