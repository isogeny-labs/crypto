//! Blind-spot item 02: price the m-summand Weil-descent route on binary curves
//! at the deployed sizes under each measured solving-degree model.
//!
//! The cost model is the one `examples/descent_degree_cost.rs` evaluates at
//! n = 131 (RESEARCH_ECC2K130_DECOMPOSITION.md section 5.1, oracle priced per
//! target by the Macaulay column count), generalised in three ways and
//! nothing else:
//!
//! * the field degree `n` is a parameter (the committed tool fixes 131), and
//!   the `l` search runs over `1..=n` instead of `4..70`;
//! * the rho reference is computed from the curve's real subgroup order and its
//!   automorphism group (`2n` for a Koblitz curve, `2` for a random one):
//!   `rho = 1/2 * (log2(pi/2) + log2 r - log2 A)`, which reproduces the
//!   committed ECC2K-130 reference `2^60.8090`;
//! * the column-touch-per-group-operation constant `K` divides the oracle
//!   term only (touches become group operations), as the committed note says
//!   ("converting would lower every row with an oracle by log2 K").
//!   `K = 1` is the committed unit.
//!
//! Derived, not measured: every number here is a model evaluation.
//!
//! The file depends on nothing but `std`, so `rustc` builds it on its own (the
//! `crypto_lib` library does not compile at the branch base, and nothing here
//! needs it). The curve orders and cofactors below are copied from
//! `src/binary_ecc/curve.rs` (`certicom_challenge("ecc2k-130")` and the
//! `sect*` constructors); each is checked against `#E = h * r` near `2^n`, and
//! the ECC2K-130 reference is checked against the committed `2^60.8090`.
//!
//! ```sh
//! rustc --edition 2021 -C opt-level=3 -C codegen-units=1 \
//!     -o <bin> examples/blindspots_02_deployed_binary_pricing.rs
//! <bin> control --scale-ref <committed 20261016-scale table> --d4-ref <cost_d4 table> --out-dir <dir>
//! <bin> cells --out-dir <dir>
//! <bin> scan  --out-dir <dir>
//! ```
//!
//! Output files are write-once (`create_new`).

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::{Path, PathBuf};

/// The committed rho constant used by the n = 131 tables.
const RHO_COMMITTED: f64 = 60.81;
const M_LIST: [usize; 4] = [2, 3, 4, 5];
const W_LIST: [u32; 2] = [1, 2];
/// Touches per group operation. 1 and 16 and 256 are the frozen design's
/// 2^0, 2^4, 2^8; 10 is the AGENTS.md 5a weight 0.1 GAE per touch.
const K_LIST: [f64; 4] = [1.0, 10.0, 16.0, 256.0];
const N_CELLS: [usize; 6] = [131, 163, 233, 283, 409, 571];

// ---------------------------------------------------------------------------
// The model (formulas as in examples/descent_degree_cost.rs)
// ---------------------------------------------------------------------------

fn log2_add(a: f64, b: f64) -> f64 {
    a.max(b) + (1.0 + (-(a - b).abs()).exp2()).log2()
}

/// log2 C(N, <= d). The committed forward recurrence, rescaled by exact powers
/// of two so it cannot overflow at the deployed sizes; bit-identical to the
/// committed evaluation wherever that one is finite.
fn log2_monomials(n_vars: usize, d: u32) -> f64 {
    let down = 2f64.powi(-600);
    let (mut total, mut binom, mut scale) = (0f64, 1f64, 0f64);
    for i in 0..=d as usize {
        if i > n_vars {
            break;
        }
        total += binom;
        binom = binom * (n_vars - i) as f64 / (i + 1) as f64;
        if binom > 1e250 {
            binom *= down;
            total *= down;
            scale += 600.0;
        }
    }
    total.log2() + scale
}

