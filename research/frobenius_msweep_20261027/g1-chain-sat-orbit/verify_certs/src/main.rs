//! Independent certificate verifier for cell G1: no SAT solver, no shared
//! code with the driver, no rerun.
//!
//! Usage: g1-verify-certs --rows ROWS_DIR --manifests ORACLE_DIR
//!
//! For every `found` row of every rows-*.jsonl, re-adds the m certificate
//! points on y^2 + xy = x^3 + a x^2 + b over GF(2^n) (polynomial basis,
//! modulus from the cell manifest) with its own field arithmetic and checks:
//! m points, each on the curve, each with an x-code in the manifest's factor
//! base, and their sum equal to the row's target. It also checks that the
//! manifest's factor-base points lie on the curve. Exit 0 only if every found
//! row passes, 1 otherwise, 2 on unreadable input. Prints one summary line
//! per rows file.

use serde_json::Value;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::process::exit;

struct Field {
    n: u32,
    modulus: u128,
}

impl Field {
    fn mul(&self, a: u64, mut b: u64) -> u64 {
        let mut r: u128 = 0;
        let mut a2 = a as u128;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a2;
            }
            b >>= 1;
            a2 <<= 1;
            if (a2 >> self.n) & 1 == 1 {
                a2 ^= self.modulus;
            }
        }
        r as u64
    }
    fn inv(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return None;
        }
        // a^(2^n - 2)
        let (mut r, mut e, mut base) = (1u64, (1u64 << self.n) - 2, a);
        while e != 0 {
            if e & 1 == 1 {
                r = self.mul(r, base);
            }
            base = self.mul(base, base);
            e >>= 1;
        }
        Some(r)
    }
}

type Pt = Option<(u64, u64)>; // None is the point at infinity

struct Curve {
    f: Field,
    a: u64,
    b: u64,
}

impl Curve {
    fn on(&self, p: Pt) -> bool {
        let Some((x, y)) = p else { return true };
        let f = &self.f;
        let x2 = f.mul(x, x);
        let lhs = f.mul(y, y) ^ f.mul(x, y);
        let rhs = f.mul(x2, x) ^ f.mul(self.a, x2) ^ self.b;
        lhs == rhs
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let Some((x1, y1)) = p else { return q };
        let Some((x2, y2)) = q else { return p };
        let f = &self.f;
        let (lam, x3);
        if x1 == x2 {
            if y1 ^ y2 == x1 {
                return None; // q = -p, including x = 0 doubling to infinity
            }
            lam = x1 ^ f.mul(y1, f.inv(x1)?);
            x3 = f.mul(lam, lam) ^ lam ^ self.a;
        } else {
            lam = f.mul(y1 ^ y2, f.inv(x1 ^ x2)?);
            x3 = f.mul(lam, lam) ^ lam ^ x1 ^ x2 ^ self.a;
        }
        let y3 = f.mul(lam, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }
}

fn pt(v: &Value) -> Option<Pt> {
    if v.is_null() {
        return Some(None);
    }
    let a = v.as_array()?;
    if a.len() != 2 {
        return None;
    }
    Some(Some((a[0].as_u64()?, a[1].as_u64()?)))
}

fn curve_of(man: &Value) -> Option<Curve> {
    let n = man["poly_degree"].as_u64()? as u32;
    if n == 0 || n > 63 {
        return None;
    }
    let mut modulus: u128 = 1u128 << n;
    for t in man["poly_low_terms"].as_array()? {
        modulus |= 1u128 << t.as_u64()?;
    }
    Some(Curve {
        f: Field { n, modulus },
        a: man["curve_a"].as_u64()?,
        b: man["curve_b"].as_u64()?,
    })
}

fn check_row(c: &Curve, fbx: &HashSet<u64>, m: usize, row: &Value) -> Result<(), &'static str> {
    let cert = row["cert_points"]
        .as_array()
        .ok_or("missing or wrong-length cert_points")?;
    if cert.len() != m {
        return Err("missing or wrong-length cert_points");
    }
    let mut acc: Pt = None;
    for raw in cert {
        let p = pt(raw).ok_or("certificate point malformed")?;
        let Some((x, _)) = p else {
            return Err("certificate point off the curve or infinity");
        };
        if !c.on(p) {
            return Err("certificate point off the curve or infinity");
        }
        if !fbx.contains(&x) {
            return Err("certificate x not in the factor base");
        }
        acc = c.add(acc, p);
    }
    let tgt = pt(&row["target_xy"]).ok_or("target missing or off the curve")?;
    if tgt.is_none() || !c.on(tgt) {
        return Err("target missing or off the curve");
    }
    if acc != tgt {
        return Err("sum of certificate points != target");
    }
    Ok(())
}

fn list(dir: &str, prefix: &str, suffix: &str) -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|n| n.starts_with(prefix) && n.ends_with(suffix))
                .map(|n| format!("{dir}/{n}"))
                .collect()
        })
        .unwrap_or_default();
    v.sort();
    v
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let (Some(rows_dir), Some(man_dir)) = (get("--rows"), get("--manifests")) else {
        eprintln!("usage: g1-verify-certs --rows ROWS_DIR --manifests ORACLE_DIR");
        exit(2);
    };
    let mut mans: BTreeMap<String, Value> = BTreeMap::new();
    for path in list(&man_dir, "manifest-", ".json") {
        let Ok(text) = fs::read_to_string(&path) else {
            exit(2)
        };
        let Ok(man) = serde_json::from_str::<Value>(&text) else {
            exit(2)
        };
        let Some(cell) = man["cell"].as_str() else {
            exit(2)
        };
        mans.insert(cell.to_string(), man);
    }
    let files = list(&rows_dir, "rows-", ".jsonl");
    if files.is_empty() || mans.is_empty() {
        eprintln!("no rows files or no manifests found");
        exit(2);
    }
    let mut bad = 0u64;
    for path in &files {
        let Ok(text) = fs::read_to_string(path) else {
            exit(2)
        };
        let (mut total, mut found) = (0u64, 0u64);
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let Ok(row) = serde_json::from_str::<Value>(line) else {
                exit(2)
            };
            total += 1;
            if row["verdict"].as_str() != Some("found") {
                continue;
            }
            found += 1;
            let cell = row["cell"].as_str().unwrap_or("");
            let Some(man) = mans.get(cell) else {
                println!("{path}: no manifest for cell {cell}");
                bad += 1;
                continue;
            };
            let Some(c) = curve_of(man) else {
                println!("{path}: unreadable manifest for cell {cell}");
                bad += 1;
                continue;
            };
            let fb_ok = man["factor_base_points"]
                .as_array()
                .is_some_and(|ps| ps.iter().all(|p| pt(p).is_some_and(|p| c.on(p))));
            if !fb_ok {
                println!("{path}: manifest factor-base point off the curve");
                bad += 1;
                continue;
            }
            let fbx: HashSet<u64> = man["factor_base_x_codes"]
                .as_array()
                .map(|xs| xs.iter().filter_map(|x| x.as_u64()).collect())
                .unwrap_or_default();
            let m = man["m"].as_u64().unwrap_or(0) as usize;
            if let Err(e) = check_row(&c, &fbx, m, &row) {
                println!("{path}: {}: {e}", row["target_id"].as_str().unwrap_or("?"));
                bad += 1;
            }
        }
        let name = path.rsplit('/').next().unwrap_or(path);
        println!("{name}: {total} rows, {found} found");
    }
    println!("VERIFY {} bad={bad}", if bad > 0 { "FAIL" } else { "OK" });
    exit(if bad > 0 { 1 } else { 0 });
}
