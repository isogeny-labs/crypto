//! Matched native (`poly_roots_fp`) vs FLINT `nmod_poly_factor` univariate
//! root extraction for the #5283 adaptive rootfinder path.
//!
//! L0 diagnostic only: single-host, unisolated wall times. Root sets must
//! agree exactly. Magma is named missing when absent. Iso Lab / claim-grade
//! timing is out of scope for this binary.
//!
//! ```text
//! cargo run --release --example rootfinder_external_qualify -- \
//!   --flint research/rootfinder_qualify_20261128/flint_helper/root_nmod \
//!   --out research/rootfinder_qualify_20261128/raw/l0-panel.jsonl
//! ```

use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Serialize;

#[derive(Serialize)]
struct Row {
    p: u64,
    degree: u32,
    seed: u64,
    coeffs: Vec<u64>,
    native_roots: Vec<u64>,
    flint_roots: Vec<u64>,
    roots_agree: bool,
    native_muls: u64,
    native_ns: u64,
    flint_ns: u64,
    native_over_flint: f64,
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn next_prime(mut n: u64) -> u64 {
    if n <= 2 {
        return 2;
    }
    if n % 2 == 0 {
        n += 1;
    }
    while !is_prime(n) {
        n = n.saturating_add(2);
    }
    n
}

fn build_split_poly(p: u64, degree: u32, seed: u64) -> (Vec<u64>, Vec<u64>) {
    // Product (x − r_i) over distinct random roots, monic, dense low-to-high.
    let mut rng = StdRng::seed_from_u64(seed ^ p ^ u64::from(degree) << 32);
    let mut roots = Vec::new();
    while roots.len() < degree as usize {
        let r = rng.gen_range(0..p);
        if !roots.contains(&r) {
            roots.push(r);
        }
    }
    roots.sort_unstable();
    let mut coeffs = vec![1u64]; // 1
    for &r in &roots {
        // multiply by (x − r) = (−r) + x
        let mut next = vec![0u64; coeffs.len() + 1];
        for (i, &c) in coeffs.iter().enumerate() {
            let neg_r_c = ((u128::from(p - (r % p)) * u128::from(c)) % u128::from(p)) as u64;
            next[i] = (next[i] + neg_r_c) % p;
            next[i + 1] = (next[i + 1] + c) % p;
        }
        coeffs = next;
    }
    (coeffs, roots)
}

struct FlintSession {
    child: std::process::Child,
    stdin: std::process::ChildStdin,
    stdout: std::io::BufReader<std::process::ChildStdout>,
}

impl FlintSession {
    fn open(helper: &Path) -> Result<Self, String> {
        let mut child = Command::new(helper)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("spawn flint helper: {e}"))?;
        let stdin = child.stdin.take().ok_or("no stdin")?;
        let stdout = child.stdout.take().ok_or("no stdout")?;
        Ok(Self {
            child,
            stdin,
            stdout: std::io::BufReader::new(stdout),
        })
    }

    fn roots(&mut self, p: u64, coeffs: &[u64]) -> Result<(Vec<u64>, u64), String> {
        let deg = coeffs.len().saturating_sub(1);
        let mut line = format!("{p} {deg}");
        for c in coeffs {
            line.push(' ');
            line.push_str(&c.to_string());
        }
        line.push('\n');
        self.stdin
            .write_all(line.as_bytes())
            .map_err(|e| format!("write flint: {e}"))?;
        self.stdin.flush().map_err(|e| format!("flush flint: {e}"))?;
        let mut resp = String::new();
        self.stdout
            .read_line(&mut resp)
            .map_err(|e| format!("read flint: {e}"))?;
        let mut parts = resp.split_whitespace();
        let tag = parts.next().ok_or("flint missing tag")?;
        if tag != "ok" {
            return Err(format!("flint response: {resp}"));
        }
        let ns: u64 = parts
            .next()
            .ok_or("flint missing ns")?
            .parse()
            .map_err(|e| format!("flint ns: {e}"))?;
        let mut roots = Vec::new();
        for tok in parts {
            roots.push(tok.parse::<u64>().map_err(|e| format!("flint root: {e}"))?);
        }
        Ok((roots, ns))
    }
}

impl Drop for FlintSession {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn median(xs: &mut [f64]) -> f64 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = xs.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        0.5 * (xs[n / 2 - 1] + xs[n / 2])
    }
}

