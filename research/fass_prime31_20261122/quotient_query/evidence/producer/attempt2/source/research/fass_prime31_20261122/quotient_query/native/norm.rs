//! The existing semaev_selector binary Lucas recurrence evaluated directly
//! in F_p[x,t]/(F(x),G(t)). Explicit dense tensor states have dim(F)*dim(G)
//! scalar slots; the recurrence does not make those ring operations free.

use crate::kernel::{add, reduce_tensor, sub, tensor_mul, tensor_trim, Tensor};
use crate::semaev_prime_poly::{Curve, Fp};
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct NormStats {
    pub ring_multiplications: usize,
    pub largest_tensor_slots: usize,
}

struct Ring<'a> {
    f: Fp,
    fx: &'a [u64],
    gt: &'a [u64],
    stats: NormStats,
}

impl Ring<'_> {
    fn reduce(&self, h: &Tensor) -> Tensor {
        reduce_tensor(self.f, h, self.fx, self.gt)
    }
    fn constant(&self, c: u64) -> Tensor {
        self.reduce(&vec![vec![c]])
    }
    fn add(&self, a: &Tensor, b: &Tensor) -> Tensor {
        tensor_trim(
            (0..a.len().max(b.len()))
                .map(|i| {
                    add(
                        self.f,
                        a.get(i).map(Vec::as_slice).unwrap_or(&[0]),
                        b.get(i).map(Vec::as_slice).unwrap_or(&[0]),
                    )
                })
                .collect(),
        )
    }
    fn sub(&self, a: &Tensor, b: &Tensor) -> Tensor {
        tensor_trim(
            (0..a.len().max(b.len()))
                .map(|i| {
                    sub(
                        self.f,
                        a.get(i).map(Vec::as_slice).unwrap_or(&[0]),
                        b.get(i).map(Vec::as_slice).unwrap_or(&[0]),
                    )
                })
                .collect(),
        )
    }
    fn mul(&mut self, a: &Tensor, b: &Tensor) -> Tensor {
        self.stats.ring_multiplications += 1;
        let out = self.reduce(&tensor_mul(self.f, a, b, self.gt));
        self.stats.largest_tensor_slots = self
            .stats
            .largest_tensor_slots
            .max(out.iter().map(Vec::len).sum());
        out
    }
    fn pow(&mut self, a: &Tensor, mut n: usize) -> Tensor {
        let mut r = self.constant(1);
        let mut b = a.clone();
        while n > 0 {
            if n & 1 == 1 {
                r = self.mul(&r, &b);
            }
            n >>= 1;
            if n > 0 {
                b = self.mul(&b, &b);
            }
        }
        r
    }
    fn lucas(&mut self, s: &Tensor, z: &Tensor, n: usize) -> Tensor {
        let mut u = self.constant(2);
        let mut v = s.clone();
        let mut z_power = self.constant(1);
        let bits = usize::BITS - n.leading_zeros();
        for bit in (0..bits).rev() {
            let u2 = self.mul(&u, &u);
            let even = self.sub(&u2, &self.add(&z_power, &z_power));
            let uv = self.mul(&u, &v);
            let sz = self.mul(s, &z_power);
            let odd = self.sub(&uv, &sz);
            let z_even = self.mul(&z_power, &z_power);
            if n >> bit & 1 == 0 {
                u = even;
                v = odd;
                z_power = z_even;
            } else {
                let z_next = self.mul(&z_power, z);
                let v2 = self.mul(&v, &v);
                let next_even = self.sub(&v2, &self.add(&z_next, &z_next));
                u = odd;
                v = next_even;
                z_power = self.mul(&z_even, z);
            }
        }
        u
    }
}

/// Construct Res_z(z^d-1,S3(x,z,t)) directly modulo F(x),G(t).
/// Preconditions: d>=1 and valid monic F,G in the declared prime field.
/// No inverse of the quadratic leading coefficient is used.
pub fn tensor_norm(curve: Curve, d: usize, fx: &[u64], gt: &[u64]) -> (Tensor, NormStats) {
    assert!(d > 0);
    if fx == [1] || gt == [1] {
        return (vec![vec![0]], NormStats::default());
    }
    let f = curve.f;
    let mut ring = Ring {
        f,
        fx,
        gt,
        stats: NormStats::default(),
    };
    let minus2 = f.neg(2);
    let minus2a = f.neg(f.mul(2, curve.a));
    let minus4b = f.neg(f.mul(4, curve.b));
    let a = ring.reduce(&vec![vec![0, 0, 1], vec![0, minus2], vec![1]]);
    let b = ring.reduce(&vec![
        vec![minus4b, minus2a],
        vec![minus2a, 0, minus2],
        vec![0, minus2],
    ]);
    let c = ring.reduce(&vec![
        vec![f.mul(curve.a, curve.a), minus4b],
        vec![minus4b, minus2a],
        vec![0, 0, 1],
    ]);
    let s = ring.sub(&ring.constant(0), &b);
    let z = ring.mul(&a, &c);
    let trace = ring.lucas(&s, &z, d);
    let ad = ring.pow(&a, d);
    let cd = ring.pow(&c, d);
    let out = ring.sub(&ring.add(&ad, &cd), &trace);
    (out, ring.stats)
}
