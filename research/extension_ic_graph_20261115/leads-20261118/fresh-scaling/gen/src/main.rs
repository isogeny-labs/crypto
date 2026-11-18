//! fresh-scaling-gen: deterministic fresh public targets for the restricted
//! weak-cover `E(F_{p^6})` family, an independent public-input checker, and
//! the preregistration's runtime/cost estimator.
//!
//!   fresh-scaling-gen gen --nonce HEX --p P --index I [--kind public|planted] --out DIR
//!   fresh-scaling-gen batch --nonce HEX --sizes P1,P2,.. --n N [--kind K] [--start I0] --out DIR
//!   fresh-scaling-gen check FILE...           (public.json files; exit 1 on any failure)
//!   fresh-scaling-gen estimate [--sizes ..] [--n N]
//!
//! All outputs are [pcc:unverified] producer artifacts. A planted target's
//! scalar is written to `owner/` beside, never into, the public file.

mod curve;
mod field;
mod gen;
mod json;
mod sha256;

use curve::{cover_admits, embedding_degree, order_4_prime, Curve, Pt};
use field::{is_prime_u64, Tower, E2, E6};
use gen::{generate, manifest_json, public_json, Kind};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

fn args_map(args: &[String]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    let mut i = 0;
    while i < args.len() {
        if let Some(k) = args[i].strip_prefix("--") {
            let v = args.get(i + 1).cloned().unwrap_or_default();
            m.insert(k.to_string(), v);
            i += 2;
        } else {
            i += 1;
        }
    }
    m
}

fn nonce_ok(n: &str) -> bool {
    n.len() >= 16 && n.chars().all(|c| c.is_ascii_hexdigit() || c == '-' || c.is_ascii_alphanumeric())
}

