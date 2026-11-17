//! crypto-ml-v1 reduced-round neural distinguishers (2026-11-02).
//!
//! Task (Gohr, CRYPTO 2019 "real-vs-random"): label 1 = ciphertext pair (E_k(P), E_k(P ⊕ Δ)); label 0 =
//! (E_k(P), E_k(P')) with P' independent. Every sample has its own fresh key, so the independent unit is the key and
//! train/test are key-disjoint by construction. Models: a small MLP, logistic regression, and a classical
//! difference-table naive-Bayes baseline. Controls: a low-round positive control, a label-shuffle negative control,
//! and an A/A arm (same data, second training seed). Decision rule and sizes: PROTOCOL.md.
//!
//! Usage: neural-distinguishers preflight | run <out dir> [cell ...] | summary <out dir> | list
mod ciphers;

use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::time::Instant;

use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rayon::prelude::*;

const SEED: u64 = 0x6e64_6973_7420_1102; // PROTOCOL.md §4
const SPECK_DELTA: (u16, u16) = (0x0040, 0x0000); // Gohr's input difference
const AES_DELTA_BYTE0: u8 = 0x01;

#[derive(Clone, Copy)]
struct Cell {
    name: &'static str,
    cipher: &'static str,
    rounds: usize,
    role: &'static str, // positive_control | target
    n_train: usize,
    n_test: usize,
    epochs: usize,
}

const CELLS: &[Cell] = &[
    Cell { name: "speck-r3", cipher: "speck32_64", rounds: 3, role: "positive_control", n_train: 1 << 18, n_test: 1 << 17, epochs: 4 },
    Cell { name: "speck-r5", cipher: "speck32_64", rounds: 5, role: "target", n_train: 1 << 20, n_test: 1 << 17, epochs: 10 },
    Cell { name: "speck-r6", cipher: "speck32_64", rounds: 6, role: "target", n_train: 1 << 20, n_test: 1 << 17, epochs: 10 },
    Cell { name: "speck-r7", cipher: "speck32_64", rounds: 7, role: "target", n_train: 1 << 20, n_test: 1 << 17, epochs: 10 },
    Cell { name: "aes-r2", cipher: "aes128", rounds: 2, role: "positive_control", n_train: 1 << 17, n_test: 1 << 17, epochs: 4 },
    Cell { name: "aes-r3", cipher: "aes128", rounds: 3, role: "target", n_train: 1 << 19, n_test: 1 << 17, epochs: 6 },
    Cell { name: "aes-r4", cipher: "aes128", rounds: 4, role: "target", n_train: 1 << 19, n_test: 1 << 17, epochs: 6 },
];

fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h = (h ^ b as u64).wrapping_mul(0x100_0000_01b3);
    }
    h ^ SEED
}

// ------------------------------------------------------------------ data

/// Bit-features (0/1 bytes, row-major) and labels. Speck: C0 ‖ C1 ‖ C0⊕C1 (96 bits). AES: C0 ‖ C1 ‖ C0⊕C1 (384 bits).
struct Data {
    d: usize,
    x: Vec<u8>,
    y: Vec<u8>,
    diff_words: Vec<Vec<u16>>, // per sample: ciphertext difference split into words for the table baseline
}

fn push_bits(out: &mut Vec<u8>, v: u64, bits: usize) {
    for i in (0..bits).rev() {
        out.push(((v >> i) & 1) as u8);
    }
}

