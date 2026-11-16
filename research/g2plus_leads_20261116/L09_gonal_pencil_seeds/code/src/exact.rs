//! Exact rank engine over F_ell, ell = 2^61 - 1 (SPEC section 9).

pub const ELL: u64 = (1u64 << 61) - 1;

#[inline]
pub fn mulm(a: u64, b: u64) -> u64 {
    let t = (a as u128) * (b as u128);
    let lo = (t & (ELL as u128)) as u64;
    let hi = (t >> 61) as u64;
    let mut r = lo + hi;
    if r >= ELL {
        r -= ELL;
    }
    r
}
#[inline]
fn subm(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + ELL - b }
}
pub fn powm(mut b: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, b);
        }
        b = mulm(b, b);
        e >>= 1;
    }
    r
}
pub fn invm(a: u64) -> u64 {
    powm(a, ELL - 2)
}

pub struct Echelon {
    pub width: usize,
    piv: Vec<Option<(Vec<u32>, Vec<u64>)>>, // sparse pivot rows, pivot entry 1
    pub rank: usize,
    pub la_ops: u64,
}

impl Echelon {
    pub fn new(width: usize) -> Echelon {
        Echelon { width, piv: (0..width).map(|_| None).collect(), rank: 0, la_ops: 0 }
    }
    /// Insert a dense row; returns true if it increased the rank.
    pub fn insert(&mut self, mut row: Vec<u64>) -> bool {
        assert!(row.len() == self.width);
        let mut c = 0usize;
        loop {
            while c < self.width && row[c] == 0 {
                c += 1;
            }
            if c >= self.width {
                return false;
            }
            if let Some((cols, vals)) = &self.piv[c] {
                let f = row[c];
                for k in 0..cols.len() {
                    let j = cols[k] as usize;
                    let m = mulm(f, vals[k]);
                    row[j] = subm(row[j], m);
                }
                self.la_ops += cols.len() as u64;
                // row[c] is now 0
            } else {
                let iv = invm(row[c]);
                self.la_ops += 1;
                let mut cols = vec![];
                let mut vals = vec![];
                for j in c..self.width {
                    if row[j] != 0 {
                        cols.push(j as u32);
                        vals.push(mulm(row[j], iv));
                        self.la_ops += 1;
                    }
                }
                self.piv[c] = Some((cols, vals));
                self.rank += 1;
                return true;
            }
        }
    }
}

pub struct Exact {
    pub n_cols: usize,
    pub n1: usize,
    pub full: Echelon,
    pub lp: Echelon,
}

impl Exact {
    pub fn new(n_cols: usize, n1: usize) -> Exact {
        Exact { n_cols, n1, full: Echelon::new(n_cols + n1), lp: Echelon::new(n1) }
    }
    /// small: (column, coefficient +-1); lps: point ids (LP column = n_cols + id).
    pub fn add_row(&mut self, small: &[(usize, i64)], lps: &[u32]) {
        let mut f = vec![0u64; self.n_cols + self.n1];
        let mut l = vec![0u64; self.n1];
        for &(c, k) in small {
            f[c] = if k >= 0 { (k as u64) % ELL } else { ELL - ((-k) as u64) };
        }
        for &id in lps {
            f[self.n_cols + id as usize] = 1;
            l[id as usize] = 1;
        }
        self.full.insert(f);
        self.lp.insert(l);
    }
    pub fn dim(&self) -> i64 {
        self.full.rank as i64 - self.lp.rank as i64
    }
    pub fn la_ops(&self) -> u64 {
        self.full.la_ops + self.lp.la_ops
    }
}
