use super::*;
use isogeny_algos::curve::{self, Curve, Pt};
use isogeny_algos::field::{Field, Rng};
use isogeny_algos::fpr::ER;
use isogeny_algos::poly;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

fn er(a: K) -> ER {
    let mut x = [0; 16];
    x[..3].copy_from_slice(&a);
    x
}
fn kc(a: ER) -> K {
    assert!(a[3..].iter().all(|&x| x == 0));
    [a[0], a[1], a[2]]
}
fn ep(p: &Point) -> Pt<ER> {
    match p {
        None => Pt::Inf,
        Some((x, y)) => Pt::Aff(er(*x), er(*y)),
    }
}
fn kp(p: Pt<ER>) -> Point {
    match p {
        Pt::Inf => None,
        Pt::Aff(x, y) => Some((kc(x), kc(y))),
    }
}
fn source(a: &Adapter) -> (FpR, Curve<ER>) {
    (
        FpR::from_modulus(a.q, &a.modulus),
        Curve::new(er(a.core.ea), er(a.core.eb)),
    )
}
fn points(a: &Adapter) -> Vec<Point> {
    let (f, c) = source(a);
    let elements: Vec<_> = (0..a.q.pow(3))
        .map(|i| er(kindex(a.q, i).unwrap()))
        .collect();
    let mut squares: HashMap<ER, Vec<ER>> = HashMap::new();
    for &y in &elements {
        squares.entry(f.sq(y)).or_default().push(y);
    }
    let mut out = vec![None];
    for x in elements {
        if let Some(ys) = squares.get(&curve::rhs(&f, &c, x)) {
            out.extend(ys.iter().map(|&y| Some((kc(x), kc(y)))));
        }
    }
    out.sort();
    assert_eq!(out.len(), a.expected_order as usize);
    assert!(isogeny_algos::field::is_prime(a.ell));
    assert_eq!(a.expected_order, 4 * a.ell);
    out
}
fn generator(a: &Adapter, ps: &[Point]) -> Point {
    let (f, c) = source(a);
    ps.iter()
        .filter_map(|p| {
            let g = curve::pmul(&f, &c, &ep(p), 4);
            (g != Pt::Inf && curve::pmul(&f, &c, &g, a.ell as u128) == Pt::Inf).then_some(kp(g))
        })
        .next()
        .expect("frozen first-generator rule")
}
fn smul(f: &FpR, a: &[ER], b: &[ER], n: usize) -> Vec<ER> {
    let mut v = poly::mul(f, &a.to_vec(), &b.to_vec());
    v.truncate(n);
    v.resize(n, f.zero());
    v
}
fn spow(f: &FpR, a: &[ER], e: u8, n: usize) -> Vec<ER> {
    let mut v = vec![f.one()];
    v.resize(n, f.zero());
    for _ in 0..e {
        v = smul(f, &v, a, n);
    }
    v
}
fn monseries(f: &FpR, xyz: &[Vec<ER>; 3], e: Mono, n: usize) -> Vec<ER> {
    smul(
        f,
        &smul(f, &spow(f, &xyz[0], e.0, n), &spow(f, &xyz[1], e.1, n), n),
        &spow(f, &xyz[2], e.2, n),
        n,
    )
}
fn curve_series(f: &FpR, a: &Adapter, xyz: &[Vec<ER>; 3], n: usize) -> Vec<ER> {
    let mut out = vec![f.zero(); n];
    for (e, &c) in pic::mons_of_degree(4)
        .into_iter()
        .zip(a.core.quartic.iter())
    {
        let value = monseries(f, xyz, e, n);
        for j in 0..n {
            out[j] = f.add(out[j], f.mul(f.from_u64(c), value[j]));
        }
    }
    out
}
type GeoPoint = [ER; 3];
fn normalized(f: &FpR, p: GeoPoint) -> GeoPoint {
    let inv = f.inv(
        *p.iter()
            .find(|&&x| !f.is_zero(x))
            .expect("nonzero projective point"),
    );
    p.map(|x| f.mul(x, inv))
}

