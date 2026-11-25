//! Row-budget retention of partial relations: which partials to keep when only
//! `budget` of them fit, scored by the exact useful rank they leave behind.
//!
//! Port of experiment T3 of isogeny-labs/workspace#144
//! (`docs/notes/fp-ic-triage-20261125/` there), which compared these policies
//! on F_p* double-large-prime pools with a Python graph count. Here the score
//! is [`super::gain_dsu_rank`]'s left nullity of the kept rows over `F_ℓ`:
//! the number of independent combinations that cancel every large prime,
//! coefficient-aware (an odd unit-coefficient cycle is unbalanced and yields
//! nothing on its own; a singleton or an unbalanced cycle pins its component).
//!
//! The graph used by the scoring policies has one vertex per LP column and one
//! *virtual* vertex standing for "no second large prime", so every row of LP
//! arity 1 or 2 is one edge. Rows of LP arity 0 (complete relations) and rows
//! of arity above 2 are outside this module's scope: callers keep the former
//! unconditionally and [`retain`] refuses the latter.
//!
//! | policy | rule |
//! | --- | --- |
//! | [`RetentionPolicy::Random`] | uniform sample, seeded |
//! | [`RetentionPolicy::SmallestLp`] | smallest product of LP identities first (factoring / F_p* textbook rule; on curves an LP identity is a point label and its size carries no frequency information) |
//! | [`RetentionPolicy::Degree`] | largest sum of LP-column degrees over the full batch |
//! | [`RetentionPolicy::Spectral`] | largest sum of leading-eigenvector weights of the multigraph (power iteration on `A + I`) |
//! | [`RetentionPolicy::CoreThenRandom`] | rows inside the 2-core ([`super::core_numbers`]) first, seeded order, then the rest |
//! | [`RetentionPolicy::SinglesThenSpectral`] | rows with one LP first, then two-LP rows; spectral order within each (exploratory, added after `research/lp_retention_20261125`'s frozen run showed that `SmallestLp`'s product key already ranks every one-LP row first) |
//!
//! Ties are broken by row position, so every policy is deterministic for a
//! fixed batch and seed. Nothing here claims a speedup of a complete solve.

use super::super::large_prime_filter::PartialRelation;
use super::{adjacency, core_numbers, gain_dsu_rank, GainDsuSnapshot, LpGraphError, LpIndex};
use serde::{Deserialize, Serialize};

/// A retention rule. See the module table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RetentionPolicy {
    Random,
    SmallestLp,
    Degree,
    Spectral,
    CoreThenRandom,
    SinglesThenSpectral,
}

impl RetentionPolicy {
    /// The five policies of the frozen protocol in `research/lp_retention_20261125`.
    pub const FROZEN: [RetentionPolicy; 5] = [
        RetentionPolicy::Random,
        RetentionPolicy::SmallestLp,
        RetentionPolicy::Degree,
        RetentionPolicy::Spectral,
        RetentionPolicy::CoreThenRandom,
    ];

    pub const ALL: [RetentionPolicy; 6] = [
        RetentionPolicy::Random,
        RetentionPolicy::SmallestLp,
        RetentionPolicy::Degree,
        RetentionPolicy::Spectral,
        RetentionPolicy::CoreThenRandom,
        RetentionPolicy::SinglesThenSpectral,
    ];

    pub fn name(self) -> &'static str {
        match self {
            RetentionPolicy::Random => "random",
            RetentionPolicy::SmallestLp => "smallest_lp",
            RetentionPolicy::Degree => "degree",
            RetentionPolicy::Spectral => "spectral",
            RetentionPolicy::CoreThenRandom => "core_then_random",
            RetentionPolicy::SinglesThenSpectral => "singles_then_spectral",
        }
    }
}

/// Errors specific to retention, on top of [`LpGraphError`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetentionError {
    Graph(LpGraphError),
    /// A row has no large prime (a complete relation) or more than two.
    UnsupportedArity { row: usize, arity: usize },
}

