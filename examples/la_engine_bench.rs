//! Linear-algebra engine benchmark for the index-calculus logarithm solve.
//!
//! Generates a deterministic synthetic relation system shaped like an
//! index-calculus relation matrix (each row is `m` factor-base summands with
//! coefficients ±1 or uniform residues, columns uniform, rows drawn until
//! every column is covered and the system has full column rank, plus a small
//! excess), plants a logarithm vector, and solves it with one engine:
//!
//! | arm | engine |
//! |:--|:--|
//! | `igauss` | `IncrementalGauss` (dense RREF, rows fed one at a time; ecbench default) |
//! | `sgauss` | `StructuredGauss` (sparse RREF, Markowitz pivot, rows fed one at a time) |
//! | `full-bw` | `solve_sparse_system`: filter (dup/singleton/clique/merge) + block Wiedemann + reconstruct |
//! | `full-bl` | the same with block Lanczos |
//! | `core-bw` / `core-bl` | the engine alone on the *core* (`export-core` below), filter disabled |
//!
//! `export-core` writes the singleton- and clique-filtered, *unmerged*,
//! square core (coefficients stay ±1/±2) in the CADO-NFS `bwc` binary format
//! (`matrix.bin`, `rhs.txt`), so CADO's block Wiedemann runs on exactly the
//! matrix `core-bw` sees.  `check-cado` verifies a CADO solution file
//! against that core and against the planted logarithms.
//!
//! Every arm checks its answer against the planted vector; a run that does
//! not reproduce it is reported `verified: false`.  One JSON line per run.
//!
//! ```text
//! cargo run --release --example la_engine_bench -- ours --arm full-bw --cols 16384 --weight 3
//! cargo run --release --example la_engine_bench -- export-core --cols 16384 --weight 3 --dir /tmp/c
//! cargo run --release --example la_engine_bench -- check-cado --cols 16384 --weight 3 --dir /tmp/c --sol /tmp/c/wdir/K.sols0-1.0
//! ```
//!
//! ## Wide moduli (`--modulus-bits B`)
//!
//! `ours --modulus-bits B` replaces `--modulus` by a fixed odd prime of
//! `B` bits (table below; primality is checked in-program by Miller–Rabin
//! over `BigUint`) and solves in the word width
//! `WordWidth::for_modulus` picks for it (`--word u128|u192|u256`
//! overrides, e.g. the 61-bit prime in `Wide<2>` to isolate limb cost):
//!
//! | `B` | prime | width |
//! |:--|:--|:--|
//! | 61 | `2^61 − 1` | `u64` (the control: exactly the `--modulus 2305843009213693951` run) |
//! | 89 | `2^89 − 1` | `U128` |
//! | 127 | `2^127 − 1` | `U128` |
//! | 129 | ECC2K-130 subgroup order `r` (`log2 r ≈ 129.0`, bit length 130) | `U192` |
//! | 190 | `2^190 − 11` | `U192` |
//! | 255 | `2^255 − 19` | `U256` |
//!
//! The wide generator draws its column and sign stream exactly as the
//! 61-bit control does for the same seed (it advances the stream by the
//! control's planted-value draws), and takes planted logarithms and
//! `--coeffs wide` values from a second stream seeded `seed ^
//! WIDE_VALUE_STREAM`, uniform in `[1, r)`.  With the default ±1
//! coefficients every width therefore solves the same matrix pattern;
//! only the modulus changes.  Wide widths run the `full-bw` and `full-bl`
//! arms.  The record adds `modulus_bits`, `modulus_bit_length`,
//! `modulus_name`, `primality`, `word_width` and the solver's counted work
//! (`row_ops`, `krylov_products`, `krylov_lanes`, `core_dimension`,
//! `core_nonzeros`).  Without `--modulus-bits` the output is unchanged.
//!
//! ```text
//! cargo run --release --example la_engine_bench -- ours --arm full-bw --cols 1024 --weight 3 --modulus-bits 129
//! ```

use crypto_lib::cryptanalysis::ic_boundary::RowStatus;
use crypto_lib::cryptanalysis::ic_framework::linalg::matrix_by_name;
use crypto_lib::cryptanalysis::{
    filter_relations, solve_sparse_system, FilterOptions, FilterStrategy, ModWord,
    SparseCoreSolver, SparseRow, SparseSolveOptions, SparseSolveOutcome, SparseSolveReport, Wide,
    WordWidth,
};
use num_bigint::{BigUint, RandBigInt};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::time::Instant;

/// 2^62 − 57, prime; fits the `u64` word path and CADO's one-limb `p1` layer.
const DEFAULT_P: u64 = 4_611_686_018_427_387_847;

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
fn addmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + b as u128) % m as u128) as u64
}

