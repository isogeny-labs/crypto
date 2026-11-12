//! Proper normalized Richelot divisor push-pull on the actual C20/T20.
//! Old field/poly/Cantor engines and the phaseA normalized data only.
use super::*;
#[path = "t20_direct_norms.rs"]
mod t20_direct_norms;

struct RawContext<'a> {
    coefficients: Vec<u64>,
    chart: Chart,
    jac: legacy::Hyp<'a, Flat<'a>>,
    roots: RootCache,
}
impl<'a> RawContext<'a> {
    fn new(f: &'a FpR, flat: &'a Flat<'a>, coefficients: Vec<u64>) -> Self {
        let base = Zp::new(101);
        let rho = base
            .sqrt(coefficients[6])
            .map(|x| f.from_u64(x))
            .unwrap_or_else(|| f.sqrt(f.from_u64(coefficients[6])).unwrap());
        let chart = Chart::from_sextic(f, &coefficients, rho, &[]);
        let jac = legacy::Hyp {
            f: flat,
            h: chart.quintic.clone(),
            g: 2,
        };
        Self {
            coefficients,
            chart,
            jac,
            roots: RootCache::new(),
        }
    }
    fn point_class(&self, f: &FpR, point: EvenPoint) -> legacy::Div<ER> {
        original_point_class(f, &self.coefficients, &self.chart, &self.jac, point)
    }
    fn tag(&self, d: &legacy::Div<ER>) -> TaggedClass {
        TaggedClass {
            coefficients: self.coefficients.clone(),
            base: self.chart.a,
            value: d.clone(),
        }
    }
    fn frobenius(&mut self, f: &FpR, d: &legacy::Div<ER>) -> legacy::Div<ER> {
        let points = original_support(
            f,
            &self.coefficients,
            &self.chart,
            &self.jac,
            d,
            &mut self.roots,
        );
        let mut out = self.jac.identity();
        for &point in &points {
            let image = match point {
                EvenPoint::Finite(x, y) => EvenPoint::Finite(f.frobenius(x), f.frobenius(y)),
                EvenPoint::Infinity(rho) => EvenPoint::Infinity(f.frobenius(rho)),
            };
            out = self.jac.add(&out, &self.point_class(f, image));
        }
        let changed_base =
            self.point_class(f, EvenPoint::Finite(f.frobenius(self.chart.a), f.zero()));
        self.jac.add(
            &out,
            &self
                .jac
                .neg(&self.jac.mul(&changed_base, points.len() as u128)),
        )
    }
    fn raw_fibre(&self, f: &FpR, u: &[ER], v: &[ER], stats: &mut FibreStats) -> legacy::Div<ER> {
        assert_eq!(u.len(), 3, "two finite points with multiplicity");
        let u = poly::monic(f, &u.to_vec());
        let v = poly::rem(f, &v.to_vec(), &u);
        assert!(poly::rem(
            f,
            &poly::sub(f, &poly::mul(f, &v, &v), &lift(f, &self.coefficients)),
            &u
        )
        .is_empty());
        if poly::gcd(f, &u, &poly::derivative(f, &u)).len() > 1 {
            stats.repeated += 1;
        }
        if poly::eval(f, &u, self.chart.a) == f.zero() {
            stats.target_corner += 1;
            assert_eq!(poly::eval(f, &v, self.chart.a), f.zero());
            let (remaining, rem) = poly::divrem(f, &u, &vec![f.neg(self.chart.a), f.one()]);
            assert!(rem.is_empty() && remaining.len() == 2);
            let x = f.neg(f.div(remaining[0], remaining[1]));
            assert_ne!(x, self.chart.a, "no duplicated ramified chart support");
            self.point_class(f, EvenPoint::Finite(x, poly::eval(f, &v, x)))
        } else {
            self.chart.class(f, &self.jac, &(u, v, 0))
        }
    }
}

struct TaggedClass {
    coefficients: Vec<u64>,
    base: ER,
    value: legacy::Div<ER>,
}

#[derive(Default, Debug)]
struct FibreStats {
    finite: usize,
    repeated: usize,
    source_infinity: usize,
    source_branch: [usize; 3],
    degree_drop: usize,
    target_corner: usize,
}

