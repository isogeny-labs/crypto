//! Release timing of `src/pqc/strassen.rs` (included by path, unchanged) over
//! the four fields the toy PQC schemes use.
//!
//! The field operations below are copies of the crate's private ones, checked
//! identical on 2026-11-24: `gf_mul` in `src/pqc/mayo.rs` (GF(16)) and
//! `src/pqc/uov.rs` (GF(256)), `w_add`/`w_sub`/`w_mul` in `src/pqc/qr_uov.rs`
//! (F_31[x]/(x^3 - x - 1)), and the McEliece GF(2) ring in
//! `src/pqc/mceliece.rs` (AND/XOR, `CUTOFF = usize::MAX`). The `old_*`
//! functions are the loops each scheme used before the Strassen path
//! (McEliece i-j-k, the others i-l-j with the zero skip).
//!
//! Usage: `pqc-strassen-harness regression|crossover OUT.json`
//!
//! Every cell is timed as one warm-up call followed by `SAMPLES` timed batches
//! of `reps` calls; the file keeps all per-call batch means, so the record can
//! state n, median and spread. One thread, no pinning.
#![allow(dead_code)]
#[path = "../../../../src/pqc/strassen.rs"]
mod strassen;

use std::fmt::Write as _;
use std::hint::black_box;
use std::time::Instant;
use strassen::{mul, mul_with_cutoff, naive, Ring};

const SAMPLES: usize = 5;

fn gf16_mul(a: u8, b: u8) -> u8 {
    let (mut a, mut b, mut r) = (a, b, 0u8);
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        a <<= 1;
        if a & 0x10 != 0 {
            a ^= 0x13;
        }
        b >>= 1;
    }
    r & 0x0f
}
fn gf256_mul(a: u8, b: u8) -> u8 {
    let (mut a, mut b, mut r) = (a as u16, b as u16, 0u16);
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        a <<= 1;
        if a & 0x100 != 0 {
            a ^= 0x11b;
        }
        b >>= 1;
    }
    r as u8
}
type W = [u32; 3];
const Q: u32 = 31;
fn w_add(a: &W, b: &W) -> W {
    [(a[0] + b[0]) % Q, (a[1] + b[1]) % Q, (a[2] + b[2]) % Q]
}
fn w_sub(a: &W, b: &W) -> W {
    [(a[0] + Q - b[0]) % Q, (a[1] + Q - b[1]) % Q, (a[2] + Q - b[2]) % Q]
}
fn w_mul(a: &W, b: &W) -> W {
    let mut t = [0u32; 5];
    for i in 0..3 {
        for j in 0..3 {
            t[i + j] = (t[i + j] + a[i] * b[j]) % Q;
        }
    }
    [(t[0] + t[3]) % Q, (t[1] + t[3] + t[4]) % Q, (t[2] + t[4]) % Q]
}

