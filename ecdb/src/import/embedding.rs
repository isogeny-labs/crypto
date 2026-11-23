//! Exact embedding degrees k = ord_r(q) of the prime-order subgroups.
//!
//! For every `subgroup` of prime order r (`is_prime_order`, primality
//! `proved` or `probable`) over a finite field of q elements, ecdb computes
//! the multiplicative order of q modulo r and stores it in `pairing_data`
//! where no embedding degree is held yet (no row, or `embedding_degree` NULL).
//! Nothing is deleted: an existing exact degree is never touched, a lower
//! bound is only ever raised, and a row a producer marked `not_applicable` is
//! left as it is.
//!
//! # Mathematics
//!
//! k divides r - 1 (Fermat: q^(r-1) = 1 mod r, checked). r - 1 is factored
//! by trial division by every prime p <= [`TRIAL_BOUND`] and then by
//! Pollard–Brent rho with a fixed step budget on what is left; each factor is
//! tested with `crate::ec::is_probable_prime` (deterministic Miller–Rabin
//! below 3.3 * 10^24, so `proved` there and `probable` above).
//!
//! * **r - 1 fully factored.** Start with k = r - 1 and, for each prime p of
//!   r - 1, divide k by p while q^(k/p) = 1 mod r. The result is the order.
//! * **A composite cofactor C is left** (r - 1 = F C, gcd(F, C) = 1, every
//!   prime factor of C above the trial bound T). Then k = k1 k2 with
//!   k1 = ord(q^C) | F, computed exactly as above, and k2 = ord(q^F) | C.
//!   If q^F = 1, k2 = 1 and k = k1 exactly. Otherwise k2 > 1 has only prime
//!   factors > T, so k > k1 T: ecdb stores `embedding_lower_bound = k1 T`
//!   with status `bounded` (k1 T is replaced by T when a prime of k1 is only
//!   probable or gcd(F, C) != 1, so the bound never rests on a probable
//!   prime). The stored bound B always means k > B, which also satisfies the
//!   weaker reading k >= B of the column comment.
//!
//! # Cross-checks (a failure is a problem in the Report and nothing is stored)
//!
//! * q^k = 1 mod r and q^(k/p) != 1 mod r for every prime p of k, whose
//!   prime powers must multiply back to k (so no proper divisor of k works:
//!   each one divides some k/p);
//! * independently of any factorization, q^i mod r for i = 1..[`SCAN`]: the
//!   first i with q^i = 1 must be k (when k <= SCAN), and none may exist
//!   when k > SCAN or only a bound k > B is known;
//! * against `pairing_data`: an existing embedding degree must equal the
//!   computed one (or, when only a bound is computed, exceed it and satisfy
//!   q^k = 1 mod r); an existing lower bound b must not exceed the computed k.
//!
//! # Status
//!
//! An exact k is `proved` when r is proved prime (`subgroup.primality_status`)
//! and every prime of k is proved prime, else `probable`. A bound is `bounded`.
//!
//! # Report
//!
//! `embedding_degree_targets`, `embedding_degree_exact_proved`,
//! `embedding_degree_exact_probable`, `embedding_degree_bounded`,
//! `embedding_degree_not_applicable` (r | q), `embedding_degree_inserted`,
//! `embedding_degree_updated`, `embedding_lower_bound_raised`,
//! `embedding_degree_existing_agree`, `embedding_degree_existing_consistent`
//! (only a bound computed, the existing k exceeds it and q^k = 1),
//! `embedding_degree_crosscheck_failures` (each also a problem).

use super::{bulk, Report};
use crate::ec::is_probable_prime;
use anyhow::Result;
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use tokio_postgres::Client;

/// Trial division of r - 1 by every prime up to this bound.
pub const TRIAL_BOUND: u32 = 1 << 20;
/// Pollard–Brent steps per polynomial x^2 + c, for c = 1..=RHO_TRIES, per composite.
pub const RHO_STEPS: u64 = 1 << 16;
pub const RHO_TRIES: u64 = 3;
/// The factorization-free scan of q^i mod r, i = 1..SCAN.
pub const SCAN: u64 = 2000;

/// The primes up to `n` (sieve of Eratosthenes).
pub fn primes_up_to(n: u32) -> Vec<u32> {
    let n = n as usize;
    let mut sieve = vec![true; n + 1];
    let mut out = vec![];
    for i in 2..=n {
        if sieve[i] {
            out.push(i as u32);
            let mut j = i * i;
            while j <= n {
                sieve[j] = false;
                j += i;
            }
        }
    }
    out
}

/// A prime factor with its exponent and whether its primality is proved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Factor {
    pub p: BigUint,
    pub e: u32,
    pub proved: bool,
}