fn generate(cell: &Cell, split: &str) -> Data {
    let n = if split == "train" { cell.n_train } else { cell.n_test };
    let seed = fnv(&format!("{}/{split}", cell.name));
    let chunks = 64;
    let per = n / chunks;
    let parts: Vec<(Vec<u8>, Vec<u8>, Vec<Vec<u16>>)> = (0..chunks).into_par_iter().map(|c| {
        let mut rng = StdRng::seed_from_u64(seed ^ (c as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        let (mut x, mut y, mut dw) = (Vec::new(), Vec::new(), Vec::new());
        for j in 0..per {
            // exactly balanced classes (alternating within each chunk; training shuffles order): a constant predictor
            // then scores exactly 1/2, so accuracy has no class-prevalence offset (PROTOCOL.md §8, dry run).
            let label: u8 = (j & 1) as u8;
            match cell.cipher {
                "speck32_64" => {
                    let ks = ciphers::speck_expand(rng.gen(), cell.rounds);
                    let p: (u16, u16) = rng.gen();
                    let p1 = if label == 1 { (p.0 ^ SPECK_DELTA.0, p.1 ^ SPECK_DELTA.1) } else { rng.gen() };
                    let (a, b) = (ciphers::speck_encrypt(p, &ks), ciphers::speck_encrypt(p1, &ks));
                    let (c0, c1) = (((a.0 as u64) << 16) | a.1 as u64, ((b.0 as u64) << 16) | b.1 as u64);
                    push_bits(&mut x, c0, 32);
                    push_bits(&mut x, c1, 32);
                    push_bits(&mut x, c0 ^ c1, 32);
                    dw.push(vec![a.0 ^ b.0, a.1 ^ b.1]);
                }
                _ => {
                    let rk = ciphers::aes_expand(rng.gen(), cell.rounds);
                    let p: [u8; 16] = rng.gen();
                    let mut p1 = p;
                    if label == 1 { p1[0] ^= AES_DELTA_BYTE0 } else { p1 = rng.gen() }
                    let (a, b) = (ciphers::aes_encrypt(p, &rk), ciphers::aes_encrypt(p1, &rk));
                    for v in a.iter().chain(b.iter()) { push_bits(&mut x, *v as u64, 8); }
                    for i in 0..16 { push_bits(&mut x, (a[i] ^ b[i]) as u64, 8); }
                    dw.push((0..16).map(|i| (a[i] ^ b[i]) as u16).collect());
                }
            }
            y.push(label);
        }
        (x, y, dw)
    }).collect();
    let d = if cell.cipher == "speck32_64" { 96 } else { 384 };
    let mut data = Data { d, x: Vec::with_capacity(n * d), y: Vec::with_capacity(n), diff_words: Vec::with_capacity(n) };
    for (x, y, dw) in parts {
        data.x.extend(x);
        data.y.extend(y);
        data.diff_words.extend(dw);
    }
    data
}

// ------------------------------------------------------------------ MLP (d -> h -> h -> 1), ReLU, BCE, Adam; h = 0 is logistic regression

#[derive(Clone)]
struct Mlp {
    d: usize,
    h: usize,
    p: Vec<f32>, // flattened parameters
}

impl Mlp {
    fn new(d: usize, h: usize, rng: &mut StdRng) -> Self {
        let n = Self::size(d, h);
        let mut p = vec![0f32; n];
        let (w1, w2) = ((2.0 / d as f32).sqrt(), (2.0 / h.max(1) as f32).sqrt());
        let mut idx = 0;
        if h > 0 {
            for _ in 0..d * h { p[idx] = w1 * (rng.gen::<f32>() * 2.0 - 1.0) * 1.7; idx += 1; }
            idx += h;
            for _ in 0..h * h { p[idx] = w2 * (rng.gen::<f32>() * 2.0 - 1.0) * 1.7; idx += 1; }
            idx += h;
            for _ in 0..h { p[idx] = w2 * (rng.gen::<f32>() * 2.0 - 1.0); idx += 1; }
        }
        Mlp { d, h, p }
    }
    fn size(d: usize, h: usize) -> usize {
        if h == 0 { d + 1 } else { d * h + h + h * h + h + h + 1 }
    }
    /// Forward pass; if `grad` is given, accumulates the BCE gradient for target `y` and returns the loss.
    fn step(&self, x: &[u8], y: f32, grad: Option<&mut [f32]>, buf: &mut Vec<f32>) -> (f32, f32) {
        let (d, h) = (self.d, self.h);
        if h == 0 {
            let mut z = self.p[d];
            for k in 0..d { if x[k] != 0 { z += self.p[k]; } }
            let p = sigmoid(z);
            if let Some(g) = grad {
                let dz = p - y;
                for k in 0..d { if x[k] != 0 { g[k] += dz; } }
                g[d] += dz;
            }
            return (p, bce(p, y));
        }
        let (o_b1, o_w2) = (d * h, d * h + h);
        let (o_b2, o_w3) = (o_w2 + h * h, o_w2 + h * h + h);
        let o_b3 = o_w3 + h;
        buf.resize(4 * h, 0.0);
        let (z1, rest) = buf.split_at_mut(h);
        let (z2, rest) = rest.split_at_mut(h);
        let (dz2, dz1) = rest.split_at_mut(h);
        z1.copy_from_slice(&self.p[o_b1..o_b1 + h]);
        for k in 0..d {
            if x[k] != 0 {
                let row = &self.p[k * h..k * h + h];
                for j in 0..h { z1[j] += row[j]; }
            }
        }
        z2.copy_from_slice(&self.p[o_b2..o_b2 + h]);
        for i in 0..h {
            let a = z1[i].max(0.0);
            if a != 0.0 {
                let row = &self.p[o_w2 + i * h..o_w2 + i * h + h];
                for j in 0..h { z2[j] += a * row[j]; }
            }
        }
        let mut z3 = self.p[o_b3];
        for j in 0..h { z3 += z2[j].max(0.0) * self.p[o_w3 + j]; }
        let p = sigmoid(z3);
        if let Some(g) = grad {
            let dz3 = p - y;
            for j in 0..h {
                g[o_w3 + j] += dz3 * z2[j].max(0.0);
                dz2[j] = if z2[j] > 0.0 { dz3 * self.p[o_w3 + j] } else { 0.0 };
                g[o_b2 + j] += dz2[j];
            }
            g[o_b3] += dz3;
            for i in 0..h {
                let a = z1[i].max(0.0);
                let row = &self.p[o_w2 + i * h..o_w2 + i * h + h];
                let mut s = 0.0;
                for j in 0..h {
                    s += row[j] * dz2[j];
                }
                if a != 0.0 {
                    let grow = &mut g[o_w2 + i * h..o_w2 + i * h + h];
                    for j in 0..h { grow[j] += a * dz2[j]; }
                }
                dz1[i] = if z1[i] > 0.0 { s } else { 0.0 };
                g[o_b1 + i] += dz1[i];
            }
            for k in 0..d {
                if x[k] != 0 {
                    let grow = &mut g[k * h..k * h + h];
                    for j in 0..h { grow[j] += dz1[j]; }
                }
            }
        }
        (p, bce(p, y))
    }
}

fn sigmoid(z: f32) -> f32 { 1.0 / (1.0 + (-z).exp()) }
fn bce(p: f32, y: f32) -> f32 { let p = p.clamp(1e-7, 1.0 - 1e-7); -(y * p.ln() + (1.0 - y) * (1.0 - p).ln()) }

struct Fit { model: Mlp, train_loss: f32, seconds: f64 }

fn train(data: &Data, labels: &[u8], h: usize, epochs: usize, seed: u64) -> Fit {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut m = Mlp::new(data.d, h, &mut rng);
    let n = labels.len();
    let (batch, lr0, b1, b2, eps) = (512usize, 2e-3f32, 0.9f32, 0.999f32, 1e-8f32);
    let np = m.p.len();
    let (mut mo, mut ve) = (vec![0f32; np], vec![0f32; np]);
    let mut order: Vec<usize> = (0..n).collect();
    let t0 = Instant::now();
    let mut t = 0i32;
    let mut last_loss = 0.0;
    let threads = rayon::current_num_threads();
    for ep in 0..epochs {
        order.shuffle(&mut rng);
        let lr = lr0 * (1.0 - ep as f32 / epochs as f32).max(0.1); // linear decay
        let mut ep_loss = 0.0f64;
        for chunk in order.chunks(batch) {
            let parts: Vec<(Vec<f32>, f32)> = chunk.par_chunks(chunk.len().div_ceil(threads)).map(|sub| {
                let mut g = vec![0f32; np];
                let mut buf = Vec::new();
                let mut loss = 0.0;
                for &i in sub {
                    loss += m.step(&data.x[i * data.d..(i + 1) * data.d], labels[i] as f32, Some(&mut g), &mut buf).1;
                }
                (g, loss)
            }).collect();
            let mut g = vec![0f32; np];
            for (pg, l) in parts {
                ep_loss += l as f64;
                for k in 0..np { g[k] += pg[k]; }
            }
            t += 1;
            let (c1, c2) = (1.0 - b1.powi(t), 1.0 - b2.powi(t));
            let inv = 1.0 / chunk.len() as f32;
            for k in 0..np {
                let gk = g[k] * inv + 1e-5 * m.p[k];
                mo[k] = b1 * mo[k] + (1.0 - b1) * gk;
                ve[k] = b2 * ve[k] + (1.0 - b2) * gk * gk;
                m.p[k] -= lr * (mo[k] / c1) / ((ve[k] / c2).sqrt() + eps);
            }
        }
        last_loss = (ep_loss / n as f64) as f32;
    }
    Fit { model: m, train_loss: last_loss, seconds: t0.elapsed().as_secs_f64() }
}

fn accuracy(m: &Mlp, data: &Data) -> usize {
    accuracy_on(m, data, &data.y)
}

fn accuracy_on(m: &Mlp, data: &Data, labels: &[u8]) -> usize {
    (0..labels.len()).into_par_iter().map_init(Vec::new, |buf, i| {
        let p = m.step(&data.x[i * data.d..(i + 1) * data.d], 0.0, None, buf).0;
        ((p >= 0.5) as u8 == labels[i]) as usize
    }).sum()
}

// ------------------------------------------------------------------ classical baseline: difference-table naive Bayes

/// Per-word empirical distribution of the ciphertext difference under the "real" class (Laplace-smoothed) versus the
/// uniform distribution of the "random" class; log-likelihood ratio summed over words, threshold fit on training data.
fn diff_table_nb(train: &Data, test: &Data, word_bits: u32) -> (usize, f64) {
    let t0 = Instant::now();
    let words = train.diff_words[0].len();
    let size = 1usize << word_bits;
    let mut counts = vec![vec![1u32; size]; words];
    let mut n_real = 0u32;
    for (i, dw) in train.diff_words.iter().enumerate() {
        if train.y[i] == 1 {
            n_real += 1;
            for (w, v) in dw.iter().enumerate() { counts[w][*v as usize] += 1; }
        }
    }
    let logp: Vec<Vec<f32>> = counts.iter().map(|c| c.iter().map(|&k| ((k as f64 / (n_real as f64 + size as f64)) * size as f64).ln() as f32).collect()).collect();
    let score = |dw: &Vec<u16>| dw.iter().enumerate().map(|(w, v)| logp[w][*v as usize]).sum::<f32>();
    // threshold maximising training accuracy (scores are LLRs; 0 is the Bayes threshold under equal priors)
    let mut tr: Vec<(f32, u8)> = train.diff_words.iter().zip(&train.y).map(|(d, y)| (score(d), *y)).collect();
    tr.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let total_pos = tr.iter().filter(|x| x.1 == 1).count();
    let (mut best, mut best_thr, mut neg_below, mut pos_below) = (0usize, f32::NEG_INFINITY, 0usize, 0usize);
    for (s, y) in &tr {
        if *y == 1 { pos_below += 1 } else { neg_below += 1 }
        let correct = neg_below + (total_pos - pos_below);
        if correct > best { best = correct; best_thr = *s; }
    }
    let ok = test.diff_words.iter().zip(&test.y).filter(|(d, y)| ((score(d) > best_thr) as u8) == **y).count();
    (ok, t0.elapsed().as_secs_f64())
}

// ------------------------------------------------------------------ statistics

/// Wilson score interval for k successes in n trials at the given z.
fn wilson(k: usize, n: usize, z: f64) -> (f64, f64) {
    let (p, n) = (k as f64 / n as f64, n as f64);
    let den = 1.0 + z * z / n;
    let c = (p + z * z / (2.0 * n)) / den;
    let r = z * (p * (1.0 - p) / n + z * z / (4.0 * n * n)).sqrt() / den;
    (c - r, c + r)
}

// ------------------------------------------------------------------ experiment

const HIDDEN: usize = 128;
// Bonferroni family: 7 cells x 3 models (mlp, logreg, table) = 21 one-sided tests at alpha = 0.01 -> z for 0.01/21.
const Z_BONF: f64 = 3.52;
const Z95: f64 = 1.96;

fn run(out: &Path, only: &[String]) {
    fs::create_dir_all(out).unwrap();
    let mut rows = fs::OpenOptions::new().create(true).append(true).open(out.join("rows.jsonl")).unwrap();
    for cell in CELLS.iter().filter(|c| only.is_empty() || only.iter().any(|o| o == c.name)) {
        let t0 = Instant::now();
        let (tr, te) = (generate(cell, "train"), generate(cell, "test"));
        let gen_s = t0.elapsed().as_secs_f64();
        let n = te.y.len();
        let mut emit = |model: &str, arm: &str, correct: usize, train_s: f64, loss: f32| {
            let (lo, hi) = wilson(correct, n, Z95);
            let (blo, _) = wilson(correct, n, Z_BONF);
            let line = format!(
                "{{\"cell\":\"{}\",\"cipher\":\"{}\",\"rounds\":{},\"role\":\"{}\",\"model\":\"{model}\",\"arm\":\"{arm}\",\"n_train\":{},\"n_test\":{n},\"correct\":{correct},\"accuracy\":{:.6},\"wilson95\":[{lo:.6},{hi:.6}],\"bonferroni_lower\":{blo:.6},\"train_loss\":{},\"train_seconds\":{train_s:.2},\"gen_seconds\":{gen_s:.2},\"independent_unit\":\"key (one fresh key per sample)\"}}",
                cell.name, cell.cipher, cell.rounds, cell.role, tr.y.len(), correct as f64 / n as f64,
                if loss.is_nan() { "null".to_string() } else { format!("{loss:.6}") });
            println!("{line}");
            writeln!(rows, "{line}").unwrap();
        };
        // arms: MLP seed 1 (primary), MLP seed 2 (A/A), logistic regression, MLP on shuffled labels (negative control)
        let base = fnv(cell.name);
        let f1 = train(&tr, &tr.y, HIDDEN, cell.epochs, base ^ 1);
        emit("mlp", "primary", accuracy(&f1.model, &te), f1.seconds, f1.train_loss);
        let f2 = train(&tr, &tr.y, HIDDEN, cell.epochs, base ^ 2);
        emit("mlp", "aa_seed2", accuracy(&f2.model, &te), f2.seconds, f2.train_loss);
        let lr = train(&tr, &tr.y, 0, cell.epochs, base ^ 3);
        emit("logreg", "primary", accuracy(&lr.model, &te), lr.seconds, lr.train_loss);
        // Negative control: labels permuted in BOTH train and test, so labels are independent of every feature and
        // any model's test accuracy is Binomial(n, 1/2)/n. (Shuffling only the training labels is not a valid null:
        // see PROTOCOL.md §8, dry run.)
        let mut shuffled = tr.y.clone();
        shuffled.shuffle(&mut StdRng::seed_from_u64(base ^ 4));
        let mut shuffled_test = te.y.clone();
        shuffled_test.shuffle(&mut StdRng::seed_from_u64(base ^ 6));
        let fs_ = train(&tr, &shuffled, HIDDEN, cell.epochs, base ^ 5);
        emit("mlp", "label_shuffle_control", accuracy_on(&fs_.model, &te, &shuffled_test), fs_.seconds, fs_.train_loss);
        let (ok, s) = diff_table_nb(&tr, &te, if cell.cipher == "speck32_64" { 16 } else { 8 });
        emit("difference_table_nb", "primary", ok, s, f32::NAN);
    }
}

fn preflight() {
    // time data generation and one MLP epoch at reduced size per cipher, to size the protocol (PROTOCOL.md §7)
    for name in ["speck-r5", "aes-r3"] {
        let mut cell = *CELLS.iter().find(|c| c.name == name).unwrap();
        cell.n_train = 1 << 16;
        cell.n_test = 1 << 14;
        let t0 = Instant::now();
        let tr = generate(&cell, "train");
        let g = t0.elapsed().as_secs_f64();
        let f = train(&tr, &tr.y, HIDDEN, 1, 7);
        let lr = train(&tr, &tr.y, 0, 1, 7);
        println!("{name}: gen {:.2}s per 2^16 samples; mlp {:.2}s per epoch per 2^16; logreg {:.2}s per epoch per 2^16; threads {}",
                 g, f.seconds, lr.seconds, rayon::current_num_threads());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("list") => {
            let mut s = String::new();
            for c in CELLS { let _ = writeln!(s, "{}\t{}\t{}\t{}\ttrain 2^{}\ttest 2^{}\tepochs {}", c.name, c.cipher, c.rounds, c.role, c.n_train.trailing_zeros(), c.n_test.trailing_zeros(), c.epochs); }
            print!("{s}");
        }
        Some("preflight") => preflight(),
        Some("run") => run(Path::new(&args[2]), &args[3..]),
        Some("summary") => summary(Path::new(&args[2])),
        _ => eprintln!("usage: neural-distinguishers list | preflight | run <out dir> [cell ...]"),
    }
}

// ------------------------------------------------------------------ summary (decision rule: PROTOCOL.md §6)

fn summary(out: &Path) {
    use serde_json::Value;
    let text = fs::read_to_string(out.join("rows.jsonl")).expect("rows.jsonl");
    let rows: Vec<Value> = text.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).unwrap()).collect();
    let get = |cell: &str, model: &str, arm: &str| rows.iter().rev().find(|r| r["cell"] == cell && r["model"] == model && r["arm"] == arm);
    let ci = |r: &Value, k: usize| r["wilson95"][k].as_f64().unwrap();
    let mut cells_out = Vec::new();
    // positive controls per cipher
    let pos_ok = |cipher: &str| CELLS.iter().filter(|c| c.cipher == cipher && c.role == "positive_control")
        .all(|c| get(c.name, "mlp", "primary").map(|r| r["accuracy"].as_f64().unwrap() > 0.9).unwrap_or(false));
    for c in CELLS {
        let Some(mlp) = get(c.name, "mlp", "primary") else { continue };
        let neg = get(c.name, "mlp", "label_shuffle_control").unwrap();
        let aa = get(c.name, "mlp", "aa_seed2").unwrap();
        let neg_ok = ci(neg, 0) <= 0.5 && ci(neg, 1) >= 0.5;
        let valid = neg_ok && pos_ok(c.cipher);
        let mut models = Vec::new();
        for m in ["mlp", "logreg", "difference_table_nb"] {
            let r = get(c.name, m, "primary").unwrap();
            let blo = r["bonferroni_lower"].as_f64().unwrap();
            let verdict = if !valid { "invalid (control failed)".to_string() }
                else if blo > 0.5 { "distinguishes".into() } else { "no distinguisher found at this size".into() };
            let beats = if m == "mlp" { Value::Null } else { Value::Bool(ci(mlp, 0) > ci(r, 1)) };
            models.push(serde_json::json!({"model": m, "accuracy": r["accuracy"], "wilson95": r["wilson95"],
                "bonferroni_lower": blo, "advantage": r["accuracy"].as_f64().unwrap() - 0.5, "verdict": verdict, "mlp_beats_this_baseline": beats}));
        }
        cells_out.push(serde_json::json!({"cell": c.name, "cipher": c.cipher, "rounds": c.rounds, "role": c.role,
            "negative_control": {"accuracy": neg["accuracy"], "wilson95": neg["wilson95"], "at_chance": neg_ok},
            "aa_seed_difference": (aa["accuracy"].as_f64().unwrap() - mlp["accuracy"].as_f64().unwrap()),
            "positive_controls_detected": pos_ok(c.cipher), "valid": valid, "models": models}));
    }
    let s = serde_json::json!({"schema": "neural-distinguishers.summary/v1", "decision_rule": "PROTOCOL.md §6",
        "bonferroni": {"family": 21, "alpha": 0.01, "z": Z_BONF}, "cells": cells_out});
    fs::write(out.join("summary.json"), serde_json::to_string_pretty(&s).unwrap() + "\n").unwrap();
    fs::write(out.join("fig_accuracy.svg"), accuracy_svg(&s)).unwrap();
    println!("summary.json and fig_accuracy.svg written ({} cells)", s["cells"].as_array().unwrap().len());
}

