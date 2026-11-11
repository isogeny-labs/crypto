use super::seven::{construct_edges, SevenEdge};
use isogeny_algos::curve::{self, Curve, Isogeny, Pt, RatIsogeny};
use isogeny_algos::field::{Field, Rng, Zp};
use isogeny_algos::fpr::{FpR, ER};
use isogeny_algos::{genus2, poly};
use std::collections::HashSet;

fn lift(f: &FpR, edge: &SevenEdge) -> RatIsogeny<FpR> {
    let m = &edge.map;
    let coefficients = |p: &Vec<u64>| p.iter().map(|c| f.from_u64(*c)).collect();
    RatIsogeny {
        dom: Curve::new(f.from_u64(m.dom.a), f.from_u64(m.dom.b)),
        cod: Curve::new(f.from_u64(m.cod.a), f.from_u64(m.cod.b)),
        deg: m.deg,
        ker: coefficients(&m.ker),
        num: coefficients(&m.num),
        den: coefficients(&m.den),
    }
}

fn frobenius(f: &FpR, p: Pt<ER>) -> Pt<ER> {
    match p {
        Pt::Inf => Pt::Inf,
        Pt::Aff(x, y) => Pt::Aff(f.frobenius(x), f.frobenius(y)),
    }
}

fn kernel_action(f: &FpR, edge: &SevenEdge) -> (Vec<Pt<ER>>, usize) {
    let map = lift(f, edge);
    let roots = poly::roots(f, &map.ker, &mut Rng::new(0x24_1017));
    assert_eq!(roots.len(), 3);
    let x = roots[0];
    let y = f
        .sqrt(curve::rhs(f, &map.dom, x))
        .expect("actual7-kernel splits in fixedFp6");
    assert_ne!(y, f.zero());
    let generator = Pt::Aff(x, y);
    let points = (0..7)
        .map(|n| curve::pmul(f, &map.dom, &generator, n))
        .collect::<Vec<_>>();
    assert_eq!(points.iter().copied().collect::<HashSet<_>>().len(), 7);
    assert_eq!(curve::pmul(f, &map.dom, &generator, 7), Pt::Inf);
    let eigen = (1..7)
        .find(|n| points[*n] == frobenius(f, generator))
        .unwrap();
    for i in 0..7 {
        assert!(curve::on_curve(f, &map.dom, &points[i]));
        assert_eq!(map.eval(f, &points[i]), Pt::Inf);
        assert_eq!(frobenius(f, points[i]), points[(i * eigen) % 7]);
        for j in 0..7 {
            assert_eq!(
                curve::padd(f, &map.dom, &points[i], &points[j]),
                points[(i + j) % 7]
            );
        }
    }
    (points, eigen)
}

fn rational_points(f: &Zp, e: &Curve<u64>) -> Vec<Pt<u64>> {
    let mut out = vec![Pt::Inf];
    for x in 0..101 {
        for y in 0..101 {
            let p = Pt::Aff(x, y);
            if curve::on_curve(f, e, &p) {
                out.push(p);
            }
        }
    }
    out
}

fn image_two(f: &FpR, edge: &SevenEdge, roots: [ER; 3]) -> [ER; 3] {
    let map = lift(f, edge);
    let points = roots.map(|x| Pt::Aff(x, f.zero()));
    let images = points.map(|p| {
        assert!(curve::on_curve(f, &map.dom, &p));
        assert_eq!(curve::pmul(f, &map.dom, &p, 2), Pt::Inf);
        map.eval(f, &p)
    });
    let out = images.map(|p| {
        assert!(curve::on_curve(f, &map.cod, &p));
        let Pt::Aff(x, y) = p else {
            panic!("odd map killed nonzero2-torsion")
        };
        assert_eq!(y, f.zero());
        x
    });
    assert_eq!(curve::padd(f, &map.cod, &images[0], &images[1]), images[2]);
    for i in 0..3 {
        assert_ne!(out[i], out[(i + 1) % 3]);
        assert_eq!(f.frobenius(out[i]), out[(i + 1) % 3]);
    }
    out
}

