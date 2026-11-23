use crate::linear::{Linear, Sparse};
use hec_native_ic::fp::Fp;
struct Expr {
    terms: Vec<(usize, u64)>,
    children: Vec<(usize, u64)>,
    scale: u64,
    value: Option<Vec<u64>>,
}
pub struct Dag {
    pub f: Fp,
    m: usize,
    root: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
    weight: Vec<u64>,
    link: Vec<usize>,
    nodes: Vec<Expr>,
    basis: Vec<Option<Vec<u64>>>,
    audit: Option<Linear>,
    eager: bool,
    pub constraints: usize,
    pub joins: usize,
    pub expanded: usize,
}
fn mul(f: &Fp, a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else if a == 1 {
        b
    } else if b == 1 {
        a
    } else if a == f.p - 1 {
        f.neg(b)
    } else if b == f.p - 1 {
        f.neg(a)
    } else {
        f.mul(a, b)
    }
}
impl Dag {
    pub fn new(m: usize, n: usize, ell: u64, audit: bool, eager: bool) -> Self {
        let root = n - m;
        Self {
            f: Fp::new(ell),
            m,
            root,
            parent: (0..=root).collect(),
            size: vec![1; root + 1],
            weight: vec![1; root + 1],
            link: vec![0; root + 1],
            nodes: vec![],
            basis: vec![None; m + 1],
            audit: audit.then(|| Linear::new(m, n, ell)),
            eager,
            constraints: 0,
            joins: 0,
            expanded: 0,
        }
    }
    pub fn ready(&self, row: &Sparse) -> bool {
        let mut roots = [usize::MAX; 2];
        let mut ok = true;
        for (&j, _) in row.range(self.m + 1..) {
            let mut r = j - self.m - 1;
            while r != self.parent[r] {
                r = self.parent[r];
            }
            if r != self.root && !roots.contains(&r) {
                if let Some(slot) = roots.iter_mut().find(|x| **x == usize::MAX) {
                    *slot = r;
                } else {
                    ok = false;
                    break;
                }
            }
        }
        if let Some(l) = &self.audit {
            assert_eq!(l.ready(row), ok);
        }
        ok
    }
    fn expand(&mut self, id: usize) {
        let mut stack = vec![(id, false)];
        while let Some((n, visited)) = stack.pop() {
            let expr = &self.nodes[n];
            if expr.value.is_some() {
                continue;
            }
            if !visited {
                stack.push((n, true));
                for &(ch, _) in &expr.children {
                    if self.nodes[ch].value.is_none() {
                        stack.push((ch, false));
                    }
                }
                continue;
            }
            let mut v = vec![0; self.m + 2];
            for &(j, t) in &expr.terms {
                v[j] = t;
            }
            for &(child, w) in &expr.children {
                for (z, &y) in v.iter_mut().zip(self.nodes[child].value.as_ref().unwrap()) {
                    if y != 0 {
                        *z = self.f.add(*z, mul(&self.f, w, y));
                    }
                }
            }
            if expr.scale != 1 {
                for z in &mut v {
                    *z = mul(&self.f, expr.scale, *z);
                }
            }
            self.nodes[n].value = Some(v);
            self.expanded += 1;
        }
    }
    fn insert_small(&mut self, mut v: Vec<u64>) {
        for j in 0..=self.m {
            let a = v[j];
            if a == 0 {
                continue;
            }
            if let Some(row) = &self.basis[j] {
                v[j] = 0;
                for i in j + 1..v.len() {
                    if row[i] != 0 {
                        v[i] = self.f.sub(v[i], mul(&self.f, a, row[i]));
                    }
                }
            } else {
                let inv = self.f.inv(a);
                for z in &mut v[j..] {
                    *z = mul(&self.f, inv, *z);
                }
                self.basis[j] = Some(v);
                return;
            }
        }
        assert_eq!(v[self.m + 1], 0);
    }
    pub fn push(&mut self, row: Sparse, rhs: u64, audit: bool) -> Option<u64> {
        assert!(self.ready(&row));
        let expected = self.audit.as_mut().map(|l| l.push(row.clone(), rhs, audit));
        let mut expr = Expr {
            terms: row.range(..=self.m).map(|(&j, &a)| (j, a)).collect(),
            children: vec![],
            scale: 1,
            value: None,
        };
        if rhs != 0 {
            expr.terms.push((self.m + 1, self.f.neg(rhs)));
        }
        let mut roots: Vec<(usize, u64)> = vec![];
        for (&j, &a) in row.range(self.m + 1..) {
            let mut r = j - self.m - 1;
            let mut w = a;
            while r != self.parent[r] {
                expr.children.push((self.link[r], w));
                w = mul(&self.f, w, self.weight[r]);
                r = self.parent[r];
            }
            if r != self.root {
                if let Some(v) = roots.iter_mut().find(|v| v.0 == r) {
                    v.1 = self.f.add(v.1, w)
                } else {
                    roots.push((r, w));
                }
            }
        }
        roots.retain(|v| v.1 != 0);
        assert!(roots.len() <= 2);
        let id = self.nodes.len();
        if roots.is_empty() {
            self.nodes.push(expr);
            self.expand(id);
            let v = self.nodes.pop().unwrap().value.unwrap();
            self.insert_small(v);
            self.constraints += 1;
        } else {
            if roots.len() == 2 && self.size[roots[0].0] > self.size[roots[1].0] {
                roots.swap(0, 1)
            }
            let (r, a) = roots[0];
            expr.scale = self.f.neg(self.f.inv(a));
            let (other, w) = roots.get(1).copied().unwrap_or((self.root, 0));
            self.parent[r] = other;
            self.weight[r] = mul(&self.f, w, expr.scale);
            self.link[r] = id;
            self.size[other] += self.size[r];
            self.nodes.push(expr);
            self.joins += 1;
            if self.eager {
                self.expand(id);
            }
        }
        let k = self.basis[self.m]
            .as_ref()
            .map(|v| self.f.neg(v[self.m + 1]));
        if let Some(e) = expected {
            assert_eq!(k, e);
        }
        k
    }
    pub fn expanded_links(&self) -> usize {
        self.expanded - self.constraints
    }
}
