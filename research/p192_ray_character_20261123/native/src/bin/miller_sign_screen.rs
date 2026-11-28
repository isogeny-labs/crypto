//! Coordinate-input screen of an unreduced Miller-function character.
//! The scalar is used only to generate and label control points.
use num_bigint::BigUint;
use num_traits::{One, Zero};

type Pt = Option<(BigUint, BigUint)>;

#[derive(Clone)]
struct Curve {
    p: BigUint,
    a: BigUint,
    b: BigUint,
}

impl Curve {
    fn addf(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x + y) % &self.p
    }
    fn subf(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x + &self.p - y) % &self.p
    }
    fn mulf(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x * y) % &self.p
    }
    fn inv(&self, x: &BigUint) -> BigUint {
        x.modpow(&(&self.p - 2u32), &self.p)
    }
    fn legendre(&self, x: &BigUint) -> i8 {
        let v = x.modpow(&((&self.p - 1u32) >> 1usize), &self.p);
        if v.is_zero() {
            0
        } else if v.is_one() {
            1
        } else {
            -1
        }
    }
    fn on_curve(&self, p: &Pt) -> bool {
        let Some((x, y)) = p else { return true };
        if x >= &self.p || y >= &self.p {
            return false;
        }
        self.mulf(y, y)
            == self.addf(
                &self.addf(&self.mulf(x, &self.mulf(x, x)), &self.mulf(&self.a, x)),
                &self.b,
            )
    }
    fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let (x1, y1) = match p {
            None => return q.clone(),
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p.clone(),
            Some(v) => v,
        };
        if x1 == x2 && self.addf(y1, y2).is_zero() {
            return None;
        }
        let lambda = if x1 == x2 {
            let num = self.addf(&self.mulf(&3u32.into(), &self.mulf(x1, x1)), &self.a);
            let den = self.mulf(&2u32.into(), y1);
            self.mulf(&num, &self.inv(&den))
        } else {
            self.mulf(&self.subf(y2, y1), &self.inv(&self.subf(x2, x1)))
        };
        let x3 = self.subf(&self.subf(&self.mulf(&lambda, &lambda), x1), x2);
        let y3 = self.subf(&self.mulf(&lambda, &self.subf(x1, &x3)), y1);
        Some((x3, y3))
    }
    fn scale(&self, p: &Pt, mut n: BigUint) -> Pt {
        let mut out = None;
        let mut cur = p.clone();
        while !n.is_zero() {
            if (&n & BigUint::one()).is_one() {
                out = self.add(&out, &cur);
            }
            cur = self.add(&cur, &cur);
            n >>= 1usize;
        }
        out
    }
    fn step(&self, t: &Pt, u: &Pt, r: &Pt) -> Option<(BigUint, Pt)> {
        let (tx, ty) = t.as_ref()?;
        let (ux, uy) = u.as_ref()?;
        let (rx, ry) = r.as_ref()?;
        if tx == ux && self.addf(ty, uy).is_zero() {
            let v = self.subf(rx, tx);
            return if v.is_zero() { None } else { Some((v, None)) };
        }
        let slope = if tx == ux {
            let num = self.addf(&self.mulf(&3u32.into(), &self.mulf(tx, tx)), &self.a);
            let den = self.mulf(&2u32.into(), ty);
            self.mulf(&num, &self.inv(&den))
        } else {
            self.mulf(&self.subf(uy, ty), &self.inv(&self.subf(ux, tx)))
        };
        let next = self.add(t, u);
        let line = self.subf(&self.subf(ry, ty), &self.mulf(&slope, &self.subf(rx, tx)));
        let vertical = match next.as_ref() {
            None => BigUint::one(),
            Some((nx, _)) => self.subf(rx, nx),
        };
        if line.is_zero() || vertical.is_zero() {
            return None;
        }
        Some((self.mulf(&line, &self.inv(&vertical)), next))
    }
    fn miller(&self, g: &Pt, r: &Pt, order: &BigUint) -> Option<BigUint> {
        let mut t = g.clone();
        let mut f = BigUint::one();
        for bit in (0..order.bits() - 1).rev() {
            let (v, next) = self.step(&t, &t, r)?;
            f = self.mulf(&self.mulf(&f, &f), &v);
            t = next;
            if order.bit(bit) {
                let (v, next) = self.step(&t, g, r)?;
                f = self.mulf(&f, &v);
                t = next;
            }
        }
        assert!(t.is_none());
        Some(f)
    }
    fn raw_feature(&self, g: &Pt, q: &Pt, order: &BigUint) -> (i8, u32) {
        // The public shift is fixed where possible; a deterministic fallback
        // handles Miller zeros or poles at intermediate multiples.
        for j in 2..80u32 {
            let s = self.scale(g, j.into());
            let qs = self.add(q, &s);
            if s.is_none() || qs.is_none() {
                continue;
            }
            if let (Some(num), Some(den)) = (self.miller(g, &qs, order), self.miller(g, &s, order))
            {
                return (self.legendre(&self.mulf(&num, &self.inv(&den))), j);
            }
        }
        panic!("no valid public shift");
    }
}

