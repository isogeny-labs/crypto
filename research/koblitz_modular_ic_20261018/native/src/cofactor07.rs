//! Expanded binary supersingular[19] cover equations and a kernel certificate.
use crate::{large_field::*, quotients07::EC, reverse06::dp, round02, round04};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::json;
use std::path::Path;
fn pmul(a: &BigUint, b: &BigUint) -> BigUint {
    let mut z = BigUint::zero();
    for i in 0..b.bits() {
        if b.bit(i) {
            z ^= a << i;
        }
    }
    z
}
fn pdiv(a: &BigUint, b: &BigUint) -> (BigUint, BigUint) {
    assert!(!b.is_zero());
    let mut r = a.clone();
    let mut q = BigUint::zero();
    while !r.is_zero() && r.bits() >= b.bits() {
        let i = r.bits() - b.bits();
        q ^= BigUint::one() << i;
        r ^= b << i;
    }
    (q, r)
}
fn pgcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let r = pdiv(&a, &b).1;
        a = b;
        b = r;
    }
    a
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Rat {
    n: BigUint,
    d: BigUint,
}
impl Rat {
    fn new(n: BigUint, d: BigUint) -> Self {
        assert!(!d.is_zero());
        let g = pgcd(n.clone(), d.clone());
        let n = pdiv(&n, &g).0;
        let d = pdiv(&d, &g).0;
        assert!(n.bits() <= 2049 && d.bits() <= 2049);
        Self { n, d }
    }
    fn v(n: u64) -> Self {
        Self {
            n: n.into(),
            d: BigUint::one(),
        }
    }
    fn add(&self, b: &Self) -> Self {
        Self::new(
            pmul(&self.n, &b.d) ^ pmul(&b.n, &self.d),
            pmul(&self.d, &b.d),
        )
    }
    fn mul(&self, b: &Self) -> Self {
        Self::new(pmul(&self.n, &b.n), pmul(&self.d, &b.d))
    }
    fn div(&self, b: &Self) -> Self {
        Self::new(pmul(&self.n, &b.d), pmul(&self.d, &b.n))
    }
    fn sq(&self) -> Self {
        self.mul(self)
    }
}
fn double(p: &(Rat, Rat)) -> (Rat, Rat) {
    let l = p.0.sq().add(&Rat::v(1));
    let x = l.sq();
    let y = p.1.add(&l.mul(&p.0.add(&x))).add(&Rat::v(1));
    (x, y)
}
fn plus(p: &(Rat, Rat), q: &(Rat, Rat)) -> (Rat, Rat) {
    let l = p.1.add(&q.1).div(&p.0.add(&q.0));
    let x = l.sq().add(&p.0).add(&q.0);
    let y = p.1.add(&l.mul(&p.0.add(&x))).add(&Rat::v(1));
    (x, y)
}
fn eval(f: &F, p: &BigUint, x: &E) -> E {
    let mut z = f.z();
    for i in (0..p.bits()).rev() {
        z = f.mul(&z, x);
        if p.bit(i) {
            z = z.add(&f.o());
        }
    }
    z
}
fn expanded(p: &BigUint) -> String {
    let mut terms = vec![];
    for i in (0..p.bits()).rev() {
        if p.bit(i) {
            terms.push(if i == 0 {
                "1".into()
            } else if i == 1 {
                "u".into()
            } else {
                format!("u^{i}")
            });
        }
    }
    if terms.is_empty() {
        "0".into()
    } else {
        terms.join(" + ")
    }
}
pub fn run(dir: &Path) {
    let p = (Rat::v(2), Rat::v(0));
    let mut q = p.clone();
    for bit in [0, 0, 1, 1] {
        q = double(&q);
        if bit == 1 {
            q = plus(&q, &p);
        }
    }
    let x = q.0;
    let r = q.1;
    let residual = x.sq().mul(&x).add(&x).add(&Rat::v(10)).add(&r.sq()).add(&r);
    assert_eq!(residual, Rat::v(0));
    assert_eq!((x.n.bits() - 1, x.d.bits() - 1), (361, 360));
    let dx3 = pmul(&pmul(&x.d, &x.d), &x.d);
    let dr2 = pmul(&r.d, &r.d);
    let nx3 = pmul(&pmul(&x.n, &x.n), &x.n);
    let one = pmul(&nx3, &dr2);
    let two = pmul(&pmul(&x.n, &pmul(&x.d, &x.d)), &dr2);
    let three = pmul(&pmul(&BigUint::from(10u64), &dx3), &dr2);
    let four = pmul(&(pmul(&r.n, &r.n) ^ pmul(&r.n, &r.d)), &dx3);
    assert!((&one ^ &two ^ &three ^ &four).is_zero());
    assert!(one.bits() <= 4097);
    let input = round04::read(&dir.join("q5-mixed-translation-atlases.json"));
    let degree = input["field"]["degree"].as_u64().unwrap() as u32;
    let terms: Vec<u32> = serde_json::from_value(input["field"]["low_terms"].clone()).unwrap();
    let f = F::new(degree, terms.iter().fold(0, |z, i| z | (1u64 << i)));
    let mut kernels = 0;
    let mut checks = vec![];
    for trace in [-2, 2] {
        let ec = EC {
            f: &f,
            ss: true,
            c: if trace == -2 { 0 } else { 1 },
            trace,
        };
        let record = input["records"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["supersingular_trace"] == trace)
            .unwrap();
        let mut count = 0;
        for cp in record["kernel"].as_array().unwrap() {
            let p = dp(&f, &cp["point"][4]);
            if let P::Affine { x: xx, .. } = &p {
                assert_eq!(eval(&f, &x.d, xx), f.z());
                assert_eq!(eval(&f, &r.d, xx), f.z());
                assert_ne!(eval(&f, &x.n, xx), f.z());
            }
            assert_eq!(ec.times(&p, 19), P::Infinity);
            count += 1;
        }
        kernels += count;
        let mut samples = vec![];
        for z in 0..4096 {
            let Some(p) = ec.point(&f.e(z)) else { continue };
            let P::Affine { x: xx, y: yy } = &p else {
                continue;
            };
            let d = eval(&f, &x.d, xx);
            let dr = eval(&f, &r.d, xx);
            if d.is_zero() || dr.is_zero() {
                continue;
            }
            let image = P::Affine {
                x: f.mul(&eval(&f, &x.n, xx), &f.inv(&d)),
                y: yy.add(&f.mul(&eval(&f, &r.n, xx), &f.inv(&dr))),
            };
            assert!(ec.valid(&image));
            assert_eq!(image, ec.times(&p, 19));
            samples.push(json!({"point":pj(&p),"image":pj(&image)}));
            if samples.len() == 32 {
                break;
            }
        }
        assert_eq!(samples.len(), 32);
        checks.push(json!({"trace":trace,"kernel_points":count,"samples":samples}));
    }
    let lean=format!("import KoblitzRound07\nnamespace KoblitzRound07Cofactor\nset_option maxRecDepth 200000\nset_option maxHeartbeats 0\ndef pmul (a b : ℕ) : ℕ :=\n  (List.range (b.log2+1)).foldl (fun z i => if b.testBit i then Nat.xor z (a*2^i) else z) 0\ndef nx : ℕ := {}\ndef dx : ℕ := {}\ndef nr : ℕ := {}\ndef dr : ℕ := {}\n/-- Exact cleared polynomial identity, encoded by binary coefficient masks. -/\ntheorem binary_coordinate_identity :\n  Nat.xor (Nat.xor (Nat.xor\n    (pmul (pmul (pmul nx nx) nx) (pmul dr dr))\n    (pmul (pmul nx (pmul dx dx)) (pmul dr dr)))\n    (pmul (pmul 10 (pmul (pmul dx dx) dx)) (pmul dr dr)))\n    (pmul (Nat.xor (pmul nr nr) (pmul nr dr)) (pmul (pmul dx dx) dx))=0 := by\n  decide +kernel\nend KoblitzRound07Cofactor\n#print axioms KoblitzRound07Cofactor.binary_coordinate_identity\n",x.n,x.d,r.n,r.d);
    round02::save(
        dir,
        "q5b-expanded-cofactor-cover.json",
        &json!({"schema":"koblitz.round07/q5b-v1","polynomial_encoding":"binary masks, bit i is coefficient of u^i over F2","Nx":x.n.to_str_radix(16),"Dx":x.d.to_str_radix(16),"Nr":r.n.to_str_radix(16),"Dr":r.d.to_str_radix(16),"degrees":[x.n.bits()-1,x.d.bits()-1,r.n.bits()-1,r.d.bits()-1],"equations":["v^2+v=u^3+u+c","Dx(u)*U+Nx(u)=0","Dr(u)*(V+v)+Nr(u)=0","V^2+V=U^3+U+c"],"exact_curve_identity":true,"kernel_checks":kernels,"pointwise_checks":checks,"mixed_invariant_image_ideal":"OPEN","scope":"expanded degree361 supersingular cofactor-cover chart equations for both traces; these supplement the mixed quotient atlas, not an expanded intrinsic five-dimensional quotient ideal"}),
    );
    let eq=format!("Expanded supersingular [19] cover, valid for c=0 and c=1 over F2.\nNx = {}\nDx = {}\nNr = {}\nDr = {}\n\nv^2+v=u^3+u+c\nDx*U+Nx=0\nDr*(V+v)+Nr=0\nV^2+V=U^3+U+c\n\nThis is the expanded cofactor cover used by the mixed determinant19 quotient atlas. The full mixed invariant image ideal remains open.\n",expanded(&x.n),expanded(&x.d),expanded(&r.n),expanded(&r.d));
    assert!(!dir.join("q5b-expanded-cofactor-equations.txt").exists());
    std::fs::write(dir.join("q5b-expanded-cofactor-equations.txt"), eq).unwrap();
    assert!(!dir.join("KoblitzRound07Cofactor.lean").exists());
    std::fs::write(dir.join("KoblitzRound07Cofactor.lean"), lean).unwrap();
    println!("Q5b:expanded [19] rational cover degrees361/360; exact identity,722 kernel and64 point checks PASS");
}
