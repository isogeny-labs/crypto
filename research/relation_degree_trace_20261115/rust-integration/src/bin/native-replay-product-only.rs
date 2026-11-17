//! Same root adapter, native product checks with existing CAS consumer.
#[allow(dead_code)]
#[path = "native-replay-worker.rs"]
mod adapter;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    adapter::run(true, false)
}
