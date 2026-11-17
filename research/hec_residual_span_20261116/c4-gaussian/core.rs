use hec_native_ic::{fp::Fp, jac::Curve};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Z {
    pub r: i128,
    pub i: i128,
}
impl Z {
    pub fn new(r: i128, i: i128) -> Self {
        Self { r, i }
    }
    pub fn add(self, b: Self) -> Self {
        Self::new(self.r + b.r, self.i + b.i)
    }
    pub fn mul(self, b: Self) -> Self {
        Self::new(self.r * b.r - self.i * b.i, self.r * b.i + self.i * b.r)
    }
    pub fn scale(self, s: i128) -> Self {
        Self::new(self.r * s, self.i * s)
    }
    pub fn bar(self) -> Self {
        Self::new(self.r, -self.i)
    }
    pub fn norm(self) -> i128 {
        self.r * self.r + self.i * self.i
    }
}
pub fn sqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = n;
    loop {
        let y = (x + n / x) / 2;
        if y >= x {
            return x;
        }
        x = y
    }
}
fn square(n: i128) -> Option<i128> {
    if n < 0 {
        return None;
    }
    let r = sqrt(n as u128) as i128;
    (r * r == n).then_some(r)
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    pub a: Z,
    pub b: Z,
    pub unit: Z,
    pub value: Z,
    pub g: Vec<Z>,
    pub lambda: u64,
}
pub struct Trace {
    pub ordinary_count: u64,
    pub twisted_count: i128,
    pub nonsquare: u64,
    pub imag_trace: i128,
}
// Input contains no source point, target, eigenvalue, full Weil polynomial or expected scalar.
pub fn derive(c: &Curve, n: u64, ell: u64, i: u64) -> (Trace, Vec<Candidate>) {
    let p = c.fp.p;
    assert_eq!(c.g, 4);
    assert_eq!(p % 4, 1);
    assert_eq!(c.fp.mul(i, i), p - 1);
    assert_eq!(n, 2 * ell);
    let mut chi = vec![-1i128; p as usize];
    chi[0] = 0;
    for y in 1..=(p - 1) / 2 {
        let t = c.fp.mul(y, y);
        chi[t as usize] = 1;
    }
    let d = (2..p).find(|&x| chi[x as usize] == -1).unwrap();
    let kappa = c.fp.pow(d, (p - 1) / 4);
    let sign = if kappa == c.fp.neg(i) {
        1
    } else {
        assert_eq!(kappa, i);
        -1
    };
    let mut ordinary_sum = 0;
    let mut twisted_sum = 0;
    // Pair t and -t, since p=1mod4, f and H are odd and chi(-1)=1.
    for t in 1..=(p - 1) / 2 {
        let x2 = c.fp.mul(t, t);
        let mut fval = 1;
        let mut hval = 1;
        let dx2 = c.fp.mul(d, x2);
        for j in [7, 5, 3, 1] {
            fval = c.fp.add(c.fp.mul(fval, x2), c.f.c[j]);
            hval = c.fp.add(c.fp.mul(hval, dx2), c.f.c[j]);
        }
        fval = c.fp.mul(fval, t);
        hval = c.fp.mul(hval, t);
        ordinary_sum += 2 * chi[fval as usize];
        twisted_sum += 2 * chi[hval as usize];
    }
    let count = p as i128 + 1 + ordinary_sum;
    let twist_count = p as i128 + 1 + sign * twisted_sum;
    let ar = ordinary_sum / 2;
    assert_eq!(ordinary_sum % 2, 0);
    let trace = -sign * twisted_sum;
    assert_eq!(trace % 2, 0);
    let ai = trace / 2;
    let a = Z::new(ar, ai);
    assert!(a.norm() <= 16 * p as i128);
    let q = p as i128;
    let nn = n as i128;
    let ef = Fp::new(ell);
    let mut candidates = vec![];
    for unit in [Z::new(1, 0), Z::new(-1, 0), Z::new(0, 1), Z::new(0, -1)] {
        let base = Z::new(1, 0)
            .add(a)
            .add(unit.mul(a.bar()).scale(q))
            .add(unit.scale(q * q));
        let mut bs = vec![];
        if unit.r == 1 {
            if let Some(s) = square(nn - base.i * base.i) {
                for r in [s, -s] {
                    bs.push(Z::new(r - base.r, 0))
                }
            }
        } else if unit.r == -1 {
            if let Some(s) = square(nn - base.r * base.r) {
                for r in [s, -s] {
                    bs.push(Z::new(0, r - base.i))
                }
            }
        } else {
            let dir = unit.i;
            let line = base.r - dir * base.i;
            let center = base.r + dir * base.i;
            if let Some(s) = square(2 * nn - line * line) {
                for x in [s, -s] {
                    let num = -center + x;
                    if num % 2 == 0 {
                        bs.push(Z::new(num / 2, dir * num / 2))
                    }
                }
            }
        }
        for b in bs {
            if b.norm() > 36 * q * q {
                continue;
            }
            assert_eq!(b, unit.mul(b.bar()));
            let value = base.add(b);
            assert_eq!(value.norm(), nn);
            let br = value.i.rem_euclid(ell as i128) as u64;
            if br == 0 {
                continue;
            }
            let lambda = ef.mul((-value.r).rem_euclid(ell as i128) as u64, ef.inv(br));
            if ef.mul(lambda, lambda) != ell - 1 {
                continue;
            }
            let candidate = Candidate {
                a,
                b,
                unit,
                value,
                g: vec![
                    unit.scale(q * q),
                    unit.mul(a.bar()).scale(q),
                    b,
                    a,
                    Z::new(1, 0),
                ],
                lambda,
            };
            if !candidates.contains(&candidate) {
                candidates.push(candidate)
            }
        }
    }
    (
        Trace {
            ordinary_count: count.try_into().unwrap(),
            twisted_count: twist_count,
            nonsquare: d,
            imag_trace: trace,
        },
        candidates,
    )
}
pub fn norm_polynomial(g: &[Z]) -> Vec<i128> {
    let mut out = vec![Z::new(0, 0); 2 * g.len() - 1];
    for j in 0..g.len() {
        for k in 0..g.len() {
            out[j + k] = out[j + k].add(g[j].mul(g[k].bar()))
        }
    }
    assert!(out.iter().all(|x| x.i == 0));
    out.iter().map(|x| x.r).collect()
}
