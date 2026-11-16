//! GF(2^n) arithmetic (n <= 31, elements are u64 bit-polynomials), bases,
//! GF(2)[T] helpers and sigma-stable subspaces V = ker g(sigma).
//! std-only; every function is covered by an exact test in `tests`.

/// Fixed irreducible reduction polynomials (bit n set). Checked in tests.
pub fn default_modulus(n: u32) -> u64 {
    match n {
        7 => (1 << 7) | (1 << 1) | 1,
        11 => (1 << 11) | (1 << 2) | 1,
        13 => (1 << 13) | (1 << 4) | (1 << 3) | (1 << 1) | 1,
        17 => (1 << 17) | (1 << 3) | 1,
        19 => (1 << 19) | (1 << 5) | (1 << 2) | (1 << 1) | 1,
        23 => (1 << 23) | (1 << 5) | 1,
        31 => (1u64 << 31) | (1 << 3) | 1,
        _ => panic!("no default modulus for n={n}"),
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub n: u32,
    pub modulus: u64,
    pub mask: u64,
    /// Tr(x) = parity(x & trace_mask)
    trace_mask: u64,
    /// half-trace as a linear map: ht(x) = xor of ht_cols[i] over set bits i
    ht_cols: Vec<u64>,
}

impl Field {
    pub fn new(n: u32) -> Field {
        Field::with_modulus(n, default_modulus(n))
    }

    pub fn with_modulus(n: u32, modulus: u64) -> Field {
        assert!(n % 2 == 1 && n <= 31, "n must be odd and <= 31");
        let mut f = Field {
            n,
            modulus,
            mask: (1u64 << n) - 1,
            trace_mask: 0,
            ht_cols: vec![],
        };
        let mut tm = 0u64;
        let mut cols = vec![];
        for i in 0..n {
            let x = 1u64 << i;
            if f.trace_slow(x) == 1 {
                tm |= x;
            }
            cols.push(f.half_trace_slow(x));
        }
        f.trace_mask = tm;
        f.ht_cols = cols;
        f
    }

    #[inline]
    pub fn mul(&self, mut a: u64, mut b: u64) -> u64 {
        let mut r = 0u64;
        let top = 1u64 << self.n;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a & top != 0 {
                a ^= self.modulus;
            }
        }
        r
    }

    #[inline]
    pub fn sq(&self, a: u64) -> u64 {
        self.mul(a, a)
    }

    pub fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e != 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.sq(a);
            e >>= 1;
        }
        r
    }

    pub fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        self.pow(a, (1u64 << self.n) - 2)
    }

    /// sigma^k(a) = a^(2^k)
    pub fn frob(&self, mut a: u64, k: u32) -> u64 {
        for _ in 0..k {
            a = self.sq(a);
        }
        a
    }

    pub fn sqrt(&self, a: u64) -> u64 {
        self.frob(a, self.n - 1)
    }

    fn trace_slow(&self, x: u64) -> u64 {
        let mut acc = x;
        let mut t = x;
        for _ in 1..self.n {
            t = self.sq(t);
            acc ^= t;
        }
        acc
    }

    fn half_trace_slow(&self, x: u64) -> u64 {
        // H(x) = sum_{i=0}^{(n-1)/2} x^(4^i); n odd so H(x)^2 + H(x) = x + Tr(x)
        let mut acc = 0u64;
        let mut t = x;
        for _ in 0..=((self.n - 1) / 2) {
            acc ^= t;
            t = self.sq(self.sq(t));
        }
        acc
    }

    #[inline]
    pub fn trace(&self, x: u64) -> u64 {
        ((x & self.trace_mask).count_ones() & 1) as u64
    }

    #[inline]
    pub fn half_trace(&self, x: u64) -> u64 {
        let mut r = 0u64;
        let mut y = x;
        while y != 0 {
            let i = y.trailing_zeros();
            r ^= self.ht_cols[i as usize];
            y &= y - 1;
        }
        r
    }

    /// Roots in F of A X^2 + B X + C = 0. `None` means every X is a root
    /// (A=B=C=0). Roots are returned sorted, deduplicated.
    pub fn solve_quadratic(&self, a: u64, b: u64, c: u64) -> Option<Vec<u64>> {
        if a == 0 {
            if b == 0 {
                return if c == 0 { None } else { Some(vec![]) };
            }
            return Some(vec![self.mul(c, self.inv(b))]);
        }
        if b == 0 {
            return Some(vec![self.sqrt(self.mul(c, self.inv(a)))]);
        }
        let ib = self.inv(b);
        let gamma = self.mul(self.mul(a, c), self.sq(ib));
        if self.trace(gamma) != 0 {
            return Some(vec![]);
        }
        let z0 = self.half_trace(gamma);
        let scale = self.mul(b, self.inv(a));
        let mut r = vec![self.mul(scale, z0), self.mul(scale, z0 ^ 1)];
        r.sort_unstable();
        Some(r)
    }

    /// Semaev S3 over the binary curve y^2+xy=x^3+a x^2+b (independent of a).
    pub fn s3(&self, x1: u64, x2: u64, x3: u64, b: u64) -> u64 {
        let s = self.sq(x1 ^ x2);
        let p = self.mul(x1, x2);
        self.mul(s, self.sq(x3)) ^ self.mul(p, x3) ^ self.sq(p) ^ b
    }
}

