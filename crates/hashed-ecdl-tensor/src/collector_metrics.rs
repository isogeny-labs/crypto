//! Narrow adapter for the mirror's existing optional phase/group recorder.
//! Crypto retains its original Meter diagnostics and does not start clocks.

use crate::Result;

pub(crate) fn phase(_name: &'static str) -> Result<()> {
    Ok(())
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