#[test]
fn successor_nondual_kernels_and_actual_hlp_gluing() {
    let f = Zp::new(101);
    let extension = FpR::from_modulus(101, &[33, 46, 2, 18, 56, 12, 1]);
    let original = [Curve::new(1, 46), Curve::new(44, 21)];
    let old = [
        construct_edges(&f, &original[0]).remove(1),
        construct_edges(&f, &original[1]).remove(0),
    ];
    assert_eq!((old[0].map.cod.a, old[0].map.cod.b), (26, 58));
    assert_eq!((old[1].map.cod.a, old[1].map.cod.b), (69, 50));
    let mut next = vec![];
    for i in 0..2 {
        let candidates = construct_edges(&f, &old[i].map.cod);
        let mut previous_count = 0;
        let mut selected = None;
        for edge in candidates {
            let (kernel, eigen) = kernel_action(&extension, &edge);
            let previous = edge.map.ker == old[i].dual.ker;
            if previous {
                previous_count += 1;
                assert_eq!(eigen, [2, 4][i]);
            } else {
                assert_eq!(eigen, [5, 6][i]);
                assert!(selected.is_none());
            }
            println!("component{i} kernel{:?},cod({},{}) j{},Frob{eigen},previous_dual{previous},actual7points{kernel:?}",edge.map.ker,edge.map.cod.a,edge.map.cod.b,curve::jinv(&f,&edge.map.cod));
            if !previous {
                selected = Some(edge);
            }
        }
        assert_eq!(previous_count, 1);
        let selected = selected.unwrap();
        assert_ne!(selected.map.ker, old[i].dual.ker);
        let source_points = rational_points(&f, &selected.map.dom);
        let target_points = rational_points(&f, &selected.map.cod);
        let n = [109_u64, 113][i];
        assert_eq!(
            (source_points.len(), target_points.len()),
            (n as usize, n as usize)
        );
        let images = source_points
            .iter()
            .map(|p| {
                let image = selected.map.eval(&f, p);
                assert_eq!(
                    selected.reverse(&f, &image),
                    curve::pmul(&f, &selected.map.dom, p, 7)
                );
                image
            })
            .collect::<HashSet<_>>();
        assert_eq!(images, target_points.iter().copied().collect());
        for p in &target_points {
            assert_eq!(
                selected.map.eval(&f, &selected.reverse(&f, p)),
                curve::pmul(&f, &selected.map.cod, p, 7)
            );
        }
        let inverse49 = (1..n).find(|k| 49 * k % n == 1).unwrap();
        let original_points = rational_points(&f, &original[i]);
        let generator = original_points[1];
        let generator_image = selected.map.eval(&f, &old[i].map.eval(&f, &generator));
        for k in 0..n {
            let p = curve::pmul(&f, &original[i], &generator, u128::from(k));
            let image = selected.map.eval(&f, &old[i].map.eval(&f, &p));
            assert_eq!(
                image,
                curve::pmul(&f, &selected.map.cod, &generator_image, u128::from(k))
            );
            let reversed = old[i].reverse(&f, &selected.reverse(&f, &image));
            assert_eq!(reversed, curve::pmul(&f, &original[i], &p, 49));
            assert_eq!(
                curve::pmul(&f, &original[i], &reversed, u128::from(inverse49)),
                p
            );
        }
        println!("component{i} next({},{}) j{},previousj{},currentj{},full{n} bijection/dual7/two-hop49-inverse{inverse49}/scalar controls pass",selected.map.cod.a,selected.map.cod.b,curve::jinv(&f,&selected.map.cod),curve::jinv(&f,&original[i]),curve::jinv(&f,&old[i].map.cod));
        next.push(selected);
    }
    let cubic = FpR::from_modulus(101, &[56, 13, 6, 1]);
    let a = cubic.gen();
    let roots = [a, cubic.frobenius(a), cubic.frobenius(cubic.frobenius(a))];
    assert_eq!(cubic.frobenius(roots[2]), roots[0]);
    let original_roots = [
        roots.map(|x| cubic.add(x, cubic.from_u64(2))),
        roots.map(|x| cubic.add(cubic.div(cubic.from_u64(56), x), cubic.from_u64(38))),
    ];
    let images: [[ER; 3]; 2] = std::array::from_fn(|i| {
        image_two(
            &cubic,
            &next[i],
            image_two(&cubic, &old[i], original_roots[i]),
        )
    });
    let raw3 = genus2::glue(&cubic, images[0], images[1])
        .expect("transported actual2-gluing gives a smooth Jacobian model");
    let raw = raw3
        .iter()
        .map(|x| {
            cubic
                .project(*x)
                .expect("actual HLP coefficients descend toF101")
        })
        .collect::<Vec<_>>();
    assert_eq!(raw.len(), 7);
    assert_ne!(raw[0], 0);
    assert!([1, 3, 5].iter().all(|i| raw[*i] == 0));
    assert_eq!(poly::gcd(&f, &raw, &poly::derivative(&f, &raw)), vec![1]);
    let l = genus2::lpoly(&f, &raw);
    assert_eq!(l, genus2::LPoly::product(-7, -11, 101));
    assert_eq!(l.jac_order(101), 12317);
    let rho = (1..101).find(|x| f.sq(*x) == raw[6]);
    let normalized = raw.iter().map(|x| f.div(*x, raw[6])).collect::<Vec<_>>();
    let constant_root = (1..101).find(|x| f.sq(*x) == normalized[0]);
    println!("SUCCESSOR actual transported2roots{images:?};raw{raw:?},leading_root{rho:?},normalized{normalized:?},constant_root{constant_root:?},L{l:?},Jac12317;chart boundaries retained without retwist;new full Jacobian transfer/global proof remain next");
}