/// An F_2-basis of the field with coordinate conversion.
#[derive(Clone, Debug)]
pub struct Basis {
    pub n: u32,
    pub vecs: Vec<u64>,
    inv_rows: Vec<u64>, // coords_i(x) = parity(inv_rows[i] & x)
    pub normal: bool,
}

impl Basis {
    pub fn from_vecs(n: u32, vecs: Vec<u64>, normal: bool) -> Option<Basis> {
        assert_eq!(vecs.len() as u32, n);
        // invert the matrix whose columns are vecs (bit r of vecs[c] = M[r][c])
        let nn = n as usize;
        let mut rows: Vec<(u64, u64)> = (0..nn)
            .map(|r| {
                let mut row = 0u64;
                for (c, v) in vecs.iter().enumerate() {
                    if (v >> r) & 1 == 1 {
                        row |= 1 << c;
                    }
                }
                (row, 1u64 << r)
            })
            .collect();
        for col in 0..nn {
            let piv = (col..nn).find(|&r| (rows[r].0 >> col) & 1 == 1)?;
            rows.swap(col, piv);
            for r in 0..nn {
                if r != col && (rows[r].0 >> col) & 1 == 1 {
                    let (a, b) = rows[col];
                    rows[r].0 ^= a;
                    rows[r].1 ^= b;
                }
            }
        }
        // M c = x  =>  c = M^-1 x; rows[i].1 is row i of M^-1 (over x bits)
        let inv_rows = rows.iter().map(|r| r.1).collect();
        Some(Basis {
            n,
            vecs,
            inv_rows,
            normal,
        })
    }

    pub fn polynomial(f: &Field) -> Basis {
        Basis::from_vecs(f.n, (0..f.n).map(|i| 1u64 << i).collect(), false).unwrap()
    }

    /// Smallest beta >= 2 such that {beta^(2^i)} is a basis (deterministic).
    pub fn normal(f: &Field) -> Basis {
        let mut beta = 2u64;
        loop {
            let mut v = vec![];
            let mut t = beta;
            for _ in 0..f.n {
                v.push(t);
                t = f.sq(t);
            }
            if let Some(b) = Basis::from_vecs(f.n, v, true) {
                return b;
            }
            beta += 1;
        }
    }

    #[inline]
    pub fn coords(&self, x: u64) -> u64 {
        let mut c = 0u64;
        for (i, r) in self.inv_rows.iter().enumerate() {
            c |= (((r & x).count_ones() & 1) as u64) << i;
        }
        c
    }

    #[inline]
    pub fn from_coords(&self, c: u64) -> u64 {
        let mut x = 0u64;
        let mut y = c;
        while y != 0 {
            x ^= self.vecs[y.trailing_zeros() as usize];
            y &= y - 1;
        }
        x
    }
}

/// n-bit cyclic rotation: bit i -> bit (i+k) mod n.
#[inline]
pub fn rot(c: u64, k: u32, n: u32) -> u64 {
    let k = k % n;
    if k == 0 {
        return c;
    }
    ((c << k) | (c >> (n - k))) & ((1u64 << n) - 1)
}

// ---------------- GF(2)[T] helpers (degree < 64) ----------------

pub fn pdeg(a: u64) -> i32 {
    63 - a.leading_zeros() as i32
}

pub fn pmod(mut a: u64, m: u64) -> u64 {
    let dm = pdeg(m);
    while a != 0 && pdeg(a) >= dm {
        a ^= m << (pdeg(a) - dm);
    }
    a
}

pub fn pmulmod(a: u64, b: u64, m: u64) -> u64 {
    let mut r = 0u64;
    let mut a = pmod(a, m);
    let mut b = b;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        b >>= 1;
        a = pmod(a << 1, m);
    }
    r
}

