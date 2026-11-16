use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
#[derive(Default)]
pub struct FxHasher(u64);
impl Hasher for FxHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ b as u64).wrapping_mul(0x517C_C1B7_2722_0A95);
        }
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x517C_C1B7_2722_0A95);
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64)
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64)
    }
}
pub type FxMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
pub type FxSet<K> = HashSet<K, BuildHasherDefault<FxHasher>>;
pub struct Mont {
    pub n: u64,
    ninv: u64, // -n^{-1} mod 2^64
    r2: u64,   // 2^128 mod n
    pub one: u64,
}

impl Mont {
    pub fn new(n: u64) -> Mont {
        assert!(n % 2 == 1 && n < (1u64 << 63));
        let mut inv = n;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(n.wrapping_mul(inv)));
        }
        let ninv = inv.wrapping_neg();
        let r = ((1u128 << 64) % n as u128) as u64;
        let r2 = ((r as u128 * r as u128) % n as u128) as u64;
        Mont {
            n,
            ninv,
            r2,
            one: r,
        }
    }
    #[inline]
    fn redc(&self, t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(self.ninv);
        let u = ((t + m as u128 * self.n as u128) >> 64) as u64;
        if u >= self.n {
            u - self.n
        } else {
            u
        }
    }
    #[inline]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        self.redc(a as u128 * b as u128)
    }
    #[inline]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.n {
            s - self.n
        } else {
            s
        }
    }
    #[inline]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.n - b
        }
    }
    #[inline]
    pub fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.n - a
        }
    }
    /// Plain residue -> Montgomery form.
    #[inline]
    pub fn to(&self, a: u64) -> u64 {
        self.mul(a % self.n, self.r2)
    }
    /// Montgomery form -> plain residue.
    #[inline]
    pub fn from(&self, a: u64) -> u64 {
        self.redc(a as u128)
    }
    pub fn pow(&self, a: u64, mut e: u64) -> u64 {
        let (mut r, mut b) = (self.one, a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    /// Inverse for prime `n` (Fermat), Montgomery form in and out.
    pub fn inv(&self, a: u64) -> u64 {
        self.pow(a, self.n - 2)
    }
    /// Signed small integer to Montgomery form.
    pub fn from_i(&self, c: i64) -> u64 {
        if c >= 0 {
            self.to(c as u64)
        } else {
            self.neg(self.to((-c) as u64))
        }
    }
}

pub type SRow = Vec<(u32, u64)>;
fn axpy_sub(mont: &Mont, a: &SRow, f: u64, b: &SRow, ops: &mut u64) -> SRow {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if j >= b.len() || (i < a.len() && a[i].0 < b[j].0) {
            out.push(a[i]);
            i += 1;
        } else if i >= a.len() || b[j].0 < a[i].0 {
            *ops += 1;
            out.push((b[j].0, mont.neg(mont.mul(f, b[j].1))));
            j += 1;
        } else {
            *ops += 1;
            let v = mont.sub(a[i].1, mont.mul(f, b[j].1));
            if v != 0 {
                out.push((a[i].0, v));
            }
            i += 1;
            j += 1;
        }
    }
    out
}

fn row_coef(row: &SRow, c: u32) -> u64 {
    match row.binary_search_by_key(&c, |e| e.0) {
        Ok(p) => row[p].1,
        Err(_) => 0,
    }
}

/// Eliminate every large-prime column (columns `>= nfb`) by minimum-weight pivoting
/// (weight 1: row dropped; weight 2: the two rows merged; general weight: Markowitz-style
/// pivot on the shortest row). Returns FB-only rows. All mod-ell work is counted in
/// `ops` / `entries`.
pub fn eliminate_lp_trace(
    mont: &Mont,
    nfb: usize,
    rows_in: Vec<SRow>,
    ops: &mut u64,
    entries: &mut u64,
) -> (Vec<SRow>, Vec<(u32, SRow)>) {
    use std::cmp::Reverse;
    let mut trace = Vec::new();
    let mut rows: Vec<Option<SRow>> = rows_in.into_iter().map(Some).collect();
    let mut colset: FxMap<u32, FxSet<u32>> = FxMap::default();
    for (r, row) in rows.iter().enumerate() {
        for &(c, _) in row.as_ref().unwrap() {
            if c as usize >= nfb {
                colset.entry(c).or_default().insert(r as u32);
                *entries += 1;
            }
        }
    }
    let mut heap: BinaryHeap<Reverse<(usize, u32)>> =
        colset.iter().map(|(&c, s)| Reverse((s.len(), c))).collect();
    while let Some(Reverse((w, c))) = heap.pop() {
        let cur = colset.get(&c).map(|s| s.len()).unwrap_or(0);
        if cur == 0 {
            colset.remove(&c);
            continue;
        }
        if cur != w {
            heap.push(Reverse((cur, c)));
            continue;
        }
        let pr = *colset[&c]
            .iter()
            .min_by_key(|&&r| (rows[r as usize].as_ref().unwrap().len(), r))
            .unwrap();
        let mut prow = rows[pr as usize].take().unwrap();
        for &(cc, _) in &prow {
            if cc as usize >= nfb {
                colset.get_mut(&cc).unwrap().remove(&pr);
            }
        }
        let pc = row_coef(&prow, c);
        let inv = mont.inv(pc);
        *ops += 128; // Fermat inversion: ~ 2 log2(ell) multiplications
        for e in prow.iter_mut() {
            e.1 = mont.mul(e.1, inv);
        }
        *ops += prow.len() as u64;
        trace.push((c, prow.clone()));
        let mut others: Vec<u32> = colset
            .get(&c)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default();
        others.sort_unstable();
        for r in others {
            let row = rows[r as usize].take().unwrap();
            let fct = row_coef(&row, c);
            let newrow = axpy_sub(mont, &row, fct, &prow, ops);
            *entries += (row.len() + prow.len()) as u64;
            // update column sets
            for &(cc, _) in &row {
                if cc as usize >= nfb && newrow.binary_search_by_key(&cc, |e| e.0).is_err() {
                    let s = colset.get_mut(&cc).unwrap();
                    s.remove(&r);
                    if cc != c {
                        heap.push(Reverse((s.len(), cc)));
                    }
                }
            }
            for &(cc, _) in &newrow {
                if cc as usize >= nfb && row.binary_search_by_key(&cc, |e| e.0).is_err() {
                    let s = colset.entry(cc).or_default();
                    s.insert(r);
                    heap.push(Reverse((s.len(), cc)));
                }
            }
            rows[r as usize] = Some(newrow);
        }
        colset.remove(&c);
    }
    let remaining = rows
        .into_iter()
        .flatten()
        .filter(|r| !r.is_empty())
        .collect();
    (remaining, trace)
}