#[derive(Clone, Debug)]
struct Params {
    cols: usize,
    weight: usize,
    excess: usize,
    seed: u64,
    p: u64,
    wide: bool,
    /// All coefficients +1 (a repeated column gives +2): the shape CADO's
    /// relation format can carry (nonnegative valuations).  Draws the same
    /// random stream as `unit`, so the column pattern is identical.
    positive: bool,
    block: usize,
    /// `--merge-max-weight` / `--max-row-weight`: override the filter's merge
    /// thresholds (defaults 8 and 32) for the `full-*` arms.
    merge_max_weight: Option<usize>,
    max_row_weight: Option<usize>,
    /// `--filter-strategy cado` and `--target-density N` (0: stop on the
    /// Krylov cost) select the CADO-style filter.
    markowitz: bool,
    target_density: usize,
    /// `--filter-preset cado`: exactly `FilterOptions::cado()` (what `ic logs`
    /// runs by default); later flags may still override single fields.
    cado_preset: bool,
}

struct System {
    rows: Vec<SparseRow<u64>>,
    planted: Vec<u64>,
    /// Rows drawn before every column was covered (coupon-collector point).
    rows_to_cover: usize,
}

/// Draw rows until every column occurs, then `excess` more.  Full column
/// rank is not forced here; every arm reports it if the rows fall short.
fn generate(p: &Params) -> System {
    let mut rng = StdRng::seed_from_u64(p.seed);
    let planted: Vec<u64> = (0..p.cols).map(|_| rng.gen_range(1..p.p)).collect();
    let mut covered = vec![false; p.cols];
    let mut uncovered = p.cols;
    let mut rows = Vec::new();
    let mut rows_to_cover = 0;
    let mut extra = 0usize;
    while extra < p.excess {
        let mut entries = Vec::with_capacity(p.weight);
        for _ in 0..p.weight {
            let c = rng.gen_range(0..p.cols) as u32;
            let v = if p.wide {
                rng.gen_range(1..p.p)
            } else if rng.gen::<bool>() || p.positive {
                1
            } else {
                p.p - 1
            };
            entries.push((c, v));
        }
        let row = SparseRow::new(entries, 0, p.p);
        if row.entries.is_empty() {
            continue;
        }
        let mut rhs = 0u64;
        for &(c, v) in &row.entries {
            rhs = addmod(rhs, mulmod(v, planted[c as usize], p.p), p.p);
            if !covered[c as usize] {
                covered[c as usize] = true;
                uncovered -= 1;
            }
        }
        rows.push(SparseRow {
            entries: row.entries,
            rhs,
        });
        if uncovered == 0 {
            if rows_to_cover == 0 {
                rows_to_cover = rows.len();
            }
            extra += 1;
        }
    }
    System {
        rows,
        planted,
        rows_to_cover,
    }
}

/// Singleton + clique filtering only (no merge), trimmed to square, so the
/// coefficients stay small integers that CADO's format can carry.
fn unmerged_core(p: &Params, sys: &System) -> (Vec<SparseRow<u64>>, Vec<u32>) {
    let opts = FilterOptions {
        remove_duplicates: true,
        remove_singletons: true,
        target_excess: 0,
        merge_max_weight: 0,
        max_row_weight: usize::MAX,
        ..FilterOptions::default()
    };
    let f = filter_relations(sys.rows.clone(), p.cols, p.p, &opts).expect("consistent");
    let index: HashMap<u32, u32> = f
        .core_columns
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i as u32))
        .collect();
    let core = f
        .core_rows
        .iter()
        .map(|r| SparseRow {
            entries: r.entries.iter().map(|&(c, v)| (index[&c], v)).collect(),
            rhs: r.rhs,
        })
        .collect();
    (core, f.core_columns.clone())
}

fn sparse_opts(p: &Params, solver: SparseCoreSolver, filter: bool) -> SparseSolveOptions {
    let mut o = SparseSolveOptions {
        solver,
        ..SparseSolveOptions::default()
    };
    o.wiedemann.block_m = p.block;
    o.wiedemann.block_n = p.block;
    o.lanczos.block_size = p.block;
    if let Some(w) = p.merge_max_weight {
        o.filter.merge_max_weight = w;
    }
    if let Some(w) = p.max_row_weight {
        o.filter.max_row_weight = w;
    }
    if p.cado_preset {
        o.filter = FilterOptions::cado();
    }
    if p.markowitz {
        o.filter.strategy = FilterStrategy::Cado;
        o.filter.target_density = p.target_density;
    }
    if !filter {
        o.filter = FilterOptions {
            remove_duplicates: false,
            remove_singletons: false,
            target_excess: usize::MAX,
            merge_max_weight: 0,
            max_row_weight: usize::MAX,
            ..FilterOptions::default()
        };
    }
    o
}

