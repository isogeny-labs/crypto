//! Native SOFTWARE TEST fixture; no crypto, experiment or network work.
use std::io::{self, Read};

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("ok") => (),
        Some("fail") => std::process::exit(7),
        Some("hold") => {
            let mut bytes = Vec::new();
            io::stdin().read_to_end(&mut bytes).unwrap();
        }
        _ => std::process::exit(2),
    }
}
