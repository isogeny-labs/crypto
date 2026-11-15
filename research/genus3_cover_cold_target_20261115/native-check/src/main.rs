//! Native arithmetic and summary checks for a frozen historical Python audit.
//! A successful local replay is not a signed PCC or execution-proof verdict.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};

mod solver;

const P: i64 = 1061;
const ELL: u64 = 298_607_311;
const ORDER: u64 = 1_194_429_244;
type F = [i64; 3];
type Point = Option<(F, F)>;
const ZERO: F = [0, 0, 0];
const ONE: F = [1, 0, 0];

fn add(a: F, b: F) -> F {
    std::array::from_fn(|i| (a[i] + b[i]).rem_euclid(P))
}
fn neg(a: F) -> F {
    a.map(|x| (-x).rem_euclid(P))
}
fn sub(a: F, b: F) -> F {
    add(a, neg(b))
}
fn scale(a: F, c: i64) -> F {
    a.map(|x| (x * c).rem_euclid(P))
}
fn mul(a: F, b: F) -> F {
    // Schoolbook convolution followed by polynomial reduction. This differs
    // from the producer's closed-form multiplication and adjugate inverse.
    let mut c = [0_i64; 5];
    for i in 0..3 {
        for j in 0..3 {
            c[i + j] += a[i] * b[j];
        }
    }
    for i in (3..5).rev() {
        let t = c[i];
        c[i - 3] -= 65 * t;
        c[i - 2] -= t;
    }
    [c[0].rem_euclid(P), c[1].rem_euclid(P), c[2].rem_euclid(P)]
}
fn pow(mut a: F, mut n: u64) -> F {
    let mut r = ONE;
    while n != 0 {
        if n & 1 != 0 {
            r = mul(r, a);
        }
        a = mul(a, a);
        n >>= 1;
    }
    r
}
fn inv(a: F) -> F {
    assert_ne!(a, ZERO);
    pow(a, (P as u64).pow(3) - 2)
}
fn div(a: F, b: F) -> F {
    mul(a, inv(b))
}