fn threads() -> usize {
    rayon::current_num_threads()
}

fn run_ours(p: &Params, arm: &str) -> serde_json::Value {
    let t0 = Instant::now();
    let sys = generate(p);
    let gen_s = t0.elapsed().as_secs_f64();
    let nnz: usize = sys.rows.iter().map(|r| r.entries.len()).sum();
    let mut out = serde_json::json!({
        "arm": arm, "cols": p.cols, "weight": p.weight, "excess": p.excess,
        "seed": p.seed, "modulus": p.p.to_string(), "coeffs": coeffs_name(p),
        "block": p.block, "threads": threads(),
        "merge_max_weight": p.merge_max_weight, "max_row_weight": p.max_row_weight,
        "filter_strategy": if p.markowitz || p.cado_preset {"cado"} else {"legacy"}, "filter_preset": p.cado_preset, "target_density": p.target_density,
        "rows": sys.rows.len(), "rows_to_cover": sys.rows_to_cover, "nonzeros": nnz,
        "generate_s": gen_s,
    });
    let o = out.as_object_mut().unwrap();
    match arm {
        "igauss" | "sgauss" => {
            let name = if arm == "igauss" {
                "incremental-gauss"
            } else {
                "structured-gauss"
            };
            let mut m = matrix_by_name(name, p.cols, p.p).unwrap();
            let t = Instant::now();
            let mut fed = 0usize;
            for row in &sys.rows {
                let mut dense = vec![0u64; p.cols];
                for &(c, v) in &row.entries {
                    dense[c as usize] = v;
                }
                fed += 1;
                if m.add_row(dense, row.rhs) == RowStatus::Inconsistent {
                    break;
                }
                if m.rank() == p.cols {
                    break;
                }
            }
            let solve_s = t.elapsed().as_secs_f64();
            let solved: Vec<Option<u64>> = (0..p.cols).map(|c| m.pinned(c)).collect();
            let ok =
                m.rank() == p.cols && solved.iter().zip(&sys.planted).all(|(s, &x)| *s == Some(x));
            let (work, unit) = m.work();
            o.insert("rows_fed".into(), fed.into());
            o.insert("rank".into(), m.rank().into());
            o.insert("work".into(), work.into());
            o.insert("work_unit".into(), unit.into());
            o.insert("solve_s".into(), solve_s.into());
            o.insert("verified".into(), ok.into());
        }
        "full-bw" | "full-bl" => {
            let solver = if arm == "full-bw" {
                SparseCoreSolver::BlockWiedemann
            } else {
                SparseCoreSolver::BlockLanczos
            };
            let opts = sparse_opts(p, solver, true);
            let tf = Instant::now();
            let f = filter_relations(sys.rows.clone(), p.cols, p.p, &opts.filter);
            let filter_s = tf.elapsed().as_secs_f64();
            drop(f);
            let t = Instant::now();
            let (outcome, report) = solve_sparse_system(&sys.rows, p.cols, p.p, &opts);
            let solve_s = t.elapsed().as_secs_f64();
            let ok = matches!(&outcome, SparseSolveOutcome::Solved(x) if *x == sys.planted);
            o.insert("filter_s".into(), filter_s.into());
            o.insert("solve_s".into(), solve_s.into());
            o.insert(
                "core_s_derived".into(),
                (solve_s - filter_s).max(0.0).into(),
            );
            o.insert("report".into(), serde_json::to_value(&report).unwrap());
            o.insert("verified".into(), ok.into());
        }
        "core-bw" | "core-bl" => {
            let solver = if arm == "core-bw" {
                SparseCoreSolver::BlockWiedemann
            } else {
                SparseCoreSolver::BlockLanczos
            };
            let tf = Instant::now();
            let (core, core_columns) = unmerged_core(p, &sys);
            let filter_s = tf.elapsed().as_secs_f64();
            let n = core_columns.len();
            let core_nnz: usize = core.iter().map(|r| r.entries.len()).sum();
            let opts = sparse_opts(p, solver, false);
            let t = Instant::now();
            let (outcome, report) = solve_sparse_system(&core, n, p.p, &opts);
            let solve_s = t.elapsed().as_secs_f64();
            let ok = matches!(&outcome, SparseSolveOutcome::Solved(x)
                if x.iter().zip(&core_columns).all(|(&v, &c)| v == sys.planted[c as usize]));
            o.insert("core_rows".into(), core.len().into());
            o.insert("core_cols".into(), n.into());
            o.insert("core_nonzeros".into(), core_nnz.into());
            o.insert("filter_s".into(), filter_s.into());
            o.insert("solve_s".into(), solve_s.into());
            o.insert("report".into(), serde_json::to_value(&report).unwrap());
            o.insert("verified".into(), ok.into());
        }
        other => panic!("unknown arm {other}"),
    }
    out
}

