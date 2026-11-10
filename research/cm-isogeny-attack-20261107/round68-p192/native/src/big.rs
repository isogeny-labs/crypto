//! Full-width finite-field and quadratic-point checks for the Frobenius normalizer.
use num_bigint::BigUint as Z;
use std::sync::atomic::{AtomicU64, Ordering};
static FM: AtomicU64 = AtomicU64::new(0);
static FI: AtomicU64 = AtomicU64::new(0);
static PA: AtomicU64 = AtomicU64::new(0);
pub fn counts() -> [u64; 3] {
    [
        FM.load(Ordering::Relaxed),
        FI.load(Ordering::Relaxed),
        PA.load(Ordering::Relaxed),
    ]
}
pub fn diff(a: [u64; 3], b: [u64; 3]) -> [u64; 3] {
    [b[0] - a[0], b[1] - a[1], b[2] - a[2]]
}

use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{fs, path::Path};
pub type K = (Z, Z);
pub type Pt = Option<(K, K)>;
pub struct E {
    pub p: Z,
    pub a: Z,
    pub b: Z,
}
impl E {
    pub fn c(&self, n: u32) -> K {
        (Z::from(n) % &self.p, Z::zero())
    }
    pub fn addk(&self, x: &K, y: &K) -> K {
        ((&x.0 + &y.0) % &self.p, (&x.1 + &y.1) % &self.p)
    }
    pub fn negk(&self, x: &K) -> K {
        ((&self.p - &x.0) % &self.p, (&self.p - &x.1) % &self.p)
    }
    pub fn subk(&self, x: &K, y: &K) -> K {
        self.addk(x, &self.negk(y))
    }
    pub fn mulk(&self, x: &K, y: &K) -> K {
        FM.fetch_add(1, Ordering::Relaxed);
        (
            (&x.0 * &y.0 + &self.p - (&x.1 * &y.1 % &self.p)) % &self.p,
            (&x.0 * &y.1 + &x.1 * &y.0) % &self.p,
        )
    }
    pub fn inverse(&self, x: &K) -> K {
        FI.fetch_add(1, Ordering::Relaxed);
        let norm = (&x.0 * &x.0 + &x.1 * &x.1) % &self.p;
        assert!(!norm.is_zero());
        let n = norm.modpow(&(&self.p - Z::from(2u32)), &self.p);
        (
            &x.0 * &n % &self.p,
            (&self.p - (&x.1 * &n % &self.p)) % &self.p,
        )
    }
    pub fn quotient(&self, x: &K, y: &K) -> K {
        self.mulk(x, &self.inverse(y))
    }
    pub fn frob(&self, p: &Pt) -> Pt {
        p.as_ref().map(|(x, y)| {
            (
                (x.0.clone(), (&self.p - &x.1) % &self.p),
                (y.0.clone(), (&self.p - &y.1) % &self.p),
            )
        })
    }
    pub fn neg(&self, p: &Pt) -> Pt {
        p.as_ref().map(|(x, y)| (x.clone(), self.negk(y)))
    }
    pub fn add(&self, p: &Pt, q: &Pt) -> Pt {
        PA.fetch_add(1, Ordering::Relaxed);
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
    pub fn on(&self, p: &Pt) -> bool {
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
pub fn z(v: &Value) -> Z {
    v.to_string().parse().unwrap()
}
pub fn point(v: &Value) -> Pt {
    Some(((z(&v[0]), Z::zero()), (z(&v[1]), Z::zero())))
}
pub fn code(p: &Pt) -> Value {
    p.as_ref()
        .map(|(x, y)| {
            json!([
                [x.0.to_string(), x.1.to_string()],
                [y.0.to_string(), y.1.to_string()]
            ])
        })
        .unwrap_or(Value::Null)
}
pub fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap()
}
