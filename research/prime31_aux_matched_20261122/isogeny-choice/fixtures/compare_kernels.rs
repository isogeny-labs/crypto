use std::collections::BTreeSet;
use std::fmt::Write as FmtWrite;
use std::fs;
const P: u64 = 2_147_483_647;
const TARGET_A: u64 = 847_514_846;
const TARGET_B: u64 = 2_087_444_724;
fn add(a: u64, b: u64) -> u64 {
    (a + b) % P
}
fn sub(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}
fn mul(a: u64, b: u64) -> u64 {
    a * b % P
}
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = mul(r, a)
        }
        a = mul(a, a);
        n >>= 1;
    }
    r
}
fn inv(a: u64) -> u64 {
    assert_ne!(a, 0);
    pow(a, P - 2)
}
fn leg(a: u64) -> i8 {
    let q = pow(a, (P - 1) / 2);
    if q == 0 {
        0
    } else if q == 1 {
        1
    } else {
        -1
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pt {
    x: u64,
    y: u64,
}
type Point = Option<Pt>;
#[derive(Clone, Copy, Debug)]
struct Curve {
    a: u64,
    b: u64,
}
impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        add(add(mul(mul(x, x), x), mul(self.a, x)), self.b)
    }
    fn contains(&self, q: Point) -> bool {
        q.map_or(true, |q| mul(q.y, q.y) == self.rhs(q.x))
    }
    fn nonsingular(&self) -> bool {
        add(
            mul(4, mul(mul(self.a, self.a), self.a)),
            mul(27, mul(self.b, self.b)),
        ) != 0
    }
    fn sum(&self, p: Point, q: Point) -> Point {
        match (p, q) {
            (None, q) => q,
            (p, None) => p,
            (Some(p), Some(q)) => {
                if p.x == q.x && add(p.y, q.y) == 0 {
                    return None;
                }
                let m = if p.x == q.x {
                    mul(add(mul(3, mul(p.x, p.x)), self.a), inv(mul(2, p.y)))
                } else {
                    mul(sub(q.y, p.y), inv(sub(q.x, p.x)))
                };
                let x = sub(sub(mul(m, m), p.x), q.x);
                let y = sub(mul(m, sub(p.x, x)), p.y);
                Some(Pt { x, y })
            }
        }
    }
    fn scalar(&self, mut n: u64, mut p: Point) -> Point {
        let mut q = None;
        while n > 0 {
            if n & 1 == 1 {
                q = self.sum(q, p)
            }
            p = self.sum(p, p);
            n >>= 1;
        }
        q
    }
}
#[derive(Clone, Copy, Debug)]
struct Iso {
    domain: Curve,
    codomain: Curve,
    k: Pt,
    tau: u64,
}
impl Iso {
    fn new(domain: Curve, k: Pt) -> Iso {
        assert_eq!(k.y, 0);
        assert!(domain.contains(Some(k)));
        let tau = add(mul(3, mul(k.x, k.x)), domain.a);
        let codomain = Curve {
            a: sub(domain.a, mul(5, tau)),
            b: sub(domain.b, mul(7, mul(k.x, tau))),
        };
        assert!(codomain.nonsingular());
        Iso {
            domain,
            codomain,
            k,
            tau,
        }
    }
    fn apply(&self, p: Point) -> Point {
        p.and_then(|p| {
            if p.x == self.k.x {
                return None;
            }
            let dinv = inv(sub(p.x, self.k.x));
            let x = add(p.x, mul(self.tau, dinv));
            let y = mul(p.y, sub(1, mul(self.tau, mul(dinv, dinv))));
            Some(Pt { x, y })
        })
    }
}
fn chain(maps: &[Iso], mut p: Point) -> Point {
    for m in maps {
        p = m.apply(p)
    }
    p
}
fn pointjson(p: Point) -> String {
    match p {
        None => "null".to_string(),
        Some(p) => format!("[{},{}]", p.x, p.y),
    }
}
fn exact_two_order(e: Curve, p: Point, exponent: u32) -> bool {
    e.scalar(1u64 << exponent, p).is_none() && e.scalar(1u64 << (exponent - 1), p).is_some()
}
type Poly = Vec<u64>; // Coefficients in ascending powers of x.
fn poly_trim(mut a: Poly) -> Poly {
    while a.len() > 1 && a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn poly_scale_add(a: &Poly, b: &Poly, scale: u64) -> Poly {
    let mut r = vec![0; a.len().max(b.len())];
    for (i, &v) in a.iter().enumerate() {
        r[i] = v;
    }
    for (i, &v) in b.iter().enumerate() {
        r[i] = add(r[i], mul(scale, v));
    }
    poly_trim(r)
}
fn poly_mul(a: &Poly, b: &Poly) -> Poly {
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &v) in a.iter().enumerate() {
        for (j, &w) in b.iter().enumerate() {
            r[i + j] = add(r[i + j], mul(v, w));
        }
    }
    poly_trim(r)
}
fn poly_eval(a: &Poly, x: u64) -> u64 {
    a.iter().rev().fold(0, |acc, &c| add(mul(acc, x), c))
}
fn polynomial_json(s: &mut String, name: &str, a: &Poly) {
    write!(
        s,
        "\"{}\":{{\"degree\":{},\"nonzero_coefficients\":{},\"coefficients_ascending\":[",
        name,
        a.len() - 1,
        a.iter().filter(|&&v| v != 0).count()
    )
    .unwrap();
    for (i, v) in a.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        write!(s, "{}", v).unwrap();
    }
    s.push_str("]}");
}