fn decimal(x: &str) -> BigUint {
    x.parse().unwrap()
}
fn hex(x: &str) -> BigUint {
    BigUint::parse_bytes(x.as_bytes(), 16).unwrap()
}
fn screen(name: &str, c: Curve, g: Pt, order: BigUint, marks: &[u64], expected_agreements: usize) {
    assert!(c.on_curve(&g));
    assert!(c.scale(&g, order.clone()).is_none());
    let (base, base_shift) = c.raw_feature(&g, &g, &order);
    let mut agreements = 0usize;
    let mut first = Vec::new();
    let mut shifts = 0usize;
    for &d in marks {
        let q = c.scale(&g, d.into());
        assert!(c.on_curve(&q));
        let (observed, shift) = c.raw_feature(&g, &q, &order);
        if shift != base_shift {
            shifts += 1;
        }
        let predicted = observed * base;
        let expected = if BigUint::from(d)
            .modpow(&((&order - 1u32) >> 1usize), &order)
            .is_one()
        {
            1
        } else {
            -1
        };
        if predicted == expected {
            agreements += 1;
        } else if first.len() < 8 {
            first.push((d, predicted, expected, shift));
        }
    }
    assert_eq!(agreements, expected_agreements, "unexpected screen count");
    println!("{name} marks={} agreements={agreements} base_shift={base_shift} changed_shifts={shifts} first_mismatches={first:?}",marks.len());
}
fn main() {
    let all: Vec<u64> = (1..97).collect();
    screen(
        "F79_a0_b3",
        Curve {
            p: 79u32.into(),
            a: 0u32.into(),
            b: 3u32.into(),
        },
        Some((1u32.into(), 2u32.into())),
        97u32.into(),
        &all,
        49,
    );
    screen(
        "F79_a1_b6",
        Curve {
            p: 79u32.into(),
            a: 1u32.into(),
            b: 6u32.into(),
        },
        Some((1u32.into(), 18u32.into())),
        97u32.into(),
        &all,
        53,
    );
    screen(
        "F6666679",
        Curve {
            p: 6666679u32.into(),
            a: 5u32.into(),
            b: 5u32.into(),
        },
        Some((9970u32.into(), 1003793u32.into())),
        10009u32.into(),
        &[1, 2, 3, 5, 7, 11, 13],
        3,
    );
    let p = decimal("6277101735386680763835789423207666416083908700390324961279");
    let r = decimal("6277101735386680763835789423176059013767194773182842284081");
    let curve = Curve {
        a: &p - 3u32,
        b: hex("64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1"),
        p,
    };
    let generator = Some((
        hex("188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012"),
        hex("07192B95FFC8DA78631011ED6B24CDD573F977A11E794811"),
    ));
    if let [_, qx, qy] = std::env::args().collect::<Vec<_>>().as_slice() {
        let q = Some((hex(qx), hex(qy)));
        assert!(
            curve.on_curve(&q) && curve.scale(&q, r.clone()).is_none(),
            "target must be P-192 rational r-torsion"
        );
        let (base, _) = curve.raw_feature(&generator, &generator, &r);
        let (value, shift) = curve.raw_feature(&generator, &q, &r);
        println!(
            "P192_coordinate_input candidate_sign={} shift={shift}",
            base * value
        );
    } else {
        let q4 = curve.scale(&generator, 4u32.into());
        let (base, base_shift) = curve.raw_feature(&generator, &generator, &r);
        let (at_four, four_shift) = curve.raw_feature(&generator, &q4, &r);
        assert_eq!((base_shift, four_shift), (4, 4));
        assert_eq!(base * at_four, -1);
        let (q4x, q4y) = q4.as_ref().unwrap();
        println!(
            "P192_fixed_shift_4_witness qx={:x} qy={:x} candidate_sign={} expected=1",
            q4x,
            q4y,
            base * at_four
        );
        screen(
            "P192",
            curve,
            generator,
            r,
            &[1, 2, 3, 4, 5, 7, 10, 11, 13, 17, 19, 23, 29, 31],
            4,
        );
    }
}
