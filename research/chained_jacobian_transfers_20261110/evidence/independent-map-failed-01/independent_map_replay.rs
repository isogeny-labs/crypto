//! Separate whole-map route:pointwise divisor pushforward and old Cantor
//! reduction after a quintic chart. No producer map/trace/Balanced imports.
use crate::legacy_jacobian as legacy;
use isogeny_algos::curve::{self, Curve, Isogeny, Pt, RatIsogeny};
use isogeny_algos::field::{Field, Rng};
use isogeny_algos::fpr::{FpR, ER};
use isogeny_algos::kernel::kohel::kohel;
use isogeny_algos::poly;
use std::collections::{HashMap, HashSet};

const MODELS: [[u64; 7]; 3] = [
    [31, 75, 74, 54, 71, 55, 1],
    [95, 26, 19, 27, 94, 91, 1],
    [95, 93, 43, 39, 92, 17, 1],
];
const CURVES: [[[u64; 2]; 2]; 3] = [
    [[1, 46], [44, 21]],
    [[26, 58], [69, 50]],
    [[59, 27], [23, 7]],
];
const MAP_NAMES: [&str; 6] = [
    "first-forward",
    "first-adjoint",
    "second-forward",
    "second-adjoint",
    "two-forward",
    "two-adjoint",
];
fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures/whole-map")
            .join(name),
    )
    .expect("frozen complete fixture required")
}
const ENDPOINTS: [(usize, usize); 6] = [(0, 1), (1, 0), (1, 2), (2, 1), (0, 2), (2, 0)];
type Input = (Vec<ER>, Vec<ER>, i64);
type RootCache = HashMap<Vec<ER>, Vec<ER>>;

struct Flat<'a>(&'a FpR);
impl legacy::Fld for Flat<'_> {
    type E = ER;
    fn zero(&self) -> ER {
        self.0.zero()
    }
    fn one(&self) -> ER {
        self.0.one()
    }
    fn add(&self, a: &ER, b: &ER) -> ER {
        self.0.add(*a, *b)
    }
    fn sub(&self, a: &ER, b: &ER) -> ER {
        self.0.sub(*a, *b)
    }
    fn neg(&self, a: &ER) -> ER {
        self.0.neg(*a)
    }
    fn mul(&self, a: &ER, b: &ER) -> ER {
        self.0.mul(*a, *b)
    }
    fn inv(&self, a: &ER) -> ER {
        assert_ne!(*a, self.0.zero());
        self.0.inv(*a)
    }
}
fn lift(f: &FpR, p: &[u64]) -> Vec<ER> {
    p.iter().map(|&c| f.from_u64(c)).collect()
}
fn elliptic(f: &FpR, model: usize, component: usize) -> Curve<ER> {
    let [a, b] = CURVES[model][component];
    Curve::new(f.from_u64(a), f.from_u64(b))
}

struct Cover {
    n: [u64; 5],
    q: [u64; 3],
    m: [u64; 4],
    infinity: [u64; 2],
}
const COVERS: [[Cover; 2]; 3] = [
    [
        Cover {
            n: [14, 0, 88, 97, 14],
            q: [63, 0, 1],
            m: [41, 40, 69, 73],
            infinity: [14, 73],
        },
        Cover {
            n: [5, 49, 31, 0, 87],
            q: [35, 0, 1],
            m: [87, 1, 82, 87],
            infinity: [87, 87],
        },
    ],
    [
        Cover {
            n: [8, 60, 36, 0, 84],
            q: [47, 0, 1],
            m: [98, 45, 84, 37],
            infinity: [84, 37],
        },
        Cover {
            n: [3, 0, 76, 42, 46],
            q: [70, 0, 1],
            m: [4, 76, 75, 41],
            infinity: [46, 41],
        },
    ],
    [
        Cover {
            n: [97, 93, 7, 0, 97],
            q: [55, 0, 1],
            m: [71, 49, 20, 63],
            infinity: [97, 63],
        },
        Cover {
            n: [74, 0, 81, 42, 64],
            q: [23, 0, 1],
            m: [37, 9, 9, 35],
            infinity: [64, 35],
        },
    ],
];

