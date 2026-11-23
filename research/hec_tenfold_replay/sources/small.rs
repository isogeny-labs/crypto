use hec_native_ic::fp::Fp;
use std::collections::BTreeMap;
pub struct Sparse {
    f: Fp,
    n: usize,
    basis: Vec<Option<BTreeMap<usize, u64>>>,
    pub rank: usize,
}
impl Sparse {
    pub fn new(n: usize, ell: u64) -> Self {
        Self {
            f: Fp::new(ell),
            n,
            basis: vec![None; n],
            rank: 0,
        }
    }
    pub fn k(&self) -> Option<u64> {
        self.basis[self.n - 1]
            .as_ref()
            .map(|r| *r.get(&self.n).unwrap_or(&0))
    }
    pub fn insert(&mut self, v: &[u64], rhs: u64) {
        assert_eq!(v.len(), self.n);
        let mut r: BTreeMap<_, _> = v
            .iter()
            .copied()
            .chain(std::iter::once(rhs))
            .enumerate()
            .filter(|(_, x)| *x != 0)
            .collect();
        loop {
            let Some((&j, &a)) = r.first_key_value() else {
                return;
            };
            assert!(j < self.n, "inconsistent sparse row");
            if let Some(p) = &self.basis[j] {
                for (&k, &b) in p {
                    let x = self.f.sub(*r.get(&k).unwrap_or(&0), self.f.mul(a, b));
                    if x == 0 {
                        r.remove(&k);
                    } else {
                        r.insert(k, x);
                    }
                }
            } else {
                let inv = self.f.inv(a);
                for x in r.values_mut() {
                    *x = self.f.mul(*x, inv)
                }
                self.basis[j] = Some(r);
                self.rank += 1;
                return;
            }
        }
    }
}
impl Sparse{pub fn line(&self)->(Vec<u64>,Vec<u64>){assert_eq!(self.rank+1,self.n);assert!(self.k().is_none());let mut a=vec![0;self.n];let mut b=vec![0;self.n];b[self.n-1]=1;for i in(0..self.n-1).rev(){let p=self.basis[i].as_ref().unwrap();assert_eq!(p.get(&i),Some(&1));a[i]=p.get(&self.n).copied().unwrap_or(0);for(&j,&v)in p.range(i+1..self.n){if a[j]!=0{a[i]=self.f.sub(a[i],self.f.mul(v,a[j]));}if b[j]!=0{b[i]=self.f.sub(b[i],self.f.mul(v,b[j]));}}}(a,b)}}
