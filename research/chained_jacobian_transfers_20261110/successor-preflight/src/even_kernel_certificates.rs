//! Geometric seven-torsion controls and exact cyclic-49 kernel data.
//! Reuses the existing extension/poly/elliptic/Cantor engines throughout.
use super::*;
use isogeny_algos::find::divpoly::division_poly;

struct CyclicKernel {
    h49: Vec<u64>,
    psi7: Vec<u64>,
    psi49: Vec<u64>,
    quotient: Vec<u64>,
    frobenius_gcds: Vec<(usize, Vec<u64>)>,
}

fn cyclic_kernel(base: &Zp, first: &RatIsogeny<Zp>, second: &RatIsogeny<Zp>) -> CyclicKernel {
    assert_eq!(first.cod, second.dom);
    assert_eq!((first.deg, second.deg), (7, 7));
    assert_eq!(first.ker, vec![72, 100, 35, 1]);
    assert_eq!(second.ker, vec![99, 94, 75, 1]);
    assert_eq!(first.den, poly::mul(base, &first.ker, &first.ker));
    assert_eq!(poly::gcd(base, &first.num, &first.den), vec![1]);
    let n = &first.num;
    let d = &first.den;
    let n2 = poly::mul(base, n, n);
    let d2 = poly::mul(base, d, d);
    let terms = [
        poly::mul(base, &d2, d),
        poly::mul(base, n, &d2),
        poly::mul(base, &n2, d),
        poly::mul(base, &n2, n),
    ];
    let mut homogeneous = vec![];
    for (coefficient, term) in second.ker.iter().zip(terms.iter()) {
        homogeneous = poly::add(base, &homogeneous, &poly::scale(base, term, *coefficient));
    }
    assert_eq!(homogeneous.len(), 22);
    assert_eq!(poly::gcd(base, &homogeneous, &first.ker), vec![1]);
    let h49 = poly::mul(base, &first.ker, &homogeneous);
    assert_eq!(h49.len(), 25);
    assert_eq!(h49.last(), Some(&1));
    assert_eq!(
        poly::gcd(base, &h49, &poly::derivative(base, &h49)),
        vec![1]
    );
    assert_eq!(
        poly::gcd(base, &h49, &vec![first.dom.b, first.dom.a, 0, 1]),
        vec![1]
    );
    let psi7 = division_poly(base, &first.dom, 7);
    let psi49 = division_poly(base, &first.dom, 49);
    assert_eq!((psi49.len(), psi49.last()), (1201, Some(&49)));
    let (quotient, remainder) = poly::divrem(base, &psi49, &h49);
    assert!(remainder.is_empty());
    assert_eq!(poly::mul(base, &h49, &quotient), psi49);
    assert_eq!(poly::gcd(base, &h49, &psi7), first.ker);
    let x = vec![0, 1];
    let mut power = x.clone();
    let mut frobenius_gcds = vec![];
    for degree in 1..=21 {
        power = poly::powmod(base, &power, 101, &h49);
        if [1, 3, 7, 21].contains(&degree) {
            let gcd = poly::gcd(base, &h49, &poly::sub(base, &power, &x));
            let expected = match degree {
                3 => first.ker.clone(),
                21 => h49.clone(),
                _ => vec![1],
            };
            assert_eq!(gcd, expected);
            frobenius_gcds.push((degree, gcd));
        }
    }
    println!("PASS evenKernel cyclic49:h49{h49:?};monic/squarefree/sourcecubiccoprime;exactpsi49degree1200division/quotient;gcdpsi7=h1;DDFdegrees3+21. Fullpointfield42useswrittenFrobenius33/order42,not an Fp42enumeration");
    CyclicKernel {
        h49,
        psi7,
        psi49,
        quotient,
        frobenius_gcds,
    }
}

fn extension_isogeny(f: &FpR, map: &RatIsogeny<Zp>) -> RatIsogeny<FpR> {
    RatIsogeny {
        dom: extension_curve(f, map.dom),
        cod: extension_curve(f, map.cod),
        deg: map.deg,
        ker: lift(f, &map.ker),
        num: lift(f, &map.num),
        den: lift(f, &map.den),
    }
}

fn json_point(p: Pt<ER>) -> String {
    match p {
        Pt::Inf => "null".to_string(),
        Pt::Aff(x, y) => format!("{{\"x\":{x:?},\"y\":{y:?}}}"),
    }
}