fn u32le(w: &mut impl Write, v: u32) {
    w.write_all(&v.to_le_bytes()).unwrap();
}

/// CADO `bwc` GF(p) input: per row `len`, then `len` × (`col`, `i32 coeff`),
/// all 32-bit little endian; rhs as ASCII (below).
fn export_core(p: &Params, dir: &str) -> serde_json::Value {
    std::fs::create_dir_all(dir).unwrap();
    let sys = generate(p);
    let (core, cols) = unmerged_core(p, &sys);
    assert_eq!(core.len(), cols.len(), "core must be square");
    let mut w =
        std::io::BufWriter::new(std::fs::File::create(format!("{dir}/matrix.bin")).unwrap());
    let half = p.p / 2;
    for r in &core {
        u32le(&mut w, r.entries.len() as u32);
        for &(c, v) in &r.entries {
            let s: i64 = if v > half {
                -((p.p - v) as i64)
            } else {
                v as i64
            };
            assert!(s.abs() < (1 << 31), "coefficient too large for CADO int32");
            u32le(&mut w, c);
            w.write_all(&(s as i32).to_le_bytes()).unwrap();
        }
    }
    drop(w);
    // `bwc.pl` takes the right-hand side as ASCII: `rows nrhs p`, then one
    // decimal residue per row.
    let mut w = std::io::BufWriter::new(std::fs::File::create(format!("{dir}/rhs.txt")).unwrap());
    writeln!(w, "{} 1 {}", core.len(), p.p).unwrap();
    for r in &core {
        writeln!(w, "{}", r.rhs).unwrap();
    }
    drop(w);
    let nnz: usize = core.iter().map(|r| r.entries.len()).sum();
    serde_json::json!({"cols": p.cols, "weight": p.weight, "seed": p.seed,
        "core_dim": cols.len(), "core_nonzeros": nnz, "modulus": p.p.to_string(),
        "dir": dir})
}

/// Read a CADO solution: `core_dim` + `nrhs` residues of two LE words each
/// (`K.sols*` holds `x || y` with `(M || RHS)(x; y) = 0`, so `x / (−y)`
/// solves `M x = RHS`).  Some builds pad `x` to a multiple of the block
/// size; the trailing `y` words are always last.
fn check_cado(p: &Params, dir: &str, sol: &str) -> serde_json::Value {
    let sys = generate(p);
    let (core, cols) = unmerged_core(p, &sys);
    let n = cols.len();
    let mut bytes = Vec::new();
    std::fs::File::open(sol)
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();
    let words: Vec<u64> = bytes
        .chunks_exact(8)
        .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
        .collect();
    let y = *words.last().unwrap() % p.p;
    let mut result = serde_json::json!({"dir": dir, "sol": sol, "core_dim": n,
        "words": words.len()});
    let o = result.as_object_mut().unwrap();
    if y == 0 || words.len() < n + 1 {
        o.insert("verified".into(), false.into());
        o.insert("reason".into(), "zero rhs coordinate or short file".into());
        return result;
    }
    // x / (−y)
    let inv = modinv(p.p - y, p.p);
    let x: Vec<u64> = words[..n]
        .iter()
        .map(|&v| mulmod(v % p.p, inv, p.p))
        .collect();
    let rows_ok = core.iter().all(|r| {
        let mut s = 0u64;
        for &(c, v) in &r.entries {
            s = addmod(s, mulmod(v, x[c as usize], p.p), p.p);
        }
        s == r.rhs
    });
    let planted_ok = x
        .iter()
        .zip(&cols)
        .all(|(&v, &c)| v == sys.planted[c as usize]);
    o.insert("rows_ok".into(), rows_ok.into());
    o.insert("verified".into(), (rows_ok && planted_ok).into());
    result
}

fn coeffs_name(p: &Params) -> &'static str {
    if p.wide {
        "wide"
    } else if p.positive {
        "positive"
    } else {
        "unit"
    }
}

// ── Wide moduli: `--modulus-bits B` ─────────────────────────────────

/// `2^61 − 1`: the 61-bit control's modulus; its planted-value draws are
/// the ones the wide generator replays to keep the column stream aligned.
const P61: u64 = (1 << 61) - 1;

/// Seed offset of the wide generator's value stream (planted logarithms
/// and `--coeffs wide` values).
const WIDE_VALUE_STREAM: u64 = 0x5749_4445_5641_4c53;