/// Small multiples (one panel per cipher): held-out accuracy by round count, one series per model, 95% Wilson bars.
fn accuracy_svg(s: &serde_json::Value) -> String {
    let models = [("mlp", "MLP", "#2a78d6"), ("logreg", "logistic regression", "#eb6834"), ("difference_table_nb", "difference-table naive Bayes", "#1baf7a")];
    let (pw, ph, l, t, b) = (420.0, 260.0, 48.0, 34.0, 40.0);
    let mut svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" font-family=\"system-ui,sans-serif\" font-size=\"11\">\n<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\n", 2.0 * pw + 20.0, ph + 46.0);
    for (pi, (cipher, title)) in [("speck32_64", "Speck32/64"), ("aes128", "AES-128 (reduced rounds)")].iter().enumerate() {
        let ox = pi as f64 * (pw + 20.0);
        let cells: Vec<&serde_json::Value> = s["cells"].as_array().unwrap().iter().filter(|c| c["cipher"] == *cipher).collect();
        if cells.is_empty() { continue; }
        let rs: Vec<f64> = cells.iter().map(|c| c["rounds"].as_f64().unwrap()).collect();
        let (r0, r1) = (rs.iter().cloned().fold(f64::MAX, f64::min) - 0.5, rs.iter().cloned().fold(f64::MIN, f64::max) + 0.5);
        let x = |r: f64| ox + l + (r - r0) / (r1 - r0) * (pw - l - 16.0);
        let y = |a: f64| t + (1.0 - (a - 0.45) / 0.55) * (ph - t - b);
        svg += &format!("<text x=\"{:.1}\" y=\"20\" font-size=\"13\" fill=\"#1b1f24\">{title}: held-out accuracy (95% Wilson)</text>\n", ox + l);
        for a in [0.5, 0.6, 0.7, 0.8, 0.9, 1.0] {
            svg += &format!("<line x1=\"{:.1}\" x2=\"{:.1}\" y1=\"{2:.1}\" y2=\"{2:.1}\" stroke=\"{3}\" {4}/><text x=\"{5:.1}\" y=\"{6:.1}\" text-anchor=\"end\" fill=\"#5c6773\">{a:.1}</text>\n",
                x(r0), x(r1), y(a), if a == 0.5 { "#8a939d" } else { "#eceff3" }, if a == 0.5 { "stroke-dasharray=\"4 3\"" } else { "" }, ox + l - 6.0, y(a) + 4.0);
        }
        svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#5c6773\">chance</text>\n", x(r1) - 40.0, y(0.5) - 4.0);
        for c in &cells {
            let r = c["rounds"].as_f64().unwrap();
            svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"#5c6773\">{}{}</text>\n", x(r), ph - b + 16.0, r, if c["role"] == "positive_control" { " (pos. ctrl)" } else { "" });
        }
        for (mi, (m, label, col)) in models.iter().enumerate() {
            let mut pts = Vec::new();
            for c in &cells {
                let mm = c["models"].as_array().unwrap().iter().find(|v| v["model"] == *m).unwrap();
                let (a, lo, hi) = (mm["accuracy"].as_f64().unwrap().max(0.45), mm["wilson95"][0].as_f64().unwrap().max(0.45), mm["wilson95"][1].as_f64().unwrap().max(0.45));
                let xr = x(c["rounds"].as_f64().unwrap()) + (mi as f64 - 1.0) * 6.0;
                svg += &format!("<line x1=\"{xr:.1}\" x2=\"{xr:.1}\" y1=\"{:.1}\" y2=\"{:.1}\" stroke=\"{col}\" stroke-width=\"2\"/><circle cx=\"{xr:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{col}\"><title>{} r{}: {label} {:.4} [{:.4}, {:.4}] — {}</title></circle>\n",
                    y(lo), y(hi), y(a), c["cell"].as_str().unwrap(), c["rounds"], a, lo, hi, mm["verdict"].as_str().unwrap());
                pts.push(format!("{xr:.1},{:.1}", y(a)));
            }
            svg += &format!("<polyline fill=\"none\" stroke=\"{col}\" stroke-width=\"2\" points=\"{}\"/>\n", pts.join(" "));
        }
        svg += &format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"#5c6773\">rounds</text>\n", ox + pw / 2.0, ph - 6.0);
    }
    let mut lx = 48.0;
    for (_, label, col) in models {
        svg += &format!("<circle cx=\"{lx:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{col}\"/><text x=\"{:.1}\" y=\"{:.1}\" fill=\"#5c6773\">{label}</text>\n", ph + 30.0, lx + 8.0, ph + 34.0);
        lx += 40.0 + 7.0 * label.len() as f64;
    }
    svg + "</svg>\n"
}