fn log2_factorial(m: usize) -> f64 {
    (1..=m).map(|k| k as f64).product::<f64>().log2()
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Model {
    /// Free oracle: the committed floor.
    Floor,
    /// Constant Macaulay degree.
    Const(u32),
    /// The measured m = 4 fit `l + 4` (a design arm).
    LPlus4,
    /// The measured m = 3 fit `ceil(l/2) + 4` (supplementary arm).
    HalfL4,
    /// Semi-regular D_reg(N): control only.
    SemiRegular,
}

impl Model {
    fn label(self) -> String {
        match self {
            Model::Floor => "floor".into(),
            Model::Const(d) => format!("D={d}"),
            Model::LPlus4 => "D=l+4".into(),
            Model::HalfL4 => "D=ceil(l/2)+4".into(),
            Model::SemiRegular => "D=semi-regular".into(),
        }
    }
    /// Arms named by the frozen design; the rest are supplementary.
    fn is_design(self) -> bool {
        matches!(
            self,
            Model::Const(3) | Model::Const(4) | Model::Const(6) | Model::Const(7) | Model::LPlus4
        )
    }
}

const DESIGN_MODELS: [Model; 5] = [
    Model::Const(3),
    Model::Const(4),
    Model::Const(6),
    Model::Const(7),
    Model::LPlus4,
];

struct Cost {
    unknowns: usize,
    degree: u32,
    targets: f64,
    /// Oracle work per target, group operations (touches minus log2 K).
    oracle: f64,
    algebra: f64,
    total: f64,
    /// log2 words held at once (oracle matrix under the w convention, or the
    /// relation matrix).
    memory: f64,
}

/// Fixed-width two's-complement integers (768 bits), enough for the
/// semi-regular series at n = 131 (N <= 2*131 + 4*131 unknowns would need
/// more, so the control only calls it with N <= 560 bits).
#[derive(Clone, Copy, PartialEq)]
struct Wide([u64; 12]);

impl Wide {
    const ZERO: Wide = Wide([0; 12]);
    fn one() -> Wide {
        let mut w = Wide::ZERO;
        w.0[0] = 1;
        w
    }
    fn is_negative(&self) -> bool {
        self.0[11] >> 63 == 1
    }
    fn is_nonpositive(&self) -> bool {
        self.is_negative() || *self == Wide::ZERO
    }
    fn sub_assign(&mut self, o: &Wide) {
        let mut borrow = 0u64;
        for i in 0..12 {
            let (d1, b1) = self.0[i].overflowing_sub(o.0[i]);
            let (d2, b2) = d1.overflowing_sub(borrow);
            self.0[i] = d2;
            borrow = (b1 | b2) as u64;
        }
    }
    fn mul_small(&mut self, k: u64) {
        let mut carry = 0u128;
        for i in 0..12 {
            let v = self.0[i] as u128 * k as u128 + carry;
            self.0[i] = v as u64;
            carry = v >> 64;
        }
    }
    /// Exact division of a non-negative value.
    fn div_small(&mut self, k: u64) {
        let mut rem = 0u128;
        for i in (0..12).rev() {
            let cur = (rem << 64) | self.0[i] as u128;
            self.0[i] = (cur / k as u128) as u64;
            rem = cur % k as u128;
        }
        assert_eq!(rem, 0, "binomial recurrence must divide exactly");
    }
}

/// The semi-regular degree of regularity: the first non-positive coefficient
/// of `(1+z)^N / prod (1+z^d)` (the committed `semi_regular_dreg`).
fn semi_regular_dreg(n_vars: usize, degrees: &[u32]) -> Option<u32> {
    let cap = 2 * n_vars + 8;
    let mut c: Vec<Wide> = Vec::with_capacity(cap + 1);
    let mut binom = Wide::one();
    for i in 0..=cap {
        c.push(if i <= n_vars { binom } else { Wide::ZERO });
        if i < n_vars {
            binom.mul_small((n_vars - i) as u64);
            binom.div_small((i + 1) as u64);
        }
    }
    for &d in degrees {
        let d = d as usize;
        for i in d..=cap {
            let prev = c[i - d];
            c[i].sub_assign(&prev);
        }
    }
    c.iter().position(|x| x.is_nonpositive()).map(|i| i as u32)
}

struct SemiCache(HashMap<(usize, usize, usize), u32>);

impl SemiCache {
    fn dreg(&mut self, n: usize, m: usize, l: usize) -> u32 {
        *self.0.entry((n, m, l)).or_insert_with(|| {
            let degs: Vec<u32> = std::iter::repeat_n(3, (m - 2) * n)
                .chain(std::iter::repeat_n(2, n))
                .collect();
            let n_vars = (m - 2) * n + m * l;
            assert!(n_vars <= 560, "Wide holds N <= 560 bits");
            semi_regular_dreg(n_vars, &degs).expect("reference exists")
        })
    }
}

fn degree_of(model: Model, n: usize, m: usize, l: usize, semi: &mut SemiCache) -> u32 {
    match model {
        Model::Floor => 0,
        Model::Const(d) => d,
        Model::LPlus4 => l as u32 + 4,
        Model::HalfL4 => (l as u32).div_ceil(2) + 4,
        Model::SemiRegular => semi.dreg(n, m, l),
    }
}

fn cost(n: usize, m: usize, l: usize, degree: u32, w: u32, k: f64) -> Cost {
    let unknowns = (m - 2) * n + m * l;
    let per_relation = (log2_factorial(m) + n as f64 - (m * l) as f64).max(0.0);
    let targets = l as f64 + per_relation;
    let (oracle, mem_oracle) = if degree == 0 {
        (0.0, 0.0)
    } else {
        let touches = w as f64 * log2_monomials(unknowns, degree);
        (touches - k.log2(), touches)
    };
    let algebra = (m as f64).log2() + (2 * l) as f64;
    let mem_la = l as f64 + (m as f64).log2();
    Cost {
        unknowns,
        degree,
        targets,
        oracle,
        algebra,
        total: log2_add(targets + oracle, algebra),
        memory: mem_oracle.max(mem_la),
    }
}

/// First strict minimum over `lmin..=lmax`, as the committed tool.
fn best(
    n: usize,
    m: usize,
    model: Model,
    w: u32,
    k: f64,
    lmin: usize,
    lmax: usize,
    semi: &mut SemiCache,
) -> (usize, Cost) {
    let mut best: Option<(usize, Cost)> = None;
    for l in lmin..=lmax {
        let c = cost(n, m, l, degree_of(model, n, m, l, semi), w, k);
        if best.as_ref().is_none_or(|(_, b)| c.total < b.total) {
            best = Some((l, c));
        }
    }
    best.expect("a row")
}

// ---------------------------------------------------------------------------
// Cells: the deployed curves
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct Cell {
    family: &'static str,
    curve: String,
    n: usize,
    log2_r: f64,
    h: u32,
    autos: f64,
    rho: f64,
}

/// log2 of a hexadecimal integer (f64 accumulation; relative error ~1e-15).
fn hex_log2(hex: &str) -> f64 {
    let mut acc = 0f64;
    for ch in hex.chars() {
        acc = acc * 16.0 + ch.to_digit(16).expect("hex digit") as f64;
    }
    acc.log2()
}

fn rho_log2(log2_r: f64, autos: f64) -> f64 {
    0.5 * ((std::f64::consts::PI / 2.0).log2() + log2_r - autos.log2())
}

/// (name, n, cofactor h, subgroup order r in hex), copied from
/// `src/binary_ecc/curve.rs`.
const K1_CURVES: [(&str, usize, u32, &str); 6] = [
    ("ECC2K-130", 131, 4, "200000000000000004D4FDD5703A3F269"),
    ("sect163k1", 163, 2, "4000000000000000000020108A2E0CC0D99F8A5EF"),
    ("sect233k1", 233, 4, "8000000000000000000000000000069d5bb915bcd46efb1ad5f173abdf"),
    ("sect283k1", 283, 4, "1ffffffffffffffffffffffffffffffffffe9ae2ed07577265dff7f94451e061e163c61"),
    ("sect409k1", 409, 4, "7ffffffffffffffffffffffffffffffffffffffffffffffffffe5f83b2d4ea20400ec4557d5ed3e3e7ca5b4b5c83b8e01e5fcf"),
    ("sect571k1", 571, 4, "20000000000000000000000000000000000000000000000000000000000000000000000131850e1f19a63e4b391a8db917f4138b630d84be5d639381e91deb45cfe778f637c1001"),
];
const R1_CURVES: [(&str, usize, u32, &str); 6] = [
    ("sect131r1", 131, 2, "0400000000000000023123953A9464B54D"),
    ("sect163r1", 163, 2, "03FFFFFFFFFFFFFFFFFFFF48AAB689C29CA710279B"),
    ("sect233r1", 233, 2, "1000000000000000000000000000013e974e72f8a6922031d2603cfe0d7"),
    ("sect283r1", 283, 2, "3ffffffffffffffffffffffffffffffffffef90399660fc938a90165b042a7cefadb307"),
    ("sect409r1", 409, 2, "10000000000000000000000000000000000000000000000000001e2aad6a612f33307be5fa47c3c9e052f838164cd37d9a21173"),
    ("sect571r1", 571, 2, "3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe661ce18ff55987308059b186823851ec7dd9ca1161de93d5174d66e8382e9bb2fe84e47"),
];

fn deployed_cells() -> Vec<Cell> {
    let mut cells = Vec::new();
    // ECC2K-130's order, decimal (RESEARCH_ECC2K130_DECOMPOSITION.md section 0)
    // against the hex above.
    let dec: f64 = "680564733841876926932320129493409985129".parse().unwrap();
    assert!((dec.log2() - hex_log2(K1_CURVES[0].3)).abs() < 1e-9, "ECC2K-130 order");
    let koblitz: Vec<(&str, usize, u32, &str)> = K1_CURVES.to_vec();
    let random: Vec<(&str, usize, u32, &str)> = R1_CURVES.to_vec();
    for (family, list, koblitz_family) in [("k1", koblitz, true), ("r1", random, false)] {
        for (name, n, h, order_hex) in list {
            let log2_r = hex_log2(order_hex);
            // Sanity: h * r is #E, within the Hasse interval of 2^n.
            let log2_e = log2_r + (h as f64).log2();
            assert!(
                (log2_e - n as f64).abs() < 1.0,
                "{name}: log2(h*r) = {log2_e} is not near n = {n}"
            );
            let autos = if koblitz_family { 2.0 * n as f64 } else { 2.0 };
            cells.push(Cell {
                family,
                curve: name.to_string(),
                n,
                log2_r,
                h,
                autos,
                rho: rho_log2(log2_r, autos),
            });
        }
    }
    for c in &cells {
        assert!(N_CELLS.contains(&c.n));
    }
    cells
}

// ---------------------------------------------------------------------------
// Output helpers
// ---------------------------------------------------------------------------

fn write_once(dir: &Path, name: &str, body: &str) {
    std::fs::create_dir_all(dir).expect("create out dir");
    let path = dir.join(name);
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap_or_else(|e| panic!("{} is write-once: {e}", path.display()));
    f.write_all(body.as_bytes()).expect("write");
    println!("wrote {} ({} bytes)", path.display(), body.len());
}

fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1).cloned())
}