/// n = prod p^e * cofactor; the cofactor is 1 or a composite with no prime
/// factor <= `trial_bound` that rho did not split.
#[derive(Clone, Debug)]
pub struct Partial {
    pub factors: Vec<Factor>,
    pub cofactor: BigUint,
    pub trial_bound: u32,
}

fn prime_test(n: &BigUint) -> (bool, bool) {
    is_probable_prime(&BigInt::from(n.clone()))
}

/// Pollard–Brent rho on odd composite n with x -> x^2 + c: a non-trivial factor or None.
pub fn pollard_brent(n: &BigUint, c: u64, steps: u64) -> Option<BigUint> {
    if n.is_even() {
        return Some(BigUint::from(2u8));
    }
    let c = BigUint::from(c);
    let f = |x: &BigUint| (x * x + &c) % n;
    let mut y = BigUint::from(2u8);
    let (mut r, m) = (1u64, 128u64);
    let mut q = BigUint::one();
    let mut g = BigUint::one();
    let (mut x, mut ys) = (y.clone(), y.clone());
    let mut done = 0u64;
    while g.is_one() {
        x = y.clone();
        for _ in 0..r {
            y = f(&y);
        }
        let mut k = 0;
        while k < r && g.is_one() {
            ys = y.clone();
            for _ in 0..m.min(r - k) {
                y = f(&y);
                let d = if x > y { &x - &y } else { &y - &x };
                q = (q * d) % n;
            }
            g = q.gcd(n);
            k += m;
        }
        done += r;
        r *= 2;
        if done > steps && g.is_one() {
            return None;
        }
    }
    if &g == n {
        // the batch overshot: step one at a time from its start
        g = BigUint::one();
        let mut guard = 0u64;
        while g.is_one() {
            ys = f(&ys);
            let d = if x > ys { &x - &ys } else { &ys - &x };
            g = d.gcd(n);
            guard += 1;
            if guard > steps + m {
                return None;
            }
        }
    }
    (g > BigUint::one() && &g < n).then_some(g)
}

/// Factor n by trial division up to `primes`' last element, then Pollard–Brent.
pub fn factor_partial(n: &BigUint, primes: &[u32], rho_steps: u64) -> Partial {
    let trial_bound = primes.last().copied().unwrap_or(1);
    let mut map: BTreeMap<BigUint, (u32, bool)> = BTreeMap::new();
    let mut rest = n.clone();
    for &p in primes {
        let pb = BigUint::from(p);
        if &pb * &pb > rest {
            break;
        }
        while (&rest % p).is_zero() {
            rest /= p;
            map.entry(pb.clone()).or_insert((0, true)).0 += 1;
        }
    }
    // what is left has no prime factor <= trial_bound
    let mut unsplit: Vec<BigUint> = vec![];
    let mut queue = if rest > BigUint::one() { vec![rest] } else { vec![] };
    while let Some(m) = queue.pop() {
        if m.is_one() {
            continue;
        }
        let (prime, proved) = prime_test(&m);
        if prime {
            let e = map.entry(m).or_insert((0, proved));
            e.0 += 1;
            continue;
        }
        match (1..=RHO_TRIES).find_map(|c| pollard_brent(&m, c, rho_steps)) {
            Some(g) => {
                let other = &m / &g;
                queue.push(g);
                queue.push(other);
            }
            None => unsplit.push(m),
        }
    }
    // a prime found in one branch may also divide an unsplit composite
    let mut cofactor: BigUint = unsplit.iter().product();
    for (p, (e, _)) in map.iter_mut() {
        while !cofactor.is_one() && (&cofactor % p).is_zero() {
            cofactor /= p;
            *e += 1;
        }
    }
    let factors = map.into_iter().map(|(p, (e, proved))| Factor { p, e, proved }).collect();
    Partial { factors, cofactor, trial_bound }
}

/// The order of g modulo r, given that g^n = 1 and the complete prime factorization of n.
pub fn order_from(g: &BigUint, r: &BigUint, n: &BigUint, factors: &[Factor]) -> BigUint {
    let mut k = n.clone();
    for f in factors {
        for _ in 0..f.e {
            if !(&k % &f.p).is_zero() {
                break;
            }
            let cand = &k / &f.p;
            if g.modpow(&cand, r).is_one() {
                k = cand;
            } else {
                break;
            }
        }
    }
    k
}

/// The prime factorization of k read off the factors of a multiple of k.
fn factors_of(k: &BigUint, of: &[Factor]) -> Vec<Factor> {
    let mut out = vec![];
    let mut m = k.clone();
    for f in of {
        let mut e = 0;
        while (&m % &f.p).is_zero() {
            m /= &f.p;
            e += 1;
        }
        if e > 0 {
            out.push(Factor { p: f.p.clone(), e, proved: f.proved });
        }
    }
    out
}