/// The ECC2K-130 subgroup order, as in `koblitz_sparse_la`'s wide tests.
const ECC2K130_ORDER: &str = "680564733841876926932320129493409985129";

/// The fixed prime for `--modulus-bits B` and its name.
fn wide_prime(bits: u32) -> (BigUint, &'static str) {
    let one = BigUint::from(1u32);
    match bits {
        61 => ((&one << 61u32) - 1u32, "2^61 - 1"),
        89 => ((&one << 89u32) - 1u32, "2^89 - 1"),
        127 => ((&one << 127u32) - 1u32, "2^127 - 1"),
        129 => (
            ECC2K130_ORDER.parse().unwrap(),
            "ECC2K-130 subgroup order (log2 r = 129.0, bit length 130)",
        ),
        190 => ((&one << 190u32) - 11u32, "2^190 - 11"),
        255 => ((&one << 255u32) - 19u32, "2^255 - 19"),
        other => {
            eprintln!("--modulus-bits must be one of 61 89 127 129 190 255, not {other}");
            std::process::exit(2);
        }
    }
}

/// Miller–Rabin with the fixed bases 2, 3, …, 71 (the first 20 primes).
/// The bases up to 41 are a proof for `n < 3 317 044 064 679 887 385 961 981`
/// (Sorenson–Webster); above that a pass is a strong probable prime to
/// all 20 bases.  Returns `None` for a composite.
fn miller_rabin(n: &BigUint) -> Option<&'static str> {
    const BASES: [u32; 20] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];
    let one = BigUint::from(1u32);
    let two = BigUint::from(2u32);
    if *n < two {
        return None;
    }
    for &b in &BASES {
        if *n == BigUint::from(b) {
            return Some("proven (small prime)");
        }
        if (n % b) == BigUint::from(0u32) {
            return None;
        }
    }
    let n1 = n - &one;
    let s = n1.trailing_zeros().unwrap();
    let d = &n1 >> s;
    'bases: for &b in &BASES {
        let mut x = BigUint::from(b).modpow(&d, n);
        if x == one || x == n1 {
            continue;
        }
        for _ in 1..s {
            x = x.modpow(&two, n);
            if x == n1 {
                continue 'bases;
            }
        }
        return None;
    }
    let bound: BigUint = "3317044064679887385961981".parse().unwrap();
    Some(if *n < bound {
        "proven (Miller-Rabin bases 2..41, n < 3.317e24)"
    } else {
        "strong probable prime to the 20 fixed bases 2..71"
    })
}

struct WideSystem<W: ModWord> {
    rows: Vec<SparseRow<W>>,
    planted: Vec<W>,
    rows_to_cover: usize,
}

/// `generate` in width `W`: the same row shape and column/sign stream as
/// the 61-bit control for the same seed, values reduced mod `r`.
fn generate_wide<W: ModWord>(p: &Params, r: &BigUint, m: &W::Modulus) -> WideSystem<W> {
    let mut rng = StdRng::seed_from_u64(p.seed);
    for _ in 0..p.cols {
        let _: u64 = rng.gen_range(1..P61);
    }
    let mut vrng = StdRng::seed_from_u64(p.seed ^ WIDE_VALUE_STREAM);
    let one = BigUint::from(1u32);
    let planted: Vec<W> = (0..p.cols)
        .map(|_| W::from_biguint(&vrng.gen_biguint_range(&one, r), m))
        .collect();
    let mut covered = vec![false; p.cols];
    let mut uncovered = p.cols;
    let mut rows = Vec::new();
    let mut rows_to_cover = 0;
    let mut extra = 0usize;
    while extra < p.excess {
        let mut entries = Vec::with_capacity(p.weight);
        for _ in 0..p.weight {
            let c = rng.gen_range(0..p.cols) as u32;
            let v = if p.wide {
                let _: u64 = rng.gen_range(1..P61);
                W::from_biguint(&vrng.gen_biguint_range(&one, r), m)
            } else if rng.gen::<bool>() || p.positive {
                W::one()
            } else {
                W::max_residue(m)
            };
            entries.push((c, v));
        }
        let row = SparseRow::new(entries, W::zero(), *m);
        if row.entries.is_empty() {
            continue;
        }
        let mut rhs = W::zero();
        for &(c, v) in &row.entries {
            rhs = rhs.add(v.mul(planted[c as usize], m), m);
            if !covered[c as usize] {
                covered[c as usize] = true;
                uncovered -= 1;
            }
        }
        rows.push(SparseRow {
            entries: row.entries,
            rhs,
        });
        if uncovered == 0 {
            if rows_to_cover == 0 {
                rows_to_cover = rows.len();
            }
            extra += 1;
        }
    }
    WideSystem {
        rows,
        planted,
        rows_to_cover,
    }
}

