//! Boolean chain construction and bounded sparse closure with u128 monomials.
//! Field coefficients remain exact GF(2^n) words; matrix rows are GF(2).
use super::Gf2;
use serde::Serialize;
use std::collections::{BTreeMap, VecDeque};

type Poly = BTreeMap<u128, u64>;
fn add(mut a: Poly, b: Poly) -> Poly {
    for (mask, c) in b {
        let dst = a.entry(mask).or_default();
        *dst ^= c;
    }
    a.retain(|_, c| *c != 0);
    a
}
fn square(gf: &Gf2, a: &Poly) -> Poly {
    a.iter().map(|(&m, &c)| (m, gf.sqr(c))).collect()
}
fn mul(gf: &Gf2, a: &Poly, b: &Poly) -> Poly {
    let mut out = Poly::new();
    for (&ma, &ca) in a {
        for (&mb, &cb) in b {
            *out.entry(ma | mb).or_default() ^= gf.mul(ca, cb);
        }
    }
    out.retain(|_, c| *c != 0);
    out
}
fn s3(gf: &Gf2, a: &Poly, c: &Poly, d: &Poly) -> Poly {
    let ac = mul(gf, a, c);
    add(
        add(
            add(
                mul(gf, &square(gf, &add(a.clone(), c.clone())), &square(gf, d)),
                mul(gf, &ac, d),
            ),
            square(gf, &ac),
        ),
        Poly::from([(0, 1)]),
    )
}
pub fn chain(gf: &Gf2, bs: &[u64], xr: u64, m: usize) -> (Vec<Vec<u128>>, usize) {
    assert!(m >= 3);
    let nv = m * bs.len() + (m - 2) * gf.n as usize;
    assert!(
        nv <= 127,
        "u128 Boolean layout supports at most 127 variables"
    );
    let xs: Vec<Poly> = (0..m)
        .map(|i| {
            bs.iter()
                .enumerate()
                .map(|(j, &b)| (1u128 << (i * bs.len() + j), b))
                .collect()
        })
        .collect();
    let us: Vec<Poly> = (0..m - 2)
        .map(|i| {
            (0..gf.n)
                .map(|j| {
                    (
                        1u128 << (m * bs.len() + i * gf.n as usize + j as usize),
                        1u64 << j,
                    )
                })
                .collect()
        })
        .collect();
    let mut links = vec![s3(gf, &xs[0], &xs[1], &us[0])];
    for i in 0..m - 3 {
        links.push(s3(gf, &us[i], &xs[i + 2], &us[i + 1]));
    }
    links.push(s3(gf, &us[m - 3], &xs[m - 1], &Poly::from([(0, xr)])));
    let eqs = links
        .into_iter()
        .flat_map(|p| {
            (0..gf.n).map(move |j| {
                p.iter()
                    .filter(|(_, c)| **c >> j & 1 != 0)
                    .map(|(&m, _)| m)
                    .collect()
            })
        })
        .collect();
    (eqs, nv)
}

fn key(m: u128) -> (u32, u128) {
    (m.count_ones(), m)
}
fn normal(mut row: Vec<u128>) -> Vec<u128> {
    row.sort_unstable_by_key(|&m| std::cmp::Reverse(key(m)));
    let mut out = Vec::with_capacity(row.len());
    for m in row {
        if out.last() == Some(&m) {
            out.pop();
        } else {
            out.push(m);
        }
    }
    out
}
fn monomials(nv: usize, d: u32) -> Vec<u128> {
    fn rec(out: &mut Vec<u128>, n: usize, left: u32, start: usize, mask: u128) {
        out.push(mask);
        if left > 0 {
            for i in start..n {
                rec(out, n, left - 1, i + 1, mask | (1u128 << i));
            }
        }
    }
    let mut out = Vec::new();
    rec(&mut out, nv, d, 0, 0);
    out
}

