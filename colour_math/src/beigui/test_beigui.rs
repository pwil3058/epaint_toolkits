// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::fdrn::FDRNum;

#[test]
fn div_by_u8() {
    assert_eq!(FDRNum::ONE / 2, FDRNum(u64::MAX as i128 / 2));
    assert_eq!(FDRNum::ONE / 3, FDRNum(u64::MAX as i128 / 3));
}
