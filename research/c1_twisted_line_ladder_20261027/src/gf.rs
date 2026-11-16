//! F_Q = F_{2^k} by log tables (k <= 15) and F_{Q^4} = F_Q[g]/(g^4+g+1)
//! (k odd, so g^4+g+1 stays irreducible over F_Q). std only.
//! Table construction follows research/lift_dense_factor_base_20261027/src/main.rs
//! (commit b3cb8b1ea) and the primitive polynomials of
//! src/cryptanalysis/gaudry_quartic_bin.rs (same commit).

pub const PRIMITIVE: [u32; 16] = [
    0, 0, 0x7, 0xB, 0x13, 0x25, 0x43, 0x89, 0x11D, 0x211, 0x409, 0x805, 0x1053, 0x201B, 0x4443,
    0x8003,
];

pub struct Fq {
    pub k: u32,
    pub q: u32,
    pub exp: Vec<u16>,
    pub log: Vec<u16>,
}

impl Fq {
    pub fn new(k: u32) -> Fq {
        assert!((2..=15).contains(&k));
        let q = 1u32 << k;
        let modulus = PRIMITIVE[k as usize];
        let mut exp = vec![0u16; 2 * q as usize];
        let mut log = vec![0u16; q as usize];
        let mut v = 1u32;
        for i in 0..(q - 1) {
            exp[i as usize] = v as u16;
            assert!(i == 0 || v != 1, "x not primitive");
            log[v as usize] = i as u16;
            v <<= 1;
            if v & q != 0 {
                v ^= modulus;
            }
        }
        assert_eq!(v, 1);
        for i in 0..(q - 1) {
            exp[(q - 1 + i) as usize] = exp[i as usize];
        }
        Fq { k, q, exp, log }
    }
    #[inline(always)]
    pub fn mul(&self, a: u16, b: u16) -> u16 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[self.log[a as usize] as usize + self.log[b as usize] as usize]
        }
    }
    #[inline(always)]
    pub fn inv(&self, a: u16) -> u16 {
        assert!(a != 0);
        self.exp[((self.q - 1) - self.log[a as usize] as u32) as usize % (self.q - 1) as usize]
    }
    #[inline(always)]
    pub fn sq(&self, a: u16) -> u16 {
        self.mul(a, a)
    }
    pub fn pow(&self, a: u16, e: u64) -> u16 {
        if a == 0 {
            return if e == 0 { 1 } else { 0 };
        }
        let l = (self.log[a as usize] as u64 * (e % (self.q as u64 - 1))) % (self.q as u64 - 1);
        self.exp[l as usize]
    }
}

/// Element of F_{Q^4}: coordinates on 1, g, g^2, g^3.
pub type E4 = [u16; 4];
pub const ZERO: E4 = [0; 4];
pub const ONE: E4 = [1, 0, 0, 0];

pub struct Fq4 {
    pub f: Fq,
    /// Frobenius x -> x^Q on the basis: images of g^i (coordinates in {0,1}).
    pub frob_img: [E4; 4],
    /// F_2-linear map z -> z^2 + z on packed 4k-bit vectors: echelon rows (image, preimage).
    as_rows: Vec<(u64, u64)>,
}

#[inline(always)]
pub fn add(a: &E4, b: &E4) -> E4 {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}
pub fn is_zero(a: &E4) -> bool {
    a[0] | a[1] | a[2] | a[3] == 0
}

