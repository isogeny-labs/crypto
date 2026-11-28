//! Explicit diagnostic entry point. Existing fused entry point disables timers.
#[allow(dead_code)]
#[path = "fused-replay.rs"]
mod fused;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fused::run(true)
}