#[test]
fn geometric_seven_kernel_and_cyclic_forty_nine_certificate() {
    let f = FpR::new(101, 12);
    let base = Zp::new(101);
    let flat = Flat(&f);
    let mut presentations = models(&f).into_iter();
    let mut source = WholeContext::new(&f, &flat, presentations.next().unwrap().1);
    let mut target = WholeContext::new(&f, &flat, presentations.next().unwrap().1);
    let first = crate::seven::construct_edges(&base, &source.model.factors[0])
        .into_iter()
        .find(|edge| edge.map.ker == vec![72, 100, 35, 1])
        .unwrap();
    let second = crate::seven::construct_edges(&base, &first.map.cod)
        .into_iter()
        .find(|edge| edge.map.ker == vec![99, 94, 75, 1])
        .unwrap();
    assert_eq!(first.map.cod, target.model.factors[0]);
    let certificate = cyclic_kernel(&base, &first.map, &second.map);
    let alpha = extension_isogeny(&f, &first.map);
    let mut xs = poly::roots(&f, &lift(&f, &certificate.psi7), &mut Rng::new(0x0749));
    xs.sort();
    assert_eq!(xs.len(), 24);
    let mut points = vec![Pt::Inf];
    for x in xs {
        let y = f.sqrt(curve::rhs(&f, &alpha.dom, x)).unwrap();
        assert_ne!(y, f.zero());
        let mut signs = [y, f.neg(y)];
        signs.sort();
        points.extend(signs.map(|sign| Pt::Aff(x, sign)));
    }
    assert_eq!(points.len(), 49);
    let point_indices = points
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<HashMap<_, _>>();
    assert_eq!(point_indices.len(), 49);
    let mut classes = vec![];
    let mut images = vec![];
    let mut alpha_points = vec![];
    let mut kernel = vec![];
    let mut different = 0;
    let mut equal = 0;
    for (index, &point) in points.iter().enumerate() {
        assert!(curve::on_curve(&f, &alpha.dom, &point));
        assert_eq!(curve::pmul(&f, &alpha.dom, &point, 7), Pt::Inf);
        let d = source
            .model
            .pull(&f, &source.chart, &source.jac, 0, point)
            .0;
        assert!(source.jac.is_valid(&d));
        assert_eq!(source.jac.mul(&d, 7), source.jac.identity());
        let norm = source
            .model
            .norm(&f, &source.chart, &source.jac, &d, &mut source.roots);
        assert_eq!(norm, [curve::pmul(&f, &alpha.dom, &point, 2), Pt::Inf]);
        let image_point = alpha.eval(&f, &point);
        let cleared = target
            .model
            .pull(&f, &target.chart, &target.jac, 0, alpha.eval(&f, &norm[0]))
            .0;
        let correct = target.jac.mul(&cleared, 4);
        let expected = target
            .model
            .pull(&f, &target.chart, &target.jac, 0, image_point)
            .0;
        assert_eq!(correct, expected);
        assert!(target.jac.is_valid(&correct));
        assert_eq!(target.jac.mul(&correct, 7), target.jac.identity());
        let wrong = target.jac.mul(&cleared, 55);
        if wrong == correct {
            equal += 1;
            assert_eq!(image_point, Pt::Inf);
        } else {
            different += 1;
            assert_ne!(image_point, Pt::Inf);
        }
        if image_point == Pt::Inf {
            kernel.push(index);
            assert_eq!(correct, target.jac.identity());
        }
        alpha_points.push(image_point);
        classes.push(d);
        images.push(correct);
    }
    assert_eq!((different, equal), (42, 7));
    assert_eq!(kernel.len(), 7);
    assert_eq!(
        classes.iter().map(class_key).collect::<HashSet<_>>().len(),
        49
    );
    assert_eq!(
        images.iter().map(class_key).collect::<HashSet<_>>().len(),
        7
    );
    let mut frobenius_indices = vec![];
    for (i, &point) in points.iter().enumerate() {
        let pi = frobenius(&f, point, 1);
        let next = *point_indices.get(&pi).unwrap();
        frobenius_indices.push(next);
        let frob_source = source.model.frobenius_class(
            &f,
            &source.chart,
            &source.jac,
            &classes[i],
            &mut source.roots,
        );
        assert_eq!(frob_source, classes[next]);
        let frob_target = target.model.frobenius_class(
            &f,
            &target.chart,
            &target.jac,
            &images[i],
            &mut target.roots,
        );
        assert_eq!(frob_target, images[next]);
        if kernel.contains(&i) {
            assert!(kernel.contains(&next));
            assert_eq!(frob_source, source.jac.mul(&classes[i], 5));
            if i == 0 {
                assert_eq!(classes[i], source.jac.identity());
            } else {
                assert_ne!(classes[i], source.jac.identity());
            }
        }
        for (j, &other) in points.iter().enumerate() {
            let sum = curve::padd(&f, &alpha.dom, &point, &other);
            let sum_index = *point_indices.get(&sum).unwrap();
            assert_eq!(source.jac.add(&classes[i], &classes[j]), classes[sum_index]);
            assert_eq!(target.jac.add(&images[i], &images[j]), images[sum_index]);
        }
    }
    let kernel_sums = kernel
        .iter()
        .map(|&i| {
            kernel
                .iter()
                .map(|&j| {
                    let sum = curve::padd(&f, &alpha.dom, &points[i], &points[j]);
                    let sum_index = *point_indices.get(&sum).unwrap();
                    assert!(kernel.contains(&sum_index));
                    assert_eq!(source.jac.add(&classes[i], &classes[j]), classes[sum_index]);
                    sum_index
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("PASS evenKernel geometricE0[7]:all49distinctoncurve/orderdiv7/actualC0pulls/norm2/cross0/all2401source+targetsumcomparisons/Frobeniuscommutators;actualkernel7classes/all49sums/nonzeroorder7/Frobenius5;correcthalf4vsWRONGrationalhalf55:42different7equal. Firstfactor49slice,notwholeJacobian2401torsion;globaldegree7bridgewritten");
    if let Ok(path) = std::env::var("CHAINED_EVEN_KERNEL_CERTIFICATE_OUT") {
        let points_json = points
            .iter()
            .map(|&p| json_point(p))
            .collect::<Vec<_>>()
            .join(",");
        let alpha_json = alpha_points
            .iter()
            .map(|&p| json_point(p))
            .collect::<Vec<_>>()
            .join(",");
        let kernel_json = kernel
            .iter()
            .map(|&i| {
                format!(
                    "{{\"point_index\":{i},\"u\":{:?},\"v\":{:?}}}",
                    classes[i].u, classes[i].v
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let gcd_json = certificate
            .frobenius_gcds
            .iter()
            .map(|(d, g)| format!("{{\"degree\":{d},\"gcd\":{g:?}}}"))
            .collect::<Vec<_>>()
            .join(",");
        let out = format!(
            concat!(
                "{{\"schema\":\"chained-even-kernel/v1\",\"p\":101,\"extension_degree\":12,",
                "\"field_modulus\":{:?},\"source_sextic\":{:?},\"target_sextic\":{:?},",
                "\"chart_a\":{:?},\"chart_l\":{:?},\"chart_quintic\":{:?},",
                "\"elliptic_models\":[[{},{}],[{},{}],[{},{}]],",
                "\"h1\":{:?},\"h2\":{:?},\"N\":{:?},\"D\":{:?},",
                "\"second_N\":{:?},\"second_D\":{:?},",
                "\"h49\":{:?},\"psi7\":{:?},\"psi49\":{:?},\"quotient\":{:?},",
                "\"frobenius_gcds\":[{}],\"E7_points\":[{}],\"alpha_points\":[{}],",
                "\"kernel_divisors\":[{}],\"kernel_sum_indices\":{:?},",
                "\"frobenius_indices\":{:?},\"correct_half_mod7\":4,\"wrong_half\":55,",
                "\"wrong_different\":42,\"wrong_equal\":7,\"standing\":\"unsigned\"}}\n"
            ),
            f.modulus(),
            source.model.coefficients,
            target.model.coefficients,
            source.chart.a,
            source.chart.l,
            source.chart.quintic,
            first.map.dom.a,
            first.map.dom.b,
            first.map.cod.a,
            first.map.cod.b,
            second.map.cod.a,
            second.map.cod.b,
            first.map.ker,
            second.map.ker,
            first.map.num,
            first.map.den,
            second.map.num,
            second.map.den,
            certificate.h49,
            certificate.psi7,
            certificate.psi49,
            certificate.quotient,
            gcd_json,
            points_json,
            alpha_json,
            kernel_json,
            kernel_sums,
            frobenius_indices
        );
        std::fs::write(path, out).expect("write complete native kernel certificate");
    }
}
