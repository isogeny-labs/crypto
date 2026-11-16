//! Courtois experiments using the maintained Isogeny Labs Rust arithmetic.
//! New code is the trace constraint/orchestration, not a second field or curve.
use crypto_lib::binary_ecc::curve;
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::ic_boundary::{invmod, IncrementalGauss};
use crypto_lib::cryptanalysis::koblitz_fast_arith::{
    artin_schreier_root, FastBinaryCurve, FastPoint,
};
use crypto_lib::cryptanalysis::pollard_rho::{pollard_rho_dlp, RhoOptions};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use crypto_lib::cryptanalysis::{gf2_elim, group_op_count};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub type Outcome<T> = Result<T, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fixture {
    pub n: u32,
    pub modulus: u64,
    pub a: u64,
    pub b: u64,
    pub order: u64,
    pub q: u64,
    pub h: u64,
    pub generator: (u64, u64),
}
pub fn fixtures() -> Vec<Fixture> {
    serde_json::from_str(include_str!("../fixtures.json")).expect("frozen public fixtures")
}

pub struct NativeCurve {
    pub spec: Fixture,
    pub fast: FastBinaryCurve,
    pub reference: BinaryCurve,
}
impl NativeCurve {
    pub fn new(spec: Fixture) -> Outcome<Self> {
        if spec.n < 3 || spec.n > 31 || spec.n % 2 == 0 || spec.b == 0 {
            return Err("study supports nonsingular odd degrees 3..31".into());
        }
        if !crypto_lib::cryptanalysis::koblitz_index_calculus::is_irreducible_f2(spec.modulus) {
            return Err("reducible field modulus".into());
        }
        let irr = IrreduciblePoly {
            degree: spec.n,
            low_terms: (0..spec.n).filter(|i| spec.modulus >> i & 1 == 1).collect(),
        };
        if spec.modulus >> spec.n != 1 {
            return Err("modulus width".into());
        }
        let e = |x: u64| F2mElement::from_biguint(&BigUint::from(x), spec.n);
        let reference = BinaryCurve {
            m: spec.n,
            irreducible: irr.clone(),
            a: e(spec.a),
            b: e(spec.b),
            generator: BinaryPoint::Affine {
                x: e(spec.generator.0),
                y: e(spec.generator.1),
            },
            order: BigUint::from(spec.q),
            cofactor: BigUint::from(spec.h),
        };
        let fast = FastBinaryCurve::new(&irr, spec.a).ok_or("native word field")?;
        let result = Self {
            spec,
            fast,
            reference,
        };
        if !result.reference.is_on_curve(&result.reference.generator)
            || result.mul(result.g(), result.spec.q).is_some()
        {
            return Err("generator validation".into());
        }
        Ok(result)
    }
    pub fn g(&self) -> FastPoint {
        Some(self.spec.generator)
    }
    pub fn mul(&self, p: FastPoint, k: u64) -> FastPoint {
        self.fast.scalar_mul(p, &BigUint::from(k))
    }
    pub fn add(&self, p: FastPoint, q: FastPoint) -> FastPoint {
        self.fast.add(p, q)
    }
    pub fn neg(&self, p: FastPoint) -> FastPoint {
        FastBinaryCurve::neg(p)
    }
    pub fn base(&self, m: u32) -> Vec<FastPoint> {
        (0..1u64 << m)
            .flat_map(|x| self.fast.points_with_x(self.spec.b, x))
            .collect()
    }
    pub fn reference_point(&self, p: FastPoint) -> BinaryPoint {
        p.map_or(BinaryPoint::Infinity, |(x, y)| BinaryPoint::Affine {
            x: self.fast.element(x),
            y: self.fast.element(y),
        })
    }
    pub fn check_sum(&self, points: &[FastPoint], target: FastPoint) -> bool {
        let actual = points.iter().fold(BinaryPoint::Infinity, |s, &p| {
            curve::point_add(&self.reference, &s, &self.reference_point(p))
        });
        actual == self.reference_point(target)
    }
    pub fn check_log(&self, value: u64, target: FastPoint) -> bool {
        curve::scalar_mul(
            &self.reference,
            &self.reference.generator,
            &BigUint::from(value),
        ) == self.reference_point(target)
    }
}

