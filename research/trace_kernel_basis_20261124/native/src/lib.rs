//! Finite coefficient-support selection, not a polynomial solver.
//!
//! Boolean monomials are u32 masks and field elements use polynomial coordinates.
//! The exhaustive selector is deliberately limited to the small listed fields.
use std::collections::BTreeMap;

pub type Polynomial = BTreeMap<u32, u16>;

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub degree: u32,
    pub modulus: u16,
}

impl Field {
    pub fn new(degree: u32) -> Result<Self, String> {
        let modulus = match degree {
            3 => 0x0b,
            4 => 0x13,
            7 => 0x83,
            9 => 0x211,
            11 => 0x805,
            _ => return Err("supported development degrees: 3, 4, 7, 9, 11".into()),
        };
        Ok(Self { degree, modulus })
    }

    pub fn size(self) -> u16 {
        1 << self.degree
    }

    pub fn multiply(self, mut a: u16, mut b: u16) -> u16 {
        assert!(a < self.size() && b < self.size());
        let mut out = 0;
        while b != 0 {
            if b & 1 != 0 {
                out ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a & self.size() != 0 {
                a ^= self.modulus;
            }
        }
        out
    }

    pub fn square(self, a: u16) -> u16 {
        self.multiply(a, a)
    }

    pub fn power(self, mut a: u16, mut exponent: u16) -> u16 {
        let mut out = 1;
        while exponent != 0 {
            if exponent & 1 != 0 {
                out = self.multiply(out, a);
            }
            a = self.square(a);
            exponent >>= 1;
        }
        out
    }

    pub fn trace(self, mut a: u16) -> u16 {
        let mut out = 0;
        for _ in 0..self.degree {
            out ^= a;
            a = self.square(a);
        }
        assert!(out <= 1);
        out
    }
}

fn toggle(p: &mut Polynomial, mask: u32, value: u16) {
    let new = p.get(&mask).copied().unwrap_or(0) ^ value;
    if new == 0 {
        p.remove(&mask);
    } else {
        p.insert(mask, new);
    }
}

pub fn add(ps: &[Polynomial]) -> Polynomial {
    let mut out = Polynomial::new();
    for p in ps {
        for (&mask, &coefficient) in p {
            toggle(&mut out, mask, coefficient);
        }
    }
    out
}

pub fn multiply(f: Field, p: &Polynomial, q: &Polynomial) -> Polynomial {
    let mut out = Polynomial::new();
    for (&a, &x) in p {
        for (&b, &y) in q {
            toggle(&mut out, a | b, f.multiply(x, y));
        }
    }
    out
}

fn square(f: Field, p: &Polynomial) -> Polynomial {
    p.iter().map(|(&mask, &x)| (mask, f.square(x))).collect()
}

pub fn affine(constant: u16, terms: impl IntoIterator<Item = (u32, u16)>) -> Polynomial {
    let mut out = Polynomial::new();
    toggle(&mut out, 0, constant);
    for (variable, coefficient) in terms {
        assert!(variable.is_power_of_two());
        toggle(&mut out, variable, coefficient);
    }
    out
}

pub fn s3(f: Field, c: u16, x: &Polynomial, y: &Polynomial, z: &Polynomial) -> Polynomial {
    let xy = multiply(f, x, y);
    let xz = multiply(f, x, z);
    let yz = multiply(f, y, z);
    add(&[
        square(f, &add(&[xy.clone(), xz, yz])),
        multiply(f, &xy, z),
        affine(c, []),
    ])
}

/// Substitute one Boolean variable by a sum of Boolean monomials.
pub fn substitute(p: &Polynomial, variable: u32, expression: &[u32]) -> Polynomial {
    assert!(variable.is_power_of_two());
    let mut out = Polynomial::new();
    for (&mask, &value) in p {
        if mask & variable == 0 {
            toggle(&mut out, mask, value);
        } else {
            for &term in expression {
                toggle(&mut out, (mask ^ variable) | term, value);
            }
        }
    }
    out
}

pub fn support(p: &Polynomial) -> u64 {
    p.values().map(|c| u64::from(c.count_ones())).sum()
}

pub fn rank(vectors: &[u16]) -> usize {
    let mut pivots = [0u16; 16];
    let mut out = 0;
    for &value in vectors {
        let mut v = value;
        for bit in (0..16).rev() {
            if v & (1 << bit) == 0 {
                continue;
            }
            if pivots[bit] != 0 {
                v ^= pivots[bit];
            } else {
                pivots[bit] = v;
                out += 1;
                break;
            }
        }
    }
    out
}

pub fn span(vectors: &[u16]) -> Vec<u16> {
    let mut out = vec![0];
    for &v in vectors {
        let more: Vec<_> = out.iter().map(|x| x ^ v).collect();
        out.extend(more);
    }
    out.sort_unstable();
    out.dedup();
    out
}

#[derive(Debug, Clone)]
pub struct Chart {
    pub original: Vec<u16>,
    pub pivot: usize,
    pub kernel: Vec<u16>,
    pub offset: u16,
}

impl Chart {
    pub fn new(f: Field, original: Vec<u16>, tau: u16) -> Result<Self, String> {
        if tau > 1
            || original.len() != f.degree as usize
            || original.iter().any(|&v| v >= f.size())
            || rank(&original) != f.degree as usize
        {
            return Err("expected a full field basis and one trace bit".into());
        }
        let pivot = original
            .iter()
            .position(|&v| f.trace(v) == 1)
            .ok_or("trace functional unexpectedly zero")?;
        let b = original[pivot];
        let kernel = original
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != pivot)
            .map(|(_, &v)| v ^ if f.trace(v) == 1 { b } else { 0 })
            .collect();
        Ok(Self {
            original,
            pivot,
            kernel,
            offset: if tau == 1 { b } else { 0 },
        })
    }