impl From<LpGraphError> for RetentionError {
    fn from(e: LpGraphError) -> Self {
        RetentionError::Graph(e)
    }
}

/// Seeded SplitMix64 (same constants as `eval`'s private generator).
#[derive(Clone, Debug)]
pub struct SplitMix(pub u64);

impl SplitMix {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Fisher–Yates shuffle.
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = (self.next_u64() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}

/// The batch as a multigraph: vertex `index.num_columns()` is the virtual
/// "no second large prime" vertex; edge `i` is row `i`.
struct RowGraph {
    /// `(u, v, row)` for every row.
    edges: Vec<(usize, usize, usize)>,
    vertices: usize,
}

impl RowGraph {
    fn build(relations: &[PartialRelation], modulus: u64) -> Result<Self, RetentionError> {
        let index = LpIndex::build(relations, modulus)?;
        let virt = index.num_columns();
        let mut edges = Vec::with_capacity(index.num_rows());
        for (i, row) in index.rows.iter().enumerate() {
            match row.as_slice() {
                [(u, _)] => edges.push((*u, virt, i)),
                [(u, _), (v, _)] => edges.push((*u, *v, i)),
                other => {
                    return Err(RetentionError::UnsupportedArity {
                        row: i,
                        arity: other.len(),
                    })
                }
            }
        }
        Ok(Self {
            edges,
            vertices: virt + 1,
        })
    }

    fn virt(&self) -> usize {
        self.vertices - 1
    }

    /// Endpoints of row `i` that are real LP columns.
    fn lp_ends(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        let (u, v, _) = self.edges[i];
        let virt = self.virt();
        [u, v].into_iter().filter(move |&x| x != virt)
    }

    fn degrees(&self) -> Vec<usize> {
        let mut d = vec![0usize; self.vertices];
        for &(u, v, _) in &self.edges {
            d[u] += 1;
            d[v] += 1;
        }
        d
    }

