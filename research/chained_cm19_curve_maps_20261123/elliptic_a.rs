use crate::geometry_a::*;

pub type EPoint = Option<(u8, u8)>;
pub const E7_POINTS: [EPoint; 11] = [
    None, Some((1, 3)), Some((1, 4)), Some((2, 2)), Some((2, 5)),
    Some((3, 2)), Some((3, 5)), Some((4, 1)), Some((4, 6)),
    Some((5, 1)), Some((5, 6)),
];

pub fn evalid(point: EPoint, b: u8) -> bool {
    point.is_none_or(|(x, y)| pow(y, 2) == add(add(pow(x, 3), mul(2, x)), b))
}

pub fn eadd(a: EPoint, b: EPoint) -> EPoint {
    assert!(evalid(a, 6) && evalid(b, 6));
    let Some((x, y)) = a else { return b; };
    let Some((xx, yy)) = b else { return a; };
    if x == xx && add(y, yy) == 0 {
        return None;
    }
    let slope = if x == xx {
        mul(add(mul(3, pow(x, 2)), 2), inv(mul(2, y)))
    } else {
        mul(sub(yy, y), inv(sub(xx, x)))
    };
    let nx = sub(sub(pow(slope, 2), x), xx);
    let ny = sub(mul(slope, sub(x, nx)), y);
    let out = Some((nx, ny));
    assert!(evalid(out, 6));
    out
}

pub fn emul(mut n: usize, mut point: EPoint) -> EPoint {
    let mut out = None;
    while n != 0 {
        if n & 1 != 0 {
            out = eadd(out, point);
        }
        point = eadd(point, point);
        n >>= 1;
    }
    out
}

pub fn check_group() {
    for &a in &E7_POINTS {
        assert!(evalid(a, 6));
        assert_eq!(emul(11, a), None);
        for &b in &E7_POINTS {
            assert!(E7_POINTS.contains(&eadd(a, b)));
            assert_eq!(eadd(a, b), eadd(b, a));
            for &c in &E7_POINTS {
                assert_eq!(eadd(eadd(a, b), c), eadd(a, eadd(b, c)));
            }
        }
    }
    let points: Vec<_> = (0..11).map(|n| emul(n, E7_POINTS[1])).collect();
    for &point in &E7_POINTS {
        assert_eq!(points.iter().filter(|&&p| p == point).count(), 1);
    }
    println!("ELLIPTIC_GROUP\tE7_points={E7_POINTS:?}\t121 additions/1331 associative triples/11 orders\tPASS");
}