struct Oracle {
    f: FpR,
    theta: ER,
    theta2: ER,
    om: [[ER; 3]; 3],
}
struct Geometry {
    space: Gen,
    equations: Vec<Vec<u32>>,
    multiplicities: Vec<usize>,
    nonsplit: bool,
    ramified: bool,
}
impl Oracle {
    fn new(a: &Adapter) -> Self {
        let f = FpR::new(a.q, 6);
        let modulus = a.modulus.iter().map(|&c| f.from_u64(c)).collect();
        let roots = poly::roots(&f, &modulus, &mut Rng::new(20261113 + a.q));
        assert_eq!(roots.len(), 3);
        let theta = roots[0];
        let theta2 = f.sq(theta);
        assert_eq!(poly::eval(&f, &modulus, theta), f.zero());
        assert_eq!(f.pow(theta, a.q.pow(3) as u128), theta);
        assert_ne!(f.pow(theta, a.q as u128), theta);
        // Construct the historical Moore rows directly in the separate field,
        // rather than multiplying the runtime's coordinate-transport matrix.
        let gamma = f.add(theta, f.from_u64(a.moore_shift));
        let basis = [f.one(), gamma, f.sq(gamma)];
        let om = basis.map(|x| [x, f.pow(x, a.q as u128), f.pow(x, a.q.pow(2) as u128)]);
        let o = Self {
            f,
            theta,
            theta2,
            om,
        };
        // Check exact embedding and inverse Moore convention, no array reinterpretation.
        for i in 0..3 {
            for j in 0..3 {
                let mut x = o.f.zero();
                for k in 0..3 {
                    x = o.f.add(x, o.f.mul(o.embed(a.core.omi[i][k]), o.om[k][j]));
                }
                assert_eq!(x, o.f.from_u64(u64::from(i == j)));
            }
        }
        o
    }
    fn embed(&self, x: K) -> ER {
        let f = &self.f;
        f.add(
            f.add(f.from_u64(x[0]), f.mul(f.from_u64(x[1]), self.theta)),
            f.mul(f.from_u64(x[2]), self.theta2),
        )
    }
    fn eval_u(&self, a: &Adapter, i: usize, t: ER) -> ER {
        let f = &self.f;
        let b = f.pow(self.embed(a.ub), a.q.pow(i as u32) as u128);
        let c = f.pow(self.embed(a.uc), a.q.pow(i as u32) as u128);
        f.add(f.add(f.sq(t), f.mul(b, t)), c)
    }
    fn raw_points(&self, a: &Adapter, p: &Point) -> (Vec<GeoPoint>, bool, bool) {
        let f = &self.f;
        let one = f.one();
        let zero = f.zero();
        // Independently invert the source short model in Fp6.
        let shift = self.embed(a.core.wshift);
        let alpha = self.embed(a.core.walpha);
        let b = f.div(self.embed(a.core.wb), f.from_u64(2));
        let s = self.embed(a.core.ws);
        let xyz: Vec<GeoPoint>;
        let mut nonsplit = false;
        let mut ramified = false;
        let finite = match p {
            None => None,
            Some((xs, y)) => {
                let x = f.sub(self.embed(*xs), shift);
                let y = self.embed(*y);
                if f.is_zero(x) && y == f.neg(alpha) {
                    Some(Err(()))
                } else {
                    let t = if f.is_zero(x) {
                        assert_eq!(y, alpha);
                        let c = self.embed(a.uc);
                        let c2 = f.pow(c, a.q.pow(2) as u128);
                        f.neg(f.div(f.sub(f.mul(c, c2), f.sq(s)), alpha))
                    } else {
                        f.div(f.sub(f.sub(y, f.mul(b, x)), alpha), f.mul(f.from_u64(2), x))
                    };
                    let ordinate = f.sub(
                        f.sub(f.sub(f.div(x, f.from_u64(2)), f.sq(t)), f.mul(b, t)),
                        s,
                    );
                    assert_eq!(
                        f.sq(ordinate),
                        f.mul(self.eval_u(a, 0, t), self.eval_u(a, 2, t))
                    );
                    Some(Ok((t, ordinate)))
                }
            }
        };
        match finite {
            None => {
                xyz = vec![[one, one, one], [f.neg(one), one, one]];
            }
            Some(Err(())) => {
                xyz = vec![[one, f.neg(one), one], [f.neg(one), f.neg(one), one]];
            }
            Some(Ok((t, y))) => {
                let u0 = self.eval_u(a, 0, t);
                let u1 = self.eval_u(a, 1, t);
                let u2 = self.eval_u(a, 2, t);
                let ratio = if f.is_zero(u0) {
                    f.div(u1, u2)
                } else {
                    f.div(u1, u0)
                };
                let root = f.sqrt(ratio).expect("all degree-two fibres split in Fp6");
                assert_eq!(f.sq(root), ratio);
                nonsplit = f.pow(root, a.q.pow(3) as u128) != root;
                ramified = f.is_zero(root);
                if f.is_zero(u0) {
                    assert!(f.is_zero(y));
                    xyz = vec![[root, one, zero], [f.neg(root), one, zero]];
                } else {
                    let yy = f.div(y, u0);
                    xyz = vec![[root, yy, one], [f.neg(root), yy, one]];
                }
            }
        }
        let mut points = Vec::new();
        for point in xyz {
            let mut w = [zero; 3];
            for (j, v) in w.iter_mut().enumerate() {
                for (i, &x) in point.iter().enumerate() {
                    *v = f.add(*v, f.mul(self.om[j][i], x));
                }
            }
            let w = normalized(f, w);
            let series = w.map(|v| vec![v]);
            assert_eq!(curve_series(f, a, &series, 1), vec![zero]);
            // Original cover coordinates, evaluated independently in Fp6.
            let sq = [f.sq(point[2]), f.sq(point[0]), f.sq(point[1])];
            let row = |i: usize| {
                (0..3).fold(zero, |acc, j| {
                    f.add(acc, f.mul(self.embed(a.core.rmat[i][j]), sq[j]))
                })
            };
            let s0 = row(0);
            let s1 = row(1);
            let got = if f.is_zero(s0) {
                let r = f.div(point[1], point[2]);
                if r == one {
                    Pt::Inf
                } else {
                    assert_eq!(r, f.neg(one));
                    Pt::Aff(shift, f.neg(alpha))
                }
            } else {
                let t = f.div(s1, s0);
                let u0 = self.eval_u(a, 0, t);
                let y = if f.is_zero(point[2]) {
                    zero
                } else {
                    f.mul(u0, f.div(point[1], point[2]))
                };
                let xx = f.mul(
                    f.from_u64(2),
                    f.add(f.add(f.add(y, f.sq(t)), f.mul(b, t)), s),
                );
                let yy = f.add(
                    f.add(f.mul(f.mul(f.from_u64(2), xx), t), f.mul(b, xx)),
                    alpha,
                );
                Pt::Aff(f.add(xx, shift), yy)
            };
            let expected = match p {
                None => Pt::Inf,
                Some((x, y)) => Pt::Aff(self.embed(*x), self.embed(*y)),
            };
            assert_eq!(got, expected);
            points.push(w);
        }
        (points, nonsplit, ramified)
    }
    fn jets(&self, a: &Adapter, p: GeoPoint, m: usize) -> [Vec<ER>; 3] {
        let f = &self.f;
        let fixed = (0..3).find(|&i| p[i] == f.one()).expect("normalized chart");
        let axes: Vec<_> = (0..3).filter(|&i| i != fixed).collect();
        let mut dependent = None;
        for &axis in &axes {
            let mut trial = p.map(|c| vec![c, f.zero()]);
            trial[axis][1] = f.one();
            let gradient = curve_series(f, a, &trial, 2)[1];
            if !f.is_zero(gradient) {
                dependent = Some((axis, gradient));
                break;
            }
        }
        let (dep, gradient) = dependent.expect("smooth affine local parameter");
        let free = *axes.iter().find(|&&i| i != dep).unwrap();
        let mut series = p.map(|c| {
            let mut v = vec![f.zero(); m];
            v[0] = c;
            v
        });
        if m > 1 {
            series[free][1] = f.one();
        }
        for k in 1..m {
            let known = curve_series(f, a, &series, k + 1)[k];
            series[dep][k] = f.neg(f.div(known, gradient));
        }
        assert!(curve_series(f, a, &series, m).iter().all(|&x| f.is_zero(x)));
        series
    }
    fn geometric_ramification_controls(&self, a: &Adapter) -> usize {
        // Coverage addendum d664a3: all four geometric branch images, including
        // the q=5 images outside the public E(K) runtime input domain.
        let f = &self.f;
        let b1 = f.pow(self.embed(a.ub), a.q as u128);
        let c1 = f.pow(self.embed(a.uc), a.q as u128);
        let roots = poly::roots(f, &vec![c1, b1, f.one()], &mut Rng::new(2026111301 + a.q));
        assert_eq!(roots.len(), 2);
        let elliptic = Curve::new(self.embed(a.core.ea), self.embed(a.core.eb));
        let b = f.div(self.embed(a.core.wb), f.from_u64(2));
        let mut rational = 0;
        let mut checked = 0;
        for t0 in roots {
            assert!(f.is_zero(self.eval_u(a, 1, t0)));
            assert!(!f.is_zero(f.add(f.mul(f.from_u64(2), t0), b1)));
            let u0 = self.eval_u(a, 0, t0);
            let u2 = self.eval_u(a, 2, t0);
            assert!(!f.is_zero(u0) && !f.is_zero(u2));
            let root = f.sqrt(f.mul(u0, u2)).expect("Fp6 branch ordinate");
            assert!(!f.is_zero(root));
            for y0 in [root, f.neg(root)] {
                let in_k =
                    f.pow(t0, a.q.pow(3) as u128) == t0 && f.pow(y0, a.q.pow(3) as u128) == y0;
                rational += usize::from(in_k);
                let xx = f.mul(
                    f.from_u64(2),
                    f.add(
                        f.add(f.add(y0, f.sq(t0)), f.mul(b, t0)),
                        self.embed(a.core.ws),
                    ),
                );
                let yy = f.add(
                    f.add(f.mul(f.mul(f.from_u64(2), xx), t0), f.mul(b, xx)),
                    self.embed(a.core.walpha),
                );
                assert!(curve::on_curve(
                    f,
                    &elliptic,
                    &Pt::Aff(f.add(xx, self.embed(a.core.wshift)), yy)
                ));
                let ratio = f.div(y0, u0);
                let ciani = [f.zero(), ratio, f.one()];
                let w = std::array::from_fn(|i| {
                    (0..3).fold(f.zero(), |sum, j| {
                        f.add(sum, f.mul(self.om[i][j], ciani[j]))
                    })
                });
                let jets = self.jets(a, normalized(f, w), 3);
                let xyz: [Vec<ER>; 3] = std::array::from_fn(|i| {
                    (0..3)
                        .map(|k| {
                            (0..3).fold(f.zero(), |sum, j| {
                                f.add(sum, f.mul(self.embed(a.core.omi[i][j]), jets[j][k]))
                            })
                        })
                        .collect()
                });
                assert!(f.is_zero(xyz[0][0]));
                assert!(!f.is_zero(xyz[0][1]));
                let squares = [
                    smul(f, &xyz[2], &xyz[2], 3),
                    smul(f, &xyz[0], &xyz[0], 3),
                    smul(f, &xyz[1], &xyz[1], 3),
                ];
                let row = |i: usize| -> Vec<ER> {
                    (0..3)
                        .map(|k| {
                            (0..3).fold(f.zero(), |sum, j| {
                                f.add(sum, f.mul(self.embed(a.core.rmat[i][j]), squares[j][k]))
                            })
                        })
                        .collect()
                };
                let s0 = row(0);
                assert!(!f.is_zero(s0[0]));
                let t = smul(f, &row(1), &isogeny_algos::series::inv(f, &s0, 3), 3);
                assert_eq!(t[0], t0);
                assert!(f.is_zero(t[1]));
                assert!(!f.is_zero(t[2]));
                let l: Vec<_> = (0..3)
                    .map(|k| f.sub(xyz[1][k], f.mul(ratio, xyz[2][k])))
                    .collect();
                assert!(l[..2].iter().all(|&v| f.is_zero(v)));
                assert!(squares[1][..2].iter().all(|&v| f.is_zero(v)));
                assert!(!f.is_zero(squares[1][2]));
                // X has order one, Q=X^2 has order two: the fibre is a
                // doubled point, with the nonzero nilpotent direction retained.
                println!(
                    "G3_GEOMETRIC_BRANCH {}",
                    serde_json::json!({"q":a.q,"t0":t0,"y0":y0,"K_rational":in_k,"t_jet":t,"X_jet":xyz[0],"L_jet":l,"Q_jet":squares[1],"ramification_index":2})
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 4);
        let expected = match a.q {
            5 => 0,
            7 | 11 => 4,
            _ => unreachable!(),
        };
        assert_eq!(rational, expected);
        checked
    }
    fn geometry(&self, a: &Adapter, p: &Point, reduced_union: bool) -> Geometry {
        let f = &self.f;
        let (raw, nonsplit, ramified) = self.raw_points(a, p);
        let mut supports: BTreeMap<GeoPoint, usize> = BTreeMap::new();
        for point in raw {
            let mut point = point;
            for _ in 0..3 {
                *supports.entry(point).or_default() += 1;
                point = normalized(f, point.map(|x| f.pow(x, a.q as u128)));
            }
        }
        assert_eq!(supports.values().sum::<usize>(), 6);
        let mut equations = Vec::new();
        for (&point, &multiplicity) in &supports {
            let m = if reduced_union { 1 } else { multiplicity };
            let jets = self.jets(a, point, m);
            let values: Vec<_> = pic::mons_of_degree(3)
                .into_iter()
                .map(|e| monseries(f, &jets, e, m))
                .collect();
            for k in 0..m {
                for component in 0..6 {
                    equations.push(values.iter().map(|v| v[k][component] as u32).collect());
                }
            }
        }
        let rows = a.pic.linear_kernel(equations.clone(), 10);
        if !reduced_union {
            assert_eq!(rows.len(), 4);
        }
        let mut multiplicities = supports.values().copied().collect::<Vec<_>>();
        multiplicities.sort();
        Geometry {
            space: Gen { k: 3, rows },
            equations,
            multiplicities,
            nonsplit,
            ramified,
        }
    }
}
#[test]
fn all_frozen_fields_models_and_completed_coordinate_branches() {
    for q in [5, 7, 11] {
        let a = Adapter::new(q).unwrap();
        let ps = points(&a);
        let (f, c) = source(&a);
        for i in 0..q.pow(3) {
            let x = kindex(q, i).unwrap();
            assert_eq!(kc(f.frobenius(er(x))), a.field.frob(&x));
            if x != [0; 3] {
                assert_eq!(kc(f.inv(er(x))), a.field.inv(&x));
            }
            for basis in [[1, 0, 0], [0, 1, 0], [0, 0, 1]] {
                assert_eq!(kc(f.mul(er(x), er(basis))), a.field.mul(&x, &basis));
            }
        }
        let mut branches = [0; 3];
        let mut seen = HashSet::new();
        for p in &ps {
            assert!(curve::on_curve(&f, &c, &ep(p)));
            a.admit(p).unwrap();
            let quartic = a.quartic_point(p).unwrap();
            assert_eq!(a.short_point(quartic).unwrap(), *p);
            let key = format!("{quartic:?}");
            assert!(seen.insert(key));
            if let Some((x, y)) = p {
                if *x == a.core.wshift {
                    if *y == a.core.walpha {
                        assert!(matches!(quartic, QuarticPoint::Finite(..)));
                        branches[0] += 1;
                    } else {
                        assert_eq!(quartic, QuarticPoint::InfinityMinus);
                        branches[1] += 1;
                    }
                }
            } else {
                branches[2] += 1;
            }
        }
        assert_eq!(branches, [1, 1, 1]);
        println!(
            "G3_COORD {}",
            serde_json::json!({"q":q,"source_points":ps.len(),"x0_finite_infinity_origin":branches,"field_basis":a.modulus,"short_a":a.core.ea,"short_b":a.core.eb,"moore_shift":a.moore_shift,"transport_scale":a.transport_scale,"generator":generator(&a,&ps)})
        );
    }
}
#[test]
fn every_fibre_scheme_matches_independent_fp6_point_and_jet_oracle() {
    for q in [5, 7, 11] {
        let a = Adapter::new(q).unwrap();
        let ps = points(&a);
        let oracle = Oracle::new(&a);
        let mut nonsplit = 0;
        let mut ramified = 0;
        let mut overlap = 0;
        let mut aux = 0;
        let origin = oracle.geometry(&a, &None, false);
        assert_eq!(origin.multiplicities, vec![1, 1, 1, 3]);
        assert!(oracle.geometry(&a, &None, true).space.rows.len() > 4);
        for p in &ps {
            let geo = oracle.geometry(&a, p, false);
            nonsplit += usize::from(geo.nonsplit);
            ramified += usize::from(geo.ramified);
            overlap += usize::from(geo.multiplicities.iter().any(|&m| m > 1));
            let runtime = a.trace_space(p).unwrap();
            let cubics = a.pic.cubic_space_deg6(&runtime);
            assert_eq!(cubics.len(), 4);
            assert_eq!(a.pic.linear_kernel(cubics.clone(), 10).len(), 6);
            for row in &cubics {
                for eq in &geo.equations {
                    let sum = row.iter().zip(eq).fold(0, |sum, (&x, &y)| {
                        a.field.f.add(sum, a.field.f.mul(x as u64, y as u64))
                    });
                    assert_eq!(sum, 0);
                }
            }
            let reference = a.pic.reduce_deg6(&geo.space);
            assert_eq!(a.pic.reduce_deg6(&runtime), reference);
            assert_eq!(
                a.transfer_point(p).unwrap(),
                a.pic.sub(&reference, a.origin_shift())
            );
            if p.is_none() || geo.ramified || (geo.nonsplit && aux < 5) {
                for which in 0..3 {
                    assert_eq!(a.pic.reduce_deg6_aux(&runtime, which), reference);
                }
                let gauged = a.trace_space_gauged(p, Some([2, 1, 0])).unwrap();
                assert_eq!(a.pic.reduce_deg6(&gauged), reference);
                aux += 1;
            }
        }
        assert!(nonsplit > 0);
        assert_eq!(ramified, if q == 5 { 0 } else { 4 });
        assert!(overlap > 0);
        let geometric_branches = oracle.geometric_ramification_controls(&a);
        println!(
            "G3_FIBRES {}",
            serde_json::json!({"q":q,"all_points":ps.len(),"nonsplit":nonsplit,"ramified":ramified,"geometric_branches":geometric_branches,"repeated_or_overlapping":overlap,"origin_multiplicities":origin.multiplicities,"aux_and_gauge_cases":aux})
        );
    }
}
#[test]
fn complete_group_generating_edges_and_all_odd_subgroup_scalars() {
    for q in [5, 7, 11] {
        let a = Adapter::new(q).unwrap();
        let ps = points(&a);
        let (f, c) = source(&a);
        let g = generator(&a, &ps);
        let mut gens = vec![g];
        gens.extend(
            ps.iter()
                .filter(|&&p| p.is_some() && curve::pmul(&f, &c, &ep(&p), 2) == Pt::Inf)
                .copied(),
        );
        let mut generated = HashSet::from([None]);
        let mut queue = VecDeque::from([None]);
        while let Some(p) = queue.pop_front() {
            for h in &gens {
                let r = kp(curve::padd(&f, &c, &ep(&p), &ep(h)));
                if generated.insert(r) {
                    queue.push_back(r);
                }
            }
        }
        assert_eq!(generated.len(), ps.len());
        assert!(ps.iter().all(|p| generated.contains(p)));
        let images: HashMap<_, _> = ps
            .iter()
            .map(|p| (*p, a.transfer_point(p).unwrap()))
            .collect();
        assert!(a.pic.is_zero(&images[&None]));
        let mut edges = 0;
        for p in &ps {
            let neg = kp(curve::neg(&f, &ep(p)));
            assert_eq!(images[&neg], a.pic.neg(&images[p]));
            let double = kp(curve::padd(&f, &c, &ep(p), &ep(p)));
            assert_eq!(images[&double], a.pic.double(&images[p]));
            for h in &gens {
                let r = kp(curve::padd(&f, &c, &ep(p), &ep(h)));
                assert_eq!(images[&r], a.pic.add(&images[p], &images[h]));
                edges += 1;
            }
        }
        let image_g = &images[&g];
        assert!(!a.pic.is_zero(image_g));
        assert!(a.pic.is_zero(&a.pic.mul_scalar(image_g, a.ell as u128)));
        let mut keys = HashSet::new();
        for k in 0..a.ell {
            let p = kp(curve::pmul(&f, &c, &ep(&g), k as u128));
            assert_eq!(images[&p], a.pic.mul_scalar(image_g, k as u128));
            assert!(keys.insert(images[&p].key()));
        }
        let kernel: Vec<_> = ps.iter().filter(|p| a.pic.is_zero(&images[p])).collect();
        assert!(kernel
            .iter()
            .all(|p| curve::pmul(&f, &c, &ep(p), 2) == Pt::Inf));
        println!(
            "G3_GROUP {}",
            serde_json::json!({"q":q,"points":ps.len(),"generators":gens.len(),"all_generating_edges":edges,"odd_scalar_images":keys.len(),"ell":a.ell,"observed_even_kernel":kernel.len()})
        );
    }
}
#[test]
fn strict_encoded_domains_and_nontrivial_negative_controls() {
    for q in [5, 7, 11] {
        let a = Adapter::new(q).unwrap();
        let positive = Some((a.core.wshift, a.core.walpha));
        let negative = Some((a.core.wshift, a.field.neg(&a.core.walpha)));
        assert_ne!(positive, negative);
        assert!(matches!(
            a.quartic_point(&positive).unwrap(),
            QuarticPoint::Finite(..)
        ));
        assert_eq!(
            a.quartic_point(&negative).unwrap(),
            QuarticPoint::InfinityMinus
        );
        assert_ne!(
            a.short_point(QuarticPoint::InfinityMinus).unwrap(),
            positive
        );
        let original = serde_json::from_str::<Value>(FIXTURES).unwrap();
        for i in 0..20 {
            let mut mutant = original.clone();
            mutant["cases"][0]["u_idx"][0] = serde_json::json!(i + 1000);
            assert!(
                Adapter::from_fixture_json(q, &serde_json::to_string(&mutant).unwrap()).is_err()
            );
        }
        let mut p = a.encode(positive);
        p.field.push('x');
        assert!(a.transfer(&p).is_err());
        let mut p = a.encode(positive);
        p.model.push('x');
        assert!(a.transfer(&p).is_err());
        let mut p = a.encode(positive);
        p.coordinates.as_mut().unwrap().0[0] = q;
        assert!(a.transfer(&p).is_err());
        let invalid = (0..q.pow(3))
            .map(|i| Some((kindex(q, i).unwrap(), [0; 3])))
            .find(|p| a.admit(p).is_err())
            .unwrap();
        assert!(a.transfer_point(&invalid).is_err());
        let origin = a.transfer_point(&None).unwrap();
        assert!(a.pic.is_zero(&origin));
        let omitted_differs = !a.pic.is_zero(a.origin_shift());
        if omitted_differs {
            assert_ne!(*a.origin_shift(), origin);
        }
        let p = a
            .pic
            .rational_points_bruteforce()
            .into_iter()
            .find(|p| *p != a.pic.base_point())
            .unwrap();
        let wrong = a.pic.add(a.origin_shift(), &a.pic.point_class(p));
        assert_ne!(a.pic.sub(a.origin_shift(), &wrong), origin);
        let packet = serde_json::json!({"schema":"ghs-prime-descent/v1","q":q,"points":[a.encode(None),a.encode(positive),a.encode(negative)]});
        let encoded = serde_json::to_vec(&packet).unwrap();
        assert_eq!(parse_request(&encoded).unwrap().points.len(), 3);
        let mut bad = packet.clone();
        bad["scalar"] = serde_json::json!(7);
        assert!(parse_request(&serde_json::to_vec(&bad).unwrap()).is_err());
        let mut bad = packet.clone();
        bad["schema"] = serde_json::json!("unknown");
        assert!(parse_request(&serde_json::to_vec(&bad).unwrap()).is_err());
        assert!(parse_request(&vec![b' '; MAX_INPUT_BYTES + 1]).is_err());
        println!(
            "G3_CONTROLS {}",
            serde_json::json!({"q":q,"fixture_mutants":20,"foreign_field_model_noncanonical_offcurve_reject":true,"two_x0_branches_distinguished":true,"origin_shift_nonzero":omitted_differs,"wrong_origin_valid_different":true,"bounded_cli_request":packet})
        );
    }
}