/// The cross-check of an exact order: q^k = 1, q^(k/p) != 1 for each listed
/// prime p, the prime powers multiply to k, and the factorization-free scan
/// agrees. Returns a reason on failure.
pub fn verify_order(q: &BigUint, r: &BigUint, k: &BigUint, factors: &[Factor]) -> Result<(), String> {
    if k.is_zero() {
        return Err("k = 0".into());
    }
    let qm = q % r;
    if !qm.modpow(k, r).is_one() {
        return Err(format!("r does not divide q^k - 1 for k = {k}"));
    }
    let product: BigUint = factors.iter().map(|f| num_traits::pow(f.p.clone(), f.e as usize)).product();
    if &product != k {
        return Err(format!("the prime powers of k = {k} multiply to {product}"));
    }
    for f in factors {
        if qm.modpow(&(k / &f.p), r).is_one() {
            return Err(format!("r divides q^(k/{}) - 1: k = {k} is not minimal", f.p));
        }
    }
    match first_power_one(&qm, r, SCAN) {
        Some(i) if BigUint::from(i) != *k => Err(format!("q^{i} = 1 mod r, but k = {k}")),
        None if *k <= BigUint::from(SCAN) => Err(format!("no i <= {SCAN} has q^i = 1 mod r, but k = {k}")),
        _ => Ok(()),
    }
}

/// The least i <= limit with g^i = 1 mod r.
pub fn first_power_one(g: &BigUint, r: &BigUint, limit: u64) -> Option<u64> {
    let g = g % r;
    let mut acc = BigUint::one() % r;
    for i in 1..=limit {
        acc = (&acc * &g) % r;
        if acc.is_one() {
            return Some(i);
        }
    }
    None
}

/// What [`embedding_degree`] found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// k = ord_r(q), cross-checked; `proved` when every prime of k is proved prime.
    Exact { k: BigUint, primes_proved: bool },
    /// Only k > lower is established.
    Bounded { lower: BigUint },
    /// r | q: no embedding degree.
    NotApplicable,
    /// A check failed: never stored.
    Failed(String),
}

/// ord_r(q) for prime r, or a proved lower bound when r - 1 does not factor.
pub fn embedding_degree(q: &BigUint, r: &BigUint, primes: &[u32], rho_steps: u64) -> Outcome {
    if *r < BigUint::from(2u8) {
        return Outcome::Failed("r < 2".into());
    }
    let qm = q % r;
    if qm.is_zero() {
        return Outcome::NotApplicable;
    }
    let one = BigUint::one();
    let rm1 = r - &one;
    if !qm.modpow(&rm1, r).is_one() {
        return Outcome::Failed("q^(r-1) != 1 mod r: r is not prime".into());
    }
    let part = factor_partial(&rm1, primes, rho_steps);
    let c = &part.cofactor;
    let f_part = &rm1 / c;
    if c.is_one() {
        let k = order_from(&qm, r, &rm1, &part.factors);
        let pk = factors_of(&k, &part.factors);
        return match verify_order(q, r, &k, &pk) {
            Ok(()) => Outcome::Exact { primes_proved: pk.iter().all(|f| f.proved), k },
            Err(e) => Outcome::Failed(e),
        };
    }
    // F = prod p^e exactly. If q^F = 1 then k | F and the order is exact.
    let g2 = qm.modpow(&f_part, r);
    if g2.is_one() {
        let k = order_from(&qm, r, &f_part, &part.factors);
        let pk = factors_of(&k, &part.factors);
        return match verify_order(q, r, &k, &pk) {
            Ok(()) => Outcome::Exact { primes_proved: pk.iter().all(|f| f.proved), k },
            Err(e) => Outcome::Failed(e),
        };
    }
    // Otherwise k / gcd(k, F) > 1 divides C, so k has a prime factor > T.
    // With gcd(F, C) = 1, k = k1 k2, k1 = ord(q^C) = gcd(k, F), k2 = ord(q^F) > T.
    let g1 = qm.modpow(c, r);
    let k1 = order_from(&g1, r, &f_part, &part.factors);
    let coprime = f_part.gcd(c).is_one();
    let t = BigUint::from(part.trial_bound);
    let k1_proved = factors_of(&k1, &part.factors).iter().all(|f| f.proved);
    let lower = if coprime && k1_proved { &k1 * &t } else { t };
    // independent of the factorization: no small power of q is 1
    if let Some(i) = first_power_one(&qm, r, SCAN) {
        return Outcome::Failed(format!("q^{i} = 1 mod r, but the factorization bounds k > {lower}"));
    }
    if lower < BigUint::from(SCAN) {
        return Outcome::Bounded { lower: BigUint::from(SCAN) };
    }
    Outcome::Bounded { lower }
}

// -------------------------------------------------------- the importer