fn point_fibre(
    f: &FpR,
    source: &RawContext<'_>,
    target: &RawContext<'_>,
    factors: &[Vec<ER>; 3],
    normalized: &[Vec<ER>; 3],
    point: EvenPoint,
    stats: &mut FibreStats,
) -> legacy::Div<ER> {
    match point {
        EvenPoint::Infinity(rho) => {
            stats.source_infinity += 1;
            assert_eq!(f.sq(rho), f.from_u64(source.coefficients[6]));
            let lead1 = factors[0][2];
            let lead2 = factors[1][2];
            let u = poly::add(
                f,
                &poly::scale(f, &normalized[0], lead1),
                &poly::scale(f, &normalized[1], lead2),
            );
            assert_eq!(u.len(), 3, "fixture infinity fibre has two finite points");
            let v = poly::scale(f, &normalized[0], f.div(lead1, rho));
            target.raw_fibre(f, &u, &v, stats)
        }
        EvenPoint::Finite(x, y) => {
            assert_eq!(f.sq(y), poly::eval(f, &lift(f, &source.coefficients), x));
            let values = factors.each_ref().map(|p| poly::eval(f, p, x));
            if y == f.zero() {
                let branch = values.iter().position(|&z| z == f.zero()).unwrap();
                stats.source_branch[branch] += 1;
                return match branch {
                    0 => target.raw_fibre(f, &normalized[1], &[], stats),
                    1 => target.raw_fibre(f, &normalized[0], &[], stats),
                    2 => target.jac.identity(),
                    _ => unreachable!(),
                };
            }
            stats.finite += 1;
            let u = poly::add(
                f,
                &poly::scale(f, &normalized[0], values[0]),
                &poly::scale(f, &normalized[1], values[1]),
            );
            let ordinate = poly::mul(
                f,
                &poly::scale(f, &normalized[0], f.div(values[0], y)),
                &vec![x, f.neg(f.one())],
            );
            match u.len() {
                3 => target.raw_fibre(f, &u, &ordinate, stats),
                2 => {
                    stats.degree_drop += 1;
                    let z = f.neg(f.div(u[0], u[1]));
                    let image_y = poly::eval(f, &ordinate, z);
                    if z == target.chart.a {
                        stats.target_corner += 1;
                    }
                    let finite = target.point_class(f, EvenPoint::Finite(z, image_y));
                    let rho = f.neg(f.div(f.mul(values[0], normalized[0][2]), y));
                    assert_eq!(f.sq(rho), f.from_u64(target.coefficients[6]));
                    let infinity = target.point_class(f, EvenPoint::Infinity(rho));
                    target.jac.add(&finite, &infinity)
                }
                _ => panic!("unsupported constant/zero fibre; no points may be dropped"),
            }
        }
    }
}

