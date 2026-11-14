//! Shared helpers: hashing, sealed JSON, modular arithmetic, Python-compatible
//! Mersenne Twister (to reconstruct the L1 cell), process counters.
use serde_json::Value;
use sha2::{Digest, Sha256, Sha512};
use std::path::Path;

pub fn sha256_hex(data: &[u8]) -> String {
    hex::encode(Sha256::digest(data))
}

// ---- modular arithmetic on u64 (p < 2^62) ---------------------------------

pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
pub fn addm(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
pub fn subm(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
pub fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
/// Modular inverse by the extended Euclidean algorithm (`a` non-zero mod `p`).
pub fn invm(a: u64, p: u64) -> u64 {
    let (mut t, mut nt) = (0i64, 1i64);
    let (mut r, mut nr) = (p as i64, (a % p) as i64);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    assert_eq!(r, 1, "not invertible");
    if t < 0 {
        t += p as i64;
    }
    t as u64
}
/// Jacobi symbol (Legendre for prime `n`).
pub fn jacobi(a: u64, n: u64) -> i32 {
    let (mut a, mut n) = (a % n, n);
    let mut res = 1i32;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            if n % 8 == 3 || n % 8 == 5 {
                res = -res;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 {
            res = -res;
        }
        a %= n;
    }
    if n == 1 {
        res
    } else {
        0
    }
}
/// Square root mod `p` (Tonelli-Shanks through the library), `muls` counts mulmods.
pub fn sqrt_counted(p: u64, n: u64, muls: &mut u64) -> Option<u64> {
    crypto_lib::cryptanalysis::ic_boundary::PrimeCurve { p, a: 0, b: 0 }.sqrt_counted(n, muls)
}

// ---- sealed JSON ----------------------------------------------------------

pub fn canonical(v: &Value) -> String {
    serde_json::to_string(v).expect("serialise")
}
/// Insert `sha256` = SHA-256 of the compact, key-sorted serialisation of the
/// object without the `sha256` key.
pub fn seal(mut v: Value) -> Value {
    v.as_object_mut().expect("object").remove("sha256");
    let h = sha256_hex(canonical(&v).as_bytes());
    v["sha256"] = Value::String(h);
    v
}
pub fn check_seal(v: &Value) -> bool {
    let mut w = v.clone();
    let Some(o) = w.as_object_mut() else {
        return false;
    };
    let Some(Value::String(h)) = o.remove("sha256") else {
        return false;
    };
    sha256_hex(canonical(&w).as_bytes()) == h
}
pub fn write_json(path: &Path, v: &Value) -> Result<(), String> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d).map_err(|e| format!("mkdir {}: {e}", d.display()))?;
    }
    let mut s = serde_json::to_string_pretty(v).map_err(|e| e.to_string())?;
    s.push('\n');
    std::fs::write(path, s).map_err(|e| format!("write {}: {e}", path.display()))
}
pub fn read_json(path: &Path) -> Result<Value, String> {
    let s = std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&s).map_err(|e| format!("parse {}: {e}", path.display()))
}
pub fn file_sha256(path: &Path) -> Result<String, String> {
    std::fs::read(path)
        .map(|b| sha256_hex(&b))
        .map_err(|e| format!("read {}: {e}", path.display()))
}
pub fn u64_of(v: &Value, key: &str) -> Result<u64, String> {
    v[key]
        .as_u64()
        .ok_or_else(|| format!("missing integer field {key}"))
}
pub fn u64_vec(v: &Value) -> Result<Vec<u64>, String> {
    v.as_array()
        .ok_or("expected array")?
        .iter()
        .map(|x| x.as_u64().ok_or_else(|| "expected integer".to_string()))
        .collect()
}

// ---- Python-compatible random.Random(str) ---------------------------------

/// MT19937 seeded as CPython does for a `str` seed (version 2): the UTF-8
/// bytes followed by their SHA-512, read as a big-endian integer, split into
/// little-endian 32-bit words and fed to `init_by_array`.
pub struct PyRandom {
    mt: [u32; 624],
    idx: usize,
}

impl PyRandom {
    fn init_genrand(s: u32) -> Self {
        let mut mt = [0u32; 624];
        mt[0] = s;
        for i in 1..624 {
            mt[i] = 1812433253u32
                .wrapping_mul(mt[i - 1] ^ (mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        PyRandom { mt, idx: 624 }
    }
    pub fn from_key(key: &[u32]) -> Self {
        let mut r = Self::init_genrand(19650218);
        let (mut i, mut j) = (1usize, 0usize);
        for _ in 0..624.max(key.len()) {
            let prev = r.mt[i - 1];
            r.mt[i] = (r.mt[i] ^ (prev ^ (prev >> 30)).wrapping_mul(1664525))
                .wrapping_add(key[j])
                .wrapping_add(j as u32);
            i += 1;
            j += 1;
            if i >= 624 {
                r.mt[0] = r.mt[623];
                i = 1;
            }
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..623 {
            let prev = r.mt[i - 1];
            r.mt[i] = (r.mt[i] ^ (prev ^ (prev >> 30)).wrapping_mul(1566083941))
                .wrapping_sub(i as u32);
            i += 1;
            if i >= 624 {
                r.mt[0] = r.mt[623];
                i = 1;
            }
        }
        r.mt[0] = 0x8000_0000;
        r.idx = 624;
        r
    }
    pub fn from_str_seed(s: &str) -> Self {
        let mut bytes = s.as_bytes().to_vec();
        bytes.extend_from_slice(&Sha512::digest(s.as_bytes()));
        // big-endian integer -> little-endian 32-bit words
        let first = bytes[0];
        let bits = 8 * bytes.len() - first.leading_zeros() as usize;
        let words = (bits - 1) / 32 + 1;
        let le: Vec<u8> = bytes.iter().rev().copied().collect();
        let key: Vec<u32> = (0..words)
            .map(|w| {
                let mut b = [0u8; 4];
                for (k, slot) in b.iter_mut().enumerate() {
                    *slot = le.get(4 * w + k).copied().unwrap_or(0);
                }
                u32::from_le_bytes(b)
            })
            .collect();
        Self::from_key(&key)
    }
    pub fn next_u32(&mut self) -> u32 {
        if self.idx >= 624 {
            for k in 0..624 {
                let y = (self.mt[k] & 0x8000_0000) | (self.mt[(k + 1) % 624] & 0x7fff_ffff);
                let mut v = self.mt[(k + 397) % 624] ^ (y >> 1);
                if y & 1 == 1 {
                    v ^= 0x9908_b0df;
                }
                self.mt[k] = v;
            }
            self.idx = 0;
        }
        let mut y = self.mt[self.idx];
        self.idx += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^= y >> 18;
        y
    }
    /// `random.randrange(start, stop)` for `stop - start < 2^32`.
    pub fn randrange(&mut self, start: u64, stop: u64) -> u64 {
        let n = stop - start;
        let k = 64 - n.leading_zeros();
        loop {
            let r = u64::from(self.next_u32() >> (32 - k));
            if r < n {
                return start + r;
            }
        }
    }
}

// ---- process counters -----------------------------------------------------

pub fn cpu_secs() -> f64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: plain libc call with a valid out pointer.
    unsafe {
        libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut ts);
    }
    ts.tv_sec as f64 + ts.tv_nsec as f64 * 1e-9
}
pub fn vm_hwm_kb() -> Option<u64> {
    let s = std::fs::read_to_string("/proc/self/status").ok()?;
    s.lines()
        .find(|l| l.starts_with("VmHWM:"))?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}
