//! Full-width finite-field and quadratic-point checks for the Frobenius normalizer.
use num_bigint::BigUint as Z;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{fs, path::Path};
type K = (Z, Z);
type Pt = Option<(K, K)>;
struct E {
    p: Z,
    a: Z,
    b: Z,
}
impl E {
    fn c(&self, n: u32) -> K {
        (Z::from(n) % &self.p, Z::zero())
    }
    fn addk(&self, x: &K, y: &K) -> K {
        ((&x.0 + &y.0) % &self.p, (&x.1 + &y.1) % &self.p)
    }
    fn negk(&self, x: &K) -> K {
        ((&self.p - &x.0) % &self.p, (&self.p - &x.1) % &self.p)
    }
    fn subk(&self, x: &K, y: &K) -> K {
        self.addk(x, &self.negk(y))
    }
    fn mulk(&self, x: &K, y: &K) -> K {
        (
            (&x.0 * &y.0 + &self.p - (&x.1 * &y.1 % &self.p)) % &self.p,
            (&x.0 * &y.1 + &x.1 * &y.0) % &self.p,
        )
    }
    fn inverse(&self, x: &K) -> K {
        let norm = (&x.0 * &x.0 + &x.1 * &x.1) % &self.p;
        assert!(!norm.is_zero());
        let n = norm.modpow(&(&self.p - Z::from(2u32)), &self.p);
        (
            &x.0 * &n % &self.p,
            (&self.p - (&x.1 * &n % &self.p)) % &self.p,
        )
    }
    fn quotient(&self, x: &K, y: &K) -> K {
        self.mulk(x, &self.inverse(y))
    }
    fn frob(&self, p: &Pt) -> Pt {
        p.as_ref().map(|(x, y)| {
            (
                (x.0.clone(), (&self.p - &x.1) % &self.p),
                (y.0.clone(), (&self.p - &y.1) % &self.p),
            )
        })
    }
    fn neg(&self, p: &Pt) -> Pt {
        p.as_ref().map(|(x, y)| (x.clone(), self.negk(y)))
    }
    fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let ((x, y), (u, v)) = match (p, q) {
            (None, _) => return q.clone(),
            (_, None) => return p.clone(),
            (Some(p), Some(q)) => (p, q),
        };
        if x == u && self.addk(y, v) == self.c(0) {
            return None;
        }
        let slope = if p == q {
            self.quotient(
                &self.addk(
                    &self.mulk(&self.c(3), &self.mulk(x, x)),
                    &(self.a.clone(), Z::zero()),
                ),
                &self.mulk(&self.c(2), y),
            )
        } else {
            self.quotient(&self.subk(v, y), &self.subk(u, x))
        };
        let z = self.subk(&self.subk(&self.mulk(&slope, &slope), x), u);
        Some((
            z.clone(),
            self.subk(&self.mulk(&slope, &self.subk(x, &z)), y),
        ))
    }
    fn on(&self, p: &Pt) -> bool {
        p.as_ref()
            .map(|(x, y)| {
                self.mulk(y, y)
                    == self.addk(
                        &self.addk(
                            &self.mulk(&self.mulk(x, x), x),
                            &self.mulk(&(self.a.clone(), Z::zero()), x),
                        ),
                        &(self.b.clone(), Z::zero()),
                    )
            })
            .unwrap_or(true)
    }
}
fn z(v: &Value) -> Z {
    v.to_string().parse().unwrap()
}
fn point(v: &Value) -> Pt {
    Some(((z(&v[0]), Z::zero()), (z(&v[1]), Z::zero())))
}
fn code(p: &Pt) -> Value {
    p.as_ref()
        .map(|(x, y)| {
            json!([
                [x.0.to_string(), x.1.to_string()],
                [y.0.to_string(), y.1.to_string()]
            ])
        })
        .unwrap_or(Value::Null)
}
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap()
}
fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("usage: p192 ROUND56_DIRECTORY");
    let root = Path::new(&arg);
    let input = load(root.join("../round52-p192/input.json"));
    let old = load(root.join("../round55-p192/P192-certificate.json"));
    let shift = load(root.join("../round54-p192/P192-shift.json"));
    let e = E {
        p: z(&input["p"]),
        a: z(&input["a"]),
        b: z(&input["b"]),
    };
    assert_eq!(&e.p % Z::from(4u32), Z::from(3u32));
    let s = Some((
        (z(&shift["S"][0][0]), z(&shift["S"][0][1])),
        (z(&shift["S"][1][0]), z(&shift["S"][1][1])),
    ));
    assert!(e.on(&s));
    assert_eq!(e.frob(&s), e.neg(&s));
    let expected = e.add(&s, &s);
    let mut cases = vec![];
    for rec in old["normalization_cases"].as_array().unwrap() {
        let u = point(&rec["point"]);
        assert!(e.on(&u));
        let (x, y) = u.as_ref().unwrap();
        let f = (&x.0 * &x.0 * &x.0 + &e.a * &x.0 + &e.b) % &e.p;
        let compact = (x.0.modpow(&(Z::from(2u32) * &e.p + Z::one()), &e.p)
            + x.0.modpow(&(&e.p + Z::from(2u32)), &e.p)
            + &e.a * x.0.modpow(&e.p, &e.p)
            + &e.a * &x.0
            + Z::from(2u32) * &e.b
            + Z::from(2u32) * f.modpow(&((&e.p + Z::one()) / Z::from(2u32)), &e.p))
            % &e.p;
        let norm = Z::from(4u32) * &y.0 * &y.0 % &e.p;
        assert_eq!(compact, norm);
        assert!(!norm.is_zero());
        let p = e.add(&u, &s);
        assert!(e.on(&p));
        let pi = e.frob(&p);
        let image = e.add(&p, &e.neg(&pi));
        assert_eq!(image, expected);
        let (px, py) = p.as_ref().unwrap();
        let (xp, yp) = pi.as_ref().unwrap();
        let slope = e.quotient(&e.addk(yp, py), &e.subk(xp, px));
        let formula = e.subk(&e.subk(&e.mulk(&slope, &slope), xp), px);
        assert_eq!(formula, image.as_ref().unwrap().0);
        cases.push(json!({"rational_U":code(&u),"off_kernel_P":code(&p),"one_minus_frobenius_image":code(&image),"x_formula":code(&Some((formula,e.c(0))))[0],"raw_denominator_local_coefficients":[0,0,1],"compact_numerator_at_U":compact.to_string(),"normalization_4y2":norm.to_string()}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"all_checks_pass":true,"p":e.p.to_string(),"map":"1-Frobenius","degree":input["r"],"cases":cases,"quadratic_field":"W^2=-1","scope":"actual normalizer map and local leading coefficient; no P192 cyclic trace or marked action"})).unwrap());
}