/// The `full-bw` / `full-bl` arms in width `W`; the record has the keys of
/// `run_ours`'s `full-*` record.
fn run_wide<W: ModWord>(p: &Params, arm: &str, r: &BigUint) -> serde_json::Value {
    let solver = match arm {
        "full-bw" => SparseCoreSolver::BlockWiedemann,
        "full-bl" => SparseCoreSolver::BlockLanczos,
        other => {
            eprintln!("--modulus-bits in a wide word runs full-bw or full-bl, not {other}");
            std::process::exit(2);
        }
    };
    let Some(m) = W::modulus_from_biguint(r) else {
        eprintln!("the modulus does not fit word width {:?}", W::WIDTH);
        std::process::exit(2);
    };
    let t0 = Instant::now();
    let sys = generate_wide::<W>(p, r, &m);
    let gen_s = t0.elapsed().as_secs_f64();
    let nnz: usize = sys.rows.iter().map(|r| r.entries.len()).sum();
    let mut out = serde_json::json!({
        "arm": arm, "cols": p.cols, "weight": p.weight, "excess": p.excess,
        "seed": p.seed, "modulus": r.to_string(), "coeffs": coeffs_name(p),
        "block": p.block, "threads": threads(),
        "merge_max_weight": p.merge_max_weight, "max_row_weight": p.max_row_weight,
        "filter_strategy": if p.markowitz || p.cado_preset {"cado"} else {"legacy"}, "filter_preset": p.cado_preset, "target_density": p.target_density,
        "rows": sys.rows.len(), "rows_to_cover": sys.rows_to_cover, "nonzeros": nnz,
        "generate_s": gen_s,
    });
    let o = out.as_object_mut().unwrap();
    let opts = sparse_opts(p, solver, true);
    let tf = Instant::now();
    let f = filter_relations(sys.rows.clone(), p.cols, m, &opts.filter);
    let filter_s = tf.elapsed().as_secs_f64();
    drop(f);
    let t = Instant::now();
    let (outcome, report) = solve_sparse_system(&sys.rows, p.cols, m, &opts);
    let solve_s = t.elapsed().as_secs_f64();
    let ok = matches!(&outcome, SparseSolveOutcome::Solved(x) if *x == sys.planted);
    o.insert("filter_s".into(), filter_s.into());
    o.insert("solve_s".into(), solve_s.into());
    o.insert(
        "core_s_derived".into(),
        (solve_s - filter_s).max(0.0).into(),
    );
    o.insert("report".into(), serde_json::to_value(&report).unwrap());
    o.insert("verified".into(), ok.into());
    out
}

/// The counted work of a `full-*` / `core-*` record, lifted to top level.
fn counted_work(report: &SparseSolveReport) -> serde_json::Value {
    let (products, lanes) = match (&report.wiedemann, &report.lanczos) {
        (_, Some(l)) => (l.sparse_products, l.block_size),
        (Some(w), None) => (w.products, w.block_n),
        (None, None) => (0, 0),
    };
    serde_json::json!({
        "row_ops": report.row_ops, "krylov_products": products, "krylov_lanes": lanes,
        "core_dimension": report.core_dimension, "core_nonzeros": report.core_nonzeros,
        "filter_row_ops": report.filter.row_ops,
    })
}

/// `ours --modulus-bits B`: the 61-bit `u64` control runs `run_ours`
/// unchanged; every other width runs `run_wide`.
fn run_modulus_bits(p: &Params, arm: &str, bits: u32, word: Option<&str>) -> serde_json::Value {
    let (r, name) = wide_prime(bits);
    let Some(primality) = miller_rabin(&r) else {
        panic!("{name} failed Miller-Rabin");
    };
    let width = match word {
        None => WordWidth::for_modulus(&r).expect("supported modulus"),
        Some("u64") => WordWidth::U64,
        Some("u128") => WordWidth::U128,
        Some("u192") => WordWidth::U192,
        Some("u256") => WordWidth::U256,
        Some(other) => {
            eprintln!("--word must be u64, u128, u192 or u256, not {other}");
            std::process::exit(2);
        }
    };
    let mut out = match width {
        WordWidth::U64 => {
            let Some(m) = <u64 as ModWord>::modulus_from_biguint(&r) else {
                eprintln!("{name} does not fit the u64 word");
                std::process::exit(2);
            };
            run_ours(&Params { p: m, ..p.clone() }, arm)
        }
        WordWidth::U128 => run_wide::<Wide<2>>(p, arm, &r),
        WordWidth::U192 => run_wide::<Wide<3>>(p, arm, &r),
        WordWidth::U256 => run_wide::<Wide<4>>(p, arm, &r),
    };
    let o = out.as_object_mut().unwrap();
    let work = o
        .get("report")
        .map(|v| counted_work(&serde_json::from_value(v.clone()).unwrap()));
    o.insert("modulus_bits".into(), bits.into());
    o.insert("modulus_bit_length".into(), r.bits().into());
    o.insert("modulus_name".into(), name.into());
    o.insert("primality".into(), primality.into());
    o.insert("word_width".into(), serde_json::to_value(width).unwrap());
    o.insert(
        "generator".into(),
        if width == WordWidth::U64 {
            "u64-control"
        } else {
            "wide-v1"
        }
        .into(),
    );
    if let Some(serde_json::Value::Object(w)) = work {
        o.extend(w);
    }
    out
}

