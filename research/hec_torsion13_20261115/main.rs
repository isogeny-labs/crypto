mod arith;
mod orders;
use arith::{base, Curve, Div, Field, Pol, E, O, P};
use hec_native_ic::{fp::Fp, jac, poly::Poly, rng::Rng};
use std::{collections::HashSet, fs};
fn fixture(path: &str) -> (jac::Curve, jac::Div, u64) {
    let text = fs::read_to_string(path).unwrap();
    let mut it = text.split_whitespace();
    assert_eq!(it.next(), Some("FIXTURE"));
    let mut take = || it.next().unwrap().parse::<u64>().unwrap();
    assert_eq!(take(), 3);
    assert_eq!(take(), P);
    assert_eq!(take(), 0);
    let _ = take();
    let ell = take();
    assert_eq!(ell, 1028893);
    let mut ps = Vec::new();
    for _ in 0..3 {
        let n = take() as usize;
        let c: Vec<_> = (0..n).map(|_| take()).collect();
        ps.push(Poly::from_coeffs(&c));
    }
    let c = jac::Curve::new(Fp::new(P), ps[0]);
    let d = jac::Div { u: ps[1], v: ps[2] };
    assert!(c.is_valid(&d) && !c.is_zero(&d));
    assert_eq!(c.mul(&d, ell), c.zero());
    (c, d, ell)
}
fn embed(d: &jac::Div) -> Div {
    Div {
        u: Pol::embed(&d.u),
        v: Pol::embed(&d.v),
    }
}
fn random(rng: &mut Rng) -> E {
    std::array::from_fn(|_| rng.below(P))
}
fn controls(c: &Curve, refc: &jac::Curve, d1: &jac::Div, ell: u64) {
    let f = &c.field;
    let mut rng = Rng::new(2026111501300);
    for _ in 0..64 {
        let mut a = random(&mut rng);
        a[0] = 1 + rng.below(P - 1);
        let b = random(&mut rng);
        let d = random(&mut rng);
        assert_eq!(f.mul(a, f.inv(a)), O);
        assert_eq!(f.inv(a), f.pow(a, f.q - 2));
        assert_eq!(f.mul(a, f.add(b, d)), f.add(f.mul(a, b), f.mul(a, d)));
        assert_eq!(f.mul(f.mul(a, b), d), f.mul(a, f.mul(b, d)));
        let mut frob = a;
        for _ in 0..13 {
            frob = f.frob(frob)
        }
        assert_eq!(frob, a);
        assert_eq!(f.pow(a, f.q), a);
        let square = f.mul(a, a);
        let root = f.pow(square, (f.q + 1) / 4);
        assert_eq!(f.mul(root, root), square);
    }
    for i in 0..128 {
        let (a, b) = match i {
            0 => (0, 0),
            1 => (0, 1),
            2 => (1, ell - 1),
            3 => (7, 7),
            _ => (rng.below(1000), rng.below(1000)),
        };
        let ra = refc.mul(d1, a);
        let rb = refc.mul(d1, b);
        let ea = c.small(&embed(d1), a);
        let eb = c.small(&embed(d1), b);
        assert_eq!(ea, embed(&ra));
        assert_eq!(eb, embed(&rb));
        assert_eq!(c.add(&ea, &eb), embed(&refc.add(&ra, &rb)));
        assert_eq!(c.neg(&ea), embed(&refc.neg(&ra)));
        assert_eq!(c.add(&ea, &ea), embed(&refc.add(&ra, &ra)));
    }
    println!("CONTROLS field_elements=64 embedded_scalar_pairs=128 pass=true");
}
fn kernels(c: &Curve, p: &Div) -> Vec<Div> {
    let q = c.frob(p);
    let r = c.frob(&q);
    let mut out = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                out.push(c.add(&c.add(&c.small(p, i), &c.small(&q, j)), &c.small(&r, k)));
            }
        }
    }
    out
}
fn verify(c: &Curve, ks: &[Vec<Div>; 2]) -> Result<(), String> {
    let mut sets = Vec::new();
    for k in 0..2 {
        if ks[k].len() != 27 {
            return Err("wrong count".into());
        }
        let set: HashSet<_> = ks[k].iter().cloned().collect();
        if set.len() != 27 {
            return Err("duplicate".into());
        }
        let factor = if k == 0 { [2, 2, 0, 1] } else { [2, 0, 1, 1] };
        for p in &ks[k] {
            if !c.valid(p) {
                return Err("invalid Mumford divisor".into());
            }
            if !c.is_zero(&c.small(p, 3)) {
                return Err("not 3-torsion".into());
            }
            if !c.is_zero(&c.operator(p, &factor)) {
                return Err("wrong primary".into());
            }
            if !set.contains(&c.frob(p)) {
                return Err("not Frobenius stable".into());
            }
        }
        for p in &ks[k] {
            for q in &ks[k] {
                if !set.contains(&c.add(p, q)) {
                    return Err("not closed".into());
                }
            }
        }
        sets.push(set);
    }
    if sets[0].intersection(&sets[1]).count() != 1
        || !sets[0].contains(&c.zero())
        || !sets[1].contains(&c.zero())
    {
        return Err("wrong intersection".into());
    }
    let mut full = HashSet::new();
    for a in &ks[0] {
        for b in &ks[1] {
            let x = c.add(a, b);
            if !c.is_zero(&c.small(&x, 3)) {
                return Err("cross sum not torsion".into());
            }
            full.insert(x);
        }
    }
    if full.len() != 729 {
        return Err("wrong full 3-torsion size".into());
    }
    let fp = Fp::new(3);
    let f = Poly::from_coeffs(&[2, 2, 0, 1]);
    let g = Poly::from_coeffs(&[2, 0, 1, 1]);
    assert_eq!(Poly::gcd(&f, &g, &fp), Poly::one());
    // For this Jacobian, p=1 mod3. The two distinct dual simple Frobenius
    // modules are isotropic for the 3-Weil pairing. Dimension3 is maximal.
    Ok(())
}
fn negatives(c: &Curve, ks: &[Vec<Div>; 2], d1: &jac::Div) {
    let mut bad = ks.clone();
    bad[0][0].u.0[0] = base(2);
    assert!(verify(c, &bad).is_err());
    let mut bad = ks.clone();
    bad[0][1] = bad[0][0].clone();
    assert!(verify(c, &bad).is_err());
    let mut bad = ks.clone();
    bad[0][1] = ks[1].iter().find(|p| !c.is_zero(p)).unwrap().clone();
    assert!(verify(c, &bad).is_err());
    let mut bad = ks.clone();
    bad[0][1] = embed(d1);
    assert!(verify(c, &bad).is_err());
    println!(
        "NEGATIVES nonmonic=true duplicate=true wrong_primary=true public_D1_not_torsion=true"
    );
}
fn emit(k: usize, i: usize, d: &Div) {
    print!("POINT {k} {i}");
    for p in [&d.u, &d.v] {
        print!(" {}", p.0.len());
        for a in &p.0 {
            for x in a {
                print!(" {x}");
            }
        }
    }
    println!();
}
fn read_cert(path: &str, refc: &jac::Curve) -> (Curve, [Vec<Div>; 2]) {
    let text = fs::read_to_string(path).unwrap();
    let header = text.lines().find(|l| l.starts_with("FIELD ")).unwrap();
    let hs: Vec<u64> = header
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(hs.len(), 3);
    assert_eq!((hs[0], hs[1]), (P, 13));
    assert!((1..P).contains(&hs[2]));
    let field = Field {
        c: hs[2],
        q: (P as u128).pow(13),
    };
    assert!(field.irreducible());
    let c = Curve {
        field,
        f: Pol::embed(&refc.f),
    };
    let mut ks = [Vec::new(), Vec::new()];
    for line in text.lines().filter(|l| l.starts_with("POINT ")) {
        let mut t = line.split_whitespace().skip(1);
        let mut take = || t.next().unwrap().parse::<u64>().unwrap();
        let k = take() as usize;
        let i = take() as usize;
        assert!(k < 2);
        assert_eq!(i, ks[k].len());
        let mut ps = Vec::new();
        for _ in 0..2 {
            let n = take() as usize;
            assert!(n <= 4);
            let mut v = Vec::new();
            for _ in 0..n {
                let a = std::array::from_fn(|_| take());
                assert!(a.iter().all(|&x| x < P));
                v.push(a);
            }
            let p = Pol::new(v);
            assert_eq!(p.0.len(), n);
            ps.push(p);
        }
        assert_eq!(t.next(), None);
        ks[k].push(Div {
            u: ps[0].clone(),
            v: ps[1].clone(),
        });
    }
    (c, ks)
}
fn main() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124);
    });
    let a: Vec<_> = std::env::args().collect();
    if a[1] == "--verify" {
        let (refc, d1, _) = fixture(&a[3]);
        let (c, ks) = read_cert(&a[2], &refc);
        verify(&c, &ks).unwrap();
        negatives(&c, &ks, &d1);
        println!(
            "REPLAY kernels=2 points_each=27 full_torsion=729 pass=true shared_arithmetic=true"
        );
        return;
    }
    let (refc, d1, ell) = fixture(&a[1]);
    let field = Field::new();
    assert_eq!(field.q % 4, 3);
    let c = Curve {
        field,
        f: Pol::embed(&refc.f),
    };
    println!("FIELD {P} 13 {}", field.c);
    println!("FIELD_Q {}", field.q);
    controls(&c, &refc, &d1, ell);
    let (n, h, v3) = orders::extension_order();
    println!("ORDER {n} {h} {v3}");
    let mut rng = Rng::new(2026111501301);
    let mut found = None;
    for attempt in 0..16 {
        let mut source = None;
        for _ in 0..32 {
            let x = random(&mut rng);
            if field.frob(x) == x {
                continue;
            }
            let value = c.f.eval(x, &field);
            let y = field.pow(value, (field.q + 1) / 4);
            if field.mul(y, y) == value {
                source = Some((x, y));
                break;
            }
        }
        let Some((x, y)) = source else {
            println!("ATTEMPT {attempt} square_cap");
            continue;
        };
        let r = Div {
            u: Pol::new(vec![field.neg(x), O]),
            v: Pol::new(vec![y]),
        };
        assert!(c.valid(&r));
        assert!(c.is_zero(&c.scalar(&r, &n)));
        let mut q = c.scalar(&r, &h);
        if c.is_zero(&q) {
            println!("ATTEMPT {attempt} zero_cofactor");
            continue;
        }
        let mut depth = 0;
        loop {
            let next = c.small(&q, 3);
            if c.is_zero(&next) {
                break;
            }
            q = next;
            depth += 1;
            assert!(depth <= v3);
        }
        let p0 = c.operator(&q, &[1, 2, 2, 0, 1]);
        let p1 = c.operator(&q, &[0, 1, 1, 0, 2]);
        println!(
            "ATTEMPT {attempt} three_depth={depth} projection0_nonzero={} projection1_nonzero={}",
            !c.is_zero(&p0),
            !c.is_zero(&p1)
        );
        if c.is_zero(&p0) || c.is_zero(&p1) {
            continue;
        }
        println!("SAMPLE_X {x:?}");
        println!("SAMPLE_Y {y:?}");
        found = Some([kernels(&c, &p0), kernels(&c, &p1)]);
        break;
    }
    let Some(ks) = found else {
        println!("TERMINAL attempt_cap no_kernel_certificate");
        std::process::exit(2)
    };
    verify(&c, &ks).unwrap();
    negatives(&c, &ks, &d1);
    for k in 0..2 {
        for (i, p) in ks[k].iter().enumerate() {
            emit(k, i, p);
        }
    }
    println!("TERMINAL kernels=2 points_each=27 full_torsion=729 isotropy=nonselfdual_primary_certificate pass=true");
}
