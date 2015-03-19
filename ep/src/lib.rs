extern crate error;

use error::{Error_type,Error};
use std::rc::Rc;
use self::consts::*;
use self::macros::*;

pub mod consts;
#[macro_use]
pub mod macros;

// TODO: determine more appropriate types instead of passing -1/NULL, psm.h:449
// TODO: when rust supports conditional compilation add PSM_VERNO conditional fields
pub struct Ep {
  // TOOD: lots of things to add
  epid: Epid
}
pub struct EpOpts {
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

enum PtlAddr {
  // TODO: add ptl_epaddr type
  ptladdr_u3 ([u32; 2]),
  ptladdr_u64 (u64),
  ptladdr_data ([u8; 0])
}

pub struct Epaddr <'a> {
  // *ptl: ptl          TODO: ptl is a reference to the parent obj
  //ptlctrl: Ptl_ctl,
  epid: Epid,
  ep: Ep,
  // void *usr_ep_ctxt  TODO: this is a raw pointer, find what to do
  // TODO: add egrlong/data things
  ptl_addr: PtlAddr,
  mctxt_gihdi: [u64; IPATH_MAX_UNIT],
  mctxt_epid: [Epid; IPATH_MAX_UNIT],
  mctxt_epcount: usize,
  mctxt_nsconn: usize,
  mctxt_send_seqnum: u16,
  mctxt_recv_seqnum: u16,
  mctxt_current: Option<&'a Epaddr<'a>>,
  // outoforder_q: Mqsq TODO: make Mqsq type
  outoforder_c: usize,

  // Linked list of Epaddr for multi-context
  mctxt_master: Option<&'a Epaddr<'a>>,
  mctxt_prev: Option<&'a Epaddr<'a>>,
  mctxt_next: Option<&'a Epaddr<'a>>
}


impl Ep {
  // TODO: change job_key to rust uuid, add psm_ep, add new
  pub fn open<'a>(job_key: u64, ep_opts: EpOpts) -> Result<(Ep, &'a Epaddr<'a>), Error> {
    Err(Error { error: Error_type::PSM_ERROR_LAST, error_str: "send help"})
  }

  pub fn close(ep: Ep, mode: isize, timeout: u64) -> Result<(), Error> {
    Ok(())
  }

  pub fn connect<'a>(ep: Ep, epids: Box<Vec<Epid>>, epid_masks: &Vec<isize>, timeout: i64) -> Result<Box<Vec<&'a Epaddr<'a>>>, Box<Vec<Error>>> {
    Err(Box::new(vec!()))
  }
}

impl <'a>Epaddr <'a> {
  // TODO: figure out what fields are commonly used when epaddr is constructed
}
/* TODO: see if we can make this into a struct so we can real getters/setters that dont have the
 * words get/set
 */

pub type Epid = u64;

pub fn get_nid(epid: Epid) -> u64 {
  1
}

pub fn get_context(epid: Epid) -> u64 {
  1
}

pub fn get_port(epid: Epid) -> u64 {
  1
}
