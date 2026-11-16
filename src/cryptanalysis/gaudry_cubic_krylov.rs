//! Sparse FGLM in shape position (Faugere--Mou, arXiv:1304.1238, §3.1).
//! Vertices are standard monomials; weighted edges are multiplication by e1.
//! Every fast acceptance checks a vector recurrence and the Hankel equations.
//! A failure requests dense extraction; it never means that there are no roots.
use super::*;

#[derive(Clone, Debug, Default, Serialize)]
pub struct KrylovStats {
    pub attempts: u64,
    pub completed: u64,
    pub dense_fallbacks: u64,
    pub graph_edges: u64,
    pub walk_muls: u64,
    pub recurrence_muls: u64,
    pub reconstruction_muls: u64,
    pub scalar_steps: u64,
    pub delayed_steps: u64,
}

fn advance(
    graph: &[Vec<(usize, u64)>],
    previous: &[u64],
    p: u64,
    muls: &mut u64,
    delayed: bool,
) -> Vec<u64> {
    graph
        .iter()
        .map(|edges| {
            let mut sum = 0;
            for &(i, c) in edges {
                if delayed {
                    // The caller establishes <=64 normalized terms and p<2^20:
                    // sum <=64*(p-1)^2 <2^46. Preserve semantic field hooks.
                    let term = if c == 1 {
                        previous[i]
                    } else {
                        super::super::jv_bitops::hook_mul(p);
                        *muls += 1;
                        c * previous[i]
                    };
                    super::super::jv_bitops::hook_add(p);
                    sum += term;
                } else {
                    let term = if c == 1 {
                        previous[i]
                    } else {
                        *muls += 1;
                        mm(c, previous[i], p)
                    };
                    sum = am(sum, term, p);
                }
            }
            if delayed {
                sum % p
            } else {
                sum
            }
        })
        .collect()
}

/// Polynomial long division, with the same multiplication accounting as PolyRing.
fn div_rem(ring: &PolyRing, a: &UPoly, b: &UPoly) -> Option<(UPoly, UPoly)> {
    let &lead = b.0.last()?;
    let inv = inv_mod(lead, ring.p);
    let mut r = a.0.clone();
    let mut q = vec![0; a.0.len().saturating_sub(b.0.len()) + 1];
    while !r.is_empty() && r.len() >= b.0.len() {
        let shift = r.len() - b.0.len();
        let c = mm(*r.last()?, inv, ring.p);
        ring.count(1 + b.0.len() as u64);
        q[shift] = c;
        for (i, &v) in b.0.iter().enumerate() {
            r[shift + i] = sm(r[shift + i], mm(c, v, ring.p), ring.p);
        }
        r = trim(r);
    }
    Some((UPoly(trim(q)), UPoly(r)))
}

fn inverse(ring: &PolyRing, a: &UPoly, q: &UPoly) -> Option<UPoly> {
    let (mut r0, mut r1) = (q.clone(), ring.rem(a, q));
    let (mut t0, mut t1) = (UPoly(vec![]), UPoly(vec![1]));
    while !r1.0.is_empty() {
        let (quotient, rest) = div_rem(ring, &r0, &r1)?;
        let t = ring.rem(&ring.sub(&t0, &ring.mul(&quotient, &t1)), q);
        (r0, r1) = (r1, rest);
        (t0, t1) = (t1, t);
    }
    if r0.0.len() != 1 {
        return None;
    }
    let out = ring.scale(&t0, inv_mod(r0.0[0], ring.p));
    (ring.mulmod(a, &out, q).0 == vec![1]).then_some(out)
}

fn numerator(ring: &PolyRing, q: &UPoly, sequence: &[u64]) -> UPoly {
    let d = q.0.len() - 1;
    let mut out = vec![0; d];
    for (j, v) in out.iter_mut().enumerate() {
        for (k, &s) in sequence.iter().take(d - j).enumerate() {
            *v = am(*v, mm(q.0[j + k + 1], s, ring.p), ring.p);
            ring.count(1);
        }
    }
    UPoly(trim(out))
}

fn checked_hankel(s: &[u64], b: &[u64], h: &UPoly, p: u64, muls: &mut u64) -> bool {
    b.iter().enumerate().all(|(j, &bj)| {
        let mut value = 0;
        for (k, &hk) in h.0.iter().enumerate() {
            value = am(value, mm(hk, s[j + k], p), p);
            *muls += 1;
        }
        value == bj
    })
}

fn checked_vector_recurrence(frames: &[Vec<u64>], q: &UPoly, p: u64, muls: &mut u64) -> bool {
    (0..frames[0].len()).all(|j| {
        let mut acc = 0;
        for (k, &c) in q.0.iter().enumerate() {
            acc = am(acc, mm(c, frames[k][j], p), p);
            *muls += 1;
        }
        acc == 0
    })
}

