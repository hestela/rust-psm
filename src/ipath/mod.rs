//! This module should be equivalent to the ipath directory in the C version.
//! Mostly support functions and functions to interface with the ib_qib driver
//! will live in this module.
pub mod service;
pub mod sysfs;
pub mod proto;

const TF_NFLOWS: usize = 32;

// XXX: do we need the default impl for all these C types? or copy,debug, clone??

// TODO: most of these members have the most cryptic names
// device and control_data get passed to the driver, must be in C format
#[repr(C)]
// NOTE: this is _ipath_dev
pub struct device {
  spd_fd: i32,
  spd_type: i32,
  // XXX: These next two are volatile in C, see if we can use rust refs
  spd_uregbase: *mut u64,
  spd_piobase: *mut u64,
  pad: [u64; 4]
}

#[repr(C)]
// TODO: see if we can use rust references instead of the i32
// NOTE: this is _ipath_ctrl
pub struct control_data {
  spc_dev: device,
  // XXX: most of these are stored as little endian 32s but should be fine
  regs: [i32; TF_NFLOWS << 1],
  tidflow_wmb_location: i32,
  sendbuf_status: u64,
  lasterr: isize,
  // XXX: these pointers are volatile, prob should be u32/64
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
#[derive(Clone, Debug)]
// FIXME: allined by 8
// Size of struct is 32 bytes, divisable by 8
pub struct user_info {
  psm_lib_version: u32,
  scif_node_id: u32,
  base_info_size: u32,
  port_alg: u32,
  sub_context_cnt: u16,
  sub_context_id: u16,
  port: u32,
  base_info_addr: u64
}

#[repr(C)]
#[derive(Clone, Debug)]
// FIXME: allined by 8
// Size of struct is 176 bytes, divisable by 8
pub struct base_info {
  hw_version: u32,
  sw_version: u32,
  context: u16,
  sub_context: u16,
  mtu: u32,
  pio_size: u32,
  tid_count: u32,
  tid_egr_count: u32,
  rcv_hdr_ent_size: u32,
  rcv_hdr_count: u32,
  runtime_flags: u32,
  rcv_hdr_base: u64,
  rcv_egr_bufs: u64,
  rcv_egr_buf_size: u32,
  q_pair: u32,
  u_reg_base: u64,
  tid_max_size: u32,
  pio_align: u32,
  pio_index: u32,
  pio_count: u32,
  pio_buf_base: u64,
  pio_avail_addr: u64,
  status: u64,
  num_contexts: u32,
  unit: u16,
  port: u16,
  rcv_egr_per_chunk: u32,
  egr_chunk_size: u32,
  rcv_egr_buf_total_len: u32,
  rhf_offset: u32,
  rcv_hdr_tail_addr: u64,
  sub_context_u_reg_base: u64,
  sub_context_rcv_egr_buf: u64,
  sub_context_rcv_hdr_base: u64,
  send_buf_status: u64,
}

#[repr(C)]
// TODO: find right sizes
pub struct ctxt_info {
    num_active: u16,
    unit: u16,
    port: u16,
    context: u16,
    subcontext: u16,
    num_contexts: u16,
    num_subcontexts: u16,
    rec_cpu: u16,
}

/// ipath::cmd types for qib driver
const CMD_CTXT_INFO: u32 = 16;
const CMD_ASSIGN_CONTEXT: u32 = 23;

#[repr(C)]
struct cmd {
  cmd_type: u32,
  cmd_data: cmd_data
}

#[repr(C)]
union cmd_data {
  tid_info: tid_info,
  user_info: user_info,
  sdma_counter: u64,
  ctxt_info: u32,
  armlaunch_ctrl: u32,
  part_key: u16,
  slave_mask_addr: u64,
  poll_type: u16,
  ctxt_bp: u8,
  event_mask: u64
}

#[repr(C)]
pub struct tid_info {
  count: u32,
  pad: u32,
  virt_addr: u64,
  list_addr: u64,
  map_addr: u64
}

// TODO: determine what should be pub
#[repr(C)]
pub struct header {
    // TODO: this is a compacted field, accessed by shift/mask
    pub ver_context_tid_offset: u32,
    pub chksum: u16,
    // TODO: this is a compacted field, accessed by shift/mask
    pub pkt_flags: u16
}

// TODO: seceure r/w access to these with big endian fns
// IBTA requires LRH and BTH to be in big endian
#[repr(C)]
pub struct ibta_header {
  lrh: [u16; 4],
  bth: [u16; 3],
  ipath_header: header,
  sub_opcode: u8
}
