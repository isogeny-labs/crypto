pub struct Sections {
    v: Vec<Poly>,
    h: Vec<Vec<u64>>,
    ts: Vec<[usize; 3]>,
}
impl Sections {
    pub fn new() -> Self {
        let (v, h, ts) = build();
        Self { v, h, ts }
    }
    pub fn coords(&self, u: [u64; 4], y: [u64; 3]) -> Option<[u64; 8]> {
        if ug(u.to_vec(), ux(&u)).len() != 1 || !eval_u(&curve(), &y, &u).is_empty() {
            return None;
        }
        let f = curve();
        let mut fy = [0; 49];
        for (i, j) in monos(4) {
            if j > 0 {
                fy[7 * i + j - 1] = mul(j as u64, f[7 * i + j]);
            }
        }
        if ug(eval_u(&fy, &y, &u), u.to_vec()).len() != 1 {
            return None;
        }
        let (a, _) = corrected(&self.v, u, y);
        let ev: Vec<_> = self
            .ts
            .iter()
            .map(|t| polar(a[t[0]], a[t[1]], a[t[2]], u))
            .collect();
        let c = std::array::from_fn(|i| {
            self.h[i]
                .iter()
                .zip(&ev)
                .fold(0, |z, (a, b)| add(z, mul(*a, *b)))
        });
        c.iter().any(|x| *x != 0).then_some(c)
    }
}
pub fn root_degree(u: [u64; 4]) -> usize {
    let mut a = vec![1];
    let mut x = vec![0, 1];
    let mut e = P;
    while e > 0 {
        if e & 1 != 0 {
            a = ur(&um(&a, &x), &u)
        }
        x = ur(&um(&x, &x), &u);
        e >>= 1;
    }
    ug(u.to_vec(), ua(&a, &[0, 1], true)).len() - 1
}
pub fn verify_canonical_line() {
    let f = curve();
    let r = div_root(&div_root(&tangent_restrict(&f, 107, 31), 18), 18);
    assert_eq!(r.len(), 3);
    assert_ne!(r[2], 0);
    assert_eq!(peval(&r, 101), 0);
    assert_eq!(peval(&r, 34), 0);
    assert_ne!(peval(&r, 18), 0);
    assert_eq!((107 * 101 + 31) % 127, 43);
    assert_eq!((107 * 34 + 31) % 127, 113);
}
