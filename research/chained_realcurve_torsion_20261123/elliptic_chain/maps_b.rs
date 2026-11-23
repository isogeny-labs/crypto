use crate::chart::{homogeneous, Chart, Divisor};
use crate::ec::{on_curve, Point};
use crate::field::Field;
use crate::inherited::{padd, pmod, pmonic, pmul, pscale, psub, ptrim, Div, Fld};

pub const ORIGINAL: [u32; 7] = [1, 0, 1, 0, 0, 1, 1];
pub const TARGET0: [u32; 7] = [1, 0, 1, 1, 0, 1, 1];
pub const TARGET2: [u32; 7] = [1, 0, 1, 2, 0, 1, 2];
pub const G0: [[u32; 3]; 3] = [[4, 24, 1], [5, 19, 1], [3, 21, 1]];
pub const G1: [[u32; 3]; 3] = [[16, 23, 1], [13, 26, 1], [9, 18, 1]];
pub const G2: [[u32; 3]; 3] = [[25, 20, 1], [20, 22, 1], [22, 25, 1]];

pub struct Kernel {
    pub id: usize,
    pub g: [Vec<u32>; 3],
    pub h: [Vec<u32>; 3],
    pub delta: u32,
    pub codomain: Vec<u32>,
}

fn derivative(f: &Field, p: &[u32]) -> Vec<u32> {
    let mut result: Vec<_> = p
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| f.mul(c, &(i as u32 % 3)))
        .collect();
    ptrim(f, &mut result);
    result
}

impl Kernel {
    pub fn new(f: &Field, id: usize, coefficients: [[u32; 3]; 3]) -> Result<Self, &'static str> {
        let g = coefficients.map(|p| p.to_vec());
        assert_eq!(pmul(f, &pmul(f, &g[0], &g[1]), &g[2]), ORIGINAL);
        let rows = g.each_ref().map(|p| [p[2], p[1], p[0]]);
        let minor = |j: usize, k: usize| {
            f.sub(
                &f.mul(&rows[1][j], &rows[2][k]),
                &f.mul(&rows[1][k], &rows[2][j]),
            )
        };
        let delta = f.add(
            &f.sub(
                &f.mul(&rows[0][0], &minor(1, 2)),
                &f.mul(&rows[0][1], &minor(0, 2)),
            ),
            &f.mul(&rows[0][2], &minor(0, 1)),
        );
        if delta == 0 {
            return Err("actual product endpoint: no genus-two target chart");
        }
        let h = [(1, 2), (2, 0), (0, 1)].map(|(j, k)| {
            psub(
                f,
                &pmul(f, &g[j], &derivative(f, &g[k])),
                &pmul(f, &derivative(f, &g[j]), &g[k]),
            )
        });
        let codomain = pscale(f, &pmul(f, &pmul(f, &h[0], &h[1]), &h[2]), &f.inv(&delta));
        let expected = if id == 0 {
            TARGET0
        } else {
            assert_eq!(id, 2);
            TARGET2
        };
        assert_eq!(codomain, expected);
        Ok(Self {
            id,
            g,
            h,
            delta,
            codomain,
        })
    }
}

pub fn cover_identity(f: &Field) {
    let mut a_powers = vec![vec![1]];
    let mut b_powers = vec![vec![1]];
    for _ in 0..6 {
        a_powers.push(pmul(f, a_powers.last().unwrap(), &[0, 1]));
        b_powers.push(pmul(f, b_powers.last().unwrap(), &[1, 2]));
    }
    let mut transformed = Vec::new();
    for (i, c) in ORIGINAL.iter().enumerate() {
        transformed = padd(
            f,
            &transformed,
            &pscale(f, &pmul(f, &a_powers[i], &b_powers[6 - i]), c),
        );
    }
    let even = vec![1, 0, 1, 0, 0, 0, 2];
    assert_eq!(transformed, even);
    let x = vec![0, 0, 2];
    let elliptic = padd(
        f,
        &padd(f, &pmul(f, &pmul(f, &x, &x), &x), &pscale(f, &x, &2)),
        &[1],
    );
    assert_eq!(elliptic, even);
}

