use crate::Error;
use anyhow::anyhow;
use rtcore::common::BitRange;
use std::ops::Range;

pub fn slice_idx(range: BitRange, idx: BitRange) -> Result<Range<usize>, Error> {
    if !range.contains_range(idx) {
        return Err(anyhow!("failed to index `{:?}` `{:?}`", range, idx));
    }

    let BitRange(_self_msb, self_lsb) = range;
    let BitRange(idx_msb, idx_lsb) = idx;

    let slice_idx = if range.is_downto() {
        let start = idx_lsb - self_lsb;
        let end = idx_msb - self_lsb + 1;
        start..end
    } else {
        let start = self_lsb - idx_lsb;
        let end = self_lsb - idx_msb + 1;
        start..end
    };

    Ok(slice_idx)
}
