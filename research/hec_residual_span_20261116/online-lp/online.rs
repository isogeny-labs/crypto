use hec_native_ic::fp::Fp;
struct Equation {
    lp: Vec<(usize, u64)>,
    label: Vec<u64>,
}
pub struct Online {
    f: Fp,
    small: usize,
    n: usize,
    pivots: Vec<Option<Equation>>,
    labels: Vec<Option<Vec<u64>>>,
    pub lp_rank: usize,
    pub label_rank: usize,
    pub max_lp_width: usize,
}
impl Online {
    pub fn new(small: usize, n: usize, ell: u64) -> Self {
        Self {
            f: Fp::new(ell),
            small,
            n,
            pivots: (0..n).map(|_| None).collect(),
            labels: vec![None; small + 1],
            lp_rank: 0,
            label_rank: 0,
            max_lp_width: 0,
        }
    }
    pub fn k(&self) -> Option<u64> {
        self.labels[self.small].as_ref().map(|v| v[self.small + 1])
    }
    pub fn cells(&self) -> usize {
        self.pivots
            .iter()
            .filter_map(|v| v.as_ref())
            .map(|e| e.lp.len() + e.label.len())
            .sum::<usize>()
            + self
                .labels
                .iter()
                .filter_map(|v| v.as_ref())
                .map(Vec::len)
                .sum::<usize>()
    }
    pub fn insert(&mut self, full: &[u64], rhs: u64) {
        assert_eq!(full.len(), self.small + self.n + 1);
        let mut lp: Vec<_> = full[self.small..self.small + self.n]
            .iter()
            .enumerate()
            .filter_map(|(j, &v)| (v != 0).then_some((j, v)))
            .collect();
        assert!(lp.len() <= 2);
        let mut label = full[..self.small].to_vec();
        label.push(full[self.small + self.n]);
        label.push(rhs);
        while !lp.is_empty() {
            let (j, a) = lp[0];
            if let Some(p) = &self.pivots[j] {
                for &(col, b) in &p.lp {
                    let at = lp.binary_search_by_key(&col, |&(i, _)| i);
                    let current = at.map(|i| lp[i].1).unwrap_or(0);
                    let value = self.f.sub(current, self.f.mul(a, b));
                    match (at, value) {
                        (Ok(i), 0) => {
                            lp.remove(i);
                        }
                        (Ok(i), v) => lp[i].1 = v,
                        (Err(_), 0) => {}
                        (Err(i), v) => lp.insert(i, (col, v)),
                    }
                }
                for k in 0..label.len() {
                    label[k] = self.f.sub(label[k], self.f.mul(a, p.label[k]))
                }
                assert!(lp.len() <= 2)
            } else {
                let inv = self.f.inv(a);
                for (_, v) in &mut lp {
                    *v = self.f.mul(*v, inv)
                }
                for v in &mut label {
                    *v = self.f.mul(*v, inv)
                }
                self.max_lp_width = self.max_lp_width.max(lp.len());
                assert!(self.max_lp_width <= 2);
                self.pivots[j] = Some(Equation { lp, label });
                self.lp_rank += 1;
                return;
            }
        }
        for j in 0..self.small + 1 {
            let a = label[j];
            if a == 0 {
                continue;
            }
            if let Some(p) = &self.labels[j] {
                for k in j..label.len() {
                    label[k] = self.f.sub(label[k], self.f.mul(a, p[k]))
                }
            } else {
                let inv = self.f.inv(a);
                for k in j..label.len() {
                    label[k] = self.f.mul(label[k], inv)
                }
                self.labels[j] = Some(label);
                self.label_rank += 1;
                return;
            }
        }
        assert_eq!(label[self.small + 1], 0, "inconsistent partial relation");
    }
}

// Stronger control: incremental full coefficient matrix, no LP-first structure.
pub struct Full {
    f: Fp,
    n: usize,
    basis: Vec<Option<Vec<u64>>>,
    pub rank: usize,
}
impl Full {
    pub fn new(n: usize, ell: u64) -> Self {
        Self {
            f: Fp::new(ell),
            n,
            basis: vec![None; n],
            rank: 0,
        }
    }
    pub fn k(&self) -> Option<u64> {
        self.basis[self.n - 1].as_ref().map(|v| v[self.n])
    }
    pub fn cells(&self) -> usize {
        self.basis
            .iter()
            .filter_map(|v| v.as_ref())
            .map(Vec::len)
            .sum()
    }
    pub fn insert(&mut self, full: &[u64], rhs: u64) {
        assert_eq!(full.len(), self.n);
        let mut row = full.to_vec();
        row.push(rhs);
        for j in 0..self.n {
            let a = row[j];
            if a == 0 {
                continue;
            }
            if let Some(p) = &self.basis[j] {
                for k in j..row.len() {
                    row[k] = self.f.sub(row[k], self.f.mul(a, p[k]))
                }
            } else {
                let inv = self.f.inv(a);
                for k in j..row.len() {
                    row[k] = self.f.mul(row[k], inv)
                }
                self.basis[j] = Some(row);
                self.rank += 1;
                return;
            }
        }
        assert_eq!(row[self.n], 0, "inconsistent full relation")
    }
}
