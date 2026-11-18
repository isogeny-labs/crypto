#[derive(Clone, Copy)]
struct F {
    n: u32,
    modulus: u32,
}

impl F {
    fn size(self) -> u32 {
        1 << self.n
    }
    fn mul(self, mut a: u32, mut b: u32) -> u32 {
        let mut out = 0;
        while b > 0 {
            if b & 1 == 1 {
                out ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a & self.size() != 0 {
                a ^= self.modulus;
            }
        }
        out
    }
    fn pow(self, mut a: u32, mut e: u32) -> u32 {
        let mut out = 1;
        while e > 0 {
            if e & 1 == 1 {
                out = self.mul(out, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        out
    }
    fn inv(self, a: u32) -> u32 {
        assert_ne!(a, 0);
        self.pow(a, self.size() - 2)
    }
    fn trace(self, mut a: u32) -> u32 {
        let mut out = 0;
        for _ in 0..self.n {
            out ^= a;
            a = self.mul(a, a);
        }
        assert!(out <= 1);
        out
    }
    fn assert_field(self) {
        assert_eq!(self.modulus >> self.n, 1);
        for a in 1..self.size() {
            assert_eq!(self.pow(a, self.size() - 1), 1);
        }
    }
}

fn l(f: F, s: u32) -> u32 {
    f.pow(s, 8) ^ f.pow(s, 4) ^ s
}
fn t(f: F, s: u32) -> u32 {
    let v = l(f, s);
    f.mul(v, v) ^ v
}
fn ag(f: F, g: u32, s: u32) -> u32 {
    let g2 = f.pow(g, 2);
    let g4 = f.pow(g, 4);
    f.mul(g, s)
        ^ f.mul(g ^ g2, f.pow(s, 2))
        ^ f.mul(g ^ g2 ^ g4, f.pow(s, 4))
        ^ f.mul(g ^ g4, f.pow(s, 8))
}

fn curve_order(f: F, b: u32) -> u32 {
    let mut count = 2; // O and the unique point with x=0.
    for x in 1..f.size() {
        let rhs = x ^ 1 ^ f.mul(b, f.inv(f.mul(x, x)));
        if f.trace(rhs) == 0 {
            count += 2;
        }
    }
    count
}

fn target_order(f: F) -> u32 {
    let mut count = if f.n % 2 == 0 { 2 } else { 0 }; // s=infinity.
    for s in 0..f.size() {
        let h = l(f, s) ^ 1;
        if h == 0 {
            count += 1;
        }
        // One ramified point above each pole.
        else if f.trace(1 ^ f.inv(h)) == 0 {
            count += 2;
        }
    }
    count
}

fn check_source_and_closure(f: F) {
    let g = 2;
    let b = f.pow(f.mul(g, g ^ 1), 2);
    assert_eq!(b, 0x14);
    assert_eq!(f.pow(g, 8) ^ f.pow(g, 2) ^ g, 0);
    assert_eq!(curve_order(f, b), 130);
    let mut split_pairs = 0;
    for s in 0..f.size() {
        let ls = l(f, s);
        let ts = t(f, s);
        assert_eq!(t(f, s ^ 1), ts);
        assert_eq!(l(f, s ^ 1), ls ^ 1);
        let a = ag(f, g, s);
        assert_eq!(f.mul(a, a) ^ a, f.mul(g, ts));
        assert_eq!(ag(f, g, s ^ 1), a);
        for x in 1..f.size() {
            if f.mul(x, x) ^ f.mul(ts, x) ^ 1 != 0 {
                continue;
            }
            split_pairs += 1;
            let sx = f.inv(x);
            assert_eq!(f.mul(sx, sx) ^ f.mul(ts, sx) ^ 1, 0);
            // S respects v²+v=x+1, using v↦v+L(s).
            assert_eq!(x ^ 1 ^ f.mul(ls, ls) ^ ls, sx ^ 1);
            // Source AS coordinate y=A_gamma(s)+v.
            assert_eq!(f.mul(g, ts) ^ x ^ 1, f.mul(g, sx) ^ f.mul(g ^ 1, x) ^ 1);
        }
    }
    assert!(split_pairs > 0);
    println!(
        "source: F2^7 modulus=0x{:x}, gamma=0x2, b=0x{:x}, order=130, split_pairs={split_pairs}",
        f.modulus, b
    );
}

fn main() {
    let fields = [
        F { n: 1, modulus: 0x3 },
        F { n: 2, modulus: 0x7 },
        F { n: 3, modulus: 0xb },
        F {
            n: 4,
            modulus: 0x13,
        },
        F {
            n: 5,
            modulus: 0x25,
        },
        F {
            n: 6,
            modulus: 0x43,
        },
        F {
            n: 7,
            modulus: 0x83,
        },
    ];
    let expected = [3, 5, 9, 17, 33, 65, 136];
    for (f, want) in fields.into_iter().zip(expected) {
        f.assert_field();
        let got = target_order(f);
        assert_eq!(got, want);
        println!(
            "target: F2^{}, modulus=0x{:x}, points={got}",
            f.n, f.modulus
        );
        if f.n == 7 {
            check_source_and_closure(f);
        }
    }
}
