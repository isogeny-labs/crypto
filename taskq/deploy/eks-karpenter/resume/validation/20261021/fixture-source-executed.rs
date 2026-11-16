//! Public SHA-256 chain for process-recovery validation, not a speed benchmark.
//! The executable never reads progress from disk: resuming beyond zero requires
//! restoring the process's RAM. Replay independently recomputes the chain.
use sha2::{Digest, Sha256};
use std::{env, fs::OpenOptions, io::Write, thread, time::Duration};

fn advance(state: [u8; 32], sequence: u64) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(state);
    h.update(sequence.to_le_bytes());
    h.finalize().into()
}

fn hex(state: &[u8; 32]) -> String {
    state.iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") => {
            let path = args.get(2).expect("run OUTPUT TARGET");
            let target: u64 = args.get(3).expect("target").parse().expect("integer target");
            let mut out = OpenOptions::new().create(true).append(true).open(path).unwrap();
            let mut state = [0u8; 32];
            writeln!(out, "start {}", std::process::id()).unwrap();
            out.sync_all().unwrap();
            for sequence in 1..=target {
                state = advance(state, sequence);
                if sequence % 1024 == 0 || sequence == target {
                    writeln!(out, "step {sequence} {}", hex(&state)).unwrap();
                    out.sync_all().unwrap();
                    thread::sleep(Duration::from_millis(100));
                }
            }
            writeln!(out, "complete {target} {}", hex(&state)).unwrap();
            out.sync_all().unwrap();
        }
        Some("verify") => {
            let text = std::fs::read_to_string(args.get(2).expect("verify OUTPUT")).unwrap();
            let mut state = [0u8; 32];
            let mut sequence = 0u64;
            let mut starts = 0;
            let mut steps = 0;
            let mut complete = false;
            for line in text.lines() {
                let fields: Vec<_> = line.split_whitespace().collect();
                match fields[0] {
                    "start" => { starts += 1; assert_eq!(sequence, 0, "restarted from zero"); }
                    "step" | "complete" => {
                        assert_eq!(fields.len(), 3);
                        let next: u64 = fields[1].parse().unwrap();
                        if fields[0] == "step" { assert!(next > sequence, "progress regressed"); steps += 1; }
                        else { assert_eq!(next, sequence); complete = true; }
                        while sequence < next { sequence += 1; state = advance(state, sequence); }
                        assert_eq!(fields[2], hex(&state), "hash-chain mismatch");
                    }
                    _ => panic!("unknown record"),
                }
            }
            assert_eq!(starts, 1, "expected exactly one process start");
            assert!(complete && steps > 1, "missing completed computation");
            println!("{{\"ok\":true,\"starts\":{starts},\"steps\":{steps},\"sequence\":{sequence},\"sha256\":\"{}\"}}", hex(&state));
        }
        _ => panic!("usage: isolab-resume-fixture run OUTPUT TARGET | verify OUTPUT"),
    }
}
