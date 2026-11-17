//! The tournament's certificate checker (`oracle.py`), natively, in the
//! three frozen versions schema-1 rounds pinned:
//!
//! - `9b760afa…` (rounds 0019 and 0020): odd degrees 5..31, the report's
//!   summand count equal to the arm's;
//! - `528c14b3…` (rounds 0023 and 0024): degrees widened to 5..61;
//! - `ae8f9b2a…` (round 0025): additionally a report may declare fewer
//!   summands than its arm (never fewer than three), recorded in the
//!   certificate as `relation_summands`.
//!
//! The checks, their order and their messages are `oracle.py`'s.  The
//! arithmetic is this file's own (shift-and-add products over the fixture's
//! modulus); nothing here calls the library the worker is built from.  The
//! one change of method is the subgroup-order primality test, a
//! deterministic Miller-Rabin in place of trial division: the predicate is
//! the same, so every verdict is.

use std::collections::{HashMap, HashSet};

use super::json::{py_eq, J};
use super::tournament_store::sha256_hex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Version {
    V9b760afa,
    V528c14b3,
    Vae8f9b2a,
}

pub const ORACLE_9B760AFA: &str =
    "9b760afa8fedfacf801596e0d263e5a4197f2e8a5649476edc3305bbe8df0cbc";
pub const ORACLE_528C14B3: &str =
    "528c14b3e75389758be83d7af07e9cae031898b341939a480e5a749fad9f0e58";
pub const ORACLE_AE8F9B2A: &str =
    "ae8f9b2a21b54dd28c6eada6d84ebd27df77a0986cd96a13c24ffb3c9f99510b";

impl Version {
    pub fn from_sha256(sha: &str) -> Option<Version> {
        match sha {
            ORACLE_9B760AFA => Some(Version::V9b760afa),
            ORACLE_528C14B3 => Some(Version::V528c14b3),
            ORACLE_AE8F9B2A => Some(Version::Vae8f9b2a),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn sha256(self) -> &'static str {
        match self {
            Version::V9b760afa => ORACLE_9B760AFA,
            Version::V528c14b3 => ORACLE_528C14B3,
            Version::Vae8f9b2a => ORACLE_AE8F9B2A,
        }
    }

    fn max_degree(self) -> i128 {
        match self {
            Version::V9b760afa => 31,
            _ => 61,
        }
    }
}

const MIN_SUMMANDS: i128 = 3;

/// `InvalidEvidence`, and the other errors `oracle.py` lets escape.
pub fn require(cond: bool, message: &str) -> Result<(), String> {
    if cond {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

/// Python's `int(x)` of a JSON value: an integer, or a decimal string.
pub fn py_int(v: &J) -> Result<i128, String> {
    match v {
        J::Int(i) => Ok(*i),
        J::Bool(b) => Ok(i128::from(*b)),
        J::Float(f) if f.is_finite() => Ok(f.trunc() as i128),
        J::Str(s) => {
            let t = s.trim().replace('_', "");
            t.parse::<i128>()
                .map_err(|_| format!("invalid literal for int() with base 10: {s:?}"))
        }
        _ => Err("int() argument must be a string or a number".into()),
    }
}

/// `type(x) is int` for a JSON value.
fn exact_int(v: Option<&J>) -> Option<i128> {
    match v {
        Some(J::Int(i)) => Some(*i),
        _ => None,
    }
}

fn get<'a>(v: &'a J, key: &str) -> Result<&'a J, String> {
    v.get(key).ok_or_else(|| format!("KeyError: '{key}'"))
}

pub type Point = Option<(u64, u64)>;

fn bits(a: u128) -> u32 {
    128 - a.leading_zeros()
}

fn mulmod(a: u128, b: u128, m: u128) -> u128 {
    // Operands are below 2^64, so the product fits.
    (a % m) * (b % m) % m
}

fn powmod(mut a: u128, mut e: u128, m: u128) -> u128 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}

/// Deterministic Miller-Rabin for n < 2^64 (the first twelve prime bases).
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n.is_multiple_of(p) {
            return n == p;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'bases: for a in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'bases;
            }
        }
        return false;
    }
    true
}

