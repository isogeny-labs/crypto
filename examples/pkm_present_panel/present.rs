//! The six presentations of one PKM Kummer-tower point-decomposition instance.
//!
//! Instance: curve `y^2 = x^3 + a x + b` over `F_p`, base `V = g * mu_{2^t}` (abscissae
//! `x = g zeta^j`), target abscissa `x_R`, `m` base points (signed) summing to `R`.
//!
//! Presentations (`[pcc:unverified]`, instrument construction, no claim):
//!  1. `chain-tower`: S_3 chain (toms.py order, `system::build`) + tower `y_{i,j+1} = y_{i,j}^2`, `y_{i,t-1}^2 = 1`.
//!  2. `tree-tower`: balanced binary merge tree of S_3 boxes (`m-2` wires) + the same tower.
//!  3. `mono-tower`: the single `S_{m+1}(x_0..x_{m-1}, x_R)` (every wire eliminated by a Sylvester
//!     resultant with formal degrees, NO content or scalar removed) + the tower.
//!  4. `chain-direct`: the S_3 chain over `x_i`, membership `x_i^(2^t) = g^(2^t)` (no `y`).
//!  5. `mono-direct`: `S_{m+1}` + the direct membership equations.
//!  6. `sym-direct`: `S_{m+1}` rewritten in `e_1..e_m` plus the `m` coefficients of
//!     `X^(2^t) - c mod P(X)` (`c = g^(2^t)`): `P` divides `X^(2^t) - c`.
//!  7. `sym-tower`: the construction of `examples/pkm_symmetric_tower.rs` (unknowns `e_k` of
//!     `Q(X) = prod(X - w_i)`, `x_i = g w_i`, and the coefficients of `R_j = X^(2^j) mod Q`,
//!     `j = 1..t-1`; equations `R_j = R_{j-1}^2 mod Q`, `R_{t-1}^2 = 1 mod Q`, and `S_{m+1}(g w, x_R)`
//!     rewritten in the `e_k`).
//!
//! Solution semantics (also in every `.system.json` as `solution_semantics`):
//!  * tower and direct forms of one arrangement are exactly equivalent over `F_p`
//!    (`y_{i,0} = x_i/g`; `y^(2^t) = 1  <=>  x^(2^t) = g^(2^t)`).
//!  * chain-* and tree-*: ORDERED tuples with `F_p` wires. A box `S_3(A,B,C)` vanishes iff the
//!    abscissae obey `C = x(+-P_A +- P_B)` over `F_pbar`. An identity partial sum (`P + (-P)`) has no finite
//!    abscissa, so tuples whose fixed-order prefix cancels are absent (their reorderings are present);
//!    chain and tree miss different orderings. All `x_i in V` have rational partial sums otherwise.
//!  * mono-*: ORDERED tuples, symmetric in `x_0..x_{m-1}` (all permutations are zeros). The wires are
//!    eliminated, so a zero can have wires in `F_pbar \ F_p` and identity partial sums (both leading
//!    coefficients vanish): monolithic forms can admit extra algebraic tuples.
//!  * sym-direct and sym-tower: UNORDERED (permutations identified, solution count is the unordered one) and the
//!    divisibility `P | X^(2^t) - c` forces `P` squarefree (`c != 0`): decompositions with a repeated abscissa
//!    (`2 P_i`, `P_i + (-P_i)`) are NOT representable. Over `F_p`, `X^(2^t) - c` splits completely
//!    (`zeta in F_p`), so every `F_p`-rational `e`-vector is an `m`-subset of `V`. It shares mono's extra
//!    algebraic solutions.
//!  * Lifting (all arms): `x_i` must be a real base abscissa (twist points of `V` do not lift); the
//!    decomposition is the set of sign patterns whose point sum is exactly `R` (verified by counted additions).
use crate::algebra::{self, s3, Pol};
use crate::mpoly;
use crate::system;
use crate::util::{self, invm, mulm, seal};
use serde_json::{json, Value};

