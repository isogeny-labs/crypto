//! Matched root adapter, native verification plus final export and Rust replay.
#[allow(dead_code)]
#[path = "native-replay-worker.rs"]
mod adapter;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    adapter::run_completion_variant(1)
}
