use num_bigint::BigUint as Z;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{collections::HashMap, path::Path};
fn val(v: &Value) -> Z {
    v.to_string().parse().unwrap()
}
#[derive(Clone, Debug, PartialEq)]
struct J {
    v: Z,
    d: Z,
}
struct Field {
    p: Z,
}
impl Field {
    fn c(&self, n: u32) -> J {
        J {
            v: Z::from(n) % &self.p,
            d: Z::zero(),
        }
    }
    fn a(&self, x: &J, y: &J) -> J {
        J {
            v: (&x.v + &y.v) % &self.p,
            d: (&x.d + &y.d) % &self.p,
        }
    }
    fn s(&self, x: &J, y: &J) -> J {
        J {
            v: (&x.v + &self.p - &y.v) % &self.p,
            d: (&x.d + &self.p - &y.d) % &self.p,
        }
    }
    fn m(&self, x: &J, y: &J) -> J {
        J {
            v: &x.v * &y.v % &self.p,
            d: (&x.v * &y.d + &x.d * &y.v) % &self.p,
        }
    }
    fn scale(&self, x: &J, n: u32) -> J {
        self.m(x, &self.c(n))
    }
    fn pow(&self, x: &J, n: u32) -> J {
        let mut z = self.c(1);
        for _ in 0..n {
            z = self.m(&z, x)
        }
        z
    }
    fn inv(&self, x: &J) -> J {
        assert!(!x.v.is_zero());
        let v = x.v.modpow(&(&self.p - Z::from(2u32)), &self.p);
        let d = (&self.p - (&x.d * &v * &v % &self.p)) % &self.p;
        J { v, d }
    }
}
// Original division-polynomial recurrence, including 2Y on even indices.
// This differs from the historical parity-normalized recurrence in jets.py.
fn psi(n: &Z, base: &[J], inv2y: &J, f: &Field, memo: &mut HashMap<Z, J>) -> J {
    if let Some(v) = memo.get(n) {
        return v.clone();
    }
    let five = Z::from(5u32);
    let v = if n < &five {
        base[n.to_u32_digits().first().copied().unwrap_or(0) as usize].clone()
    } else {
        let m = n >> 1usize;
        if n.bit(0) {
            let a = f.m(
                &psi(&(&m + Z::from(2u32)), base, inv2y, f, memo),
                &f.pow(&psi(&m, base, inv2y, f, memo), 3),
            );
            let b = f.m(
                &psi(&(&m - Z::one()), base, inv2y, f, memo),
                &f.pow(&psi(&(&m + Z::one()), base, inv2y, f, memo), 3),
            );
            f.s(&a, &b)
        } else {
            let a = f.m(
                &psi(&(&m + Z::from(2u32)), base, inv2y, f, memo),
                &f.pow(&psi(&(&m - Z::one()), base, inv2y, f, memo), 2),
            );
            let b = f.m(
                &psi(&(&m - Z::from(2u32)), base, inv2y, f, memo),
                &f.pow(&psi(&(&m + Z::one()), base, inv2y, f, memo), 2),
            );
            f.m(&f.m(&psi(&m, base, inv2y, f, memo), inv2y), &f.s(&a, &b))
        }
    };
    memo.insert(n.clone(), v.clone());
    v
}
fn jet(n: &Z, x: &Z, y: &Z, a: &Z, b: &Z, f: &Field) -> (J, usize) {
    let x = J {
        v: x.clone(),
        d: Z::one(),
    };
    let aj = J {
        v: a.clone(),
        d: Z::zero(),
    };
    let bj = J {
        v: b.clone(),
        d: Z::zero(),
    };
    let yj = J {
        v: y.clone(),
        d: ((&Z::from(3u32) * &x.v * &x.v + a) % &f.p)
            * (&Z::from(2u32) * y % &f.p).modpow(&(&f.p - Z::from(2u32)), &f.p)
            % &f.p,
    };
    let two = f.scale(&yj, 2);
    let p3 = f.s(
        &f.a(
            &f.a(
                &f.scale(&f.pow(&x, 4), 3),
                &f.scale(&f.m(&aj, &f.pow(&x, 2)), 6),
            ),
            &f.scale(&f.m(&bj, &x), 12),
        ),
        &f.pow(&aj, 2),
    );
    let p4poly = f.s(
        &f.s(
            &f.s(
                &f.a(
                    &f.a(&f.pow(&x, 6), &f.scale(&f.m(&aj, &f.pow(&x, 4)), 5)),
                    &f.scale(&f.m(&bj, &f.pow(&x, 3)), 20),
                ),
                &f.scale(&f.m(&f.pow(&aj, 2), &f.pow(&x, 2)), 5),
            ),
            &f.scale(&f.m(&f.m(&aj, &bj), &x), 4),
        ),
        &f.a(&f.scale(&f.pow(&bj, 2), 8), &f.pow(&aj, 3)),
    );
    let base = [
        f.c(0),
        f.c(1),
        two.clone(),
        p3,
        f.scale(&f.m(&yj, &p4poly), 4),
    ];
    let mut memo = HashMap::new();
    let out = psi(n, &base, &f.inv(&two), f, &mut memo);
    (out, memo.len())
}
fn ceil_sqrt(x: &Z) -> Z {
    if x.is_zero() {
        return Z::zero();
    }
    let mut a = Z::one() << ((x.bits() + 1) / 2) as usize;
    loop {
        let b = (&a + x / &a) >> 1usize;
        if b >= a {
            break;
        }
        a = b;
    }
    if &a * &a < *x {
        a + Z::one()
    } else {
        a
    }
}
pub fn verify(root: &Path) -> Value {
    let c = super::load(root.join("P192-certificate.json"));
    let input = super::load(root.join("../round52-p192/input.json"));
    for name in ["p", "r", "a", "b"] {
        assert_eq!(c[name], input[name]);
    }
    assert_eq!(c["normalization_cases"].as_array().unwrap().len(), 3);
    for (i, name) in ["G", "Q"].iter().enumerate() {
        assert_eq!(c["normalization_cases"][i]["point"], input[*name]);
    }
    let p = val(&input["p"]);
    let r = val(&input["r"]);
    let a = val(&input["a"]);
    let b = val(&input["b"]);
    let f = Field { p: p.clone() };
    let mut rows = vec![];
    for case in c["normalization_cases"].as_array().unwrap() {
        let x = val(&case["point"][0]);
        let y = val(&case["point"][1]);
        assert_eq!(&y * &y % &p, (&x * &x * &x + &a * &x + &b) % &p);
        let (out, memo) = jet(&r, &x, &y, &a, &b, &f);
        assert!(out.v.is_zero());
        assert!(!out.d.is_zero());
        assert_eq!(out.d, val(&case["psi_derivative"]));
        let k = Z::from(4u32) * &y * &y * &out.d * &out.d % &p;
        assert_eq!(k, val(&case["kappa"]));
        rows.push(json!({"point_x":x.to_string(),"psi_value":"0","psi_derivative":out.d.to_string(),"kappa":k.to_string(),"original_recurrence_memo_indices":memo}));
    }
    assert_eq!(
        val(&c["normalization_cases"][0]["point"][0]),
        val(&input["G"][0])
    );
    assert_eq!(
        val(&c["normalization_cases"][1]["point"][0]),
        val(&input["Q"][0])
    );
    // Check the third point really is 2G with separate affine formulas.
    let gx = val(&input["G"][0]);
    let gy = val(&input["G"][1]);
    let slope = (Z::from(3u32) * &gx * &gx + &a) % &p
        * (Z::from(2u32) * &gy % &p).modpow(&(&p - Z::from(2u32)), &p)
        % &p;
    let xx = (&slope * &slope + Z::from(2u32) * &p - Z::from(2u32) * &gx) % &p;
    let yy = (&slope * (&gx + &p - &xx) + &p - &gy) % &p;
    assert_eq!(xx, val(&c["normalization_cases"][2]["point"][0]));
    assert_eq!(yy, val(&c["normalization_cases"][2]["point"][1]));
    let d = Z::from(191120u32);
    assert_eq!((&r - Z::one()) % &d, Z::zero());
    let l = (&r - Z::one()) / &d;
    assert_eq!(l, val(&c["auxiliary_input_candidate"]["L"]));
    assert_eq!(
        ceil_sqrt(&l),
        val(&c["auxiliary_input_candidate"]["sqrtL_ceil"])
    );
    assert_eq!(
        ceil_sqrt(&d),
        val(&c["auxiliary_input_candidate"]["sqrtd_ceil"])
    );
    json!({"normalization_cases":rows,"d":191120,"action_queries":191119,"sqrtL_ceil":ceil_sqrt(&l).to_string(),"sqrtd_ceil":438,"numerator":"not constructed; no P192 action or Cheon run"})
}
