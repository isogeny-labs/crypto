//! J4 (port of j4_band.py): S' of PROTOCOL section 5 and the section 5a order.
//!
//! S'_w(record) = (priced_gae_w + w * table_filter_bit_sets) / sqrt(r), with
//! priced_gae_w = G - C + w (U + n_C) as band.rs (3271a25d02) defines it.
//! Ratios are sum S'_w(B) / sum S'_w(A) over matched measured pairs, overall
//! and per curve. Without the bit-set term the ratio is band.rs's priced S ratio.

use crate::py::{self, fsum_py, Obj};
use crate::v::{self, Arms};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;

const W: [(f64, &str); 3] = [(0.0, "0.0"), (0.1, "0.1"), (1.0, "1.0")];

fn ratio(arms: &Arms, a: &str, b: &str, w: f64, curve: Option<&str>, with_bits: bool) -> (f64, usize, f64, f64) {
    let (ma, mb) = (&arms[&a.to_string()], &arms[&b.to_string()]);
    if ma.key_set() != mb.key_set() {
        v::stop(format!("unmatched keys {a} {b}"));
    }
    let ks: Vec<&v::Key> = ma.keys().filter(|k| curve.is_none_or(|c| k.0 == c)).collect();
    let f = |r: &Value| if with_bits { v::sprime(r, w) } else { v::priced_gae(r, w) / v::curve_r_sqrt(r) };
    let sa = fsum_py(ks.iter().map(|k| f(ma[*k])));
    let sb = fsum_py(ks.iter().map(|k| f(mb[*k])));
    let n = ks.len();
    (sb / sa, n, sa / n as f64, sb / n as f64)
}

fn table(arms: &Arms, pairs: &[(&str, &str)]) -> Value {
    let first = arms.values().next().unwrap();
    let curves: Vec<String> = first.keys().map(|k| k.0.clone()).collect::<BTreeSet<_>>().into_iter().collect();
    let mut out = Obj::new();
    for (a, b) in pairs {
        let mut row = Obj::new();
        let mut per: Vec<Vec<f64>> = Vec::new(); // per weight, per curve
        for (w, ws) in W {
            let (r, n, ma, mb) = ratio(arms, a, b, w, None, true);
            let (r0, _, _, _) = ratio(arms, a, b, w, None, false);
            let (mut pc, mut pcm) = (Obj::new(), Obj::new());
            let mut pw = Vec::new();
            for c in &curves {
                let x = ratio(arms, a, b, w, Some(c), true);
                pw.push(x.0);
                pc.set(c.clone(), py::fval(x.0));
                pcm.set(c.clone(), serde_json::json!({"a": py::fval(x.2), "b": py::fval(x.3)}));
            }
            per.push(pw);
            let mut o = Obj::new();
            o.set("ratio_b_over_a", py::fval(r))
                .set("pairs", n as u64)
                .set("mean_Sprime_a", py::fval(ma))
                .set("mean_Sprime_b", py::fval(mb))
                .set("ratio_without_bitset_term", py::fval(r0))
                .set("per_curve", pc.val())
                .set("per_curve_mean_Sprime", pcm.val());
            row.set(ws, o.val());
        }
        // 5a: order per curve at each weight (B costlier means ratio > 1)
        let mut orders = Obj::new();
        let mut strict = true;
        for (ci, c) in curves.iter().enumerate() {
            let o: Vec<&str> = (0..3)
                .map(|wi| {
                    let x = per[wi][ci];
                    if x > 1.0 {
                        "A<B"
                    } else if x < 1.0 {
                        "A>B"
                    } else {
                        "tie"
                    }
                })
                .collect();
            strict &= o.iter().collect::<BTreeSet<_>>().len() == 1 && o[0] != "tie";
            orders.set(c.clone(), o);
        }
        row.set("order_per_curve_at_w0_0.1_1", orders.val());
        row.set("strict_same_order_across_band_every_curve", strict);
        out.set(format!("{b}/{a}"), row.val());
    }
    out.val()
}

pub fn run(out: &Path) {
    let mut res = Obj::new();
    for b in [32u32, 24] {
        let recs = v::load_counted(b, "wall"); // counts only; no wall field needed here
        let arms = v::by_arm(&recs, true);
        res.set(format!("P{b}-wall"), table(&arms, &[("dgb", "dgbf"), ("dgb", "dgb-aa"), ("dgbf-edge", "dgbf")]));
    }
    for b in [32u32, 24, 28] {
        let recs = v::load_counted(b, "counted");
        let arms = v::by_arm(&recs, true);
        res.set(
            format!("P{b}-counted"),
            table(&arms, &[("rho-neg", "dgb"), ("rho-neg", "dgbf"), ("dgb", "dgbf"), ("rho-neg", "rho-neg-aa")]),
        );
    }
    py::dump(&res.val(), out);
    println!("j4 done");
}
