//! Tiny prime-order cold-DLP correctness control, not the qualified G6 panel.
//! No target scalar or known factor-base labels enter the interface.
use super::{algebra, reference, Point, Report, Witness};
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Instant;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Request {
    pub schema: String,
    pub oracle: super::Request,
    pub generator: Point,
    pub order: u64,
    pub seed: u64,
    pub max_queries: usize,
}

#[derive(Clone, Copy, Default, Debug, Serialize)]
pub(super) struct ScalarOps {
    pub multiplications: u64,
    pub subtractions: u64,
    pub inverse_calls: u64,
}

struct Elimination {
    p: u64,
    vars: usize,
    rows: Vec<(usize, Vec<u64>)>,
    ops: ScalarOps,
}
impl Elimination {
    fn new(p: u64, vars: usize) -> Self {
        Self {
            p,
            vars,
            rows: Vec::new(),
            ops: ScalarOps::default(),
        }
    }
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.ops.multiplications += 1;
        (u128::from(a) * u128::from(b) % u128::from(self.p)) as u64
    }
    fn sub(&mut self, a: u64, b: u64) -> u64 {
        self.ops.subtractions += 1;
        (a + self.p - b) % self.p
    }
    fn inv(&mut self, mut a: u64) -> u64 {
        assert_ne!(a, 0);
        self.ops.inverse_calls += 1;
        let mut n = self.p - 2;
        let mut out = 1;
        while n != 0 {
            if n & 1 != 0 {
                out = self.mul(out, a);
            }
            n >>= 1;
            if n != 0 {
                a = self.mul(a, a);
            }
        }
        out
    }
    fn insert(&mut self, mut row: Vec<u64>) -> Result<bool, String> {
        if row.len() != self.vars + 1 || row.iter().any(|&x| x >= self.p) {
            return Err("noncanonical scalar row".into());
        }
        // Copy the small matrix so arithmetic can be counted without aliases.
        for (pivot, old) in self.rows.clone() {
            let coefficient = row[pivot];
            if coefficient == 0 {
                continue;
            }
            for k in 0..=self.vars {
                let term = self.mul(coefficient, old[k]);
                row[k] = self.sub(row[k], term);
            }
        }
        let Some(pivot) = (0..self.vars).find(|&k| row[k] != 0) else {
            return if row[self.vars] == 0 {
                Ok(false)
            } else {
                Err("inconsistent relation system".into())
            };
        };
        let inverse = self.inv(row[pivot]);
        for x in &mut row {
            *x = self.mul(*x, inverse);
        }
        let old = std::mem::take(&mut self.rows);
        for (column, mut prior) in old {
            let coefficient = prior[pivot];
            if coefficient != 0 {
                for k in 0..=self.vars {
                    let term = self.mul(coefficient, row[k]);
                    prior[k] = self.sub(prior[k], term);
                }
            }
            self.rows.push((column, prior));
        }
        self.rows.push((pivot, row));
        self.rows.sort_by_key(|r| r.0);
        Ok(true)
    }
    fn target(&self) -> Option<u64> {
        self.rows.iter().find_map(|(pivot, row)| {
            (*pivot == self.vars - 1 && row[..self.vars - 1].iter().all(|&x| x == 0))
                .then_some(row[self.vars])
        })
    }
}

#[derive(Debug, Serialize)]
struct Query {
    a: u64,
    b: u64,
    rhs: Option<Point>,
    oracle: Option<Report>,
    useful_rows: usize,
    rank_after: usize,
    elapsed_ns: u128,
}

#[derive(Debug, Serialize)]
pub(super) struct Output {
    schema: &'static str,
    request: Request,
    factor_base: Vec<Point>,
    queries: Vec<Query>,
    original_rows: Vec<Vec<u64>>,
    final_rref: Vec<(usize, Vec<u64>)>,
    setup_ns: u128,
    setup_helper_arithmetic: algebra::Arithmetic,
    setup_reference_counts: reference::Counts,
    query_generation_counts: GroupOps,
    scalar_elimination_ns: u128,
    scalar_ops: ScalarOps,
    scalar_verification_ns: u128,
    scalar_verification_counts: reference::Counts,
    cold_total_ns: u128,
    cold_unattributed_ns: u128,
    recovered: Option<u64>,
    scalar_verified: bool,
    status: &'static str,
    comparison_admissible: bool,
    scope: &'static str,
}

