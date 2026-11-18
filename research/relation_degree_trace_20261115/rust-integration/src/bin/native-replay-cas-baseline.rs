//! Same root adapter, legacy native verification and existing CAS consumer.
#[allow(dead_code)]
#[path = "native-replay-worker.rs"]
mod adapter;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    adapter::run(false, false)
}