fn decode_classes(f: &FpR, bytes: &[u8], model: usize) -> Result<Vec<Input>, &'static str> {
    if bytes.len() != 18 + 5 * 12317
        || &bytes[..8] != b"CT27CLS1"
        || bytes[8] != 101
        || bytes[9..16] != MODELS[model].map(|x| u8::try_from(x).unwrap())
        || bytes[16..18] != 12317_u16.to_le_bytes()
    {
        return Err("class schema/model/count");
    }
    bytes[18..]
        .chunks_exact(5)
        .map(|r| {
            let d = usize::from(r[0] / 3);
            let n = r[0] % 3;
            if d > 2
                || usize::from(n) > 2 - d
                || r[1..].iter().any(|&x| x >= 101)
                || (d == 0 && r[1..] != [0, 0, 0, 0])
                || (d == 1 && (r[2] != 0 || r[4] != 0))
            {
                return Err("class row");
            }
            let mut u = match d {
                0 => vec![],
                1 => vec![f.from_u64(u64::from(r[1]))],
                2 => vec![f.from_u64(u64::from(r[1])), f.from_u64(u64::from(r[2]))],
                _ => unreachable!(),
            };
            u.push(f.one());
            let mut v = vec![f.from_u64(u64::from(r[3])), f.from_u64(u64::from(r[4]))];
            poly::trim(f, &mut v);
            Ok((u, v, i64::from(n)))
        })
        .collect()
}
fn decode_map(bytes: &[u8], case: usize) -> Result<Vec<usize>, &'static str> {
    let (source, target) = ENDPOINTS[case];
    if bytes.len() != 14 + 2 * 12317
        || &bytes[..8] != b"CT35MAP1"
        || bytes[8..12]
            != [
                101,
                u8::try_from(source).unwrap(),
                u8::try_from(target).unwrap(),
                u8::try_from(case).unwrap(),
            ]
        || bytes[12..14] != 12317_u16.to_le_bytes()
    {
        return Err("map schema/model/count");
    }
    bytes[14..]
        .chunks_exact(2)
        .map(|r| {
            let i = usize::from(u16::from_le_bytes([r[0], r[1]]));
            if i >= 12317 {
                Err("map index")
            } else {
                Ok(i)
            }
        })
        .collect()
}