    /// Leading eigenvector of `A + I` by power iteration (`A` the multigraph
    /// adjacency, parallel edges counted). `A + I` has the same eigenvectors
    /// as `A` and a positive spectrum shift, so the iteration does not
    /// oscillate on bipartite components. Returns weights normalised to max 1.
    fn eigenvector(&self, iterations: usize) -> Vec<f64> {
        let n = self.vertices;
        let mut x = vec![1.0f64; n];
        for _ in 0..iterations {
            let mut y = x.clone();
            for &(u, v, _) in &self.edges {
                y[u] += x[v];
                y[v] += x[u];
            }
            let m = y.iter().cloned().fold(0.0f64, f64::max);
            if m == 0.0 {
                break;
            }
            for t in y.iter_mut() {
                *t /= m;
            }
            x = y;
        }
        x
    }
}

/// Split a batch by *canonical* LP arity (after [`LpIndex::build`] merges
/// repeated LP keys and drops zero coefficients mod `ℓ`): returns the
/// retainable partials (arity 1 or 2), the complete relations (arity 0, which
/// includes rows whose LP coefficients cancel), and the count of rows with
/// arity above 2. Use this before [`retain`]; splitting on the raw
/// `large.len()` misclassifies cancelling rows.
pub fn split_by_arity(
    relations: &[PartialRelation],
    modulus: u64,
) -> Result<(Vec<PartialRelation>, Vec<PartialRelation>, usize), RetentionError> {
    let index = LpIndex::build(relations, modulus)?;
    let (mut partials, mut complete, mut wide) = (Vec::new(), Vec::new(), 0usize);
    for (r, row) in relations.iter().zip(&index.rows) {
        match row.len() {
            0 => complete.push(r.clone()),
            1 | 2 => partials.push(r.clone()),
            _ => wide += 1,
        }
    }
    Ok((partials, complete, wide))
}

/// Positions of the rows kept by `policy` under `budget` (sorted ascending).
/// `budget >= relations.len()` keeps everything.
pub fn retain(
    relations: &[PartialRelation],
    modulus: u64,
    budget: usize,
    policy: RetentionPolicy,
    seed: u64,
) -> Result<Vec<usize>, RetentionError> {
    let g = RowGraph::build(relations, modulus)?;
    let n = relations.len();
    let budget = budget.min(n);
    let mut order: Vec<usize> = (0..n).collect();
    match policy {
        RetentionPolicy::Random => SplitMix(seed).shuffle(&mut order),
        RetentionPolicy::SmallestLp => {
            let key = |i: usize| -> u128 {
                relations[i]
                    .large
                    .iter()
                    .fold(1u128, |acc, &(q, _)| acc.saturating_mul(q as u128))
            };
            order.sort_by_key(|&i| (key(i), i));
        }
        RetentionPolicy::Degree => {
            let d = g.degrees();
            let score: Vec<usize> = (0..n).map(|i| g.lp_ends(i).map(|v| d[v]).sum()).collect();
            order.sort_by_key(|&i| (std::cmp::Reverse(score[i]), i));
        }
        RetentionPolicy::Spectral => {
            let w = g.eigenvector(200);
            let score: Vec<f64> = (0..n).map(|i| g.lp_ends(i).map(|v| w[v]).sum()).collect();
            order.sort_by(|&a, &b| score[b].total_cmp(&score[a]).then(a.cmp(&b)));
        }
        RetentionPolicy::SinglesThenSpectral => {
            let w = g.eigenvector(200);
            let score: Vec<f64> = (0..n).map(|i| g.lp_ends(i).map(|v| w[v]).sum()).collect();
            let arity = |i: usize| g.lp_ends(i).count();
            order.sort_by(|&a, &b| {
                arity(a).cmp(&arity(b)).then(score[b].total_cmp(&score[a])).then(a.cmp(&b))
            });
        }
        RetentionPolicy::CoreThenRandom => {
            let core = core_numbers(&adjacency(g.vertices, &g.edges));
            let in_core = |i: usize| {
                let (u, v, _) = g.edges[i];
                core[u] >= 2 && core[v] >= 2
            };
            let (mut inside, mut outside): (Vec<usize>, Vec<usize>) =
                (0..n).partition(|&i| in_core(i));
            let mut rng = SplitMix(seed);
            rng.shuffle(&mut inside);
            rng.shuffle(&mut outside);
            order = inside.into_iter().chain(outside).collect();
        }
    }
    let mut kept: Vec<usize> = order.into_iter().take(budget).collect();
    kept.sort_unstable();
    Ok(kept)
}

/// Exact LP-block rank of a subset of rows, by [`gain_dsu_rank`].
pub fn useful_rank(
    relations: &[PartialRelation],
    modulus: u64,
    kept: &[usize],
) -> Result<GainDsuSnapshot, RetentionError> {
    let subset: Vec<PartialRelation> = kept.iter().map(|&i| relations[i].clone()).collect();
    Ok(gain_dsu_rank(&LpIndex::build(&subset, modulus)?)?)
}

/// Topological count `b1 = E − V + C` of the kept rows' multigraph, virtual
/// vertex included, counting only vertices that carry an edge. It ignores
/// coefficients, so it is an upper bound on [`useful_rank`]'s left nullity;
/// kept for comparison with workspace#144, which reported it.
pub fn cycle_count(
    relations: &[PartialRelation],
    modulus: u64,
    kept: &[usize],
) -> Result<usize, RetentionError> {
    let subset: Vec<PartialRelation> = kept.iter().map(|&i| relations[i].clone()).collect();
    let g = RowGraph::build(&subset, modulus)?;
    let mut parent: Vec<usize> = (0..g.vertices).collect();
    fn find(p: &mut [usize], mut x: usize) -> usize {
        while p[x] != x {
            p[x] = p[p[x]];
            x = p[x];
        }
        x
    }
    let mut touched = vec![false; g.vertices];
    let mut comps = 0usize;
    for &(u, v, _) in &g.edges {
        for w in [u, v] {
            if !touched[w] {
                touched[w] = true;
                comps += 1;
            }
        }
        let (a, b) = (find(&mut parent, u), find(&mut parent, v));
        if a != b {
            parent[a] = b;
            comps -= 1;
        }
    }
    let v = touched.iter().filter(|&&t| t).count();
    Ok(g.edges.len() + comps - v)
}

/// One policy's outcome on one batch.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionOutcome {
    pub policy: RetentionPolicy,
    pub budget: usize,
    pub kept: usize,
    pub rank: GainDsuSnapshot,
    pub cycle_count: usize,
}