fn main() {
    let mut flint = PathBuf::from("research/rootfinder_qualify_20261128/flint_helper/root_nmod");
    let mut out_path: Option<PathBuf> = None;
    let mut reps = 7usize;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--flint" => {
                i += 1;
                flint = PathBuf::from(&args[i]);
            }
            "--out" => {
                i += 1;
                out_path = Some(PathBuf::from(&args[i]));
            }
            "--reps" => {
                i += 1;
                reps = args[i].parse().expect("--reps");
            }
            other => panic!("unknown arg {other}"),
        }
        i += 1;
    }
    assert!(
        flint.is_file(),
        "FLINT helper missing at {}: build with gcc -O2 -o root_nmod root_nmod.c -lflint -lgmp -lmpfr -lm",
        flint.display()
    );

    let magma_present = Command::new("magma")
        .arg("-b")
        .arg("-e")
        .arg("print \"ok\"; quit;")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    // p = 2^31-1 is included only at deg <= 3: higher degrees are for the
    // Iso Lab panel (PROTOCOL.md), not this L0 host diagnostic.
    let primes: Vec<u64> = [17u64, 31, 101, 1009, 65537, (1u64 << 31) - 1]
        .into_iter()
        .map(next_prime)
        .collect();
    let degrees = [2u32, 3, 4, 5, 6];
    let seeds: Vec<u64> = (1..=reps as u64).collect();

    let mut flint_sess = FlintSession::open(&flint).expect("open flint session");
    // Warm the helper once so the first timed cell is not cold-start only.
    {
        let (c, _) = build_split_poly(17, 2, 0);
        let _ = flint_sess.roots(17, &c);
        let mut muls = 0u64;
        let _ = poly_roots_fp_counted(&c, 17, 0, &mut muls);
    }

    let mut rows = Vec::new();
    let mut disagree = 0usize;
    for &p in &primes {
        for &degree in &degrees {
            if degree >= p as u32 {
                continue;
            }
            if p >= (1u64 << 30) && degree > 3 {
                continue;
            }
            for &seed in &seeds {
                let (coeffs, planted) = build_split_poly(p, degree, seed);

                let mut muls = 0u64;
                let t0 = Instant::now();
                let native = poly_roots_fp_counted(&coeffs, p, seed, &mut muls);
                let native_ns = t0.elapsed().as_nanos() as u64;

                let (flint_roots, flint_ns) = flint_sess
                    .roots(p, &coeffs)
                    .unwrap_or_else(|e| panic!("flint failed p={p} deg={degree} seed={seed}: {e}"));

                let agree = native == flint_roots && native == planted;
                if !agree {
                    disagree += 1;
                    eprintln!(
                        "DISAGREE p={p} deg={degree} seed={seed} planted={planted:?} native={native:?} flint={flint_roots:?}"
                    );
                }
                let ratio = if flint_ns == 0 {
                    f64::INFINITY
                } else {
                    native_ns as f64 / flint_ns as f64
                };
                rows.push(Row {
                    p,
                    degree,
                    seed,
                    coeffs,
                    native_roots: native,
                    flint_roots,
                    roots_agree: agree,
                    native_muls: muls,
                    native_ns,
                    flint_ns,
                    native_over_flint: ratio,
                });
            }
        }
    }

    let mut ratios: Vec<f64> = rows
        .iter()
        .filter(|r| r.roots_agree && r.flint_ns > 0)
        .map(|r| r.native_over_flint)
        .collect();
    let med = median(&mut ratios);
    let n_ok = rows.iter().filter(|r| r.roots_agree).count();

    println!("host_l0_unisolated");
    println!("native=poly_roots_fp_counted (glv_invariant_base / #5283 path)");
    println!("external=FLINT nmod_poly_factor ({})", {
        let v = Command::new("pkg-config")
            .args(["--modversion", "flint"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_else(|| "unknown".into());
        v.trim().to_string()
    });
    println!("magma_present={magma_present}");
    println!("rows={} agree={} disagree={}", rows.len(), n_ok, disagree);
    println!("median_native_ns_over_flint_ns={med:.4}");
    println!("note=wall_ratio_L0_only; not claim-grade; Magma absent unless magma_present");

    // Per (p, degree) medians
    println!("panel_medians:");
    for &p in &primes {
        for &degree in &degrees {
            let mut cell: Vec<f64> = rows
                .iter()
                .filter(|r| r.p == p && r.degree == degree && r.roots_agree && r.flint_ns > 0)
                .map(|r| r.native_over_flint)
                .collect();
            if cell.is_empty() {
                continue;
            }
            let m = median(&mut cell);
            println!("  p={p} deg={degree} n={} median_native/flint={m:.4}", cell.len());
        }
    }

    if let Some(path) = out_path {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("mkdir out");
        }
        let mut f = std::fs::File::create(&path).expect("create out");
        for row in &rows {
            serde_json::to_writer(&mut f, row).expect("json");
            f.write_all(b"\n").unwrap();
        }
        println!("wrote {}", path.display());
    }

    // Also echo FLINT helper identity for the receipt.
    let helper_sha = {
        let bytes = std::fs::read(&flint).expect("read helper");
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(&bytes);
        format!("{:x}", h.finalize())
    };
    println!("flint_helper_sha256={helper_sha}");

    assert_eq!(disagree, 0, "root sets must agree with planted and FLINT");
}
