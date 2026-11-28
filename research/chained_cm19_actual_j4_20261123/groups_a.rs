use crate::arithmetic::{I, K, Q, Z};
use crate::certificate::{point_bytes, Certificate};
use crate::cm::{Matrix, O};
use crate::schemes::{self, Point};
use std::collections::BTreeSet;

pub struct Group {
    pub points: Vec<Point>,
    pub plus: Vec<Vec<usize>>,
    pub negative: Vec<usize>,
    pub frobenius: Vec<usize>,
    pub exponent: usize,
}
impl Group {
    fn new(points: Vec<Point>, exponent: usize) -> Self {
        assert_eq!(points[0], None);
        let index = |p| {
            points
                .iter()
                .position(|&q| q == p)
                .expect("actual full group closed")
        };
        let plus = points
            .iter()
            .map(|&p| points.iter().map(|&q| index(schemes::add(p, q))).collect())
            .collect();
        let negative = points
            .iter()
            .map(|&p| index(schemes::negative(p)))
            .collect();
        let frobenius = points
            .iter()
            .map(|&p| index(schemes::frobenius(p)))
            .collect();
        Self {
            points,
            plus,
            negative,
            frobenius,
            exponent,
        }
    }
    pub fn id(&self, p: Point) -> usize {
        self.points
            .iter()
            .position(|&q| q == p)
            .expect("actual coordinate point")
    }
    pub fn scalar(&self, p: usize, n: usize) -> usize {
        (0..n % self.exponent).fold(0, |s, _| self.plus[s][p])
    }
    pub fn omega(&self, p: usize) -> usize {
        self.plus[self.frobenius[p]][self.scalar(p, 2)]
    }
    pub fn cm_point(&self, p: usize, c: O) -> Point {
        let n = self.exponent as i128;
        schemes::add(
            schemes::times(self.points[p], c.0.rem_euclid(n) as usize),
            schemes::times(
                schemes::add(
                    schemes::frobenius(self.points[p]),
                    schemes::times(self.points[p], 2),
                ),
                c.1.rem_euclid(n) as usize,
            ),
        )
    }
    pub fn qualify(&self) {
        let n = self.points.len();
        for p in 0..n {
            assert!(schemes::valid(self.points[p]));
            assert_eq!(schemes::times(self.points[p], self.exponent), None);
            assert_eq!(self.plus[p][self.negative[p]], 0);
            let w = self.omega(p);
            let ww = self.omega(w);
            assert_eq!(
                self.plus[self.plus[ww][self.negative[w]]][self.scalar(p, 5)],
                0
            );
            for q in 0..n {
                assert_eq!(self.plus[p][q], self.plus[q][p]);
                assert_eq!(
                    self.frobenius[self.plus[p][q]],
                    self.plus[self.frobenius[p]][self.frobenius[q]]
                );
                for r in 0..n {
                    assert_eq!(self.plus[self.plus[p][q]][r], self.plus[p][self.plus[q][r]]);
                }
            }
        }
    }
    pub fn encode(p: [usize; 3], n: usize) -> usize {
        p[0] + n * p[1] + n * n * p[2]
    }
    pub fn decode(mut id: usize, n: usize) -> [usize; 3] {
        assert!(id < n * n * n);
        std::array::from_fn(|_| {
            let digit = id % n;
            id /= n;
            digit
        })
    }
    pub fn point_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for &p in &self.points {
            point_bytes(&mut bytes, p);
        }
        bytes
    }
    pub fn addition_bytes(&self) -> Vec<u8> {
        self.plus
            .iter()
            .flatten()
            .map(|&i| i.try_into().expect("small actual point table"))
            .collect()
    }
}