/// Plain (non-modular) product, degrees must sum to < 64.
pub fn pmul(a: u64, b: u64) -> u64 {
    let mut r = 0u64;
    let mut b = b;
    let mut s = 0;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a << s;
        }
        b >>= 1;
        s += 1;
    }
    r
}

pub fn ppowmod(base: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    let mut b = pmod(base, m);
    while e != 0 {
        if e & 1 == 1 {
            r = pmulmod(r, b, m);
        }
        b = pmulmod(b, b, m);
        e >>= 1;
    }
    r
}

/// Irreducible factors of T^n - 1 over F_2 for odd prime n, split as
/// (T+1, [factors of Phi_n sorted numerically]); each factor of Phi_n has
/// degree ord_n(2).
pub fn cyclotomic_factors(n: u32) -> (u64, Vec<u64>, u32) {
    let ord = multiplicative_order_2(n);
    let mut out = vec![];
    // monic, constant term 1, degree ord
    for low in 0..(1u64 << (ord - 1)) {
        let f = (1u64 << ord) | (low << 1) | 1;
        if ppowmod(2, n as u64, f) == 1 {
            out.push(f);
        }
    }
    assert_eq!(
        out.len() as u32 * ord,
        n - 1,
        "factor degrees must sum to n-1"
    );
    (0b11, out, ord)
}

pub fn multiplicative_order_2(n: u32) -> u32 {
    let mut x = 2 % n;
    let mut k = 1;
    while x != 1 {
        x = (x * 2) % n;
        k += 1;
    }
    k
}

/// sigma-stable subspace spec: which Phi_n factors (by index) and whether (T+1) is included.
#[derive(Clone, Debug)]
pub struct VSpec {
    pub factor_idx: Vec<usize>,
    pub with_t_plus_1: bool,
}

impl VSpec {
    pub fn dim(&self, ord: u32) -> u32 {
        self.factor_idx.len() as u32 * ord + self.with_t_plus_1 as u32
    }
    pub fn g(&self, t1: u64, fs: &[u64]) -> u64 {
        let mut g = 1u64;
        if self.with_t_plus_1 {
            g = pmul(g, t1);
        }
        for &i in &self.factor_idx {
            g = pmul(g, fs[i]);
        }
        g
    }
}

/// The `which`-th (lexicographic) V of dimension l at degree n, or None.
pub fn vspec_for_dim(n: u32, l: u32, which: usize) -> Option<VSpec> {
    let (_, fs, ord) = cyclotomic_factors(n);
    let k_total = fs.len();
    for with in [false, true] {
        if l < with as u32 || (l - with as u32) % ord != 0 {
            continue;
        }
        let k = ((l - with as u32) / ord) as usize;
        if k > k_total {
            continue;
        }
        // enumerate k-subsets of 0..k_total lexicographically
        let mut combos = vec![];
        let mut cur = vec![];
        fn rec(
            start: usize,
            k: usize,
            total: usize,
            cur: &mut Vec<usize>,
            out: &mut Vec<Vec<usize>>,
        ) {
            if cur.len() == k {
                out.push(cur.clone());
                return;
            }
            for i in start..total {
                cur.push(i);
                rec(i + 1, k, total, cur, out);
                cur.pop();
            }
        }
        rec(0, k, k_total, &mut cur, &mut combos);
        if let Some(c) = combos.get(which) {
            return Some(VSpec {
                factor_idx: c.clone(),
                with_t_plus_1: with,
            });
        }
        return None;
    }
    None
}