/// Full degree plus these exact checks certifies the shape representation.
/// `indices` locates 1,e1,e2,e3 in the supplied standard-monomial basis.
fn shape_polynomials(
    matrix: &[Vec<u64>],
    indices: [usize; 4],
    p: u64,
    rng: &mut StdRng,
    work: &mut KrylovStats,
    muls: &mut u64,
    use_delayed: bool,
) -> Option<(UPoly, [UPoly; 2])> {
    let d = matrix.len();
    if d < 2 || indices.iter().any(|&i| i >= d) {
        return None;
    }
    let delayed =
        use_delayed && d <= 64 && p > 1 && p < (1 << 20) && matrix.iter().flatten().all(|&c| c < p);
    // M's columns contain NF(e1*b). These edges apply M^t to a projection.
    let graph: Vec<Vec<_>> = (0..d)
        .map(|j| {
            (0..d)
                .filter_map(|i| (matrix[i][j] != 0).then_some((i, matrix[i][j])))
                .collect()
        })
        .collect();
    work.graph_edges += graph.iter().map(Vec::len).sum::<usize>() as u64;
    for attempt in 0..3 {
        work.attempts += 1;
        let mut projection = vec![0; d];
        if attempt == 0 {
            projection[0] = 1;
        } else {
            for v in &mut projection {
                *v = rng.gen_range(0..p);
            }
        }
        let before = *muls;
        let mut frames = vec![projection];
        for k in 1..2 * d {
            let previous = &frames[k - 1];
            let next = advance(&graph, previous, p, muls, delayed);
            if delayed {
                work.delayed_steps += 1;
            } else {
                work.scalar_steps += 1;
            }
            frames.push(next);
        }
        work.walk_muls += *muls - before;
        let s: Vec<_> = frames.iter().map(|w| w[indices[0]]).collect();
        let before = *muls;
        let mut recurrence = berlekamp_massey_u64(&s, p, muls);
        recurrence.reverse();
        let q = UPoly(recurrence);
        let certified = q.0.len() == d + 1
            && q.0[0] != 0
            && q.0[d] == 1
            && checked_vector_recurrence(&frames, &q, p, muls);
        work.recurrence_muls += *muls - before;
        if !certified {
            continue;
        }
        let before = *muls;
        let ring = PolyRing::new(p);
        let recovered = (|| {
            let u = numerator(&ring, &q, &s);
            let inv = inverse(&ring, &u, &q)?;
            let mut coordinates = [UPoly(vec![]), UPoly(vec![])];
            for (i, h) in coordinates.iter_mut().enumerate() {
                let b: Vec<_> = frames.iter().take(d).map(|w| w[indices[i + 2]]).collect();
                *h = ring.mulmod(&numerator(&ring, &q, &b), &inv, &q);
                if !checked_hankel(&s, &b, h, p, muls) {
                    return None;
                }
            }
            Some(coordinates)
        })();
        *muls += ring.muls.get();
        work.reconstruction_muls += *muls - before;
        if let Some(h) = recovered {
            return Some((q, h));
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
pub(super) fn read(
    matrix: &[Vec<u64>],
    indices: [usize; 4],
    comps: &[HashMap<[u8; 3], u64>],
    p: u64,
    rng: &mut StdRng,
    stats: &mut SolveStats,
    work: &mut KrylovStats,
    muls: &mut u64,
    use_delayed: bool,
) -> Option<Vec<[u64; 3]>> {
    // Projection randomness cannot alter the caller's root-factoring stream.
    let mut projections = StdRng::seed_from_u64(0x4b52_594c_4f56 ^ p);
    let (q, h) = shape_polynomials(
        matrix,
        indices,
        p,
        &mut projections,
        work,
        muls,
        use_delayed,
    )?;
    let ring = PolyRing::new(p);
    let mut out = Vec::new();
    for lam in ring.roots(&q, rng) {
        let e = [lam, ring.eval(&h[0], lam), ring.eval(&h[1], lam)];
        // Redundant equation check also guards the representation convention.
        if !comps.iter().all(|comp| {
            let mut acc = 0;
            for (&m, &c) in comp {
                let mut term = c;
                for k in 0..3 {
                    for _ in 0..m[k] {
                        term = mm(term, e[k], p);
                        *muls += 1;
                    }
                }
                acc = am(acc, term, p);
            }
            acc == 0
        }) {
            *muls += ring.muls.get();
            stats.roots_muls += ring.muls.get();
            return None;
        }
        let cubic = UPoly(vec![(p - e[2]) % p, e[1], (p - e[0]) % p, 1]);
        let roots = ring.roots(&cubic, rng);
        'split: for &a in &roots {
            for &b in &roots {
                for &c in &roots {
                    if a > b || b > c {
                        continue;
                    }
                    let pair_sum = am(am(mm(a, b, p), mm(a, c, p), p), mm(b, c, p), p);
                    let product = mm(mm(a, b, p), c, p);
                    *muls += 5;
                    if am(am(a, b, p), c, p) == e[0] && pair_sum == e[1] && product == e[2] {
                        out.push([a, b, c]);
                        stats.split_cubics += 1;
                        break 'split;
                    }
                }
            }
        }
        stats.e_solutions += 1;
    }
    *muls += ring.muls.get();
    stats.roots_muls += ring.muls.get();
    work.completed += 1;
    out.sort_unstable();
    out.dedup();
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn companion_graph_recovers_checked_coordinate_polynomials() {
        let p = 101;
        let ring = PolyRing::new(p);
        let mut q = UPoly(vec![1]);
        for r in [1, 2, 5, 7] {
            q = ring.mul(&q, &UPoly(vec![p - r, 1]));
        }
        let mut matrix = vec![vec![0; 4]; 4];
        for j in 0..3 {
            matrix[j + 1][j] = 1;
        }
        for i in 0..4 {
            matrix[i][3] = sm(0, q.0[i], p);
        }
        // Exercise arbitrary basis ordering, including the constant position.
        for permutation in [[0, 1, 2, 3], [3, 1, 0, 2]] {
            let mut m = vec![vec![0; 4]; 4];
            let mut indices = [0; 4];
            for i in 0..4 {
                indices[permutation[i]] = i;
                for j in 0..4 {
                    m[i][j] = matrix[permutation[i]][permutation[j]];
                }
            }
            let got = shape_polynomials(
                &m,
                indices,
                p,
                &mut StdRng::seed_from_u64(3),
                &mut KrylovStats::default(),
                &mut 0,
                false,
            )
            .unwrap();
            assert_eq!(got.0 .0, q.0);
            assert_eq!(got.1[0].0, vec![0, 0, 1]);
            assert_eq!(got.1[1].0, vec![0, 0, 0, 1]);
        }
    }
    #[test]
    fn deficient_and_zero_eigenvalue_graphs_request_fallback() {
        for m in [vec![vec![1, 0], vec![0, 1]], vec![vec![0, 0], vec![1, 0]]] {
            assert!(shape_polynomials(
                &m,
                [0, 1, 0, 1],
                101,
                &mut StdRng::seed_from_u64(9),
                &mut KrylovStats::default(),
                &mut 0,
                false
            )
            .is_none());
        }
    }
    #[test]
    fn changed_hankel_coordinates_and_vector_recurrence_are_rejected() {
        let s = vec![1, 2, 4, 8];
        assert!(checked_hankel(&s, &[2, 4], &UPoly(vec![0, 1]), 101, &mut 0));
        assert!(!checked_hankel(
            &s,
            &[2, 5],
            &UPoly(vec![0, 1]),
            101,
            &mut 0
        ));
        let frames = vec![vec![1, 3], vec![2, 6], vec![4, 12]];
        assert!(checked_vector_recurrence(
            &frames,
            &UPoly(vec![99, 1]),
            101,
            &mut 0
        ));
        assert!(!checked_vector_recurrence(
            &frames,
            &UPoly(vec![98, 1]),
            101,
            &mut 0
        ));
    }

    #[test]
    fn delayed_walks_match_u128_reference_at_the_guard_boundary() {
        let mut rng = StdRng::seed_from_u64(0x44454c4159);
        for p in [2, 7, 1039, (1 << 20) - 1] {
            for d in [2, 17, 64] {
                for boundary in [false, true] {
                    let matrix: Vec<Vec<u64>> = (0..d)
                        .map(|_| {
                            (0..d)
                                .map(|_| if boundary { p - 1 } else { rng.gen_range(0..p) })
                                .collect()
                        })
                        .collect();
                    let graph: Vec<Vec<_>> = matrix
                        .iter()
                        .map(|row| {
                            row.iter()
                                .enumerate()
                                .filter_map(|(i, &c)| (c != 0).then_some((i, c)))
                                .collect()
                        })
                        .collect();
                    let previous: Vec<_> = (0..d)
                        .map(|_| if boundary { p - 1 } else { rng.gen_range(0..p) })
                        .collect();
                    let (mut scalar_muls, mut delayed_muls) = (0, 0);
                    let scalar = advance(&graph, &previous, p, &mut scalar_muls, false);
                    let delayed = advance(&graph, &previous, p, &mut delayed_muls, true);
                    let reference: Vec<_> = graph
                        .iter()
                        .map(|edges| {
                            (edges
                                .iter()
                                .map(|&(i, c)| c as u128 * previous[i] as u128)
                                .sum::<u128>()
                                % p as u128) as u64
                        })
                        .collect();
                    assert_eq!(scalar, reference);
                    assert_eq!(delayed, reference);
                    assert_eq!(scalar_muls, delayed_muls);
                }
            }
        }
    }
}