struct RichelotMaps<'a> {
    source: RawContext<'a>,
    target: RawContext<'a>,
    forward_factors: [Vec<ER>; 3],
    forward_normalized: [Vec<ER>; 3],
    reverse_factors: [Vec<ER>; 3],
    reverse_normalized: [Vec<ER>; 3],
    forward_base: legacy::Div<ER>,
    reverse_base: legacy::Div<ER>,
    forward_cache: HashMap<EvenClassKey, legacy::Div<ER>>,
    reverse_cache: HashMap<EvenClassKey, legacy::Div<ER>>,
    forward_stats: FibreStats,
    reverse_stats: FibreStats,
}
impl<'a> RichelotMaps<'a> {
    fn new(f: &'a FpR, flat: &'a Flat<'a>) -> Self {
        let T20Geometry {
            source,
            target,
            factors,
            normalized,
            ..
        } = t20_geometry(f);
        let source = RawContext::new(f, flat, source.coefficients);
        let target = RawContext::new(f, flat, target);
        let mut forward_factors = factors;
        let mut forward_normalized = normalized;
        let shift = forward_factors
            .iter()
            .position(|p| poly::eval(f, p, source.chart.a) == f.zero())
            .unwrap();
        forward_factors.rotate_left(shift);
        forward_normalized.rotate_left(shift);
        let mut reverse_factors = forward_normalized.clone();
        let mut reverse_normalized = forward_factors.clone();
        let shift = reverse_factors
            .iter()
            .position(|p| poly::eval(f, p, target.chart.a) == f.zero())
            .unwrap();
        reverse_factors.rotate_left(shift);
        reverse_normalized.rotate_left(shift);
        let mut forward_stats = FibreStats::default();
        let mut reverse_stats = FibreStats::default();
        let forward_base = point_fibre(
            f,
            &source,
            &target,
            &forward_factors,
            &forward_normalized,
            EvenPoint::Finite(source.chart.a, f.zero()),
            &mut forward_stats,
        );
        let reverse_base = point_fibre(
            f,
            &target,
            &source,
            &reverse_factors,
            &reverse_normalized,
            EvenPoint::Finite(target.chart.a, f.zero()),
            &mut reverse_stats,
        );
        assert_ne!(forward_base, target.jac.identity());
        assert_ne!(reverse_base, source.jac.identity());
        Self {
            source,
            target,
            forward_factors,
            forward_normalized,
            reverse_factors,
            reverse_normalized,
            forward_base,
            reverse_base,
            forward_cache: HashMap::new(),
            reverse_cache: HashMap::new(),
            forward_stats,
            reverse_stats,
        }
    }
    fn evaluate(
        &mut self,
        f: &FpR,
        input: &TaggedClass,
        reverse: bool,
        omit_base: bool,
    ) -> Result<legacy::Div<ER>, &'static str> {
        let (source, target, factors, normalized, base, cache, stats) = if reverse {
            (
                &mut self.target,
                &self.source,
                &self.reverse_factors,
                &self.reverse_normalized,
                &self.reverse_base,
                &mut self.reverse_cache,
                &mut self.reverse_stats,
            )
        } else {
            (
                &mut self.source,
                &self.target,
                &self.forward_factors,
                &self.forward_normalized,
                &self.forward_base,
                &mut self.forward_cache,
                &mut self.forward_stats,
            )
        };
        if input.coefficients != source.coefficients || input.base != source.chart.a {
            return Err("source curve/chart tag mismatch");
        }
        if !source.jac.is_valid(&input.value) {
            return Err("invalid source Mumford divisor");
        }
        let key = class_key(&input.value);
        if !omit_base {
            if let Some(d) = cache.get(&key) {
                return Ok(d.clone());
            }
        }
        let points = original_support(
            f,
            &source.coefficients,
            &source.chart,
            &source.jac,
            &input.value,
            &mut source.roots,
        );
        let mut out = target.jac.identity();
        for &point in &points {
            let fibre = point_fibre(f, source, target, factors, normalized, point, stats);
            out = target.jac.add(&out, &fibre);
        }
        if !omit_base {
            out = target.jac.add(
                &out,
                &target.jac.neg(&target.jac.mul(base, points.len() as u128)),
            );
        }
        if reverse {
            out = target.jac.neg(&out);
        }
        assert!(target.jac.is_valid(&out));
        if !omit_base {
            cache.insert(key, out.clone());
        }
        Ok(out)
    }
    fn forward(&mut self, f: &FpR, input: &TaggedClass) -> Result<legacy::Div<ER>, &'static str> {
        self.evaluate(f, input, false, false)
    }
    fn adjoint(&mut self, f: &FpR, input: &TaggedClass) -> Result<legacy::Div<ER>, &'static str> {
        self.evaluate(f, input, true, false)
    }
}

fn curve_points(f: &FpR, coefficients: &[u64]) -> Vec<EvenPoint> {
    let base = Zp::new(101);
    let mut points = vec![];
    for x in 0..101 {
        let value = poly::eval(&base, &coefficients.to_vec(), x);
        for y in 0..101 {
            if base.sq(y) == value {
                points.push(EvenPoint::Finite(f.from_u64(x), f.from_u64(y)));
            }
        }
    }
    let rho = base
        .sqrt(coefficients[6])
        .map(|x| f.from_u64(x))
        .unwrap_or_else(|| f.sqrt(f.from_u64(coefficients[6])).unwrap());
    points.extend([EvenPoint::Infinity(rho), EvenPoint::Infinity(f.neg(rho))]);
    points
}

fn geometric_two(f: &FpR, context: &RawContext<'_>) -> Vec<legacy::Div<ER>> {
    let mut roots = poly::roots(f, &lift(f, &context.coefficients), &mut Rng::new(0x2046));
    roots.sort();
    assert_eq!(roots.len(), 6);
    let points = roots
        .iter()
        .map(|&x| context.point_class(f, EvenPoint::Finite(x, f.zero())))
        .collect::<Vec<_>>();
    let mut classes = vec![context.jac.identity()];
    for i in 0..6 {
        for j in i + 1..6 {
            let d = context.jac.add(&points[i], &points[j]);
            assert_ne!(d, context.jac.identity());
            assert!(!classes.contains(&d));
            assert_eq!(context.jac.mul(&d, 2), context.jac.identity());
            classes.push(d);
        }
    }
    assert_eq!(classes.len(), 16);
    for a in &classes {
        for b in &classes {
            assert!(classes.contains(&context.jac.add(a, b)));
        }
    }
    classes
}