fn setup(r: &Request) -> Result<(Vec<Point>, reference::Counts), String> {
    r.oracle.validate()?;
    if r.schema != "pkm.toy-dlp-request/v1"
        || r.oracle.p > 257
        || r.max_queries == 0
        || r.max_queries > 10_000
        || !crypto_lib::cryptanalysis::residual_walk::is_prime_u64(r.order)
        || r.order >= 1 << 32
    {
        return Err(
            "toy control requires p<=257, prime full curve order and 1..10000 queries".into(),
        );
    }
    let mut c = reference::Curve::new(&r.oracle);
    if !c.on_curve(r.generator) {
        return Err("invalid generator".into());
    }
    let mut order = 1u64;
    for x in 0..r.oracle.p {
        for y in 0..r.oracle.p {
            if c.on_curve(Point { x, y }) {
                order += 1;
            }
        }
    }
    if order != r.order || c.mul(Some(r.generator), r.order).is_some() {
        return Err("independent exhaustive full-curve order certificate failed".into());
    }
    let mut base = Vec::new();
    for x in r.oracle.instance().base() {
        let p = r.oracle.p;
        let x2 = algebra::mulm(x, x, p);
        let x3 = algebra::mulm(x2, x, p);
        let ax = algebra::mulm(r.oracle.a, x, p);
        let rhs = algebra::addm(x3, ax, p);
        let rhs = algebra::addm(rhs, r.oracle.b, p);
        if let Some(y) = algebra::sqrtm(rhs, p) {
            base.push(Point {
                x,
                y: y.min((p - y) % p),
            });
        }
    }
    base.sort_unstable();
    Ok((base, c.counts))
}

fn relation(
    r: &Request,
    base: &[Point],
    a: u64,
    b: u64,
    witness: &Witness,
) -> Result<Vec<u64>, String> {
    let index: BTreeMap<_, _> = base
        .iter()
        .enumerate()
        .map(|(i, q)| (q.x, (i, q.y)))
        .collect();
    let mut row = vec![0; base.len() + 2];
    for q in witness {
        let &(i, y) = index.get(&q.x).ok_or("witness outside factor base")?;
        let coefficient = if q.y == y {
            1
        } else if q.y == (r.oracle.p - y) % r.oracle.p {
            r.order - 1
        } else {
            return Err("invalid witness ordinate".into());
        };
        row[i] = (row[i] + coefficient) % r.order;
    }
    // sum(e_j log B_j) - b*k = a mod n.
    row[base.len()] = (r.order - b) % r.order;
    row[base.len() + 1] = a;
    Ok(row)
}

fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

pub(super) fn run(r: &Request) -> Result<Output, String> {
    let total = Instant::now();
    let setup_start = Instant::now();
    let before = algebra::counts();
    let (base, setup_counts) = setup(r)?;
    let setup_arithmetic = algebra::counts().since(before);
    let setup_ns = setup_start.elapsed().as_nanos();
    let curve = PrimeCurve {
        p: r.oracle.p,
        a: r.oracle.a,
        b: r.oracle.b,
    };
    let mut generation = GroupOps::default();
    let mut matrix = Elimination::new(r.order, base.len() + 1);
    let mut queries = Vec::new();
    let mut rows = Vec::new();
    let mut seed = r.seed;
    let mut elimination_ns = 0;
    for _ in 0..r.max_queries {
        let start = Instant::now();
        let a = next(&mut seed) % r.order;
        let b = next(&mut seed) % r.order;
        let ap = curve.mul(
            &mut generation,
            PrimePoint::affine(r.generator.x, r.generator.y),
            a,
        );
        let bq = curve.mul(
            &mut generation,
            PrimePoint::affine(r.oracle.target.x, r.oracle.target.y),
            b,
        );
        let raw_rhs = curve.add(&mut generation, ap, bq);
        let rhs = if raw_rhs.infinity {
            None
        } else {
            Some(Point {
                x: raw_rhs.x,
                y: raw_rhs.y,
            })
        };
        let mut useful = 0;
        let mut report = None;
        if let Some(q) = rhs {
            let mut request = r.oracle.clone();
            request.target = q;
            let result = super::run(&request)?;
            if result.status == "verification_failed" {
                return Err("oracle witness verification failed".into());
            }
            for witness in &result.witnesses {
                let row = relation(r, &base, a, b, witness)?;
                let stage = Instant::now();
                let increase = matrix.insert(row.clone())?;
                elimination_ns += stage.elapsed().as_nanos();
                rows.push(row);
                useful += usize::from(increase);
            }
            report = Some(result);
        }
        // Identity RHS attempts are charged and retained, never used as a
        // generic shortcut to supply the target scalar to this PKM control.
        queries.push(Query {
            a,
            b,
            rhs,
            oracle: report,
            useful_rows: useful,
            rank_after: matrix.rows.len(),
            elapsed_ns: start.elapsed().as_nanos(),
        });
        if matrix.target().is_some() {
            break;
        }
    }
    let answer = matrix.target();
    let stage = Instant::now();
    let mut verifier = reference::Curve::new(&r.oracle);
    let verified =
        answer.is_some_and(|k| verifier.mul(Some(r.generator), k) == Some(r.oracle.target));
    let verification_ns = stage.elapsed().as_nanos();
    if answer.is_some() && !verified {
        return Err("final scalar equation failed independent affine verification".into());
    }
    let cold_total_ns = total.elapsed().as_nanos();
    let query_ns: u128 = queries.iter().map(|q| q.elapsed_ns).sum();
    let cold_unattributed_ns = cold_total_ns
        .checked_sub(setup_ns + query_ns + verification_ns)
        .expect("exclusive cold pipeline intervals fit total");
    Ok(Output {schema:"pkm.toy-dlp-output/v1",request:r.clone(),factor_base:base,queries,
        original_rows:rows,final_rref:matrix.rows,setup_ns,setup_helper_arithmetic:setup_arithmetic,
        setup_reference_counts:setup_counts,query_generation_counts:generation,
        scalar_elimination_ns:elimination_ns,scalar_ops:matrix.ops,scalar_verification_ns:verification_ns,
        scalar_verification_counts:verifier.counts,cold_total_ns,cold_unattributed_ns,
        recovered:answer,scalar_verified:verified,status:if verified {"verified"} else {"unknown_query_cap"},
        comparison_admissible:false,scope:"p<=257 whole-curve prime-order correctness control; all setup/failed queries/LA retained; no matched reference, complete GAE, L2, scaling or speed claim"})
}