// ── CADO's own filter: purge → merge-dl → replay-dl → bwc ─────────────

/// Relations in CADO's indexed format, `a,b:i1,…,ik` (hex), an index
/// repeated once per unit of its (nonnegative) exponent.  `a` is the row
/// number, so the purged file maps back to the original rows.
fn cado_export_rels(p: &Params, dir: &str) -> serde_json::Value {
    assert!(p.positive, "CADO relations need --coeffs positive");
    std::fs::create_dir_all(dir).unwrap();
    let sys = generate(p);
    let mut w = std::io::BufWriter::new(std::fs::File::create(format!("{dir}/rels")).unwrap());
    for (i, r) in sys.rows.iter().enumerate() {
        let mut ids = Vec::new();
        for &(c, v) in &r.entries {
            assert!(v <= 64, "positive coefficients stay small");
            for _ in 0..v {
                ids.push(format!("{c:x}"));
            }
        }
        writeln!(w, "{i:x},1:{}", ids.join(",")).unwrap();
    }
    serde_json::json!({"rows": sys.rows.len(), "cols": p.cols, "dir": dir})
}

/// Original row of each line of a purged file (its `a`, in hex).
fn purged_rows(path: &str) -> Vec<usize> {
    let text = std::fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| usize::from_str_radix(l.split(',').next().unwrap(), 16).unwrap())
        .collect()
}

/// Right-hand side of each row of the replayed matrix: a relation set
/// `n id:e …` (ids index the purged file, exponents signed) sums
/// `e · rhs(row)`.  Written as `bwc.pl`'s ASCII rhs.
fn cado_rhs(p: &Params, dir: &str) -> serde_json::Value {
    let t = Instant::now();
    let sys = generate(p);
    let gen_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let purged = purged_rows(&format!("{dir}/purged"));
    let index = std::fs::read_to_string(format!("{dir}/small.index")).unwrap();
    let mut lines = index.lines();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut w = std::io::BufWriter::new(std::fs::File::create(format!("{dir}/rhs.txt")).unwrap());
    writeln!(w, "{n} 1 {}", p.p).unwrap();
    for line in lines.take(n) {
        let mut acc = 0u64;
        for item in line.split_whitespace().skip(1) {
            let (id, e) = item.split_once(':').unwrap_or((item, "1"));
            let id = usize::from_str_radix(id, 16).unwrap();
            let e: i64 = e.parse().unwrap();
            let e = if e < 0 {
                p.p - ((-e) as u64 % p.p)
            } else {
                e as u64 % p.p
            };
            acc = addmod(acc, mulmod(e, sys.rows[purged[id]].rhs, p.p), p.p);
        }
        writeln!(w, "{acc}").unwrap();
    }
    serde_json::json!({"small_rows": n, "rhs_s": t.elapsed().as_secs_f64(), "generate_s": gen_s})
}