pub const PRESENTATIONS: [&str; 7] =
    ["chain-tower", "tree-tower", "mono-tower", "chain-direct", "mono-direct", "sym-direct", "sym-tower"];

pub fn is_symmetric(pres: &str) -> bool {
    pres.starts_with("sym-")
}

pub type Terms = Vec<(Vec<u32>, u64)>;

#[derive(Clone, Debug)]
pub struct Inst {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub g: u64,
    pub zeta: u64,
    pub m: usize,
    pub t: usize,
    pub x_r: u64,
}

/// How a solution assignment decodes to the multiset of abscissae `x_i`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Decode {
    /// `x_i = scale * var[idx[i]]` (`scale = g` for towers, 1 for direct).
    Vars { idx: Vec<usize>, scale: u64 },
    /// `e_1..e_m` at `idx`; the `x_i` are `scale *` the roots of `P(X)` (`scale = g` for sym-tower).
    Elem { idx: Vec<usize>, scale: u64 },
}

/// Construction work in field multiplications (the `algebra.rs` counter: every `Pol::mul` and
/// `Pol::scale`, including the `p-1` scaling inside `Pol::sub`) and process CPU seconds,
/// split into what depends only on the cell and what depends on the target.
#[derive(Clone, Copy, Default, Debug)]
pub struct Cost {
    pub cell_muls: u64,
    pub cell_cpu: f64,
    pub target_muls: u64,
    pub target_cpu: f64,
}

pub struct Sys {
    pub presentation: String,
    pub inst: Inst,
    pub vars: Vec<String>,
    pub polys: Vec<Terms>,
    /// polynomials before the tower equations (`polys.len() - head` tower equations, 0 for other forms)
    pub head: usize,
    pub tower: bool,
    pub decode: Decode,
    /// for each wire, the leaves it sums (planted assignments)
    pub wire_leaves: Vec<Vec<usize>>,
    pub dropped_zero_polys: usize,
    pub cost: Cost,
}

#[derive(Clone, Copy)]
enum Arg {
    Leaf(usize),
    Wire(usize),
}

struct BoxS {
    a: Arg,
    b: Arg,
    out: Option<usize>, // None = x_R
}

/// Merge structure: chain (`chain = true`) or balanced tree; returns boxes and the leaves under each wire.
fn boxes(m: usize, chain: bool) -> (Vec<BoxS>, Vec<Vec<usize>>) {
    let mut bx: Vec<BoxS> = Vec::new();
    let mut wl: Vec<Vec<usize>> = Vec::new();
    if m == 2 {
        bx.push(BoxS { a: Arg::Leaf(0), b: Arg::Leaf(1), out: None });
        return (bx, wl);
    }
    if chain {
        for k in 0..m - 2 {
            let a = if k == 0 { Arg::Leaf(0) } else { Arg::Wire(k - 1) };
            let b = Arg::Leaf(k + 1);
            bx.push(BoxS { a, b, out: Some(k) });
            wl.push((0..k + 2).collect());
        }
        bx.push(BoxS { a: Arg::Wire(m - 3), b: Arg::Leaf(m - 1), out: None });
        return (bx, wl);
    }
    let mut items: Vec<(Arg, Vec<usize>)> = (0..m).map(|i| (Arg::Leaf(i), vec![i])).collect();
    while items.len() > 2 {
        let mut next: Vec<(Arg, Vec<usize>)> = Vec::new();
        let mut it = items.into_iter();
        while let Some(x) = it.next() {
            let Some(y) = it.next() else {
                next.push(x);
                break;
            };
            let k = wl.len();
            let mut lv = x.1.clone();
            lv.extend(y.1.iter());
            bx.push(BoxS { a: x.0, b: y.0, out: Some(k) });
            wl.push(lv.clone());
            next.push((Arg::Wire(k), lv));
        }
        items = next;
    }
    bx.push(BoxS { a: items[0].0, b: items[1].0, out: None });
    (bx, wl)
}