// ---------------------------------------------------------------------------
// control: the positive control against the committed tables, the negative
// control (m = 2), and internal monotonicity checks
// ---------------------------------------------------------------------------

fn parse_cells(line: &str) -> Option<Vec<String>> {
    let t = line.trim();
    if !t.starts_with('|') {
        return None;
    }
    let cells: Vec<String> = t.trim_matches('|').split('|').map(|c| c.trim().to_string()).collect();
    cells.first()?.parse::<usize>().ok()?;
    Some(cells)
}

fn scale_model(name: &str) -> Option<Model> {
    if name.starts_with("free oracle") {
        Some(Model::Floor)
    } else if name.starts_with("2 (any") {
        Some(Model::Const(2))
    } else if name.starts_with("4, constant") {
        Some(Model::Const(4))
    } else if name.starts_with("5, constant") {
        Some(Model::Const(5))
    } else if name.starts_with("6, constant") {
        Some(Model::Const(6))
    } else if name.starts_with("7, constant") {
        Some(Model::Const(7))
    } else if name.starts_with("semi-regular") {
        Some(Model::SemiRegular)
    } else if name.starts_with("ceil(l/2) + 4") {
        Some(Model::HalfL4)
    } else if name.starts_with("l + 4") {
        Some(Model::LPlus4)
    } else {
        None
    }
}