pub fn pullback(source: &Chart<'_>, p: Point) -> Divisor {
    assert!(on_curve(p));
    if p.infinity {
        return source.h.identity();
    }
    let f = source.h.f;
    let a = vec![1, source.r];
    let b = vec![1, f.add(&source.r, &1)];
    let u = pmonic(
        f,
        &psub(
            f,
            &pmul(f, &a, &a),
            &pscale(f, &pmul(f, &b, &b), &f.mul(&2, &p.x)),
        ),
    );
    assert_eq!(u.len(), 3);
    let numerator = pscale(f, &pmul(f, &pmul(f, &b, &b), &b), &f.mul(&2, &p.y));
    let v = pmod(f, &pscale(f, &numerator, &f.inv(&source.scale)), &u);
    let d = Div { u, v };
    assert!(source.h.is_valid(&d));
    d
}

pub fn preimages(f: &Field, p: Point) -> [(u32, u32, u32); 2] {
    assert!(on_curve(p) && !p.infinity);
    let root = f.sqrt(f.mul(&2, &p.x)).unwrap();
    [root, f.neg(&root)].map(|u| (u, f.sub(&1, &u), f.mul(&2, &p.y)))
}

pub fn source_fibre_reference(source: &Chart<'_>, p: Point) -> Divisor {
    assert!(on_curve(p));
    if p.infinity {
        let y = source.h.f.sqrt(2).unwrap();
        let a = source.point_homogeneous(2, 1, y);
        let b = source.point_homogeneous(2, 1, source.h.f.neg(&y));
        return source.h.add(&a, &b);
    }
    let points = preimages(source.h.f, p).map(|(a, b, y)| source.point_homogeneous(a, b, y));
    source.h.add(&points[0], &points[1])
}

fn binary_quadratic(f: &Field, g: &[u32], a: u32, b: u32) -> u32 {
    let mut value = 0;
    for (i, c) in g.iter().enumerate() {
        value = f.add(
            &value,
            &f.mul(c, &f.mul(&f.pow(a, i as u32), &f.pow(b, (2 - i) as u32))),
        );
    }
    value
}

pub fn relation(f: &Field, kernel: &Kernel, a: u32, b: u32, ordinate: u32) -> (Vec<u32>, Vec<u32>) {
    assert!(a != 0 || b != 0);
    assert_ne!(ordinate, 0);
    let mut source_equation = 0;
    for (i, c) in ORIGINAL.iter().enumerate() {
        source_equation = f.add(
            &source_equation,
            &f.mul(c, &f.mul(&f.pow(a, i as u32), &f.pow(b, (6 - i) as u32))),
        );
    }
    assert_eq!(f.mul(&ordinate, &ordinate), source_equation);
    let g0 = binary_quadratic(f, &kernel.g[0], a, b);
    let g1 = binary_quadratic(f, &kernel.g[1], a, b);
    let u = padd(
        f,
        &pscale(f, &kernel.h[0], &g0),
        &pscale(f, &kernel.h[1], &g1),
    );
    assert!(!u.is_empty());
    let v = pscale(
        f,
        &pmul(f, &kernel.h[0], &[a, f.neg(&b)]),
        &f.mul(&g0, &f.inv(&ordinate)),
    );
    (u, v)
}

pub fn convert_ideal(target: &Chart<'_>, u: &[u32], numerator: &[u32]) -> Divisor {
    let f = target.h.f;
    let u = pmonic(f, &homogeneous(f, u, 2, target.r));
    assert!(!u.is_empty() && u.len() <= 3);
    let v = pmod(
        f,
        &pscale(
            f,
            &homogeneous(f, numerator, 3, target.r),
            &f.inv(&target.scale),
        ),
        &u,
    );
    Div { u, v }
}

pub fn image_point(target: &Chart<'_>, kernel: &Kernel, a: u32, b: u32, ordinate: u32) -> Divisor {
    let (u, numerator) = relation(target.h.f, kernel, a, b, ordinate);
    let d = convert_ideal(target, &u, &numerator);
    assert!(target.h.is_valid(&d));
    d
}

pub fn chain(target: &Chart<'_>, kernel: &Kernel, p: Point) -> Divisor {
    assert!(on_curve(p));
    if p.infinity {
        return target.h.identity();
    }
    let images = preimages(target.h.f, p).map(|(a, b, y)| image_point(target, kernel, a, b, y));
    target.h.add(&images[0], &images[1])
}

pub fn identity_fibre(target: &Chart<'_>, kernel: &Kernel) -> Divisor {
    let f = target.h.f;
    let y = f.sqrt(2).unwrap();
    let a = image_point(target, kernel, 2, 1, y);
    let b = image_point(target, kernel, 2, 1, f.neg(&y));
    target.h.add(&a, &b)
}
