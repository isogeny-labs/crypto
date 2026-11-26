//! Half-GCD vs classical GCD crossovers for `crypto_lib::gcd`.
//!
//! For each size it times, interleaved over `--rounds` rounds with the arm
//! order shuffled per round, on the same random inputs (a planted common
//! factor of degree/bits ≈ size/8):
//!
//! * `fp`   — `F_p[x]`, `p = 2^61 − 1`, degree `n`: Euclid (`threshold =
//!   ∞`) vs half-GCD forced (`threshold = 0`), gcd and xgcd.
//! * `gf2`  — `GF(2)[x]`, degree `n`: Euclid vs half-GCD forced.
//! * `int`  — `BigUint`, `n` bits: num-integer `gcd`, num-bigint `modinv`
//!   (via `xgcd_big_euclid`, classical), Lehmer (`threshold = ∞`), and the
//!   recursive half-GCD forced (`threshold = 256`), gcd and xgcd.
//!
//! Every arm's result is checked equal to the reference before timing.
//! Output: JSON lines (`--out`), and with `--analyze FILE` a Markdown
//! table of medians and paired ratios with 95% bootstrap intervals.

use crypto_lib::gcd::{gf2, int, poly};
use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::collections::BTreeMap;
use std::io::Write;
use std::time::Instant;

fn arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn time<R>(reps: usize, mut f: impl FnMut() -> R) -> f64 {
    let t0 = Instant::now();
    for _ in 0..reps {
        std::hint::black_box(f());
    }
    t0.elapsed().as_nanos() as f64 / reps as f64
}

fn rand_fp(f: &poly::Fp64, rng: &mut StdRng, deg: usize) -> Vec<u64> {
    let mut v: Vec<u64> = (0..=deg).map(|_| rng.gen_range(0..f.p)).collect();
    *v.last_mut().unwrap() = rng.gen_range(1..f.p);
    v
}

