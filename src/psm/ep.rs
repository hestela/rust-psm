use psm::epid::*;
use psm::error::*;
use psm::consts::*;
use std::rc::Rc;

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

pub struct Epaddr {
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
  mctxt_current: Option<Rc<Epaddr>>,
  // outoforder_q: Mqsq TODO: make Mqsq type
  outoforder_c: usize,

  // Linked list of Epaddr for multi-context
  // TODO: what type of pointers? box, raw, Rc?
  mctxt_master: Option<Rc<Epaddr>>,
  mctxt_prev: Option<Rc<Epaddr>>,
  mctxt_next: Option<Rc<Epaddr>>
}


impl Ep {
  // TODO: change job_key to rust uuid, add psm_ep
  pub fn open(job_key: u64, ep_opts: EpOpts) -> Result<Box<(Ep, Epaddr)>, Error> {
    Err(Error { error: Error_type::PSM_ERROR_LAST, error_str: "send help"})
  }

  pub fn close(ep: Ep, mode: isize, timeout: u64) -> Result<(), Error> {
    Ok(())
  }

  pub fn connect(ep: Ep, epids: &Vec<Epid>, epid_masks: &Vec<isize>, timeout: i64) -> Result<Box<Vec<Epaddr>>, Box<Vec<Error>>> {
    let dummy: Vec<Error> = Vec::new();
    Err(Box::new(dummy))
  }
}

impl Epaddr {

}