fn inverse_mod(a: u128, r: u128) -> Result<u128, String> {
    let (mut t, mut nt) = (0i128, 1i128);
    let (mut rr, mut nr) = (r as i128, (a % r) as i128);
    while nr != 0 {
        let q = rr / nr;
        (t, nt) = (nt, t - q * nt);
        (rr, nr) = (nr, rr - q * nr);
    }
    require(rr == 1, "base is not invertible for the given modulus")?;
    Ok(t.rem_euclid(r as i128) as u128)
}

pub struct Curve {
    pub n: u32,
    pub a: u64,
    pub r: u128,
    pub h: u128,
    modulus: u64,
    pub g: Point,
    lam: u128,
}

impl Curve {
    pub fn new(fixture: &J, version: Version) -> Result<Curve, String> {
        let n = py_int(get(fixture, "degree")?)?;
        let a = py_int(get(fixture, "curve_a")?)?;
        let r = py_int(get(fixture, "subgroup_order")?)?;
        require(
            5 <= n && n <= version.max_degree() && n % 2 == 1,
            "unsupported degree",
        )?;
        require(a == 0 || a == 1, "unsupported coefficient")?;
        let irr = get(fixture, "irreducible")?;
        let terms_j = get(irr, "low_terms")?;
        require(
            py_eq(get(irr, "degree")?, &J::Int(n)),
            "field degree mismatch",
        )?;
        let terms: Vec<i128> = terms_j
            .as_arr()
            .ok_or("bad modulus")?
            .iter()
            .map(|t| exact_int(Some(t)).ok_or_else(|| "bad modulus".to_string()))
            .collect::<Result<_, _>>()?;
        let distinct: HashSet<i128> = terms.iter().copied().collect();
        require(
            distinct.len() == terms.len() && terms.iter().all(|&t| 0 <= t && t < n),
            "bad modulus",
        )?;
        let n = n as u32;
        let modulus = terms.iter().fold(1u64 << n, |m, &t| m | (1u64 << t));
        require(
            r > 2 && r < (1i128 << 64) && is_prime(r as u128),
            "nonprime subgroup",
        )?;
        let t: i128 = if a == 0 { -1 } else { 1 };
        let (mut s0, mut s1): (i128, i128) = (2, t);
        for _ in 1..n {
            (s0, s1) = (s1, t * s1 - 2 * s0);
        }
        let order = (1i128 << n) + 1 - s1;
        require(
            order == py_int(get(fixture, "group_order")?)?,
            "wrong group order",
        )?;
        let h = py_int(get(fixture, "cofactor")?)?;
        require(order == h * r, "wrong cofactor")?;
        let mut c = Curve {
            n,
            a: a as u64,
            r: r as u128,
            h: h as u128,
            modulus,
            g: None,
            lam: 0,
        };
        let g = c.decode(get(fixture, "generator")?)?;
        require(g.is_some() && c.mul(g, c.r)?.is_none(), "bad generator")?;
        c.g = g;
        let lam = py_int(get(fixture, "lambda")?)?;
        require(lam >= 0, "negative scalar")?;
        c.lam = lam as u128;
        require(c.mul(g, c.lam)? == c.frob(g), "wrong Frobenius eigenvalue")?;
        Ok(c)
    }