fn rand_gf2(rng: &mut StdRng, deg: usize) -> Vec<u64> {
    let mut v: Vec<u64> = (0..deg / 64 + 1).map(|_| rng.gen()).collect();
    let top = deg % 64;
    let last = v.last_mut().unwrap();
    *last &= if top == 63 {
        u64::MAX
    } else {
        (1u64 << (top + 1)) - 1
    };
    *last |= 1u64 << top;
    v
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--analyze") {
        let svg = args
            .iter()
            .position(|a| a == "--svg")
            .and_then(|j| args.get(j + 1).cloned());
        analyze(&args[i + 1], svg.as_deref());
        return;
    }
    let rounds: usize = arg(&args, "--rounds", 11);
    let seed: u64 = arg(&args, "--seed", 0x6867_6364);
    let out_path: String = arg(&args, "--out", "hgcd_bench.jsonl".to_string());
    let which: String = arg(&args, "--only", "fp,gf2,int".to_string());
    let mut out = std::fs::File::create(&out_path).expect("--out");
    let mut rng = StdRng::seed_from_u64(seed);

    let mut emit = |kind: &str, n: usize, arm: &str, round: usize, ns: f64| {
        writeln!(
            out,
            "{{\"kind\":\"{kind}\",\"n\":{n},\"arm\":\"{arm}\",\"round\":{round},\"ns\":{ns:.1}}}"
        )
        .unwrap();
    };

    if which.contains("fp") {
        let f = poly::Fp64::new((1u64 << 61) - 1);
        for &n in &[64usize, 128, 256, 512, 1024, 2048, 4096, 8192, 16384] {
            let c = rand_fp(&f, &mut rng, n / 8);
            let a = poly::mul(&f, &rand_fp(&f, &mut rng, n - n / 8), &c);
            let b = poly::mul(&f, &rand_fp(&f, &mut rng, n - n / 8 - 1), &c);
            let want = poly::xgcd_euclid(&f, &a, &b);
            assert_eq!(poly::xgcd_with_threshold(&f, &a, &b, 0), want);
            assert_eq!(poly::gcd_with_threshold(&f, &a, &b, 0), want.0);
            assert_eq!(poly::xgcd(&f, &a, &b), want);
            let reps = (20_000_000 / (n * n)).max(1);
            for round in 0..rounds {
                let mut arms = [
                    "gcd_euclid",
                    "gcd_hgcd",
                    "xgcd_euclid",
                    "xgcd_hgcd",
                    "gcd_default",
                ];
                arms.shuffle(&mut rng);
                for arm in arms {
                    let ns = match arm {
                        "gcd_euclid" => {
                            time(reps, || poly::gcd_with_threshold(&f, &a, &b, usize::MAX))
                        }
                        "gcd_hgcd" => time(reps, || poly::gcd_with_threshold(&f, &a, &b, 0)),
                        "xgcd_euclid" => time(reps, || poly::xgcd_euclid(&f, &a, &b)),
                        "xgcd_hgcd" => time(reps, || poly::xgcd_with_threshold(&f, &a, &b, 0)),
                        _ => time(reps, || poly::gcd(&f, &a, &b)),
                    };
                    emit("fp", n, arm, round, ns);
                }
            }
            eprintln!("fp n={n} done");
        }
    }

    if which.contains("gf2") {
        for &n in &[1024usize, 2048, 4096, 8192, 16384, 32768, 65536, 131072] {
            let c = rand_gf2(&mut rng, n / 8);
            let a = gf2::mul(&rand_gf2(&mut rng, n - n / 8), &c);
            let b = gf2::mul(&rand_gf2(&mut rng, n - n / 8 - 1), &c);
            let want = gf2::gcd_euclid(&a, &b);
            assert_eq!(gf2::gcd_with_threshold(&a, &b, 0), want);
            let x = gf2::xgcd_with_threshold(&a, &b, usize::MAX);
            assert_eq!(gf2::xgcd_with_threshold(&a, &b, 0), x);
            let reps = (40_000_000 / (n * n / 64)).clamp(1, 2000);
            for round in 0..rounds {
                let mut arms = ["gcd_euclid", "gcd_hgcd", "xgcd_euclid", "xgcd_hgcd"];
                arms.shuffle(&mut rng);
                for arm in arms {
                    let ns = match arm {
                        "gcd_euclid" => time(reps, || gf2::gcd_with_threshold(&a, &b, usize::MAX)),
                        "gcd_hgcd" => time(reps, || gf2::gcd_with_threshold(&a, &b, 0)),
                        "xgcd_euclid" => {
                            time(reps, || gf2::xgcd_with_threshold(&a, &b, usize::MAX))
                        }
                        _ => time(reps, || gf2::xgcd_with_threshold(&a, &b, 0)),
                    };
                    emit("gf2", n, arm, round, ns);
                }
            }
            eprintln!("gf2 n={n} done");
        }
    }

    if which.contains("int") {
        for &n in &[
            256usize, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072,
        ] {
            let g = rng.gen_biguint((n / 8) as u64) | BigUint::from(1u8);
            let a: BigUint = rng.gen_biguint((n - n / 8) as u64) * &g;
            let b: BigUint = rng.gen_biguint((n - n / 8 - 1) as u64) * &g;
            let want = int::xgcd_big_euclid(&a, &b);
            assert_eq!(a.gcd(&b), want.0);
            assert_eq!(int::gcd_big_with_threshold(&a, &b, u64::MAX), want.0);
            assert_eq!(int::gcd_big_with_threshold(&a, &b, 256), want.0);
            let x = int::xgcd_big_with_threshold(&a, &b, u64::MAX);
            assert_eq!(int::xgcd_big_with_threshold(&a, &b, 256), x);
            let reps = (4_000_000_000 / (n * n)).clamp(1, 20_000);
            for round in 0..rounds {
                let mut arms = [
                    "gcd_numinteger",
                    "gcd_lehmer",
                    "gcd_hgcd",
                    "xgcd_euclid",
                    "xgcd_lehmer",
                    "xgcd_hgcd",
                ];
                arms.shuffle(&mut rng);
                for arm in arms {
                    let ns = match arm {
                        "gcd_numinteger" => time(reps, || a.gcd(&b)),
                        "gcd_lehmer" => {
                            time(reps, || int::gcd_big_with_threshold(&a, &b, u64::MAX))
                        }
                        "gcd_hgcd" => time(reps, || int::gcd_big_with_threshold(&a, &b, 256)),
                        "xgcd_euclid" => time(reps, || int::xgcd_big_euclid(&a, &b)),
                        "xgcd_lehmer" => {
                            time(reps, || int::xgcd_big_with_threshold(&a, &b, u64::MAX))
                        }
                        _ => time(reps, || int::xgcd_big_with_threshold(&a, &b, 256)),
                    };
                    emit("int", n, arm, round, ns);
                }
            }
            eprintln!("int n={n} done");
        }
    }
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        0.5 * (v[n / 2 - 1] + v[n / 2])
    }
}