pub fn sorted_terms(f: &Pol) -> Terms {
    system::sorted_terms(f)
}

fn unit(n: usize, i: usize, k: u32) -> Vec<u32> {
    let mut e = vec![0; n];
    e[i] = k;
    e
}

fn tower_polys(n: usize, m: usize, t: usize, p: u64) -> Vec<Terms> {
    let mut out = Vec::new();
    for i in 0..m {
        for j in 0..t {
            let tail = if j + 1 < t { (unit(n, i * t + j + 1, 1), 1) } else { (vec![0; n], 1) };
            let mut f = vec![(unit(n, i * t + j, 2), p - 1), tail];
            f.sort_by(|x, y| y.0.cmp(&x.0));
            out.push(f);
        }
    }
    out
}

fn membership_polys(n: usize, m: usize, t: usize, g: u64, p: u64) -> Vec<Terms> {
    let big = 1u64 << t;
    let c = algebra::powm(g, big, p);
    (0..m)
        .map(|i| {
            let mut f = vec![(unit(n, i, big as u32), 1u64)];
            if c != 0 {
                f.push((vec![0; n], p - c));
            }
            f.sort_by(|x, y| y.0.cmp(&x.0));
            f
        })
        .collect()
}

fn names_tower(m: usize, t: usize, wires: usize) -> Vec<String> {
    let mut v: Vec<String> = (0..m).flat_map(|i| (0..t).map(move |j| format!("y{i}_{j}"))).collect();
    v.extend((0..wires).map(|k| format!("u{k}")));
    v
}
fn names_direct(m: usize, wires: usize) -> Vec<String> {
    let mut v: Vec<String> = (0..m).map(|i| format!("x{i}")).collect();
    v.extend((0..wires).map(|k| format!("u{k}")));
    v
}
fn names_sym_tower(m: usize, t: usize) -> Vec<String> {
    let mut v: Vec<String> = (1..=m).map(|k| format!("e{k}")).collect();
    for j in 1..t {
        v.extend((0..m).map(|k| format!("r{j}_{k}")));
    }
    v
}

fn s3_chain_polys(s: &Inst, bx: &[BoxS], n: usize, leaf: &dyn Fn(usize) -> Pol, wire: &dyn Fn(usize) -> Pol) -> Vec<Terms> {
    let arg = |a: Arg| match a {
        Arg::Leaf(i) => leaf(i),
        Arg::Wire(k) => wire(k),
    };
    bx.iter()
        .map(|b| {
            let out = match b.out {
                Some(k) => wire(k),
                None => Pol::c(n, s.x_r, s.p),
            };
            sorted_terms(&s3(&arg(b.a), &arg(b.b), &out, s.a, s.b, s.p))
        })
        .collect()
}

fn strip_zero(polys: Vec<Terms>) -> (Vec<Terms>, usize) {
    let before = polys.len();
    let kept: Vec<Terms> = polys.into_iter().filter(|f| !f.is_empty()).collect();
    let d = before - kept.len();
    (kept, d)
}

/// Run `f`, returning its value, the counted field multiplications and the CPU seconds it took.
fn metered<T>(f: impl FnOnce() -> T) -> (T, u64, f64) {
    let (m0, c0) = (algebra::counts(), util::cpu_secs());
    let v = f();
    (v, algebra::counts().since(m0).multiplications, util::cpu_secs() - c0)
}

/// Pieces of the construction that do not depend on the target (`x_R`), each built once and
/// metered once. `S_{m+1}` is kept split by the power of `x_R`.
pub struct CellPre {
    m: usize,
    a: u64,
    b: u64,
    g: u64,
    p: u64,
    t: usize,
    sx: Option<(Vec<Pol>, u64, f64)>,
    se: Option<(Vec<Pol>, u64, f64)>,
    rem: Option<(Vec<Pol>, u64, f64)>,
    stower: Option<(Vec<Pol>, u64, f64)>,
}

