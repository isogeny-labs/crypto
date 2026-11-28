//! Multiplication ablation for generic-prime ECC scalar multiplication
//! (`Point::scalar_mul` on the fixed-width Montgomery path, `mont_vt`).
//!
//! Two measurements, written as JSON lines to `--out`:
//!
//! * `kernel` rows: latency of one dependent modular multiplication or
//!   squaring per field width (CIOS product, CIOS `a·a`, dedicated
//!   squaring, one-level Karatsuba product, `num-bigint`).
//! * `scalar` rows: nanoseconds per scalar multiplication for every arm of
//!   `point::scalar_mul_arm` on every field, rounds interleaved with a
//!   per-round shuffled arm order (derived seed), the same scalars for
//!   every arm within a round, and an A/A copy of the baseline arm.
//!
//! Before timing, every arm's point is checked against the `num-bigint`
//! ladder on edge and random scalars; any mismatch aborts the run.
//!
//! Usage: `mont_mul_bench [--rounds R] [--batch B] [--seed S] [--out FILE]
//!         [--fields a,b,..] [--kernel-iters I]`
//!
//! `--batch`: eight `Point::scalar_mul` calls vs one
//! `Point::scalar_mul_batch` (AVX-512 IFMA, eight lanes) on the same points
//! and scalars, interleaved rounds, `batch` rows (ns per point).
//!
//! `--rho`: marginal cost per step of the `ecdlp_nist` rho walk, one walk
//! per worker (`lanes = 1`) vs lockstep walks (`--lanes`, batched on IFMA),
//! `rho` rows.

use crypto_lib::ecc::endo_generic_chain_set::is_probable_prime;
use crypto_lib::ecc::field::FieldElement;
use crypto_lib::ecc::point::{field_kernel_probe, scalar_mul_arm, Point};
use num_bigint::{BigUint, RandBigInt};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::io::Write;

const ARMS: &[&str] = &[
    "pad9_cios",
    "cios",
    "sqr",
    "karatsuba",
    "sqr_bingcd",
    "sqr_wnaf4",
    "sqr_wnaf5",
    "tuned",
    "tuned_wnaf",
    "adx",
    "adx_sqr",
    "mulx",
    "adx_wnaf",
    "adx6",
    "adx6_mul",
    "adx_il",
    "adx_qp",
    "adx_tw",
    "ifma1",
];
const BASELINE: &str = "pad9_cios";
const AA: &str = "pad9_cios#aa";
const KERNEL_OPS: &[&str] = &[
    "cios_mul",
    "adx_mul",
    "adx6_mul",
    "adx_il_mul",
    "adx_qp_mul",
    "adx_tw_mul",
    "ifma1_mul",
    "adx8_mul",
    "ifma8_mul",
    "mulx_mul",
    "sos_mul",
    "karatsuba_mul",
    "cios_sqr",
    "sqr",
    "inv_fermat",
    "inv_bingcd",
    "biguint_mul",
];

fn two(e: u32) -> BigUint {
    BigUint::from(1u8) << e
}

/// The largest prime below `2^bits`.
fn prime_below_pow2(bits: u32) -> BigUint {
    let mut c = two(bits) - 1u32;
    while !is_probable_prime(&c) {
        c -= 2u32;
    }
    c
}

fn fields() -> Vec<(&'static str, BigUint)> {
    vec![
        ("p127_mersenne", two(127) - 1u32),
        ("p192_nist", two(192) - two(64) - 1u32),
        ("p255_25519", two(255) - 19u32),
        ("p320_pmax", prime_below_pow2(320)),
        ("p352_pmax", prime_below_pow2(352)),
        ("p384_nist", two(384) - two(128) - two(96) + two(32) - 1u32),
        ("p448_goldilocks", two(448) - two(224) - 1u32),
        ("p512_pmax", prime_below_pow2(512)),
        ("p521_mersenne", two(521) - 1u32),
    ]
}