/// V = ker g(sigma) inside F_{2^n}, returned as a reduced-echelon basis in the
/// coordinates of `coord_basis` (field elements in polynomial representation).
/// g(sigma) acts on NORMAL coordinates as a circulant, so the kernel is computed
/// there and converted.
pub fn stable_subspace(f: &Field, normal: &Basis, g: u64, coord_basis: &Basis) -> Vec<u64> {
    let n = f.n;
    // (g(sigma) c)_j = sum_k g_k c_{j-k}: rotate and xor
    // kernel of the n x n matrix M with column i = g(sigma) e_i
    let mut cols = vec![0u64; n as usize];
    for i in 0..n {
        let e = 1u64 << i;
        let mut img = 0u64;
        for k in 0..=(pdeg(g).max(0) as u32) {
            if (g >> k) & 1 == 1 {
                img ^= rot(e, k, n);
            }
        }
        cols[i as usize] = img;
    }
    // nullspace of M (acting on column vectors c): gaussian elimination on rows
    let nn = n as usize;
    let mut rows: Vec<u64> = (0..nn)
        .map(|r| {
            let mut row = 0u64;
            for (c, col) in cols.iter().enumerate() {
                if (col >> r) & 1 == 1 {
                    row |= 1 << c;
                }
            }
            row
        })
        .collect();
    let mut pivots = vec![];
    let mut rank = 0;
    for col in 0..nn {
        if let Some(p) = (rank..nn).find(|&r| (rows[r] >> col) & 1 == 1) {
            rows.swap(rank, p);
            for r in 0..nn {
                if r != rank && (rows[r] >> col) & 1 == 1 {
                    let t = rows[rank];
                    rows[r] ^= t;
                }
            }
            pivots.push(col);
            rank += 1;
        }
    }
    let free: Vec<usize> = (0..nn).filter(|c| !pivots.contains(c)).collect();
    let mut basis_n = vec![];
    for &fc in &free {
        let mut v = 1u64 << fc;
        for (ri, &pc) in pivots.iter().enumerate() {
            if (rows[ri] >> fc) & 1 == 1 {
                v |= 1 << pc;
            }
        }
        basis_n.push(normal.from_coords(v));
    }
    // reduced echelon form in coord_basis coordinates
    let mut vs: Vec<u64> = basis_n.iter().map(|&x| coord_basis.coords(x)).collect();
    let mut r = 0;
    for bit in (0..nn).rev() {
        if let Some(p) = (r..vs.len()).find(|&i| (vs[i] >> bit) & 1 == 1) {
            vs.swap(r, p);
            for i in 0..vs.len() {
                if i != r && (vs[i] >> bit) & 1 == 1 {
                    let t = vs[r];
                    vs[i] ^= t;
                }
            }
            r += 1;
        }
    }
    vs.iter().map(|&c| coord_basis.from_coords(c)).collect()
}

/// Membership predicate for V = ker g(sigma) using the NORMAL basis.
pub struct VMember {
    n: u32,
    g: u64,
}