struct Curve {
    a2: F,
    a4: F,
}
impl Curve {
    fn new() -> Self {
        let a = [0, 1, 0];
        let b = pow(a, P as u64);
        Self {
            a2: neg(add(a, b)),
            a4: mul(a, b),
        }
    }
    fn contains(&self, q: Point) -> bool {
        q.is_none_or(|(x, y)| mul(y, y) == mul(x, add(mul(x, add(x, self.a2)), self.a4)))
    }
    fn add(&self, a: Point, b: Point) -> Point {
        let (x, y) = match a {
            Some(q) => q,
            None => return b,
        };
        let (u, v) = match b {
            Some(q) => q,
            None => return a,
        };
        if x == u && add(y, v) == ZERO {
            return None;
        }
        let m = if a == b {
            div(
                add(add(scale(mul(x, x), 3), scale(mul(self.a2, x), 2)), self.a4),
                scale(y, 2),
            )
        } else {
            div(sub(v, y), sub(u, x))
        };
        let z = sub(sub(sub(mul(m, m), self.a2), x), u);
        Some((z, sub(mul(m, sub(x, z)), y)))
    }
    fn scalar(&self, mut n: u64, mut q: Point) -> Point {
        let mut r = None;
        while n != 0 {
            if n & 1 != 0 {
                r = self.add(r, q);
            }
            q = self.add(q, q);
            n >>= 1;
        }
        r
    }
}
fn require(ok: bool, message: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn field(v: &Value) -> Result<F, String> {
    let a = v.as_array().ok_or("field element is not an array")?;
    require(a.len() == 3, "field element length")?;
    let mut r = ZERO;
    for i in 0..3 {
        r[i] = a[i].as_i64().ok_or("noninteger field coefficient")?;
        require((0..P).contains(&r[i]), "noncanonical field coefficient")?;
    }
    Ok(r)
}
fn point(v: &Value) -> Result<Point, String> {
    if v.is_null() {
        return Ok(None);
    }
    let a = v.as_array().ok_or("point is not an array")?;
    require(a.len() == 2, "point length")?;
    Ok(Some((field(&a[0])?, field(&a[1])?)))
}
fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn median(v: &[f64]) -> f64 {
    let mut a = v.to_vec();
    a.sort_by(f64::total_cmp);
    (a[(a.len() - 1) / 2] + a[a.len() / 2]) / 2.0
}
fn check_number(s: &Value, name: &str, got: f64) -> Result<(), String> {
    let expected = s[name].as_f64().ok_or_else(|| format!("missing {name}"))?;
    require(
        (expected - got).abs() < 1e-12,
        &format!("summary mismatch: {name}"),
    )
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn check(data: &Value, archive: &Path) -> Result<Value, String> {
    let s = &data["summary"];
    require(s["curve"]["q"] == P, "unsupported historical fixture")?;
    require(s["curve"]["subgroup_order"] == ELL, "subgroup mismatch")?;
    require(s["curve"]["group_order"] == ORDER, "group order mismatch")?;
    require(prime(P as u64) && prime(ELL), "primality")?;
    require(
        (0..P).all(|x| (x * x * x + x + 65) % P != 0),
        "cubic not irreducible",
    )?;
    let curve = Curve::new();
    let g = point(&s["curve"]["generator"])?;
    require(g.is_some() && curve.contains(g), "invalid generator")?;
    require(curve.scalar(ELL, g).is_none(), "generator subgroup order")?;
    let files = s["source_sha256"]
        .as_object()
        .ok_or("source hashes absent")?;
    let mut seen = BTreeMap::new();
    let file = fs::File::open(archive).map_err(|e| e.to_string())?;
    for entry in tar::Archive::new(file)
        .entries()
        .map_err(|e| e.to_string())?
    {
        let mut entry = entry.map_err(|e| e.to_string())?;
        let name = entry
            .path()
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .into_owned();
        require(files.contains_key(&name), "unexpected archived source")?;
        require(!seen.contains_key(&name), "duplicate archived source")?;
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        let digest = sha(&bytes);
        require(
            files[&name].as_str() == Some(digest.as_str()),
            "source hash mismatch",
        )?;
        seen.insert(name, digest);
    }
    require(seen.len() == files.len(), "incomplete source archive")?;
    let rows = data["raw_trials"].as_array().ok_or("raw trials absent")?;
    require(
        rows.len() == 64 && s["trials"] == 64,
        "historical trial count",
    )?;
    let mut ic = Vec::new();
    let mut bs = Vec::new();
    let mut wins = 0;
    let mut seeds = std::collections::BTreeSet::new();
    for (i, row) in rows.iter().enumerate() {
        require(row["trial"] == i, "trial sequence")?;
        let q = point(&row["target"])?;
        require(q.is_some() && curve.contains(q), "invalid recorded target")?;
        let log = row["recovered_log"].as_u64().ok_or("missing scalar")?;
        require(log > 0 && log < ELL, "scalar range")?;
        require(curve.scalar(log, g) == q, "scalar replay failed")?;
        let a = row["cover_seconds"].as_f64().ok_or("missing cover time")?;
        let b = row["direct_seconds"]
            .as_f64()
            .ok_or("missing direct time")?;
        require(
            a.is_finite() && b.is_finite() && a > 0.0 && b > 0.0,
            "invalid timing",
        )?;
        require(
            seeds.insert(row["relation_seed"].as_u64().ok_or("missing seed")?),
            "reused relation seed",
        )?;
        ic.push(a);
        bs.push(b);
        if a < b {
            wins += 1;
        }
    }
    check_number(s, "cover_mean_seconds", mean(&ic))?;
    check_number(s, "direct_mean_seconds", mean(&bs))?;
    check_number(s, "cover_median_seconds", median(&ic))?;
    check_number(s, "direct_median_seconds", median(&bs))?;
    check_number(s, "speedup_ratio_of_means", mean(&bs) / mean(&ic))?;
    require(s["cover_faster_trials"] == wins, "win count")?;
    let cert = &s["order_certificate"];
    let lo = cert["hasse_interval"][0].as_u64().ok_or("Hasse lower")?;
    let hi = cert["hasse_interval"][1].as_u64().ok_or("Hasse upper")?;
    let q = (P as u64).pow(3);
    let mut radius = (4 * q).isqrt();
    while (radius + 1) * (radius + 1) <= 4 * q {
        radius += 1;
    }
    require(lo == q + 1 - radius && hi == q + 1 + radius, "Hasse bounds")?;
    let certified = cert["certified_points"]
        .as_array()
        .ok_or("certified points")?;
    require(certified.len() == 1, "historical order certificate shape")?;
    let witness = point(&certified[0]["point"])?;
    let d = certified[0]["exact_point_order"]
        .as_u64()
        .ok_or("point order")?;
    require(d == 2 * ELL && curve.contains(witness), "order witness")?;
    require(
        curve.scalar(d, witness).is_none(),
        "order witness annihilation",
    )?;
    require(
        curve.scalar(d / 2, witness).is_some() && curve.scalar(d / ELL, witness).is_some(),
        "order witness exactness",
    )?;
    let first = lo.div_ceil(d) * d;
    require(first == ORDER && first + d > hi, "unique Hasse multiple")?;
    Ok(
        json!({"schema":"genus3.historical-audit-check/v1","status":"self_check_pass",
        "pcc_standing":"unverified","native_scalar_replays":rows.len(),
        "source_hashes_checked":seen,"primality_and_order_witness_checked":true,
        "recomputed":{"cover_mean_seconds":mean(&ic),"direct_mean_seconds":mean(&bs),
          "cover_median_seconds":median(&ic),"direct_median_seconds":median(&bs),
          "speedup_ratio_of_means":mean(&bs)/mean(&ic),"cover_faster_trials":wins},
        "not_checked":["historical execution or isolation","timing provenance",
          "the producer's NumPy bootstrap samples","signed independent PCC validation",
          "full native cover solver or matched counted Pollard rho"]}),
    )
}

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args
        .get(1)
        .is_some_and(|arg| arg == "--solve-public-target")
    {
        require(args.len() == 3, "usage: --solve-public-target REQUEST.json")?;
        return solver::public_target(&args[2]);
    }
    if std::env::args().any(|arg| arg == "--native-development-check") {
        return solver::development_check();
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let raw = root.join("historical/cold_target_audit_q1061_v65.json");
    let bytes = fs::read(raw).map_err(|e| e.to_string())?;
    let data: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let mut report = check(&data, &root.join("historical/source.tar"))?;
    report["raw_sha256"] = json!(sha(&bytes));
    report["status"] = json!("pass");
    report["status_scope"] =
        json!("local self-check only; unsigned and not independent PCC validation");
    report["raw_data"] =
        json!([{"path":"../historical/cold_target_audit_q1061_v65.json","sha256":sha(&bytes)}]);
    report["headline"]=json!(["cover_mean_seconds","direct_mean_seconds","cover_median_seconds",
        "direct_median_seconds","speedup_ratio_of_means","cover_faster_trials"].iter().map(|name|
        json!({"name":name,"claimed":data["summary"][name],"recomputed":report["recomputed"][name]})).collect::<Vec<_>>());
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, std::path::PathBuf) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        (
            serde_json::from_slice(
                &fs::read(root.join("historical/cold_target_audit_q1061_v65.json")).unwrap(),
            )
            .unwrap(),
            root.join("historical/source.tar"),
        )
    }
    #[test]
    fn minimal_polynomial_and_inverses() {
        let a = [0, 1, 0];
        assert_eq!(add(add(pow(a, 3), a), [65, 0, 0]), ZERO);
        for i in 1..80 {
            let x = [i, (i * i) % P, (7 * i + 3) % P];
            assert_eq!(mul(x, inv(x)), ONE);
        }
    }
    #[test]
    fn archived_audit_replays() {
        let (v, p) = fixture();
        assert!(check(&v, &p).is_ok());
    }
    #[test]
    fn wrong_scalar_rejected() {
        let (mut v, p) = fixture();
        v["raw_trials"][0]["recovered_log"] = json!(1);
        assert!(check(&v, &p).unwrap_err().contains("scalar replay"));
    }
    #[test]
    fn missing_trial_rejected() {
        let (mut v, p) = fixture();
        v["raw_trials"].as_array_mut().unwrap().pop();
        assert!(check(&v, &p).is_err());
    }
    #[test]
    fn edited_summary_rejected() {
        let (mut v, p) = fixture();
        v["summary"]["cover_mean_seconds"] = json!(0.0);
        assert!(check(&v, &p).unwrap_err().contains("summary mismatch"));
    }
    #[test]
    fn edited_source_pin_rejected() {
        let (mut v, p) = fixture();
        v["summary"]["source_sha256"]["cold_target.py"] = json!("00");
        assert!(check(&v, &p).unwrap_err().contains("source hash"));
    }
}
