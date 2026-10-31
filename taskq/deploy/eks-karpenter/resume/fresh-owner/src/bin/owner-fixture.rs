//! Bounded software fixture, never packaged into a task image.
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if let Some(i) = args.iter().position(|v| v == "--fixture-marker") {
        std::fs::write(&args[i + 1], b"ready").unwrap();
    }
    if args.iter().any(|v| v == "--fixture-wait") {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    std::thread::sleep(std::time::Duration::from_millis(30));
    if std::env::args().any(|v| v == "--fixture-signal") {
        unsafe {
            libc::raise(libc::SIGTERM);
        }
    }
    if std::env::args().any(|v| v == "--fixture-failure") {
        std::process::exit(7);
    }
}