fn write_target(nonce: &str, p: u64, index: u64, kind: Kind, out: &Path) -> String {
    let t0 = Instant::now();
    let t = generate(nonce, p, index, kind);
    let secs = t0.elapsed().as_secs_f64();
    let public = public_json(&t);
    let man = manifest_json(&t, nonce, &public);
    let dir = out.join(format!("p{p}")).join(format!("{}-{index:04}", kind.name()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    std::fs::write(dir.join("public.json"), &public).expect("write public");
    std::fs::write(dir.join("manifest.json"), &man).expect("write manifest");
    if let Some(d) = t.planted_scalar {
        let od = out.join("owner").join(format!("p{p}"));
        std::fs::create_dir_all(&od).expect("mkdir owner");
        std::fs::write(
            od.join(format!("{}-{index:04}.answer.json", kind.name())),
            format!("{{\"schema\":\"fresh-scaling.owner-answer/v1\",\"p\":{p},\"index\":{index},\"planted_scalar\":{d}}}"),
        )
        .expect("write owner");
    }
    // generation wall is an untimed-harness diagnostic, kept off the manifest
    // so that the manifest is a pure function of the derivation inputs.
    format!(
        "{{\"p\":{p},\"index\":{index},\"kind\":\"{}\",\"l\":{},\"attempts\":{},\"generator_wall_s\":{secs:.3},\"dir\":\"{}\"}}",
        kind.name(),
        t.l,
        t.attempts,
        dir.display()
    )
}

/// Independent re-check of a public input: on-curve points, prime `l`,
/// `[l]G = [l]Q = O`, `#E = 4l` by a fresh BSGS, the cover admission, `j`
/// outside proper subfields and the embedding-degree bound.
fn check_public(text: &str) -> Result<String, String> {
    let v = json::parse(text)?;
    let p = v.get("p").and_then(|x| x.as_u64()).ok_or("p")?;
    let l = v.get("l").and_then(|x| x.as_u64()).ok_or("l")?;
    let e6 = |x: &json::Value| -> Result<E6, String> {
        let a = x.as_arr().ok_or("e6")?;
        if a.len() != 3 {
            return Err("e6 len".into());
        }
        let mut out = E6::ZERO;
        for (i, c) in a.iter().enumerate() {
            let c = c.as_arr().ok_or("e2")?;
            if c.len() != 2 {
                return Err("e2 len".into());
            }
            let x0 = c[0].as_u64().ok_or("u64")?;
            let x1 = c[1].as_u64().ok_or("u64")?;
            if x0 >= p || x1 >= p {
                return Err("coordinate not reduced".into());
            }
            out.0[i] = E2([x0, x1]);
        }
        Ok(out)
    };
    let pt = |x: &json::Value| -> Result<Pt, String> {
        Ok(Pt {
            inf: x.get("inf").and_then(|b| b.as_bool()).ok_or("inf")?,
            x: e6(x.get("x").ok_or("x")?)?,
            y: e6(x.get("y").ok_or("y")?)?,
        })
    };
    for forbidden in ["d", "answer", "scalar", "planted_scalar", "expected_scalar"] {
        if v.get(forbidden).is_some() {
            return Err(format!("public input carries answer-like field {forbidden}"));
        }
    }
    let t = Tower::new(p);
    let alpha = e6(v.get("alpha").ok_or("alpha")?)?;
    let g = pt(v.get("g").ok_or("g")?)?;
    let q = pt(v.get("q").ok_or("q")?)?;
    let c = Curve::new(&t, alpha);
    let mut fails = Vec::new();
    if !cover_admits(&t, &alpha) {
        fails.push("cover");
    }
    if !is_prime_u64(l) {
        fails.push("l not prime");
    }
    if g.inf || q.inf || !c.on_curve(&g) || !c.on_curve(&q) {
        fails.push("points");
    }
    if !c.mul(&g, l as u128).inf || !c.mul(&q, l as u128).inf {
        fails.push("subgroup");
    }
    // #E = 4l: BSGS on points derived from the file's own hash
    let tag = sha256::hex(&sha256::sha256(text.as_bytes()));
    let mut s = sha256::Stream::new(&format!("fresh-scaling/v1|check|{tag}"));
    let z = gen::draw_nonsquare(&t, &mut s);
    let p0 = gen::draw_point(&c, &mut s, &z);
    let checks = [gen::draw_point(&c, &mut s, &z), gen::draw_point(&c, &mut s, &z)];
    let ord = order_4_prime(&c, &p0, &checks);
    if ord.map(|(_, ll)| ll) != Some(l) {
        fails.push("#E != 4l");
    }
    let j = c.j_invariant();
    let jsub = j.map(|j| t.in_subfield(&j, 2) || t.in_subfield(&j, 3));
    if jsub != Some(false) {
        fails.push("j in proper subfield or singular");
    }
    let emb = embedding_degree(p, l, gen::EMBEDDING_KMAX);
    if emb.is_some() {
        fails.push("small embedding degree");
    }
    let row = format!(
        "{{\"p\":{p},\"l\":{l},\"b\":{:.4},\"sha256\":\"{tag}\",\"ok\":{},\"fails\":[{}]}}",
        (l as f64).log2(),
        fails.is_empty(),
        fails.iter().map(|f| format!("\"{f}\"")).collect::<Vec<_>>().join(",")
    );
    if fails.is_empty() { Ok(row) } else { Err(row) }
}

/// Preregistration runtime model. Constants are Runpod CPU observations of
/// Run50 (runpod-deferred-pair-20261117-50, p1009, one walk each) and the
/// local nagao-e2e medians; they are indicative, [pcc:unverified].
fn estimate(sizes: &[u64], n: u64) {
    // per-step seconds of each registered rho config at p1009 (time.txt real_s / steps)
    let rho = [
        ("rho-plain-b16", 36.908 / 249_417_689.0, 2.0f64),
        ("rho-plain-b1", 119.768 / 587_751_424.0, 2.0),
        ("rho-neg-b16", 102.924 / 617_229_160.0, 4.0),
        ("rho-neg-b1", 117.565 / 535_450_837.0, 4.0),
    ];
    // IC model: T = T_nd (p/1009)^e + R c_res, residuals R ~ NB(2, pi)
    let t_nd = 13.0; // s at p1009, non-descent share of the 20.7 s nagao-e2e median
    let r_mean = 2000.0; // residuals per run (1506, 1728, 3415 observed)
    let c_res = 0.0038; // s per residual (Run50 quadratic-table 15.1 s / 3415 upper bound ~4.4 ms)
    println!("p,b_log2_l,arm,steps_or_model,seconds_per_target,seconds_for_n,cpu_hours_for_n");
    let mut total = [0.0f64; 3];
    for &p in sizes {
        let l = (p as f64).powi(6) / 4.0;
        let b = l.log2();
        let mut size_rho = 0.0;
        for (name, sps, div) in rho {
            let steps = (std::f64::consts::PI * l / div).sqrt();
            let s = steps * sps;
            size_rho += s;
            println!("{p},{b:.2},{name},{steps:.4e},{s:.1},{:.0},{:.2}", s * n as f64, s * n as f64 / 3600.0);
        }
        // A/A: the a-priori cheapest config again with independent seeds
        let aa = (std::f64::consts::PI * l / 4.0).sqrt() * rho[2].1;
        println!("{p},{b:.2},rho-AA(neg-b16),{:.4e},{aa:.1},{:.0},{:.2}", (std::f64::consts::PI * l / 4.0).sqrt(), aa * n as f64, aa * n as f64 / 3600.0);
        size_rho += aa;
        let mut ic = [0.0; 2];
        for (k, e) in [2.0f64, 3.0].iter().enumerate() {
            let s = t_nd * (p as f64 / 1009.0).powf(*e) + r_mean * c_res;
            ic[k] = s;
            println!("{p},{b:.2},ic-graph(e={e}),T_nd*(p/1009)^{e}+R*c_res,{s:.1},{:.0},{:.2}", s * n as f64, s * n as f64 / 3600.0);
        }
        // IC A/A arm (independent worker seed) doubles the IC cost
        total[0] += (size_rho + 2.0 * ic[0]) * n as f64;
        total[1] += (size_rho + 2.0 * ic[1]) * n as f64;
        total[2] += size_rho * n as f64;
    }
    println!(
        "# totals (all arms, IC + IC-A/A + 4 rho configs + rho-A/A, n={n} per size): {:.1} CPU-h (IC e=2), {:.1} CPU-h (IC e=3); rho share {:.1} CPU-h",
        total[0] / 3600.0,
        total[1] / 3600.0,
        total[2] / 3600.0
    );
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let cmd = argv.get(1).map(String::as_str).unwrap_or("");
    let a = args_map(&argv[2.min(argv.len())..]);
    let sizes = |s: Option<&String>| -> Vec<u64> {
        s.map(|v| v.split(',').map(|x| x.trim().parse().expect("size")).collect())
            .unwrap_or_else(|| vec![127, 223, 383, 661, 1153, 2017])
    };
    match cmd {
        "gen" | "batch" => {
            let nonce = a.get("nonce").expect("--nonce");
            assert!(nonce_ok(nonce), "nonce must be >= 16 hex/alnum characters");
            let kind = Kind::parse(a.get("kind").map(String::as_str).unwrap_or("public")).expect("--kind");
            let out = Path::new(a.get("out").expect("--out"));
            if cmd == "gen" {
                let p: u64 = a.get("p").expect("--p").parse().unwrap();
                let i: u64 = a.get("index").expect("--index").parse().unwrap();
                println!("{}", write_target(nonce, p, i, kind, out));
            } else {
                let n: u64 = a.get("n").expect("--n").parse().unwrap();
                let start: u64 = a.get("start").map(|s| s.parse().unwrap()).unwrap_or(0);
                for p in sizes(a.get("sizes")) {
                    for i in start..start + n {
                        println!("{}", write_target(nonce, p, i, kind, out));
                    }
                }
            }
        }
        "check" => {
            let mut bad = 0;
            for f in &argv[2..] {
                let text = std::fs::read_to_string(f).expect("read");
                match check_public(text.trim_end()) {
                    Ok(r) => println!("{{\"file\":\"{f}\",\"result\":{r}}}"),
                    Err(r) => {
                        bad += 1;
                        println!("{{\"file\":\"{f}\",\"error\":{}}}", if r.starts_with('{') { r } else { format!("\"{r}\"") })
                    }
                }
            }
            std::process::exit(if bad == 0 { 0 } else { 1 });
        }
        "estimate" => {
            let n: u64 = a.get("n").map(|s| s.parse().unwrap()).unwrap_or(32);
            estimate(&sizes(a.get("sizes")), n);
        }
        _ => {
            eprintln!("usage: fresh-scaling-gen gen|batch|check|estimate (see source header)");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const LANE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../..");

    fn committed(rel: &str) -> String {
        std::fs::read_to_string(format!("{LANE}/{rel}")).expect("committed input").trim_end().to_string()
    }

    #[test]
    fn committed_p101_input_is_in_this_basis_and_checks() {
        // Stage32's p101 native-rho input (pretty-printed): its points lie on
        // the curve only if w and s match the library's tower.
        let r = check_public(&committed("runpod-rho-native-price-validation-20261116-32/public.json"));
        assert!(r.is_ok(), "{r:?}");
    }

    #[test]
    fn committed_p1009_nagao_input_is_in_this_basis_and_checks() {
        let r = check_public(&committed("nagao-e2e/public.json"));
        assert!(r.is_ok(), "{r:?}");
    }

    #[test]
    fn generation_is_deterministic_and_checks_public_and_planted() {
        let nonce = "0000000000000000dryrun-test-only";
        for p in [101u64, 127] {
            let a = generate(nonce, p, 3, Kind::Public);
            let b = generate(nonce, p, 3, Kind::Public);
            let pa = public_json(&a);
            assert_eq!(pa, public_json(&b));
            assert!(a.planted_scalar.is_none());
            assert!(check_public(&pa).is_ok(), "{:?}", check_public(&pa));
            // another index or kind gives another target
            let c = generate(nonce, p, 4, Kind::Public);
            assert_ne!(pa, public_json(&c));
            let d = generate(nonce, p, 3, Kind::Planted);
            let pd = public_json(&d);
            assert!(check_public(&pd).is_ok());
            let t = Tower::new(p);
            let cv = Curve::new(&t, d.alpha);
            assert_eq!(cv.mul(&d.g, d.planted_scalar.unwrap() as u128), d.q);
            assert!(!pd.contains("planted"));
        }
    }

    #[test]
    fn check_refuses_tampered_inputs() {
        let nonce = "0000000000000000dryrun-test-only";
        let t = generate(nonce, 101, 0, Kind::Public);
        let good = public_json(&t);
        // wrong l
        let bad_l = good.replace(&format!("\"l\":{}", t.l), &format!("\"l\":{}", t.l + 2));
        assert!(check_public(&bad_l).is_err());
        // answer-like field present
        let leaked = good.replacen('{', "{\"d\":5,", 1);
        assert!(check_public(&leaked).is_err());
        // a point moved off the curve
        let mut q = t.q;
        q.y.0[0].0[0] = (q.y.0[0].0[0] + 1) % 101;
        let off = public_json(&gen::Target { q, ..t });
        assert!(check_public(&off).is_err());
    }
}