const JOBS_SQL: &str = "
SELECT s.id, s.order_value::text, s.primality_status::text, f.order_q::text,
       pd.subgroup_id IS NOT NULL, pd.embedding_degree::text, pd.embedding_lower_bound::text, pd.status::text,
       pd.complement_bits, pd.target_field_bits, f.order_bits
FROM ec.subgroup s
JOIN ec.field f ON f.id = s.field_id
LEFT JOIN ec.pairing_data pd ON pd.subgroup_id = s.id
WHERE s.is_prime_order AND s.primality_status IN ('proved', 'probable') AND f.order_q IS NOT NULL
ORDER BY s.id";

// An exact degree: a new row, or the degree of a row that has none. The old
// lower bound stays (it remains true); the rho-value, complement and target
// field sizes are filled only if empty (`decide` refuses a held value that
// disagrees, and the WHERE clause repeats that guard).
const EXACT_SQL: &str = "
INSERT INTO ec.pairing_data AS pd (subgroup_id, embedding_degree, complement_bits, target_field_bits, rho_value, status,
       provenance_id)
SELECT s.id, r.degree, r.complement_bits,
       CASE WHEN r.degree < 1000000 THEN (r.degree * f.order_bits)::int END,
       round(f.order_bits::numeric / nullif(s.order_bits, 0), 6), r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(subgroup_id bigint, degree numeric, complement_bits int, status text)
JOIN ec.subgroup s ON s.id = r.subgroup_id
JOIN ec.field f ON f.id = s.field_id
ON CONFLICT (subgroup_id) DO UPDATE SET embedding_degree = EXCLUDED.embedding_degree,
       complement_bits = coalesce(pd.complement_bits, EXCLUDED.complement_bits),
       target_field_bits = coalesce(pd.target_field_bits, EXCLUDED.target_field_bits),
       rho_value = coalesce(pd.rho_value, EXCLUDED.rho_value), status = EXCLUDED.status,
       provenance_id = EXCLUDED.provenance_id
WHERE pd.embedding_degree IS NULL AND pd.status <> 'not_applicable'
  AND (pd.embedding_lower_bound IS NULL OR pd.embedding_lower_bound <= EXCLUDED.embedding_degree)
  AND (pd.complement_bits IS NULL OR pd.complement_bits = EXCLUDED.complement_bits)
  AND (pd.target_field_bits IS NULL OR EXCLUDED.target_field_bits IS NULL
       OR pd.target_field_bits = EXCLUDED.target_field_bits)";

// A lower bound k > B: a new row, or a larger bound on a row without a degree.
const BOUND_SQL: &str = "
INSERT INTO ec.pairing_data AS pd (subgroup_id, embedding_lower_bound, rho_value, status, provenance_id)
SELECT s.id, r.lower_bound, round(f.order_bits::numeric / nullif(s.order_bits, 0), 6), 'bounded', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(subgroup_id bigint, lower_bound numeric)
JOIN ec.subgroup s ON s.id = r.subgroup_id
JOIN ec.field f ON f.id = s.field_id
ON CONFLICT (subgroup_id) DO UPDATE SET embedding_lower_bound = EXCLUDED.embedding_lower_bound,
       rho_value = coalesce(pd.rho_value, EXCLUDED.rho_value), status = 'bounded',
       provenance_id = EXCLUDED.provenance_id
WHERE pd.embedding_degree IS NULL AND pd.status <> 'not_applicable'
  AND (pd.embedding_lower_bound IS NULL OR pd.embedding_lower_bound < EXCLUDED.embedding_lower_bound)";

#[derive(Clone)]
struct Job {
    subgroup_id: i64,
    r: BigUint,
    r_proved: bool,
    q: BigUint,
    has_row: bool,
    degree: Option<BigUint>,
    lower: Option<BigUint>,
    status: Option<String>,
    /// The held `complement_bits` and `target_field_bits`, and log2 q rounded up.
    complement_bits: Option<i32>,
    target_bits: Option<i32>,
    order_bits: Option<i32>,
}

fn run_jobs(jobs: &[Job]) -> Vec<Outcome> {
    let primes = primes_up_to(TRIAL_BOUND);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let threads = std::thread::available_parallelism().map_or(2, |n| n.get()).min(8);
    let results = std::sync::Mutex::new(Vec::with_capacity(jobs.len()));
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(job) = jobs.get(i) else { break };
                let out = embedding_degree(&job.q, &job.r, &primes, RHO_STEPS);
                if let Ok(mut r) = results.lock() {
                    r.push((i, out));
                }
            });
        }
    });
    let mut r = results.into_inner().unwrap_or_default();
    r.sort_by_key(|(i, _)| *i);
    r.into_iter().map(|(_, o)| o).collect()
}

/// The row to store for one job, after the comparison with what ecdb holds.
enum Row {
    Exact(Value),
    Bound(Value),
    None,
}