#[derive(Serialize)]
pub(super) struct Certificate {
    schema: &'static str,
    oracle_records: usize,
    rows_checked: usize,
    scalar_verified: bool,
    rank: usize,
    scope: &'static str,
}

pub(super) fn replay(r: &Request, record: &serde_json::Value) -> Result<Certificate, String> {
    let (base, _) = setup(r)?;
    if record["schema"] != "pkm.toy-dlp-output/v1"
        || record["request"] != serde_json::to_value(r).map_err(|e| e.to_string())?
        || record["factor_base"] != serde_json::to_value(&base).map_err(|e| e.to_string())?
        || record["comparison_admissible"] != false
    {
        return Err("toy DLP binding/admission mismatch".into());
    }
    let mut curve = reference::Curve::new(&r.oracle);
    let mut matrix = Elimination::new(r.order, base.len() + 1);
    let mut original = Vec::new();
    let mut checked = 0;
    let mut seed = r.seed;
    let queries = record["queries"]
        .as_array()
        .ok_or("missing query records")?;
    if queries.len() > r.max_queries {
        return Err("query cap exceeded".into());
    }
    for query in queries {
        if matrix.target().is_some() {
            return Err("queries continue after scalar recovery".into());
        }
        let a = next(&mut seed) % r.order;
        let b = next(&mut seed) % r.order;
        if query["a"] != a || query["b"] != b {
            return Err("query stream mismatch".into());
        }
        let ap = curve.mul(Some(r.generator), a);
        let bq = curve.mul(Some(r.oracle.target), b);
        let q = curve.add(ap, bq);
        if query["rhs"] != serde_json::to_value(q).map_err(|e| e.to_string())? {
            return Err("query point mismatch".into());
        }
        let mut useful = 0;
        if let Some(q) = q {
            let mut request = r.oracle.clone();
            request.target = q;
            super::replay(&request, &query["oracle"])?;
            checked += 1;
            let witnesses: Vec<Witness> =
                serde_json::from_value(query["oracle"]["witnesses"].clone())
                    .map_err(|e| e.to_string())?;
            for witness in witnesses {
                let row = relation(r, &base, a, b, &witness)?;
                // Independently check each scalar row as an actual group equation.
                let mut lhs = None;
                for (coefficient, point) in row[..base.len()].iter().zip(&base) {
                    let term = curve.mul(Some(*point), *coefficient);
                    lhs = curve.add(lhs, term);
                }
                let qterm = curve.mul(Some(r.oracle.target), row[base.len()]);
                lhs = curve.add(lhs, qterm);
                if lhs != curve.mul(Some(r.generator), a) {
                    return Err("original relation does not hold on curve".into());
                }
                useful += usize::from(matrix.insert(row.clone())?);
                original.push(row);
            }
        } else if !query["oracle"].is_null() {
            return Err("identity attempt must not carry an oracle answer".into());
        }
        if query["rank_after"] != matrix.rows.len() || query["useful_rows"] != useful {
            return Err("rank/yield recomputation mismatch".into());
        }
    }
    if record["original_rows"] != serde_json::to_value(&original).map_err(|e| e.to_string())?
        || record["final_rref"] != serde_json::to_value(&matrix.rows).map_err(|e| e.to_string())?
    {
        return Err("retained relation matrix mismatch".into());
    }
    let answer: Option<u64> =
        serde_json::from_value(record["recovered"].clone()).map_err(|e| e.to_string())?;
    if answer != matrix.target() || answer.is_some_and(|k| k >= r.order) {
        return Err("recovery certificate mismatch".into());
    }
    if answer.is_none() && queries.len() != r.max_queries {
        return Err("truncated query stream is not a query-cap result".into());
    }
    let ns = |v: &serde_json::Value| {
        v.as_u64()
            .map(u128::from)
            .ok_or("invalid bounded cold nanosecond count")
    };
    let query_ns = queries
        .iter()
        .try_fold(0u128, |s, q| ns(&q["elapsed_ns"]).map(|n| s + n))?;
    let charged = ns(&record["setup_ns"])?
        + query_ns
        + ns(&record["scalar_verification_ns"])?
        + ns(&record["cold_unattributed_ns"])?;
    if charged != ns(&record["cold_total_ns"])? || ns(&record["scalar_elimination_ns"])? > query_ns
    {
        return Err("cold phase intervals do not reconcile".into());
    }
    let verified = answer.is_some_and(|k| curve.mul(Some(r.generator), k) == Some(r.oracle.target));
    if record["scalar_verified"] != verified
        || record["status"]
            != if verified {
                "verified"
            } else {
                "unknown_query_cap"
            }
    {
        return Err("scalar verification/status mismatch".into());
    }
    Ok(Certificate{schema:"pkm.toy-dlp-replay/v1",oracle_records:checked,rows_checked:original.len(),scalar_verified:verified,rank:matrix.rows.len(),scope:"independent affine row/scalar equations; rank recomputed with producer elimination; no independent cost admission"})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rank_deficiency_and_contradiction_are_not_answers() {
        let mut matrix = Elimination::new(19, 3);
        assert!(matrix.insert(vec![1, 1, 1, 5]).unwrap());
        assert_eq!(matrix.target(), None);
        assert!(!matrix.insert(vec![2, 2, 2, 10]).unwrap());
        assert!(matrix.insert(vec![2, 2, 2, 11]).is_err());
        assert!(matrix.insert(vec![0, 0, 1, 7]).unwrap());
        assert_eq!(matrix.target(), Some(7));
    }
    #[test]
    fn public_target_recovery_has_no_scalar_input_and_replays() {
        let mut oracle = crate::tests::fixture();
        oracle.target = Point { x: 6, y: 3 };
        let request = Request {
            schema: "pkm.toy-dlp-request/v1".into(),
            oracle,
            generator: Point { x: 5, y: 1 },
            order: 19,
            seed: 20261030,
            max_queries: 16,
        };
        let result = run(&request).unwrap();
        assert_eq!(result.recovered, Some(2));
        assert!(result.scalar_verified);
        assert!(!result.comparison_admissible);
        let record = serde_json::to_value(&result).unwrap();
        assert!(replay(&request, &record).unwrap().scalar_verified);
        let mut bad = record.clone();
        bad["recovered"] = serde_json::json!(3);
        assert!(replay(&request, &bad).is_err());
        let mut bad = record;
        bad["queries"][0]["a"] = serde_json::json!(19);
        assert!(replay(&request, &bad).is_err());
    }
    #[test]
    fn cap_and_full_group_order_are_checked() {
        let mut oracle = crate::tests::fixture();
        oracle.limits.solver_millis = 0;
        let mut request = Request {
            schema: "pkm.toy-dlp-request/v1".into(),
            oracle,
            generator: Point { x: 5, y: 1 },
            order: 19,
            seed: 1,
            max_queries: 1,
        };
        let result = run(&request).unwrap();
        assert_eq!(result.recovered, None);
        assert_eq!(result.status, "unknown_query_cap");
        replay(&request, &serde_json::to_value(&result).unwrap()).unwrap();
        request.order = 17;
        assert!(run(&request).is_err());
    }
}