fn arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--analyze") {
        let input = args.get(i + 1).expect("--analyze FILE");
        let dir: String = arg(&args, "--report-dir", ".".to_string());
        analyze::run(input, &dir);
        return;
    }
    if args.iter().any(|a| a == "--rho") {
        rho_mode(&args);
        return;
    }
    if args.iter().any(|a| a == "--batch") {
        batch_mode(&args);
        return;
    }
    let rounds: usize = arg(&args, "--rounds", 15);
    let batch: usize = arg(&args, "--batch", 40);
    let seed: u64 = arg(&args, "--seed", 0x6d6f_6e74_6d75_6c31);
    let kernel_iters: u64 = arg(&args, "--kernel-iters", 2_000_000);
    let out_path: String = arg(&args, "--out", "mont_mul_bench.jsonl".to_string());
    let only: Option<String> = args
        .iter()
        .position(|a| a == "--fields")
        .and_then(|i| args.get(i + 1).cloned());
    let mut out = std::fs::File::create(&out_path).expect("create --out");

    let fields: Vec<_> = fields()
        .into_iter()
        .filter(|(n, _)| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *n)))
        .collect();

    for (fi, (name, p)) in fields.iter().enumerate() {
        let limbs = p.bits().div_ceil(64);
        let mut rng = StdRng::seed_from_u64(seed ^ (fi as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        let a = FieldElement::new(rng.gen_biguint_below(p), p.clone());
        let base = Point::Affine {
            x: FieldElement::new(rng.gen_biguint_below(p), p.clone()),
            y: FieldElement::new(rng.gen_biguint_below(p), p.clone()),
        };

        // ── Correctness gate ────────────────────────────────────────────
        let mut scalars: Vec<BigUint> = [0u32, 1, 2, 3, 7, 8, 15, 16, 17, 255, 256]
            .iter()
            .map(|&v| BigUint::from(v))
            .collect();
        scalars.push(p - 1u32);
        scalars.push(two(p.bits() as u32) - 1u32);
        scalars.push(two(p.bits() as u32 - 1));
        for _ in 0..24 {
            scalars.push(rng.gen_biguint(p.bits()));
        }
        for k in &scalars {
            let want = scalar_mul_arm("biguint", &base, k, &a).unwrap();
            assert_eq!(base.scalar_mul(k, &a), want, "production {name} k={k}");
            assert_eq!(
                base.scalar_mul_vartime(k, &a),
                want,
                "production vartime {name} k={k}"
            );
            for arm in ARMS {
                let got = scalar_mul_arm(arm, &base, k, &a).expect("arm covers field");
                assert_eq!(got, want, "arm {arm} field {name} k={k}");
            }
        }
        eprintln!(
            "{name}: {limbs} limbs, {} scalars x {} arms agree with num-bigint",
            scalars.len(),
            ARMS.len()
        );

        // ── Kernel latency ─────────────────────────────────────────────
        let (ka, kb) = (rng.gen_biguint_below(p), rng.gen_biguint_below(p));
        for rep in 0..7 {
            for op in KERNEL_OPS {
                let iters = match *op {
                    "biguint_mul" => kernel_iters / 10,
                    "inv_fermat" | "inv_bingcd" => kernel_iters / 1000,
                    _ => kernel_iters,
                };
                // `adx_mul` / `adx6_mul` exist only at 4 / 6 limbs (and `*_mulx` only with ADX).
                let Some((ns, _)) = field_kernel_probe(op, p, &ka, &kb, iters) else {
                    continue;
                };
                writeln!(
                    out,
                    "{{\"kind\":\"kernel\",\"field\":\"{name}\",\"limbs\":{limbs},\"op\":\"{op}\",\"rep\":{rep},\"iters\":{iters},\"ns_per_op\":{ns:.3}}}"
                )
                .unwrap();
            }
        }

        // ── Scalar multiplication, interleaved ─────────────────────────
        let mut arms: Vec<&str> = ARMS.to_vec();
        arms.push(AA);
        arms.push("biguint");
        arms.push("prod_vartime");
        arms.push("prod");
        for round in 0..rounds {
            let ks: Vec<BigUint> = (0..batch).map(|_| rng.gen_biguint(p.bits())).collect();
            let mut order = arms.clone();
            let mut orng = StdRng::seed_from_u64(seed ^ ((fi as u64) << 32) ^ round as u64);
            order.shuffle(&mut orng);
            for (pos, arm) in order.iter().enumerate() {
                let real = arm.trim_end_matches("#aa");
                // biguint is ~10-50x slower: time a fifth of the batch.
                let n = if real == "biguint" {
                    (batch / 5).max(1)
                } else {
                    batch
                };
                let t0 = std::time::Instant::now();
                if real == "prod" {
                    for k in &ks[..n] {
                        std::hint::black_box(std::hint::black_box(&base).scalar_mul(k, &a));
                    }
                } else if real == "prod_vartime" {
                    for k in &ks[..n] {
                        std::hint::black_box(std::hint::black_box(&base).scalar_mul_vartime(k, &a));
                    }
                } else {
                    for k in &ks[..n] {
                        std::hint::black_box(scalar_mul_arm(
                            real,
                            std::hint::black_box(&base),
                            k,
                            &a,
                        ));
                    }
                }
                let ns = t0.elapsed().as_nanos() as f64 / n as f64;
                writeln!(
                    out,
                    "{{\"kind\":\"scalar\",\"field\":\"{name}\",\"limbs\":{limbs},\"bits\":{},\"arm\":\"{arm}\",\"round\":{round},\"order_pos\":{pos},\"n\":{n},\"ns_per_op\":{ns:.1}}}",
                    p.bits()
                )
                .unwrap();
            }
        }
        eprintln!("{name}: {rounds} rounds done");
    }
}

mod analyze {
    //! Summary of a `mont_mul_bench` JSON-lines file: medians, paired
    //! per-round speedups against the baseline arm with percentile-bootstrap
    //! 95% intervals, the A/A noise floor, and two SVG graphs.
    use super::{AA, ARMS, BASELINE, KERNEL_OPS};
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use serde_json::Value;
    use std::collections::BTreeMap;
    use std::fmt::Write as _;

    fn median(v: &mut [f64]) -> f64 {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = v.len();
        if n % 2 == 1 {
            v[n / 2]
        } else {
            0.5 * (v[n / 2 - 1] + v[n / 2])
        }
    }

    /// Median of paired ratios with a 2000-resample percentile bootstrap.
    fn ratio_ci(ratios: &[f64], seed: u64) -> (f64, f64, f64) {
        let mut r = ratios.to_vec();
        let mid = median(&mut r);
        let mut rng = StdRng::seed_from_u64(seed);
        let mut boots: Vec<f64> = (0..2000)
            .map(|_| {
                let mut s: Vec<f64> = (0..ratios.len())
                    .map(|_| ratios[rng.gen_range(0..ratios.len())])
                    .collect();
                median(&mut s)
            })
            .collect();
        boots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        (mid, boots[50], boots[1949])
    }

    pub fn run(input: &str, dir: &str) {
        let text = std::fs::read_to_string(input).expect("read input");
        // field -> (limbs, arm -> round -> ns)
        let mut scal: BTreeMap<(u64, String), BTreeMap<String, BTreeMap<u64, f64>>> =
            BTreeMap::new();
        let mut kern: BTreeMap<(u64, String), BTreeMap<String, Vec<f64>>> = BTreeMap::new();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).expect("json line");
            let field = v["field"].as_str().unwrap().to_string();
            let limbs = v["limbs"].as_u64().unwrap();
            let ns: f64 = v["ns_per_op"].to_string().parse().unwrap();
            match v["kind"].as_str().unwrap() {
                "scalar" => {
                    scal.entry((limbs, field))
                        .or_default()
                        .entry(v["arm"].as_str().unwrap().to_string())
                        .or_default()
                        .insert(v["round"].as_u64().unwrap(), ns);
                }
                _ => kern
                    .entry((limbs, field))
                    .or_default()
                    .entry(v["op"].as_str().unwrap().to_string())
                    .or_default()
                    .push(ns),
            }
        }

        let mut arms: Vec<&str> = vec![AA];
        arms.extend(ARMS.iter().filter(|a| **a != BASELINE));
        arms.push("prod");
        arms.push("prod_vartime");
        arms.push("biguint");
        let mut md = String::new();
        let mut json_rows = Vec::new();
        writeln!(md, "## Scalar multiplication: median ns per k·P, and speedup = baseline / arm (paired per round, median, 95% bootstrap CI)\n").unwrap();
        write!(md, "| field | limbs | {BASELINE} ns |").unwrap();
        for a in &arms {
            write!(md, " {a} |").unwrap();
        }
        md.push_str("\n|:--|--:|--:|");
        for _ in &arms {
            md.push_str("--:|");
        }
        md.push('\n');
        // (field, arm, mid, lo, hi) for the graph
        let mut bars: Vec<(String, u64, String, f64, f64, f64)> = Vec::new();
        for (si, ((limbs, field), by_arm)) in scal.iter().enumerate() {
            let base = &by_arm[BASELINE];
            let mut bvals: Vec<f64> = base.values().copied().collect();
            let bmed = median(&mut bvals);
            write!(md, "| {field} | {limbs} | {bmed:.0} |").unwrap();
            for (ai, a) in arms.iter().enumerate() {
                let Some(series) = by_arm.get(*a) else {
                    md.push_str(" – |");
                    continue;
                };
                let ratios: Vec<f64> = series
                    .iter()
                    .filter_map(|(r, ns)| base.get(r).map(|b| b / ns))
                    .collect();
                let (mid, lo, hi) = ratio_ci(&ratios, 0x5eed ^ ((si as u64) << 8) ^ ai as u64);
                let mut av: Vec<f64> = series.values().copied().collect();
                let amed = median(&mut av);
                write!(md, " {mid:.3}× [{lo:.3}, {hi:.3}] |").unwrap();
                json_rows.push(format!(
                    "{{\"field\":\"{field}\",\"limbs\":{limbs},\"arm\":\"{a}\",\"median_ns\":{amed:.1},\"baseline_median_ns\":{bmed:.1},\"speedup_median\":{mid:.4},\"ci95_low\":{lo:.4},\"ci95_high\":{hi:.4},\"rounds\":{}}}",
                    ratios.len()
                ));
                bars.push((field.clone(), *limbs, a.to_string(), mid, lo, hi));
            }
            md.push('\n');
        }

        writeln!(
            md,
            "\n## Field kernel latency: median ns per dependent operation\n"
        )
        .unwrap();
        write!(md, "| field | limbs |").unwrap();
        for op in KERNEL_OPS {
            write!(md, " {op} |").unwrap();
        }
        md.push_str(" sqr / cios_sqr | karatsuba / cios_mul | sos / cios_mul | adx / cios_mul | adx6 / cios_mul | mulx / cios_mul | adx6 / mulx | il / adx | il / adx6 | qp / adx | qp / adx6 | tw / adx | tw / adx6 | ifma1 / adx | ifma1 / adx6 | ifma8 / adx8 | bingcd / fermat |\n|:--|--:|");
        for _ in 0..KERNEL_OPS.len() + 17 {
            md.push_str("--:|");
        }
        md.push('\n');
        let mut klines: BTreeMap<String, Vec<(u64, f64)>> = BTreeMap::new();
        for ((limbs, field), ops) in &kern {
            write!(md, "| {field} | {limbs} |").unwrap();
            let mut meds = BTreeMap::new();
            for op in KERNEL_OPS {
                let Some(v) = ops.get(*op) else {
                    md.push_str(" – |");
                    continue;
                };
                let mut v = v.clone();
                let m = median(&mut v);
                meds.insert(*op, m);
                write!(md, " {m:.2} |").unwrap();
                if !op.starts_with("inv_") {
                    klines.entry(op.to_string()).or_default().push((*limbs, m));
                }
                json_rows.push(format!(
                    "{{\"field\":\"{field}\",\"limbs\":{limbs},\"kernel_op\":\"{op}\",\"median_ns\":{m:.3}}}"
                ));
            }
            let ratio = |a: &str, b: &str| match (meds.get(a), meds.get(b)) {
                (Some(x), Some(y)) => format!("{:.3}", x / y),
                _ => "–".to_string(),
            };
            writeln!(
                md,
                " {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                ratio("sqr", "cios_sqr"),
                ratio("karatsuba_mul", "cios_mul"),
                ratio("sos_mul", "cios_mul"),
                ratio("adx_mul", "cios_mul"),
                ratio("adx6_mul", "cios_mul"),
                ratio("mulx_mul", "cios_mul"),
                ratio("adx6_mul", "mulx_mul"),
                ratio("adx_il_mul", "adx_mul"),
                ratio("adx_il_mul", "adx6_mul"),
                ratio("adx_qp_mul", "adx_mul"),
                ratio("adx_qp_mul", "adx6_mul"),
                ratio("adx_tw_mul", "adx_mul"),
                ratio("adx_tw_mul", "adx6_mul"),
                ratio("ifma1_mul", "adx_mul"),
                ratio("ifma1_mul", "adx6_mul"),
                ratio("ifma8_mul", "adx8_mul"),
                ratio("inv_bingcd", "inv_fermat")
            )
            .unwrap();
        }
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(format!("{dir}/summary.md"), &md).unwrap();
        std::fs::write(
            format!("{dir}/summary.json"),
            format!("[\n{}\n]\n", json_rows.join(",\n")),
        )
        .unwrap();
        std::fs::write(format!("{dir}/speedup.svg"), speedup_svg(&bars)).unwrap();
        std::fs::write(format!("{dir}/kernels.svg"), kernel_svg(&klines)).unwrap();
        print!("{md}");
    }

    const COLORS: [&str; 11] = [
        "#9aa0a6", "#4e79a7", "#f28e2b", "#e15759", "#76b7b2", "#59a14f", "#edc948", "#b07aa1",
        "#ff9da7", "#9c755f", "#2f4b7c",
    ];

    fn speedup_svg(bars: &[(String, u64, String, f64, f64, f64)]) -> String {
        let arms: Vec<String> = {
            let mut a: Vec<String> = Vec::new();
            for b in bars {
                if b.2 != "biguint" && !a.contains(&b.2) {
                    a.push(b.2.clone());
                }
            }
            a
        };
        let fields: Vec<(String, u64)> = {
            let mut f: Vec<(String, u64)> = Vec::new();
            for b in bars {
                if !f.iter().any(|x| x.0 == b.0) {
                    f.push((b.0.clone(), b.1));
                }
            }
            f
        };
        let legend_rows = arms.len().div_ceil(6) as f64;
        let (w, ml, mt) = (980.0, 60.0, 40.0);
        let mb = 70.0 + 20.0 * legend_rows;
        let h = 370.0 + mb;
        let ymax = bars
            .iter()
            .filter(|b| b.2 != "biguint")
            .map(|b| b.5)
            .fold(1.0f64, f64::max)
            * 1.08;
        let pw = w - ml - 20.0;
        let ph = h - mb - mt;
        let gw = pw / fields.len() as f64;
        let bw = gw * 0.8 / arms.len() as f64;
        let y = |v: f64| mt + ph * (1.0 - v / ymax);
        let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>");
        write!(s, "<text x='{ml}' y='22' font-size='14' font-weight='bold'>Scalar multiplication speedup vs pre-change mont_vt (pad9_cios); median, 95% bootstrap CI</text>").unwrap();
        let mut t = 0.0;
        while t <= ymax {
            write!(s, "<line x1='{ml}' x2='{x2}' y1='{yy:.1}' y2='{yy:.1}' stroke='#e0e0e0'/><text x='{lx}' y='{ly:.1}' text-anchor='end'>{t:.1}×</text>", x2 = ml + pw, yy = y(t), lx = ml - 6.0, ly = y(t) + 4.0).unwrap();
            t += 0.5;
        }
        write!(s, "<line x1='{ml}' x2='{x2}' y1='{yy:.1}' y2='{yy:.1}' stroke='#333' stroke-dasharray='4 3'/>", x2 = ml + pw, yy = y(1.0)).unwrap();
        for (fi, (f, limbs)) in fields.iter().enumerate() {
            let gx = ml + gw * fi as f64 + gw * 0.1;
            for (ai, a) in arms.iter().enumerate() {
                if let Some(b) = bars.iter().find(|b| &b.0 == f && &b.2 == a) {
                    let x = gx + bw * ai as f64;
                    write!(s, "<rect x='{x:.1}' y='{:.1}' width='{:.1}' height='{:.1}' fill='{}'><title>{a} {f}: {:.3}× [{:.3}, {:.3}]</title></rect>", y(b.3), bw * 0.9, y(0.0) - y(b.3), COLORS[ai % COLORS.len()], b.3, b.4, b.5).unwrap();
                    let cx = x + bw * 0.45;
                    write!(
                        s,
                        "<line x1='{cx:.1}' x2='{cx:.1}' y1='{:.1}' y2='{:.1}' stroke='#222'/>",
                        y(b.4),
                        y(b.5)
                    )
                    .unwrap();
                }
            }
            write!(s, "<text x='{tx:.1}' y='{y1:.1}' text-anchor='middle'>{f}</text><text x='{tx:.1}' y='{y2:.1}' text-anchor='middle' fill='#555'>{limbs} limbs</text>", tx = ml + gw * (fi as f64 + 0.5), y1 = h - mb + 18.0, y2 = h - mb + 33.0).unwrap();
        }
        for (ai, a) in arms.iter().enumerate() {
            let lx = ml + ((ai % 6) as f64) * 150.0;
            let ly = h - mb + 52.0 + ((ai / 6) as f64) * 20.0;
            write!(s, "<rect x='{lx}' y='{ly}' width='12' height='12' fill='{}'/><text x='{}' y='{}'>{a}</text>", COLORS[ai % COLORS.len()], lx + 16.0, ly + 10.0).unwrap();
        }
        s.push_str("</svg>\n");
        s
    }

    fn kernel_svg(lines: &BTreeMap<String, Vec<(u64, f64)>>) -> String {
        let (w, h, ml, mb, mt) = (760.0, 420.0, 70.0, 70.0, 40.0);
        let pw = w - ml - 30.0;
        let ph = h - mb - mt;
        let ymax_log = lines
            .values()
            .flatten()
            .map(|p| p.1)
            .fold(1.0f64, f64::max)
            .log10()
            .ceil();
        let ymin_log = lines
            .values()
            .flatten()
            .map(|p| p.1)
            .fold(1e9f64, f64::min)
            .log10()
            .floor();
        let x = |l: u64| ml + pw * (l as f64 - 2.0) / 7.0;
        let y = |v: f64| mt + ph * (1.0 - (v.log10() - ymin_log) / (ymax_log - ymin_log));
        let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>");
        write!(s, "<text x='{ml}' y='22' font-size='14' font-weight='bold'>Modular multiply / square latency (ns, log scale) vs limbs</text>").unwrap();
        let mut e = ymin_log;
        while e <= ymax_log {
            let v = 10f64.powf(e);
            write!(s, "<line x1='{ml}' x2='{x2}' y1='{yy:.1}' y2='{yy:.1}' stroke='#e0e0e0'/><text x='{lx}' y='{ly:.1}' text-anchor='end'>{v}</text>", x2 = ml + pw, yy = y(v), lx = ml - 6.0, ly = y(v) + 4.0).unwrap();
            e += 1.0;
        }
        for l in 2..=9u64 {
            write!(
                s,
                "<text x='{:.1}' y='{}' text-anchor='middle'>{l}</text>",
                x(l),
                h - mb + 18.0
            )
            .unwrap();
        }
        write!(
            s,
            "<text x='{:.1}' y='{}' text-anchor='middle'>64-bit limbs</text>",
            ml + pw / 2.0,
            h - mb + 36.0
        )
        .unwrap();
        for (i, (op, pts)) in lines.iter().enumerate() {
            let c = COLORS[(i + 1) % COLORS.len()];
            let d: Vec<String> = pts
                .iter()
                .map(|(l, v)| format!("{:.1},{:.1}", x(*l), y(*v)))
                .collect();
            write!(
                s,
                "<polyline points='{}' fill='none' stroke='{c}' stroke-width='2'/>",
                d.join(" ")
            )
            .unwrap();
            for (l, v) in pts {
                write!(s, "<circle cx='{:.1}' cy='{:.1}' r='3' fill='{c}'><title>{op} {l} limbs: {v:.2} ns</title></circle>", x(*l), y(*v)).unwrap();
            }
            let lx = ml + i as f64 * 112.0;
            write!(s, "<rect x='{lx}' y='{}' width='12' height='12' fill='{c}'/><text x='{}' y='{}'>{op}</text>", h - 24.0, lx + 16.0, h - 14.0).unwrap();
        }
        s.push_str("</svg>\n");
        s
    }
}