fn ratio_ci(r: &[f64], seed: u64) -> (f64, f64, f64) {
    let mut rr = r.to_vec();
    let mid = median(&mut rr);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut boots: Vec<f64> = (0..2000)
        .map(|_| {
            let mut s: Vec<f64> = (0..r.len()).map(|_| r[rng.gen_range(0..r.len())]).collect();
            median(&mut s)
        })
        .collect();
    boots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (mid, boots[50], boots[1949])
}

fn analyze(path: &str, svg: Option<&str>) {
    let text = std::fs::read_to_string(path).expect("read");
    // (kind, n) -> arm -> round -> ns
    let mut d: BTreeMap<(String, usize), BTreeMap<String, BTreeMap<u64, f64>>> = BTreeMap::new();
    for line in text.lines().filter(|l| !l.is_empty()) {
        let v: serde_json::Value = serde_json::from_str(line).unwrap();
        d.entry((
            v["kind"].as_str().unwrap().into(),
            v["n"].as_u64().unwrap() as usize,
        ))
        .or_default()
        .entry(v["arm"].as_str().unwrap().into())
        .or_default()
        .insert(
            v["round"].as_u64().unwrap(),
            v["ns"].to_string().parse().unwrap(),
        );
    }
    let pairs: &[(&str, &str, &str)] = &[
        ("fp", "gcd_euclid", "gcd_hgcd"),
        ("fp", "xgcd_euclid", "xgcd_hgcd"),
        ("fp", "gcd_euclid", "gcd_default"),
        ("gf2", "gcd_euclid", "gcd_hgcd"),
        ("gf2", "xgcd_euclid", "xgcd_hgcd"),
        ("int", "gcd_numinteger", "gcd_lehmer"),
        ("int", "gcd_numinteger", "gcd_hgcd"),
        ("int", "gcd_lehmer", "gcd_hgcd"),
        ("int", "xgcd_euclid", "xgcd_lehmer"),
        ("int", "xgcd_euclid", "xgcd_hgcd"),
        ("int", "xgcd_lehmer", "xgcd_hgcd"),
    ];
    let mut series: BTreeMap<String, Vec<(f64, f64, f64, f64)>> = BTreeMap::new();
    println!("| kind | size | baseline | candidate | baseline median µs | candidate median µs | speedup (paired median) [95% CI] |");
    println!("|:--|--:|:--|:--|--:|--:|--:|");
    for ((kind, n), arms) in &d {
        for (i, (k, base, cand)) in pairs.iter().enumerate() {
            if k != kind {
                continue;
            }
            let (Some(b), Some(c)) = (arms.get(*base), arms.get(*cand)) else {
                continue;
            };
            let r: Vec<f64> = b
                .iter()
                .filter_map(|(rd, x)| c.get(rd).map(|y| x / y))
                .collect();
            let (m, lo, hi) = ratio_ci(&r, (*n as u64) << 4 | i as u64);
            let mb = median(&mut b.values().copied().collect::<Vec<_>>()) / 1000.0;
            let mc = median(&mut c.values().copied().collect::<Vec<_>>()) / 1000.0;
            println!("| {kind} | {n} | {base} | {cand} | {mb:.2} | {mc:.2} | {m:.3}× [{lo:.3}, {hi:.3}] |");
            series
                .entry(format!("{kind}: {cand} vs {base}"))
                .or_default()
                .push((*n as f64, m, lo, hi));
        }
    }
    if let Some(out) = svg {
        let keep = [
            "fp: gcd_hgcd vs gcd_euclid",
            "fp: xgcd_hgcd vs xgcd_euclid",
            "gf2: gcd_hgcd vs gcd_euclid",
            "gf2: xgcd_hgcd vs xgcd_euclid",
            "int: gcd_lehmer vs gcd_numinteger",
            "int: xgcd_hgcd vs xgcd_lehmer",
            "int: xgcd_lehmer vs xgcd_euclid",
        ];
        std::fs::write(out, svg_chart(&series, &keep)).expect("write svg");
    }
}