impl Fq4 {
    pub fn new(k: u32) -> Fq4 {
        assert!(k % 2 == 1);
        let f = Fq::new(k);
        let mut me = Fq4 { f, frob_img: [ZERO; 4], as_rows: vec![] };
        // g^Q = g^(2^k mod 15) since g has order 15
        let e = ((1u64 << k) % 15) as u32;
        let gq = me.pow(&[0, 1, 0, 0], e as u64);
        for i in 0..4 {
            me.frob_img[i] = me.pow(&gq, i as u64);
            for c in me.frob_img[i] {
                assert!(c <= 1);
            }
        }
        // AS rows
        let n = 4 * k;
        let mut rows: Vec<(u64, u64)> = vec![];
        for i in 0..n {
            let z = 1u64 << i;
            let ze = me.unpack(z);
            let (mut im, mut pre) = (me.pack(&add(&me.sq(&ze), &ze)), z);
            for &(r, q) in &rows {
                if im ^ r < im {
                    im ^= r;
                    pre ^= q;
                }
            }
            if im != 0 {
                rows.push((im, pre));
                rows.sort_unstable_by(|a, b| b.0.cmp(&a.0));
            }
        }
        assert_eq!(rows.len() as u32, n - 1);
        me.as_rows = rows;
        me
    }
    #[inline(always)]
    pub fn mul(&self, a: &E4, b: &E4) -> E4 {
        let f = &self.f;
        let mut p = [0u16; 7];
        for i in 0..4 {
            if a[i] == 0 {
                continue;
            }
            let la = f.log[a[i] as usize] as usize;
            for j in 0..4 {
                if b[j] != 0 {
                    p[i + j] ^= f.exp[la + f.log[b[j] as usize] as usize];
                }
            }
        }
        [p[0] ^ p[4], p[1] ^ p[4] ^ p[5], p[2] ^ p[5] ^ p[6], p[3] ^ p[6]]
    }
    #[inline(always)]
    pub fn sq(&self, a: &E4) -> E4 {
        let f = &self.f;
        let c: [u16; 4] = [f.sq(a[0]), f.sq(a[1]), f.sq(a[2]), f.sq(a[3])];
        [c[0] ^ c[2], c[2], c[1] ^ c[3], c[3]]
    }
    pub fn scale(&self, s: u16, a: &E4) -> E4 {
        [self.f.mul(s, a[0]), self.f.mul(s, a[1]), self.f.mul(s, a[2]), self.f.mul(s, a[3])]
    }
    pub fn pow(&self, a: &E4, mut e: u64) -> E4 {
        let mut r = ONE;
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.sq(&b);
            e >>= 1;
        }
        r
    }
    /// x -> x^Q (F_Q-linear)
    pub fn frob(&self, a: &E4) -> E4 {
        let mut r = ZERO;
        for i in 0..4 {
            if a[i] != 0 {
                for j in 0..4 {
                    if self.frob_img[i][j] == 1 {
                        r[j] ^= a[i];
                    }
                }
            }
        }
        r
    }
    /// Tr_{Q^4/Q}(a) as an element of F_Q
    pub fn tr_q(&self, a: &E4) -> u16 {
        let f1 = self.frob(a);
        let f2 = self.frob(&f1);
        let f3 = self.frob(&f2);
        let t = add(&add(a, &f1), &add(&f2, &f3));
        assert!(t[1] == 0 && t[2] == 0 && t[3] == 0);
        t[0]
    }
    pub fn norm(&self, a: &E4) -> u16 {
        let f1 = self.frob(a);
        let f2 = self.frob(&f1);
        let f3 = self.frob(&f2);
        let n = self.mul(&self.mul(a, &f1), &self.mul(&f2, &f3));
        assert!(n[1] == 0 && n[2] == 0 && n[3] == 0);
        n[0]
    }
    pub fn inv(&self, a: &E4) -> E4 {
        assert!(!is_zero(a));
        let f1 = self.frob(a);
        let f2 = self.frob(&f1);
        let f3 = self.frob(&f2);
        let c = self.mul(&self.mul(&f1, &f2), &f3);
        let n = self.mul(a, &c);
        assert!(n[1] == 0 && n[2] == 0 && n[3] == 0);
        self.scale(self.f.inv(n[0]), &c)
    }
    pub fn pack(&self, a: &E4) -> u64 {
        let k = self.f.k;
        (a[0] as u64) | (a[1] as u64) << k | (a[2] as u64) << (2 * k) | (a[3] as u64) << (3 * k)
    }
    pub fn unpack(&self, v: u64) -> E4 {
        let k = self.f.k;
        let m = (1u64 << k) - 1;
        [(v & m) as u16, (v >> k & m) as u16, (v >> (2 * k) & m) as u16, (v >> (3 * k) & m) as u16]
    }
    /// z with z^2 + z = w, if Tr_{Q^4/F_2}(w) = 0
    pub fn artin_schreier(&self, w: &E4) -> Option<E4> {
        let (mut x, mut z) = (self.pack(w), 0u64);
        for &(r, q) in &self.as_rows {
            if x ^ r < x {
                x ^= r;
                z ^= q;
            }
        }
        if x == 0 {
            Some(self.unpack(z))
        } else {
            None
        }
    }
    /// F_16 element from its 4-bit coordinates on 1, g, g^2, g^3.
    pub fn f16(&self, v: u8) -> E4 {
        [(v & 1) as u16, (v >> 1 & 1) as u16, (v >> 2 & 1) as u16, (v >> 3 & 1) as u16]
    }
    pub fn sqrt(&self, a: &E4) -> E4 {
        // a^(2^(N-1)), N = 4k
        let mut r = *a;
        for _ in 0..(4 * self.f.k - 1) {
            r = self.sq(&r);
        }
        r
    }
    /// Tr_{Q^4/F_2}
    pub fn tr_abs(&self, a: &E4) -> u32 {
        let t = self.tr_q(a);
        let mut s = 0u32;
        let mut z = t;
        for _ in 0..self.f.k {
            s ^= z as u32;
            z = self.f.sq(z);
        }
        assert!(s <= 1);
        s
    }
    /// Batch inversion (Montgomery trick); zeros not allowed.
    pub fn batch_inv(&self, v: &mut [E4], scratch: &mut Vec<E4>) {
        scratch.clear();
        let mut acc = ONE;
        for x in v.iter() {
            scratch.push(acc);
            acc = self.mul(&acc, x);
        }
        let mut inv = self.inv(&acc);
        for i in (0..v.len()).rev() {
            let xi = v[i];
            v[i] = self.mul(&inv, &scratch[i]);
            inv = self.mul(&inv, &xi);
        }
    }
}