#[derive(Clone)]
struct Arm {
    name: &'static str,
    domain: Curve,
    source: Pt,
    maps: Vec<Iso>,
    kernel: Vec<Point>,
    numerator: Poly,
    denominator: Poly,
    degree4: Vec<(Poly, Poly)>,
}
fn rational_polynomials(maps: &[Iso]) -> (Poly, Poly) {
    let mut n = vec![0, 1];
    let mut d = vec![1];
    for map in maps {
        let nn = poly_mul(&n, &n);
        let nd = poly_mul(&n, &d);
        let dd = poly_mul(&d, &d);
        n = poly_scale_add(&poly_scale_add(&nn, &nd, sub(0, map.k.x)), &dd, map.tau);
        d = poly_scale_add(&nd, &dd, sub(0, map.k.x));
    }
    (n, d)
}
fn map_x_affine(maps: &[Iso], mut x: u64) -> Option<u64> {
    for map in maps {
        if x == map.k.x {
            return None;
        }
        x = add(x, mul(map.tau, inv(sub(x, map.k.x))));
    }
    Some(x)
}
fn map_x_projective(maps: &[Iso], x: u64) -> Option<u64> {
    let mut n = x;
    let mut d = 1;
    for map in maps {
        let nn = mul(n, n);
        let nd = mul(n, d);
        let dd = mul(d, d);
        n = add(sub(nn, mul(map.k.x, nd)), mul(map.tau, dd));
        d = sub(nd, mul(map.k.x, dd));
    }
    if d == 0 {
        None
    } else {
        Some(mul(n, inv(d)))
    }
}
fn homogeneous_pair(n: &Poly, d: &Poly, x: u64, z: u64) -> (u64, u64) {
    let deg = n.len().max(d.len()) - 1;
    assert_eq!(deg, 4);
    let xx = mul(x, x);
    let xz = mul(x, z);
    let zz = mul(z, z);
    let monomials = [
        mul(zz, zz),
        mul(xz, zz),
        mul(xx, zz),
        mul(xx, xz),
        mul(xx, xx),
    ];
    let mut nn = 0;
    let mut dd = 0;
    for i in 0..=deg {
        let mon = monomials[i];
        if i < n.len() {
            nn = add(nn, mul(n[i], mon));
        }
        if i < d.len() {
            dd = add(dd, mul(d[i], mon));
        }
    }
    (nn, dd)
}
fn map_x_degree4(blocks: &[(Poly, Poly)], x: u64) -> Option<u64> {
    let mut n = x;
    let mut d = 1;
    for (pn, pd) in blocks {
        (n, d) = homogeneous_pair(pn, pd, n, d);
    }
    if d == 0 {
        None
    } else {
        Some(mul(n, inv(d)))
    }
}
fn map_x_dense(n: &Poly, d: &Poly, x: u64) -> Option<u64> {
    let den = poly_eval(d, x);
    if den == 0 {
        None
    } else {
        Some(mul(poly_eval(n, x), inv(den)))
    }
}
fn cyclic_kernel(e: Curve, k: Point, n: usize) -> Vec<Point> {
    let mut q = None;
    let mut result = Vec::with_capacity(n);
    for _ in 0..n {
        result.push(q);
        q = e.sum(q, k);
    }
    assert!(q.is_none());
    assert_eq!(result.iter().copied().collect::<BTreeSet<_>>().len(), n);
    result
}
fn cyclic_chain(e: Curve, k: Point, exponent: u32) -> Vec<Iso> {
    let mut cur = e;
    let mut h = k;
    let mut maps = vec![];
    for j in 0..exponent {
        let ker = cur.scalar(1 << (exponent - j - 1), h).unwrap();
        let m = Iso::new(cur, ker);
        h = m.apply(h);
        cur = m.codomain;
        maps.push(m);
    }
    assert!(h.is_none());
    maps
}
fn make_arm(
    name: &'static str,
    domain: Curve,
    source: Pt,
    maps: Vec<Iso>,
    kernel: Vec<Point>,
) -> Arm {
    assert_eq!(maps.len(), 10);
    assert_eq!(kernel.len(), 1024);
    assert_eq!(kernel.iter().copied().collect::<BTreeSet<_>>().len(), 1024);
    for &h in &kernel {
        assert!(domain.contains(h));
        assert_eq!(chain(&maps, h), None);
    }
    let final_curve = maps.last().unwrap().codomain;
    let mut tsum = 0;
    let mut wsum = 0;
    for h in kernel.iter().flatten() {
        tsum = add(tsum, add(mul(3, mul(h.x, h.x)), domain.a));
        wsum = add(
            wsum,
            add(
                add(mul(5, mul(mul(h.x, h.x), h.x)), mul(3, mul(domain.a, h.x))),
                mul(2, domain.b),
            ),
        );
    }
    assert_eq!(final_curve.a, sub(domain.a, mul(5, tsum)));
    assert_eq!(final_curve.b, sub(domain.b, mul(7, wsum)));
    let (numerator, denominator) = rational_polynomials(&maps);
    assert_eq!(numerator.len() - 1, 1024);
    assert_eq!(denominator.len() - 1, 1023);
    let degree4 = maps.chunks(2).map(rational_polynomials).collect::<Vec<_>>();
    assert!(degree4.iter().all(|(n, d)| n.len() == 5 && d.len() == 4));
    Arm {
        name,
        domain,
        source,
        maps,
        kernel,
        numerator,
        denominator,
        degree4,
    }
}
fn arms() -> Vec<Arm> {
    let e = Curve { a: P - 1, b: 0 };
    let s = Pt { x: 4, y: 639874715 };
    let k = Pt {
        x: 1724938470,
        y: 923814573,
    };
    let u = Pt { x: 0, y: 0 };
    assert!(exact_two_order(e, Some(s), 30));
    assert_eq!(e.scalar(1 << 20, Some(s)), Some(k));
    assert_eq!(e.scalar(512, Some(k)), Some(Pt { x: 1, y: 0 }));
    let twice = e.sum(Some(k), Some(k));
    let prefix = cyclic_chain(e, twice, 9);
    let ep = prefix.last().unwrap().codomain;
    let ki = chain(&prefix, Some(k)).unwrap();
    let ui = chain(&prefix, Some(u)).unwrap();
    let kui = ep.sum(Some(ki), Some(ui)).unwrap();
    assert_ne!(ki, ui);
    assert_ne!(kui, ki);
    assert_ne!(kui, ui);
    let h0 = cyclic_kernel(e, Some(k), 1024);
    let h1 = cyclic_kernel(e, e.sum(Some(k), Some(u)), 1024);
    let mut h2 = cyclic_kernel(e, twice, 512);
    let shifted = h2.iter().map(|&q| e.sum(q, Some(u))).collect::<Vec<_>>();
    h2.extend(shifted);
    let mut out = vec![];
    for (name, last, kernel) in [
        ("Aminus_H0", ki, h0),
        ("Aminus_H1", kui, h1),
        ("Aminus_H2", ui, h2),
    ] {
        let mut maps = prefix.clone();
        maps.push(Iso::new(ep, last));
        out.push(make_arm(name, e, s, maps, kernel));
    }
    let ep = Curve { a: 1, b: 0 };
    let sp = Pt { x: 3, y: 615668188 };
    let kp = ep.scalar(1 << 21, Some(sp));
    assert!(exact_two_order(ep, Some(sp), 31));
    out.push(make_arm(
        "Aplus_cyclic",
        ep,
        sp,
        cyclic_chain(ep, kp, 10),
        cyclic_kernel(ep, kp, 1024),
    ));
    out
}
fn xs_for(a: &Arm, scalar: u64) -> (Pt, Vec<u64>) {
    let p0 = a.domain.scalar(scalar, Some(a.source));
    let t = chain(&a.maps, p0).unwrap();
    let end = a.maps.last().unwrap().codomain;
    assert!(end.contains(Some(t)));
    assert!(end.sum(Some(t), Some(t)).is_some());
    let mut xs = BTreeSet::new();
    for &h in &a.kernel {
        let q = a.domain.sum(p0, h).unwrap();
        assert!(a.domain.contains(Some(q)));
        assert_eq!(chain(&a.maps, Some(q)), Some(t));
        assert!(xs.insert(q.x));
    }
    assert_eq!(xs.len(), 1024);
    (t, xs.into_iter().collect())
}
fn scalars() -> Vec<u64> {
    let mut out = vec![1];
    let mut state = 0x859a47c1741024u64;
    while out.len() < 16 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let n = (state % ((1 << 20) - 2)) | 1;
        if !out.contains(&n) {
            out.push(n);
        }
    }
    out
}
fn json_nums(s: &mut String, v: &[u64]) {
    s.push('[');
    for (i, x) in v.iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        write!(s, "{x}").unwrap();
    }
    s.push(']');
}
fn verify_and_write(arms: &[Arm], out: &std::path::Path, census: bool) {
    let targets = [
        Curve {
            a: TARGET_A,
            b: TARGET_B,
        },
        Curve {
            a: 323891735,
            b: 1985248359,
        },
    ];
    let sample_scalars = if census { scalars() } else { vec![1] };
    let mut json=String::from("{\"schema\":\"p31.kernel-comparison/v1\",\"status\":\"arithmetic_correctness_fixtures_no_timing\",\"p\":2147483647,\"degree\":1024,\"source_scalars\":");
    json_nums(&mut json, &sample_scalars);
    json.push_str(",\"target_curves\":[[847514846,2087444724],[323891735,1985248359]],\"arms\":[");
    let mut csv = String::from("arm,source_scalar,chi,accepted_E_x,accepted_E1_x,cardinality_V\n");
    for (ai, a) in arms.iter().enumerate() {
        if ai > 0 {
            json.push(',');
        }
        let end = a.maps.last().unwrap().codomain;
        write!(
            json,
            "{{\"name\":\"{}\",\"domain\":[{},{}],\"source\":{},\"codomain\":[{},{}],\"maps\":[",
            a.name,
            a.domain.a,
            a.domain.b,
            pointjson(Some(a.source)),
            end.a,
            end.b
        )
        .unwrap();
        for (j, m) in a.maps.iter().enumerate() {
            if j > 0 {
                json.push(',');
            }
            write!(
                json,
                "{{\"stage\":{},\"alpha\":{},\"tau\":{},\"domain\":[{},{}],\"codomain\":[{},{}]}}",
                j + 1,
                m.k.x,
                m.tau,
                m.domain.a,
                m.domain.b,
                m.codomain.a,
                m.codomain.b
            )
            .unwrap();
        }
        json.push_str("],\"cosets\":[");
        for (ci, &scalar) in sample_scalars.iter().enumerate() {
            let (t, xs) = xs_for(a, scalar);
            let counts = targets.map(|e| xs.iter().filter(|&&x| leg(e.rhs(x)) >= 0).count());
            assert!(xs.iter().all(|&x| targets.iter().all(|e| e.rhs(x) != 0)));
            if ci == 0 {
                let membership = poly_scale_add(&a.numerator, &a.denominator, sub(0, t.x));
                let mut product = vec![1];
                for &x in &xs {
                    product = poly_mul(&product, &vec![sub(0, x), 1]);
                    assert_ne!(poly_eval(&a.denominator, x), 0);
                }
                assert_eq!(membership, product);
                for &x in &xs {
                    assert_eq!(map_x_affine(&a.maps, x), Some(t.x));
                    assert_eq!(map_x_projective(&a.maps, x), Some(t.x));
                    assert_eq!(map_x_degree4(&a.degree4, x), Some(t.x));
                    assert_eq!(map_x_dense(&a.numerator, &a.denominator, x), Some(t.x));
                }
                println!("{}: endpoint={:?}, E lifts={}, E1 lifts={}, final curve=({},{}), F=product verified",a.name,t,counts[0],counts[1],end.a,end.b);
            }
            if ci > 0 {
                json.push(',');
            }
            write!(json,"{{\"source_scalar\":{},\"T\":{},\"chi\":{},\"accepted_E_x\":{},\"accepted_E1_x\":{},\"V\":",scalar,pointjson(Some(t)),t.x,counts[0],counts[1]).unwrap();
            json_nums(&mut json, &xs);
            json.push('}');
            writeln!(
                csv,
                "{},{},{},{},{},{}",
                a.name,
                scalar,
                t.x,
                counts[0],
                counts[1],
                xs.len()
            )
            .unwrap();
        }
        json.push_str("],");
        polynomial_json(&mut json, "numerator", &a.numerator);
        json.push(',');
        polynomial_json(&mut json, "denominator", &a.denominator);
        json.push('}');
        // Off-coset values and poles verify all representations agree, not only their designated roots.
        let mut probe_inputs = (0..256).collect::<Vec<u64>>();
        probe_inputs.extend(a.kernel.iter().flatten().map(|q| q.x));
        for x in probe_inputs {
            let value = map_x_affine(&a.maps, x);
            assert_eq!(map_x_projective(&a.maps, x), value);
            assert_eq!(map_x_degree4(&a.degree4, x), value);
            assert_eq!(map_x_dense(&a.numerator, &a.denominator, x), value);
        }
    }
    write!(json,"],\"checks\":{{\"kernel_cardinality_each\":1024,\"all_kernel_points_map_to_infinity\":true,\"full_kernel_velu_codomain_matches_chain\":true,\"cosets_per_arm\":{},\"all_coset_points_map_to_T\":true,\"all_T_non_2torsion\":true,\"all_coset_x_coordinates_distinct\":true,\"initial_coset_F_equals_product\":true,\"degree2_degree4_dense_evaluators_agree\":true}},\"timings\":null}}",sample_scalars.len()).unwrap();
    let stem = if census {
        "kernel_census"
    } else {
        "kernel_correctness_fixtures"
    };
    fs::write(out.join(format!("{stem}.json")), json).unwrap();
    fs::write(out.join(format!("{stem}.csv")), csv).unwrap();
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert!(args==["--unit-fixtures"]||args==["--census"],"Use --unit-fixtures for focused correctness checks; --census requires experiment admission.");
    let census = args == ["--census"];
    let out = std::path::Path::new(".");
    let arms = arms();
    verify_and_write(&arms, out, census);
    println!("Arithmetic checks completed; timing and relation-solving benchmarks not run.");
}
