//! Native evaluation of the historical normal Weil functions and 64 sections.
use crate::field::{add, cubic_roots, neg, Point, F};

#[derive(Clone)]
pub struct Weil {
    pub a: F,
    pub b: F,
    two: [Point; 4],
    three: [Point; 9],
    alpha: [F; 9],
    g: [F; 4],
}
impl Weil {
    pub fn new(z: F) -> Self {
        let enc = |n: u16| {
            F::c((n % 7) as u8) + F::c((n / 7 % 7) as u8) * z + F::c((n / 49) as u8) * z * z
        };
        let (a, b) = (enc(248), enc(42));
        let roots = cubic_roots([b, a, F::ZERO, F::c(1)]);
        let mut w = Self {
            a,
            b,
            two: [None; 4],
            three: [None; 9],
            alpha: [F::c(1); 9],
            g: [F::c(1); 4],
        };
        for (i, x) in roots.iter().enumerate() {
            w.two[i + 1] = Some((*x, F::ZERO));
        }
        let points: Vec<Point> = [19, 65]
            .into_iter()
            .map(|n| {
                let x = enc(n);
                Some((
                    x,
                    (x.pow(3) + a * x + b).sqrt().expect("torsion square root"),
                ))
            })
            .collect();
        for p in &points {
            assert_eq!(add(a, add(a, *p, *p), *p), None);
        }
        for i in 0..3 {
            for j in 0..3 {
                let u = if i == 0 {
                    None
                } else if i == 1 {
                    points[0]
                } else {
                    neg(points[0])
                };
                let v = if j == 0 {
                    None
                } else if j == 1 {
                    points[1]
                } else {
                    neg(points[1])
                };
                w.three[3 * i + j] = add(a, u, v);
            }
        }
        for i in 0..9 {
            for j in 0..i {
                assert_ne!(w.three[i], w.three[j], "dependent torsion basis");
            }
        }
        let probe = (0..50)
            .find_map(|n| {
                let x = F::generator() + F::c(n);
                (x.pow(3) + a * x + b).sqrt().map(|y| (x, y))
            })
            .expect("probe search exhausted");
        for i in 0..3 {
            for j in 0..3 {
                let n = 3 * i + j;
                if n == 0 {
                    continue;
                }
                let opposite = 3 * ((3 - i) % 3) + (3 - j) % 3;
                let c = w.raw(n, Some(probe)) * w.raw(n, add(a, Some(probe), w.three[n]))
                    / w.raw(opposite, Some(probe));
                w.alpha[n] = -F::c(1) / c;
            }
        }
        let zeta = w.w3(3, Some(probe)) * w.w3(1, add(a, Some(probe), w.three[3]))
            / (w.w3(1, Some(probe)) * w.w3(3, add(a, Some(probe), w.three[1])));
        assert_eq!(zeta.pow(3), F::c(1));
        assert_ne!(zeta, F::c(1));
        for i in 0..9 {
            for j in 0..9 {
                let sum = 3 * ((i / 3 + j / 3) % 3) + (i % 3 + j % 3) % 3;
                let c = w.w3(i, Some(probe)) * w.w3(j, add(a, Some(probe), w.three[i]))
                    / w.w3(sum, Some(probe));
                let exponent =
                    (2 * ((i / 3 * (j % 3)) as i32 - (i % 3 * (j / 3)) as i32)).rem_euclid(3);
                assert_eq!(c, zeta.pow(exponent as u64), "3-Weil normalization");
            }
        }
        let (e1, e2, e3) = (roots[0], roots[1], roots[2]);
        w.g = [
            F::c(1),
            ((e1 - e3) / (e1 - e2)).sqrt().unwrap(),
            ((e2 - e3) / (e2 - e1)).sqrt().unwrap(),
            ((e3 - e1) * (e2 - e3)).sqrt().unwrap(),
        ];
        let ratio =
            w.w2(1, Some(probe)) * w.w2(2, add(a, Some(probe), w.two[1])) / w.w2(3, Some(probe));
        assert!(ratio == F::c(1) || ratio == -F::c(1));
        if ratio == F::c(1) {
            w.g[3] = -w.g[3];
        }
        for i in 0..4 {
            for j in 0..4 {
                let ratio = w.w2(i, Some(probe)) * w.w2(j, add(a, Some(probe), w.two[i]))
                    / w.w2(i ^ j, Some(probe));
                assert_eq!(
                    ratio,
                    if i & 1 != 0 && j & 2 != 0 {
                        -F::c(1)
                    } else {
                        F::c(1)
                    },
                    "2-Weil normalization"
                );
            }
        }
        w
    }
    fn raw(&self, n: usize, point: Point) -> F {
        let Some((x, y)) = neg(self.three[n]) else {
            return F::c(1);
        };
        let (sx, sy) = add(self.a, point, self.two[3]).expect("Weil pole");
        let m = (F::c(3) * x * x + self.a) / (F::c(2) * y);
        sy - y - m * (sx - x)
    }
    fn w3(&self, n: usize, p: Point) -> F {
        self.alpha[n] * self.raw(n, p)
    }
    fn w2(&self, i: usize, p: Point) -> F {
        if i == 0 {
            return F::c(1);
        }
        let Some((x, _)) = p else {
            return if i == 3 { F::ZERO } else { self.g[i] };
        };
        let e1 = self.two[1].unwrap().0;
        let e2 = self.two[2].unwrap().0;
        let e3 = self.two[3].unwrap().0;
        match i {
            1 => self.g[1] * (x - e2) / (x - e3),
            2 => self.g[2] * (x - e1) / (x - e3),
            3 => self.g[3] / (x - e3),
            _ => unreachable!(),
        }
    }
    pub fn conjugate(&self, n: usize) -> Self {
        let point = |p: Point| p.map(|(x, y)| (x.frob(n), y.frob(n)));
        Self {
            a: self.a.frob(n),
            b: self.b.frob(n),
            two: self.two.map(point),
            three: self.three.map(point),
            alpha: self.alpha.map(|v| v.frob(n)),
            g: self.g.map(|v| v.frob(n)),
        }
    }
}