impl CellPre {
    pub fn new(i: &Inst) -> CellPre {
        CellPre { m: i.m, a: i.a, b: i.b, g: i.g, p: i.p, t: i.t, sx: None, se: None, rem: None, stower: None }
    }
    fn sx(&mut self) -> &Vec<Pol> {
        if self.sx.is_none() {
            let (m, a, b, p) = (self.m, self.a, self.b, self.p);
            let (v, mu, cpu) = metered(|| mpoly::summation_by_last(m, a, b, p));
            self.sx = Some((v, mu, cpu));
        }
        &self.sx.as_ref().expect("set").0
    }
    fn se(&mut self) -> &Vec<Pol> {
        if self.se.is_none() {
            let _ = self.sx();
            let (m, p) = (self.m, self.p);
            let parts = self.sx.as_ref().expect("set").0.clone();
            let (v, mu, cpu) = metered(|| {
                parts.iter().map(|q| mpoly::to_elementary(q, m, p)).collect::<Result<Vec<_>, _>>()
            });
            let v = v.expect("S_(m+1) is symmetric in its first m arguments");
            self.se = Some((v, mu, cpu));
        }
        &self.se.as_ref().expect("set").0
    }
    fn rem(&mut self) -> &Vec<Pol> {
        if self.rem.is_none() {
            let (m, t, p) = (self.m, self.t, self.p);
            let (v, mu, cpu) = metered(|| mpoly::x_pow_2t_mod(m, t, p));
            self.rem = Some((v, mu, cpu));
        }
        &self.rem.as_ref().expect("set").0
    }
    fn stower(&mut self) -> &Vec<Pol> {
        if self.stower.is_none() {
            let (m, t, p) = (self.m, self.t, self.p);
            let (v, mu, cpu) = metered(|| mpoly::sym_tower_equations(m, t, p));
            self.stower = Some((v, mu, cpu));
        }
        &self.stower.as_ref().expect("set").0
    }
    /// Per-cell cost of the pieces an arm depends on.
    fn cost_of(&self, pres: &str) -> (u64, f64) {
        let get = |o: &Option<(Vec<Pol>, u64, f64)>| o.as_ref().map_or((0, 0.0), |x| (x.1, x.2));
        let add = |v: &[(u64, f64)]| v.iter().fold((0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));
        match pres {
            "mono-tower" | "mono-direct" => get(&self.sx),
            "sym-direct" => add(&[get(&self.sx), get(&self.se), get(&self.rem)]),
            "sym-tower" => add(&[get(&self.sx), get(&self.se), get(&self.stower)]),
            _ => (0, 0.0),
        }
    }
}

pub fn build(pres: &str, s: &Inst) -> Result<Sys, String> {
    build_pre(&mut CellPre::new(s), pres, s)
}

pub fn build_pre(pre: &mut CellPre, pres: &str, s: &Inst) -> Result<Sys, String> {
    let (m, t) = (s.m, s.t);
    if m < 2 || t == 0 || t > 12 {
        return Err("need m >= 2 and 1 <= t <= 12".into());
    }
    if m > 5 && (pres.starts_with("mono") || is_symmetric(pres)) {
        return Err("monolithic and symmetric presentations are implemented for m <= 5".into());
    }
    // make the per-cell pieces first so the per-target meter below excludes them
    match pres {
        "mono-tower" | "mono-direct" => {
            pre.sx();
        }
        "sym-direct" => {
            pre.se();
            pre.rem();
        }
        "sym-tower" => {
            pre.se();
            pre.stower();
        }
        _ => {}
    }
    let (res, tmuls, tcpu) = metered(|| build_target(pre, pres, s));
    let mut sys = res?;
    let (cm, cc) = pre.cost_of(pres);
    sys.cost = Cost { cell_muls: cm, cell_cpu: cc, target_muls: tmuls, target_cpu: tcpu };
    Ok(sys)
}

