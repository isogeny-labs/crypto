//! Same root adapter, separate Rust consumer with legacy native verification.
#[allow(dead_code)]
#[path = "native-replay-worker.rs"]
mod adapter;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    adapter::run(false, true)
}
