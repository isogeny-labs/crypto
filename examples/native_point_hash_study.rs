//! Ordinary point-table pilot binary: default allocator and uninstrumented
//! keys in clocks mode. Counts-only allocation hooks use the separate example.
#[path = "native_point_hash_study/driver.rs"]
mod study;
fn main() {
    if let Err(error) = study::entry(std::env::args_os().collect(), None) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