    fn fm(&self, mut a: u64, mut b: u64) -> u64 {
        let mut z = 0u64;
        while b != 0 {
            if b & 1 == 1 {
                z ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a >> self.n != 0 {
                a ^= self.modulus;
            }
        }
        z
    }

    fn inv(&self, x: u64) -> Result<u64, String> {
        require(x != 0, "zero denominator")?;
        let (mut u, mut v, mut a, mut b) = (x as u128, self.modulus as u128, 1u128, 0u128);
        while u != 1 {
            require(u != 0, "reducible field modulus")?;
            let mut shift = bits(u) as i64 - bits(v) as i64;
            if shift < 0 {
                std::mem::swap(&mut u, &mut v);
                std::mem::swap(&mut a, &mut b);
                shift = -shift;
            }
            u ^= v << shift;
            a ^= b << shift;
        }
        while bits(a) > self.n {
            a ^= (self.modulus as u128) << (bits(a) - self.n - 1);
        }
        Ok(a as u64)
    }

    pub fn decode(&self, p: &J) -> Result<Point, String> {
        if matches!(p, J::Null) {
            return Ok(None);
        }
        let pair = p
            .as_arr()
            .filter(|v| v.len() == 2)
            .ok_or("malformed point")?;
        let (x, y) = (py_int(&pair[0])?, py_int(&pair[1])?);
        let top = 1i128 << self.n;
        require(
            (0..top).contains(&x) && (0..top).contains(&y),
            "point outside field",
        )?;
        let (x, y) = (x as u64, y as u64);
        let xx = self.fm(x, x);
        require(
            self.fm(y, y) ^ self.fm(x, y) == self.fm(xx, x) ^ self.fm(self.a, xx) ^ 1,
            "point does not lift to curve",
        )?;
        Ok(Some((x, y)))
    }

    fn neg(&self, p: Point) -> Point {
        p.map(|(x, y)| (x, x ^ y))
    }

    fn frob(&self, p: Point) -> Point {
        p.map(|(x, y)| (self.fm(x, x), self.fm(y, y)))
    }

    fn add(&self, p: Point, q: Point) -> Result<Point, String> {
        let (Some((x, y)), Some((u, v))) = (p, q) else {
            return Ok(p.or(q));
        };
        if x == u {
            if y != v || x == 0 {
                return Ok(None);
            }
            let lam = x ^ self.fm(y, self.inv(x)?);
            let z = self.fm(lam, lam) ^ lam ^ self.a;
            return Ok(Some((z, self.fm(x, x) ^ self.fm(lam ^ 1, z))));
        }
        let lam = self.fm(y ^ v, self.inv(x ^ u)?);
        let z = self.fm(lam, lam) ^ lam ^ x ^ u ^ self.a;
        Ok(Some((z, self.fm(lam, x ^ z) ^ z ^ y)))
    }

    fn mul(&self, mut p: Point, mut k: u128) -> Result<Point, String> {
        let mut q = None;
        while k != 0 {
            if k & 1 == 1 {
                q = self.add(q, p)?;
            }
            p = self.add(p, p)?;
            k >>= 1;
        }
        Ok(q)
    }
}

/// `oracle.rank`: incremental echelon form modulo `r`, row by row.
fn rank(rows: &[Vec<u128>], columns: usize, r: u128) -> Result<usize, String> {
    let mut pivots: HashMap<usize, Vec<u128>> = HashMap::new();
    for original in rows {
        let mut row = original.clone();
        for j in 0..columns {
            if row[j] == 0 {
                continue;
            }
            if let Some(p) = pivots.get(&j) {
                let c = row[j];
                row = row
                    .iter()
                    .zip(p)
                    .map(|(&a, &b)| (a + r - mulmod(c, b, r)) % r)
                    .collect();
            } else {
                let inv = inverse_mod(row[j], r)?;
                pivots.insert(j, row.iter().map(|&a| mulmod(a, inv, r)).collect());
                break;
            }
        }
    }
    Ok(pivots.len())
}

fn index_of(v: Option<&J>) -> Option<i128> {
    match v {
        Some(J::Int(i)) => Some(*i),
        Some(J::Bool(b)) => Some(i128::from(*b)),
        _ => None,
    }
}

/// `oracle.verify(report, expected_fixture, expected_mode=…, summands=…)`.
pub fn verify(
    report: &J,
    fixture: &J,
    expected_mode: &str,
    summands: &J,
    version: Version,
) -> Result<J, String> {
    require(
        report
            .get("schema_version")
            .is_some_and(|v| py_eq(v, &J::Int(1))),
        "wrong schema",
    )?;
    require(
        report.get("status") == Some(&J::Str("complete".into())),
        "incomplete workload",
    )?;
    require(
        report.get("mode") == Some(&J::Str(expected_mode.into())),
        "wrong algorithm mode",
    )?;
    require(
        report.get("fixture").is_some_and(|f| py_eq(f, fixture)),
        "changed curve or target",
    )?;
    require(
        fixture.get("target_scalar_constructed") == Some(&J::Bool(false)),
        "planted scalar supplied",
    )?;
    let c = Curve::new(fixture, version)?;
    let targets: Vec<Point> = get(fixture, "targets")?
        .as_arr()
        .ok_or("TypeError: targets")?
        .iter()
        .map(|p| c.decode(p))
        .collect::<Result<_, _>>()?;
    let mut ok = true;
    for q in &targets {
        ok = ok && q.is_some() && c.mul(*q, c.r)?.is_none();
    }
    require(ok, "bad subgroup target")?;
    let empty = Vec::new();
    let solutions: &[J] = match report.get("solutions") {
        None => &empty,
        Some(s) => s.as_arr().ok_or("TypeError: solutions")?,
    };
    require(
        solutions.len() == targets.len(),
        "missing or extra solutions",
    )?;
    let indices: HashSet<Option<i128>> =
        solutions.iter().map(|s| index_of(s.get("index"))).collect();
    let want: HashSet<Option<i128>> = (0..targets.len() as i128).map(Some).collect();
    require(indices == want, "duplicate solution indices")?;
    for s in solutions {
        require(
            matches!(s.get("recovered"), Some(J::Str(_))),
            "missing scalar",
        )?;
        let d = py_int(get(s, "recovered")?)?;
        let idx = index_of(s.get("index")).ok_or("TypeError: index")? as usize;
        require(
            0 <= d && (d as u128) < c.r && c.mul(c.g, d as u128)? == targets[idx],
            "incorrect scalar",
        )?;
    }
    let mut ordered: Vec<&J> = solutions.iter().collect();
    ordered.sort_by_key(|s| index_of(s.get("index")));
    let recovered = J::Arr(
        ordered
            .iter()
            .map(|s| s.get("recovered").cloned().unwrap_or(J::Null))
            .collect(),
    );
    if expected_mode == "rho" {
        require(
            report
                .get("automorphism_order")
                .is_some_and(|v| py_eq(v, &J::Int(2 * c.n as i128))),
            "rho quotient mismatch",
        )?;
        return Ok(J::Obj(vec![
            ("verified_targets".into(), J::Int(targets.len() as i128)),
            ("verified_relations".into(), J::Int(0)),
            ("rank".into(), J::Null),
            ("solutions".into(), recovered),
        ]));
    }
    require(
        report
            .get("rejected_relations")
            .is_some_and(|v| py_eq(v, &J::Int(0))),
        "rejected relation in run",
    )?;
    let declared = report.get("summands");
    let length: i128 = if version == Version::Vae8f9b2a {
        let equal = declared.is_some_and(|d| py_eq(d, summands));
        let configured = py_int(summands)?;
        let lower = exact_int(declared).is_some_and(|d| MIN_SUMMANDS <= d && d < configured);
        require(equal || lower, "changed summand count")?;
        if equal {
            configured
        } else {
            exact_int(declared).expect("checked")
        }
    } else {
        require(
            declared.is_some_and(|d| py_eq(d, summands)),
            "changed summand count",
        )?;
        py_int(summands)?
    };
    let configured = py_int(summands)?;
    let mut base: Vec<Point> = Vec::new();
    if let Some(orbits) = report.get("factor_base_orbits") {
        require(
            report.get("factor_base").is_none(),
            "both factor-base formats in one report",
        )?;
        let reps: Vec<Point> = orbits
            .as_arr()
            .ok_or("TypeError: factor_base_orbits")?
            .iter()
            .map(|p| c.decode(p))
            .collect::<Result<_, _>>()?;
        require(
            !reps.is_empty() && reps.iter().all(Option::is_some),
            "invalid orbit representatives",
        )?;
        for rep in reps {
            let mut q = rep;
            for _ in 0..c.n {
                base.push(q);
                base.push(c.neg(q));
                q = c.frob(q);
            }
            require(
                q == rep,
                "orbit representative does not close under Frobenius",
            )?;
        }
    } else {
        base = get(report, "factor_base")?
            .as_arr()
            .ok_or("TypeError: factor_base")?
            .iter()
            .map(|p| c.decode(p))
            .collect::<Result<_, _>>()?;
    }
    let distinct: HashSet<Point> = base.iter().copied().collect();
    require(
        !base.is_empty() && base.iter().all(Option::is_some) && distinct.len() == base.len(),
        "invalid factor base",
    )?;
    let convention = match report.get("column_convention") {
        None => "cofactor".to_string(),
        Some(J::Str(s)) => s.clone(),
        Some(_) => String::new(),
    };
    require(
        convention == "cofactor" || convention == "representative",
        "unknown column convention",
    )?;
    let cofactor = convention == "cofactor";
    let scale = if cofactor { c.h } else { 1 };
    let locate = |p: Point| -> Result<Point, String> {
        if cofactor {
            c.mul(p, c.h)
        } else {
            Ok(p)
        }
    };
    let logs: &[J] = match report.get("column_logs") {
        None => &empty,
        Some(l) => l.as_arr().ok_or("TypeError: column_logs")?,
    };
    require(
        report
            .get("columns")
            .is_some_and(|v| py_eq(v, &J::Int(logs.len() as i128)))
            && logs.len() > 1,
        "missing/nontrivial log matrix",
    )?;
    let mut log_values = Vec::with_capacity(logs.len());
    let mut mapping: HashMap<(u64, u64), (usize, u128)> = HashMap::new();
    for (j, entry) in logs.iter().enumerate() {
        let mut p = c.decode(get(entry, "point")?)?;
        let ell = py_int(get(entry, "log")?)?;
        require(
            0 <= ell && (ell as u128) < c.r && p.is_some() && c.mul(c.g, ell as u128)? == p,
            "bad column log",
        )?;
        log_values.push(ell as u128);
        let mut coeff = 1u128;
        for _ in 0..c.n {
            for (q, k) in [(p, coeff), (c.neg(p), (c.r - coeff % c.r) % c.r)] {
                let key = q.expect("nonzero");
                require(
                    mapping.get(&key).is_none_or(|v| *v == (j, k)),
                    "overlapping projected columns",
                )?;
                mapping.insert(key, (j, k));
            }
            p = c.frob(p);
            coeff = mulmod(coeff, c.lam, c.r);
        }
    }
    let mut projected = Vec::with_capacity(base.len());
    for &p in &base {
        let l = locate(p)?;
        let x = l.and_then(|key| mapping.get(&key).copied());
        require(x.is_some() || l.is_none(), "uncovered base column")?;
        projected.push(x);
    }
    let mut matrix = Vec::new();
    let mut seen: HashSet<(u128, Vec<i128>)> = HashSet::new();
    let rows: &[J] = match report.get("relations") {
        None => &empty,
        Some(r) => r.as_arr().ok_or("TypeError: relations")?,
    };
    for rel in rows {
        let ids_j = get(rel, "points")?.as_arr().ok_or("TypeError: points")?;
        let ids: Vec<Option<i128>> = ids_j.iter().map(|i| exact_int(Some(i))).collect();
        require(
            ids.len() as i128 == length
                && ids
                    .iter()
                    .all(|i| i.is_some_and(|i| 0 <= i && (i as usize) < base.len())),
            "bad relation indices",
        )?;
        let ids: Vec<i128> = ids.into_iter().map(|i| i.expect("checked")).collect();
        let a = py_int(get(rel, "a")?)?;
        require(
            0 < a && (a as u128) < c.r,
            "trivial or invalid relation scalar",
        )?;
        let a = a as u128;
        let mut q = None;
        let mut row = vec![0u128; logs.len()];
        for &i in &ids {
            q = c.add(q, base[i as usize])?;
            if let Some((j, k)) = projected[i as usize] {
                row[j] = (row[j] + k) % c.r;
            }
        }
        require(
            q.is_some() && q == c.mul(c.g, a)?,
            "incorrect point relation",
        )?;
        let total = row
            .iter()
            .zip(&log_values)
            .fold(0u128, |acc, (&x, &l)| (acc + mulmod(x, l, c.r)) % c.r);
        require(total == mulmod(scale, a, c.r), "incorrect scalar-field row")?;
        let mut key_ids = ids.clone();
        key_ids.sort_unstable();
        if seen.insert((a, key_ids)) {
            matrix.push(row);
        }
    }
    let achieved = rank(&matrix, logs.len(), c.r)?;
    require(achieved == logs.len(), "rank-deficient log recovery")?;
    require(
        report
            .get("accepted_relations")
            .is_some_and(|v| py_eq(v, &J::Int(seen.len() as i128))),
        "accepted-row accounting mismatch",
    )?;
    require(
        report
            .get("duplicate_relations")
            .is_some_and(|v| py_eq(v, &J::Int((rows.len() - seen.len()) as i128))),
        "duplicate-row accounting mismatch",
    )?;
    let mut degenerate = 0i128;
    for s in solutions {
        let rel = s.get("relation");
        require(
            matches!(rel, Some(J::Obj(_))),
            "missing descent relation: logarithm not certified as index calculus",
        )?;
        let rel = rel.expect("checked");
        let (a, b) = (exact_int(rel.get("a")), exact_int(rel.get("b")));
        require(
            a.is_some_and(|a| 0 <= a && (a as u128) < c.r)
                && b.is_some_and(|b| 0 < b && (b as u128) < c.r),
            "invalid descent scalars",
        )?;
        let (a, b) = (a.expect("checked") as u128, b.expect("checked") as u128);
        let ids: Option<Vec<Option<i128>>> = match rel.get("points") {
            Some(J::Arr(v)) => Some(v.iter().map(|i| exact_int(Some(i))).collect()),
            _ => None,
        };
        require(
            ids.as_ref().is_some_and(|ids| {
                (ids.is_empty() || ids.len() as i128 == length)
                    && ids
                        .iter()
                        .all(|i| i.is_some_and(|i| 0 <= i && (i as usize) < base.len()))
            }),
            "bad descent relation indices",
        )?;
        let ids: Vec<usize> = ids
            .expect("checked")
            .into_iter()
            .map(|i| i.expect("checked") as usize)
            .collect();
        let idx = index_of(s.get("index")).ok_or("TypeError: index")? as usize;
        let q = targets[idx];
        let probe = c.add(c.mul(c.g, a)?, c.mul(q, b)?)?;
        let mut total = None;
        let mut logsum = 0u128;
        for &i in &ids {
            total = c.add(total, base[i])?;
            if let Some((j, k)) = projected[i] {
                logsum = (logsum + mulmod(k, log_values[j], c.r)) % c.r;
            }
        }
        require(
            total == probe,
            "descent relation does not hold in the group",
        )?;
        let d = py_int(get(s, "recovered")?)?.rem_euclid(c.r as i128) as u128;
        let lhs = mulmod(scale, (a + mulmod(b, d, c.r)) % c.r, c.r);
        require(
            lhs == logsum,
            "logarithm is not the consequence of its descent relation",
        )?;
        degenerate += i128::from(ids.is_empty());
    }
    let mut sorted_base: Vec<(u64, u64)> = base.iter().map(|p| p.expect("checked")).collect();
    sorted_base.sort_unstable();
    let text = format!(
        "[{}]",
        sorted_base
            .iter()
            .map(|(x, y)| format!("[{x},{y}]"))
            .collect::<Vec<_>>()
            .join(",")
    );
    let mut proof = vec![
        (
            "verified_targets".to_string(),
            J::Int(targets.len() as i128),
        ),
        ("verified_relations".into(), J::Int(rows.len() as i128)),
        ("fresh_rows".into(), J::Int(seen.len() as i128)),
        ("certified_descents".into(), J::Int(solutions.len() as i128)),
        ("degenerate_descents".into(), J::Int(degenerate)),
        ("rank".into(), J::Int(achieved as i128)),
        ("signed_base_size".into(), J::Int(base.len() as i128)),
        (
            "factor_base_sha256".into(),
            J::Str(sha256_hex(text.as_bytes())),
        ),
        ("solutions".into(), recovered),
    ];
    if version == Version::Vae8f9b2a && length != configured {
        proof.push(("relation_summands".into(), J::Int(length)));
    }
    Ok(J::Obj(proof))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primality_matches_trial_division() {
        for n in 0u128..5000 {
            let trial = n > 1 && (2..n).take_while(|d| d * d <= n).all(|d| n % d != 0);
            assert_eq!(is_prime(n), trial, "{n}");
        }
        assert!(is_prime(2_305_843_009_213_693_951));
        assert!(!is_prime(3_215_031_751));
    }

    #[test]
    fn rank_of_small_matrices_mod_a_prime() {
        let r = 11;
        assert_eq!(rank(&[vec![1, 2], vec![2, 4]], 2, r).unwrap(), 1);
        assert_eq!(rank(&[vec![1, 2], vec![0, 3]], 2, r).unwrap(), 2);
        assert_eq!(rank(&[vec![0, 0], vec![5, 10]], 2, r).unwrap(), 1);
        assert_eq!(inverse_mod(3, 11).unwrap(), 4);
    }
}
