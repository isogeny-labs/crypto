//! R5 emitter and runner: the Weil-descended, `e_i`-symmetrised Semaev `S_4` system of the
//! published-IC reference arm (`binary_semaev_s4::weil_descend_s4`, the same system the msolve
//! arm solves in its `form=e`), written in WDSat's ANF input format and solved by the external
//! WDSat binary (github.com/mtrimoska/WDSat, 61c6ff3f49445af2729274819edb27b85a89efc1, GPL-3.0).
//!
//! WDSat's source is not in this repository; this file only writes its input file, starts the
//! process and reads its stdout.
//!
//! Input format (WDSat README "ANF"): header `p cnf <#unary vars> <#equations>`, then one line
//! per equation `x [T] <terms> 0`.  A term is a variable number `v`, or `.d v1 .. vd` for the
//! conjunction of `d` variables.  A line asserts that the XOR of its terms (with the constant
//! `1` when `T` is present) is TRUE.  So a polynomial `P = 0` is written without `T` iff `P`
//! has a constant term (`terms + 1 = 0` is `terms = 1`), and with `T` otherwise.
//! Numbering follows WDSat's own `benchmarks/IC_S4` files: variables `1..=3l` are the bits of
//! `X_1, X_2, X_3` (block `i`, bit `j` is number `i*l + j + 1`), then the bits of `e_1` (`l`),
//! `e_2` (`2l-1`), `e_3` (`3l-2`).  The `3l+6l-3` definitional equations `e_{i,d} = sigma_{i,d}(x)`
//! come first, then the non-zero Semaev equations in the `e` bits.
//!
//! Output (WDSat built with `__FIND_ALL_SOLUTIONS__`): per solution `SAT:`, a 0/1 string over all
//! unary variables, `conf:<n>`; the run ends with `UNSAT` and a counter.  With `-b` WDSat prunes
//! the permutation symmetry of `(X_1, X_2, X_3)`, so only a representative of each class is
//! printed; the caller (`decompose`) sorts and dedups triples anyway.
//!
//! Each call is one child process under `/usr/bin/time -l` so instructions retired are counted
//! exactly as the msolve arm's (`msolve_s4::run_msolve`).
use std::path::PathBuf;
use std::process::Command;

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::binary_semaev_s4::{weil_descend_s4, AnfPoly};

