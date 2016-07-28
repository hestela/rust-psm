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
// FIXME: allined by 8
// Size of struct is 32 bytes, divisable by 8
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

#[repr(C)]
// FIXME: allined by 8
// Size of struct is 176 bytes, divisable by 8
struct base_info {
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
    num_active: u32,
    unit: u32,
    port: u32,
    context: u32,
    subcontext: u32,
    num_contexts: u32,
    num_subcontexts: u32,
    rec_cpu: u32,
}

#[repr(C)]
pub struct driver_cmd {
    cmd_type: u32,
    cmd: cmd_union,
}

// XXX: bindgen did some magic to get the structs into this union for cmd
#[repr(C)]
pub struct cmd_union {
    pub _bindgen_data_: [u64; 4usize]
}
// TODO: find right sizes
// TODO: remove extra structs/types
// TODO: keep largest struct to make union compatible
impl cmd_union {
    pub unsafe fn mem_info(&mut self) -> *mut mem_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn mic_info(&mut self) -> *mut mic_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn tid_info(&mut self) -> *mut tid_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn user_info(&mut self) -> *mut user_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn sdma_cntr(&mut self) -> *mut u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ctxt_info(&mut self) -> *mut u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn recv_ctrl(&mut self) -> *mut u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn armlaunch_ctrl(&mut self) -> *mut u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn part_key(&mut self) -> *mut u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn slave_mask_addr(&mut self) -> *mut u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn poll_type(&mut self) -> *mut u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ctxt_bp(&mut self) -> *mut u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn event_mask(&mut self) -> *mut u32 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

#[repr(C)]
pub struct pbc {
    pub _bindgen_data_: [u32; 3usize]
}
impl pbc {
    pub unsafe fn qword(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn dword(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn length(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn fill1(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(4))
    }
    pub unsafe fn pbcflags(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(8))
    }
}

#[repr(C)]
pub struct header {
    pub ver_context_tid_offset: ::std::os::raw::c_int,
    pub chksum: ::std::os::raw::c_int,
    pub pkt_flags: ::std::os::raw::c_int
}

#[repr(C)]
pub struct message_header {
    pub lrh: [::std::os::raw::c_int; 4usize],
    pub bth: [::std::os::raw::c_int; 3usize],
    pub iph: header,
    pub sub_opcode: ::std::os::raw::c_int
}