fn fmt2(prefix: &str, v: f64) -> String {
    format!("{prefix}{v:.2}")
}

fn control(args: &[String]) -> bool {
    let out_dir = PathBuf::from(arg_value(args, "--out-dir").expect("--out-dir"));
    let scale_ref = std::fs::read_to_string(arg_value(args, "--scale-ref").expect("--scale-ref"))
        .expect("read scale ref");
    let d4_ref = std::fs::read_to_string(arg_value(args, "--d4-ref").expect("--d4-ref"))
        .expect("read d4 ref");
    let mut report = String::new();
    let mut ok = true;
    let mut semi = SemiCache(HashMap::new());

    // 1. Positive control A: every row of the committed n = 131 table
    //    (descent_degree_cost-output-20261016-scale.txt), l in 4..70 as there.
    let (mut seen, mut matched) = (0usize, 0usize);
    for line in scale_ref.lines() {
        let Some(c) = parse_cells(line) else { continue };
        if c.len() != 11 {
            continue;
        }
        let m: usize = c[0].parse().unwrap();
        let Some(model) = scale_model(&c[1]) else {
            writeln!(report, "UNMAPPED scale row: {line}").unwrap();
            ok = false;
            continue;
        };
        let w: u32 = c[2].parse().unwrap();
        seen += 1;
        let (l, cc) = best(131, m, model, w, 1.0, 4, 69, &mut semi);
        let want = vec![
            c[0].clone(),
            c[1].clone(),
            c[2].clone(),
            l.to_string(),
            cc.unknowns.to_string(),
            if cc.degree == 0 { "\u{2014}".to_string() } else { cc.degree.to_string() },
            fmt2("2^", cc.targets),
            fmt2("2^", cc.oracle),
            fmt2("2^", cc.algebra),
            format!("**{}**", fmt2("2^", cc.total)),
            format!("2^{:+.2}", cc.total - RHO_COMMITTED),
        ];
        if want == c {
            matched += 1;
        } else {
            ok = false;
            writeln!(report, "MISMATCH scale\n  committed {c:?}\n  computed  {want:?}").unwrap();
        }
    }
    writeln!(
        report,
        "positive control A (descent_degree_cost-output-20261016-scale.txt, n = 131, l in 4..70, rho 60.81): {matched}/{seen} rows identical"
    )
    .unwrap();
    if seen == 0 || matched != seen {
        ok = false;
    }

    // 2. Positive control B: cost_d4-output.txt (D = 3..6, l in 4..=70).
    let (mut seen_b, mut matched_b) = (0usize, 0usize);
    for line in d4_ref.lines() {
        let Some(c) = parse_cells(line) else { continue };
        if c.len() != 7 {
            continue;
        }
        let m: usize = c[0].parse().unwrap();
        let d: u32 = c[1].parse().unwrap();
        let w: u32 = c[2].parse().unwrap();
        seen_b += 1;
        let (l, cc) = best(131, m, Model::Const(d), w, 1.0, 4, 70, &mut semi);
        let want = vec![
            c[0].clone(),
            c[1].clone(),
            c[2].clone(),
            l.to_string(),
            cc.unknowns.to_string(),
            fmt2("2^", cc.total),
            format!("2^{:+.2}", cc.total - RHO_COMMITTED),
        ];
        if want == c {
            matched_b += 1;
        } else {
            ok = false;
            writeln!(report, "MISMATCH d4\n  committed {c:?}\n  computed  {want:?}").unwrap();
        }
    }
    writeln!(
        report,
        "positive control B (06-equivariant-f4-m4/cost_d4-output.txt, n = 131, l in 4..=70, rho 60.81): {matched_b}/{seen_b} rows identical"
    )
    .unwrap();
    if seen_b == 0 || matched_b != seen_b {
        ok = false;
    }

    // 3. The headline rows named by the design, stated explicitly.
    let (_, c76) = best(131, 4, Model::Const(6), 1, 1.0, 4, 69, &mut semi);
    let (_, c88) = best(131, 3, Model::Const(6), 1, 1.0, 4, 69, &mut semi);
    let headline = fmt2("2^", c76.total) == "2^76.31" && fmt2("2^", c88.total) == "2^88.05";
    writeln!(
        report,
        "headline rows: m=4 D=6 w=1 -> {} (committed 2^76.31); m=3 D=6 w=1 -> {} (committed 2^88.05): {}",
        fmt2("2^", c76.total),
        fmt2("2^", c88.total),
        if headline { "identical" } else { "DIFFERENT" }
    )
    .unwrap();
    ok &= headline;

    // 4. Rho reference check: the ECC2K-130 cell must give the committed 2^60.8090.
    let cells = deployed_cells();
    let ecc = cells.iter().find(|c| c.curve == "ECC2K-130").unwrap();
    let rho_ok = format!("{:.4}", ecc.rho) == "60.8090" && ecc.h == 4;
    writeln!(
        report,
        "rho reference: ECC2K-130 log2 r = {:.6}, h = {}, A = {}, rho = {:.4} (committed 60.8090): {}",
        ecc.log2_r,
        ecc.h,
        ecc.autos,
        ecc.rho,
        if rho_ok { "identical" } else { "DIFFERENT" }
    )
    .unwrap();
    ok &= rho_ok;

    // 5. Extended-range check at n = 131: the committed minimiser does not
    //    move when the search widens from 4..70 to 1..=131.
    let mut moved = Vec::new();
    for &m in &[3usize, 4] {
        for model in [Model::Const(3), Model::Const(4), Model::Const(6), Model::Const(7)] {
            for &w in &W_LIST {
                let (l0, c0) = best(131, m, model, w, 1.0, 4, 69, &mut semi);
                let (l1, c1) = best(131, m, model, w, 1.0, 1, 131, &mut semi);
                if l0 != l1 || (c0.total - c1.total).abs() > 1e-12 {
                    moved.push(format!("m={m} {} w={w}: l {l0}->{l1}", model.label()));
                }
            }
        }
    }
    writeln!(
        report,
        "range check (constant-D rows, n = 131, 4..70 vs 1..=131): {}",
        if moved.is_empty() { "minimiser unchanged".to_string() } else { moved.join("; ") }
    )
    .unwrap();
    ok &= moved.is_empty();

    // 6. Negative control: m = 2 must never cross rho. Every cell, both
    //    families, every model incl. the floor, every w and every K.
    let mut m2_min = f64::INFINITY;
    let mut m2_at = String::new();
    let mut models: Vec<Model> = DESIGN_MODELS.to_vec();
    models.push(Model::HalfL4);
    models.push(Model::Floor);
    for cell in &cells {
        for &model in &models {
            for &w in &W_LIST {
                for &k in &K_LIST {
                    if model == Model::Floor && (w != 1 || k != 1.0) {
                        continue;
                    }
                    let (l, c) = best(cell.n, 2, model, w, k, 1, cell.n, &mut semi);
                    let gap = c.total - cell.rho;
                    if gap < m2_min {
                        m2_min = gap;
                        m2_at = format!("{} {} w={w} K={k} l={l}", cell.curve, model.label());
                    }
                }
            }
        }
    }
    let neg_ok = m2_min > 0.0;
    writeln!(
        report,
        "negative control (m = 2, all cells x models x w x K): minimum gap to rho = {m2_min:+.2} bits at {m2_at}: {}",
        if neg_ok { "never crosses rho" } else { "CROSSES RHO: control fails" }
    )
    .unwrap();
    ok &= neg_ok;

    // 7. Internal monotonicity: gap(w=2) >= gap(w=1); gap non-increasing in K;
    //    gap non-decreasing in constant D.
    let mut violations = 0usize;
    let mut checks = 0usize;
    for cell in &cells {
        for &m in &M_LIST {
            for &model in &[Model::Const(3), Model::Const(4), Model::Const(6), Model::Const(7), Model::LPlus4] {
                let g = |w: u32, k: f64, semi: &mut SemiCache| {
                    best(cell.n, m, model, w, k, 1, cell.n, semi).1.total
                };
                let (w1, w2) = (g(1, 1.0, &mut semi), g(2, 1.0, &mut semi));
                checks += 1;
                if w2 < w1 - 1e-12 {
                    violations += 1;
                }
                let mut prev = f64::INFINITY;
                for &k in &K_LIST {
                    let t = g(1, k, &mut semi);
                    checks += 1;
                    if t > prev + 1e-12 {
                        violations += 1;
                    }
                    prev = t;
                }
            }
            let mut prev = f64::NEG_INFINITY;
            for d in [3u32, 4, 6, 7] {
                let t = best(cell.n, m, Model::Const(d), 1, 1.0, 1, cell.n, &mut semi).1.total;
                checks += 1;
                if t < prev - 1e-12 {
                    violations += 1;
                }
                prev = t;
            }
        }
    }
    writeln!(
        report,
        "internal monotonicity (w, K, D): {} checks, {} violations: {}",
        checks,
        violations,
        if violations == 0 { "pass" } else { "FAIL" }
    )
    .unwrap();
    ok &= violations == 0;

    writeln!(report, "CONTROL {}", if ok { "PASS" } else { "FAIL" }).unwrap();
    write_once(&out_dir, "control.txt", &report);
    print!("{report}");
    ok
}

