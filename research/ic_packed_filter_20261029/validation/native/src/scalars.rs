//! Own scalar check (port of scalars.py).
//!
//! For every record of a session, warm-ups included, check with the
//! validator's own short-Weierstrass arithmetic over the record's prime that
//! [recovered] G = Q. The curve is bound independently of the record's claim:
//! (A, B) are fitted from G and the targets of that curve; the fit must place
//! every point on the curve, j(A, B) must equal the j-invariant in the ICV1
//! string, p + 1 - t must equal r, and [r] G must be the identity.

use crate::py::{self, int, int_hex, int_str, truthy, Obj};
use crate::v::{self, OMap};
use serde_json::Value;
use std::path::Path;

type Pt = Option<(i128, i128)>; // None is the point at infinity

fn md(a: i128, p: i128) -> i128 {
    a.rem_euclid(p)
}

/// pow(a, e, p) for e >= 0.
fn powm(a: i128, mut e: i128, p: i128) -> i128 {
    let mut base = md(a, p);
    let mut r = 1 % p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * base % p;
        }
        base = base * base % p;
        e >>= 1;
    }
    r
}

/// pow(a, -1, p): the inverse mod p (ValueError when not invertible).
fn inv(a: i128, p: i128) -> i128 {
    let (mut r0, mut r1) = (md(a, p), p);
    let (mut s0, mut s1) = (1i128, 0i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
    }
    if r0 != 1 {
        v::stop("base is not invertible for the given modulus");
    }
    md(s0, p)
}

pub fn ec_add(pp: Pt, qq: Pt, a: i128, p: i128) -> Pt {
    let (x1, y1) = match pp {
        None => return qq,
        Some(x) => x,
    };
    let (x2, y2) = match qq {
        None => return pp,
        Some(x) => x,
    };
    let lam = if x1 == x2 {
        if md(y1 + y2, p) == 0 {
            return None;
        }
        md((3 * x1 * x1 + a) * inv(2 * y1, p), p)
    } else {
        md((y2 - y1) * inv(x2 - x1, p), p)
    };
    let x3 = md(lam * lam - x1 - x2, p);
    Some((x3, md(lam * (x1 - x3) - y1, p)))
}

pub fn ec_mul(mut k: i128, mut pp: Pt, a: i128, p: i128) -> Pt {
    let mut r: Pt = None;
    while k != 0 {
        if k & 1 == 1 {
            r = ec_add(r, pp, a, p);
        }
        pp = ec_add(pp, pp, a, p);
        k >>= 1;
    }
    r
}

pub struct Bound {
    pub checks: Value,
    pub g: (i128, i128),
    pub a: i128,
    pub p: i128,
}

fn bind_curve(curve: &Value, points: &[(i128, i128)]) -> Bound {
    let f: Vec<&str> = curve["icv1"].as_str().unwrap().split(':').collect();
    let p: i128 = f[1].split('-').nth(1).unwrap().parse().unwrap();
    let t: i128 = f[2].parse().unwrap();
    let r: i128 = f[3].parse().unwrap();
    let j: i128 = f[4].parse().unwrap();
    let gen = curve["generator"].as_array().unwrap();
    let g = (int_hex(&gen[0]), int_hex(&gen[1]));
    let lhs = |pt: (i128, i128)| md(pt.1 * pt.1 - pt.0 * pt.0 * pt.0, p);
    let q = *points.iter().find(|q| q.0 != g.0).expect("StopIteration");
    let a = md((lhs(g) - lhs(q)) * inv(g.0 - q.0, p), p);
    let b = md(lhs(g) - a * g.0, p);
    let on = std::iter::once(&g).chain(points.iter()).all(|pt| md(lhs(*pt) - a * pt.0 - b, p) == 0);
    let a3 = powm(a, 3, p);
    let jj = md(1728 * 4 * a3 % p * inv(4 * a3 + 27 * b * b, p), p);
    let mut c = Obj::new();
    c.set("p", py::int_val(p))
        .set("A", py::int_val(a))
        .set("B", py::int_val(b))
        .set("trace", py::int_val(t))
        .set("r", py::int_val(r))
        .set("all_points_on_curve", on)
        .set("j_matches_icv1", jj == j)
        .set("hasse_r_eq_p_plus_1_minus_t", p + 1 - t == r && r == int(&curve["r"]))
        .set("r_G_is_identity", ec_mul(r, Some(g), a, p).is_none())
        .set("p_le_2pow32_minus_1", p <= 4294967295);
    Bound { checks: c.val(), g, a, p }
}