/// Logs of the replayed matrix's columns from a `K.sols` file (column `i`
/// is ideal `h` per `small.ideal`), then iterative propagation through
/// every original row with exactly one unknown — what CADO's
/// `reconstructlog-dl` does, minus the NFS-specific parts.
fn cado_reconstruct(p: &Params, dir: &str, sol: &str) -> serde_json::Value {
    let sys = generate(p);
    let t = Instant::now();
    let ideals = std::fs::read_to_string(format!("{dir}/small.ideal")).unwrap();
    let col_of: Vec<usize> = ideals
        .lines()
        // A one-token line is the header (column count), when present.
        .filter(|l| !l.starts_with('#') && l.split_whitespace().count() == 2)
        .map(|l| {
            let mut it = l.split_whitespace();
            let _i: usize = it.next().unwrap().parse().unwrap();
            usize::from_str_radix(it.next().unwrap(), 16).unwrap()
        })
        .collect();
    let mut bytes = Vec::new();
    std::fs::File::open(sol)
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();
    let words: Vec<u64> = bytes
        .chunks_exact(8)
        .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
        .collect();
    let k = col_of.len();
    let y = *words.last().unwrap() % p.p;
    let mut value: Vec<Option<u64>> = vec![None; p.cols];
    if y != 0 && words.len() > k {
        let inv = modinv(p.p - y, p.p);
        for (i, &c) in col_of.iter().enumerate() {
            value[c] = Some(mulmod(words[i] % p.p, inv, p.p));
        }
    }
    // Propagate: a row with one unknown determines it.
    let mut col_rows: Vec<Vec<u32>> = vec![Vec::new(); p.cols];
    for (i, r) in sys.rows.iter().enumerate() {
        for &(c, _) in &r.entries {
            col_rows[c as usize].push(i as u32);
        }
    }
    let mut queue: Vec<u32> = (0..sys.rows.len() as u32).collect();
    while let Some(i) = queue.pop() {
        let r = &sys.rows[i as usize];
        let unknown: Vec<&(u32, u64)> = r
            .entries
            .iter()
            .filter(|(c, _)| value[*c as usize].is_none())
            .collect();
        if unknown.len() != 1 {
            continue;
        }
        let (c, v) = *unknown[0];
        let mut s = r.rhs;
        for &(c2, v2) in &r.entries {
            if let Some(x) = value[c2 as usize] {
                s = (s + p.p - mulmod(v2, x, p.p)) % p.p;
            }
        }
        value[c as usize] = Some(mulmod(s, modinv(v, p.p), p.p));
        queue.extend(col_rows[c as usize].iter().copied());
    }
    let reconstruct_s = t.elapsed().as_secs_f64();
    let known = value.iter().filter(|v| v.is_some()).count();
    let ok = value.iter().zip(&sys.planted).all(|(v, &x)| *v == Some(x));
    serde_json::json!({"small_cols": k, "determined": known, "cols": p.cols,
        "reconstruct_s": reconstruct_s, "verified": ok})
}

fn modinv(a: u64, m: u64) -> u64 {
    let (mut r0, mut r1) = (m as i128, a as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    ((t0 % m as i128 + m as i128) % m as i128) as u64
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().cloned().unwrap_or_default();
    let get = |k: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
    };
    let num = |k: &str, d: u64| get(k).map(|v| v.parse::<u64>().unwrap()).unwrap_or(d);
    let p = Params {
        cols: num("--cols", 4096) as usize,
        weight: num("--weight", 3) as usize,
        excess: num("--excess", 32) as usize,
        seed: num("--seed", 1),
        p: num("--modulus", DEFAULT_P),
        wide: get("--coeffs").as_deref() == Some("wide"),
        positive: get("--coeffs").as_deref() == Some("positive"),
        block: num("--block", 4) as usize,
        merge_max_weight: get("--merge-max-weight").map(|v| v.parse().unwrap()),
        max_row_weight: get("--max-row-weight").map(|v| v.parse().unwrap()),
        markowitz: get("--filter-strategy").as_deref() == Some("cado"),
        target_density: num("--target-density", 0) as usize,
        cado_preset: get("--filter-preset").as_deref() == Some("cado"),
    };
    let modulus_bits = get("--modulus-bits").map(|v| v.parse::<u32>().unwrap());
    if modulus_bits.is_some() && (get("--modulus").is_some() || cmd != "ours") {
        eprintln!("--modulus-bits applies to `ours` and replaces --modulus");
        std::process::exit(2);
    }
    let v = match cmd.as_str() {
        "ours" if modulus_bits.is_some() => run_modulus_bits(
            &p,
            &get("--arm").expect("--arm"),
            modulus_bits.unwrap(),
            get("--word").as_deref(),
        ),
        "ours" => run_ours(&p, &get("--arm").expect("--arm")),
        "export-core" => export_core(&p, &get("--dir").expect("--dir")),
        "check-cado" => check_cado(
            &p,
            &get("--dir").expect("--dir"),
            &get("--sol").expect("--sol"),
        ),
        "cado-export-rels" => cado_export_rels(&p, &get("--dir").expect("--dir")),
        "cado-rhs" => cado_rhs(&p, &get("--dir").expect("--dir")),
        "cado-reconstruct" => cado_reconstruct(
            &p,
            &get("--dir").expect("--dir"),
            &get("--sol").expect("--sol"),
        ),
        _ => {
            eprintln!("usage: la_engine_bench ours|export-core|check-cado|cado-export-rels|cado-rhs|cado-reconstruct [options]");
            std::process::exit(2);
        }
    };
    println!("{}", serde_json::to_string(&v).unwrap());
}