// ---------------------------------------------------------------------------
// cells: the deployed-size table
// ---------------------------------------------------------------------------

struct Row {
    cell: Cell,
    model: Model,
    m: usize,
    w: u32,
    k: Option<f64>,
    l: usize,
    c: Cost,
}

impl Row {
    fn gap(&self) -> f64 {
        self.c.total - self.cell.rho
    }
}

fn cells_mode(args: &[String]) {
    let out_dir = PathBuf::from(arg_value(args, "--out-dir").expect("--out-dir"));
    // --smoke: the smallest cell only (n = 131), to prove the code path.
    let smoke = args.iter().any(|a| a == "--smoke");
    let cells: Vec<Cell> = deployed_cells()
        .into_iter()
        .filter(|c| !smoke || c.n == 131)
        .collect();
    let n_cells: Vec<usize> = N_CELLS.iter().copied().filter(|&n| !smoke || n == 131).collect();
    let mut semi = SemiCache(HashMap::new());
    let mut models: Vec<Model> = vec![Model::Floor];
    models.extend(DESIGN_MODELS);
    models.push(Model::HalfL4);
    let mut rows: Vec<Row> = Vec::new();
    for cell in &cells {
        for &m in &M_LIST {
            for &model in &models {
                for &w in &W_LIST {
                    for &k in &K_LIST {
                        if model == Model::Floor && (w != 1 || k != 1.0) {
                            continue;
                        }
                        let (l, c) = best(cell.n, m, model, w, k, 1, cell.n, &mut semi);
                        rows.push(Row {
                            cell: cell.clone(),
                            model,
                            m,
                            w,
                            k: if model == Model::Floor { None } else { Some(k) },
                            l,
                            c,
                        });
                    }
                }
            }
        }
    }

    // Full TSV.
    let mut tsv = String::from(
        "family\tcurve\tn\tlog2_r\th\tautomorphisms\trho_log2\tmodel\tdesign_arm\tm\tw\tK\tbest_l\tunknowns_N\tdegree_D\ttargets_log2\toracle_per_target_log2\talgebra_log2\ttotal_log2\tgap_to_rho_bits\tmemory_log2_words\tl_at_search_edge\ttargets_exceed_subgroup\ttotal_exceeds_subgroup\n",
    );
    for r in &rows {
        writeln!(
            tsv,
            "{}\t{}\t{}\t{:.6}\t{}\t{}\t{:.4}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.2}\t{:.2}\t{:.2}\t{:.4}\t{:+.4}\t{:.2}\t{}\t{}\t{}",
            r.cell.family,
            r.cell.curve,
            r.cell.n,
            r.cell.log2_r,
            r.cell.h,
            r.cell.autos,
            r.cell.rho,
            r.model.label(),
            if r.model.is_design() { "design" } else if r.model == Model::Floor { "floor" } else { "supplementary" },
            r.m,
            r.w,
            r.k.map_or("na".to_string(), |k| format!("{k}")),
            r.l,
            r.c.unknowns,
            r.c.degree,
            r.c.targets,
            r.c.oracle,
            r.c.algebra,
            r.c.total,
            r.gap(),
            r.c.memory,
            r.l == 1 || r.l == r.cell.n,
            r.c.targets > r.cell.log2_r,
            r.c.total > r.cell.log2_r,
        )
        .unwrap();
    }
    write_once(&out_dir, "cells.tsv", &tsv);

    // Gap matrix: one row per (family, model, m, w, K); one column per n.
    let mut md = String::new();
    writeln!(md, "# Item 02 cell table: gap to matched rho, bits (total - rho; <= 0 means at or below rho)\n").unwrap();
    writeln!(md, "Derived from the cost model; K divides the oracle term only; best l per cell, l searched over 1..=n.\n").unwrap();
    writeln!(md, "## Reference rows\n").unwrap();
    writeln!(md, "| family | curve | n | log2 r | h | automorphisms A | rho log2 (group ops) |").unwrap();
    writeln!(md, "|---|---|--:|--:|--:|--:|--:|").unwrap();
    for c in &cells {
        writeln!(md, "| {} | {} | {} | {:.4} | {} | {} | {:.4} |", c.family, c.curve, c.n, c.log2_r, c.h, c.autos, c.rho).unwrap();
    }
    for family in ["k1", "r1"] {
        writeln!(md, "\n## Family {family}: gap to rho by n (best l; memory log2 words in brackets)\n").unwrap();
        write!(md, "| model | m | w | K |").unwrap();
        for &n in &n_cells {
            write!(md, " n={n} |").unwrap();
        }
        write!(md, "\n|---|--:|--:|--:|").unwrap();
        for _ in &n_cells {
            write!(md, "--:|").unwrap();
        }
        writeln!(md).unwrap();
        for &model in &models {
            for &m in &M_LIST {
                for &w in &W_LIST {
                    for &k in &K_LIST {
                        if model == Model::Floor && (w != 1 || k != 1.0) {
                            continue;
                        }
                        write!(
                            md,
                            "| {}{} | {} | {} | {} |",
                            model.label(),
                            if model.is_design() || model == Model::Floor { "" } else { " (supp.)" },
                            m,
                            w,
                            if model == Model::Floor { "na".to_string() } else { format!("{k}") }
                        )
                        .unwrap();
                        for &n in &n_cells {
                            let r = rows
                                .iter()
                                .find(|r| {
                                    r.cell.family == family
                                        && r.cell.n == n
                                        && r.model == model
                                        && r.m == m
                                        && r.w == w
                                        && r.k.is_none_or(|kk| kk == k)
                                })
                                .unwrap();
                            write!(md, " {:+.2} [{:.0}] |", r.gap(), r.c.memory).unwrap();
                        }
                        writeln!(md).unwrap();
                    }
                }
            }
        }
    }
    // Flip table: minimum gap over the six cells.
    writeln!(md, "\n## Flip table (rule: a model flips if its gap is <= 0 at some deployed n)\n").unwrap();
    writeln!(md, "| family | model | arm | m | w | K | min gap over n in {{131..571}} | at n | flips |").unwrap();
    writeln!(md, "|---|---|---|--:|--:|--:|--:|--:|---|").unwrap();
    let mut flips = Vec::new();
    for family in ["k1", "r1"] {
        for &model in &models {
            for &m in &M_LIST {
                for &w in &W_LIST {
                    for &k in &K_LIST {
                        if model == Model::Floor && (w != 1 || k != 1.0) {
                            continue;
                        }
                        let sel: Vec<&Row> = rows
                            .iter()
                            .filter(|r| {
                                r.cell.family == family
                                    && r.model == model
                                    && r.m == m
                                    && r.w == w
                                    && r.k.is_none_or(|kk| kk == k)
                            })
                            .collect();
                        let min = sel
                            .iter()
                            .min_by(|a, b| a.gap().partial_cmp(&b.gap()).unwrap())
                            .unwrap();
                        let flips_here = min.gap() <= 0.0;
                        let arm = if model.is_design() { "design" } else if model == Model::Floor { "floor" } else { "supplementary" };
                        writeln!(
                            md,
                            "| {} | {} | {} | {} | {} | {} | {:+.2} | {} | {} |",
                            family,
                            model.label(),
                            arm,
                            m,
                            w,
                            if model == Model::Floor { "na".to_string() } else { format!("{k}") },
                            min.gap(),
                            min.cell.n,
                            if flips_here { "YES" } else { "no" }
                        )
                        .unwrap();
                        if flips_here {
                            flips.push(format!(
                                "{family} {} ({arm}) m={m} w={w} K={} at n={}: gap {:+.2}",
                                model.label(),
                                if model == Model::Floor { "na".to_string() } else { format!("{k}") },
                                min.cell.n,
                                min.gap()
                            ));
                        }
                    }
                }
            }
        }
    }
    writeln!(md, "\n## Rows with gap <= 0 at a deployed n ({})\n", flips.len()).unwrap();
    for f in &flips {
        writeln!(md, "- {f}").unwrap();
    }
    write_once(&out_dir, "cells.md", &md);
    println!("cells: {} rows; {} rows with min gap <= 0", rows.len(), flips.len());
    for f in flips.iter().take(40) {
        println!("  FLIP {f}");
    }
}

