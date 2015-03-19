macro_rules! pack_epid {
  ($lid:expr, $ctxt:expr, $sub_ctxt:expr, $hca_type:expr, $sl:expr) => (
    ((($lid as u64) & 0xffff) << 16) |
    ((($sub_ctxt as u64) & 0x3) << 14) |
    ((($ctxt as u64) & 0x3f) << 8) |
    ((($sl as u64) & 0x4) << 4) |
    (($hca_type as u64) & 0xf));
}
