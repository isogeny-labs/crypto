#[path = "../canonical-root/fast.rs"]
mod fast;
#[path = "../canonical-root/root.rs"]
mod root;
use hec_native_ic::{counted, fp::Fp, jac::Curve, poly::Poly};
fn order(x: u64, f: &Fp) -> u64 {
    assert_ne!(x, 0);
    let mut v = 1;
    for n in 1..f.p {
        v = f.mul(v, x);
        if v == 1 {
            return n;
        }
    }
    unreachable!()
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
fn rank(mut a: Vec<[u64; 6]>, f: &Fp) -> usize {
    let mut r = 0;
    for j in 0..6 {
        let Some(i) = (r..a.len()).find(|&i| a[i][j] != 0) else {
            continue;
        };
        a.swap(i, r);
        let inv = f.inv(a[r][j]);
        for k in j..6 {
            a[r][k] = f.mul(a[r][k], inv);
        }
        for i in 0..a.len() {
            if i == r {
                continue;
            }
            let z = a[i][j];
            for k in j..6 {
                a[i][k] = f.sub(a[i][k], f.mul(z, a[r][k]));
            }
        }
        r += 1;
        if r == a.len() {
            break;
        }
    }
    r
}
fn pairing(a: &[u64; 6], b: &[u64; 6], f: &Fp) -> u64 {
    [(0, 2), (1, 3), (4, 5)].into_iter().fold(0, |s, (i, j)| {
        f.add(s, f.sub(f.mul(a[i], b[j]), f.mul(a[j], b[i])))
    })
}
fn action(a: [u64; 6], d: [u64; 6], f: &Fp) -> [u64; 6] {
    std::array::from_fn(|i| f.mul(a[i], d[i]))
}
fn main() {
    counted::disable();
    let f = Fp::new(659);
    let cv = Curve::new(Fp::new(421), Poly::from_coeffs(&[2, 0, 0, 0, 0, 0, 0, 1]));
    let (_, _, coeff) = fast::prepare(&cv);
    let z = root::power_map(&f, 7).0;
    let mut eig = std::collections::BTreeMap::<u64, Vec<u64>>::new();
    for e in 1..7 {
        let r = f.pow(z, e);
        let mut value = 0;
        for &a in coeff.iter().rev() {
            value = f.add(f.mul(value, r), a.rem_euclid(659) as u64);
        }
        eig.entry(value).or_default().push(r);
    }
    assert_eq!(
        eig.keys().copied().collect::<Vec<_>>(),
        vec![219, 357, 388, 424]
    );
    assert_eq!(eig[&219].len(), 2);
    assert_eq!(eig[&357].len(), 2);
    for (&lambda, rs) in &eig {
        println!(
            "EIGEN\tlambda={lambda}\tmultiplicity={}\torder={}\tsigma={rs:?}",
            rs.len(),
            order(lambda, &f)
        );
    }
    let degree = 659u64.pow(3);
    println!(
        "DEGREE\tisogeny={degree}\tmu659_extension_degree={}",
        order(421, &f)
    );
    let a = eig[&219][0];
    let b = eig[&219][1];
    assert_ne!(a, b);
    let mut total = 0;
    for gamma in [388, 424] {
        let delta = f.mul(421, f.inv(gamma));
        let r = eig[&gamma][0];
        let sigma = [a, b, f.inv(a), f.inv(b), r, f.inv(r)];
        let phi = [219, 219, 357, 357, gamma, delta];
        assert_eq!(
            sigma
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            6
        );
        for i in 0..6 {
            let mut value = 0;
            for &c in coeff.iter().rev() {
                value = f.add(f.mul(value, sigma[i]), c.rem_euclid(659) as u64);
            }
            assert_eq!(value, phi[i]);
        }
        let orders = [order(219, &f), order(357, &f), order(gamma, &f)];
        let m = orders.into_iter().fold(1, lcm);
        let mut seen = std::collections::BTreeSet::new();
        for t in 1..659 {
            let h = vec![
                [1, t, 0, 0, 0, 0],
                [0, 0, f.neg(t), 1, 0, 0],
                [0, 0, 0, 0, 1, 0],
            ];
            assert_eq!(rank(h.clone(), &f), 3);
            for u in &h {
                for v in &h {
                    assert_eq!(pairing(u, v, &f), 0)
                }
            }
            let mut frob = h.clone();
            for &u in &h {
                frob.push(action(u, phi, &f));
            }
            assert_eq!(rank(frob, &f), 3);
            let mut rot = h.clone();
            rot.push(action(h[0], sigma, &f));
            assert_eq!(rank(rot, &f), 4);
            // RREF identifies U by its unique normalized slope t; distinct t gives distinct projection line.
            assert!(seen.insert(t));
            for &n in &orders {
                assert_eq!(m % n, 0)
            }
            if t == 1 {
                println!("REPRESENTATIVE\tgamma={gamma}\tslope={t}\tbasis={h:?}\tsigma={sigma:?}\tfrobenius={phi:?}\tall_points_field_degree={m}");
            }
            total += 1;
        }
        assert_eq!(seen.len(), 658);
        let mut exact = std::collections::BTreeMap::<u64, u64>::new();
        let mut orbits = 0u64;
        let mut weighted = 0u64;
        for d in 1..=m {
            if m % d != 0 {
                continue;
            }
            let dimension = orders.iter().filter(|&&n| d % n == 0).count();
            let fixed = 659u64.pow(dimension as u32);
            let smaller = exact
                .iter()
                .filter(|(e, _)| d % **e == 0)
                .map(|(_, v)| *v)
                .sum::<u64>();
            let value = fixed - smaller;
            exact.insert(d, value);
            assert_eq!(value % d, 0);
            orbits += value / d;
            weighted += value;
            println!("POINT_ORBITS\tgamma={gamma}\tdegree={d}\tfixed_points={fixed}\texact_degree_points={value}\torbits={}",value/d);
        }
        assert_eq!(weighted, degree);
        assert_eq!(exact.values().sum::<u64>(), degree);
        // zero is a degree-one orbit; remove it only from nonzero-kernel reporting.
        println!("FAMILY\tgamma={gamma}\tkernels=658\torders={orders:?}\tall_points_field_degree={m}\tnonzero_point_orbits={}\tnonzero_points={}",orbits-1,degree-1);
    }
    assert_eq!(total, 1316);
    eprintln!("PASS abstract_non_O_stable_kernels=1316 rank_isotropy_frobenius_sigma_checks=true exact_point_orbit_counts=true geometric_torsion_points_not_constructed=true independent_validation=false speed_claim=false");
}