fn build_target(pre: &mut CellPre, pres: &str, s: &Inst) -> Result<Sys, String> {
    let (p, m, t) = (s.p, s.m, s.t);
    let wires = m - 2;
    let mk = |vars: Vec<String>, polys: Vec<Terms>, head: usize, tower: bool, decode: Decode, wl: Vec<Vec<usize>>, dz: usize| Sys {
        presentation: pres.into(),
        inst: s.clone(),
        vars,
        polys,
        head,
        tower,
        decode,
        wire_leaves: wl,
        dropped_zero_polys: dz,
        cost: Cost::default(),
    };
    let ydec = || Decode::Vars { idx: (0..m).map(|i| i * t).collect(), scale: s.g };
    let xdec = || Decode::Vars { idx: (0..m).collect(), scale: 1 };
    match pres {
        "chain-tower" => {
            let spec = system::Spec { p, a: s.a, b: s.b, g: s.g, zeta: s.zeta, m, t, x_r: s.x_r };
            let (polys, _n) = system::build(&spec);
            let head = polys.len() - m * t;
            let (_, wl) = boxes(m, true);
            Ok(mk(names_tower(m, t, wires), polys, head, true, ydec(), wl, 0))
        }
        "tree-tower" => {
            let n = m * t + wires;
            let (bx, wl) = boxes(m, false);
            let leaf = |i: usize| Pol::var(n, i * t).scale(s.g, p);
            let wire = |k: usize| Pol::var(n, m * t + k);
            let mut polys = s3_chain_polys(s, &bx, n, &leaf, &wire);
            let head = polys.len();
            polys.extend(tower_polys(n, m, t, p));
            Ok(mk(names_tower(m, t, wires), polys, head, true, ydec(), wl, 0))
        }
        "mono-tower" => {
            let n = m * t;
            let sm = mpoly::eval_last(pre.sx(), s.x_r, p);
            let mut f = Pol::zero(n);
            for (e, &c) in &sm.t {
                let mut e2 = vec![0u32; n];
                let mut cc = c;
                for (i, &k) in e.iter().enumerate() {
                    e2[i * t] = k;
                    cc = algebra::mulm(cc, algebra::powm(s.g, u64::from(k), p), p);
                }
                f.t.insert(e2, cc);
            }
            let mut polys = vec![sorted_terms(&f)];
            let head = polys.len();
            polys.extend(tower_polys(n, m, t, p));
            let (polys, dz) = strip_zero(polys);
            Ok(mk(names_tower(m, t, 0), polys, head, true, ydec(), vec![], dz))
        }
        "chain-direct" => {
            let n = m + wires;
            let (bx, wl) = boxes(m, true);
            let leaf = |i: usize| Pol::var(n, i);
            let wire = |k: usize| Pol::var(n, m + k);
            let mut polys = s3_chain_polys(s, &bx, n, &leaf, &wire);
            let head = polys.len();
            polys.extend(membership_polys(n, m, t, s.g, p));
            Ok(mk(names_direct(m, wires), polys, head, false, xdec(), wl, 0))
        }
        "mono-direct" => {
            let sm = mpoly::eval_last(pre.sx(), s.x_r, p);
            let mut polys = vec![sorted_terms(&sm)];
            let head = polys.len();
            polys.extend(membership_polys(m, m, t, s.g, p));
            let (polys, dz) = strip_zero(polys);
            Ok(mk(names_direct(m, 0), polys, head, false, xdec(), vec![], dz))
        }
        "sym-direct" => {
            let se = mpoly::eval_last(pre.se(), s.x_r, p);
            let c = algebra::powm(s.g, 1u64 << t, p);
            let mut polys = vec![sorted_terms(&se)];
            let head = polys.len();
            for (k, r) in pre.rem().iter().enumerate() {
                let mut f = r.clone();
                if k == 0 {
                    mpoly::add_in(&mut f, &Pol::c(m, p - c, p), p);
                }
                polys.push(sorted_terms(&f));
            }
            let (polys, dz) = strip_zero(polys);
            Ok(mk((1..=m).map(|k| format!("e{k}")).collect(), polys, head, false, Decode::Elem { idx: (0..m).collect(), scale: 1 }, vec![], dz))
        }
        "sym-tower" => {
            let n = m * t;
            let se = mpoly::eval_last(pre.se(), s.x_r, p);
            // x_i = g w_i: e_k(x) = g^k e_k(w); a monomial with exponents a_k gets g^(sum k a_k)
            let mut f = Pol::zero(n);
            for (e, &c) in &se.t {
                let mut e2 = vec![0u32; n];
                e2[..m].copy_from_slice(e);
                let w: u64 = e.iter().enumerate().map(|(k, &a)| (k as u64 + 1) * u64::from(a)).sum();
                f.t.insert(e2, algebra::mulm(c, algebra::powm(s.g, w, p), p));
            }
            let mut polys = vec![sorted_terms(&f)];
            let head = polys.len();
            polys.extend(pre.stower().iter().map(sorted_terms));
            let (polys, dz) = strip_zero(polys);
            Ok(mk(names_sym_tower(m, t), polys, head, false, Decode::Elem { idx: (0..m).collect(), scale: s.g }, vec![], dz))
        }
        other => Err(format!("unknown presentation {other}")),
    }
}