    pub fn standard(f: Field, tau: u16) -> Self {
        Self::new(f, (0..f.degree).map(|i| 1 << i).collect(), tau).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Context {
    pub field: Field,
    pub c: u16,
    pub chart: Chart,
    pub leaf_basis: Vec<u16>,
    pub training: Vec<u16>,
}

impl Context {
    pub fn new(field: Field, c: u16, training: Vec<u16>) -> Result<Self, String> {
        if c == 0
            || c >= field.size()
            || training.is_empty()
            || training.iter().any(|&v| v >= field.size())
        {
            return Err("nonzero curve coefficient and nonempty in-field targets required".into());
        }
        let chart = Chart::standard(field, 1);
        let leaf_dimension = field.degree.div_ceil(3).saturating_sub(1) as usize;
        let leaf_basis = chart.kernel[..leaf_dimension].to_vec();
        Ok(Self {
            field,
            c,
            chart,
            leaf_basis,
            training,
        })
    }

    pub fn auxiliary_start(&self) -> u32 {
        3 * self.leaf_basis.len() as u32
    }

    pub fn leaves(&self) -> [Polynomial; 3] {
        std::array::from_fn(|block| {
            affine(
                self.chart.offset,
                self.leaf_basis
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| (1 << (block * self.leaf_basis.len() + i), v)),
            )
        })
    }

    pub fn chain_with_auxiliary(&self, u: &Polynomial, target: u16) -> [Polynomial; 2] {
        let [x, y, z] = self.leaves();
        [
            s3(self.field, self.c, &x, &y, u),
            s3(self.field, self.c, u, &z, &affine(target, [])),
        ]
    }

    pub fn chain(&self, basis: &[u16], target: u16) -> [Polynomial; 2] {
        assert!(self.auxiliary_start() + basis.len() as u32 <= 32);
        let u = affine(
            self.chart.offset,
            basis
                .iter()
                .enumerate()
                .map(|(i, &v)| (1 << (self.auxiliary_start() + i as u32), v)),
        );
        self.chain_with_auxiliary(&u, target)
    }

    /// Exact number of descended Boolean coefficient occurrences, not work.
    pub fn total_support(&self, basis: &[u16], targets: &[u16]) -> u64 {
        targets
            .iter()
            .flat_map(|&t| self.chain(basis, t))
            .map(|p| support(&p))
            .sum()
    }

    /// Weight of the column for ONE auxiliary coordinate, over training targets.
    pub fn column_weight(&self, v: u16) -> u64 {
        let bit = 1 << self.auxiliary_start();
        self.training
            .iter()
            .flat_map(|&t| self.chain(&[v], t))
            .flat_map(|p| p.into_iter())
            .filter(|(mask, _)| mask & bit != 0)
            .map(|(_, c)| u64::from(c.count_ones()))
            .sum()
    }

    pub fn optimize(&self) -> Selection {
        let mut weights: Vec<_> = (1..self.field.size())
            .filter(|&v| self.field.trace(v) == 0)
            .map(|v| (v, self.column_weight(v)))
            .collect();
        weights.sort_unstable_by_key(|&(v, w)| (w, v));
        let basis = greedy_basis(&weights, self.field.degree as usize - 1).unwrap();
        let objective = basis.iter().map(|&v| self.column_weight(v)).sum();
        Selection {
            basis,
            objective,
            candidates_scored: weights.len(),
        }
    }
}

#[derive(Debug)]
pub struct Selection {
    pub basis: Vec<u16>,
    pub objective: u64,
    pub candidates_scored: usize,
}

/// Minimum additive weight basis of the given vector candidate set.
/// Global kernel optimality requires that ALL nonzero kernel vectors occur.
pub fn greedy_basis(candidates: &[(u16, u64)], dimension: usize) -> Result<Vec<u16>, String> {
    if dimension == 0 {
        return Ok(vec![]);
    }
    let mut sorted = candidates.to_vec();
    sorted.sort_unstable_by_key(|&(v, w)| (w, v));
    let mut selected = Vec::new();
    for (v, _) in sorted {
        let mut next = selected.clone();
        next.push(v);
        if rank(&next) > selected.len() {
            selected.push(v);
            if selected.len() == dimension {
                return Ok(selected);
            }
        }
    }
    Err("candidate set does not span requested rank".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate(_f: Field, p: &Polynomial, assignment: u32) -> u16 {
        p.iter()
            .filter(|(&m, _)| assignment & m == m)
            .fold(0, |a, (_, &c)| a ^ c)
    }

    fn numeric_s3(f: Field, c: u16, x: u16, y: u16, z: u16) -> u16 {
        f.square(f.multiply(x, y) ^ f.multiply(x, z) ^ f.multiply(y, z))
            ^ f.multiply(f.multiply(x, y), z)
            ^ c
    }

    #[test]
    fn field_inverses_and_trace_are_consistent() {
        for n in [3, 4, 7] {
            let f = Field::new(n).unwrap();
            for x in 1..f.size() {
                assert_eq!(f.multiply(x, f.power(x, f.size() - 2)), 1);
                for y in 0..f.size() {
                    assert_eq!(f.trace(x ^ y), f.trace(x) ^ f.trace(y));
                }
            }
        }
        assert!(Field::new(131).is_err());
    }

    #[test]
    fn trace_charts_cover_exact_hyperplanes() {
        for n in [3, 4, 7] {
            let f = Field::new(n).unwrap();
            for tau in [0, 1] {
                let c = Chart::standard(f, tau);
                let mut actual: Vec<_> =
                    span(&c.kernel).into_iter().map(|v| v ^ c.offset).collect();
                actual.sort_unstable();
                let expected: Vec<_> = (0..f.size()).filter(|&v| f.trace(v) == tau).collect();
                assert_eq!(actual, expected);
                assert_eq!(rank(&c.kernel), n as usize - 1);
            }
        }
        let f = Field::new(4).unwrap();
        assert!(Chart::new(f, vec![1, 1, 2, 4], 1).is_err());
    }

    #[test]
    fn direct_chart_is_identical_to_trace_elimination() {
        for n in [4, 7] {
            let f = Field::new(n).unwrap();
            let ctx = Context::new(f, 3, vec![1, 2]).unwrap();
            for tau in [0, 1] {
                let chart = Chart::standard(f, tau);
                let start = ctx.auxiliary_start();
                let raw = affine(
                    0,
                    chart
                        .original
                        .iter()
                        .enumerate()
                        .map(|(i, &b)| (1 << (start + i as u32), b)),
                );
                let mut expr: Vec<_> = chart
                    .original
                    .iter()
                    .enumerate()
                    .filter(|(i, b)| *i != chart.pivot && f.trace(**b) == 1)
                    .map(|(i, _)| 1 << (start + i as u32))
                    .collect();
                if tau == 1 {
                    expr.push(0);
                }
                let adapted = affine(
                    chart.offset,
                    chart
                        .original
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != chart.pivot)
                        .map(|(i, &b)| {
                            (
                                1 << (start + i as u32),
                                b ^ if f.trace(b) == 1 {
                                    chart.original[chart.pivot]
                                } else {
                                    0
                                },
                            )
                        }),
                );
                for target in 0..16.min(f.size()) {
                    let raw_chain = ctx.chain_with_auxiliary(&raw, target);
                    let direct = ctx.chain_with_auxiliary(&adapted, target);
                    for (a, b) in raw_chain.iter().zip(&direct) {
                        assert_eq!(substitute(a, 1 << (start + chart.pivot as u32), &expr), *b);
                    }
                }
            }
        }
    }

    #[test]
    fn symbolic_chain_matches_independent_field_evaluation() {
        let f = Field::new(4).unwrap();
        let ctx = Context::new(f, 3, vec![1, 2]).unwrap();
        let basis = &ctx.chart.kernel;
        let u = affine(
            ctx.chart.offset,
            basis
                .iter()
                .enumerate()
                .map(|(i, &v)| (1 << (ctx.auxiliary_start() + i as u32), v)),
        );
        let [x, y, z] = ctx.leaves();
        for t in 0..f.size() {
            let polys = ctx.chain(basis, t);
            for assignment in 0..(1 << (ctx.auxiliary_start() + basis.len() as u32)) {
                let values: Vec<_> = [&x, &y, &z, &u]
                    .iter()
                    .map(|p| evaluate(f, p, assignment))
                    .collect();
                assert_eq!(
                    evaluate(f, &polys[0], assignment),
                    numeric_s3(f, ctx.c, values[0], values[1], values[3])
                );
                assert_eq!(
                    evaluate(f, &polys[1], assignment),
                    numeric_s3(f, ctx.c, values[3], values[2], t)
                );
            }
        }
    }

    #[test]
    fn support_is_exactly_additive_in_auxiliary_columns() {
        for n in [4, 7] {
            let ctx = Context::new(Field::new(n).unwrap(), 3, vec![1, 2, 3]).unwrap();
            let fixed = ctx.total_support(&[], &ctx.training);
            for basis in [&ctx.chart.kernel, &ctx.optimize().basis] {
                let predicted = fixed + basis.iter().map(|&v| ctx.column_weight(v)).sum::<u64>();
                assert_eq!(ctx.total_support(basis, &ctx.training), predicted);
                let polys = ctx.chain(basis, 3);
                let mask = ((1 << basis.len()) - 1) << ctx.auxiliary_start();
                assert!(polys
                    .iter()
                    .all(|p| p.keys().all(|m| (m & mask).count_ones() <= 1)));
            }
        }
    }

    #[test]
    fn greedy_matches_all_small_bases_under_many_weights() {
        let f = Field::new(4).unwrap();
        let vs: Vec<_> = (1..f.size()).filter(|&v| f.trace(v) == 0).collect();
        for seed in 0u64..32 {
            let candidates: Vec<_> = vs
                .iter()
                .map(|&v| (v, (u64::from(v) * 17 + seed * 13) % 23))
                .collect();
            let greedy = greedy_basis(&candidates, 3).unwrap();
            let weight = |basis: &[u16]| {
                basis
                    .iter()
                    .map(|v| candidates.iter().find(|(w, _)| w == v).unwrap().1)
                    .sum::<u64>()
            };
            let mut best = u64::MAX;
            for i in 0..vs.len() {
                for j in i + 1..vs.len() {
                    for k in j + 1..vs.len() {
                        let b = [vs[i], vs[j], vs[k]];
                        if span(&b).len() == 8 {
                            best = best.min(weight(&b));
                        }
                    }
                }
            }
            assert_eq!(weight(&greedy), best);
        }
        assert!(greedy_basis(&[(1, 0), (1, 2), (0, 0)], 2).is_err());
        assert_eq!(greedy_basis(&[], 0).unwrap(), Vec::<u16>::new());
    }

    #[test]
    fn selected_kernel_preserves_domain_and_rejects_wrong_trace() {
        let ctx = Context::new(Field::new(7).unwrap(), 3, vec![3, 5]).unwrap();
        let picked = ctx.optimize();
        assert_eq!(span(&picked.basis), span(&ctx.chart.kernel));
        assert_eq!(picked.candidates_scored, 63);
        assert!(picked.basis.iter().all(|&v| ctx.field.trace(v) == 0));
        let mut bad = picked.basis;
        bad[0] ^= ctx.chart.offset;
        assert!(bad.iter().any(|&v| ctx.field.trace(v) != 0));
        assert_ne!(span(&bad), span(&ctx.chart.kernel));
        assert!(Context::new(ctx.field, 0, vec![1]).is_err());
        assert!(Context::new(ctx.field, 3, vec![]).is_err());
    }
}
