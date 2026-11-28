use crate::arithmetic::{determinant, K, I, Z};

pub type Point = Option<(K, K)>;
pub type Monomial = [usize; 3];
pub type Polynomial = Vec<(Monomial, K)>;

pub fn negative(p: Point) -> Point {
    p.map(|(x, y)| (x, y.opposite()))
}
pub fn valid(p: Point) -> bool {
    p.is_none_or(|(x, y)| {
        y.product(y) == x.power(3).sum(K::prime(2).product(x)).sum(K::prime(6))
    })
}
pub fn add(a: Point, b: Point) -> Point {
    let (Some((x, y)), Some((u, v))) = (a, b) else {
        return a.or(b);
    };
    if x == u && y.sum(v) == Z {
        return None;
    }
    let slope = if x == u {
        assert_eq!(y, v);
        K::prime(3)
            .product(x.product(x))
            .sum(K::prime(2))
            .quotient(K::prime(2).product(y))
    } else {
        v.difference(y).quotient(u.difference(x))
    };
    let xx = slope.product(slope).difference(x).difference(u);
    let result = Some((xx, slope.product(x.difference(xx)).difference(y)));
    assert!(valid(result));
    result
}
pub fn times(p: Point, n: usize) -> Point {
    (0..n).fold(None, |s, _| add(s, p))
}
pub fn frobenius(p: Point) -> Point {
    p.map(|(x, y)| (x.power(7), y.power(7)))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ring {
    modulus: Vec<K>,
}
impl Ring {
    pub fn new(degree: usize, constant: K) -> Self {
        let mut modulus = vec![Z; degree + 1];
        modulus[0] = constant.opposite();
        modulus[degree] = I;
        Self { modulus }
    }
    pub fn degree(&self) -> usize {
        self.modulus.len() - 1
    }
    pub fn value(&self, mut data: Vec<K>) -> Value {
        while data.len() > self.degree() {
            let last = data.len() - 1;
            let coefficient = data[last];
            for j in 0..self.degree() {
                let k = last - self.degree() + j;
                data[k] = data[k].difference(coefficient.product(self.modulus[j]));
            }
            data.pop();
        }
        data.resize(self.degree(), Z);
        Value { ring: self.clone(), data }
    }
    pub fn scalar(&self, c: K) -> Value {
        self.value(vec![c])
    }
    pub fn variable(&self) -> Value {
        self.value(vec![Z, I])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Value {
    pub ring: Ring,
    pub data: Vec<K>,
}
impl Value {
    pub fn sum(&self, rhs: &Self) -> Self {
        assert_eq!(self.ring, rhs.ring);
        self.ring.value(self.data.iter().zip(&rhs.data).map(|(&a, &b)| a.sum(b)).collect())
    }
    pub fn difference(&self, rhs: &Self) -> Self {
        self.sum(&rhs.scale(K::prime(-1)))
    }
    pub fn scale(&self, c: K) -> Self {
        self.ring.value(self.data.iter().map(|&a| a.product(c)).collect())
    }
    pub fn product(&self, rhs: &Self) -> Self {
        assert_eq!(self.ring, rhs.ring);
        let mut data = vec![Z; self.data.len() + rhs.data.len() - 1];
        for (i, &a) in self.data.iter().enumerate() {
            for (j, &b) in rhs.data.iter().enumerate() {
                data[i + j] = data[i + j].sum(a.product(b));
            }
        }
        self.ring.value(data)
    }
    pub fn power(&self, n: usize) -> Self {
        (0..n).fold(self.ring.scalar(I), |s, _| s.product(self))
    }
    pub fn is_zero(&self) -> bool {
        self.data.iter().all(|&c| c == Z)
    }
    pub fn norm(&self) -> K {
        let d = self.ring.degree();
        let columns: Vec<_> = (0..d).map(|j| self.product(&self.ring.variable().power(j))).collect();
        determinant((0..d).map(|i| columns.iter().map(|c| c.data[i]).collect()).collect())
    }
}

pub fn monomials(d: usize) -> Vec<Monomial> {
    let mut result = Vec::new();
    for non_x in 0..=d {
        for z in 0..=non_x {
            result.push([d - non_x, non_x - z, z]);
        }
    }
    result
}
pub fn conic(c: &[K]) -> Polynomial {
    assert_eq!(c.len(), 6);
    monomials(2).into_iter().zip(c.iter().copied()).collect()
}
pub fn quartic() -> Polynomial {
    [([4,0,0],1),([1,3,0],4),([0,4,0],2),([2,0,2],5),([1,1,2],6),([0,2,2],2),([0,0,4],1)]
        .into_iter().map(|(m,c)| (m,K::prime(c))).collect()
}
pub fn evaluate(poly: &Polynomial, point: &[Value; 3]) -> Value {
    poly.iter().fold(point[0].ring.scalar(Z), |total, (m, c)| {
        let term = (0..3).fold(point[0].ring.scalar(*c), |s, j| s.product(&point[j].power(m[j])));
        total.sum(&term)
    })
}
fn curve(point: &[Value; 3]) {
    assert!(evaluate(&quartic(), point).is_zero());
    let [x,y,z] = point;
    let g = z.power(2).sum(&x.power(2).scale(K::prime(6))).sum(&x.product(y).scale(K::prime(3))).sum(&y.power(2));
    let rhs = y.product(&x.power(3).sum(&x.product(&y.power(2)).scale(K::prime(2))).sum(&y.power(3).scale(K::prime(6))));
    assert!(g.power(2).difference(&rhs).is_zero());
}
pub fn constraints(point: &[Value; 3]) -> Vec<Vec<K>> {
    let values: Vec<_> = monomials(2).into_iter().map(|m| evaluate(&vec![(m,I)],point)).collect();
    (0..point[0].ring.degree()).map(|i| values.iter().map(|v|v.data[i]).collect()).collect()
}
pub fn multiples(poly: &Polynomial, degree: usize) -> Vec<Vec<K>> {
    let total = poly[0].0.iter().sum::<usize>() + degree;
    let basis = monomials(total);
    monomials(degree).into_iter().map(|shift| {
        let mut row = vec![Z;basis.len()];
        for (m,c) in poly {
            let exponent = std::array::from_fn(|i|m[i]+shift[i]);
            let j = basis.iter().position(|m|*m==exponent).expect("canonical monomial");
            row[j] = row[j].sum(*c);
        }
        row
    }).collect()
}

fn linear(matrix: [[i64;3];3], p: &[Value;3]) -> [Value;3] {
    matrix.map(|row| row.into_iter().zip(p).fold(p[0].ring.scalar(Z),|s,(c,v)|s.sum(&v.scale(K::prime(c)))))
}
fn deck(p: &[Value;3], target: (K,K)) {
    let [x,y,z] = p;
    assert!(x.difference(&y.scale(target.0)).is_zero());
    let ordinate = z.power(2).sum(&x.power(2).scale(K::prime(6))).sum(&x.product(y).scale(K::prime(3))).sum(&y.power(2));
    assert!(ordinate.difference(&y.power(2).scale(target.1)).is_zero());
    assert_ne!(y.norm(),Z);
}
fn quotient(p: &[Value;3], target: (K,K)) {
    let nx = p[1].scale(K::prime(3));
    assert!(nx.data[1..].iter().all(|&c|c==Z));
    let ni = nx.data[0].inverse();
    let ny = p[0].difference(&nx).sum(&p[2]).scale(K::prime(4).product(ni));
    let nz = p[0].difference(&nx).difference(&p[2]).scale(K::prime(4).product(ni));
    let u = ny.product(&nz);
    let v = ny.power(3);
    let w = v.scale(K::prime(4)).sum(&u.ring.scalar(I)).sum(&u.scale(K::prime(3))).sum(&u.power(2).scale(K::prime(2)));
    let delta = w.sum(&u.power(2).scale(K::prime(2))).sum(&u.scale(K::prime(6))).sum(&u.ring.scalar(K::prime(3)));
    assert_eq!(delta.sum(&u.ring.scalar(I)),u.ring.scalar(target.0));
    assert_eq!(u.scale(K::prime(5)).sum(&u.ring.scalar(K::prime(4))).product(&delta).sum(&u.ring.scalar(K::prime(4))),u.ring.scalar(target.1));
}

pub struct Fibres {
    pub f0: [Value;3],
    pub f1: [Value;3],
    pub f3: [Value;3],
    pub jet: [Value;3],
}
pub fn fibres(point: (K,K), target: (K,K)) -> Fibres {
    let (x,y) = point;
    let g = K::prime(6).product(x.product(x)).sum(K::prime(3).product(x)).sum(I);
    let r0 = Ring::new(2,y.difference(g));
    let f0 = [r0.scalar(x),r0.scalar(I),r0.variable()];
    curve(&f0);deck(&f0,point);
    let f1 = linear([[1,5,5],[0,5,0],[5,6,1]],&f0);
    curve(&f1);deck(&linear([[1,5,2],[0,5,0],[2,1,1]],&f1),point);
    let delta = target.0.difference(I);
    assert_ne!(delta,Z);
    let u = K::prime(3).product(target.1.difference(K::prime(4)).quotient(delta).difference(K::prime(4)));
    let w = delta.difference(K::prime(2).product(u.power(2))).difference(K::prime(6).product(u)).difference(K::prime(3));
    let v = w.difference(I).difference(K::prime(3).product(u)).difference(K::prime(2).product(u.power(2))).quotient(K::prime(4));
    let (r3,ny,nz) = if v==Z {
        assert_eq!(u,Z);
        let r = Ring::new(3,K::prime(3));
        (r.clone(),r.scalar(Z),r.variable())
    } else {
        let r = Ring::new(3,v);
        (r.clone(),r.variable(),r.variable().power(2).scale(u.quotient(v)))
    };
    let f3 = [r3.scalar(I).sum(&ny).sum(&nz),r3.scalar(K::prime(5)),ny.difference(&nz)];
    curve(&f3);quotient(&f3,target);quotient(&linear([[1,5,2],[0,5,0],[2,1,1]],&f3),target);
    let jet_ring = Ring::new(3,Z);
    let t = jet_ring.variable();
    let jet = [jet_ring.scalar(I).sum(&t).difference(&t.power(2)),t.power(2).scale(K::prime(2)),t.difference(&jet_ring.scalar(I))];
    curve(&jet);
    assert_ne!(K::prime(2),Z); // normal quartic derivative in x at (0:0:1)
    Fibres {f0,f1,f3,jet}
}
