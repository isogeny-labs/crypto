//! Narrow adapter for the optional native phase recorder.
//! An inactive recorder preserves the original solver behavior and counters.

use crate::Result;

pub(crate) fn phase(name: &'static str) -> Result<()> {
    crate::metrics::phase(name)
}

pub(crate) fn batch_group(_doubling: bool) -> Result<()> {
    Ok(())
}

#[cfg(test)]
pub(crate) fn observe_group_calls<T>(
    meter: &mut crate::Meter,
    run: impl FnOnce(&mut crate::Meter) -> T,
) -> (T, u64) {
    let before = meter.curve_additions;
    let result = run(meter);
    (result, meter.curve_additions - before)
}