impl Sys {
    pub fn nvars(&self) -> usize {
        self.vars.len()
    }
    pub fn max_input_degree(&self) -> u32 {
        self.polys.iter().map(total_degree).max().unwrap_or(0)
    }
    pub fn nterms_total(&self) -> usize {
        self.polys.iter().map(Vec::len).sum()
    }
}

pub fn total_degree(f: &Terms) -> u32 {
    f.iter().map(|(e, _)| e.iter().sum::<u32>()).max().unwrap_or(0)
}

/// `.ms` text (variable line, characteristic, equations). Tower equations are written as
/// toms.py / `system::ms_text` does (`1*y_{j+1}+(p-1)*y_j^2`, `1+(p-1)*y_{t-1}^2`); all other
/// equations term by term, terms by descending exponent vector.
pub fn ms_text(sys: &Sys) -> String {
    let p = sys.inst.p;
    let names = &sys.vars;
    let mut out = names.join(",");
    out.push('\n');
    out.push_str(&p.to_string());
    out.push('\n');
    let mut eqs: Vec<String> = Vec::new();
    for f in &sys.polys[..sys.head] {
        eqs.push(f.iter().map(|(e, c)| system::term_text(e, *c, names)).collect::<Vec<_>>().join("+"));
    }
    if sys.tower {
        let (m, t) = (sys.inst.m, sys.inst.t);
        for i in 0..m {
            for j in 0..t.saturating_sub(1) {
                eqs.push(format!("1*y{i}_{}+{}*y{i}_{j}^2", j + 1, p - 1));
            }
            eqs.push(format!("1+{}*y{i}_{}^2", p - 1, t - 1));
        }
    } else {
        for f in &sys.polys[sys.head..] {
            eqs.push(f.iter().map(|(e, c)| system::term_text(e, *c, names)).collect::<Vec<_>>().join("+"));
        }
    }
    out.push_str(&eqs.join(",\n"));
    out.push('\n');
    out
}

pub fn semantics_class(pres: &str) -> &'static str {
    match pres {
        "chain-tower" | "chain-direct" | "tree-tower" => "ordered_repeats",
        "mono-tower" | "mono-direct" => "ordered_repeats_unlifted",
        _ => "unordered_distinct",
    }
}