fn check_geometric_class(
    f: &FpR,
    maps: &mut RichelotMaps<'_>,
    d: &legacy::Div<ER>,
    reverse: bool,
) -> legacy::Div<ER> {
    let input = if reverse {
        maps.target.tag(d)
    } else {
        maps.source.tag(d)
    };
    let image = maps.evaluate(f, &input, reverse, false).unwrap();
    let other_input = if reverse {
        maps.source.tag(&image)
    } else {
        maps.target.tag(&image)
    };
    let back = maps.evaluate(f, &other_input, !reverse, false).unwrap();
    let twice = if reverse {
        maps.target.jac.mul(d, 2)
    } else {
        maps.source.jac.mul(d, 2)
    };
    assert_eq!(back, twice);
    let pi_source = if reverse {
        maps.target.frobenius(f, d)
    } else {
        maps.source.frobenius(f, d)
    };
    let pi_image = if reverse {
        maps.source.frobenius(f, &image)
    } else {
        maps.target.frobenius(f, &image)
    };
    let input = if reverse {
        maps.target.tag(&pi_source)
    } else {
        maps.source.tag(&pi_source)
    };
    assert_eq!(maps.evaluate(f, &input, reverse, false).unwrap(), pi_image);
    image
}

#[test]
fn proper_t20_fibres_geometric_controls() {
    let f = FpR::new(101, 12);
    let flat = Flat(&f);
    let mut maps = RichelotMaps::new(&f, &flat);
    let mut base_witnesses = [0, 0];
    let mut forced_counts = [0, 0];
    let mut fixed_counts = [0, 0];
    let mut repeated_inputs = [0, 0];
    for reverse in [false, true] {
        let direction = usize::from(reverse);
        let coefficients = if reverse {
            maps.target.coefficients.clone()
        } else {
            maps.source.coefficients.clone()
        };
        let points = curve_points(&f, &coefficients);
        assert_eq!(points.len(), [122, 120][direction]);
        for point in points {
            let d = if reverse {
                maps.target.point_class(&f, point)
            } else {
                maps.source.point_class(&f, point)
            };
            assert_eq!(d.u.len(), 2, "degree-one omitted-basepoint control");
            let image = check_geometric_class(&f, &mut maps, &d, reverse);
            let doubled = if reverse {
                maps.target.jac.mul(&d, 2)
            } else {
                maps.source.jac.mul(&d, 2)
            };
            assert_eq!(doubled.u.len(), 3);
            assert!(poly::gcd(&f, &doubled.u, &poly::derivative(&f, &doubled.u)).len() > 1);
            let doubled_image = check_geometric_class(&f, &mut maps, &doubled, reverse);
            let twice_image = if reverse {
                maps.source.jac.mul(&image, 2)
            } else {
                maps.target.jac.mul(&image, 2)
            };
            assert_eq!(doubled_image, twice_image);
            repeated_inputs[direction] += 1;
            let input = if reverse {
                maps.target.tag(&d)
            } else {
                maps.source.tag(&d)
            };
            let omitted = maps.evaluate(&f, &input, reverse, true).unwrap();
            assert_ne!(image, omitted);
            base_witnesses[direction] += 1;
        }
        let roots = poly::roots(&f, &lift(&f, &coefficients), &mut Rng::new(0x2047));
        assert_eq!(roots.len(), 6);
        for x in roots {
            let p = EvenPoint::Finite(x, f.zero());
            let d = if reverse {
                maps.target.point_class(&f, p)
            } else {
                maps.source.point_class(&f, p)
            };
            check_geometric_class(&f, &mut maps, &d, reverse);
        }
        let (factors, normalized) = if reverse {
            (
                maps.reverse_factors.clone(),
                maps.reverse_normalized.clone(),
            )
        } else {
            (
                maps.forward_factors.clone(),
                maps.forward_normalized.clone(),
            )
        };
        let leading = poly::add(
            &f,
            &poly::scale(&f, &factors[0], normalized[0][2]),
            &poly::scale(&f, &factors[1], normalized[1][2]),
        );
        assert!((2..=3).contains(&leading.len()));
        let roots = poly::roots(&f, &leading, &mut Rng::new(0x2048));
        assert!(!roots.is_empty());
        for x in roots {
            let y = f.sqrt(poly::eval(&f, &lift(&f, &coefficients), x)).unwrap();
            assert_ne!(y, f.zero());
            for sign in [y, f.neg(y)] {
                let p = EvenPoint::Finite(x, sign);
                let d = if reverse {
                    maps.target.point_class(&f, p)
                } else {
                    maps.source.point_class(&f, p)
                };
                let image = check_geometric_class(&f, &mut maps, &d, reverse);
                for &c in &image.u {
                    let mut pi = c;
                    for _ in 0..6 {
                        pi = f.frobenius(pi);
                    }
                    assert_eq!(
                        pi, c,
                        "planned degree-drop image support polynomial is in F101^6"
                    );
                }
                let input = if reverse {
                    maps.target.tag(&d)
                } else {
                    maps.source.tag(&d)
                };
                maps.evaluate(&f, &input, reverse, true).unwrap();
                forced_counts[direction] += 1;
            }
        }
        let classes = if reverse {
            geometric_two(&f, &maps.target)
        } else {
            geometric_two(&f, &maps.source)
        };
        let mut images = HashSet::new();
        let mut image_rows = vec![];
        let mut killed = vec![];
        for d in &classes {
            let image = check_geometric_class(&f, &mut maps, d, reverse);
            let zero = if reverse {
                maps.source.jac.identity()
            } else {
                maps.target.jac.identity()
            };
            if image == zero {
                killed.push(class_key(d));
            }
            images.insert(class_key(&image));
            image_rows.push(image);
            let mut pi = d.clone();
            for _ in 0..3 {
                pi = if reverse {
                    maps.target.frobenius(&f, &pi)
                } else {
                    maps.source.frobenius(&f, &pi)
                };
            }
            if pi == *d {
                fixed_counts[direction] += 1;
            }
        }
        assert_eq!((images.len(), killed.len()), (4, 4));
        for (i, a) in classes.iter().enumerate() {
            for (j, b) in classes.iter().enumerate() {
                let sum = if reverse {
                    maps.target.jac.add(a, b)
                } else {
                    maps.source.jac.add(a, b)
                };
                let tag = if reverse {
                    maps.target.tag(&sum)
                } else {
                    maps.source.tag(&sum)
                };
                let actual = maps.evaluate(&f, &tag, reverse, false).unwrap();
                let expected = if reverse {
                    maps.source.jac.add(&image_rows[i], &image_rows[j])
                } else {
                    maps.target.jac.add(&image_rows[i], &image_rows[j])
                };
                assert_eq!(actual, expected);
            }
        }
        let expected = if reverse {
            let mut expected = vec![class_key(&maps.target.jac.identity())];
            for g in &maps.forward_normalized {
                expected.push(class_key(&maps.target.raw_fibre(
                    &f,
                    g,
                    &[],
                    &mut FibreStats::default(),
                )));
            }
            expected.into_iter().collect::<HashSet<_>>()
        } else {
            t20_geometry(&f)
                .kernel
                .iter()
                .map(class_key)
                .collect::<HashSet<_>>()
        };
        assert_eq!(killed.into_iter().collect::<HashSet<_>>(), expected);
    }
    assert_eq!(base_witnesses, [122, 120]);
    assert_eq!(repeated_inputs, [122, 120]);
    assert_eq!(fixed_counts, [16, 4]);
    assert!(forced_counts.iter().all(|&n| n > 0));
    assert!(maps.forward_stats.degree_drop > 0 && maps.reverse_stats.degree_drop > 0);
    assert!(maps.forward_stats.source_infinity >= 2 && maps.reverse_stats.source_infinity >= 2);
    assert!(maps.forward_stats.source_branch.iter().all(|&n| n > 0));
    assert!(maps.reverse_stats.source_branch.iter().all(|&n| n > 0));
    let zero = maps.source.jac.identity();
    let mut wrong = maps.source.tag(&zero);
    wrong.coefficients[0] = (wrong.coefficients[0] + 1) % 101;
    assert!(maps.forward(&f, &wrong).is_err());
    let mut wrong = maps.source.tag(&zero);
    wrong.base = f.add(wrong.base, f.one());
    assert!(maps.forward(&f, &wrong).is_err());
    let mut wrong = maps.source.tag(&zero);
    wrong.value.u = vec![f.zero()];
    assert!(maps.forward(&f, &wrong).is_err());
    assert!(maps
        .forward(&f, &maps.target.tag(&maps.target.jac.identity()))
        .is_err());
    println!("PASS T20properpoints:source122/target120actualcurvepointclasses+all6branchpoints+bothsignedinfinities/Frobenius/positivebothdual2;omittedbasepointmutants{base_witnesses:?};explicitrepeatedMumfordinputdoubles{repeated_inputs:?};forceddegree-dropinputs{forced_counts:?};all16actualJ2each/all256groupsums+mapadditions/kernel4image4/Frob3fixed{fixed_counts:?}/actualkernels;wrongcurve/chart/malformedreject. Forwardstats{:?};reversestats{:?}. PlannedFp6support/Fp12points only,no arbitraryFp12Jacdomain/costclaim",maps.forward_stats,maps.reverse_stats);
}