fn division_factor(x: K) -> K {
    [1, 0, 3, 1, 1, 1, 5]
        .into_iter()
        .fold(Z, |s, c| s.product(x).sum(K::prime(c)))
}
pub fn e4(certificate: &mut Certificate) -> Group {
    let mut roots = Vec::new();
    let mut decisions = vec![0_u8; (Q as usize).div_ceil(8)];
    for i in 0..Q {
        let x = K::encoded(i);
        if division_factor(x) == Z {
            roots.push(x);
            decisions[i as usize / 8] |= 1 << (i % 8);
        }
    }
    certificate.section(b"E4XS", Q as usize, 0, &decisions);
    println!(
        "E4_FULL_SCAN\tall_x={}\tdivision_roots={:?}\tall_decisions_bitpacked_original",
        Q, roots
    );
    assert_eq!(roots.len(), 6);
    let a = K::encoded(7);
    let mut points = BTreeSet::from([None]);
    for x in [a, a.power(7), a.power(49)] {
        let p = Some((x, Z));
        assert!(schemes::valid(p));
        points.insert(p);
    }
    for x in roots {
        assert_eq!(division_factor(x), Z);
        assert_ne!(division_factor(x).sum(I), Z);
        assert_eq!(division_factor(x), Z);
        let rhs = x.power(3).sum(K::prime(2).product(x)).sum(K::prime(6));
        let y = rhs
            .sqrt()
            .expect("all actual fourth-division ordinates in fixed field");
        assert_ne!(y, Z);
        points.insert(Some((x, y)));
        points.insert(Some((x, y.opposite())));
    }
    let points: Vec<_> = points.into_iter().collect();
    let mut payload = Vec::new();
    for &p in &points {
        point_bytes(&mut payload, p);
        println!(
            "ACTUAL_E4_POINT\tindex={}\tP={:?}",
            payload.len() / 9 - 1,
            p
        );
    }
    certificate.section(b"E4PT", points.len(), 9, &payload);
    assert_eq!(points.len(), 16);
    let group = Group::new(points, 4);
    certificate.section(b"E4AD", 256, 1, &group.addition_bytes());
    let mut maps = Vec::new();
    for p in 0..16 {
        maps.extend([
            group.frobenius[p] as u8,
            group.negative[p] as u8,
            group.scalar(p, 2) as u8,
        ]);
    }
    certificate.section(b"E4FR", 16, 3, &maps);
    group.qualify();
    assert_eq!((0..16).filter(|&p| group.scalar(p, 2) == 0).count(), 4);
    assert!((0..16).all(|p| {
        let mut q = p;
        for _ in 0..6 {
            q = group.frobenius[q];
        }
        q == p
    }));
    assert!((0..16).any(|p| group.frobenius[group.frobenius[group.frobenius[p]]] != p));
    group
}