#[derive(Debug, Default, Clone)]
pub struct WdOutcome {
    /// Triples (X1, X2, X3) packed bit k = coefficient of z^k, one per printed solution.
    pub roots: Vec<[u64; 3]>,
    /// Instructions retired by the WDSat process (`/usr/bin/time -l`).
    pub ir: u64,
    /// WDSat processes started (0 when the emitter refutes the system outright).
    pub calls: u64,
    /// Systems refuted by the emitter alone (a polynomial that is the constant 1).
    pub refuted_by_emitter: u64,
    /// 1 when no binary with the exact `__MAX_ID__` existed and a larger one was used.
    pub inexact: u64,
    /// Systems WDSat reported UNSAT (no solution printed).
    pub unsat: u64,
    /// WDSat's decision counter of the last line (`conf`), summed.
    pub conflicts: u64,
    pub n_vars: usize,
    pub n_eqs: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct WdTune {
    /// `-b`: break the permutation symmetry of the three summands.
    pub sym: bool,
    /// `-x`: WDSat's XOR-Gauss module (the published recommendation for these ANF inputs).
    pub xg: bool,
    /// `true`: the binary is built with `__FIND_ALL_SOLUTIONS__` (every model, then UNSAT); `false`: WDSat's
    /// default, stop at the first model.  The pipeline uses only the first liftable root, and every model
    /// lifted in the sessions run, so `false` is the solver as normally used.
    pub all: bool,
}

impl Default for WdTune {
    fn default() -> Self {
        Self { sym: true, xg: true, all: true }
    }
}

/// The emitted system.
pub struct Emitted {
    pub text: String,
    pub n_unary: usize,
    /// Boolean variables WDSat will use: unary plus one per distinct monomial of degree >= 2
    /// (upstream `dimacs.c`); it fixes the `__MAX_ID__` of the binary built for this system.
    pub n_total: usize,
    pub n_eqs: usize,
    /// `true` when some equation is the constant 1 (no solution).
    pub refuted: bool,
}

/// Emit the e-form system for target abscissa `x_r` in WDSat's ANF format.
pub fn emit(n: u32, l: u32, irr: &IrreduciblePoly, b: &F2mElement, x_r: &F2mElement) -> Emitted {
    let sys = weil_descend_s4(n, l, irr, b, x_r);
    let nx = sys.n_x_vars();
    let ne = sys.n_e_vars();
    let mut polys: Vec<AnfPoly> = Vec::new();
    for (i, row) in sys.correspondence.iter().enumerate() {
        for (d, sigma) in row.iter().enumerate() {
            let mut eq = AnfPoly::var(nx + sys.e_var(i, d));
            eq.xor_assign(sigma);
            polys.push(eq);
        }
    }
    let mut refuted = false;
    for f in &sys.semaev {
        if f.is_zero() {
            continue;
        }
        // The e-space polynomial shifts into the shared numbering after the x-bits.
        let mut shifted = AnfPoly::zero();
        for mono in f.monomials() {
            let mut one = AnfPoly::one();
            for &v in mono {
                one = one.mul(&AnfPoly::var(v + nx));
            }
            shifted.xor_assign(&one);
        }
        if shifted.len() == 1 && shifted.has_constant() {
            refuted = true;
        }
        polys.push(shifted);
    }
    let n_unary = (nx + ne) as usize;
    let mut text = format!("p cnf {} {}\n", n_unary, polys.len());
    let mut mons: std::collections::BTreeSet<Vec<u32>> = std::collections::BTreeSet::new();
    for p in &polys {
        let has_const = p.has_constant();
        text.push('x');
        if !has_const {
            text.push_str(" T");
        }
        for mono in p.monomials() {
            if mono.is_empty() {
                continue;
            }
            let mut vs: Vec<u32> = mono.iter().map(|v| v + 1).collect();
            vs.sort_unstable();
            if vs.len() >= 2 {
                mons.insert(vs.clone());
            }
            if vs.len() == 1 {
                text.push_str(&format!(" {}", vs[0]));
            } else {
                text.push_str(&format!(" .{}", vs.len()));
                for v in vs {
                    text.push_str(&format!(" {v}"));
                }
            }
        }
        text.push_str(" 0\n");
    }
    Emitted { text, n_unary, n_total: n_unary + mons.len(), n_eqs: polys.len(), refuted }
}

/// Run `wdsat -i <file> -x [-b] -n n -l l -m 3` through `/usr/bin/time -l`; return (stdout, ir).
fn run_wdsat(bin: &str, input: &str, tag: &str, n: u32, l: u32, tune: WdTune) -> Result<(String, u64), String> {
    let dir: PathBuf = std::env::temp_dir().join(format!("wdsat_s4_{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let fin = dir.join(format!("{tag}.anf"));
    let fout = dir.join(format!("{tag}.out"));
    let ftime = dir.join(format!("{tag}.time"));
    std::fs::write(&fin, input).map_err(|e| e.to_string())?;
    let stdout = std::fs::File::create(&fout).map_err(|e| e.to_string())?;
    let mut cmd = Command::new("/usr/bin/time");
    cmd.arg("-l").arg("-o").arg(&ftime).arg(bin).arg("-i").arg(&fin);
    if tune.xg {
        cmd.arg("-x");
    }
    if tune.sym {
        cmd.arg("-b");
    }
    cmd.args(["-n", &n.to_string(), "-l", &l.to_string(), "-m", "3"]);
    let status = cmd
        .stdout(stdout)
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("cannot run wdsat: {e}"))?;
    if !status.success() {
        return Err(format!("wdsat exited with {status}"));
    }
    let out = std::fs::read_to_string(&fout).map_err(|e| e.to_string())?;
    let tm = std::fs::read_to_string(&ftime).map_err(|e| e.to_string())?;
    let ir = tm
        .lines()
        .find(|l| l.contains("instructions retired"))
        .and_then(|l| l.split_whitespace().next())
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or("no `instructions retired` line (is the process sandboxed?)")?;
    let _ = std::fs::remove_file(&fin);
    let _ = std::fs::remove_file(&fout);
    let _ = std::fs::remove_file(&ftime);
    Ok((out, ir))
}

/// Parse a find-all run: every `SAT:` block's bit string, and whether the run ended with a verdict.
fn parse(out: &str, n_unary: usize, l: u32) -> Result<(Vec<[u64; 3]>, u64), String> {
    let mut roots = Vec::new();
    let mut lines = out.lines().peekable();
    let mut conf = 0u64;
    let mut ended = false;
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line == "SAT:" {
            let bits = lines.next().ok_or("SAT: without a model line")?.trim();
            if bits.len() != n_unary || !bits.bytes().all(|c| c == b'0' || c == b'1') {
                return Err(format!("model line of length {} for {n_unary} unknowns", bits.len()));
            }
            let b = bits.as_bytes();
            let mut xs = [0u64; 3];
            for i in 0..3usize {
                for j in 0..l as usize {
                    if b[i * l as usize + j] == b'1' {
                        xs[i] |= 1 << j;
                    }
                }
            }
            roots.push(xs);
            if let Some(c) = lines.next() {
                if let Some(v) = c.trim().strip_prefix("conf:") {
                    conf = v.trim().parse().unwrap_or(0);
                }
            }
        } else if line == "UNSAT" || line == "UNSAT on XORGAUSS init" {
            // Find-all mode always ends with UNSAT (the search returns false after each model).
            ended = true;
            break;
        }
    }
    if !ended {
        return Err("WDSat output ended without a UNSAT verdict line (find-all mode)".into());
    }
    Ok((roots, conf))
}

/// Parse a first-solution run: a model line (0/1 string over all unary variables) then the decision
/// counter, or `UNSAT` / `UNSAT on XORGAUSS init` then the counter.
fn parse_first(out: &str, n_unary: usize, l: u32) -> Result<(Vec<[u64; 3]>, u64), String> {
    for line in out.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('!') || line.starts_with('c') {
            continue;
        }
        if line == "UNSAT" || line == "UNSAT on XORGAUSS init" {
            return Ok((Vec::new(), 0));
        }
        if line.len() == n_unary && line.bytes().all(|c| c == b'0' || c == b'1') {
            let b = line.as_bytes();
            let mut xs = [0u64; 3];
            for i in 0..3usize {
                for j in 0..l as usize {
                    if b[i * l as usize + j] == b'1' {
                        xs[i] |= 1 << j;
                    }
                }
            }
            return Ok((vec![xs], 0));
        }
        return Err(format!("unrecognised WDSat output line `{}`", &line[..line.len().min(40)]));
    }
    Err("empty WDSat output".into())
}