#[test]
fn whole_t20_rational_correspondence_and_unbalanced_chain() {
    let f = FpR::new(101, 12);
    let base = Zp::new(101);
    let flat = Flat(&f);
    let mut maps = RichelotMaps::new(&f, &flat);
    let c20_model = models(&f).into_iter().nth(2).unwrap().1;
    let mut c20 = WholeContext::new(&f, &flat, c20_model);
    assert_eq!(c20.model.coefficients, maps.source.coefficients);
    assert_eq!(c20.chart.a, maps.source.chart.a);
    let e_points = rational_points(&base, &c20.model.factors[0]);
    let f_points = rational_points(&base, &c20.model.factors[1]);
    assert_eq!((e_points.len(), f_points.len()), (109, 113));
    let mut sources = vec![];
    for &p in &e_points {
        for &q in &f_points {
            sources.push(c20.product(&f, [p, q]));
        }
    }
    assert_eq!(sources.len(), 12317);
    assert_eq!(
        sources.iter().map(class_key).collect::<HashSet<_>>().len(),
        12317
    );
    let mut targets = vec![];
    let mut wrong_transpose = [0, 0];
    for (i, d) in sources.iter().enumerate() {
        let image = maps.forward(&f, &maps.source.tag(d)).unwrap();
        assert!(maps.target.jac.is_valid(&image));
        assert_eq!(maps.source.frobenius(&f, d), *d);
        assert_eq!(maps.target.frobenius(&f, &image), image);
        let dual = maps.adjoint(&f, &maps.target.tag(&image)).unwrap();
        let twice = maps.source.jac.mul(d, 2);
        assert_eq!(dual, twice);
        assert_eq!(
            maps.forward(&f, &maps.source.tag(&dual)).unwrap(),
            maps.target.jac.mul(&image, 2)
        );
        let uncorrected_transpose = maps.source.jac.neg(&dual);
        if uncorrected_transpose == twice {
            wrong_transpose[1] += 1;
        } else {
            wrong_transpose[0] += 1;
        }
        assert_eq!(maps.source.jac.mul(&dual, 6159), *d);
        targets.push(image);
        if i % 4096 == 0 {
            println!("PROGRESS T20whole:processed{i}/12317");
        }
    }
    let target_set = targets.iter().map(class_key).collect::<HashSet<_>>();
    assert_eq!(target_set.len(), 12317);
    assert_eq!(wrong_transpose, [12316, 1]);
    let generators = [
        c20.component_pull(&f, 0, e_points[1]),
        c20.component_pull(&f, 1, f_points[1]),
    ];
    let images = generators
        .each_ref()
        .map(|d| maps.forward(&f, &maps.source.tag(d)).unwrap());
    assert_eq!(
        maps.forward(&f, &maps.source.tag(&maps.source.jac.identity()))
            .unwrap(),
        maps.target.jac.identity()
    );
    for (i, d) in sources.iter().enumerate() {
        assert_eq!(
            maps.forward(&f, &maps.source.tag(&maps.source.jac.neg(d)))
                .unwrap(),
            maps.target.jac.neg(&targets[i])
        );
        for component in 0..2 {
            let sum = maps.source.jac.add(d, &generators[component]);
            assert_eq!(
                maps.forward(&f, &maps.source.tag(&sum)).unwrap(),
                maps.target.jac.add(&targets[i], &images[component])
            );
        }
    }
    for component in 0..2 {
        let order = [109, 113][component];
        assert_ne!(images[component], maps.target.jac.identity());
        assert_eq!(
            maps.target.jac.mul(&images[component], order),
            maps.target.jac.identity()
        );
        let mut orbit = HashSet::new();
        for k in 0..order {
            let d = maps.source.jac.mul(&generators[component], k);
            let image = maps.forward(&f, &maps.source.tag(&d)).unwrap();
            assert_eq!(image, maps.target.jac.mul(&images[component], k));
            orbit.insert(class_key(&image));
        }
        assert_eq!(orbit.len(), order as usize);
    }
    let c0_model = EvenModel::source();
    let mut c0 = WholeContext::new(&f, &flat, c0_model);
    let first = crate::seven::construct_edges(&base, &c0.model.factors[0])
        .into_iter()
        .find(|e| e.map.ker == vec![72, 100, 35, 1])
        .unwrap();
    let second = crate::seven::construct_edges(&base, &first.map.cod)
        .into_iter()
        .find(|e| e.map.ker == vec![99, 94, 75, 1])
        .unwrap();
    let edges = [first, second];
    let start_e = rational_points(&base, &c0.model.factors[0]);
    let start_f = rational_points(&base, &c0.model.factors[1]);
    let mut chain_outputs = HashSet::new();
    for &p in &start_e {
        for &q in &start_f {
            let d = c0.product(&f, [p, q]);
            let middle = whole_forward(
                &f,
                &base,
                &edges,
                WholeCase::FortyNine,
                &mut c0,
                &mut c20,
                &d,
            );
            let image = maps.forward(&f, &maps.source.tag(&middle)).unwrap();
            assert!(target_set.contains(&class_key(&image)));
            assert!(chain_outputs.insert(class_key(&image)));
            let back = maps.adjoint(&f, &maps.target.tag(&image)).unwrap();
            let (adjoint, recovered_twice) = whole_adjoint_recovery(
                &f,
                &base,
                &edges,
                WholeCase::FortyNine,
                &mut c0,
                &mut c20,
                &back,
            );
            let expected = c0.product(
                &f,
                [
                    curve::pmul(&base, &c0.model.factors[0], &p, 98),
                    curve::pmul(&base, &c0.model.factors[1], &q, 2),
                ],
            );
            assert_eq!(adjoint, expected);
            assert_eq!(c0.jac.mul(&recovered_twice, 6159), d);
        }
    }
    assert_eq!(chain_outputs.len(), 12317);
    for component in 0..2 {
        let point = if component == 0 {
            start_e[1]
        } else {
            start_f[1]
        };
        let generator = c0.component_pull(&f, component, point);
        let middle = whole_forward(
            &f,
            &base,
            &edges,
            WholeCase::FortyNine,
            &mut c0,
            &mut c20,
            &generator,
        );
        let image_generator = maps.forward(&f, &maps.source.tag(&middle)).unwrap();
        let order = [109, 113][component];
        assert_ne!(image_generator, maps.target.jac.identity());
        assert_eq!(
            maps.target.jac.mul(&image_generator, order),
            maps.target.jac.identity()
        );
        for k in 0..order {
            let d = c0.jac.mul(&generator, k);
            let middle = whole_forward(
                &f,
                &base,
                &edges,
                WholeCase::FortyNine,
                &mut c0,
                &mut c20,
                &d,
            );
            assert_eq!(
                maps.forward(&f, &maps.source.tag(&middle)).unwrap(),
                maps.target.jac.mul(&image_generator, k)
            );
        }
    }
    println!("PASS T20whole:all12317actualC20Mumfordinputs/valid/distinct/Frobeniusfixed/positivebothdual2/public6159recovery;identity/negation/bothgeneratortranslations/all109+113DLPscalars/exactorders;wronguncorrectedtranspose12316different1equal. ActualC0-C20-T20chainall12317bijection/combinedadjointnormdiag98,2/recovery/bothprimeDLPgroups. PhysicalMumfordcoordinate-keycachesonly,noindices/loglabels;no cost/signatureclaim");
}
