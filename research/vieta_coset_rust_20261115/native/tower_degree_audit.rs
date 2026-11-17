//! Separate finite constructible-space calculation, using direct modular
//! arithmetic and sparse polynomial row reduction, not the Vieta solver.
//! Producer-owned evidence: this cannot grant independent signed standing.
use serde_json::json;
use std::collections::{BTreeMap, VecDeque};
type Monomial = Vec<u16>;
type Poly = BTreeMap<Monomial, u64>;
struct Ring {
    p: u64,
    n: usize,
}
impl Ring {
    fn plus(&self, a: u64, b: u64) -> u64 {
        (a + b) % self.p
    }
    fn times(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn minus(&self, a: u64, b: u64) -> u64 {
        (a + self.p - b) % self.p
    }
    fn inv(&self, a: u64) -> u64 {
        assert_ne!(a, 0);
        let (mut x, mut r, mut e) = (a, 1, self.p - 2);
        while e > 0 {
            if e & 1 == 1 {
                r = self.times(r, x);
            }
            x = self.times(x, x);
            e >>= 1;
        }
        r
    }
    fn term(&self, c: u64, v: Option<usize>) -> Poly {
        let mut m = vec![0; self.n];
        if let Some(i) = v {
            m[i] = 1;
        }
        if c.is_multiple_of(self.p) {
            Poly::new()
        } else {
            BTreeMap::from([(m, c % self.p)])
        }
    }
    fn add(&self, a: &Poly, b: &Poly) -> Poly {
        let mut r = a.clone();
        for (m, c) in b {
            let x = r.entry(m.clone()).or_default();
            *x = self.plus(*x, *c);
        }
        r.retain(|_, c| *c != 0);
        r
    }
    fn scale(&self, a: &Poly, c: u64) -> Poly {
        a.iter()
            .filter_map(|(m, x)| {
                let v = self.times(*x, c);
                (v != 0).then_some((m.clone(), v))
            })
            .collect()
    }
    fn sub(&self, a: &Poly, b: &Poly) -> Poly {
        self.add(a, &self.scale(b, self.p - 1))
    }
    fn mul(&self, a: &Poly, b: &Poly) -> Poly {
        let mut r = Poly::new();
        for (m, c) in a {
            for (n, d) in b {
                let e = m.iter().zip(n).map(|(x, y)| x + y).collect::<Vec<_>>();
                let v = r.entry(e).or_default();
                *v = self.plus(*v, self.times(*c, *d));
            }
        }
        r.retain(|_, c| *c != 0);
        r
    }
    fn univariate(&self, a: &[u64], i: usize) -> Poly {
        a.iter()
            .enumerate()
            .filter_map(|(e, c)| {
                let mut m = vec![0; self.n];
                m[i] = e as u16;
                (*c != 0).then_some((m, *c))
            })
            .collect()
    }
    fn eval(&self, a: &[u64], x: u64) -> u64 {
        a.iter()
            .rev()
            .fold(0, |y, c| self.plus(self.times(y, x), *c))
    }
}
fn degree(a: &Poly) -> usize {
    a.keys()
        .map(|m| m.iter().map(|x| *x as usize).sum())
        .max()
        .unwrap_or(0)
}
fn lead(a: &Poly) -> Option<(Monomial, u64)> {
    a.iter()
        .max_by_key(|(m, _)| (m.iter().map(|x| *x as usize).sum::<usize>(), *m))
        .map(|(m, c)| (m.clone(), *c))
}
struct Space<'a> {
    ring: &'a Ring,
    basis: BTreeMap<Monomial, Poly>,
    cap: usize,
}
impl Space<'_> {
    fn remainder(&self, mut f: Poly) -> Poly {
        while let Some((m, c)) = lead(&f) {
            let Some(row) = self.basis.get(&m) else {
                break;
            };
            f = self.ring.sub(&f, &self.ring.scale(row, c));
        }
        f
    }
    fn close(&mut self, generators: Vec<Poly>) {
        let mut todo: VecDeque<_> = generators.into();
        while let Some(f) = todo.pop_front() {
            assert!(degree(&f) <= self.cap);
            let f = self.remainder(f);
            let Some((m, c)) = lead(&f) else {
                continue;
            };
            let f = self.ring.scale(&f, self.ring.inv(c));
            if degree(&f) < self.cap {
                for i in 0..self.ring.n {
                    todo.push_back(self.ring.mul(&f, &self.ring.term(1, Some(i))));
                }
            }
            self.basis.insert(m, f);
        }
    }
    fn contains(&self, f: Poly) {
        assert!(
            self.remainder(f).is_empty(),
            "missing degree-bounded constructible-space relation"
        );
    }
}
#[derive(Clone)]
struct Map {
    n: Vec<u64>,
    d: Vec<u64>,
}
fn fiber(r: &Ring, map: &Map, child: u64) -> Vec<u64> {
    map.n
        .iter()
        .enumerate()
        .map(|(i, n)| r.minus(*n, r.times(child, *map.d.get(i).unwrap_or(&0))))
        .collect()
}
fn roots(r: &Ring, a: &[u64]) -> Vec<u64> {
    (0..r.p).filter(|x| r.eval(a, *x) == 0).collect()
}
fn paths(r: &Ring, maps: &[Map], terminal: u64) -> Vec<Vec<u64>> {
    let mut paths = vec![vec![]];
    for map in maps.iter().rev() {
        let mut next = vec![];
        for path in paths {
            let child = path.first().copied().unwrap_or(terminal);
            let rr = roots(r, &fiber(r, map, child));
            assert_eq!(rr.len(), map.n.len() - 1);
            for x in rr {
                assert_ne!(r.eval(&map.d, x), 0);
                let mut new = vec![x];
                new.extend(&path);
                next.push(new);
            }
        }
        paths = next;
    }
    paths
}
fn projector(
    r: &Ring,
    maps: &[Map],
    terminal: u64,
    path: &[u64],
    offset: usize,
) -> (Poly, Vec<Poly>) {
    let one = r.term(1, None);
    let mut es = vec![];
    let mut gammas = vec![];
    let mut gates = vec![];
    for (i, map) in maps.iter().enumerate() {
        let child = path.get(i + 1).copied().unwrap_or(terminal);
        let p = fiber(r, map, child);
        let q = p.len() - 1;
        let mut quotient = vec![0; q];
        quotient[q - 1] = p[q];
        for k in (1..q).rev() {
            quotient[k - 1] = r.plus(p[k], r.times(path[i], quotient[k]));
        }
        assert_eq!(r.plus(p[0], r.times(path[i], quotient[0])), 0);
        let gamma = r.inv(r.eval(&quotient, path[i]));
        gammas.push(gamma);
        es.push(r.scale(&r.univariate(&quotient, offset + i), gamma));
        let d = r.univariate(&map.d, offset + i);
        let childvar = if i + 1 == maps.len() {
            r.term(terminal, None)
        } else {
            r.term(1, Some(offset + i + 1))
        };
        gates.push(r.sub(&r.univariate(&map.n, offset + i), &r.mul(&childvar, &d)));
    }
    let e = es.iter().fold(one.clone(), |v, e| r.mul(&v, e));
    let s: usize = maps.iter().map(|m| m.n.len() - 2).sum();
    for (j, value) in path.iter().enumerate() {
        let left = r.mul(
            &r.sub(&r.term(1, Some(offset + j)), &r.term(*value, None)),
            &e,
        );
        let mut rhs = Poly::new();
        let mut gamma = 1;
        for k in j..maps.len() {
            gamma = r.times(gamma, gammas[k]);
            let mut factor = r.term(gamma, None);
            for (i, ei) in es.iter().enumerate() {
                if i < j || i > k {
                    factor = r.mul(&factor, ei);
                } else if i < k {
                    factor = r.mul(&factor, &r.univariate(&maps[i].d, offset + i));
                }
            }
            let term = r.mul(&factor, &gates[k]);
            assert!(degree(&term) <= s + 1);
            rhs = r.add(&rhs, &term);
        }
        assert_eq!(left, rhs);
    }
    (e, gates)
}
fn monomial_count(n: usize, t: usize) -> usize {
    (1..=n).fold(1usize, |a, i| a * (t + i) / i)
}
fn audit(
    name: &str,
    p: u64,
    maps: Vec<Map>,
    terminal: u64,
    a: u64,
    t: u64,
    ordinate: u64,
    closure: bool,
) -> serde_json::Value {
    let d = maps.len();
    let ring = Ring { p, n: 2 * d + 1 };
    let r = &ring;
    let paths = paths(r, &maps, terminal);
    let s: usize = maps.iter().map(|m| m.n.len() - 2).sum();
    let cap = 2 * s + 2;
    let mut first = vec![];
    let mut second = vec![];
    let mut generators = vec![];
    let mut partition = Poly::new();
    for path in &paths {
        let (e, g) = projector(r, &maps, terminal, path, 0);
        partition = r.add(&partition, &e);
        first.push(e);
        let (e, gg) = projector(r, &maps, terminal, path, d);
        second.push(e);
        if generators.is_empty() {
            generators = g;
            generators.extend(gg);
        }
    }
    let one = r.term(1, None);
    assert_eq!(partition, one);
    let l = r.term(1, Some(2 * d));
    let x = r.term(1, Some(0));
    let y = r.term(1, Some(d));
    let lsq = r.mul(&l, &l);
    generators.push(r.sub(&lsq, &r.add(&r.add(&x, &y), &r.term(t, None))));
    generators.push(r.sub(
        &r.sub(
            &r.mul(&r.sub(&x, &r.term(t, None)), &r.sub(&y, &r.term(t, None))),
            &r.term(r.plus(r.times(3, r.times(t, t)), a), None),
        ),
        &r.scale(&l, r.times(2, ordinate)),
    ));
    let mut space = Space {
        ring: r,
        basis: BTreeMap::new(),
        cap,
    };
    if closure {
        space.close(generators);
    }
    let mut dimension = 0;
    let mut nilpotent_blocks = 0;
    let mut nonsplit_blocks = 0;
    let mut goodsum = Poly::new();
    let mut actions = 0;
    for (i, pa) in paths.iter().enumerate() {
        for (j, pb) in paths.iter().enumerate() {
            let e = r.mul(&first[i], &second[j]);
            let q = r.plus(r.plus(pa[0], pb[0]), t);
            let k = r.minus(
                r.times(r.minus(pa[0], t), r.minus(pb[0], t)),
                r.plus(r.times(3, r.times(t, t)), a),
            );
            let ell = if ordinate != 0 {
                Some(r.times(k, r.inv(r.times(2, ordinate))))
            } else {
                None
            };
            let good = ell.map_or(k == 0, |ell| r.times(ell, ell) == q);
            if !good {
                if closure {
                    space.contains(e);
                }
                continue;
            }
            goodsum = r.add(&goodsum, &e);
            let mut reps = vec![e.clone()];
            if ell.is_none() {
                reps.push(r.mul(&l, &e));
                nilpotent_blocks += usize::from(q == 0);
                nonsplit_blocks += usize::from(roots(r, &[r.minus(0, q), 0, 1]).is_empty());
            }
            dimension += reps.len();
            if closure {
                for (power, w) in reps.iter().enumerate() {
                    for z in 0..r.n {
                        let nf = if z < 2 * d {
                            let value = if z < d { pa[z] } else { pb[z - d] };
                            r.scale(w, value)
                        } else if let Some(ell) = ell {
                            r.scale(&e, ell)
                        } else if power == 0 {
                            r.mul(&l, &e)
                        } else {
                            r.scale(&e, q)
                        };
                        space.contains(r.sub(&r.mul(&r.term(1, Some(z)), w), &nf));
                        actions += 1;
                    }
                }
            }
        }
    }
    if closure {
        space.contains(r.sub(&one, &goodsum));
        assert_eq!(monomial_count(r.n, cap) - space.basis.len(), dimension);
    }
    json!({"case":name,"p":p,"map_degrees":maps.iter().map(|m|m.n.len()-1).collect::<Vec<_>>(),"leaves":paths.len(),"S":s,"T":cap,"coordinate_certificates":paths.len()*d*2,"partition_literal_identity":true,"constructible_closure_computed":closure,"rank":if closure{Some(space.basis.len())}else{None},"predicted_algebra_dimension":dimension,"nilpotent_blocks":nilpotent_blocks,"nonsplit_blocks":nonsplit_blocks,"basis_actions_checked_in_V_T":actions,"standing":"producer finite exact check [pcc:unverified], not a general proof or independent signature"})
}
fn main() {
    let quadratic = Map {
        n: vec![1, 0, 1],
        d: vec![0, 2],
    };
    let square = Map {
        n: vec![0, 0, 1],
        d: vec![1],
    };
    let cubic = Map {
        n: vec![0, 6, 0, 2],
        d: vec![2, 0, 6],
    };
    let cases = vec![
        audit(
            "rational-prime-curve",
            17,
            vec![quadratic.clone(), quadratic.clone()],
            4,
            2,
            6,
            14,
            true,
        ),
        audit(
            "rational-two-torsion",
            17,
            vec![quadratic.clone(), quadratic.clone()],
            4,
            1,
            0,
            0,
            true,
        ),
        audit(
            "nilpotent-two-torsion",
            17,
            vec![square.clone()],
            4,
            13,
            0,
            0,
            true,
        ),
        audit("nonsplit-two-torsion", 17, vec![square], 9, 9, 0, 0, true),
        audit(
            "cubic-rational",
            97,
            vec![cubic.clone()],
            29,
            2,
            6,
            14,
            true,
        ),
        audit(
            "mixed-quadratic-cubic",
            97,
            vec![quadratic, cubic],
            58,
            2,
            6,
            14,
            false,
        ),
    ];
    println!(
        "{}",
        json!({"schema":"vieta-degree-audit/v1","method":"Literal projectors and coordinate identities, independently structured finite V_T closure from the original generators","general_independent_signed_standing":false,"cases":cases})
    );
}