fn decide(job: &Job, out: Outcome, report: &mut Report) -> Row {
    let fail = |report: &mut Report, why: String| {
        report.count("embedding_degree_crosscheck_failures", 1);
        report.problem(format!("embedding degree of subgroup {}: {why}", job.subgroup_id));
        Row::None
    };
    let qm = &job.q % &job.r;
    match out {
        Outcome::Failed(why) => fail(report, why),
        Outcome::NotApplicable => {
            if let Some(old) = &job.degree {
                return fail(report, format!("r divides q, so there is no embedding degree, but ecdb holds k = {old}"));
            }
            report.count("embedding_degree_not_applicable", 1);
            Row::None
        }
        Outcome::Exact { k, primes_proved } => {
            if let Some(old) = &job.degree {
                if *old != k {
                    return fail(report, format!("ecdb holds k = {old}, ecdb computes {k}"));
                }
                report.count("embedding_degree_existing_agree", 1);
                return Row::None;
            }
            if let Some(b) = &job.lower {
                if k < *b {
                    return fail(report, format!("ecdb holds the lower bound {b}, ecdb computes k = {k}"));
                }
            }
            if job.status.as_deref() == Some("not_applicable") {
                return fail(
                    report,
                    format!("the producer marks it not_applicable, but r does not divide q (k = {k})"),
                );
            }
            let complement = (&job.r - 1u8) / &k;
            if let Some(old) = job.complement_bits {
                if i64::from(old) != complement.bits() as i64 {
                    return fail(
                        report,
                        format!("ecdb holds complement_bits {old}, ecdb computes {} (k = {k})", complement.bits()),
                    );
                }
            }
            if let Some(old) = job.target_bits {
                let want = job.order_bits.and_then(|b| u64::try_from(b).ok()).map(|b| &k * b);
                if want.as_ref() != u64::try_from(old).ok().map(BigUint::from).as_ref() {
                    return fail(
                        report,
                        format!("ecdb holds target_field_bits {old}, ecdb computes {want:?} (k = {k})"),
                    );
                }
            }
            let status = if job.r_proved && primes_proved { "proved" } else { "probable" };
            report.count(&format!("embedding_degree_exact_{status}"), 1);
            Row::Exact(json!({
                "subgroup_id": job.subgroup_id, "degree": k.to_string(),
                "complement_bits": complement.bits(), "status": status,
            }))
        }
        Outcome::Bounded { lower } => {
            if let Some(old) = &job.degree {
                if *old <= lower || !qm.modpow(old, &job.r).is_one() {
                    return fail(report, format!("ecdb holds k = {old}, ecdb proves k > {lower} and needs q^k = 1"));
                }
                report.count("embedding_degree_existing_consistent", 1);
                return Row::None;
            }
            if job.status.as_deref() == Some("not_applicable") {
                return fail(
                    report,
                    format!("the producer marks it not_applicable, but r does not divide q (k > {lower})"),
                );
            }
            report.count("embedding_degree_bounded", 1);
            if job.lower.as_ref().is_some_and(|b| *b >= lower) {
                return Row::None;
            }
            Row::Bound(json!({ "subgroup_id": job.subgroup_id, "lower_bound": lower.to_string() }))
        }
    }
}

