//! Frozen single-target panel (protocol: ../PROTOCOL.md). Seed policy isogeny-labs/experiment-seeds/v1.
//! panel <evidence_dir> <T>
//! Writes <dir>/seed_manifest.json (refuses to replace a different pre-call manifest), then
//! <dir>/panel.jsonl: one row per solve, arms interleaved (rotated by trial index), single thread.
//! Target logs stay evaluator-side: `solve` receives only public coordinates; the log is compared after.
use m31_structure_rho::structure::{self, is_prime, mul, mul_jacobian, rand_point, Rng};
use m31_structure_rho::*;
use std::io::Write;

const PANEL_MASTER: &str = "4d33315f72686f5f"; // "M31_rho_"

struct Curve {
    name: &'static str,
    a: u64,
    b: u64,
    n: u64,
    g: (u64, u64),
    beta: u64,
    lambda: u64,
}

fn j0() -> Curve {
    let (b, n0, r) = (5u64, 2147444533u64, 165188041u64);
    assert!(n0 == 13 * r && is_prime(r));
    let mut rng = Rng::new("panel/j0/G");
    let g = loop {
        let gg = mul(13, Some(rand_point(0, b, &mut rng)), 0);
        if gg.is_some() && mul(r, gg, 0).is_none() {
            break gg.unwrap();
        }
    };
    let gen = (2..50u64).find(|&x| powmod(x, (P - 1) / 3, P) != 1).unwrap();
    let beta = powmod(gen, (P - 1) / 3, P);
    let h = (2..50u64).find(|&x| powmod(x, (r - 1) / 3, r) != 1).unwrap();
    let l1 = powmod(h, (r - 1) / 3, r);
    let target = Some((mulmod(beta, g.0, P), g.1));
    let lambda = [l1, mulmod(l1, l1, r)].into_iter().find(|&l| mul(l, Some(g), 0) == target).unwrap();
    Curve { name: "J0_b5_r", a: 0, b, n: r, g, beta, lambda }
}

fn curves() -> Vec<Curve> {
    let mut rng = Rng::new("panel/M31_minus3_21/G");
    let g2 = rand_point(P - 3, 21, &mut rng);
    vec![
        Curve { name: "B677", a: 847514846, b: 2087444724, n: 2147446831, g: (625037715, 2100627251), beta: 0, lambda: 0 },
        Curve { name: "M31_minus3_21", a: P - 3, b: 21, n: 2147487637, g: g2, beta: 0, lambda: 0 },
        j0(),
    ]
}

// (curve index, arms, W, dp, frozen_Q)
const DESIGN: &[(usize, &[&str], usize, u32, bool)] = &[
    (0, &["plain", "neg", "neg_aa"], 64, 5, true),
    (1, &["plain", "neg", "neg_aa"], 64, 5, false),
    (2, &["plain", "neg", "neg_aa", "aut6"], 64, 5, false),
    (2, &["neg", "aut6"], 16, 3, false),
];

fn main() {
    assert!(kdf::selftest());
    let v: Vec<String> = std::env::args().collect();
    let dir = &v[1];
    let t: usize = v[2].parse().unwrap();
    let cs = curves();
    let mut m = format!(
        "{{\"policy\":\"isogeny-labs/experiment-seeds/v1\",\"study\":\"m31_curve_structure_20261126\",\
\"master_seed_hex\":\"{PANEL_MASTER}\",\"structure_master_hex\":\"{}\",\
\"kdf\":\"first 8 bytes little-endian of SHA-256(master '/' parts joined by '/') (native/src/kdf.rs)\",\
\"streams\":{{\"target\":\"target/<curve>/<i>: k = key mod (n-1) + 1 (evaluator custody)\",\
\"walk\":\"walk/<curve>/<i>/<arm>/<W>/<dp>: SplitMix64 state for jump table and start schedule (arms independent; neg_aa = second independent neg)\",\
\"generators\":\"panel/<curve>/G via structure::Rng (SplitMix64 keyed by SHA-256(structure master/stream))\"}},\
\"trials_per_cell\":{t},\"arm_order\":\"rotate arms by trial index\",\"threads\":1,\"curves\":[",
        structure::MASTER
    );
    for (i, c) in cs.iter().enumerate() {
        m += &format!(
            "{}{{\"name\":\"{}\",\"a\":{},\"b\":{},\"n\":{},\"G\":[{},{}],\"beta\":{},\"lambda\":{}}}",
            if i > 0 { "," } else { "" },
            c.name, c.a, c.b, c.n, c.g.0, c.g.1, c.beta, c.lambda
        );
    }
    m += "],\"design\":[";
    for (i, d) in DESIGN.iter().enumerate() {
        m += &format!(
            "{}{{\"curve\":\"{}\",\"arms\":{:?},\"W\":{},\"dp_bits\":{},\"targets\":\"{}\"}}",
            if i > 0 { "," } else { "" },
            cs[d.0].name, d.1, d.2, d.3, if d.4 { "frozen public Q, log 987654321" } else { "seeded" }
        );
    }
    m += "]}";
    let mpath = format!("{}/seed_manifest.json", dir);
    if let Ok(old) = std::fs::read_to_string(&mpath) {
        assert_eq!(old.trim(), m, "refusing to replace a different pre-call manifest");
    } else {
        std::fs::write(&mpath, format!("{}\n", m)).unwrap();
    }
    eprintln!("manifest sha256 {}", kdf::hex(&kdf::sha256(m.as_bytes())));
    let mut out = std::fs::File::create(format!("{}/panel.jsonl", dir)).unwrap();
    let mut seen = std::collections::HashSet::new();
    let (mut ok, mut bad) = (0u64, 0u64);
    for d in DESIGN {
        let c = &cs[d.0];
        for i in 0..t {
            let (k, q) = if d.4 {
                (987654321u64, (976979610u64, 1228979761u64))
            } else {
                let k = kdf::key(PANEL_MASTER, &["target", c.name, &i.to_string()]) % (c.n - 1) + 1;
                (k, mul(k, Some(c.g), c.a).unwrap())
            };
            let rot = i % d.1.len();
            let order: Vec<&str> = d.1[rot..].iter().chain(d.1[..rot].iter()).cloned().collect();
            for arm in order {
                let ws = kdf::key(PANEL_MASTER, &["walk", c.name, &i.to_string(), arm, &d.2.to_string(), &d.3.to_string()]);
                assert!(seen.insert(ws), "duplicate walk stream");
                let real = if arm == "neg_aa" { "neg" } else { arm };
                let job = Job {
                    arm: parse_arm(real),
                    a: c.a,
                    b: c.b,
                    n: c.n,
                    g: c.g,
                    q,
                    seed: ws,
                    walks: d.2,
                    dp_bits: d.3,
                    beta: c.beta,
                    lambda: c.lambda,
                };
                let s = solve(&job);
                let v_jac = mul_jacobian(s.recovered, Some(c.g), c.a) == Some(q);
                let v_log = s.recovered == k;
                if s.verified && v_jac && v_log { ok += 1 } else { bad += 1 }
                let row = s.json.trim_end_matches('}');
                writeln!(
                    out,
                    "{{\"curve\":\"{}\",\"trial\":{},\"arm_label\":\"{}\",{},\"verify_jacobian_dG_eq_Q\":{},\"verify_eq_evaluator_log\":{}}}",
                    c.name, i, arm, &row[1..], v_jac, v_log
                )
                .unwrap();
            }
        }
        eprintln!("{} W={} dp={} done", c.name, d.2, d.3);
    }
    eprintln!("verified {} failed {}", ok, bad);
}