/// Solve the descended system for target abscissa `x_r`; `bin` is the per-cell WDSat binary.
pub fn solve_s4(
    bin: &str,
    n: u32,
    l: u32,
    irr: &IrreduciblePoly,
    b: &F2mElement,
    x_r: &F2mElement,
    tag: &str,
    tune: WdTune,
) -> Result<WdOutcome, String> {
    let em = emit(n, l, irr, b, x_r);
    // `bin` is a directory of per-size binaries `wdsat_id<N>` (N = Boolean variables of the system):
    // upstream's static arrays are sized by `__MAX_ID__`, which must be exact for speed (it warns otherwise).
    let mut inexact = 0u64;
    let bin_owned;
    let bin = if std::path::Path::new(bin).is_dir() {
        // `<bin>/l<l>/wdsat_id<N>`: the exact N if built, else the next larger one (counted in `inexact`).
        let dir = if tune.all { format!("{bin}/l{l}") } else { format!("{bin}/first/l{l}") };
        let mut best: Option<usize> = None;
        for e in std::fs::read_dir(&dir).map_err(|e| format!("{dir}: {e}"))?.flatten() {
            if let Some(k) = e.file_name().to_string_lossy().strip_prefix("wdsat_id").and_then(|k| k.parse::<usize>().ok()) {
                if k >= em.n_total && best.map_or(true, |b| k < b) {
                    best = Some(k);
                }
            }
        }
        let k = best.ok_or_else(|| format!("no WDSat binary with MAX_ID >= {} in {dir}", em.n_total))?;
        if k != em.n_total {
            inexact = 1;
        }
        bin_owned = format!("{dir}/wdsat_id{k}");
        bin_owned.as_str()
    } else {
        bin
    };
    let mut out = WdOutcome { n_vars: em.n_unary, n_eqs: em.n_eqs, ..Default::default() };
    if em.refuted {
        out.refuted_by_emitter = 1;
        return Ok(out);
    }
    let (text, ir) = run_wdsat(bin, &em.text, tag, n, l, tune)?;
    out.calls = 1;
    out.inexact = inexact;
    out.ir = ir;
    let (roots, conf) = if tune.all { parse(&text, em.n_unary, l)? } else { parse_first(&text, em.n_unary, l)? };
    if roots.is_empty() {
        out.unsat = 1;
    }
    out.conflicts = conf;
    out.roots = roots;
    Ok(out)
}