struct Chart {
    a: ER,
    l: ER,
    quintic: Vec<ER>,
    q: legacy::Div<ER>,
}
impl Chart {
    fn new(f: &FpR, model: usize, avoid: &[Input]) -> Self {
        let sextic = lift(f, &MODELS[model]);
        assert_eq!(
            poly::gcd(f, &sextic, &poly::derivative(f, &sextic)),
            vec![f.one()]
        );
        let mut roots = poly::roots(f, &sextic, &mut Rng::new(0x35c417));
        roots.sort();
        assert_eq!(roots.len(), 6);
        let a = *roots
            .iter()
            .find(|&&a| {
                avoid
                    .iter()
                    .all(|(u, _, _)| poly::eval(f, u, a) != f.zero())
            })
            .expect("chart avoiding every support");
        let l = poly::eval(f, &poly::derivative(f, &sextic), a);
        assert_ne!(l, f.zero());
        let az1 = vec![f.one(), a];
        let mut power = vec![f.one()];
        let mut h = vec![];
        for (i, c) in sextic.iter().enumerate() {
            let mut term = vec![f.zero(); 6 - i];
            term.extend(poly::scale(f, &power, *c));
            h = poly::add(f, &h, &term);
            power = poly::mul(f, &power, &az1);
        }
        assert_eq!(h.len(), 6);
        assert_eq!(h[5], l);
        let quintic = h
            .iter()
            .enumerate()
            .map(|(i, c)| {
                f.mul(
                    *c,
                    if i <= 4 {
                        f.pow(l, (4 - i) as u128)
                    } else {
                        f.inv(l)
                    },
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(quintic.last(), Some(&f.one()));
        assert_eq!(
            poly::gcd(f, &quintic, &poly::derivative(f, &quintic)),
            vec![f.one()]
        );
        Self {
            a,
            l,
            quintic,
            q: legacy::Div {
                u: vec![f.zero(), f.one()],
                v: vec![f.sq(l)],
            },
        }
    }
    fn class(&self, f: &FpR, j: &legacy::Hyp<'_, Flat<'_>>, d: &Input) -> legacy::Div<ER> {
        let (u, v, n) = d;
        let degree = u.len() - 1;
        assert!(degree <= 4 && v.len() <= 4);
        let ua = poly::eval(f, u, self.a);
        assert_ne!(ua, f.zero(), "unhandled branch-chart support");
        let finite = if degree == 0 {
            j.identity()
        } else {
            let linear = vec![self.l, self.a];
            let mut power = vec![f.one()];
            let mut new_u = vec![];
            for (i, &c) in u.iter().enumerate() {
                let mut term = vec![f.zero(); degree - i];
                term.extend(poly::scale(f, &power, c));
                new_u = poly::add(f, &new_u, &term);
                power = poly::mul(f, &power, &linear);
            }
            new_u = poly::monic(f, &new_u);
            assert_eq!(new_u.len(), u.len());
            let mut new_v = vec![];
            power = vec![f.one()];
            for (i, &c) in v.iter().enumerate() {
                let mut term = vec![f.zero(); 3 - i];
                term.extend(poly::scale(f, &power, f.div(c, self.l)));
                new_v = poly::add(f, &new_v, &term);
                power = poly::mul(f, &power, &linear);
            }
            let raw = legacy::Div {
                u: new_u.clone(),
                v: poly::rem(f, &new_v, &new_u),
            };
            assert!(j.is_valid(&raw));
            j.reduce(raw)
        };
        let coefficient = i64::try_from(degree).unwrap() + 2 * n - 2;
        let correction = j.mul(&self.q, u128::from(coefficient.unsigned_abs()));
        let correction = if coefficient < 0 {
            j.neg(&correction)
        } else {
            correction
        };
        let out = j.add(&finite, &correction);
        assert!(j.is_valid(&out) && out.u.len() <= 3);
        out
    }
}

fn norms(f: &FpR, model: usize, d: &Input, cache: &mut RootCache) -> [Pt<ER>; 2] {
    let (u, v, n) = d;
    let degree = u.len() - 1;
    assert!(degree <= 2);
    let source = lift(f, &MODELS[model]);
    assert!(poly::rem(f, &poly::sub(f, &source, &poly::mul(f, v, v)), u).is_empty());
    let roots = cache.entry(u.clone()).or_insert_with(|| {
        let mut roots = poly::roots(f, u, &mut Rng::new(0x35d1));
        if degree == 2 && roots.len() == 1 {
            roots.push(roots[0]);
        }
        assert_eq!(
            roots.len(),
            degree,
            "all quadratic roots with multiplicity required"
        );
        let product = roots.iter().fold(vec![f.one()], |a, &x| {
            poly::mul(f, &a, &vec![f.neg(x), f.one()])
        });
        assert_eq!(&product, u);
        roots
    });
    std::array::from_fn(|i| {
        let cover = &COVERS[model][i];
        let e = elliptic(f, model, i);
        let nn = lift(f, &cover.n);
        let qq = lift(f, &cover.q);
        let mm = lift(f, &cover.m);
        let mut sum = Pt::Inf;
        for &x in roots.iter() {
            let y = poly::eval(f, v, x);
            assert_eq!(f.sq(y), poly::eval(f, &source, x));
            let qx = poly::eval(f, &qq, x);
            let p = if qx == f.zero() {
                assert_ne!(poly::eval(f, &nn, x), f.zero());
                assert_ne!(poly::eval(f, &mm, x), f.zero());
                Pt::Inf
            } else {
                Pt::Aff(
                    f.div(poly::eval(f, &nn, x), f.sq(qx)),
                    f.div(f.mul(y, poly::eval(f, &mm, x)), f.pow(qx, 3)),
                )
            };
            assert!(curve::on_curve(f, &e, &p));
            sum = curve::padd(f, &e, &sum, &p);
        }
        let coefficient = i64::try_from(degree).unwrap() + 2 * n - 2;
        let q = Pt::Aff(f.from_u64(cover.infinity[0]), f.from_u64(cover.infinity[1]));
        assert!(curve::on_curve(f, &e, &q));
        let correction = curve::pmul(f, &e, &q, u128::from(coefficient.unsigned_abs()));
        curve::padd(
            f,
            &e,
            &sum,
            &if coefficient < 0 {
                curve::neg(f, &correction)
            } else {
                correction
            },
        )
    })
}

fn pull(
    f: &FpR,
    model: usize,
    component: usize,
    p: &Pt<ER>,
    chart: &Chart,
    j: &legacy::Hyp<'_, Flat<'_>>,
) -> legacy::Div<ER> {
    assert!(curve::on_curve(f, &elliptic(f, model, component), p));
    let Pt::Aff(x, y) = *p else {
        return j.identity();
    };
    assert_ne!(y, f.zero());
    let c = &COVERS[model][component];
    let n = lift(f, &c.n);
    let q = lift(f, &c.q);
    let m = lift(f, &c.m);
    let u = poly::monic(
        f,
        &poly::sub(f, &n, &poly::scale(f, &poly::mul(f, &q, &q), x)),
    );
    let degree = u.len() - 1;
    assert!((2..=4).contains(&degree));
    let qi = poly::invmod(f, &q, &u).expect("Q unit on finite nonorigin fibre");
    let mi = poly::invmod(f, &m, &u).expect("M unit including repeats");
    assert_eq!(poly::rem(f, &poly::mul(f, &q, &qi), &u), vec![f.one()]);
    assert_eq!(poly::rem(f, &poly::mul(f, &m, &mi), &u), vec![f.one()]);
    let q3 = poly::mul(f, &poly::mul(f, &q, &q), &q);
    let v = poly::rem(f, &poly::scale(f, &poly::mul(f, &q3, &mi), y), &u);
    assert!(poly::rem(
        f,
        &poly::sub(f, &poly::mul(f, &v, &v), &lift(f, &MODELS[model])),
        &u
    )
    .is_empty());
    let infinity = Pt::Aff(f.from_u64(c.infinity[0]), f.from_u64(c.infinity[1]));
    let drop = 4 - i64::try_from(degree).unwrap();
    let plus = if *p == infinity { drop - 1 } else { -1 };
    if degree < 4 {
        assert!(*p == infinity || *p == curve::neg(f, &elliptic(f, model, component), &infinity));
    }
    chart.class(f, j, &(u, v, plus))
}

struct Stage {
    maps: [RatIsogeny<FpR>; 2],
    reverse: bool,
    target: usize,
}
fn stages(f: &FpR, case: usize) -> Vec<Stage> {
    let steps: &[(usize, bool)] = match case {
        0 => &[(0, false)],
        1 => &[(0, true)],
        2 => &[(1, false)],
        3 => &[(1, true)],
        4 => &[(0, false), (1, false)],
        5 => &[(1, true), (0, true)],
        _ => unreachable!(),
    };
    steps
        .iter()
        .map(|&(hop, reverse)| {
            let source = if reverse { hop + 1 } else { hop };
            let target = if reverse { hop } else { hop + 1 };
            let kernels = match (hop, reverse) {
                (0, false) => [[72, 100, 35, 1], [59, 46, 8, 1]],
                (1, false) => [[99, 94, 75, 1], [53, 19, 40, 1]],
                (0, true) => [[58, 26, 58, 1], [29, 88, 45, 1]],
                (1, true) => [[13, 62, 81, 1], [11, 64, 23, 1]],
                _ => unreachable!(),
            };
            let maps = std::array::from_fn(|i| {
                kohel(f, &elliptic(f, source, i), &lift(f, &kernels[i]), 7)
            });
            for (i, map) in maps.iter().enumerate() {
                let e = elliptic(f, target, i);
                let scale = f.from_u64(if reverse { 29 } else { 1 });
                assert_eq!(f.mul(f.pow(scale, 4), map.cod.a), e.a);
                assert_eq!(f.mul(f.pow(scale, 6), map.cod.b), e.b);
            }
            Stage {
                maps,
                reverse,
                target,
            }
        })
        .collect()
}
fn apply(
    f: &FpR,
    model: usize,
    base: [Pt<ER>; 2],
    steps: &[Stage],
    quarter: [u128; 2],
) -> [Pt<ER>; 2] {
    std::array::from_fn(|i| {
        let mut p = curve::pmul(f, &elliptic(f, model, i), &base[i], quarter[i]);
        for stage in steps {
            p = stage.maps[i].eval(f, &p);
            if stage.reverse {
                p = match p {
                    Pt::Inf => Pt::Inf,
                    Pt::Aff(x, y) => Pt::Aff(
                        f.mul(f.sq(f.from_u64(29)), x),
                        f.mul(f.pow(f.from_u64(29), 3), y),
                    ),
                };
            }
            assert!(curve::on_curve(f, &elliptic(f, stage.target, i), &p));
        }
        p
    })
}
fn rational_lift(from: &FpR, to: &FpR, p: &Pt<ER>) -> Pt<ER> {
    match p {
        Pt::Inf => Pt::Inf,
        Pt::Aff(x, y) => {
            assert_eq!(from.frobenius(*x), *x);
            assert_eq!(from.frobenius(*y), *y);
            assert!(x[1..].iter().chain(&y[1..]).all(|&c| c == 0));
            Pt::Aff(to.from_u64(x[0]), to.from_u64(y[0]))
        }
    }
}
fn sum_pulls(
    f: &FpR,
    model: usize,
    points: [Pt<ER>; 2],
    chart: &Chart,
    j: &legacy::Hyp<'_, Flat<'_>>,
) -> legacy::Div<ER> {
    j.add(
        &pull(f, model, 0, &points[0], chart, j),
        &pull(f, model, 1, &points[1], chart, j),
    )
}

fn replay_rational(case: usize) {
    let (source, target) = ENDPOINTS[case];
    let f = FpR::from_modulus(101, &[98, 0, 1]);
    let chart_field = FpR::from_modulus(101, &[33, 46, 2, 18, 56, 12, 1]);
    let source_bytes = fixture(&format!("t{source}-classes.bin"));
    let target_bytes = fixture(&format!("t{target}-classes.bin"));
    let map_bytes = fixture(&format!("{}.bin", MAP_NAMES[case]));
    let inputs = decode_classes(&f, &source_bytes, source).unwrap();
    let outputs = decode_classes(&chart_field, &target_bytes, target).unwrap();
    let indices = decode_map(&map_bytes, case).unwrap();
    assert!(decode_classes(&f, &source_bytes, (source + 1) % 3).is_err());
    let mut bad = map_bytes.clone();
    bad[11] ^= 1;
    assert!(decode_map(&bad, case).is_err());
    bad = map_bytes;
    bad[14..16].copy_from_slice(&12317_u16.to_le_bytes());
    assert!(decode_map(&bad, case).is_err());
    let chart = Chart::new(&chart_field, target, &outputs);
    let flat = Flat(&chart_field);
    let j = legacy::Hyp {
        f: &flat,
        h: chart.quintic.clone(),
        g: 2,
    };
    assert!(j.is_valid(&chart.q));
    let expected = outputs
        .iter()
        .map(|d| chart.class(&chart_field, &j, d))
        .collect::<Vec<_>>();
    assert_eq!(
        expected
            .iter()
            .map(|d| (d.u.clone(), d.v.clone()))
            .collect::<HashSet<_>>()
            .len(),
        12317
    );
    assert_eq!(indices.iter().collect::<HashSet<_>>().len(), 12317);
    let stages = stages(&f, case);
    let mut cache = RootCache::new();
    let mut wrong = 0;
    for (i, d) in inputs.iter().enumerate() {
        let base = norms(&f, source, d, &mut cache);
        let mapped = apply(&f, source, base, &stages, [82, 85]);
        let mapped = mapped.map(|p| rational_lift(&f, &chart_field, &p));
        let result = sum_pulls(&chart_field, target, mapped, &chart, &j);
        assert!(j.is_valid(&result));
        assert_eq!(
            (&result.u, &result.v),
            (&expected[indices[i]].u, &expected[indices[i]].v),
            "case{case} class{i}"
        );
        let mutated =
            apply(&f, source, base, &stages, [1, 1]).map(|p| rational_lift(&f, &chart_field, &p));
        let mutated = sum_pulls(&chart_field, target, mutated, &chart, &j);
        let four = j.mul(&result, 4);
        assert_eq!((&mutated.u, &mutated.v), (&four.u, &four.v));
        if mutated.u != result.u || mutated.v != result.v {
            wrong += 1;
        }
    }
    assert_eq!(wrong, 12316);
    println!("PASS independent whole-map case{case} T{source}→T{target}:all12317 old outputs in separate genericCantor chart;all target valid/distinct;pointwise split-support pushforwards/multiplicities/poles/infinities and Euclid finite-fibre pullbacks bypass producer trace/Balanced;wrongquarter differs12316 and equals4correct;schemas/models/index mutations reject. Same existing elliptic/Kohel engine explicitly reused,no timing claim");
}

fn geometric_kernel(second: bool) {
    let f = FpR::new(101, 12);
    let flat = Flat(&f);
    let model = usize::from(second);
    let target = model + 1;
    let mut roots = poly::roots(
        &f,
        &lift(&f, &[33, 46, 2, 18, 56, 12, 1]),
        &mut Rng::new(0x35e6),
    );
    roots.sort();
    assert_eq!(roots.len(), 6);
    let gamma = roots[0];
    let s = f.div(
        poly::eval(&f, &lift(&f, &[68, 19, 6, 1]), gamma),
        poly::eval(&f, &lift(&f, &[15, 12, 3]), gamma),
    );
    let t = f.sub(gamma, s);
    assert_eq!(f.sq(s), f.from_u64(2));
    assert_eq!(poly::eval(&f, &lift(&f, &[56, 13, 6, 1]), t), f.zero());
    let basis = [f.one(), t, f.sq(t), s, f.mul(s, t), f.mul(s, f.sq(t))];
    let witness = if second {
        include_str!("../fixtures/explicit-psi2-kernel-12.tsv")
    } else {
        include_str!("../fixtures/explicit-psi-kernel-07.tsv")
    };
    let mut inputs = vec![];
    for (i, line) in witness.lines().skip(1).enumerate() {
        let row = line.split('\t').collect::<Vec<_>>();
        let offset = usize::from(!second);
        assert_eq!(row.len(), 4 + offset);
        if !second {
            assert_eq!(row[0], "actualPsiKernel");
        }
        assert_eq!(row[offset].parse::<usize>().unwrap(), i);
        inputs.push((
            crate::kernel_replay::parse_polynomial(&f, row[1 + offset], &basis),
            crate::kernel_replay::parse_polynomial(&f, row[2 + offset], &basis),
            row[3 + offset].parse::<i64>().unwrap(),
        ));
    }
    assert_eq!(inputs.len(), 49);
    let source_chart = Chart::new(&f, model, &inputs);
    let source_j = legacy::Hyp {
        f: &flat,
        h: source_chart.quintic.clone(),
        g: 2,
    };
    let target_chart = Chart::new(&f, target, &[]);
    let target_j = legacy::Hyp {
        f: &flat,
        h: target_chart.quintic.clone(),
        g: 2,
    };
    let steps = stages(&f, if second { 2 } else { 0 });
    let mut cache = RootCache::new();
    let mut seen = HashSet::new();
    let mut rejected = 0;
    for (i, d) in inputs.iter().enumerate() {
        let input = source_chart.class(&f, &source_j, d);
        assert!(seen.insert((input.u.clone(), input.v.clone())));
        assert!(source_j.is_identity(&source_j.mul(&input, 7)));
        let base = norms(&f, model, d, &mut cache);
        let image = sum_pulls(
            &f,
            target,
            apply(&f, model, base, &steps, [2, 2]),
            &target_chart,
            &target_j,
        );
        assert!(target_j.is_identity(&image), "kernel{model} class{i}");
        let omitted = sum_pulls(
            &f,
            model,
            apply(&f, model, base, &[], [2, 2]),
            &source_chart,
            &source_j,
        );
        assert_eq!((&omitted.u, &omitted.v), (&input.u, &input.v));
        if !source_j.is_identity(&omitted) {
            rejected += 1;
        }
    }
    assert_eq!(seen.len(), 49);
    assert_eq!(rejected, 48);
    println!("PASS independent actual Psi{} kernel:all49 valid/distinct/7-annihilated in existingCantor overFp12,checked Fp6 tower embedding,pointwise cover norm/elliptic7/finite-fibre pullback kills49;omitting7 returns each input and rejects48 nonidentity. Modulus{:?}. Geometric degree49/completeness remains separate written dependency,no timing claim",model+1,f.modulus());
}

#[test]
fn independent_first_forward() {
    replay_rational(0)
}
#[test]
fn independent_first_adjoint() {
    replay_rational(1)
}
#[test]
fn independent_second_forward() {
    replay_rational(2)
}
#[test]
fn independent_second_adjoint() {
    replay_rational(3)
}
#[test]
fn independent_compressed_forward() {
    replay_rational(4)
}
#[test]
fn independent_compressed_adjoint() {
    replay_rational(5)
}
#[test]
fn independent_first_geometric_kernel() {
    geometric_kernel(false)
}
#[test]
fn independent_second_geometric_kernel() {
    geometric_kernel(true)
}
