//! RNG, hashing, a flat JSON-lines reader, number theory helpers.
use std::collections::HashMap;

pub fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

/// Domain-separated seed derivation: `derive(seed, &[tag, idx, ...])`.
pub fn derive(seed: u64, tags: &[u64]) -> u64 {
    let mut h = mix64(seed ^ 0x51ed_2701_f3a5_c0de);
    for &t in tags {
        h = mix64(h ^ mix64(t.wrapping_add(0x2545_f491_4f6c_dd1d)));
    }
    h
}

pub struct Rng {
    s: u64,
}
impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng { s: mix64(seed ^ 0x9e37_79b9_7f4a_7c15) }
    }
    pub fn next(&mut self) -> u64 {
        self.s = self.s.wrapping_add(0x9e37_79b9_7f4a_7c15);
        mix64(self.s)
    }
    /// Uniform in [0, n), unbiased (rejection).
    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0);
        let lim = u64::MAX - u64::MAX % n;
        loop {
            let v = self.next();
            if v < lim {
                return v % n;
            }
        }
    }
    pub fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

pub fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r: u128 = 1;
    let mut bb = (b % m) as u128;
    b = 0;
    let _ = b;
    while e > 0 {
        if e & 1 == 1 {
            r = r * bb % m as u128;
        }
        bb = bb * bb % m as u128;
        e >>= 1;
    }
    r as u64
}

/// Modular inverse for any modulus m > 1 with gcd(a, m) = 1 (None otherwise).
pub fn invmod(a: u64, m: u64) -> Option<u64> {
    let (mut t, mut nt) = (0i128, 1i128);
    let (mut r, mut nr) = (m as i128, (a % m) as i128);
    while nr != 0 {
        let q = r / nr;
        let x = t - q * nt;
        t = nt;
        nt = x;
        let y = r - q * nr;
        r = nr;
        nr = y;
    }
    if r != 1 {
        return None;
    }
    if t < 0 {
        t += m as i128;
    }
    Some(t as u64)
}

pub fn mulmod128(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod128(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

pub fn primes_up_to(n: usize) -> Vec<u64> {
    let mut s = vec![true; n + 1];
    s[0] = false;
    if n >= 1 {
        s[1] = false;
    }
    let mut i = 2;
    while i * i <= n {
        if s[i] {
            let mut j = i * i;
            while j <= n {
                s[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    (2..=n).filter(|&k| s[k]).map(|k| k as u64).collect()
}

/// Legendre symbol of a mod odd prime l: -1, 0, +1.
pub fn legendre(a: u64, l: u64) -> i32 {
    let a = a % l;
    if a == 0 {
        return 0;
    }
    if powmod(a, (l - 1) / 2, l) == 1 {
        1
    } else {
        -1
    }
}

/// Square root mod an odd prime p (Tonelli-Shanks); None for a non-residue.
pub fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if powmod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(powmod(a, (p + 1) / 4, p));
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while powmod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = powmod(z, q, p);
    let mut t = powmod(a, q, p);
    let mut r = powmod(a, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulmod128(tt, tt, p);
            i += 1;
        }
        let mut b = c;
        for _ in 0..(m - i - 1) {
            b = mulmod128(b, b, p);
        }
        m = i;
        c = mulmod128(b, b, p);
        t = mulmod128(t, c, p);
        r = mulmod128(r, b, p);
    }
    Some(r)
}

// ---- flat JSON-lines reading (objects of numbers and strings; nested values skipped) ----

#[derive(Clone, Debug)]
pub enum Val {
    Num(f64),
    Str(String),
    Other,
}
pub type Obj = HashMap<String, Val>;

pub fn parse_obj(line: &str) -> Option<Obj> {
    let b: Vec<char> = line.trim().chars().collect();
    let mut i = 0;
    let ws = |i: &mut usize| {
        while *i < b.len() && b[*i].is_whitespace() {
            *i += 1;
        }
    };
    ws(&mut i);
    if i >= b.len() || b[i] != '{' {
        return None;
    }
    i += 1;
    let mut m = Obj::new();
    loop {
        ws(&mut i);
        if i >= b.len() {
            return None;
        }
        if b[i] == '}' {
            return Some(m);
        }
        if b[i] == ',' {
            i += 1;
            continue;
        }
        if b[i] != '"' {
            return None;
        }
        i += 1;
        let mut k = String::new();
        while i < b.len() && b[i] != '"' {
            k.push(b[i]);
            i += 1;
        }
        i += 1;
        ws(&mut i);
        if i >= b.len() || b[i] != ':' {
            return None;
        }
        i += 1;
        ws(&mut i);
        if i >= b.len() {
            return None;
        }
        let v = if b[i] == '"' {
            i += 1;
            let mut s = String::new();
            while i < b.len() && b[i] != '"' {
                s.push(b[i]);
                i += 1;
            }
            i += 1;
            Val::Str(s)
        } else if b[i] == '{' || b[i] == '[' {
            let mut depth = 0;
            loop {
                match b.get(i) {
                    Some('{') | Some('[') => depth += 1,
                    Some('}') | Some(']') => depth -= 1,
                    Some('"') => {
                        i += 1;
                        while i < b.len() && b[i] != '"' {
                            i += 1;
                        }
                    }
                    None => return None,
                    _ => {}
                }
                i += 1;
                if depth == 0 {
                    break;
                }
            }
            Val::Other
        } else {
            let st = i;
            while i < b.len() && b[i] != ',' && b[i] != '}' {
                i += 1;
            }
            let t: String = b[st..i].iter().collect();
            match t.trim().parse::<f64>() {
                Ok(x) => Val::Num(x),
                Err(_) => Val::Other,
            }
        };
        m.insert(k, v);
    }
}

pub fn num(o: &Obj, k: &str) -> Option<f64> {
    match o.get(k) {
        Some(Val::Num(x)) => Some(*x),
        _ => None,
    }
}
pub fn string(o: &Obj, k: &str) -> Option<String> {
    match o.get(k) {
        Some(Val::Str(s)) => Some(s.clone()),
        _ => None,
    }
}

pub fn read_lines(path: &str, kind: &str) -> Vec<Obj> {
    let txt = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
    txt.lines()
        .filter_map(parse_obj)
        .filter(|o| string(o, "kind").as_deref() == Some(kind))
        .collect()
}

// ---- tiny CLI helpers ----
pub fn opt(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}
pub fn opt_u64(args: &[String], name: &str, default: Option<u64>) -> u64 {
    match opt(args, name) {
        Some(s) => s.replace('_', "").parse().unwrap_or_else(|_| panic!("bad value for {name}")),
        None => default.unwrap_or_else(|| panic!("missing {name}")),
    }
}
pub struct Sink(Option<std::io::BufWriter<std::fs::File>>);
impl Sink {
    pub fn open(path: Option<String>) -> Sink {
        Sink(path.map(|p| std::io::BufWriter::new(std::fs::File::create(p).expect("open output"))))
    }
    pub fn line(&mut self, s: &str) {
        use std::io::Write;
        match &mut self.0 {
            Some(w) => {
                writeln!(w, "{s}").unwrap();
            }
            None => println!("{s}"),
        }
    }
    pub fn flush(&mut self) {
        use std::io::Write;
        if let Some(w) = &mut self.0 {
            w.flush().unwrap();
        }
    }
}