pub fn semantics(pres: &str) -> Value {
    let (ordered, wires, extra, missing) = match pres {
        "chain-tower" | "chain-direct" => (
            true,
            "m-2 F_p wires: partial-sum abscissae of the fixed left-to-right order",
            "none beyond twist points of V (x_i in V not on the real base), which do not lift",
            "tuples whose fixed-order prefix is an identity partial sum (P + (-P)); a multiset is absent altogether when every ordering has one (observed: chain misses none of the smoke targets, but see tree-tower)",
        ),
        "tree-tower" => (
            true,
            "m-2 F_p wires: abscissae of the subtree sums of the fixed balanced tree",
            "none beyond twist points of V, which do not lift",
            "tuples whose tree-order subtree sums are identity partial sums; a multiset is absent altogether when every pairing has one (observed at m = 4: {-P,-P,-P,+P} for R = -2P, every pairing puts +P with a -P), so tree-tower can miss real decompositions that chain-tower finds",
        ),
        "mono-tower" | "mono-direct" => (
            true,
            "no wires: every internal wire eliminated by a Sylvester resultant (formal degrees, nothing divided out)",
            "tuples whose internal partial sums are not F_p-rational (wires in F_pbar) and identity partial sums (leading coefficients both vanish)",
            "none: symmetric in x_0..x_{m-1}, all permutations are zeros",
        ),
        "sym-tower" => (
            false,
            "no wires; unordered; unknowns e_k of Q(X) = prod(X - w_i), x_i = g w_i, and the coefficients of R_j = X^(2^j) mod Q (j = 1..t-1)",
            "the monolithic extras (non-rational or identity partial sums)",
            "decompositions with a repeated abscissa (Q | X^(2^t) - 1 forces Q squarefree): only DISTINCT abscissae are seen",
        ),
        _ => (
            false,
            "no wires; unordered: permutations identified, solution count is the unordered one",
            "the monolithic extras (non-rational or identity partial sums)",
            "decompositions with a repeated abscissa (P | X^(2^t) - c forces P squarefree): only DISTINCT abscissae are seen, e.g. 2*P_i or P_i + (-P_i) are not representable",
        ),
    };
    json!({
        "ordered": ordered,
        "wires": wires,
        "extra_algebraic_solutions": extra,
        "not_representable": missing,
        "classifier_set": if is_symmetric(pres) { "real_count_distinct (pairwise distinct abscissae)" } else { "real_count" },
        "tower_vs_direct": "exactly equivalent over F_p: y_{i,0} = x_i/g, y^(2^t) = 1 iff x^(2^t) = g^(2^t)",
        "exact_on": "F_p-rational decompositions with pairwise distinct abscissae and no identity partial sums in the arm's fixed order",
        "lifting": "x_i must be a real base abscissa; decomposition = sign patterns whose point sum is exactly R, verified by counted point additions",
    })
}

/// Full assignment of every variable for a decomposition given as ordered points
/// `(abscissa, ordinate)` (signed), or `None` if a partial sum is the identity.
pub fn planted_assignment(sys: &Sys, pts: &[(u64, u64)]) -> Option<Vec<u64>> {
    use crate::ec::add_opt;
    let i = &sys.inst;
    let p = i.p;
    let m = i.m;
    let sum = |leaves: &[usize]| -> Option<(u64, u64)> {
        leaves.iter().fold(None, |acc, &l| add_opt(p, i.a, acc, Some(pts[l])))
    };
    let xs: Vec<u64> = pts.iter().map(|q| q.0).collect();
    let mut asg = vec![0u64; sys.nvars()];
    match &sys.decode {
        Decode::Elem { idx, scale } => {
            let ginv = invm(*scale, p);
            let w: Vec<u64> = xs.iter().map(|&x| mulm(x, ginv, p)).collect();
            if sys.presentation == "sym-tower" {
                return Some(mpoly::sym_tower_point(&w, i.t, p));
            }
            let e = mpoly::elem_values(&w, p);
            for (k, &v) in idx.iter().zip(e.iter()) {
                asg[*k] = v;
            }
            return Some(asg);
        }
        Decode::Vars { idx, scale } => {
            let ginv = invm(*scale, p);
            for (l, &x) in xs.iter().enumerate() {
                if sys.tower {
                    let mut y = mulm(x, ginv, p);
                    for j in 0..i.t {
                        asg[idx[l] + j] = y;
                        y = mulm(y, y, p);
                    }
                } else {
                    asg[idx[l]] = x;
                }
            }
        }
    }
    let base = if sys.tower { m * i.t } else { m };
    for (k, lv) in sys.wire_leaves.iter().enumerate() {
        asg[base + k] = sum(lv)?.0;
    }
    Some(asg)
}