/// Compute and store the embedding degree of every prime-order subgroup that lacks one.
pub async fn import(client: &Client, report: &mut Report, prov_exact: i64, prov_bound: i64) -> Result<()> {
    let rows = client.query(JOBS_SQL, &[]).await?;
    let parse = |s: Option<String>| s.and_then(|x| x.parse::<BigUint>().ok());
    let mut jobs = Vec::with_capacity(rows.len());
    for row in rows {
        let (Some(r), Some(q)) = (parse(row.get(1)), parse(row.get(3))) else {
            report.problem(format!("subgroup {}: unparsed r or q", row.get::<_, i64>(0)));
            continue;
        };
        let primality: String = row.get(2);
        jobs.push(Job {
            subgroup_id: row.get(0),
            r,
            r_proved: primality == "proved",
            q,
            has_row: row.get(4),
            degree: parse(row.get(5)),
            lower: parse(row.get(6)),
            status: row.get(7),
            complement_bits: row.get(8),
            target_bits: row.get(9),
            order_bits: row.get(10),
        });
    }
    let targets = jobs.iter().filter(|j| j.degree.is_none()).count();
    report.count("embedding_degree_targets", targets as u64);
    report.count("embedding_degree_existing", (jobs.len() - targets) as u64);
    if jobs.is_empty() {
        return Ok(());
    }
    let jobs = std::sync::Arc::new(jobs);
    let outcomes = {
        let jobs = jobs.clone();
        tokio::task::spawn_blocking(move || run_jobs(&jobs)).await?
    };
    let (mut exact, mut bound) = (vec![], vec![]);
    let (mut new_exact, mut new_bound) = (0u64, 0u64);
    for (job, out) in jobs.iter().zip(outcomes) {
        match decide(job, out, report) {
            Row::Exact(v) => {
                new_exact += u64::from(!job.has_row);
                exact.push(v);
            }
            Row::Bound(v) => {
                new_bound += u64::from(!job.has_row);
                bound.push(v);
            }
            Row::None => {}
        }
    }
    let n_exact = bulk(client, EXACT_SQL, &exact, prov_exact).await?;
    let n_bound = bulk(client, BOUND_SQL, &bound, prov_bound).await?;
    // every row `decide` passed must be written: a row the SQL guard refused
    // (a concurrent writer) is a problem, never a silent skip
    for (what, sent, written) in [("exact degrees", exact.len(), n_exact), ("lower bounds", bound.len(), n_bound)] {
        if sent as u64 != written {
            report.problem(format!("embedding degree: {sent} {what} decided, {written} written"));
        }
    }
    report.count("embedding_degree_inserted", new_exact.min(n_exact) + new_bound.min(n_bound));
    report.count("embedding_degree_updated", n_exact.saturating_sub(new_exact));
    report.count("embedding_lower_bound_raised", n_bound.saturating_sub(new_bound));
    report.count("embedding_degree_rows_written", n_exact + n_bound);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::ToPrimitive;

    fn b(s: &str) -> BigUint {
        s.parse().unwrap()
    }

    fn small_primes() -> Vec<u32> {
        primes_up_to(1000)
    }

    fn brute_order(q: u64, r: u64) -> Option<u64> {
        if q.is_multiple_of(r) {
            return None;
        }
        let mut acc = 1u64;
        for i in 1..=r {
            acc = acc * (q % r) % r;
            if acc == 1 {
                return Some(i);
            }
        }
        None
    }

    fn is_prime(n: u64) -> bool {
        n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| !n.is_multiple_of(d))
    }

    /// Brute-force agreement: every prime r < 400 and every q < 300.
    #[test]
    fn agrees_with_brute_force_on_tiny_fields() {
        let primes = small_primes();
        for r in (2u64..400).filter(|&r| is_prime(r)) {
            for q in 2u64..300 {
                let got = embedding_degree(&BigUint::from(q), &BigUint::from(r), &primes, RHO_STEPS);
                match brute_order(q, r) {
                    None => assert_eq!(got, Outcome::NotApplicable, "q={q} r={r}"),
                    Some(k) => {
                        assert_eq!(got, Outcome::Exact { k: BigUint::from(k), primes_proved: true }, "q={q} r={r}")
                    }
                }
            }
        }
    }

    /// Brute force through the importer's path for curves: q = p^n and r a
    /// prime dividing #E(F_q) for the supersingular y^2 = x^3 + x, p = 3 mod 4,
    /// whose order is p + 1 over F_p: k = 2 for every r | p + 1, r > 2.
    #[test]
    fn supersingular_curves_have_degree_two() {
        let primes = small_primes();
        for p in (7u64..2000).filter(|&p| is_prime(p) && p % 4 == 3) {
            let mut n = p + 1;
            let mut d = 2;
            while n > 1 {
                if n % d == 0 {
                    if d > 2 {
                        let got = embedding_degree(&BigUint::from(p), &BigUint::from(d), &primes, RHO_STEPS);
                        let want = if p % d == 1 { 1 } else { 2 };
                        assert_eq!(got, Outcome::Exact { k: BigUint::from(want as u64), primes_proved: true });
                    }
                    while n % d == 0 {
                        n /= d;
                    }
                }
                d += 1;
            }
        }
    }

    /// Literature: BN254 (alt_bn128) and BLS12-381 have embedding degree 12.
    #[test]
    fn pinned_pairing_curves() {
        let primes = primes_up_to(TRIAL_BOUND);
        let bn_p = b("21888242871839275222246405745257275088696311157297823662689037894645226208583");
        let bn_r = b("21888242871839275222246405745257275088548364400416034343698204186575808495617");
        match embedding_degree(&bn_p, &bn_r, &primes, RHO_STEPS) {
            Outcome::Exact { k, .. } => assert_eq!(k, BigUint::from(12u8)),
            o => panic!("BN254: {o:?}"),
        }
        let hex = |s: &str| BigUint::parse_bytes(s.as_bytes(), 16).unwrap();
        let bls_q =
            hex("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab");
        let bls_r = hex("73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001");
        match embedding_degree(&bls_q, &bls_r, &primes, RHO_STEPS) {
            Outcome::Exact { k, .. } => assert_eq!(k, BigUint::from(12u8)),
            o => panic!("BLS12-381: {o:?}"),
        }
    }

    /// secp256k1 (SEC 2 parameters): k = (n - 1)/6 (an independent sympy
    /// n_order run). n - 1 has 57- to 109-bit prime factors that rho does not
    /// reach within the budget, so ecdb proves a bound below that k.
    #[test]
    fn secp256k1_exact_and_large() {
        let primes = primes_up_to(TRIAL_BOUND);
        let p = b("115792089237316195423570985008687907853269984665640564039457584007908834671663");
        let n = b("115792089237316195423570985008687907852837564279074904382605163141518161494337");
        match embedding_degree(&p, &n, &primes, RHO_STEPS) {
            Outcome::Exact { k, .. } => assert_eq!(k, (&n - 1u8) / 6u8),
            Outcome::Bounded { lower } => {
                assert!(lower >= BigUint::from(TRIAL_BOUND));
                assert!(lower < (&n - 1u8) / 6u8);
            }
            o => panic!("secp256k1: {o:?}"),
        }
    }

    /// Data ecdb already holds (docs/curves/traits.json, status proved):
    /// icv1-f2m41-tm2308219-7f48b14a, q = 2^41, r = 549756390943, k = 6704346231;
    /// icv1-f2m37-tm534059-32aad96b, q = 2^37, r = 230603167, k = 3116259.
    #[test]
    fn pinned_against_traits_catalogue() {
        let primes = primes_up_to(TRIAL_BOUND);
        let q41 = BigUint::one() << 41;
        assert_eq!(
            embedding_degree(&q41, &b("549756390943"), &primes, RHO_STEPS),
            Outcome::Exact { k: b("6704346231"), primes_proved: true }
        );
        let q37 = BigUint::one() << 37;
        assert_eq!(
            embedding_degree(&q37, &b("230603167"), &primes, RHO_STEPS),
            Outcome::Exact { k: b("3116259"), primes_proved: true }
        );
    }

    /// The bounded branch: r - 1 = 2 * P * Q with P, Q primes above the trial
    /// bound and a rho budget of zero, so the cofactor P Q stays unsplit.
    #[test]
    fn bound_when_r_minus_1_does_not_factor() {
        // r = 2 * 1000003 * 1000121 + 1 is prime; trial division only to 1000.
        let r = b("2000248000727");
        assert_eq!(r, BigUint::from(2u8) * 1000003u32 * 1000121u32 + 1u8);
        assert!(prime_test(&r).0, "test prime");
        let primes = small_primes();
        let got = embedding_degree(&BigUint::from(3u8), &r, &primes, 0);
        let Outcome::Bounded { lower } = got.clone() else { panic!("{got:?}") };
        // the brute-force order exceeds the stated bound
        let full = embedding_degree(&BigUint::from(3u8), &r, &primes_up_to(1_100_000), RHO_STEPS);
        let Outcome::Exact { k, .. } = full else { panic!("{full:?}") };
        assert!(k > lower, "k = {k}, bound {lower}");
        assert!(lower >= BigUint::from(997u32));
    }

    /// When q^F = 1 the order is exact even though C did not factor.
    #[test]
    fn exact_through_unfactored_cofactor() {
        let r = b("2000248000727");
        let primes = small_primes();
        // q = g^C has order dividing F = 2, i.e. q = r - 1: k = 2
        let q = &r - 1u8;
        assert_eq!(embedding_degree(&q, &r, &primes, 0), Outcome::Exact { k: BigUint::from(2u8), primes_proved: true });
        assert_eq!(
            embedding_degree(&BigUint::one(), &r, &primes, 0),
            Outcome::Exact { k: BigUint::one(), primes_proved: true }
        );
    }

    #[test]
    fn pollard_brent_splits() {
        let n = BigUint::from(1000003u64) * BigUint::from(1000033u64);
        let g = pollard_brent(&n, 1, RHO_STEPS).unwrap();
        assert!(g == BigUint::from(1000003u64) || g == BigUint::from(1000033u64));
        let part = factor_partial(&(n.clone() * 8u8 * 1000003u64), &small_primes(), RHO_STEPS);
        assert!(part.cofactor.is_one());
        assert_eq!(
            part.factors.iter().map(|f| (f.p.to_u64().unwrap(), f.e)).collect::<Vec<_>>(),
            vec![(2, 3), (1000003, 2), (1000033, 1)]
        );
    }

    /// The cross-check rejects corrupted inputs: a non-minimal k, a wrong
    /// factor list, a k that does not satisfy q^k = 1, and a composite r.
    #[test]
    fn crosscheck_rejects_corruption() {
        let (q, r) = (BigUint::from(2u8), BigUint::from(13u8)); // ord_13(2) = 12
        let f = |p: u32, e: u32| Factor { p: BigUint::from(p), e, proved: true };
        assert!(verify_order(&q, &r, &BigUint::from(12u8), &[f(2, 2), f(3, 1)]).is_ok());
        // k = 24 is a multiple, not the order
        assert!(verify_order(&q, &r, &BigUint::from(24u8), &[f(2, 3), f(3, 1)]).is_err());
        // a factor list that omits a prime of k
        assert!(verify_order(&q, &r, &BigUint::from(12u8), &[f(2, 2)]).is_err());
        // q^k != 1
        assert!(verify_order(&q, &r, &BigUint::from(6u8), &[f(2, 1), f(3, 1)]).is_err());
        // the factorization-free scan catches a k whose listed primes look minimal
        assert!(verify_order(&BigUint::from(3u8), &r, &BigUint::from(6u8), &[f(2, 1), f(3, 1)]).is_err());
        // a corrupted r (composite): Fermat fails
        assert!(matches!(embedding_degree(&q, &BigUint::from(91u8), &small_primes(), RHO_STEPS), Outcome::Failed(_)));
    }

    /// The comparison with ecdb's rows: an existing degree that disagrees is a
    /// problem and nothing is written; an agreeing one is counted.
    #[test]
    fn decide_against_existing_rows() {
        let job = |degree: Option<u64>, lower: Option<u64>, status: Option<&str>| Job {
            subgroup_id: 1,
            r: BigUint::from(13u8),
            r_proved: true,
            q: BigUint::from(2u8),
            has_row: degree.is_some() || lower.is_some() || status.is_some(),
            degree: degree.map(BigUint::from),
            lower: lower.map(BigUint::from),
            status: status.map(str::to_string),
            complement_bits: None,
            target_bits: None,
            order_bits: Some(4),
        };
        let out = || Outcome::Exact { k: BigUint::from(12u8), primes_proved: true };
        let mut rep = Report::default();
        assert!(matches!(decide(&job(Some(12), None, Some("proved")), out(), &mut rep), Row::None));
        assert_eq!(rep.counts["embedding_degree_existing_agree"], 1);
        assert!(matches!(decide(&job(Some(6), None, Some("proved")), out(), &mut rep), Row::None));
        assert!(matches!(decide(&job(None, Some(20), Some("bounded")), out(), &mut rep), Row::None));
        assert_eq!(rep.counts["embedding_degree_crosscheck_failures"], 2);
        assert_eq!(rep.problems.len(), 2);
        match decide(&job(None, Some(10), Some("bounded")), out(), &mut rep) {
            Row::Exact(v) => assert_eq!((v["degree"].as_str(), v["status"].as_str()), (Some("12"), Some("proved"))),
            _ => panic!("expected an exact row"),
        }
        assert!(matches!(decide(&job(None, None, None), out(), &mut rep), Row::Exact(_)));
        assert_eq!(rep.counts["embedding_degree_crosscheck_failures"], 2);
    }

    /// Held values that disagree with the computation are refused, not
    /// overwritten: complement_bits and target_field_bits on a row without a
    /// degree, and a degree held for r | q.
    #[test]
    fn decide_refuses_conflicting_held_values() {
        // ord_13(2) = 12: complement (13 - 1)/12 = 1 has 1 bit; q = 2 has order_bits 1 here
        let base = Job {
            subgroup_id: 7,
            r: BigUint::from(13u8),
            r_proved: true,
            q: BigUint::from(2u8),
            has_row: true,
            degree: None,
            lower: None,
            status: Some("unknown".into()),
            complement_bits: None,
            target_bits: None,
            order_bits: Some(1),
        };
        let out = || Outcome::Exact { k: BigUint::from(12u8), primes_proved: true };
        let mut rep = Report::default();
        let agree = Job { complement_bits: Some(1), target_bits: Some(12), ..base.clone() };
        assert!(matches!(decide(&agree, out(), &mut rep), Row::Exact(_)));
        let bad_c = Job { complement_bits: Some(3), ..base.clone() };
        assert!(matches!(decide(&bad_c, out(), &mut rep), Row::None));
        let bad_t = Job { target_bits: Some(24), ..base.clone() };
        assert!(matches!(decide(&bad_t, out(), &mut rep), Row::None));
        let unknown_t = Job { target_bits: Some(12), order_bits: None, ..base.clone() };
        assert!(matches!(decide(&unknown_t, out(), &mut rep), Row::None));
        // r = 13 divides q = 26: no embedding degree, yet one is held
        let na = Job { q: BigUint::from(26u8), degree: Some(BigUint::from(12u8)), ..base.clone() };
        assert!(matches!(decide(&na, Outcome::NotApplicable, &mut rep), Row::None));
        let na_empty = Job { q: BigUint::from(26u8), ..base.clone() };
        assert!(matches!(decide(&na_empty, Outcome::NotApplicable, &mut rep), Row::None));
        assert_eq!(rep.counts["embedding_degree_crosscheck_failures"], 4);
        assert_eq!(rep.problems.len(), 4);
        assert_eq!(rep.counts["embedding_degree_not_applicable"], 1);
    }
}
