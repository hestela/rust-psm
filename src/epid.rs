use error::*;

/* TODO: see if we can make this into a struct so we can real getters/setters that dont have the
 * words get/set
 */

pub type psm_epid = u64;

pub fn get_nid(epid: psm_epid) -> u64 {
  1
}

pub fn get_context(epid: psm_epid) -> u64 {
  1
}

pub fn get_port(epid: psm_epid) -> u64 {
  1
}