pub fn eval_terms(f: &Terms, x: &[u64], p: u64) -> u64 {
    use util::{addm, powm};
    f.iter().fold(0, |acc, (e, c)| {
        let mut v = *c;
        for (i, &k) in e.iter().enumerate() {
            if k > 0 {
                v = mulm(v, powm(x[i], u64::from(k), p), p);
            }
        }
        addm(acc, v, p)
    })
}

/// Polynomials as the `system.json` `polys` field.
pub fn polys_json(polys: &[Terms]) -> Value {
    Value::Array(polys.iter().map(|f| Value::Array(f.iter().map(|(e, c)| json!([e, c])).collect())).collect())
}

/// Sealed `system.json`: a superset of `system::system_json` (same keys and values for
/// `p, a, b, g, zeta, m, t, x_r, variables, chain_polys, polys`) plus the arm's metadata.
pub fn system_json(sys: &Sys, verify_data: &Value) -> Value {
    let i = &sys.inst;
    let decode = match &sys.decode {
        Decode::Vars { idx, scale } => json!({"kind": "vars", "idx": idx, "scale": scale}),
        Decode::Elem { idx, scale } => json!({"kind": "elem", "idx": idx, "scale": scale}),
    };
    let term_counts: Vec<usize> = sys.polys.iter().map(Vec::len).collect();
    let c = &sys.cost;
    seal(json!({
        "schema": "pkm.present-panel.system/v1",
        "presentation": sys.presentation,
        "p": i.p, "a": i.a, "b": i.b, "g": i.g, "zeta": i.zeta, "m": i.m, "t": i.t, "x_r": i.x_r,
        "variables": sys.vars,
        "nvars": sys.nvars(), "nequations": sys.polys.len(),
        "chain_polys": sys.head,
        "head_polys": sys.head, "tower_equations": sys.polys.len() - sys.head,
        "max_input_degree": sys.max_input_degree(), "max_total_degree": sys.max_input_degree(),
        "nterms_total": sys.nterms_total(),
        "term_counts": term_counts,
        "construction_field_multiplications": c.target_muls,
        "construction_cpu_s": c.target_cpu,
        "construction_cell_field_multiplications": c.cell_muls,
        "construction_cell_cpu_s": c.cell_cpu,
        "construction": {
            "per_cell": {"field_multiplications": c.cell_muls, "cpu_s": c.cell_cpu,
                "what": "independent of the target: S_{m+1} by resultants (split by power of x_R), its elementary-symmetric rewrite, X^(2^t) mod P, the R_j tower"},
            "per_target": {"field_multiplications": c.target_muls, "cpu_s": c.target_cpu,
                "what": "substituting x_R, scaling by powers of g, S_3 boxes, tower and membership equations"},
            "unit": "field multiplications counted by examples/pkm_oracle_program/algebra.rs (every Pol::mul and Pol::scale, including the p-1 scaling inside Pol::sub); additions not counted; per_cell is shared by every target of the cell and is metered once per export process",
        },
        "dropped_zero_polys": sys.dropped_zero_polys,
        "decode": decode,
        "wire_leaves": sys.wire_leaves,
        "solution_semantics": semantics_class(&sys.presentation),
        "solution_semantics_detail": semantics(&sys.presentation),
        "verify_data": verify_data,
        "doc": "[pcc:unverified] instrument construction; no claim",
        "polys": polys_json(&sys.polys),
    }))
}