/// Run every policy in `policies` on the same batch, budget and seed.
pub fn compare_policies(
    relations: &[PartialRelation],
    modulus: u64,
    budget: usize,
    policies: &[RetentionPolicy],
    seed: u64,
) -> Result<Vec<RetentionOutcome>, RetentionError> {
    policies
        .iter()
        .map(|&policy| {
            let kept = retain(relations, modulus, budget, policy, seed)?;
            Ok(RetentionOutcome {
                policy,
                budget,
                kept: kept.len(),
                rank: useful_rank(relations, modulus, &kept)?,
                cycle_count: cycle_count(relations, modulus, &kept)?,
            })
        })
        .collect()
}

/// Toy F_p* double-large-prime collector (32-bit `p`), for exercising
/// retention on a stream whose LP identities are real primes. Not a DLP
/// solver: it emits `PartialRelation`s with logs taken modulo `ℓ`, the largest
/// prime factor of `p − 1`, and replays every relation it emits.
pub mod fp_toy {
    use super::SplitMix;
    use super::super::super::large_prime_filter::PartialRelation;
    use serde::{Deserialize, Serialize};

    fn pow_mod(mut b: u64, mut e: u64, p: u64) -> u64 {
        let mut r = 1u64;
        b %= p;
        while e > 0 {
            if e & 1 == 1 {
                r = r * b % p;
            }
            b = b * b % p;
            e >>= 1;
        }
        r
    }

    pub fn primes_up_to(n: u64) -> Vec<u64> {
        let n = n as usize;
        let mut sieve = vec![true; n + 1];
        let mut out = Vec::new();
        for i in 2..=n {
            if sieve[i] {
                out.push(i as u64);
                let mut j = i * i;
                while j <= n {
                    sieve[j] = false;
                    j += i;
                }
            }
        }
        out
    }

