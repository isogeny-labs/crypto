//! Native-only attribution: existing external Singular consumer stays enabled.
#[allow(dead_code)]
#[path = "fused-replay.rs"]
mod fused;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fused::run_with(false, false, true)
}
