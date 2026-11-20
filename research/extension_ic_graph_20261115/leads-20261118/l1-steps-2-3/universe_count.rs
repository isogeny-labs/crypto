//! L1 step 2: exact count of the degree-2 class universe D_universe, and the
//! independent Mumford check of every D_exact class.
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/l1-steps-2-3.
//! Everything here is [pcc:unverified].
//!
//! The classes of the deferred graph are D = (U, V) with U = x^2 + u1 x + u0
//! monic irreducible over F_p and V = v0 + v1 x over F_{p^2} with
//! V^2 = H mod U, on y^2 = H(x) (H over F_{p^2}, monic of degree 7), keyed
//! canonically up to sign as [u0, u1, v0re, v0im, v1re, v1im] (lexicographic
//! minimum of V and -V; jv_quadratic_pair_relation::canonical).
//!
//! For each of the (p^2 - p)/2 monic irreducible U, U = (x - r)(x - r^p) over
//! F_{p^2} and F_{p^2}[x]/(U) = F_{p^2} x F_{p^2}, so the number of V is
//! (1 + chi(H(r))) (1 + chi(H(r^p))), chi the quadratic character of F_{p^2}.
//! Two independent methods:
//!   (a) that character product (Euler criterion);
//!   (b) explicit construction: square roots from a brute-force table of
//!       F_{p^2}, interpolation of V through (r, y1), (r^p, y2), and a direct
//!       polynomial check V^2 - H = 0 mod U; V's are counted and canonicalized.
//! (a) and (b) must agree for every U. Canonical classes (up to sign) are
//! counted by (b): V and -V share a key, V = 0 only when H(r) = H(r^p) = 0.
//!
//! H is read from the Run50-rerun collection record ("h", as encoded by the
//! Run50 module) and must equal the library's ctx.cov.hx for the public target.
//! The factor-base points recorded there are checked on y^2 = H(x) with this
//! file's own F_{p^2}, which pins the [re, im] presentation.
//!
//! Run: universe_count PUBLIC_JSON RUN50_SUMMARY DEXACT_JSONL OUT_JSON
#[path = "ffield.rs"]
mod ff;
use crypto_lib::cryptanalysis::jv_cover::{self as cover, Ctx, PtE6, E2, E6};
use ff::{is_qr, mulm, E, F2};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashSet};
use std::io::{BufRead, Write};

fn e2(v: &Value) -> Result<E2, String> {
    let a = v.as_array().ok_or("e2")?;
    Ok(E2([a[0].as_u64().ok_or("e2")?, a[1].as_u64().ok_or("e2")?]))
}
fn e6(v: &Value) -> Result<E6, String> {
    let a = v.as_array().ok_or("e6")?;
    Ok(E6([e2(&a[0])?, e2(&a[1])?, e2(&a[2])?]))
}
fn pt(v: &Value) -> Result<PtE6, String> {
    Ok(PtE6 { x: e6(&v["x"])?, y: e6(&v["y"])?, inf: v["inf"].as_bool().ok_or("inf")? })
}
fn pair(v: &Value) -> Result<E, String> {
    Ok((v[0].as_u64().ok_or("pair")?, v[1].as_u64().ok_or("pair")?))
}
pub fn canonical(f: &F2, u0: u64, u1: u64, v0: E, v1: E) -> ([u64; 6], i8) {
    let key = [u0, u1, v0.0, v0.1, v1.0, v1.1];
    let (n0, n1) = (f.neg(v0), f.neg(v1));
    let neg = [u0, u1, n0.0, n0.1, n1.0, n1.1];
    if neg < key {
        (neg, -1)
    } else {
        (key, 1)
    }
}