pub const KERNEL: [[usize; 6]; 3] = [[1, 0, 0, 0, 2, 0], [0, 1, 0, 2, 0, 1], [0, 0, 1, 0, 1, 0]];
pub fn sections(weils: &[Weil; 3], point: (F, F)) -> Vec<F> {
    // The protocol freezes the already certified even seed (0,0,0).
    // No seed search or target-based coordinate selection occurs here.
    let mut values = [[[F::ZERO; 9]; 4]; 3];
    let mut gvalues = [[F::ZERO; 4]; 3];
    for (i, w) in weils.iter().enumerate() {
        let p = Some((point.0.frob(i), point.1.frob(i)));
        for ti in 0..4 {
            gvalues[i][ti] = w.w2(ti, p);
            let shifted = add(w.a, p, w.two[ti]);
            for (ab, value) in values[i][ti].iter_mut().enumerate() {
                *value = w.w3(ab, shifted);
            }
        }
    }
    let mut kernel = vec![];
    for a in 0..3 {
        for b in 0..3 {
            for c in 0..3 {
                kernel.push(std::array::from_fn::<_, 6, _>(|i| {
                    (a * KERNEL[0][i] + b * KERNEL[1][i] + c * KERNEL[2][i]) % 3
                }));
            }
        }
    }
    (0..64)
        .map(|n| {
            let t: [usize; 3] = std::array::from_fn(|i| ((n >> (3 + i)) & 1) + 2 * ((n >> i) & 1));
            let mut total = F::ZERO;
            for p in &kernel {
                total = total
                    + values[0][t[0]][3 * p[0] + p[1]]
                        * values[1][t[1]][3 * p[2] + p[3]]
                        * values[2][t[2]][3 * p[4] + p[5]];
            }
            (gvalues[0][t[0]] * gvalues[1][t[1]] * gvalues[2][t[2]]).pow(3) * total * total
        })
        .collect()
}