/// `--batch`: per field, `rounds` rounds of `batch` groups of eight random
/// points and full-width scalars; arms `scalar8` (eight
/// `Point::scalar_mul`) and `batch8` (one `Point::scalar_mul_batch`) in a
/// seeded shuffled order per round, same inputs.  Every output is checked
/// against `scalar_mul` first (edge scalars included).  Writes `batch` rows
/// and prints the paired median ratio per field.
fn batch_mode(args: &[String]) {
    let rounds: usize = arg(args, "--rounds", 21);
    let groups: usize = arg(args, "--batch", 4);
    let seed: u64 = arg(args, "--seed", 0x6261_7463_6838);
    let out_path: String = arg(args, "--out", "mont_mul_batch.jsonl".to_string());
    let only: Option<String> = args
        .iter()
        .position(|a| a == "--fields")
        .and_then(|i| args.get(i + 1).cloned());
    let mut out = std::fs::File::create(&out_path).expect("create --out");
    for (fi, (name, p)) in fields()
        .into_iter()
        .filter(|(n, _)| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *n)))
        .enumerate()
    {
        let mut rng = StdRng::seed_from_u64(seed ^ (fi as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        let a = FieldElement::new(rng.gen_biguint_below(&p), p.clone());
        let rand_pt = |rng: &mut StdRng| Point::Affine {
            x: FieldElement::new(rng.gen_biguint_below(&p), p.clone()),
            y: FieldElement::new(rng.gen_biguint_below(&p), p.clone()),
        };
        // Correctness gate: edge scalars, the point at infinity, random.
        let mut pts: Vec<Point> = (0..16).map(|_| rand_pt(&mut rng)).collect();
        pts[3] = Point::Infinity;
        let mut ks: Vec<BigUint> = (0..16).map(|_| rng.gen_biguint(p.bits())).collect();
        for (i, v) in [0u32, 1, 2, 15, 16, 17].iter().enumerate() {
            ks[i + 8] = BigUint::from(*v);
        }
        ks[4] = &p - 1u32;
        let got = Point::scalar_mul_batch(&pts, &ks, &a);
        for i in 0..16 {
            assert_eq!(
                got[i],
                pts[i].scalar_mul(&ks[i], &a),
                "batch {name} lane {i}"
            );
        }
        eprintln!("{name}: 16 batch lanes agree with scalar_mul");
        let mut ratios = Vec::new();
        for round in 0..rounds {
            let pts: Vec<Point> = (0..8 * groups).map(|_| rand_pt(&mut rng)).collect();
            let ks: Vec<BigUint> = (0..8 * groups).map(|_| rng.gen_biguint(p.bits())).collect();
            let mut order = ["scalar8", "batch8"];
            let mut orng = StdRng::seed_from_u64(seed ^ ((fi as u64) << 32) ^ round as u64);
            order.shuffle(&mut orng);
            let mut ns = std::collections::BTreeMap::new();
            for (pos, arm) in order.iter().enumerate() {
                let t0 = std::time::Instant::now();
                if *arm == "scalar8" {
                    for (pt, k) in pts.iter().zip(&ks) {
                        std::hint::black_box(std::hint::black_box(pt).scalar_mul(k, &a));
                    }
                } else {
                    for g in 0..groups {
                        let r = 8 * g..8 * g + 8;
                        std::hint::black_box(Point::scalar_mul_batch(&pts[r.clone()], &ks[r], &a));
                    }
                }
                let per = t0.elapsed().as_nanos() as f64 / (8 * groups) as f64;
                ns.insert(*arm, per);
                writeln!(
                    out,
                    "{{\"kind\":\"batch\",\"field\":\"{name}\",\"limbs\":{},\"round\":{round},\"pos\":{pos},\"arm\":\"{arm}\",\"points\":{},\"ns_per_point\":{per:.1}}}",
                    p.bits().div_ceil(64),
                    8 * groups
                )
                .unwrap();
            }
            ratios.push(ns["scalar8"] / ns["batch8"]);
        }
        ratios.sort_by(|x, y| x.partial_cmp(y).unwrap());
        println!(
            "{name}: scalar8 / batch8 per point, median {:.3} (min {:.3}, max {:.3}, {} rounds)",
            ratios[ratios.len() / 2],
            ratios[0],
            ratios[ratios.len() - 1],
            ratios.len()
        );
    }
}

/// `--rho`: per curve and round, four budget-capped rho runs in a seeded
/// shuffled order: `lanes ∈ {1, L}` × budget `{B, 2B}` (one thread, same
/// target and seed).  The marginal cost per step is
/// `(t(2B) − t(B)) / B` per arm, which removes walk start-up (each walk
/// starts with two scalar multiplications).  Writes `rho` rows and prints
/// the median marginal ns per step per arm and their paired ratio.
fn rho_mode(args: &[String]) {
    use crypto_lib::cryptanalysis::ecdlp_nist::rho::{pollard_rho, RhoOptions};
    use crypto_lib::cryptanalysis::ecdlp_nist::{EcdlpGroup, PrimeGroup};
    use crypto_lib::ecc::curve::CurveParams;
    let rounds: usize = arg(args, "--rounds", 9);
    let budget: u64 = arg(args, "--budget", 1u64 << 15);
    let lanes: usize = arg(args, "--lanes", 256);
    let seed: u64 = arg(args, "--seed", 0x7268_6f6c);
    let out_path: String = arg(args, "--out", "mont_mul_rho.jsonl".to_string());
    let only: Option<String> = args
        .iter()
        .position(|a| a == "--fields")
        .and_then(|i| args.get(i + 1).cloned());
    let mut out = std::fs::File::create(&out_path).expect("create --out");
    let curves = [
        ("eccp131", CurveParams::eccp131()),
        ("secp256k1", CurveParams::secp256k1()),
        ("p256", CurveParams::p256()),
        ("gost512_test", CurveParams::gost_3410_2012_512_test()),
    ];
    for (ci, (name, curve)) in curves
        .into_iter()
        .filter(|(n, _)| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *n)))
        .enumerate()
    {
        let g = PrimeGroup::new(curve);
        let mut rng = StdRng::seed_from_u64(seed ^ ci as u64);
        let k = rng.gen_biguint_below(g.order());
        let q = g.mul(&g.generator(), &k);
        let mut marg: std::collections::BTreeMap<usize, Vec<f64>> = Default::default();
        let mut ratios = Vec::new();
        for round in 0..rounds {
            let mut runs = [
                (1usize, budget),
                (1, 2 * budget),
                (lanes, budget),
                (lanes, 2 * budget),
            ];
            let mut orng = StdRng::seed_from_u64(seed ^ ((ci as u64) << 32) ^ round as u64);
            runs.shuffle(&mut orng);
            let mut t = std::collections::BTreeMap::new();
            for (pos, &(ln, b)) in runs.iter().enumerate() {
                let opts = RhoOptions {
                    threads: 1,
                    seed: seed ^ round as u64,
                    max_iterations: Some(b),
                    lanes: ln,
                    ..RhoOptions::default()
                };
                let t0 = std::time::Instant::now();
                let rep = pollard_rho(&g, &q, &opts);
                let ns = t0.elapsed().as_nanos() as f64;
                t.insert((ln, b), (ns, rep.iterations));
                writeln!(
                    out,
                    "{{\"kind\":\"rho\",\"curve\":\"{name}\",\"round\":{round},\"pos\":{pos},\"lanes\":{ln},\"budget\":{b},\"iterations\":{},\"ns\":{ns:.0}}}",
                    rep.iterations
                )
                .unwrap();
            }
            let m = |ln: usize| {
                let (t1, i1) = t[&(ln, budget)];
                let (t2, i2) = t[&(ln, 2 * budget)];
                (t2 - t1) / (i2 - i1).max(1) as f64
            };
            let (m1, ml) = (m(1), m(lanes));
            marg.entry(1).or_default().push(m1);
            marg.entry(lanes).or_default().push(ml);
            ratios.push(m1 / ml);
        }
        let med = |v: &mut Vec<f64>| {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v[v.len() / 2]
        };
        let m1 = med(marg.get_mut(&1).unwrap());
        let ml = med(marg.get_mut(&lanes).unwrap());
        let r = med(&mut ratios);
        println!(
            "{name}: marginal ns/step lanes=1 {m1:.0}, lanes={lanes} {ml:.0}; paired ratio median {r:.3} ({rounds} rounds)"
        );
    }
}
