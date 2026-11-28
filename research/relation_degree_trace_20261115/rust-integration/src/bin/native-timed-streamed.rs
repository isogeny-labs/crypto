#[allow(dead_code)]
#[path = "native-timed-worker.rs"]
mod timed;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    timed::run(false)
}