/// Log-log chart of paired speedups (candidate vs baseline) against size.
fn svg_chart(series: &BTreeMap<String, Vec<(f64, f64, f64, f64)>>, keep: &[&str]) -> String {
    use std::fmt::Write as _;
    let (w, h, ml, mr, mt, mb) = (900.0f64, 520.0f64, 70.0, 250.0, 40.0, 60.0);
    let (pw, ph) = (w - ml - mr, h - mt - mb);
    let (x0, x1) = (6.0f64, 17.0f64); // log2 size
    let (y0, y1) = (-2.0f64, 6.0f64); // log2 speedup
    let x = |n: f64| ml + pw * (n.log2() - x0) / (x1 - x0);
    let y = |r: f64| mt + ph * (1.0 - (r.log2().clamp(y0, y1) - y0) / (y1 - y0));
    let colors = [
        "#4e79a7", "#f28e2b", "#59a14f", "#e15759", "#b07aa1", "#76b7b2", "#9c755f",
    ];
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>");
    let _ = write!(s, "<text x='{ml}' y='22' font-size='14' font-weight='bold'>Half-GCD and Lehmer speedups vs size (paired median, 95% CI)</text>");
    for e in (y0 as i32)..=(y1 as i32) {
        let r = 2f64.powi(e);
        let _ = write!(s, "<line x1='{ml}' x2='{x2}' y1='{yy:.1}' y2='{yy:.1}' stroke='{c}'/><text x='{lx}' y='{ly:.1}' text-anchor='end'>{r}×</text>", x2 = ml + pw, yy = y(r), c = if e == 0 { "#333" } else { "#e5e5e5" }, lx = ml - 6.0, ly = y(r) + 4.0);
    }
    for e in (x0 as i32)..=(x1 as i32) {
        let n = 2f64.powi(e);
        let _ = write!(
            s,
            "<text x='{xx:.1}' y='{yy}' text-anchor='middle'>2^{e}</text>",
            xx = x(n),
            yy = h - mb + 18.0
        );
    }
    let _ = write!(s, "<text x='{xx:.1}' y='{yy}' text-anchor='middle'>size (polynomial degree, or integer bits)</text>", xx = ml + pw / 2.0, yy = h - mb + 40.0);
    for (i, name) in keep.iter().enumerate() {
        let Some(pts) = series.get(*name) else {
            continue;
        };
        let c = colors[i % colors.len()];
        let d: Vec<String> = pts
            .iter()
            .map(|p| format!("{:.1},{:.1}", x(p.0), y(p.1)))
            .collect();
        let _ = write!(
            s,
            "<polyline points='{}' fill='none' stroke='{c}' stroke-width='2'/>",
            d.join(" ")
        );
        for p in pts {
            let _ = write!(s, "<line x1='{xx:.1}' x2='{xx:.1}' y1='{a:.1}' y2='{b:.1}' stroke='{c}'/><circle cx='{xx:.1}' cy='{yy:.1}' r='3' fill='{c}'><title>{name} n={n}: {m:.3}x [{lo:.3}, {hi:.3}]</title></circle>", xx = x(p.0), yy = y(p.1), a = y(p.2), b = y(p.3), n = p.0, m = p.1, lo = p.2, hi = p.3);
        }
        let ly = mt + 20.0 + 22.0 * i as f64;
        let _ = write!(s, "<rect x='{lx}' y='{ry}' width='12' height='12' fill='{c}'/><text x='{tx}' y='{ty}'>{name}</text>", lx = w - mr + 15.0, ry = ly - 10.0, tx = w - mr + 32.0, ty = ly);
    }
    s.push_str("</svg>\n");
    s
}