fn f4(p: Point, x: Point, changed: bool) -> Option<K> {
    let Some((px, py)) = p else {
        return Some(I);
    };
    let (x, y) = x?;
    let double = schemes::add(p, p);
    let value = if let Some((rx, _)) = double {
        let denominator = x.difference(rx);
        if denominator == Z {
            return None;
        }
        let slope = K::prime(3)
            .product(px.power(2))
            .sum(K::prime(2))
            .quotient(K::prime(2).product(py));
        let line = y
            .difference(py)
            .difference(slope.product(x.difference(px)))
            .sum(if changed { I } else { Z });
        line.power(2).quotient(denominator)
    } else {
        x.difference(px).power(2)
    };
    (value != Z).then_some(value)
}
fn weil(p: Point, q: Point, t: Point, changed: bool) -> Option<K> {
    let a = f4(p, schemes::add(q, t), changed)?;
    let b = f4(p, t, changed)?;
    let c = f4(q, schemes::negative(t), changed)?;
    let d = f4(q, schemes::add(p, schemes::negative(t)), changed)?;
    Some(a.quotient(b).product(c.quotient(d)))
}
pub fn pairing(group: &Group, certificate: &mut Certificate) -> (Vec<Vec<u8>>, [usize; 2]) {
    let t = Some((K::prime(1), K::prime(3)));
    assert!(schemes::valid(t));
    assert_ne!(t, None);
    assert_eq!(schemes::times(t, 11), None);
    let i = K::encoded(343).product(K::prime(3));
    let values = [I, i, i.power(2), i.power(3)];
    let mut all = vec![vec![0_u8; 16]; 16];
    let mut changed_rejections = 0;
    for (p, row) in all.iter_mut().enumerate() {
        for (q, entry) in row.iter_mut().enumerate() {
            let value =
                weil(group.points[p], group.points[q], t, false).expect("disjoint actual support");
            let alternate = weil(
                group.points[p],
                group.points[q],
                schemes::times(t, 2),
                false,
            )
            .expect("second actual disjoint support");
            assert_eq!(value, alternate);
            *entry = values
                .iter()
                .position(|&v| v == value)
                .expect("actual fourth root of unity") as u8;
            if weil(group.points[p], group.points[q], t, true) != Some(value) {
                changed_rejections += 1;
            }
            assert_eq!(
                weil(group.points[p], group.points[q], t, false),
                Some(value)
            );
            println!(
                "ACTUAL_WEIL4\tP={}\tQ={}\tvalue={:?}\ti_exponent={}\tT_and_2T_agree",
                p, q, value, *entry
            );
        }
    }
    certificate.section(
        b"E4WP",
        256,
        1,
        &all.iter().flatten().copied().collect::<Vec<_>>(),
    );
    assert!(changed_rejections > 0);
    for p in 0..16 {
        assert_eq!(all[p][p], 0);
        if p != 0 {
            assert!(all[p].iter().any(|&e| e != 0));
        }
        for q in 0..16 {
            assert_eq!((all[p][q] + all[q][p]) % 4, 0);
            assert_eq!(
                all[group.frobenius[p]][group.frobenius[q]],
                (7 * all[p][q]) % 4
            );
            for r in 0..16 {
                assert_eq!(all[group.plus[p][q]][r], (all[p][r] + all[q][r]) % 4);
                assert_eq!(all[p][group.plus[q][r]], (all[p][q] + all[p][r]) % 4);
            }
        }
    }
    let p = (1..16)
        .find(|&p| group.scalar(p, 2) != 0)
        .expect("order4 basis first point");
    let q = (1..16)
        .find(|&q| {
            let span: BTreeSet<_> = (0..4)
                .flat_map(|i| {
                    (0..4).map(move |j| group.plus[group.scalar(p, i)][group.scalar(q, j)])
                })
                .collect();
            span.len() == 16
        })
        .expect("full actual basis");
    assert_eq!(all[p][q] % 2, 1);
    println!("WEIL4_CONTROLS\tactual_basis_indices={:?}\tprimitive_value={:?}\tall256_T_2T_values\tbilinear_alternating_nondegenerate_Frobenius\tchanged_line_rejections={}\tpristine_restored\tPASS",[p,q],values[all[p][q] as usize],changed_rejections);
    (all, [p, q])
}

pub struct Action {
    table: Vec<Vec<Vec<usize>>>,
}
impl Action {
    pub fn new(matrix: Matrix, group: &Group) -> Self {
        let table: Vec<Vec<Vec<usize>>> = matrix
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| {
                        (0..group.points.len())
                            .map(|p| group.id(group.cm_point(p, c)))
                            .collect()
                    })
                    .collect()
            })
            .collect();
        for row in &table {
            for action in row {
                for p in 0..group.points.len() {
                    for q in 0..group.points.len() {
                        assert_eq!(action[group.plus[p][q]], group.plus[action[p]][action[q]]);
                    }
                }
            }
        }
        Self { table }
    }
    pub fn apply(&self, group: &Group, p: [usize; 3]) -> [usize; 3] {
        std::array::from_fn(|i| {
            self.table[i]
                .iter()
                .enumerate()
                .fold(0, |s, (j, entry)| group.plus[s][entry[p[j]]])
        })
    }
}
pub fn h_pairing(group: &Group, e: &[Vec<u8>], h: &Action, p: [usize; 3], q: [usize; 3]) -> u8 {
    let image = h.apply(group, q);
    (0..3).fold(0, |s, i| (s + e[p[i]][image[i]]) % 4)
}
pub fn rational(certificate: &mut Certificate) -> Group {
    let t = Some((K::prime(1), K::prime(3)));
    let points: BTreeSet<_> = (0..11).map(|n| schemes::times(t, n)).collect();
    let group = Group::new(points.into_iter().collect(), 11);
    certificate.section(b"E11P", group.points.len(), 9, &group.point_bytes());
    assert_eq!(group.points.len(), 11);
    certificate.section(b"E11A", 121, 1, &group.addition_bytes());
    group.qualify();
    assert_eq!(schemes::times(t, 11), None);
    for (p, &point) in group.points.iter().enumerate() {
        assert_eq!(group.frobenius[p], p);
        println!("ACTUAL_E11_POINT\tindex={}\tP={:?}", p, point);
    }
    group
}
