//! This module should be equivalent to the ipath directory in the C version.
//! Mostly support functions and functions to interface with the ib_qib driver
//! will live in this module.
pub mod service;
pub mod sysfs;
pub mod proto;

const TF_NFLOWS: isize = 32;

// TODO: most of these members have the most cryptic names
// device and control_data get passed to the driver, must be in C format
#[repr(C)]
struct device {
  spd_fd: i32,
  spd_type: i32,
  // XXX: These next two are volatile in C
  spd_uregbase: *mut u64,
  spd_piobase: *mut u64,
  pad: [u64; 4]
}

#[repr(C)]
struct control_data {
  spc_dev: device,
  // XXX: most of these are stored as little endian 32s but should be fine
  regs: [i32; TF_NFLOWS << 1],
  tidflow_wmb_location: i32,
  sendbuf_status: u64,
  lasterr: isize,
  rcv_tail: *mut i32,
  rcv_hdr_head: *mut i32,
  rcv_egr_head: *mut i32,
  rcv_egr_tail: *mut i32,
  tid_egr_cnt: u32,
  rcv_tid_flow: *mut i32,
  tid_flow_wmb: *mut i32,
  spi_status: u64
}

#[repr(C)]
#[repr(align("8"))]
struct user_info {
  psm_lib_version: u32,
  scif_node_id: u32,
  base_info_size: u32,
  port_alg: u32,
  sub_context_cnt: u16,
  sub_context_id: u16,
  port: u32,
  base_info_addr: u64
}