// ---------------------------------------------------------------------------
// scan: the smooth family, crossover n per model
// ---------------------------------------------------------------------------

fn scan_mode(args: &[String]) {
    let out_dir = PathBuf::from(arg_value(args, "--out-dir").expect("--out-dir"));
    let mut semi = SemiCache(HashMap::new());
    // Smooth families: #E = 2^n; Koblitz h = 4, A = 2n; random h = 2, A = 2.
    let families: [(&str, f64, bool); 2] = [("k1", 4.0, true), ("r1", 2.0, false)];
    // --smoke: grids end at n = 131 (the smallest cell) to prove the code path.
    let smoke = args.iter().any(|a| a == "--smoke");
    let mut const_grid: Vec<usize> = (64..=1024).collect();
    const_grid.extend((1032..=4096).step_by(8));
    let mut growth_grid: Vec<usize> = (64..=1024).step_by(16).collect();
    for n in N_CELLS {
        if !growth_grid.contains(&n) {
            growth_grid.push(n);
        }
    }
    growth_grid.sort_unstable();
    if smoke {
        const_grid.retain(|&n| n <= 131);
        growth_grid.retain(|&n| n <= 131);
    }
    let models: Vec<Model> = {
        let mut v = vec![Model::Floor];
        v.extend(DESIGN_MODELS);
        v.push(Model::HalfL4);
        v
    };
    let mut tsv = String::from(
        "family\tmodel\tarm\tm\tw\tK\tgrid\tfirst_n_gap_le_0\tsustained_from_n\tgap_at_571\tgap_at_grid_max\tgrid_max_n\tmin_gap\tn_of_min_gap\n",
    );
    let mut md = String::new();
    writeln!(md, "# Item 02 crossover scan (smooth families; derived)\n").unwrap();
    writeln!(md, "Smooth family: #E = 2^n, h = 4 and A = 2n (k1) or h = 2 and A = 2 (r1). Constant-D arms scan n = 64..=1024 step 1 then to 4096 step 8; growth arms (D = l+4, ceil(l/2)+4) scan n = 64..=1024 step 16 plus the six deployed n. `first n` is the smallest grid n with gap <= 0; `sustained from` is the smallest grid n after which every larger grid n also has gap <= 0.\n").unwrap();
    writeln!(md, "| family | model | arm | m | w | K | first n (gap<=0) | sustained from | gap at 571 | gap at grid max (n) | min gap (n) |").unwrap();
    writeln!(md, "|---|---|---|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    let mut flipped = 0usize;
    for &(family, h, koblitz) in &families {
        for &model in &models {
            let growth = matches!(model, Model::LPlus4 | Model::HalfL4);
            let grid = if growth { &growth_grid } else { &const_grid };
            for &m in &M_LIST {
                for &w in &W_LIST {
                    for &k in &K_LIST {
                        if model == Model::Floor && (w != 1 || k != 1.0) {
                            continue;
                        }
                        let mut gaps: Vec<(usize, f64)> = Vec::with_capacity(grid.len());
                        for &n in grid {
                            let log2_r = n as f64 - h.log2();
                            let autos = if koblitz { 2.0 * n as f64 } else { 2.0 };
                            let rho = rho_log2(log2_r, autos);
                            let (_, c) = best(n, m, model, w, k, 1, n, &mut semi);
                            gaps.push((n, c.total - rho));
                        }
                        let first = gaps.iter().find(|(_, g)| *g <= 0.0).map(|(n, _)| *n);
                        let mut sustained = None;
                        for i in (0..gaps.len()).rev() {
                            if gaps[i].1 <= 0.0 {
                                sustained = Some(gaps[i].0);
                            } else {
                                break;
                            }
                        }
                        let g571 = gaps
                            .iter()
                            .find(|(n, _)| *n == 571)
                            .map_or(f64::NAN, |(_, g)| *g);
                        let (nmax, gmax) = *gaps.last().unwrap();
                        let (nmin, gmin) = gaps
                            .iter()
                            .cloned()
                            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                            .unwrap();
                        let arm = if model.is_design() { "design" } else if model == Model::Floor { "floor" } else { "supplementary" };
                        let kk = if model == Model::Floor { "na".to_string() } else { format!("{k}") };
                        let f = first.map_or("none".to_string(), |n| n.to_string());
                        let s = sustained.map_or("none".to_string(), |n| n.to_string());
                        writeln!(
                            tsv,
                            "{family}\t{}\t{arm}\t{m}\t{w}\t{kk}\t{}\t{f}\t{s}\t{g571:+.4}\t{gmax:+.4}\t{nmax}\t{gmin:+.4}\t{nmin}",
                            model.label(),
                            if growth { "growth" } else { "const" },
                        )
                        .unwrap();
                        writeln!(
                            md,
                            "| {family} | {} | {arm} | {m} | {w} | {kk} | {f} | {s} | {g571:+.2} | {gmax:+.2} ({nmax}) | {gmin:+.2} ({nmin}) |",
                            model.label()
                        )
                        .unwrap();
                        if first.is_some() {
                            flipped += 1;
                        }
                    }
                }
            }
        }
    }
    write_once(&out_dir, "scan.tsv", &tsv);
    write_once(&out_dir, "scan.md", &md);
    println!("scan: {flipped} (family, model, m, w, K) rows have a grid n with gap <= 0");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str).unwrap_or("");
    match mode {
        "control" => {
            if !control(&args) {
                std::process::exit(1);
            }
        }
        "cells" => cells_mode(&args),
        "scan" => scan_mode(&args),
        _ => {
            eprintln!("usage: control --scale-ref F --d4-ref F --out-dir D | cells --out-dir D | scan --out-dir D");
            std::process::exit(2);
        }
    }
}