/// The partial trace in the photographed paper; no inverse is presumed.
pub fn tr1(f: &Gf2, x: u64) -> u64 {
    let (mut s, mut z) = (0, f.sqr(x));
    for _ in 0..(f.n - 1) / 2 {
        s ^= z;
        z = f.sqr(f.sqr(z));
    }
    s
}

#[derive(Clone)]
struct BinarySystem {
    rows: Vec<(u128, u64)>,
    pivots: Vec<usize>,
    kernel: Vec<u128>,
    word_xors: u64,
}
impl BinarySystem {
    fn new(columns: &[u64], n: usize) -> Self {
        let vars = columns.len();
        // Include the identity as augmented columns. RREF on [A|I] produces
        // the coefficient RREF and a complete, explicit RHS transformation.
        let width = (vars + n).div_ceil(64);
        let mut matrix = vec![vec![0; width]; n];
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, &column) in columns.iter().enumerate() {
                row[j / 64] |= ((column >> i) & 1) << (j % 64);
            }
            row[(vars + i) / 64] |= 1 << ((vars + i) % 64);
        }
        let mut word_xors = 0;
        gf2_elim::rref_counted(&mut matrix, vars + n, &mut word_xors);
        let mut rows = Vec::new();
        let mut pivots = Vec::new();
        for row in matrix {
            let mut coefficients = 0u128;
            let mut mask = 0u64;
            for j in 0..vars {
                coefficients |= (((row[j / 64] >> (j % 64)) & 1) as u128) << j;
            }
            for j in 0..n {
                mask |= ((row[(vars + j) / 64] >> ((vars + j) % 64)) & 1) << j;
            }
            if coefficients != 0 {
                pivots.push(coefficients.trailing_zeros() as usize);
            }
            rows.push((coefficients, mask));
        }
        let mut kernel = Vec::new();
        for free in (0..vars).filter(|j| !pivots.contains(j)) {
            let mut v = 1u128 << free;
            for (i, &p) in pivots.iter().enumerate() {
                v |= ((rows[i].0 >> free) & 1) << p;
            }
            kernel.push(v);
        }
        Self {
            rows,
            pivots,
            kernel,
            word_xors,
        }
    }
    fn solutions(&self, rhs: u64) -> Outcome<Vec<u128>> {
        if self.rows[self.pivots.len()..]
            .iter()
            .any(|r| (r.1 & rhs).count_ones() % 2 == 1)
        {
            return Ok(vec![]);
        }
        if self.kernel.len() > 16 {
            return Err(format!(
                "nullity {} exceeds frozen limit 16",
                self.kernel.len()
            ));
        }
        let mut current = 0u128;
        for (i, &p) in self.pivots.iter().enumerate() {
            current |= (((self.rows[i].1 & rhs).count_ones() & 1) as u128) << p;
        }
        let mut answer = vec![current];
        for i in 1usize..1 << self.kernel.len() {
            current ^= self.kernel[i.trailing_zeros() as usize];
            answer.push(current);
        }
        Ok(answer)
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct SearchWork {
    pub pair_calls: u64,
    pub prefix_trials: u64,
    pub linear_candidates: u64,
    pub systems_built: u64,
    pub cache_hits: u64,
    pub elimination_word_xors: u64,
    pub max_nullity: usize,
}
pub struct Splitter<'a> {
    pub curve: &'a NativeCurve,
    pub work: SearchWork,
    pub cached: bool,
    bases: BTreeMap<u32, Vec<FastPoint>>,
    systems: BTreeMap<(u64, u32, u64), (BinarySystem, u64, u32, u32)>,
    pub symmetry: bool,
}
fn polynomial_mul(mut a: u64, mut b: u64) -> u64 {
    let mut out = 0;
    while b != 0 {
        if b & 1 == 1 {
            out ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    out
}
fn polynomial_remainder(mut a: u64, b: u64) -> u64 {
    while a != 0 && a.leading_zeros() <= b.leading_zeros() {
        a ^= b << (b.leading_zeros() - a.leading_zeros());
    }
    a
}
impl<'a> Splitter<'a> {
    pub fn new(curve: &'a NativeCurve, cached: bool, symmetry: bool) -> Self {
        Self {
            curve,
            cached,
            symmetry,
            work: SearchWork::default(),
            bases: BTreeMap::new(),
            systems: BTreeMap::new(),
        }
    }
    fn base(&mut self, m: u32) -> Vec<FastPoint> {
        self.bases
            .entry(m)
            .or_insert_with(|| self.curve.base(m))
            .clone()
    }
    pub fn pairs(
        &mut self,
        target: FastPoint,
        m: u32,
        divisor: u64,
    ) -> Outcome<Vec<[FastPoint; 2]>> {
        self.work.pair_calls += 1;
        if self.work.pair_calls > 1_000_000 {
            return Err("pair-call budget".into());
        }
        if divisor == 0 || m == 0 || m > 16 || 2 * m - 2 >= self.curve.spec.n {
            return Err("polynomial-product bounds".into());
        }
        let mut found = BTreeSet::new();
        let c = self.curve;
        let f = &c.fast.gf;
        let s = target.map(|p| p.0).unwrap_or(0);
        if s == 0 {
            for p in self.base(m) {
                let q = c.add(target, c.neg(p));
                if let (Some((x, _)), Some((y, _))) = (p, q) {
                    if y < 1 << m
                        && polynomial_remainder(x, divisor) == 0
                        && polynomial_remainder(y, divisor) == 0
                    {
                        found.insert([p, q]);
                    }
                }
            }
            return Ok(found.into_iter().collect());
        }
        let key = (s, m, divisor);
        let (system, constant, udim, l) = if let Some(value) = self.systems.get(&key) {
            self.work.cache_hits += 1;
            value.clone()
        } else {
            let d = 63 - divisor.leading_zeros();
            if d >= m {
                return Err("divisor degree".into());
            }
            let l = m - d;
            let udim = 2 * l - 1;
            let m2 = polynomial_mul(divisor, divisor);
            let mut columns: Vec<u64> = (0..udim).map(|j| m2 << j).collect();
            columns.extend((0..l).map(|j| f.mul(s, tr1(f, f.sqr(divisor << j)))));
            let system = BinarySystem::new(&columns, c.spec.n as usize);
            self.work.systems_built += 1;
            self.work.elimination_word_xors += system.word_xors;
            self.work.max_nullity = self.work.max_nullity.max(system.kernel.len());
            let constant = f.mul(s, tr1(f, f.mul(c.spec.b, f.inv(f.sqr(s)))));
            let value = (system, constant, udim, l);
            if self.cached {
                if self.systems.len() == 256 {
                    self.systems.clear();
                }
                self.systems.insert(key, value.clone());
            }
            value
        };
        for bit in [0, s] {
            for solution in system.solutions(constant ^ bit)? {
                self.work.linear_candidates += 1;
                if self.work.linear_candidates > 10_000_000 {
                    return Err("candidate budget".into());
                }
                let u = (solution & ((1u128 << udim) - 1)) as u64;
                let v = (solution >> udim) as u64;
                let roots = if v == 0 {
                    vec![f.sqr_k(u, c.spec.n - 1)]
                } else {
                    let argument = f.mul(u, f.inv(f.sqr(v)));
                    match artin_schreier_root(f, c.spec.n, argument) {
                        Some(z) => {
                            let x = f.mul(v, z);
                            vec![x, x ^ v]
                        }
                        None => vec![],
                    }
                };
                for x in roots {
                    let y = x ^ v;
                    if x >= 1 << l || y >= 1 << l {
                        continue;
                    }
                    let full_x = polynomial_mul(divisor, x);
                    let full_y = polynomial_mul(divisor, y);
                    for p in c.fast.points_with_x(c.spec.b, full_x) {
                        let q = c.add(target, c.neg(p));
                        if q.is_some_and(|q| q.0 == full_y) {
                            found.insert([p, q]);
                        }
                    }
                }
            }
        }
        Ok(found.into_iter().collect())
    }
    pub fn split(&mut self, target: FastPoint, t: u32, m: u32) -> Outcome<Option<Vec<FastPoint>>> {
        if !(2..=5).contains(&t) || m == 0 || m > 16 {
            return Err("split search bounds".into());
        }
        self.search(target, t, m, 0)
    }
    fn search(
        &mut self,
        target: FastPoint,
        t: u32,
        m: u32,
        lower: usize,
    ) -> Outcome<Option<Vec<FastPoint>>> {
        let base = self.base(m);
        if t == 2 {
            for pair in self.pairs(target, m, 1)? {
                if !self.symmetry
                    || (base[lower..].contains(&pair[0]) && base[lower..].contains(&pair[1]))
                {
                    return Ok(Some(pair.to_vec()));
                }
            }
            return Ok(None);
        }
        for (j, &p) in base.iter().enumerate().skip(lower) {
            self.work.prefix_trials += 1;
            if self.work.prefix_trials > 1_000_000 {
                return Err("prefix budget".into());
            }
            let target = self.curve.add(target, self.curve.neg(p));
            if let Some(mut result) =
                self.search(target, t - 1, m, if self.symmetry { j } else { 0 })?
            {
                result.insert(0, p);
                return Ok(Some(result));
            }
        }
        Ok(None)
    }
    pub fn common_multiple(
        &mut self,
        target: FastPoint,
        max_k: u64,
    ) -> Outcome<Option<(u64, u64, [FastPoint; 2])>> {
        let n = self.curve.spec.n;
        let d = (n / 6).saturating_sub(1);
        let mut s = None;
        for k in 1..=max_k {
            s = self.curve.add(s, target);
            if s.is_none() {
                continue;
            }
            for divisor in 1 << d..1 << (d + 1) {
                if let Some(pair) = self.pairs(s, n.div_ceil(2), divisor)?.first() {
                    return Ok(Some((k, divisor, *pair)));
                }
            }
        }
        Ok(None)
    }
}

/// Standard reusable pair-table control; all construction is charged cold.
pub struct PairTable<'a> {
    curve: &'a NativeCurve,
    base: Vec<FastPoint>,
    pairs: BTreeMap<FastPoint, [FastPoint; 2]>,
    pub probes: u64,
}
impl<'a> PairTable<'a> {
    pub fn new(curve: &'a NativeCurve, m: u32) -> Self {
        let base = curve.base(m);
        let mut pairs = BTreeMap::new();
        for (i, &p) in base.iter().enumerate() {
            for &q in &base[i..] {
                pairs.entry(curve.add(p, q)).or_insert([p, q]);
            }
        }
        Self {
            curve,
            base,
            pairs,
            probes: 0,
        }
    }
    pub fn split(&mut self, target: FastPoint, t: u32) -> Outcome<Option<Vec<FastPoint>>> {
        match t {
            2 => Ok(self.pairs.get(&target).map(|p| p.to_vec())),
            4 => {
                for (&sum, &left) in &self.pairs {
                    self.probes += 1;
                    if let Some(right) =
                        self.pairs.get(&self.curve.add(target, self.curve.neg(sum)))
                    {
                        return Ok(Some([left.as_slice(), right.as_slice()].concat()));
                    }
                }
                Ok(None)
            }
            3 | 5 => {
                for p in self.base.clone() {
                    if let Some(mut r) =
                        self.split(self.curve.add(target, self.curve.neg(p)), t - 1)?
                    {
                        r.insert(0, p);
                        return Ok(Some(r));
                    }
                }
                Ok(None)
            }
            _ => Err("pair-table arity".into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SolveRecord {
    pub fixture: Fixture,
    pub target: FastPoint,
    pub recovered: u64,
    pub arity: u32,
    pub engine: String,
    pub columns: usize,
    pub attempts: u64,
    pub relations: u64,
    pub rank: usize,
    pub row_ops: u64,
    pub row_scans: u64,
    pub pivot_inversions: u64,
    pub decomposition: Vec<FastPoint>,
    pub individual_offset: u64,
    pub search_work: SearchWork,
    pub group_ops: group_op_count::OpCounts,
    pub claim_standing: String,
}
fn canonical(c: &NativeCurve, p: FastPoint) -> (FastPoint, i64) {
    let q = c.neg(p);
    if p <= q {
        (p, 1)
    } else {
        (q, -1)
    }
}
fn mod_signed(a: i64, q: u64) -> u64 {
    (a as i128).rem_euclid(q as i128) as u64
}
fn mm(a: u64, b: u64, q: u64) -> u64 {
    (a as u128 * b as u128 % q as u128) as u64
}

/// Native relation pipeline. The secret scalar is not an input.
pub fn index_calculus(
    c: &NativeCurve,
    target: FastPoint,
    t: u32,
    engine: &str,
    seed: u64,
) -> Outcome<SolveRecord> {
    if !(3..=5).contains(&t) || !["plain", "cached", "symmetry", "pairtable"].contains(&engine) {
        return Err("unknown native engine or arity".into());
    }
    let before = group_op_count::ops_snapshot();
    let m = c.spec.n.div_ceil(t);
    let q = c.spec.q;
    let h = c.spec.h;
    let hinv = invmod(h, q).ok_or("cofactor not invertible")?;
    let mut columns = Vec::new();
    let mut ids = BTreeMap::new();
    let mut sources = BTreeMap::new();
    for x in 0..1 << m {
        let Some(p) = c.fast.points_with_x(c.spec.b, x).into_iter().min() else {
            continue;
        };
        let (projected, sign) = canonical(c, c.mul(p, h));
        let id = if projected.is_none() {
            None
        } else {
            Some(*ids.entry(projected).or_insert_with(|| {
                columns.push(projected);
                columns.len() - 1
            }))
        };
        sources.insert(x, (p, id, sign));
    }
    let row_for = |points: &[FastPoint]| -> Outcome<Vec<u64>> {
        let mut row = vec![0; columns.len()];
        for &p in points {
            let x = p.ok_or("infinite base term")?.0;
            let &(source, id, sign) = sources.get(&x).ok_or("outside fixed base")?;
            if let Some(id) = id {
                row[id] = (row[id] + mod_signed(if p == source { sign } else { -sign }, q)) % q;
            }
        }
        Ok(row)
    };
    let mut trace = Splitter::new(c, engine != "plain", engine == "symmetry");
    let mut table = if engine == "pairtable" {
        Some(PairTable::new(c, m))
    } else {
        None
    };
    let mut split = |r| {
        if let Some(table) = &mut table {
            table.split(r, t)
        } else {
            trace.split(r, t, m)
        }
    };
    let mut matrix = IncrementalGauss::new(columns.len(), q);
    let mut rng = StdRng::seed_from_u64(seed);
    let mut scalars: Vec<u64> = (1..q).collect();
    scalars.shuffle(&mut rng);
    let (mut attempts, mut relations) = (0, 0);
    for a in scalars {
        attempts += 1;
        if let Some(points) = split(c.mul(c.g(), a))? {
            relations += 1;
            matrix.add_row(row_for(&points)?, mm(h, a, q));
            if matrix.inconsistent != 0 {
                return Err("inconsistent exact relation".into());
            }
            if matrix.rank() == columns.len() {
                break;
            }
        }
    }
    if matrix.rank() != columns.len() {
        return Err(format!(
            "rank deficient: {} of {}",
            matrix.rank(),
            columns.len()
        ));
    }
    let logs: Vec<u64> = (0..columns.len())
        .map(|j| matrix.pinned(j).expect("full rank"))
        .collect();
    for (&p, &d) in columns.iter().zip(&logs) {
        if !c.check_log(d, p) {
            return Err("base-log reference replay".into());
        }
    }
    for a in 0..q {
        let r = c.add(target, c.mul(c.g(), a));
        if let Some(points) = split(r)? {
            let row = row_for(&points)?;
            let value = row
                .iter()
                .zip(&logs)
                .fold(0, |s, (&x, &y)| (s + mm(x, y, q)) % q);
            let recovered = (mm(value, hinv, q) + q - a) % q;
            if !c.check_sum(&points, r) || !c.check_log(recovered, target) {
                return Err("slow native replay failed".into());
            }
            return Ok(SolveRecord {
                fixture: c.spec.clone(),
                target,
                recovered,
                arity: t,
                engine: engine.into(),
                columns: columns.len(),
                attempts,
                relations,
                rank: matrix.rank(),
                row_ops: matrix.row_ops,
                row_scans: matrix.entries_scanned,
                pivot_inversions: matrix.pivot_inversions,
                decomposition: points,
                individual_offset: a,
                search_work: trace.work.clone(),
                group_ops: group_op_count::ops_snapshot().since(&before),
                claim_standing: "pcc:unverified".into(),
            });
        }
    }
    Err("individual logarithm search exhausted".into())
}

pub fn native_rho(c: &NativeCurve, target: FastPoint, seed: u64) -> Outcome<u64> {
    let result = pollard_rho_dlp(
        &c.g(),
        &target,
        &BigUint::from(c.spec.q),
        |p, q| c.add(*p, *q),
        |p, q| p == q,
        |p| p.map_or(0, |(x, y)| ((x ^ (y.rotate_left(11))) % 3) as u8),
        |p, k| c.fast.scalar_mul(*p, k),
        &RhoOptions {
            max_iterations: 1_000_000,
            max_restarts: 64,
            seed: Some(seed),
        },
    )
    .map_err(str::to_string)?;
    let value = result.x.to_u64().ok_or("rho scalar width")?;
    if !c.check_log(value, target) {
        return Err("rho slow replay".into());
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn toy() -> NativeCurve {
        NativeCurve::new(fixtures()[0].clone()).unwrap()
    }
    #[test]
    fn native_binary_elimination_matches_exhaustion() {
        let columns = [5, 3, 7, 2, 1];
        let system = BinarySystem::new(&columns, 3);
        for rhs in 0..8 {
            let expected: BTreeSet<u128> = (0..32u128)
                .filter(|x| {
                    columns
                        .iter()
                        .enumerate()
                        .fold(0, |s, (j, c)| s ^ if x >> j & 1 == 1 { *c } else { 0 })
                        == rhs
                })
                .collect();
            assert_eq!(
                system
                    .solutions(rhs)
                    .unwrap()
                    .into_iter()
                    .collect::<BTreeSet<_>>(),
                expected
            );
        }
    }
    #[test]
    fn pairs_match_all_full_point_sums() {
        let c = toy();
        let mut s = Splitter::new(&c, true, false);
        for (m, divisor) in [(2, 1), (3, 1), (4, 3)] {
            let base: Vec<_> = c
                .base(m)
                .into_iter()
                .filter(|p| polynomial_remainder(p.unwrap().0, divisor) == 0)
                .collect();
            let mut expected: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
            for &p in &base {
                for &q in &base {
                    expected.entry(c.add(p, q)).or_default().insert([p, q]);
                }
            }
            for x in 0..128 {
                for target in c.fast.points_with_x(c.spec.b, x) {
                    let actual = s
                        .pairs(target, m, divisor)
                        .unwrap()
                        .into_iter()
                        .collect::<BTreeSet<_>>();
                    assert_eq!(actual, expected.get(&target).cloned().unwrap_or_default());
                }
            }
            assert_eq!(
                s.pairs(None, m, divisor)
                    .unwrap()
                    .into_iter()
                    .collect::<BTreeSet<_>>(),
                expected.get(&None).cloned().unwrap_or_default()
            );
        }
    }
    #[test]
    fn higher_arity_and_symmetry_match_exact_reachability() {
        let c = toy();
        for t in 3..=5 {
            let m = c.spec.n.div_ceil(t);
            let base = c.base(m);
            let mut reachable = BTreeSet::from([None]);
            let c_ref = &c;
            for _ in 0..t {
                reachable = reachable
                    .into_iter()
                    .flat_map(|p| base.iter().map(move |&q| c_ref.add(p, q)))
                    .collect();
            }
            let mut trace = Splitter::new(&c, true, false);
            let mut symmetric = Splitter::new(&c, true, true);
            let mut table = PairTable::new(&c, m);
            for x in 0..128 {
                for target in c.fast.points_with_x(c.spec.b, x) {
                    for result in [
                        trace.split(target, t, m).unwrap(),
                        symmetric.split(target, t, m).unwrap(),
                        table.split(target, t).unwrap(),
                    ] {
                        assert_eq!(result.is_some(), reachable.contains(&target));
                        if let Some(points) = result {
                            assert!(c.check_sum(&points, target));
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn partial_trace_kernel_rank_is_computed() {
        for n in [3, 5, 7, 11, 13, 17, 19, 23, 29, 31] {
            let irr = crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse(n)
                .unwrap();
            let f = Gf2::new(&irr);
            let columns: Vec<_> = (0..n).map(|j| tr1(&f, f.sqr(1 << j))).collect();
            let s = BinarySystem::new(&columns, n as usize);
            assert_eq!(s.pivots.len(), n as usize - usize::from(n % 4 == 1));
        }
    }
    #[test]
    fn all_native_workstreams_recover_without_secret_input() {
        for fixture in fixtures().into_iter().take(3) {
            let c = NativeCurve::new(fixture).unwrap();
            let secret = 137 % c.spec.q;
            let target = c.mul(c.g(), secret);
            let mut s = Splitter::new(&c, true, false);
            if let Some((k, _, pair)) = s
                .common_multiple(target, 1 << c.spec.n.div_ceil(6))
                .unwrap()
            {
                assert!(c.check_sum(&pair, c.mul(target, k)));
            }
            for t in 3..=5 {
                for engine in ["plain", "cached", "symmetry", "pairtable"] {
                    let r = index_calculus(&c, target, t, engine, 701).unwrap();
                    assert_eq!(r.recovered, secret);
                }
            }
            assert_eq!(native_rho(&c, target, 701).unwrap(), secret);
        }
    }
    #[test]
    fn seventeen_bit_native_complete_solver() {
        let c = NativeCurve::new(fixtures()[3].clone()).unwrap();
        let target = c.mul(c.g(), 32123);
        for engine in ["cached", "pairtable"] {
            assert_eq!(
                index_calculus(&c, target, 3, engine, 1901)
                    .unwrap()
                    .recovered,
                32123
            );
        }
        assert_eq!(native_rho(&c, target, 1901).unwrap(), 32123);
    }
    #[test]
    fn common_divisor_search_finds_a_constructed_target() {
        let c = NativeCurve::new(fixtures()[2].clone()).unwrap();
        let divisor = 2;
        let base: Vec<_> = c
            .base(c.spec.n.div_ceil(2))
            .into_iter()
            .filter(|p| polynomial_remainder(p.unwrap().0, divisor) == 0)
            .collect();
        let target = base
            .iter()
            .flat_map(|&p| base.iter().map(move |&q| (p, q)))
            .map(|(p, q)| c.add(p, q))
            .find(|p| p.is_some_and(|p| p.0 != 0))
            .unwrap();
        let mut search = Splitter::new(&c, true, false);
        let (k, _, pair) = search.common_multiple(target, 1).unwrap().unwrap();
        assert_eq!(k, 1);
        assert!(c.check_sum(&pair, target));
    }
    #[test]
    fn native_field_reference_and_export() {
        let vectors = field_vectors();
        assert_eq!(vectors.len(), 256);
        if let Ok(path) = std::env::var("COURTOIS_VECTOR_EXPORT") {
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(path)
                .unwrap();
            f.write_all(serde_json::to_string_pretty(&vectors).unwrap().as_bytes())
                .unwrap();
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldVector {
    pub n: u32,
    pub modulus: u64,
    pub x: u64,
    pub y: u64,
    pub product: u64,
    pub inverse: Option<u64>,
}
pub fn field_vectors() -> Vec<FieldVector> {
    use rand::Rng;
    let mut rng = StdRng::seed_from_u64(20261115);
    let mut rows = Vec::new();
    for fixture in fixtures() {
        let c = NativeCurve::new(fixture).unwrap();
        let f = &c.fast.gf;
        for _ in 0..64 {
            let x = rng.gen_range(0..1 << c.spec.n);
            let y = rng.gen_range(0..1 << c.spec.n);
            let product = f.mul(x, y);
            let inverse = if x == 0 { None } else { Some(f.inv(x)) };
            let a = c.fast.element(x);
            let b = c.fast.element(y);
            assert_eq!(
                a.mul(&b, &c.reference.irreducible).to_biguint().to_u64(),
                Some(product)
            );
            assert_eq!(
                a.flt_inverse(&c.reference.irreducible)
                    .map(|v| v.to_biguint().to_u64().unwrap()),
                inverse
            );
            rows.push(FieldVector {
                n: c.spec.n,
                modulus: c.spec.modulus,
                x,
                y,
                product,
                inverse,
            });
        }
    }
    rows
}
