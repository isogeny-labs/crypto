#[derive(Clone, Debug, PartialEq, Eq)]
struct Jet([E; 35]);
struct JC {
    f: Field,
    es: Vec<[u8; 3]>,
    pairs: Vec<(usize, usize, usize)>,
}
impl JC {
    fn new(f: Field) -> Self {
        let mut es = Vec::new();
        for d in 0..=4 {
            for a in 0..=d {
                for b in 0..=d - a {
                    es.push([a, b, d - a - b])
                }
            }
        }
        assert_eq!(es.len(), 35);
        let mut pairs = Vec::new();
        for (i, a) in es.iter().enumerate() {
            for (j, b) in es.iter().enumerate() {
                let s = std::array::from_fn(|k| a[k] + b[k]);
                if s.iter().sum::<u8>() <= 4 {
                    let k = es.iter().position(|e| *e == s).unwrap();
                    pairs.push((i, j, k))
                }
            }
        }
        Self { f, es, pairs }
    }
    fn z(&self) -> Jet {
        Jet([Z; 35])
    }
    fn c(&self, a: E) -> Jet {
        let mut o = self.z();
        o.0[0] = a;
        o
    }
    fn var(&self, j: usize) -> Jet {
        let mut e = [0; 3];
        e[j] = 1;
        let mut o = self.z();
        o.0[self.es.iter().position(|a| *a == e).unwrap()] = O;
        o
    }
    fn add(&self, a: &Jet, b: &Jet) -> Jet {
        Jet(std::array::from_fn(|i| self.f.add(a.0[i], b.0[i])))
    }
    fn neg(&self, a: &Jet) -> Jet {
        Jet(std::array::from_fn(|i| self.f.neg(a.0[i])))
    }
    fn sub(&self, a: &Jet, b: &Jet) -> Jet {
        self.add(a, &self.neg(b))
    }
    fn scale(&self, a: &Jet, b: E) -> Jet {
        Jet(std::array::from_fn(|i| {
            if a.0[i] == Z {
                Z
            } else {
                self.f.mul(a.0[i], b)
            }
        }))
    }
    fn mul(&self, a: &Jet, b: &Jet) -> Jet {
        let mut o = self.z();
        for &(i, j, k) in &self.pairs {
            if a.0[i] != Z && b.0[j] != Z {
                o.0[k] = self.f.add(o.0[k], self.f.mul(a.0[i], b.0[j]))
            }
        }
        o
    }
    fn inv(&self, a: &Jet) -> Jet {
        assert_ne!(a.0[0], Z);
        let inv = self.f.inv(a.0[0]);
        let mut d = self.scale(a, self.f.neg(inv));
        d.0[0] = Z;
        let mut sum = self.c(O);
        let mut term = self.c(O);
        for _ in 0..4 {
            term = self.mul(&term, &d);
            sum = self.add(&sum, &term)
        }
        self.scale(&sum, inv)
    }
    fn trace(&self, a: &Jet) -> Jet {
        Jet(std::array::from_fn(|i| field_trace(a.0[i], &self.f)))
    }
    fn det(&self, m: &[Vec<Jet>]) -> Jet {
        assert_eq!(m.len(), 3);
        let mut out = self.z();
        for (p, positive) in [
            ([0, 1, 2], true),
            ([1, 2, 0], true),
            ([2, 0, 1], true),
            ([0, 2, 1], false),
            ([1, 0, 2], false),
            ([2, 1, 0], false),
        ] {
            let v = self.mul(&self.mul(&m[0][p[0]], &m[1][p[1]]), &m[2][p[2]]);
            out = if positive {
                self.add(&out, &v)
            } else {
                self.sub(&out, &v)
            }
        }
        out
    }
}
impl Jet {
    fn zero(&self) -> bool {
        self.0.iter().all(|&a| a == Z)
    }
}
#[derive(Clone)]
struct JP(Vec<Jet>);
impl JP {
    fn new(mut v: Vec<Jet>) -> Self {
        while v.last().is_some_and(Jet::zero) {
            v.pop();
        }
        Self(v)
    }
    fn embed(p: &Pol, j: &JC) -> Self {
        Self::new(p.0.iter().map(|&a| j.c(a)).collect())
    }
    fn coeff(&self, i: usize, j: &JC) -> Jet {
        self.0.get(i).cloned().unwrap_or_else(|| j.z())
    }
    fn add(&self, b: &Self, j: &JC) -> Self {
        Self::new(
            (0..self.0.len().max(b.0.len()))
                .map(|i| j.add(&self.coeff(i, j), &b.coeff(i, j)))
                .collect(),
        )
    }
    fn neg(&self, j: &JC) -> Self {
        Self::new(self.0.iter().map(|a| j.neg(a)).collect())
    }
    fn sub(&self, b: &Self, j: &JC) -> Self {
        self.add(&b.neg(j), j)
    }
    fn mul(&self, b: &Self, j: &JC) -> Self {
        if self.0.is_empty() || b.0.is_empty() {
            return Self(Vec::new());
        }
        let mut out = vec![j.z(); self.0.len() + b.0.len() - 1];
        for (i, a) in self.0.iter().enumerate() {
            for (k, b) in b.0.iter().enumerate() {
                out[i + k] = j.add(&out[i + k], &j.mul(a, b))
            }
        }
        Self::new(out)
    }
    fn divrem(&self, b: &Self, j: &JC) -> (Self, Self) {
        assert!(!b.0.is_empty());
        if self.0.len() < b.0.len() {
            return (Self(Vec::new()), self.clone());
        }
        let inv = j.inv(b.0.last().unwrap());
        let mut r = self.clone();
        let mut q = vec![j.z(); self.0.len() - b.0.len() + 1];
        while !r.0.is_empty() && r.0.len() >= b.0.len() {
            let k = r.0.len() - b.0.len();
            let c = j.mul(r.0.last().unwrap(), &inv);
            q[k] = c.clone();
            for (i, a) in b.0.iter().enumerate() {
                r.0[i + k] = j.sub(&r.0[i + k], &j.mul(&c, a))
            }
            r = Self::new(r.0)
        }
        (Self::new(q), r)
    }
    fn rem(&self, b: &Self, j: &JC) -> Self {
        self.divrem(b, j).1
    }
    fn exact(&self, b: &Self, j: &JC) -> Self {
        let (q, r) = self.divrem(b, j);
        assert!(r.0.is_empty());
        q
    }
    fn matrix(&self, u: &Self, j: &JC) -> Vec<Vec<Jet>> {
        assert_eq!(u.0.len(), 4);
        let mut cols = Vec::new();
        let mut a = self.rem(u, j);
        let x = Self(vec![j.z(), j.c(O)]);
        for _ in 0..3 {
            cols.push(a.clone());
            a = a.mul(&x, j).rem(u, j)
        }
        (0..3)
            .map(|i| cols.iter().map(|p| p.coeff(i, j)).collect())
            .collect()
    }
}
fn evaluate_jet(p: &Div, h: &H, u: &JP, v: &JP, j: &JC) -> Jet {
    if p.u == Pol::one() {
        return j.c(O);
    }
    let mut matrix = JP::embed(&p.u, j).matrix(u, j);
    let den = j.det(&matrix);
    let rhs = v.add(&JP::embed(&p.v, j), j).rem(u, j);
    for i in 0..3 {
        matrix[i][2] = rhs.coeff(i, j)
    }
    let r2 = j.mul(&j.det(&matrix), &j.inv(&den));
    let hv = JP::embed(&h.a, j).add(&JP::embed(&h.b, j).mul(v, j), j);
    let norm = j.det(&hv.matrix(u, j));
    j.mul(&j.mul(&j.mul(&r2, &r2), &r2), &norm)
}
fn jet_controls(j: &JC) {
    let mut rng = hec_native_ic::rng::Rng::new(2026111502301);
    for _ in 0..32 {
        let mut a = j.z();
        let mut b = j.z();
        for i in 0..35 {
            a.0[i] = std::array::from_fn(|_| rng.below(P));
            b.0[i] = std::array::from_fn(|_| rng.below(P))
        }
        a.0[0][0] = 1;
        let mut refm = j.z();
        for i in 0..35 {
            for k in 0..35 {
                let s = std::array::from_fn(|l| j.es[i][l] + j.es[k][l]);
                if let Some(at) = j.es.iter().position(|e| *e == s) {
                    refm.0[at] = j.f.add(refm.0[at], j.f.mul(a.0[i], b.0[k]))
                }
            }
        }
        assert_eq!(j.mul(&a, &b), refm);
        assert_eq!(j.mul(&a, &j.inv(&a)), j.c(O))
    }
    let m = vec![
        vec![j.c(base(1)), j.c(base(2)), j.c(base(3))],
        vec![j.c(base(0)), j.c(base(4)), j.c(base(5))],
        vec![j.c(base(1)), j.c(base(0)), j.c(base(6))],
    ];
    assert_eq!(j.det(&m), j.c(base(22)));
    println!("JET_CONTROLS multiplication_pairs=32 inverses=32 determinant=true pass=true");
}
