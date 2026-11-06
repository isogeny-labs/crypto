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

fn verify(text: &str, target: u64, prefix: Option<&str>) -> Result<(u64, String), String> {
    if let Some(before) = prefix {
        if before.is_empty() || !text.starts_with(before) || text.len() <= before.len() {
            return Err("restored output must preserve and advance the checkpoint prefix".into());
        }
    }
    let mut state = [0u8; 32];
    let mut sequence = 0u64;
    let mut starts = 0;
    let mut steps = 0;
    let mut complete = false;
    for line in text.lines() {
        if complete {
            return Err("records after completion".into());
        }
        let fields: Vec<_> = line.split_whitespace().collect();
        match fields.first().copied() {
            Some("start") if fields.len() == 2 && starts == 0 && sequence == 0 => {
                fields[1].parse::<u32>().map_err(|_| "invalid PID")?;
                starts += 1;
            }
            Some("step" | "complete") if fields.len() == 3 && starts == 1 => {
                let next: u64 = fields[1].parse().map_err(|_| "invalid sequence")?;
                if next > target {
                    return Err("sequence exceeds requested target".into());
                }
                if fields[0] == "step" {
                    if next <= sequence {
                        return Err("progress regressed".into());
                    }
                    steps += 1;
                } else {
                    if next != sequence || next != target {
                        return Err("wrong completion target".into());
                    }
                    complete = true;
                }
                while sequence < next {
                    sequence += 1;
                    state = advance(state, sequence);
                }
                if fields[2] != hex(&state) {
                    return Err("hash-chain mismatch".into());
                }
            }
            _ => return Err("unknown record or more than one process start".into()),
        }
    }
    if !complete || steps < 2 {
        return Err("missing completed computation".into());
    }
    Ok((steps, hex(&state)))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("run") => {
            let path = args.get(2).expect("run OUTPUT TARGET");
            let target: u64 = args
                .get(3)
                .expect("target")
                .parse()
                .expect("integer target");
            let mut out = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap();
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
            let target: u64 = args
                .get(3)
                .expect("verify OUTPUT TARGET [BEFORE]")
                .parse()
                .unwrap();
            let before = args.get(4).map(|p| std::fs::read_to_string(p).unwrap());
            let (steps, hash) =
                verify(&text, target, before.as_deref()).expect("recovery verification");
            println!("{{\"ok\":true,\"starts\":1,\"steps\":{steps},\"sequence\":{target},\"sha256\":\"{hash}\"}}");
        }
        _ => {
            panic!("usage: isolab-resume-fixture run OUTPUT TARGET | verify OUTPUT TARGET [BEFORE]")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn records() -> String {
        let a = advance([0; 32], 1);
        let b = advance(a, 2);
        format!(
            "start 88\nstep 1 {}\nstep 2 {}\ncomplete 2 {}\n",
            hex(&a),
            hex(&b),
            hex(&b)
        )
    }
    #[test]
    fn known_sha256_openssl_reference() {
        // Independently generated using OpenSSL SHA-256 over 32 zero bytes and LE64(1).
        assert_eq!(
            hex(&advance([0; 32], 1)),
            "19ea44be89eece0fd4ec7482049f472a11af19384bffb38a88e77b3b1dd54c19"
        );
    }
    #[test]
    fn accepts_continuation_with_preserved_prefix() {
        assert!(verify(&records(), 2, Some("start 88\n")).is_ok());
    }
    #[test]
    fn rejects_restart_corruption_and_truncation() {
        let good = records();
        assert!(verify(&format!("start 88\n{good}"), 2, None).is_err());
        assert!(verify(&good.replace("19ea44be", "29ea44be"), 2, None).is_err());
        assert!(verify(good.split("complete").next().unwrap(), 2, None).is_err());
        assert!(verify(&good, 3, None).is_err());
        assert!(verify(&good, 2, Some("start 99\n")).is_err());
        assert!(verify(&good, 2, Some(&good)).is_err());
        assert!(verify(&format!("{good}start 99\n"), 2, None).is_err());
    }
}