fn target(rec: &Value) -> (i128, i128) {
    let t = rec["workload"]["target"].as_array().unwrap();
    (int_hex(&t[0]), int_hex(&t[1]))
}

pub fn check_session(records: &[&Value]) -> Value {
    // by curve: (curve dict, set of targets) in first-seen order
    let mut by_curve: OMap<String, (&Value, Vec<(i128, i128)>)> = OMap::new();
    for rec in records {
        let c = &rec["workload"]["curve"];
        let e = by_curve.entry_or(c["slug"].as_str().unwrap().to_string(), || (c, Vec::new()));
        let t = target(rec);
        if !e.1.contains(&t) {
            e.1.push(t);
        }
    }
    let mut curves: OMap<String, Bound> = OMap::new();
    for (slug, (c, pts)) in by_curve.iter() {
        curves.insert(slug.clone(), bind_curve(c, pts));
    }
    let (mut n, mut warm, mut sok, mut sver, mut mt) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut fails = Vec::new();
    for rec in records {
        n += 1;
        warm += truthy(&rec["warmup"]) as u64;
        let bd = &curves[&rec["workload"]["curve"]["slug"].as_str().unwrap().to_string()];
        let q = target(rec);
        let o = &rec["outcome"];
        let mut ok = false;
        if o.get("recovered").is_some_and(|x| !x.is_null()) {
            let k = int_str(&o["recovered"]);
            ok = ec_mul(k, Some(bd.g), bd.a, bd.p) == Some(q);
        }
        let verified = o["status"].as_str() == Some("verified");
        let mtt = o.get("matches_target") == Some(&Value::Bool(true));
        sok += ok as u64;
        sver += verified as u64;
        mt += mtt as u64;
        if !(ok && verified && mtt) {
            let mut f = Obj::new();
            f.set("record_id", rec["record_id"].clone())
                .set("seq", rec["seq"].clone())
                .set("arm", rec["arm"].clone())
                .set("recovered", o.get("recovered").cloned().unwrap_or(Value::Null))
                .set("status", o["status"].clone());
            fails.push(f.val());
        }
    }
    let mut cv = Obj::new();
    for (s, b) in curves.iter() {
        cv.set(s.clone(), b.checks.clone());
    }
    let all_ok = fails.is_empty()
        && curves
            .values()
            .all(|b| b.checks.as_object().unwrap().values().filter(|x| x.is_boolean()).all(|x| x == &Value::Bool(true)));
    let mut out = Obj::new();
    out.set("records", n)
        .set("warmups", warm)
        .set("scalar_ok", sok)
        .set("status_verified", sver)
        .set("matches_target_true", mt)
        .set("failures", fails)
        .set("curves", cv.val())
        .set("all_ok", all_ok);
    out.val()
}

pub fn run(out: &Path) {
    let mut res = Obj::new();
    let chk = |recs: Vec<Value>| -> Value {
        let refs: Vec<&Value> = recs.iter().collect();
        check_session(&refs)
    };
    for b in [24u32, 32, 28] {
        let r = chk(v::load_counted(b, "counted"));
        println!("P{b}-counted records {} scalar_ok {} all_ok {}", r["records"], r["scalar_ok"], r["all_ok"]);
        res.set(format!("P{b}-counted"), r);
        if b == 28 {
            res.set("P28-wall(descriptive)", chk(v::load_wall_counts_only(28)));
        } else {
            res.set(format!("P{b}-wall"), chk(v::load_counted(b, "wall")));
        }
    }
    py::dump(&res.val(), out);
}