#[derive(Default, Serialize)]
pub struct SparseOutcome {
    pub status: &'static str,
    pub rank: usize,
    pub products: u64,
    pub seeded_rows: u64,
    pub mask_comparisons: u64,
    pub max_weight: usize,
    pub peak_stored_entries: usize,
}
struct Sparse {
    pivots: BTreeMap<u128, Vec<u128>>,
    queue: VecDeque<Vec<u128>>,
    stored: usize,
    limit: usize,
    degree: u32,
    out: SparseOutcome,
}
impl Sparse {
    fn insert(&mut self, row: Vec<u128>) -> bool {
        let mut row = normal(row);
        while let Some(&lead) = row.first() {
            let Some(pivot) = self.pivots.get(&lead) else {
                break;
            };
            let mut next = Vec::with_capacity(row.len() + pivot.len());
            let (mut i, mut j) = (0, 0);
            while i < row.len() && j < pivot.len() {
                self.out.mask_comparisons += 1;
                match key(row[i]).cmp(&key(pivot[j])) {
                    std::cmp::Ordering::Greater => {
                        next.push(row[i]);
                        i += 1;
                    }
                    std::cmp::Ordering::Less => {
                        next.push(pivot[j]);
                        j += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        i += 1;
                        j += 1;
                    }
                }
            }
            next.extend_from_slice(&row[i..]);
            next.extend_from_slice(&pivot[j..]);
            row = next;
        }
        let Some(&lead) = row.first() else {
            return true;
        };
        self.out.max_weight = self.out.max_weight.max(row.len());
        if lead == 0 {
            self.out.status = "refuted";
            return false;
        }
        let fall = lead.count_ones() < self.degree;
        let required = row.len() * (1 + usize::from(fall));
        if self.stored.saturating_add(required) > self.limit {
            self.out.status = "sparse_entry_cap";
            return false;
        }
        self.stored += required;
        self.out.peak_stored_entries = self.out.peak_stored_entries.max(self.stored);
        if fall {
            self.queue.push_back(row.clone());
        }
        self.pivots.insert(lead, row);
        self.out.rank += 1;
        true
    }
}
pub fn closure(eqs: &[Vec<u128>], nv: usize, degree: u32, limit: usize) -> SparseOutcome {
    assert!(nv <= 127 && (2..=4).contains(&degree));
    let mut state = Sparse {
        pivots: BTreeMap::new(),
        queue: VecDeque::new(),
        stored: 0,
        limit,
        degree,
        out: SparseOutcome {
            status: "started",
            ..Default::default()
        },
    };
    let mut cache = BTreeMap::new();
    for p in eqs {
        if p.is_empty() {
            continue;
        }
        let d = p.iter().map(|m| m.count_ones()).max().unwrap();
        if d > degree {
            continue;
        }
        for &t in cache
            .entry(degree - d)
            .or_insert_with(|| monomials(nv, degree - d))
            .iter()
        {
            state.out.seeded_rows += 1;
            if !state.insert(p.iter().map(|&m| m | t).collect()) {
                return state.out;
            }
        }
    }
    while let Some(fall) = state.queue.pop_front() {
        state.stored -= fall.len();
        let room = degree - fall[0].count_ones();
        for &t in cache
            .entry(room)
            .or_insert_with(|| monomials(nv, room))
            .iter()
        {
            state.out.products += 1;
            if !state.insert(fall.iter().map(|&m| m | t).collect()) {
                return state.out;
            }
        }
    }
    state.out.status = "saturated_not_refuted";
    state.out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        build_decomposition_system, element, find_irreducible_sparse, FieldStructure, ReducedBasis,
    };
    #[test]
    fn wide_builder_matches_library_equations() {
        let n = 5;
        let irr = find_irreducible_sparse(n).unwrap();
        let gf = Gf2::new(&irr);
        let st = FieldStructure::new(n, &irr);
        for m in 3..=6 {
            for xr in 0..8 {
                let (mut eqs, nv) = chain(&gf, &[1, 2], xr, m);
                let base = [element(1, n), element(2, n)];
                let narrow =
                    build_decomposition_system(&base, &element(xr, n), &element(1, n), m, &st)
                        .unwrap();
                assert_eq!(nv, narrow.n_vars);
                let mut expected: Vec<Vec<u128>> = narrow
                    .equations
                    .iter()
                    .map(|p| p.terms.iter().map(|m| m.mask as u128).collect())
                    .collect();
                for p in &mut eqs {
                    p.sort_unstable();
                }
                for p in &mut expected {
                    p.sort_unstable();
                }
                assert_eq!(eqs, expected);
            }
        }
    }
    #[test]
    fn sparse_fixed_points_match_dense_engine() {
        let n = 5;
        let irr = find_irreducible_sparse(n).unwrap();
        let gf = Gf2::new(&irr);
        let st = FieldStructure::new(n, &irr);
        for m in [3, 4] {
            for xr in 0..8 {
                let (eqs, nv) = chain(&gf, &[1], xr, m);
                let sys = build_decomposition_system(
                    &[element(1, n)],
                    &element(xr, n),
                    &element(1, n),
                    m,
                    &st,
                )
                .unwrap();
                for d in [2, 3] {
                    let got = closure(&eqs, nv, d, 1_000_000);
                    let (expected, _) =
                        ReducedBasis::from_system_closed_full(&sys.equations, nv, d, 1_000_000)
                            .unwrap();
                    assert_eq!(got.status == "refuted", expected.is_refuted());
                    if !expected.is_refuted() {
                        assert_eq!(got.rank, expected.rank());
                    }
                }
            }
        }
    }
    #[test]
    fn masks_above_64_and_caps_are_checked() {
        let x = 1u128 << 99;
        let out = closure(&[vec![x], vec![x, 0]], 100, 2, 1_000_000);
        assert_eq!(out.status, "refuted");
        assert_eq!(closure(&[vec![x, 0]], 100, 2, 0).status, "sparse_entry_cap");
    }
}