fn main() -> Result<(), String> {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 5 {
        return Err("usage: universe_count PUBLIC_JSON RUN50_SUMMARY DEXACT_JSONL OUT_JSON".into());
    }
    let public: Value = serde_json::from_slice(&std::fs::read(&a[1]).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let summary: Value = serde_json::from_slice(&std::fs::read(&a[2]).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let p = public["p"].as_u64().ok_or("p")?;
    let l = public["l"].as_u64().ok_or("l")?;
    let spec = cover::spec_from_curve(p, e6(&public["alpha"])?, l, pt(&public["g"])?, pt(&public["q"])?, 0)
        .ok_or("public transfer failed")?;
    let ctx = Ctx::new(&spec);
    let h: Vec<E> = summary["h"].as_array().ok_or("h")?.iter().map(pair).collect::<Result<_, _>>()?;
    let lib_h: Vec<E> = ctx.cov.hx.iter().map(|c| (c.0[0], c.0[1])).collect();
    let h_matches_library = h == lib_h;
    if !h_matches_library {
        return Err("recorded H differs from library cover".into());
    }
    if h.len() != 8 || h[7] != (1, 0) {
        return Err("H must be monic of degree 7".into());
    }
    let f = F2::new(p);
    // presentation check on recorded factor-base points
    let mut base_ok = 0usize;
    for b in summary["base"].as_array().ok_or("base")? {
        let x = b["x"].as_u64().ok_or("x")?;
        let y = pair(&b["y"])?;
        if f.mul(y, y) != f.peval(&h, f.from_fp(x)) {
            return Err("factor-base point not on y^2 = H(x) in own F_{p^2}".into());
        }
        base_ok += 1;
    }
    let table = f.sqrt_table();
    let inv2 = ff::invm(2, p);
    let (mut n_u, mut n_v_a, mut n_v_b, mut n_keys) = (0u64, 0u64, 0u64, 0u64);
    let mut by_count: BTreeMap<u64, u64> = BTreeMap::new();
    let mut ramified_u = 0u64; // U with H(r) = 0 or H(r^p) = 0
    let mut keys: HashSet<[u64; 6]> = HashSet::new();
    let mut disagreements = 0u64;
    for u1 in 0..p {
        for u0 in 0..p {
            let disc = (mulm(u1, u1, p) + p - mulm(4, u0, p)) % p;
            if is_qr(disc, p) {
                continue; // reducible (or a square) over F_p
            }
            n_u += 1;
            // sqrt(disc) = s*w with s^2 = disc/omega
            let t = mulm(disc, ff::invm(f.w, p), p);
            let s = (1..p).find(|&s| mulm(s, s, p) == t).ok_or("sqrt in F_p")?;
            let re = mulm((p - u1) % p, inv2, p);
            let im = mulm(s, inv2, p);
            let (r, rb) = ((re, im), (re, (p - im) % p));
            let (hr, hrb) = (f.peval(&h, r), f.peval(&h, rb));
            let count_a = ((1 + f.chi(hr) as i64) * (1 + f.chi(hrb) as i64)) as u64;
            ramified_u += u64::from(hr == (0, 0) || hrb == (0, 0));
            // (b) explicit construction
            let ys = |z: E| -> Vec<E> {
                match f.sqrt(&table, z) {
                    None => vec![],
                    Some(y) if y == (0, 0) => vec![y],
                    Some(y) => vec![y, f.neg(y)],
                }
            };
            let uu: Vec<E> = vec![(u0, 0), (u1, 0), (1, 0)];
            let hmod = f.prem(&h, &uu);
            let mut count_b = 0u64;
            let dinv = f.inv(f.sub(r, rb));
            for &y1 in &ys(hr) {
                for &y2 in &ys(hrb) {
                    let v1 = f.mul(f.sub(y1, y2), dinv);
                    let v0 = f.sub(y1, f.mul(v1, r));
                    let vv = f.pmul(&[v0, v1], &[v0, v1]);
                    if f.prem(&f.psub(&vv, &hmod), &uu).is_empty() {
                        count_b += 1;
                        let (k, _) = canonical(&f, u0, u1, v0, v1);
                        if keys.insert(k) {
                            n_keys += 1;
                        }
                    }
                }
            }
            if count_a != count_b {
                disagreements += 1;
            }
            n_v_a += count_a;
            n_v_b += count_b;
            *by_count.entry(count_b).or_default() += 1;
        }
    }
    // ---- independent Mumford check of D_exact ----
    let file = std::fs::File::open(&a[3]).map_err(|e| e.to_string())?;
    let (mut d_exact, mut d_valid, mut d_in_universe, mut d_noncanonical) = (0u64, 0u64, 0u64, 0u64);
    for line in std::io::BufReader::new(file).lines() {
        let line = line.map_err(|e| e.to_string())?;
        let v: Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
        let k: Vec<u64> = v["class"].as_array().ok_or("class")?.iter().map(|x| x.as_u64().unwrap()).collect();
        d_exact += 1;
        let (u0, u1) = (k[0], k[1]);
        let (v0, v1) = ((k[2], k[3]), (k[4], k[5]));
        let disc = (mulm(u1, u1, p) + p - mulm(4, u0, p)) % p;
        let uu: Vec<E> = vec![(u0, 0), (u1, 0), (1, 0)];
        let vv = f.pmul(&[v0, v1], &[v0, v1]);
        let valid = !is_qr(disc, p)
            && k.iter().all(|&x| x < p)
            && f.prem(&f.psub(&vv, &h), &uu).is_empty();
        d_valid += u64::from(valid);
        let karr: [u64; 6] = core::array::from_fn(|i| k[i]);
        d_in_universe += u64::from(keys.contains(&karr));
        d_noncanonical += u64::from(canonical(&f, u0, u1, v0, v1).0 != karr);
    }
    let p2 = p as f64;
    let out = json!({
        "schema":"l1-steps-2-3.universe/v1","pcc":"unverified","p":p,"omega":f.w,
        "h":h,"h_matches_library_cover":h_matches_library,"factor_base_points_checked":base_ok,
        "monic_irreducible_U":n_u,"expected_monic_irreducible_U":(p*p - p)/2,
        "signed_V_total_method_a":n_v_a,"signed_V_total_method_b":n_v_b,"method_disagreements":disagreements,
        "U_by_number_of_V":by_count,"U_with_ramified_root":ramified_u,
        "canonical_classes_up_to_sign":n_keys,
        "heuristic_p2_over_2":p2*p2/2.0,
        "d_exact_classes":d_exact,"d_exact_mumford_valid":d_valid,"d_exact_in_universe":d_in_universe,"d_exact_noncanonical_keys":d_noncanonical,
        "f_canonical":d_exact as f64 / n_keys as f64,
    });
    let mut w = std::fs::OpenOptions::new().write(true).create_new(true).open(&a[4]).map_err(|e| e.to_string())?;
    writeln!(w, "{}", serde_json::to_string_pretty(&out).unwrap()).map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    Ok(())
}
