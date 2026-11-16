//! Both opt-in native product checks and separate exact Rust consumer.
#[allow(dead_code)]
#[path = "fused-replay.rs"]
mod fused;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    fused::run_with(false, true, true)
}