struct Gf2;
impl Ring for Gf2 {
    type Elt = u8;
    const ZERO: u8 = 0;
    const SKIP_ZERO: bool = false;
    const CUTOFF: usize = usize::MAX;
    fn add(a: u8, b: u8) -> u8 { a ^ b }
    fn sub(a: u8, b: u8) -> u8 { a ^ b }
    fn mul(a: u8, b: u8) -> u8 { a & b }
}
struct Gf16;
impl Ring for Gf16 {
    type Elt = u8;
    const ZERO: u8 = 0;
    const SKIP_ZERO: bool = true;
    fn add(a: u8, b: u8) -> u8 { a ^ b }
    fn sub(a: u8, b: u8) -> u8 { a ^ b }
    fn mul(a: u8, b: u8) -> u8 { gf16_mul(a, b) }
}
struct Gf256;
impl Ring for Gf256 {
    type Elt = u8;
    const ZERO: u8 = 0;
    const SKIP_ZERO: bool = true;
    fn add(a: u8, b: u8) -> u8 { a ^ b }
    fn sub(a: u8, b: u8) -> u8 { a ^ b }
    fn mul(a: u8, b: u8) -> u8 { gf256_mul(a, b) }
}
struct WRing;
impl Ring for WRing {
    type Elt = W;
    const ZERO: W = [0; 3];
    const SKIP_ZERO: bool = true;
    fn add(a: W, b: W) -> W { w_add(&a, &b) }
    fn sub(a: W, b: W) -> W { w_sub(&a, &b) }
    fn mul(a: W, b: W) -> W { w_mul(&a, &b) }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
fn rand_mat<T>(rng: &mut Rng, n: usize, m: usize, elt: &impl Fn(u64) -> T) -> Vec<Vec<T>> {
    (0..n).map(|_| (0..m).map(|_| elt(rng.next())).collect()).collect()
}

/// Per-call seconds of each of `SAMPLES` batches of `reps` calls, after one warm-up call.
fn samples(reps: usize, f: &dyn Fn()) -> Vec<f64> {
    f();
    (0..SAMPLES)
        .map(|_| {
            let t = Instant::now();
            for _ in 0..reps {
                f();
            }
            t.elapsed().as_secs_f64() / reps as f64
        })
        .collect()
}

fn json_f64s(v: &[f64]) -> String {
    let parts: Vec<String> = v.iter().map(|x| format!("{x:.6e}")).collect();
    format!("[{}]", parts.join(", "))
}

#[allow(clippy::too_many_arguments)]
fn row(out: &mut Vec<String>, mode: &str, field: &str, impl_: &str, shape: (usize, usize, usize), cutoff: Option<usize>, reps: usize, s: &[f64]) {
    let mut line = String::new();
    let cutoff = cutoff.map_or("null".to_string(), |c| c.to_string());
    write!(
        line,
        "{{\"mode\": \"{mode}\", \"field\": \"{field}\", \"impl\": \"{impl_}\", \"r\": {}, \"k\": {}, \"c\": {}, \"cutoff\": {cutoff}, \"reps\": {reps}, \"seconds_per_call\": {}}}",
        shape.0, shape.1, shape.2, json_f64s(s)
    )
    .unwrap();
    out.push(line);
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    s[s.len() / 2]
}

type Loop<T> = fn(&[Vec<T>], &[Vec<T>]) -> Vec<Vec<T>>;

/// Old loop against the new `strassen::mul` at the scheme's own cutoff, on the shapes the schemes use.
fn regress<R: Ring>(out: &mut Vec<String>, field: &str, old: Loop<R::Elt>, elt: impl Fn(u64) -> R::Elt)
where
    R::Elt: std::fmt::Debug,
{
    let mut rng = Rng(42);
    for shape in [(12, 12, 12), (20, 20, 20), (32, 32, 32), (36, 36, 36), (12, 4, 12), (36, 24, 12)] {
        let (r, k, c) = shape;
        let (a, b) = (rand_mat(&mut rng, r, k, &elt), rand_mat(&mut rng, k, c, &elt));
        assert_eq!(old(&a, &b), mul::<R>(&a, &b), "{field} {shape:?}: old and new disagree");
        let reps = 3000;
        let so = samples(reps, &|| {
            black_box(old(black_box(&a), black_box(&b)));
        });
        let sn = samples(reps, &|| {
            black_box(mul::<R>(black_box(&a), black_box(&b)));
        });
        row(out, "regression", field, "old", shape, None, reps, &so);
        row(out, "regression", field, "new", shape, Some(R::CUTOFF), reps, &sn);
        eprintln!("  {field:7} {r}x{k}x{c}: old {:8.2} us, new/old {:4.2}", median(&so) * 1e6, median(&sn) / median(&so));
    }
}

/// Schoolbook against Strassen at cutoffs 16/32/64 on square n x n matrices.
fn crossover<R: Ring>(out: &mut Vec<String>, field: &str, elt: impl Fn(u64) -> R::Elt) {
    let mut rng = Rng(0xC0FFEE);
    for n in [36usize, 48, 64, 96, 128, 192, 256, 384, 512] {
        let (a, b) = (rand_mat(&mut rng, n, n, &elt), rand_mat(&mut rng, n, n, &elt));
        let reps = (400_000_000 / (n * n * n)).clamp(3, 2000);
        let base = samples(reps, &|| {
            black_box(naive::<R>(black_box(&a), black_box(&b)));
        });
        row(out, "crossover", field, "schoolbook", (n, n, n), None, reps, &base);
        let mut cells = String::new();
        for c in [16, 32, 64] {
            let s = samples(reps, &|| {
                black_box(mul_with_cutoff::<R>(black_box(&a), black_box(&b), c));
            });
            row(out, "crossover", field, "strassen", (n, n, n), Some(c), reps, &s);
            write!(cells, " c{c} {:4.2}", median(&s) / median(&base)).unwrap();
        }
        eprintln!("  {field:7} n={n:4}: schoolbook {:9.3} ms,{cells}", median(&base) * 1e3);
    }
}

fn old_gf256(a: &[Vec<u8>], b: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let (r, k, c) = (a.len(), b.len(), b[0].len());
    let mut out = vec![vec![0u8; c]; r];
    for i in 0..r {
        for l in 0..k {
            if a[i][l] == 0 {
                continue;
            }
            for j in 0..c {
                out[i][j] ^= gf256_mul(a[i][l], b[l][j]);
            }
        }
    }
    out
}
fn old_gf16(a: &[Vec<u8>], b: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let (r, k, c) = (a.len(), b.len(), b[0].len());
    let mut out = vec![vec![0u8; c]; r];
    for i in 0..r {
        for l in 0..k {
            if a[i][l] == 0 {
                continue;
            }
            for j in 0..c {
                out[i][j] ^= gf16_mul(a[i][l], b[l][j]);
            }
        }
    }
    out
}
fn old_gf2(a: &[Vec<u8>], b: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let (r, inner, c) = (a.len(), b.len(), b[0].len());
    let mut out = vec![vec![0u8; c]; r];
    for i in 0..r {
        for j in 0..c {
            let mut s = 0u8;
            for k in 0..inner {
                s ^= a[i][k] & b[k][j];
            }
            out[i][j] = s;
        }
    }
    out
}
fn old_w(a: &[Vec<W>], b: &[Vec<W>]) -> Vec<Vec<W>> {
    let (r, k, c) = (a.len(), b.len(), b[0].len());
    let mut out = vec![vec![[0u32; 3]; c]; r];
    for i in 0..r {
        for l in 0..k {
            if a[i][l] == [0; 3] {
                continue;
            }
            for j in 0..c {
                out[i][j] = w_add(&out[i][j], &w_mul(&a[i][l], &b[l][j]));
            }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (mode, path) = match args.as_slice() {
        [_, m, p] if m == "regression" || m == "crossover" => (m.clone(), p.clone()),
        _ => {
            eprintln!("usage: pqc-strassen-harness regression|crossover OUT.json");
            std::process::exit(2);
        }
    };
    let w = |x: u64| [(x % 31) as u32, ((x >> 16) % 31) as u32, ((x >> 32) % 31) as u32];
    let mut rows = Vec::new();
    if mode == "regression" {
        regress::<Gf2>(&mut rows, "GF(2)", old_gf2, |x| (x & 1) as u8);
        regress::<Gf16>(&mut rows, "GF(16)", old_gf16, |x| (x & 15) as u8);
        regress::<Gf256>(&mut rows, "GF(256)", old_gf256, |x| x as u8);
        regress::<WRing>(&mut rows, "F_31^3", old_w, w);
    } else {
        crossover::<Gf2>(&mut rows, "GF(2)", |x| (x & 1) as u8);
        crossover::<Gf16>(&mut rows, "GF(16)", |x| (x & 15) as u8);
        crossover::<Gf256>(&mut rows, "GF(256)", |x| x as u8);
        crossover::<WRing>(&mut rows, "F_31^3", w);
    }
    let body = format!(
        "{{\"schema\": \"pqc-strassen-timing/v1\", \"mode\": \"{mode}\", \"samples_per_cell\": {SAMPLES}, \"strassen_cutoff_default\": {}, \"rows\": [\n  {}\n]}}\n",
        strassen::STRASSEN_CUTOFF,
        rows.join(",\n  ")
    );
    std::fs::write(&path, body).expect("write output");
    eprintln!("wrote {} rows to {path}", rows.len());
}
