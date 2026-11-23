//! Symmetric-coordinate S3 pair decomposition over a cubic extension.
//!
//! For s=x1+x2, t=x1*x2, write S3=t²+u0(s)t+v0(s), with the two
//! nonconstant extension components uk(s)t+vk(s)=0. Each u is linear,
//! each v quadratic. Their eliminant has degree at most three. Intersect
//! it with the constant equation *before* finite-field root extraction.
//! This is elementary elimination of the existing Semaev equation, not a
//! new discrete-log complexity claim. Singular cases retain a counted scan.

use super::*;

#[derive(Clone, Debug, Default, Serialize)]
pub struct SymmetricPairStats {
    pub calls: u64,
    pub cubic_eliminations: u64,
    pub constant_gcd_rejections: u64,
    pub degenerate_scans: u64,
    pub zero_denominator_sums: u64,
    pub symmetric_candidates: u64,
}

pub fn weil_s3_pair_test_symmetric(
    inst: &Instance3,
    base: &SubspaceBase,
    y: &Pt3,
    rng: &mut StdRng,
) -> (Vec<Vec<(usize, i64)>>, u64) {
    pair_test(inst, base, y, rng, &mut SymmetricPairStats::default())
}

pub(super) fn pair_test(
    inst: &Instance3,
    base: &SubspaceBase,
    y: &Pt3,
    rng: &mut StdRng,
    stats: &mut SymmetricPairStats,
) -> (Vec<Vec<(usize, i64)>>, u64) {
    stats.calls += 1;
    if y.inf {
        return (vec![], 0);
    }
    let f = &inst.curve.field;
    let before = f.muls();
    let x2 = f.sq(&y.x);
    // S3 = t² + (-4x²-2a-2xs)t
    //            + (a²-4bx) + (-2ax-4b)s + x²s².
    let uc = f.neg(&f.add(&f.scale(&x2, 4), &f.scale(&inst.curve.a, 2)));
    let us = f.neg(&f.scale(&y.x, 2));
    let vc = f.sub(
        &f.sq(&inst.curve.a),
        &f.scale(&f.mul(&inst.curve.b, &y.x), 4),
    );
    let vs = f.neg(&f.add(
        &f.scale(&f.mul(&inst.curve.a, &y.x), 2),
        &f.scale(&inst.curve.b, 4),
    ));
    let component_muls = f.muls() - before;
    let u: [UPoly; 3] = std::array::from_fn(|k| UPoly(trim(vec![uc.0[k], us.0[k]])));
    let v: [UPoly; 3] = std::array::from_fn(|k| UPoly(trim(vec![vc.0[k], vs.0[k], x2.0[k]])));
    let ring = PolyRing::new(f.p);
    let eliminant = ring.sub(&ring.mul(&u[1], &v[2]), &ring.mul(&u[2], &v[1]));
    if eliminant.0.is_empty() {
        // Dependent components can define a positive-dimensional system.
        // Do not divide by zero or reinterpret a zero polynomial as no roots.
        stats.degenerate_scans += 1;
        return (scan_pairs(inst, base, y), component_muls + ring.muls.get());
    }
    stats.cubic_eliminations += 1;
    let pivot = if !u[1].0.is_empty() { 1 } else { 2 };
    // Substitute t=-v/u into t²+u0*t+v0, without division:
    // v²-u*u0*v+u²*v0. All genuine solutions survive gcd, including
    // exceptional sums for which the chosen denominator vanishes.
    let compatibility = ring.add(
        &ring.sub(
            &ring.mul(&v[pivot], &v[pivot]),
            &ring.mul(&ring.mul(&u[pivot], &u[0]), &v[pivot]),
        ),
        &ring.mul(&ring.mul(&u[pivot], &u[pivot]), &v[0]),
    );
    let common = ring.gcd(&eliminant, &compatibility);
    if PolyRing::deg(&common) == 0 {
        stats.constant_gcd_rejections += 1;
    }
    let found = finish_candidates(inst, base, y, rng, stats, &u, &v, &common, &ring);
    (found, component_muls + ring.muls.get())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn finish_candidates(
    inst: &Instance3,
    base: &SubspaceBase,
    y: &Pt3,
    rng: &mut StdRng,
    stats: &mut SymmetricPairStats,
    u: &[UPoly; 3],
    v: &[UPoly; 3],
    common: &UPoly,
    ring: &PolyRing,
) -> Vec<Vec<(usize, i64)>> {
    let f = &inst.curve.field;
    let mut found = Vec::new();
    for s in ring.roots(common, rng) {
        let us: [u64; 3] = std::array::from_fn(|k| ring.eval(&u[k], s));
        let vs: [u64; 3] = std::array::from_fn(|k| ring.eval(&v[k], s));
        let products = if let Some(k) = (1..3).find(|&k| us[k] != 0) {
            ring.count(1);
            vec![mm((f.p - vs[k]) % f.p, inv_mod(us[k], f.p), f.p)]
        } else {
            stats.zero_denominator_sums += 1;
            if vs[1] != 0 || vs[2] != 0 {
                continue;
            }
            ring.roots(&UPoly(vec![vs[0], us[0], 1]), rng)
        };
        for t in products {
            if (0..3)
                .any(|k| ring.eval(&UPoly(trim(vec![vs[k], us[k], u64::from(k == 0)])), t) != 0)
            {
                continue;
            }
            stats.symmetric_candidates += 1;
            let roots = ring.roots(&UPoly(vec![t, (f.p - s) % f.p, 1]), rng);
            let (x1, x2) = match *roots.as_slice() {
                [r] => {
                    ring.count(1);
                    if am(r, r, f.p) != s || mm(r, r, f.p) != t {
                        continue;
                    }
                    (r, r)
                }
                [a, b] => (a, b),
                _ => continue,
            };
            let (Some(&i), Some(&j)) = (base.by_x.get(&x1), base.by_x.get(&x2)) else {
                continue;
            };
            append_signs(inst, base, y, i.min(j), i.max(j), &mut found);
        }
    }
    found.sort();
    found.dedup();
    found
}

fn append_signs(
    inst: &Instance3,
    base: &SubspaceBase,
    y: &Pt3,
    i: usize,
    j: usize,
    found: &mut Vec<Vec<(usize, i64)>>,
) {
    let c = &inst.curve;
    for si in [1, -1] {
        let rest = c.sub(y, &c.mul_signed(&base.points[i], si));
        let sj = if rest == base.points[j] {
            1
        } else if rest == c.neg(&base.points[j]) {
            -1
        } else {
            continue;
        };
        if i != j {
            found.push(vec![(i, si), (j, sj)]);
        } else if si + sj != 0 {
            found.push(vec![(i, si + sj)]);
        }
    }
}

pub(super) fn scan_pairs(inst: &Instance3, base: &SubspaceBase, y: &Pt3) -> Vec<Vec<(usize, i64)>> {
    let c = &inst.curve;
    let mut found = Vec::new();
    for (i, point) in base.points.iter().enumerate() {
        for si in [1, -1] {
            let rest = c.sub(y, &c.mul_signed(point, si));
            if let Some((j, sj)) = base.lookup(&c.field, &rest) {
                if j > i {
                    found.push(vec![(i, si), (j, sj)]);
                } else if j == i && si + sj != 0 {
                    found.push(vec![(i, si + sj)]);
                }
            }
        }
    }
    found.sort();
    found.dedup();
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    // Independent full signed-pair enumeration, including repeated indices.
    fn brute(inst: &Instance3, base: &SubspaceBase, y: &Pt3) -> Vec<Vec<(usize, i64)>> {
        let c = &inst.curve;
        let mut result = Vec::new();
        for i in 0..base.len() {
            for j in i..base.len() {
                for si in [1, -1] {
                    for sj in [1, -1] {
                        if i == j && si + sj == 0 {
                            continue;
                        }
                        if c.add(
                            &c.mul_signed(&base.points[i], si),
                            &c.mul_signed(&base.points[j], sj),
                        ) == *y
                        {
                            result.push(if i == j {
                                vec![(i, si + sj)]
                            } else {
                                vec![(i, si), (j, sj)]
                            });
                        }
                    }
                }
            }
        }
        result.sort();
        result.dedup();
        result
    }

    #[test]
    fn symmetric_pairs_match_every_tiny_group_target() {
        for seed in [3, 7] {
            let inst = generate_instance3(7, seed);
            let mut rng = StdRng::seed_from_u64(91);
            let base = SubspaceBase::build(&inst, &mut rng);
            for d in 1..inst.curve.n {
                let y = inst.curve.mul(&inst.curve.g, d);
                let (got, _) = weil_s3_pair_test_symmetric(&inst, &base, &y, &mut rng);
                assert_eq!(got, brute(&inst, &base, &y), "seed={seed} d={d}");
            }
        }
    }

    #[test]
    fn symmetric_pairs_match_all_constructed_signed_pairs() {
        let inst = generate_instance3(19, 11);
        let mut rng = StdRng::seed_from_u64(23);
        let base = SubspaceBase::build(&inst, &mut rng);
        let mut stats = SymmetricPairStats::default();
        for i in 0..base.len() {
            for j in i..base.len() {
                for sign in [1, -1] {
                    let y = inst.curve.add(
                        &base.points[i],
                        &inst.curve.mul_signed(&base.points[j], sign),
                    );
                    if !y.inf {
                        let (got, _) = pair_test(&inst, &base, &y, &mut rng, &mut stats);
                        assert_eq!(got, brute(&inst, &base, &y));
                    }
                }
            }
        }
        assert!(stats.cubic_eliminations > 0);
    }

    #[test]
    fn general_irreducible_cubic_uses_the_same_pair_identity() {
        let field = Fp3::with_irreducible_cubic(11, [1, 0, 4]).unwrap();
        let curve = Curve3::new(field, E3([1, 1, 0]), E3([2, 0, 1]), 5, Pt3::INFINITY);
        let inst = Instance3 {
            curve,
            q: Pt3::INFINITY,
            d: 0,
        };
        let mut rng = StdRng::seed_from_u64(71);
        let base = SubspaceBase::build(&inst, &mut rng);
        assert!(!base.is_empty());
        for i in 0..base.len() {
            for j in i..base.len() {
                let y = inst.curve.add(&base.points[i], &base.points[j]);
                if !y.inf {
                    assert_eq!(
                        weil_s3_pair_test_symmetric(&inst, &base, &y, &mut rng).0,
                        brute(&inst, &base, &y)
                    );
                }
            }
        }
    }

    #[test]
    fn symmetric_triple_channel_agrees_with_existing_oracle() {
        let inst = generate_instance3(13, 3);
        let base = SubspaceBase::build(&inst, &mut StdRng::seed_from_u64(17));
        for scalar in 1..30 {
            let y = inst.curve.mul(&inst.curve.g, scalar);
            let expected =
                subspace_triple_oracle(&inst, &base, &y, &mut StdRng::seed_from_u64(19)).0;
            let mut stats = SymmetricPairStats::default();
            let got = triple_with_pair(
                &inst,
                &base,
                &y,
                &mut StdRng::seed_from_u64(19),
                |point, rng| pair_test(&inst, &base, point, rng, &mut stats),
            )
            .0;
            assert_eq!(got, expected, "scalar={scalar}");
        }
        let rho = run_rho3(&inst, 3);
        assert!(rho.solved);
        assert_eq!(
            inst.curve.mul(&inst.curve.g, rho.recovered_scalar.unwrap()),
            inst.q
        );
    }

    #[test]
    fn vanishing_both_denominators_retains_a_repeated_pair() {
        // t^3=2, P=(0,3), a=-2t^2, b=2 give x(2P)=t.
        // For s=0 both extension-component denominators vanish, although
        // their eliminant is a nonzero multiple of s^3. Dividing first
        // would lose this genuine double-point relation.
        let field = Fp3::with_irreducible_cubic(7, [5, 0, 0]).unwrap();
        let curve = Curve3::new(field, E3([0, 0, 5]), E3([2, 0, 0]), 5, Pt3::INFINITY);
        let inst = Instance3 {
            curve,
            q: Pt3::INFINITY,
            d: 0,
        };
        let mut rng = StdRng::seed_from_u64(31);
        let base = SubspaceBase::build(&inst, &mut rng);
        let point = Pt3::affine(E3([0, 0, 0]), E3([3, 0, 0]));
        assert!(inst.curve.is_on_curve(&point));
        let y = inst.curve.double(&point);
        assert_eq!(y.x, E3([0, 1, 0]));
        let mut stats = SymmetricPairStats::default();
        let (got, _) = pair_test(&inst, &base, &y, &mut rng, &mut stats);
        assert_eq!(got, brute(&inst, &base, &y));
        assert!(!got.is_empty());
        assert_eq!(stats.degenerate_scans, 0);
        assert!(stats.zero_denominator_sums > 0);
    }

    #[test]
    fn dependent_extension_components_use_exact_counted_scan() {
        let mut rng = StdRng::seed_from_u64(41);
        let curve = Curve3::new(
            Fp3::new(7, &mut rng),
            E3([1, 0, 0]),
            E3([1, 0, 0]),
            5,
            Pt3::INFINITY,
        );
        let inst = Instance3 {
            curve,
            q: Pt3::INFINITY,
            d: 0,
        };
        let base = SubspaceBase::build(&inst, &mut rng);
        let y = base.points[0];
        let before = inst.curve.ops();
        let mut stats = SymmetricPairStats::default();
        let (got, muls) = pair_test(&inst, &base, &y, &mut rng, &mut stats);
        assert_eq!(stats.degenerate_scans, 1);
        assert!(inst.curve.ops() > before && muls > 0);
        assert_eq!(got, brute(&inst, &base, &y));
    }
}
