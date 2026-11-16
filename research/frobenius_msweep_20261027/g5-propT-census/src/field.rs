//! GF(2^n) with log/antilog tables (n odd prime, n <= 19), std only.

pub fn poly_for(n: u32) -> u32 {
    match n {
        5 => 0x25,     // x^5 + x^2 + 1
        7 => 0x83,     // x^7 + x + 1
        11 => 0x805,   // x^11 + x^2 + 1
        13 => 0x201B,  // x^13 + x^4 + x^3 + x + 1
        17 => 0x20009, // x^17 + x^3 + 1
        19 => 0x80027, // x^19 + x^5 + x^2 + x + 1
        _ => panic!("no polynomial registered for n={n}"),
    }
}

pub fn clmul_reduce(a: u32, b: u32, n: u32, poly: u32) -> u32 {
    let mut r: u64 = 0;
    let (a, mut b, mut i) = (a as u64, b, 0);
    while b != 0 {
        if b & 1 == 1 {
            r ^= a << i;
        }
        b >>= 1;
        i += 1;
    }
    for k in (n..2 * n).rev() {
        if (r >> k) & 1 == 1 {
            r ^= (poly as u64) << (k - n);
        }
    }
    r as u32
}

pub struct Fq {
    pub n: u32,
    pub q: u32,
    pub poly: u32,
    q1: usize,
    exp: Vec<u32>,
    log: Vec<u32>,
    tr_mask: u32,
    ht_tab: Vec<u32>,
}

impl Fq {
    pub fn new(n: u32) -> Fq {
        let poly = poly_for(n);
        assert!(n % 2 == 1 && (poly >> n) == 1);
        // n prime: x^(2^n) = x mod poly and no root in F_2 imply irreducible.
        assert!(poly & 1 == 1 && poly.count_ones() % 2 == 1, "root in F_2");
        let mut z = 2u32;
        for _ in 0..n {
            z = clmul_reduce(z, z, n, poly);
        }
        assert_eq!(z, 2, "poly not irreducible");
        let q1 = (1usize << n) - 1;
        let mut exp = vec![0u32; 2 * q1];
        let mut log = vec![0u32; q1 + 1];
        let mut g = 2u32;
        'search: loop {
            let mut e = 1u32;
            for i in 0..q1 {
                if i > 0 && e == 1 {
                    g += 1;
                    continue 'search;
                }
                exp[i] = e;
                log[e as usize] = i as u32;
                e = clmul_reduce(e, g, n, poly);
            }
            assert_eq!(e, 1);
            break;
        }
        for i in 0..q1 {
            exp[q1 + i] = exp[i];
        }
        let mut f = Fq {
            n,
            q: 1 << n,
            poly,
            q1,
            exp,
            log,
            tr_mask: 0,
            ht_tab: vec![],
        };
        for i in 0..n {
            let e = 1u32 << i;
            let (mut t, mut z) = (0u32, e);
            for _ in 0..n {
                t ^= z;
                z = f.sq(z);
            }
            assert!(t <= 1);
            f.tr_mask |= t << i;
            let (mut h, mut z) = (0u32, e);
            for _ in 0..=(n - 1) / 2 {
                h ^= z;
                z = f.sq(f.sq(z));
            }
            f.ht_tab.push(h);
        }
        f
    }
    #[inline]
    pub fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            return 0;
        }
        self.exp[(self.log[a as usize] + self.log[b as usize]) as usize]
    }
    #[inline]
    pub fn sq(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    #[inline]
    pub fn inv(&self, a: u32) -> u32 {
        assert!(a != 0);
        self.exp[(self.q1 - self.log[a as usize] as usize) % self.q1]
    }
    #[inline]
    pub fn tr(&self, a: u32) -> u32 {
        (a & self.tr_mask).count_ones() & 1
    }
    /// Half trace H(a): solves z^2 + z = a when Tr(a) = 0 (n odd).
    pub fn ht(&self, a: u32) -> u32 {
        let mut r = 0;
        for i in 0..self.n {
            if (a >> i) & 1 == 1 {
                r ^= self.ht_tab[i as usize];
            }
        }
        r
    }
    /// sigma^j(x) = x^(2^j), j taken mod n.
    pub fn frob(&self, x: u32, j: u32) -> u32 {
        let mut z = x;
        for _ in 0..(j % self.n) {
            z = self.sq(z);
        }
        z
    }
    /// sigma^(-j)(x).
    pub fn frob_inv(&self, x: u32, j: u32) -> u32 {
        self.frob(x, (self.n - j % self.n) % self.n)
    }
    pub fn sqrt(&self, x: u32) -> u32 {
        self.frob(x, self.n - 1)
    }
    /// A uniformly random element (0 allowed).
    pub fn rand(&self, rng: &mut crate::rng::Rng) -> u32 {
        rng.below(self.q as u64) as u32
    }
}
