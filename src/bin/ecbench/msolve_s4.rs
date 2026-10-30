//! The published-method decomposer of the reference-arm pilot (card
//! `XP-20261024-published-ic-reference-arm`, PILOT_20261030): the `e_i`-symmetrised
//! Weil-descended Semaev `S_4` of `binary_semaev_s4` (Faugere-Gaudry-Huot-Renault's
//! symmetrisation of `S_4` over the elementary symmetric functions, quadratic in the
//! `e_i`) on the x-base `{P : x(P) in <1, z, ..., z^{l-1}>}`, solved by the external
//! `msolve` (F4, `-g 2`, one thread) over GF(2) with the field equations appended.
//!
//! What is NOT the published pipeline here, recorded so it is never mistaken for it:
//! * only the `S_3`-symmetrisation (`e_1, e_2, e_3`) is applied, no 2-torsion `T` invariants;
//! * msolve 0.9.5 returns no solutions at characteristic 2 (its `solve` output is empty
//!   there), so the roots are read off the reduced Groebner basis here, by the multiplication
//!   matrices of the quotient algebra and a joint eigenspace split (an FGLM-type step);
//!   this native work is counted in `ext_ops` and reported apart from msolve's instructions;
//! * one msolve process per system, so each call carries the process start-up cost.
use std::collections::{BTreeSet, HashSet};
use std::path::PathBuf;
use std::process::Command;

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::binary_semaev_s4::{weil_descend_s4, AnfPoly};

/// Result of one `msolve` solve of one `S_4` system.
#[derive(Debug, Default, Clone)]
pub struct MsolveOutcome {
    /// Ordered triples (X1, X2, X3) of x-coordinates in the subspace, packed
    /// bit k = coefficient of z^k, one per F_2-solution of the x-variables.
    pub roots: Vec<[u64; 3]>,
    /// Instructions retired by the msolve process (as reported by `/usr/bin/time -l`).
    pub ir: u64,
    /// Number of F_2-points of the ideal (dimension of the quotient algebra).
    pub quotient_dim: usize,
    pub gb_len: usize,
    /// Native work of the root extraction: monomial-reduction steps + vector XORs.
    pub ext_ops: u64,
    pub n_vars: usize,
    pub n_eqs: usize,
}

fn render(poly: &AnfPoly, name: &dyn Fn(u32) -> String) -> String {
    let mut terms: Vec<String> = Vec::new();
    for mono in poly.monomials() {
        if mono.is_empty() {
            terms.push("1".into());
        } else {
            terms.push(mono.iter().map(|&v| name(v)).collect::<Vec<_>>().join("*"));
        }
    }
    terms.join("+")
}

fn shift(poly: &AnfPoly, by: u32) -> AnfPoly {
    let mut out = AnfPoly::zero();
    for mono in poly.monomials() {
        let m: Vec<u32> = mono.iter().map(|v| v + by).collect();
        let mut one = AnfPoly::one();
        for v in m {
            one = one.mul(&AnfPoly::var(v));
        }
        out.xor_assign(&one);
    }
    out
}

