//! Evaluate Tian's published source map and check the resulting cubic cut.
//! This is a support gate, not a source-to-Jacobian group-law certificate.
use serde_json::{json, Value};
use std::fs;

const P: i64 = 523;
const Q3: u64 = 523 * 523 * 523;
fn m(x: i64) -> i64 {
    x.rem_euclid(P)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct K([i64; 3]);
impl K {
    const ZERO: Self = Self([0, 0, 0]);
    const ONE: Self = Self([1, 0, 0]);
    fn c(x: i64) -> Self {
        Self([m(x), 0, 0])
    }
    fn add(self, y: Self) -> Self {
        Self(std::array::from_fn(|i| m(self.0[i] + y.0[i])))
    }
    fn neg(self) -> Self {
        Self(self.0.map(|x| m(-x)))
    }
    fn sub(self, y: Self) -> Self {
        self.add(y.neg())
    }
    fn mul(self, y: Self) -> Self {
        let mut t = [0; 5];
        for i in 0..3 {
            for j in 0..3 {
                t[i + j] += self.0[i] * y.0[j];
            }
        }
        for i in (3..5).rev() {
            t[i - 3] -= 2 * t[i];
        }
        Self([m(t[0]), m(t[1]), m(t[2])])
    }
    fn pow(self, mut n: u64) -> Self {
        let mut x = self;
        let mut y = Self::ONE;
        while n != 0 {
            if n & 1 != 0 {
                y = y.mul(x);
            }
            x = x.mul(x);
            n >>= 1;
        }
        y
    }
    fn inv(self) -> Self {
        assert_ne!(self, Self::ZERO);
        self.pow(Q3 - 2)
    }
    fn from_json(v: &Value) -> Self {
        Self(std::array::from_fn(|i| v[i].as_i64().unwrap()))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: K,
    y: K,
    inf: bool,
}
impl Point {
    const O: Self = Self {
        x: K::ZERO,
        y: K::ZERO,
        inf: true,
    };
}
fn curve_a() -> K {
    K([140, 92, 82])
}
fn curve_b() -> K {
    K([243, 339, 359])
}
fn on_curve(p: Point) -> bool {
    p.inf || p.y.mul(p.y) == p.x.mul(p.x).mul(p.x).add(curve_a().mul(p.x)).add(curve_b())
}
fn add(p: Point, q: Point) -> Point {
    if p.inf {
        return q;
    }
    if q.inf {
        return p;
    }
    if p.x == q.x && p.y.add(q.y) == K::ZERO {
        return Point::O;
    }
    let slope = if p == q {
        p.x.mul(p.x)
            .mul(K::c(3))
            .add(curve_a())
            .mul(p.y.mul(K::c(2)).inv())
    } else {
        q.y.sub(p.y).mul(q.x.sub(p.x).inv())
    };
    let x = slope.mul(slope).sub(p.x).sub(q.x);
    let y = slope.mul(p.x.sub(x)).sub(p.y);
    let out = Point { x, y, inf: false };
    assert!(on_curve(out));
    out
}
fn times(mut p: Point, mut n: u64) -> Point {
    let mut out = Point::O;
    while n != 0 {
        if n & 1 != 0 {
            out = add(out, p);
        }
        p = add(p, p);
        n >>= 1;
    }
    out
}
fn polynomial(v: &Value) -> Vec<K> {
    v.as_array().unwrap().iter().map(K::from_json).collect()
}
fn eval(poly: &[K], x: K) -> K {
    poly.iter().rev().fold(K::ZERO, |acc, c| acc.mul(x).add(*c))
}
fn map_coefficients(fixture: &Value, p: Point) -> Option<[K; 6]> {
    if p.inf {
        return Some([K::ZERO; 6]);
    }
    let denominator = eval(&polynomial(&fixture["denominator"]), p.x);
    if denominator == K::ZERO {
        return None;
    }
    let inverse = denominator.inv();
    let functions = fixture["functions"].as_array().unwrap();
    assert_eq!(functions.len(), 6);
    Some(std::array::from_fn(|i| {
        let a = eval(&polynomial(&functions[i]["x_only"]), p.x);
        let b = eval(&polynomial(&functions[i]["y_factor"]), p.x);
        a.add(p.y.mul(b)).mul(inverse)
    }))
}
fn x_relation(c: &[K; 6]) -> [K; 3] {
    [c[2], c[1].neg(), c[0]]
}
fn y_relation(c: &[K; 6]) -> [K; 3] {
    [c[5], c[4].neg(), c[3]]
}
fn cubic_discriminant(relation: &[K; 3]) -> K {
    // z^3 + a z^2 + b z + c = 0, with z^3 = r0+r1*z+r2*z^2.
    let a = relation[2].neg();
    let b = relation[1].neg();
    let c = relation[0].neg();
    a.mul(a)
        .mul(b.mul(b))
        .sub(K::c(4).mul(b.mul(b).mul(b)))
        .sub(K::c(4).mul(a.mul(a).mul(a)).mul(c))
        .sub(K::c(27).mul(c.mul(c)))
        .add(K::c(18).mul(a.mul(b).mul(c)))
}
fn powers_mod(relation: &[K; 3], max: usize) -> Vec<[K; 3]> {
    let mut powers = vec![[K::ONE, K::ZERO, K::ZERO]];
    for _ in 0..max {
        let before = *powers.last().unwrap();
        let high = before[2];
        powers.push([
            high.mul(relation[0]),
            before[0].add(high.mul(relation[1])),
            before[1].add(high.mul(relation[2])),
        ]);
    }
    powers
}
// Multiplication by the quartic in K[x,y]/(cubic_x,cubic_y).
// Nullity 3 means the cubics cut exactly a degree-three divisor on C,
// when both coordinate cubics are squarefree.
fn quartic_rank(c: &[K; 6]) -> usize {
    let xp = powers_mod(&x_relation(c), 6);
    let yp = powers_mod(&y_relation(c), 6);
    const TERMS: [(usize, usize, i64); 15] = [
        (4, 0, 1),
        (3, 1, 144),
        (2, 2, 112),
        (1, 3, 393),
        (0, 4, 488),
        (3, 0, 397),
        (2, 1, 49),
        (1, 2, 299),
        (0, 3, 500),
        (2, 0, 311),
        (1, 1, 185),
        (0, 2, 288),
        (1, 0, 462),
        (0, 1, 468),
        (0, 0, 284),
    ];
    let mut matrix = [[K::ZERO; 9]; 9];
    for col in 0..9 {
        let ix = col / 3;
        let iy = col % 3;
        for &(a, b, coefficient) in &TERMS {
            for u in 0..3 {
                for v in 0..3 {
                    matrix[3 * u + v][col] = matrix[3 * u + v][col]
                        .add(K::c(coefficient).mul(xp[a + ix][u]).mul(yp[b + iy][v]));
                }
            }
        }
    }
    let mut rank = 0;
    for col in 0..9 {
        let Some(pivot) = (rank..9).find(|&row| matrix[row][col] != K::ZERO) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let inverse = matrix[rank][col].inv();
        for j in col..9 {
            matrix[rank][j] = matrix[rank][j].mul(inverse);
        }
        for row in rank + 1..9 {
            let factor = matrix[row][col];
            if factor != K::ZERO {
                for j in col..9 {
                    matrix[row][j] = matrix[row][j].sub(factor.mul(matrix[rank][j]));
                }
            }
        }
        rank += 1;
    }
    rank
}
fn main() {
    let fixture: Value = serde_json::from_str(
        &fs::read_to_string("research/five_leads_e2e_20261019/fixtures/tian-cubic-map-v1.json")
            .unwrap(),
    )
    .unwrap();
    assert_eq!(fixture["schema"], "tian-cubic-map-coefficients/v1");
    let base = Point {
        x: K::c(7),
        y: K([310, 306, 155]),
        inf: false,
    };
    let published_q = Point {
        x: K([38, 393, 311]),
        y: K([33, 167, 418]),
        inf: false,
    };
    assert!(on_curve(base) && on_curve(published_q));
    assert_eq!(times(base, 1987), published_q);
    assert_eq!(times(base, 143_069_629), Point::O);
    let mut corrupted = map_coefficients(&fixture, base).unwrap();
    corrupted[0] = corrupted[0].add(K::ONE);
    let corrupted_rank = quartic_rank(&corrupted);
    assert_ne!(corrupted_rank, 6, "corrupted map passed support check");
    for scalar in (1..=100).chain(std::iter::once(1987)) {
        let point = times(base, scalar);
        let coefficients = map_coefficients(&fixture, point);
        let (rank, denominator_nonzero, separable) = if let Some(c) = &coefficients {
            let separable = cubic_discriminant(&x_relation(c)) != K::ZERO
                && cubic_discriminant(&y_relation(c)) != K::ZERO;
            assert!(
                separable,
                "coordinate cubic not squarefree for scalar {scalar}"
            );
            let rank = quartic_rank(c);
            assert_eq!(rank, 6, "quartic support rank for scalar {scalar}");
            (Some(rank), true, separable)
        } else {
            (None, false, false)
        };
        println!(
            "{}",
            json!({
                "schema":"tian-cubic-map-gate/v1", "scalar_check_only":scalar,
                "source_point":[point.x.0,point.y.0],"source_verified":point==times(base,scalar),
                "denominator_nonzero":denominator_nonzero,"quartic_multiplication_rank":rank,
                "coordinate_cubics_separable":separable,
                "corrupted_p_rank_control":corrupted_rank,
                "map_coefficients":coefficients.map(|a|a.map(|x|x.0)),
                "jacobian_class_constructed":false,"source_transfer_dlp_executed":false
            })
        );
    }
}
