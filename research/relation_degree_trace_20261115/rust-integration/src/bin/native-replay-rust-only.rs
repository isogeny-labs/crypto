//! Native pending wire: mandatory complete root replay before exposing a basis.
#[allow(dead_code)]
#[path = "native-replay-worker.rs"]
mod adapter;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    adapter::run_completion_variant(0)
}