    /// Trial-division primality for `n < 2^32`.
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let mut d = 2u64;
        while d * d <= n {
            if n % d == 0 {
                return false;
            }
            d += if d == 2 { 1 } else { 2 };
        }
        true
    }

    fn factor(mut n: u64) -> Vec<u64> {
        let mut f = Vec::new();
        let mut d = 2u64;
        while d * d <= n {
            while n % d == 0 {
                f.push(d);
                n /= d;
            }
            d += 1;
        }
        if n > 1 {
            f.push(n);
        }
        f
    }

    /// Instance: prime `p < 2^32`, its least primitive root at or above
    /// `g_min`, and `ℓ`, the largest prime factor of `p − 1`.
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Instance {
        pub p: u64,
        pub g: u64,
        pub ell: u64,
    }

    impl Instance {
        pub fn new(p: u64, g_min: u64) -> Self {
            assert!(p < (1 << 32) && is_prime(p), "p must be a prime below 2^32");
            let mut qs = factor(p - 1);
            qs.dedup();
            let ell = *qs.last().expect("p > 2");
            let g = (g_min..p)
                .find(|&g| qs.iter().all(|&q| pow_mod(g, (p - 1) / q, p) != 1))
                .expect("primitive root");
            Self { p, g, ell }
        }

        /// The first prime at or above `start` (workspace#144 used
        /// `2^31 + 11·10^6·i` for `i = 0, 1, 2`).
        pub fn next_prime(start: u64) -> u64 {
            (start..).find(|&n| is_prime(n)).expect("prime")
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Collection {
        pub instance: Instance,
        pub smooth_bound: u64,
        pub lp_bound: u64,
        pub draws: usize,
        pub seed: u64,
        /// Complete relations seen (arity 0); not part of the partial pool.
        pub complete: usize,
        /// Partials with one or two large primes below `lp_bound`.
        pub partials: Vec<PartialRelation>,
        /// The exponent `k` of each partial, for replay.
        pub exponents: Vec<u64>,
    }

    /// Draw `draws` uniform exponents `k`, factor `g^k mod p` over primes
    /// `≤ smooth_bound`, and keep cofactors that are one prime `< lp_bound` or a
    /// product of two such primes. Small-prime exponents and `rhs = k` are
    /// reduced mod `ℓ`; LP coefficients are their multiplicities (1, or 2 for a
    /// squared large prime, which `LpIndex::build` merges).
    pub fn collect(
        instance: &Instance,
        smooth_bound: u64,
        lp_bound: u64,
        draws: usize,
        seed: u64,
    ) -> Collection {
        let Instance { p, g, ell } = *instance;
        let fb = primes_up_to(smooth_bound);
        let mid = primes_up_to(lp_bound);
        let mut rng = SplitMix(seed);
        let (mut partials, mut exponents, mut complete) = (Vec::new(), Vec::new(), 0usize);
        for _ in 0..draws {
            let k = 1 + rng.next_u64() % (p - 2);
            let mut c = pow_mod(g, k, p);
            let mut small = Vec::new();
            for (j, &q) in fb.iter().enumerate() {
                let mut e = 0u64;
                while c % q == 0 {
                    c /= q;
                    e += 1;
                }
                if e > 0 {
                    small.push((j as u32, e % ell));
                }
            }
            let large: Vec<(u64, u64)> = if c == 1 {
                complete += 1;
                continue;
            } else if c < lp_bound && is_prime(c) {
                vec![(c, 1)]
            } else if !is_prime(c) {
                match mid
                    .iter()
                    .skip_while(|&&q| q <= smooth_bound)
                    .take_while(|&&q| q * q <= c)
                    .find(|&&q| c % q == 0)
                {
                    Some(&a) if c / a < lp_bound && is_prime(c / a) => vec![(a, 1), (c / a, 1)],
                    _ => continue,
                }
            } else {
                continue;
            };
            partials.push(PartialRelation {
                id: partials.len() as u64,
                small,
                large,
                rhs: k % ell,
            });
            exponents.push(k);
        }
        Collection {
            instance: instance.clone(),
            smooth_bound,
            lp_bound,
            draws,
            seed,
            complete,
            partials,
            exponents,
        }
    }

    /// Replay: `g^k ≡ ∏ q_j^{e_j} · ∏ L` (mod p) for every partial, with the
    /// small exponents recovered from the factorisation (they are stored mod
    /// `ℓ`, which exceeds any exponent at these sizes). Returns the number checked.
    pub fn replay(col: &Collection) -> Result<usize, usize> {
        let Instance { p, g, .. } = col.instance;
        let fb = primes_up_to(col.smooth_bound);
        for (i, (rel, &k)) in col.partials.iter().zip(&col.exponents).enumerate() {
            let mut prod = 1u64;
            for &(j, e) in &rel.small {
                prod = prod * pow_mod(fb[j as usize], e, p) % p;
            }
            for &(q, m) in &rel.large {
                if !(q > col.smooth_bound && q < col.lp_bound && is_prime(q)) {
                    return Err(i);
                }
                prod = prod * pow_mod(q, m, p) % p;
            }
            if prod != pow_mod(g, k, p) {
                return Err(i);
            }
        }
        Ok(col.partials.len())
    }
}
