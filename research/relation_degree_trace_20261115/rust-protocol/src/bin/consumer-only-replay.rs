//! Consumer-only attribution: existing native verification stays enabled.
#[allow(dead_code)]
#[path = "fused-replay.rs"]
mod fused;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fused::run_with(false, true, false)
}