pub fn cover(point: Point) -> EPoint {
    let [x, y, z] = point;
    if y == 0 {
        return None;
    }
    let g = add(add(mul(6, pow(x, 2)), mul(3, mul(x, y))), pow(y, 2));
    let out = Some((mul(x, inv(y)), mul(add(pow(z, 2), g), inv(pow(y, 2)))));
    assert!(evalid(out, 6));
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum QPoint {
    Finite(u8, u8),
    Infinity(u8),
}

#[derive(Debug)]
pub struct QuarticModel {
    pub coefficients: [u8; 5],
    s: u8,
    t: u8,
    r: u8,
    l: u8,
    m: u8,
    k: u8,
    a1: u8,
    a3: u8,
    shift: u8,
    scaling: u8,
}

fn trim(poly: &mut Vec<u8>) {
    while poly.last() == Some(&0) {
        poly.pop();
    }
}

pub fn squarefree(poly: [u8; 5]) -> bool {
    let mut a = poly.to_vec();
    trim(&mut a);
    let mut b: Vec<_> = a.iter().enumerate().skip(1).map(|(i, &v)| mul(i as u8, v)).collect();
    trim(&mut b);
    while !b.is_empty() {
        while a.len() >= b.len() {
            let shift = a.len() - b.len();
            let factor = mul(*a.last().unwrap(), inv(*b.last().unwrap()));
            for (i, &v) in b.iter().enumerate() {
                a[i + shift] = sub(a[i + shift], mul(factor, v));
            }
            trim(&mut a);
        }
        (a, b) = (b, a);
    }
    a.len() == 1
}

impl QuarticModel {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8) -> Self {
        assert_ne!(c, 0);
        assert_ne!(d, 0);
        assert_ne!(e, 0);
        let q = [a, b, e];
        let mut coefficients = [0; 5];
        for (i, &x) in q.iter().enumerate() {
            for (j, &y) in q.iter().enumerate() {
                coefficients[i + j] = add(coefficients[i + j], mul(x, y));
            }
        }
        coefficients[3] = sub(coefficients[3], mul(4, mul(c, d)));
        assert!(squarefree(coefficients));
        let s = e;
        let k = mul(2, s);
        let t = mul(coefficients[3], inv(k));
        let r = mul(sub(coefficients[2], pow(t, 2)), inv(k));
        let l = sub(coefficients[1], mul(2, mul(t, r)));
        let m = sub(coefficients[0], pow(r, 2));
        let a1 = mul(2, t);
        let a2 = sub(0, mul(2, mul(r, k)));
        let a3 = mul(l, k);
        let a4 = sub(0, mul(m, pow(k, 2)));
        let aa = add(a2, mul(pow(a1, 2), inv(4)));
        let bb = add(a4, mul(mul(a1, a3), inv(2)));
        let cc = mul(pow(a3, 2), inv(4));
        let shift = mul(aa, inv(3));
        let short_a = sub(bb, mul(pow(aa, 2), inv(3)));
        let short_b = add(sub(cc, mul(mul(aa, bb), inv(3))), mul(2, mul(pow(aa, 3), inv(6))));
        let scaling = (1..7).find(|&u| mul(short_a, pow(u, 4)) == 2 && mul(short_b, pow(u, 6)) == 6).expect("actual F7 short model isomorphism");
        println!("QUARTIC_MODEL\tABCDEF={:?}\tf={coefficients:?}\tgeneral_a1a2a3a4={:?}\tshort_a={short_a}\tshort_b={short_b}\tscaling={scaling}", [a,b,c,d,e], [a1,a2,a3,a4]);
        Self { coefficients, s, t, r, l, m, k, a1, a3, shift, scaling }
    }

    fn q(&self, u: u8) -> u8 {
        add(add(mul(self.s, pow(u, 2)), mul(self.t, u)), self.r)
    }

    pub fn valid(&self, point: QPoint) -> bool {
        match point {
            QPoint::Finite(u, w) => pow(w, 2) == eval_poly(self.coefficients, u),
            QPoint::Infinity(sign) => sign == self.s || sign == sub(0, self.s),
        }
    }

    pub fn to_e(&self, point: QPoint) -> EPoint {
        assert!(self.valid(point));
        let (x, y) = match point {
            QPoint::Infinity(sign) if sign == self.s => return None,
            QPoint::Infinity(_) => (0, sub(0, self.a3)),
            QPoint::Finite(u, w) => {
                let delta = add(w, self.q(u));
                (mul(self.k, delta), mul(pow(self.k, 2), mul(u, delta)))
            }
        };
        let short_x = add(x, self.shift);
        let short_y = add(y, mul(add(mul(self.a1, x), self.a3), inv(2)));
        let out = Some((mul(pow(self.scaling, 2), short_x), mul(pow(self.scaling, 3), short_y)));
        assert!(evalid(out, 6));
        out
    }

    pub fn inverse_e(&self, point: EPoint) -> QPoint {
        assert!(evalid(point, 6));
        let Some((sx, sy)) = point else { return QPoint::Infinity(self.s); };
        let x = sub(mul(sx, inv(pow(self.scaling, 2))), self.shift);
        let y = sub(mul(sy, inv(pow(self.scaling, 3))), mul(add(mul(self.a1, x), self.a3), inv(2)));
        let out = if x == 0 {
            if y == 0 {
                let u = mul(sub(0, self.m), inv(self.l));
                QPoint::Finite(u, sub(0, self.q(u)))
            } else {
                assert_eq!(y, sub(0, self.a3));
                QPoint::Infinity(sub(0, self.s))
            }
        } else {
            let u = mul(y, inv(mul(self.k, x)));
            let w = sub(mul(x, inv(self.k)), self.q(u));
            QPoint::Finite(u, w)
        };
        assert!(self.valid(out));
        out
    }

    pub fn identity_residue(&self, target_b: u8) -> Form {
        let u = term([1, 0, 0], 1);
        let w = term([0, 1, 0], 1);
        let q = fadd(&fadd(&fscale(&fpow(&u, 2), self.s), &fscale(&u, self.t)), &term([0, 0, 0], self.r));
        let delta = fadd(&w, &q);
        let x = fscale(&delta, self.k);
        let y = fscale(&fmul(&u, &delta), pow(self.k, 2));
        let sx = fscale(&fadd(&x, &term([0, 0, 0], self.shift)), pow(self.scaling, 2));
        let sy = fscale(&fadd(&y, &fscale(&fadd(&fscale(&x, self.a1), &term([0, 0, 0], self.a3)), inv(2))), pow(self.scaling, 3));
        let equation = fsub(&fsub(&fsub(&fpow(&sy, 2), &fpow(&sx, 3)), &fscale(&sx, 2)), &term([0, 0, 0], target_b));
        reduce_w2(equation, self.coefficients)
    }

    pub fn check_all(&self) {
        assert!(self.identity_residue(6).is_empty());
        assert!(!self.identity_residue(0).is_empty());
        assert!(self.identity_residue(6).is_empty());
        let mut points = Vec::new();
        for u in 0..7 {
            for w in 0..7 {
                let point = QPoint::Finite(u, w);
                if self.valid(point) {
                    points.push(point);
                }
            }
        }
        points.push(QPoint::Infinity(self.s));
        points.push(QPoint::Infinity(sub(0, self.s)));
        assert_eq!(points.len(), 11);
        assert_ne!(self.l, 0);
        assert_ne!(self.a3, 0);
        let mut images = Vec::new();
        let mut mutated_failures = 0;
        for point in points {
            let image = self.to_e(point);
            assert_eq!(self.inverse_e(image), point);
            assert!(E7_POINTS.contains(&image));
            assert!(!images.contains(&image));
            images.push(image);
            mutated_failures += usize::from(!evalid(image, 0));
            println!("ELLIPTIC_ISO\tquartic={point:?}\tE7={image:?}\tactual_roundtrip=PASS");
        }
        assert!(mutated_failures > 0);
        for point in E7_POINTS {
            assert_eq!(self.to_e(self.inverse_e(point)), point);
        }
        println!("ELLIPTIC_ISO_CONTROLS\t11+11 roundtrips\tfull functionfield identity\tactual target constant mutation rejects/restores\tPASS");
    }
}