/// Run `msolve -g 2 -t 1` through `/usr/bin/time -l` and return (output text, instructions retired).
fn run_msolve(msolve: &str, input: &str, tag: &str) -> Result<(String, u64), String> {
    let dir: PathBuf = std::env::temp_dir().join(format!("msolve_s4_{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let fin = dir.join(format!("{tag}.ms"));
    let fout = dir.join(format!("{tag}.out"));
    let ftime = dir.join(format!("{tag}.time"));
    std::fs::write(&fin, input).map_err(|e| e.to_string())?;
    let status = Command::new("/usr/bin/time")
        .arg("-l")
        .arg("-o")
        .arg(&ftime)
        .arg(msolve)
        .args(["-g", "2", "-t", "1", "-f"])
        .arg(&fin)
        .arg("-o")
        .arg(&fout)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|e| format!("cannot run msolve: {e}"))?;
    if !status.success() {
        return Err(format!("msolve exited with {status}"));
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

/// Parse msolve's reduced basis into (lead mask, set of masks) per element, with
/// `x^k = x` (the field equations are in the ideal, so the reduced basis is squarefree
/// apart from the `x^2 + x` elements, which vanish under the identification and are dropped).
fn parse_gb(text: &str, vars: &[String]) -> Result<Vec<(u64, BTreeSet<u64>)>, String> {
    let lo = text.find('[').ok_or("no `[` in msolve output")?;
    let hi = text.rfind(']').ok_or("no `]` in msolve output")?;
    let body = &text[lo + 1..hi];
    let mut out = Vec::new();
    for elt in body.split(',') {
        let elt: String = elt.chars().filter(|c| !c.is_whitespace()).collect();
        if elt.is_empty() {
            continue;
        }
        let mut set: BTreeSet<u64> = BTreeSet::new();
        let mut lead: Option<u64> = None;
        for term in elt.split('+') {
            let mut mask = 0u64;
            for tok in term.split('*') {
                if tok == "1" {
                    continue;
                }
                let name = tok.split('^').next().unwrap();
                let idx = vars
                    .iter()
                    .position(|v| v == name)
                    .ok_or_else(|| format!("unknown variable `{name}` in msolve output"))?;
                mask |= 1u64 << idx;
            }
            lead.get_or_insert(mask);
            if !set.remove(&mask) {
                set.insert(mask);
            }
        }
        let lead = lead.unwrap();
        if set.contains(&lead) {
            out.push((lead, set));
        }
    }
    Ok(out)
}

/// Normal form of a squarefree monomial mask modulo the basis (leads divide by mask inclusion).
fn normal_form(mask: u64, gb: &[(u64, BTreeSet<u64>)], ops: &mut u64) -> Result<BTreeSet<u64>, String> {
    let mut poly: BTreeSet<u64> = BTreeSet::new();
    poly.insert(mask);
    let mut guard = 0u64;
    loop {
        let hit = poly
            .iter()
            .find_map(|&m| gb.iter().position(|(l, _)| m & l == *l).map(|j| (m, j)));
        let Some((m, j)) = hit else { return Ok(poly) };
        guard += 1;
        if guard > 5_000_000 {
            return Err("normal-form reduction did not terminate".into());
        }
        *ops += 1;
        let (l, set) = &gb[j];
        let rest = m & !l;
        poly.remove(&m);
        for &t in set.iter().filter(|&&t| t != *l) {
            let nm = t | rest;
            if !poly.remove(&nm) {
                poly.insert(nm);
            }
            *ops += 1;
        }
    }
}

type Vecs = Vec<u128>;

fn span_basis(vs: &[u128]) -> Vecs {
    let mut basis: Vecs = Vec::new();
    for &v0 in vs {
        let mut v = v0;
        for &b in &basis {
            let hb = 127 - b.leading_zeros();
            if (v >> hb) & 1 == 1 {
                v ^= b;
            }
        }
        if v != 0 {
            basis.push(v);
            basis.sort_by(|a, b| b.cmp(a));
        }
    }
    basis
}

fn apply(cols: &[u128], v: u128, ops: &mut u64) -> u128 {
    let mut r = 0u128;
    let mut x = v;
    while x != 0 {
        let b = x.trailing_zeros() as usize;
        r ^= cols[b];
        *ops += 1;
        x &= x - 1;
    }
    r
}

/// Solve the descended, symmetrised `S_4` system for target abscissa `x_r`.
pub fn solve_s4(
    msolve: &str,
    n: u32,
    l: u32,
    irr: &IrreduciblePoly,
    b: &F2mElement,
    x_r: &F2mElement,
    tag: &str,
) -> Result<MsolveOutcome, String> {
    let sys = weil_descend_s4(n, l, irr, b, x_r);
    let nx = sys.n_x_vars();
    // `MSOLVE_S4_FORM=x` substitutes e_i = sigma_i(x) into the descended S_4 and solves over the
    // 3l x-bits alone (higher degree, fewer unknowns); the default keeps the e variables and the
    // correspondence equations (quadratic descended system + cubic correspondence, 3l + 6l - 3 unknowns).
    let x_only = std::env::var("MSOLVE_S4_FORM").map(|v| v == "x").unwrap_or(false);
    let ne = if x_only { 0 } else { sys.n_e_vars() };
    let nv = (nx + ne) as usize;
    if nv > 64 {
        return Err(format!("{nv} unknowns exceed the 64-variable mask"));
    }
    let names: Vec<String> = (0..nx)
        .map(|i| format!("x{i}"))
        .chain((0..ne).map(|i| format!("e{i}")))
        .collect();
    let name = |v: u32| names[v as usize].clone();
    let mut eqs: Vec<String> = Vec::new();
    // correspondence e_{i,d} + sigma_{i,d}(x) = 0, and the n descended S_4 equations over e.
    if x_only {
        let mut emap: Vec<AnfPoly> = vec![AnfPoly::zero(); sys.n_e_vars() as usize];
        for (i, row) in sys.correspondence.iter().enumerate() {
            for (d, sigma) in row.iter().enumerate() {
                emap[sys.e_var(i, d) as usize] = sigma.clone();
            }
        }
        for f in &sys.semaev {
            let mut acc = AnfPoly::zero();
            for mono in f.monomials() {
                let mut prod = AnfPoly::one();
                for &v in mono {
                    prod = prod.mul(&emap[v as usize]);
                }
                acc.xor_assign(&prod);
            }
            if !acc.is_zero() {
                eqs.push(render(&acc, &name));
            }
        }
    }
    for (i, row) in sys.correspondence.iter().enumerate() {
        if x_only {
            break;
        }
        for (d, sigma) in row.iter().enumerate() {
            let ev = nx + sys.e_var(i, d);
            let mut s = format!("{}", names[ev as usize]);
            if !sigma.is_zero() {
                s = format!("{}+{}", s, render(sigma, &name));
            }
            eqs.push(s);
        }
    }
    for f in &sys.semaev {
        if x_only || f.is_zero() {
            continue;
        }
        eqs.push(render(&shift(f, nx), &name));
    }
    let n_eqs_core = eqs.len();
    for v in 0..nv {
        eqs.push(format!("{0}^2+{0}", names[v]));
    }
    let input = format!("{}\n2\n{}\n", names.join(","), eqs.join(",\n"));
    let (text, ir) = run_msolve(msolve, &input, tag)?;
    let gb = parse_gb(&text, &names)?;
    let mut out = MsolveOutcome {
        ir,
        gb_len: gb.len(),
        n_vars: nv,
        n_eqs: n_eqs_core,
        ..Default::default()
    };
    // Unit ideal: a constant leads.
    if gb.iter().any(|(l, _)| *l == 0) {
        return Ok(out);
    }
    // Standard monomials (squarefree, not divisible by a lead), by closure from 1.
    let leads: Vec<u64> = gb.iter().map(|(l, _)| *l).collect();
    let mut basis: Vec<u64> = vec![0];
    let mut seen: HashSet<u64> = HashSet::new();
    seen.insert(0);
    let mut qi = 0;
    while qi < basis.len() {
        let m = basis[qi];
        qi += 1;
        for v in 0..nv {
            let nm = m | (1u64 << v);
            if nm == m || seen.contains(&nm) {
                continue;
            }
            if leads.iter().any(|&lm| nm & lm == lm) {
                continue;
            }
            seen.insert(nm);
            basis.push(nm);
            if basis.len() > 128 {
                return Err("quotient algebra larger than 128: extraction not attempted".into());
            }
        }
    }
    let d = basis.len();
    out.quotient_dim = d;
    let index: std::collections::HashMap<u64, usize> =
        basis.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let mut ops = 0u64;
    // Multiplication matrices (columns) of the x-variables.
    let mut cols: Vec<Vec<u128>> = Vec::new();
    for v in 0..nx as usize {
        let mut c = vec![0u128; d];
        for (j, &bm) in basis.iter().enumerate() {
            let nf = normal_form(bm | (1u64 << v), &gb, &mut ops)?;
            let mut col = 0u128;
            for m in nf {
                let k = *index
                    .get(&m)
                    .ok_or("normal form left the standard-monomial basis")?;
                col ^= 1u128 << k;
            }
            c[j] = col;
        }
        cols.push(c);
    }
    // Joint eigenspace splitting over all x-variables.
    let mut spaces: Vec<Vecs> = vec![(0..d).map(|i| 1u128 << i).collect()];
    for v in 0..nx as usize {
        let mut next: Vec<Vecs> = Vec::new();
        for u in &spaces {
            let im: Vec<u128> = u.iter().map(|&w| apply(&cols[v], w, &mut ops)).collect();
            let ker: Vec<u128> = u.iter().zip(&im).map(|(&w, &mw)| w ^ mw).collect();
            for part in [span_basis(&im), span_basis(&ker)] {
                if !part.is_empty() {
                    next.push(part);
                }
            }
        }
        spaces = next;
    }
    for u in &spaces {
        if u.len() != 1 {
            return Err(format!("joint eigenspace of dimension {} after splitting", u.len()));
        }
        let w = u[0];
        let mut xs = [0u64; 3];
        for v in 0..nx as usize {
            let mw = apply(&cols[v], w, &mut ops);
            let bit = if mw == w {
                1u64
            } else if mw == 0 {
                0
            } else {
                return Err("multiplication matrix not idempotent on a joint eigenvector".into());
            };
            xs[v / l as usize] |= bit << (v % l as usize);
        }
        out.roots.push(xs);
    }
    out.ext_ops = ops;
    if out.roots.len() != d {
        return Err(format!("{} joint eigenvectors for a quotient of dimension {d}", out.roots.len()));
    }
    Ok(out)
}