impl VMember {
    pub fn new(n: u32, g: u64) -> VMember {
        VMember { n, g }
    }
    #[inline]
    pub fn contains_coords(&self, c: u64) -> bool {
        let mut img = 0u64;
        let mut gg = self.g;
        while gg != 0 {
            let k = gg.trailing_zeros();
            img ^= rot(c, k, self.n);
            gg &= gg - 1;
        }
        img == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moduli_are_irreducible() {
        for n in [7u32, 11, 13, 17, 19, 23, 31] {
            let f = default_modulus(n);
            // prime degree: irreducible iff x^(2^n) = x mod f and f has no root in F_2
            assert_eq!(f & 1, 1);
            assert_eq!(f.count_ones() % 2, 1, "f(1) must be 1, n={n}");
            let mut x = 2u64;
            for _ in 0..n {
                x = pmulmod(x, x, f);
            }
            assert_eq!(x, 2, "x^(2^n) != x mod f, n={n}");
        }
    }

    #[test]
    fn field_axioms_and_inverse() {
        let f = Field::new(17);
        for &(a, b) in &[(3u64, 77u64), (131071, 5), (12345, 54321)] {
            assert_eq!(f.mul(a, b), f.mul(b, a));
            assert_eq!(f.mul(a, f.inv(a)), 1);
            assert_eq!(f.mul(a ^ b, 9), f.mul(a, 9) ^ f.mul(b, 9));
        }
        // x^(2^n) = x
        assert_eq!(f.frob(12345, 17), 12345);
        assert_eq!(f.sqrt(f.sq(999)), 999);
    }

    #[test]
    fn trace_halftrace_quadratic_exact_n7() {
        let f = Field::new(7);
        for x in 0..128u64 {
            assert_eq!(f.trace(x), f.trace_slow(x) & 1);
            assert!(f.trace_slow(x) <= 1);
            let h = f.half_trace(x);
            assert_eq!(f.sq(h) ^ h, x ^ f.trace(x));
        }
        // every quadratic, compared with brute force
        for a in 0..16u64 {
            for b in 0..128u64 {
                for c in (0..128u64).step_by(5) {
                    let brute: Vec<u64> = (0..128u64)
                        .filter(|&x| f.mul(a, f.sq(x)) ^ f.mul(b, x) ^ c == 0)
                        .collect();
                    match f.solve_quadratic(a, b, c) {
                        None => assert_eq!(brute.len(), 128),
                        Some(r) => assert_eq!(r, brute, "a={a} b={b} c={c}"),
                    }
                }
            }
        }
    }

    #[test]
    fn bases_roundtrip_and_rotation() {
        for n in [7u32, 17] {
            let f = Field::new(n);
            let nb = Basis::normal(&f);
            let pb = Basis::polynomial(&f);
            for x in [1u64, 2, 5, 77, (1 << n) - 1, 0x1234 & ((1 << n) - 1)] {
                assert_eq!(nb.from_coords(nb.coords(x)), x);
                assert_eq!(pb.from_coords(pb.coords(x)), x);
                // sigma is coordinate rotation in the normal basis
                assert_eq!(nb.coords(f.sq(x)), rot(nb.coords(x), 1, n));
            }
        }
    }

    #[test]
    fn ord_and_factors() {
        assert_eq!(multiplicative_order_2(17), 8);
        assert_eq!(multiplicative_order_2(23), 11);
        assert_eq!(multiplicative_order_2(31), 5);
        assert_eq!(multiplicative_order_2(7), 3);
        assert_eq!(multiplicative_order_2(131), 130);
        assert_eq!(multiplicative_order_2(163), 162);
        let (_, fs, ord) = cyclotomic_factors(31);
        assert_eq!((fs.len() as u32, ord), (6, 5));
        let (_, fs, ord) = cyclotomic_factors(17);
        assert_eq!((fs.len() as u32, ord), (2, 8));
        // product of all factors times (T+1) is T^n + 1
        let mut p = 0b11u64;
        for &q in &fs {
            p = pmul(p, q);
        }
        assert_eq!(p, (1u64 << 17) | 1);
    }

    #[test]
    fn stable_subspace_dimension_and_stability() {
        for (n, l) in [
            (7u32, 3u32),
            (7, 4),
            (17, 8),
            (17, 9),
            (23, 11),
            (23, 12),
            (31, 10),
            (31, 11),
            (31, 15),
            (31, 16),
        ] {
            let f = Field::new(n);
            let nb = Basis::normal(&f);
            let spec = vspec_for_dim(n, l, 0).expect("spec");
            let (t1, fs, ord) = cyclotomic_factors(n);
            assert_eq!(spec.dim(ord), l);
            let g = spec.g(t1, &fs);
            assert_eq!(pdeg(g) as u32, l);
            let vb = stable_subspace(&f, &nb, g, &nb);
            assert_eq!(vb.len() as u32, l, "n={n} l={l}");
            // closed under squaring, and membership predicate agrees
            let mem = VMember::new(n, g);
            for &v in &vb {
                assert!(mem.contains_coords(nb.coords(v)));
                assert!(mem.contains_coords(nb.coords(f.sq(v))));
            }
            // reduced echelon: pivot (top) bits are distinct
            let mut tops: Vec<i32> = vb.iter().map(|&v| pdeg(nb.coords(v))).collect();
            tops.sort();
            tops.dedup();
            assert_eq!(tops.len() as u32, l);
            // poly-basis RREF spans the same space
            let pb = Basis::polynomial(&f);
            let vp = stable_subspace(&f, &nb, g, &pb);
            for &v in &vp {
                assert!(mem.contains_coords(nb.coords(v)));
            }
            // dimension: 1 in V iff T+1 included
            assert_eq!(mem.contains_coords(nb.coords(1)), spec.with_t_plus_1);
        }
        // no admissible dimension 100 at n=31
        assert!(vspec_for_dim(31, 12, 0).is_none());
        // K-163 / ECC2K-130 dimension lattice: only {0,1,162,163} / {0,1,130,131}
        let reach = |n: u32| -> Vec<u32> {
            let ord = multiplicative_order_2(n);
            let k = (n - 1) / ord;
            let mut d = vec![];
            for i in 0..=1u32 {
                for j in 0..=k {
                    d.push(i + j * ord);
                }
            }
            d.sort();
            d.dedup();
            d
        };
        assert_eq!(reach(163), vec![0, 1, 162, 163]);
        assert_eq!(reach(131), vec![0, 1, 130, 131]);
    }

    #[test]
    fn s3_matches_group_law_is_checked_in_curve_tests() {
        // formula identity: S3 = (x1x2 + x3(x1+x2))^2 + x1x2x3 + b
        let f = Field::new(13);
        let (x1, x2, x3, b) = (17u64, 301, 4001, 1);
        let p = f.mul(x1, x2);
        let alt = f.sq(p ^ f.mul(x3, x1 ^ x2)) ^ f.mul(p, x3) ^ b;
        assert_eq!(f.s3(x1, x2, x3, b), alt);
    }
}
