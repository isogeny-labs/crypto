//! CNF + native-XOR encodings of the chained Semaev decomposition on a
//! sigma-stable factor base V, in the arms of H-BINSTD-c72284 Stage 2.
//!
//! Field elements are symbolic sums  sum_p lin_p * e_p  (lin_p a GF(2)-linear
//! form over SAT variables, e_p a constant field element), so squaring is free
//! (e_p -> e_p^2) and a product costs one AND per pair of terms. Equations are
//! emitted as n native XOR rows in the coordinates of the arm's output basis.
//!
//! Arms (all sound unless stated; the symmetry group is S_m x C_n):
//!  C        plain single instance, normal-basis V coordinates, S_m lex break
//!  CPoly    same, V coordinates and equation rows in the polynomial basis
//!  CNoBreak C without the S_m break (positive control, effect ~ m!)
//!  A        single-instance representative-and-shift: x_i = sigma^{s_i}(rho_i),
//!           rho_i a necklace-minimal orbit representative, s_i one-hot
//!  B        gauge-fixed one-hot orbit system over the n conjugate targets:
//!           target = sum_j h_j sigma^j(xR), tuple gauge-fixed by "x_1 is the
//!           lex-minimal rotation of every slot"
//!  BNoGauge B without the gauge (positive control, effect ~ n)
//!  G        C plus B's gauge comparator circuits made vacuous by free
//!           selector variables (size-matched generic control; sound)
//!  U        C plus B's gauge comparators ENFORCED against the fixed target
//!           (the literal per-instance orbit clauses; UNSOUND tripwire)

use crate::gf::*;
use std::collections::{BTreeMap, HashMap};

pub type Lit = i32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arm {
    C,
    CPoly,
    CNoBreak,
    A,
    B,
    BNoGauge,
    G,
    U,
    /// P3 positive control: C plus one unit clause per slot pinning that slot's top u-bit to 0
    /// (all m slots in the lower half of V's coordinate order). The retained fraction is at most
    /// 1/2 under any enumeration model: 1/2 of the first-summand branches if the solver enumerates
    /// one slot, 2^-m (1/4 at m = 2, 1/8 at m = 3) of the sorted solution space. UNSAT-targets only
    /// (it deliberately loses solutions on satisfiable targets).
    CHalf,
}

impl Arm {
    pub fn name(&self) -> &'static str {
        match self {
            Arm::C => "C",
            Arm::CPoly => "Cpoly",
            Arm::CNoBreak => "Cnobreak",
            Arm::A => "A",
            Arm::B => "B",
            Arm::BNoGauge => "Bnogauge",
            Arm::G => "G",
            Arm::U => "U",
            Arm::CHalf => "Chalf",
        }
    }
    pub fn parse(s: &str) -> Option<Arm> {
        [
            Arm::C,
            Arm::CPoly,
            Arm::CNoBreak,
            Arm::A,
            Arm::B,
            Arm::BNoGauge,
            Arm::G,
            Arm::U,
            Arm::CHalf,
        ]
        .into_iter()
        .find(|a| a.name() == s)
    }
}

// ---------------------------------------------------------------- Lin / Cnf

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Lin {
    pub vars: Vec<u32>,
    pub c: bool,
}

impl Lin {
    pub fn zero() -> Lin {
        Lin {
            vars: vec![],
            c: false,
        }
    }
    pub fn one() -> Lin {
        Lin {
            vars: vec![],
            c: true,
        }
    }
    pub fn var(v: u32) -> Lin {
        Lin {
            vars: vec![v],
            c: false,
        }
    }
    pub fn is_const(&self) -> bool {
        self.vars.is_empty()
    }
    pub fn is_zero(&self) -> bool {
        self.vars.is_empty() && !self.c
    }
    pub fn xor_with(&mut self, o: &Lin) {
        self.c ^= o.c;
        if o.vars.is_empty() {
            return;
        }
        let mut out = Vec::with_capacity(self.vars.len() + o.vars.len());
        let (mut i, mut j) = (0, 0);
        while i < self.vars.len() && j < o.vars.len() {
            match self.vars[i].cmp(&o.vars[j]) {
                std::cmp::Ordering::Less => {
                    out.push(self.vars[i]);
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    out.push(o.vars[j]);
                    j += 1;
                }
                std::cmp::Ordering::Equal => {
                    i += 1;
                    j += 1;
                }
            }
        }
        out.extend_from_slice(&self.vars[i..]);
        out.extend_from_slice(&o.vars[j..]);
        self.vars = out;
    }
}

#[derive(Clone, Debug, Default)]
pub struct Sizes {
    pub vars: u32,
    pub clauses: usize,
    pub clause_lits: usize,
    pub xor_rows: usize,
    pub xor_lits: usize,
    pub and_aux: usize,
    pub materialised: usize,
}

pub struct Cnf {
    pub nv: u32,
    pub clauses: Vec<Vec<Lit>>,
    pub xors: Vec<(Vec<u32>, bool)>,
    pub root_unsat: bool,
    and_cache: HashMap<(Lit, Lit), Lit>,
    mat_cache: HashMap<Vec<u32>, u32>,
    pub n_and: usize,
    pub n_mat: usize,
}

pub const TRUE_VAR: u32 = 1;

impl Cnf {
    pub fn new() -> Cnf {
        Cnf {
            nv: 1,
            clauses: vec![vec![1]],
            xors: vec![],
            root_unsat: false,
            and_cache: HashMap::new(),
            mat_cache: HashMap::new(),
            n_and: 0,
            n_mat: 0,
        }
    }
    pub fn fresh(&mut self) -> u32 {
        self.nv += 1;
        self.nv
    }
    pub fn clause(&mut self, c: Vec<Lit>) {
        self.clauses.push(c);
    }
    pub fn xor(&mut self, vars: Vec<u32>, rhs: bool) {
        if vars.is_empty() {
            if rhs {
                self.root_unsat = true;
            }
            return;
        }
        self.xors.push((vars, rhs));
    }
    pub fn lin_of_lit(&self, l: Lit) -> Lin {
        let v = l.unsigned_abs();
        if v == TRUE_VAR {
            return if l > 0 { Lin::one() } else { Lin::zero() };
        }
        Lin {
            vars: vec![v],
            c: l < 0,
        }
    }
    /// Literal equal to the linear form (a fresh XOR-defined variable if needed).
    pub fn lit_of(&mut self, l: &Lin) -> Lit {
        match l.vars.len() {
            0 => {
                if l.c {
                    TRUE_VAR as Lit
                } else {
                    -(TRUE_VAR as Lit)
                }
            }
            1 => {
                if l.c {
                    -(l.vars[0] as Lit)
                } else {
                    l.vars[0] as Lit
                }
            }
            _ => {
                let z = match self.mat_cache.get(&l.vars) {
                    Some(&z) => z,
                    None => {
                        let z = self.fresh();
                        let mut row = l.vars.clone();
                        row.push(z);
                        self.xor(row, false);
                        self.mat_cache.insert(l.vars.clone(), z);
                        self.n_mat += 1;
                        z
                    }
                };
                if l.c {
                    -(z as Lit)
                } else {
                    z as Lit
                }
            }
        }
    }
    pub fn and_lits(&mut self, a: Lit, b: Lit) -> Lit {
        let t = TRUE_VAR as Lit;
        if a == t {
            return b;
        }
        if b == t {
            return a;
        }
        if a == -t || b == -t || a == -b {
            return -t;
        }
        if a == b {
            return a;
        }
        let key = (a.min(b), a.max(b));
        if let Some(&y) = self.and_cache.get(&key) {
            return y;
        }
        let y = self.fresh() as Lit;
        self.clause(vec![-y, a]);
        self.clause(vec![-y, b]);
        self.clause(vec![y, -a, -b]);
        self.and_cache.insert(key, y);
        self.n_and += 1;
        y
    }
    pub fn and_lin(&mut self, a: &Lin, b: &Lin) -> Lin {
        if a.is_const() {
            return if a.c { b.clone() } else { Lin::zero() };
        }
        if b.is_const() {
            return if b.c { a.clone() } else { Lin::zero() };
        }
        let la = self.lit_of(a);
        let lb = self.lit_of(b);
        let y = self.and_lits(la, lb);
        self.lin_of_lit(y)
    }
    pub fn sizes(&self) -> Sizes {
        Sizes {
            vars: self.nv,
            clauses: self.clauses.len(),
            clause_lits: self.clauses.iter().map(|c| c.len()).sum(),
            xor_rows: self.xors.len(),
            xor_lits: self.xors.iter().map(|x| x.0.len()).sum(),
            and_aux: self.n_and,
            materialised: self.n_mat,
        }
    }
    /// Native-work counter of building the encoding: literals written.
    pub fn encode_ops(&self) -> u64 {
        let s = self.sizes();
        (s.clause_lits + s.xor_lits) as u64
    }
}

// ---------------------------------------------------------------- Fe

#[derive(Clone, Debug, Default)]
pub struct Fe {
    pub terms: Vec<(Lin, u64)>,
}

// ---------------------------------------------------------------- Cell

pub struct Cell {
    pub f: Field,
    pub nb: Basis,
    pub pb: Basis,
    pub n: u32,
    pub l: u32,
    pub m: usize,
    pub b: u64,
    pub g: u64,
    pub vb_n: Vec<u64>,
    pub vb_p: Vec<u64>,
    pub with_one: bool,
}

impl Cell {
    pub fn new(n: u32, l: u32, m: usize, vsel: usize, b: u64) -> Cell {
        let f = Field::new(n);
        let nb = Basis::normal(&f);
        let pb = Basis::polynomial(&f);
        let spec = vspec_for_dim(n, l, vsel)
            .unwrap_or_else(|| panic!("no sigma-stable V of dim {l} at n={n} (vsel {vsel})"));
        let (t1, fs, _) = cyclotomic_factors(n);
        let g = spec.g(t1, &fs);
        let vb_n = stable_subspace(&f, &nb, g, &nb);
        let vb_p = stable_subspace(&f, &nb, g, &pb);
        Cell {
            f,
            nb,
            pb,
            n,
            l,
            m,
            b,
            g,
            vb_n,
            vb_p,
            with_one: spec.with_t_plus_1,
        }
    }
}

// ---------------------------------------------------------------- Decode

#[derive(Clone, Debug)]
pub enum Decode {
    Plain { u: Vec<Vec<u32>>, poly: bool },
    RepShift { r: Vec<Vec<u32>>, h: Vec<Vec<u32>> },
    Orbit { u: Vec<Vec<u32>>, h: Vec<u32> },
}

pub struct Built {
    pub arm: Arm,
    pub cnf: Cnf,
    pub priority: Vec<u32>,
    pub dec: Decode,
    pub n_comparators: usize,
    /// m = 3: the free n-bit chain variable X (normal-basis coordinates); empty otherwise.
    pub xchain: Vec<u32>,
}

// ---------------------------------------------------------------- Encoder

struct Enc<'a> {
    cx: &'a Cell,
    ob: &'a Basis, // output coordinate basis for equation rows
    cnf: Cnf,
    n_cmp: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Guard {
    None,
    Free,
}

impl<'a> Enc<'a> {
    fn n(&self) -> usize {
        self.cx.n as usize
    }

    // ---- Fe algebra
    fn fe_const(&self, c: u64) -> Fe {
        if c == 0 {
            Fe::default()
        } else {
            Fe {
                terms: vec![(Lin::one(), c)],
            }
        }
    }
    fn merge(&self, items: Vec<(Lin, u64)>) -> Fe {
        let mut m: BTreeMap<u64, Lin> = BTreeMap::new();
        for (l, e) in items {
            if e == 0 || l.is_zero() {
                continue;
            }
            m.entry(e).or_default().xor_with(&l);
        }
        Fe {
            terms: m
                .into_iter()
                .filter(|(_, l)| !l.is_zero())
                .map(|(e, l)| (l, e))
                .collect(),
        }
    }
    fn fe_add(&self, a: &Fe, b: &Fe) -> Fe {
        let mut v = a.terms.clone();
        v.extend(b.terms.iter().cloned());
        self.merge(v)
    }
    fn fe_sq(&self, a: &Fe) -> Fe {
        self.merge(
            a.terms
                .iter()
                .map(|(l, e)| (l.clone(), self.cx.f.sq(*e)))
                .collect(),
        )
    }
    fn fe_coord_lins(&self, a: &Fe) -> Vec<Lin> {
        let n = self.n();
        let mut lins = vec![Lin::zero(); n];
        for (l, e) in &a.terms {
            let c = self.ob.coords(*e);
            for k in 0..n {
                if (c >> k) & 1 == 1 {
                    lins[k].xor_with(l);
                }
            }
        }
        lins
    }
    fn fe_coordinize(&self, a: &Fe) -> Fe {
        let lins = self.fe_coord_lins(a);
        Fe {
            terms: lins
                .into_iter()
                .enumerate()
                .filter(|(_, l)| !l.is_zero())
                .map(|(k, l)| (l, self.ob.vecs[k]))
                .collect(),
        }
    }
    fn fe_mul(&mut self, a: &Fe, b: &Fe) -> Fe {
        let n = self.n();
        let a2;
        let a = if a.terms.len() > n {
            a2 = self.fe_coordinize(a);
            &a2
        } else {
            a
        };
        let b2;
        let b = if b.terms.len() > n {
            b2 = self.fe_coordinize(b);
            &b2
        } else {
            b
        };
        let mut items = vec![];
        for (la, ea) in &a.terms {
            for (lb, eb) in &b.terms {
                let y = self.cnf.and_lin(la, lb);
                if y.is_zero() {
                    continue;
                }
                items.push((y, self.cx.f.mul(*ea, *eb)));
            }
        }
        self.merge(items)
    }
    /// Emit  e = 0  as n native XOR rows in the output basis.
    fn emit_zero(&mut self, e: &Fe) {
        for l in self.fe_coord_lins(e) {
            self.cnf.xor(l.vars.clone(), l.c);
        }
    }
    /// S3(x1,x2,x3) = (x1x2 + x3(x1+x2))^2 + x1x2x3 + b = 0
    fn s3_zero(&mut self, x1: &Fe, x2: &Fe, x3: &Fe) {
        let a = self.fe_mul(x1, x2);
        let s12 = self.fe_add(x1, x2);
        let t1 = self.fe_mul(x3, &s12);
        let p = self.fe_add(&a, &t1);
        let sqp = self.fe_sq(&p);
        let t2 = self.fe_mul(x3, &a);
        let mut eq = self.fe_add(&sqp, &t2);
        eq = self.fe_add(&eq, &self.fe_const(self.cx.b));
        self.emit_zero(&eq);
    }

    // ---- comparators
    /// a <=_lex b (index 0 most significant). With Guard::Free a fresh selector
    /// is appended to every violation clause (the circuit is then vacuous).
    fn lex_le(&mut self, a: &[Lit], b: &[Lit], guard: Guard) {
        self.n_cmp += 1;
        let sel: Option<Lit> = if guard == Guard::Free {
            Some(self.cnf.fresh() as Lit)
        } else {
            None
        };
        let mut e: Lit = TRUE_VAR as Lit;
        for c in 0..a.len() {
            let mut viol = vec![-e, -a[c], b[c]];
            if let Some(s) = sel {
                viol.push(s);
            }
            self.cnf.clause(viol);
            if c + 1 < a.len() {
                let e2 = self.cnf.fresh() as Lit;
                self.cnf.clause(vec![-e, a[c], b[c], e2]);
                self.cnf.clause(vec![-e, -a[c], -b[c], e2]);
                e = e2;
            }
        }
    }

    fn coord_bits(&mut self, terms: &[(Lin, u64)]) -> Vec<Lit> {
        // normal-basis coordinates of  sum lin_p e_p  as literals
        let n = self.n();
        let mut lins = vec![Lin::zero(); n];
        for (l, e) in terms {
            let c = self.cx.nb.coords(*e);
            for k in 0..n {
                if (c >> k) & 1 == 1 {
                    lins[k].xor_with(l);
                }
            }
        }
        lins.iter().map(|l| self.cnf.lit_of(l)).collect()
    }

    fn rotated(bits: &[Lit], k: usize) -> Vec<Lit> {
        let n = bits.len();
        (0..n).map(|c| bits[(c + n - k) % n]).collect()
    }

    /// Gauge: x_1 is the lex-minimal rotation of every slot; x_2 <= x_3 ... .
    fn gauge(&mut self, bits: &[Vec<Lit>], guard: Guard) {
        let n = self.n();
        for i in 0..bits.len() {
            for k in 0..n {
                if i == 0 && k == 0 {
                    continue;
                }
                let r = Self::rotated(&bits[i], k);
                let b0 = bits[0].clone();
                self.lex_le(&b0, &r, guard);
            }
        }
        for i in 1..bits.len().saturating_sub(1) {
            let (a, b) = (bits[i].clone(), bits[i + 1].clone());
            // Free for G: C's own u-lex break orders slots by V-coordinate, an
            // unrelated order; enforcing this comparator too would be unsound.
            self.lex_le(&a, &b, guard);
        }
    }

    fn one_hot(&mut self, h: &[u32]) {
        self.cnf.clause(h.iter().map(|&v| v as Lit).collect());
        for i in 0..h.len() {
            for j in (i + 1)..h.len() {
                self.cnf.clause(vec![-(h[i] as Lit), -(h[j] as Lit)]);
            }
        }
    }

    fn plain_slot(&mut self, vb: &[u64]) -> (Fe, Vec<u32>) {
        let u: Vec<u32> = (0..vb.len()).map(|_| self.cnf.fresh()).collect();
        let fe = Fe {
            terms: u.iter().zip(vb).map(|(&v, &e)| (Lin::var(v), e)).collect(),
        };
        (fe, u)
    }

    fn lex_on_vars(&mut self, groups: &[Vec<u32>]) {
        for w in groups.windows(2) {
            let a: Vec<Lit> = w[0].iter().map(|&v| v as Lit).collect();
            let b: Vec<Lit> = w[1].iter().map(|&v| v as Lit).collect();
            self.lex_le(&a, &b, Guard::None);
        }
    }

    /// representative-and-shift slot
    fn repshift_slot(&mut self) -> (Fe, Vec<u32>, Vec<u32>, Vec<Lit>) {
        let n = self.n();
        let l = self.cx.l as usize;
        let vb = self.cx.vb_n.clone();
        let r: Vec<u32> = (0..l).map(|_| self.cnf.fresh()).collect();
        let h: Vec<u32> = (0..n).map(|_| self.cnf.fresh()).collect();
        self.one_hot(&h);
        let rho_terms: Vec<(Lin, u64)> =
            r.iter().zip(&vb).map(|(&v, &e)| (Lin::var(v), e)).collect();
        let rho_bits = self.coord_bits(&rho_terms);
        for k in 1..n {
            let rk = Self::rotated(&rho_bits, k);
            self.lex_le(&rho_bits, &rk, Guard::None);
        }
        // sigma-fixed rho (0 and all-ones) must use shift 0
        let mut c0: Vec<Lit> = rho_bits.clone();
        c0.push(h[0] as Lit);
        self.cnf.clause(c0);
        let mut c1: Vec<Lit> = rho_bits.iter().map(|x| -x).collect();
        c1.push(h[0] as Lit);
        self.cnf.clause(c1);
        // x = sigma^s(rho) in V-coordinates (reduced echelon pivots)
        let piv: Vec<u32> = vb
            .iter()
            .map(|&v| pdeg(self.cx.nb.coords(v)) as u32)
            .collect();
        let mut y = vec![vec![0 as Lit; l]; n];
        for s in 0..n {
            for p in 0..l {
                y[s][p] = self.cnf.and_lits(h[s] as Lit, r[p] as Lit);
            }
        }
        let mut terms = vec![];
        for q in 0..l {
            let mut lin = Lin::zero();
            for s in 0..n {
                for p in 0..l {
                    let cp = rot(self.cx.nb.coords(vb[p]), s as u32, self.cx.n);
                    if (cp >> piv[q]) & 1 == 1 {
                        let yl = self.cnf.lin_of_lit(y[s][p]);
                        lin.xor_with(&yl);
                    }
                }
            }
            terms.push((lin, vb[q]));
        }
        (Fe { terms }, r, h, rho_bits)
    }
}

/// Build the encoding of arm `arm` for target abscissa `xr` (normal-basis
/// output rows except Cpoly). `semaev = false` builds the domain only (used by
/// the counting-identity tests).
pub fn build(arm: Arm, cx: &Cell, xr: u64, semaev: bool) -> Built {
    let ob: &Basis = if arm == Arm::CPoly { &cx.pb } else { &cx.nb };
    let mut e = Enc {
        cx,
        ob,
        cnf: Cnf::new(),
        n_cmp: 0,
    };
    let m = cx.m;
    assert!(
        m == 2 || m == 3,
        "chain encoder implemented for m in {{2,3}}"
    );
    let n = cx.n as usize;
    let vb: Vec<u64> = if arm == Arm::CPoly {
        cx.vb_p.clone()
    } else {
        cx.vb_n.clone()
    };
    let mut xs: Vec<Fe> = vec![];
    let mut priority: Vec<u32> = vec![];
    let dec;
    let mut target = e.fe_const(xr);
    match arm {
        Arm::C | Arm::CPoly | Arm::CNoBreak | Arm::G | Arm::U | Arm::CHalf => {
            let mut us = vec![];
            for _ in 0..m {
                let (fe, u) = e.plain_slot(&vb);
                priority.extend(&u);
                xs.push(fe);
                us.push(u);
            }
            if arm != Arm::CNoBreak {
                e.lex_on_vars(&us);
            }
            if arm == Arm::CHalf {
                // every slot, not only slot 1: C's lex break makes slot 1 the smallest, so a pin on
                // it alone retains 3/4 (m=2) or 7/8 (m=3) of the sorted space (review round 2).
                for u in &us {
                    e.cnf.clause(vec![-(u[0] as Lit)]);
                }
            }
            if arm == Arm::G || arm == Arm::U {
                let bits: Vec<Vec<Lit>> = xs.iter().map(|x| e.coord_bits(&x.terms)).collect();
                e.gauge(
                    &bits,
                    if arm == Arm::G {
                        Guard::Free
                    } else {
                        Guard::None
                    },
                );
            }
            dec = Decode::Plain {
                u: us,
                poly: arm == Arm::CPoly,
            };
        }
        Arm::A => {
            let mut rs = vec![];
            let mut hs = vec![];
            let mut rhos = vec![];
            for _ in 0..m {
                let (fe, r, h, rho) = e.repshift_slot();
                priority.extend(&r);
                priority.extend(&h);
                xs.push(fe);
                rs.push(r);
                hs.push(h);
                rhos.push(rho);
            }
            for w in rhos.windows(2) {
                e.lex_le(&w[0].clone(), &w[1].clone(), Guard::None);
            }
            dec = Decode::RepShift { r: rs, h: hs };
        }
        Arm::B | Arm::BNoGauge => {
            let mut us = vec![];
            for _ in 0..m {
                let (fe, u) = e.plain_slot(&vb);
                priority.extend(&u);
                xs.push(fe);
                us.push(u);
            }
            let h: Vec<u32> = (0..n).map(|_| e.cnf.fresh()).collect();
            priority.extend(&h);
            e.one_hot(&h);
            let terms: Vec<(Lin, u64)> = (0..n)
                .map(|j| (Lin::var(h[j]), cx.f.frob(xr, j as u32)))
                .collect();
            target = e.merge(terms);
            if arm == Arm::B {
                let bits: Vec<Vec<Lit>> = xs.iter().map(|x| e.coord_bits(&x.terms)).collect();
                e.gauge(&bits, Guard::None);
            } else {
                e.lex_on_vars(&us);
            }
            dec = Decode::Orbit { u: us, h };
        }
    }
    let mut xchain: Vec<u32> = vec![];
    if semaev {
        if m == 2 {
            let (a, b) = (xs[0].clone(), xs[1].clone());
            e.s3_zero(&a, &b, &target);
        } else {
            let xv: Vec<u32> = (0..n).map(|_| e.cnf.fresh()).collect();
            xchain = xv.clone();
            let xfe = Fe {
                terms: xv
                    .iter()
                    .enumerate()
                    .map(|(k, &v)| (Lin::var(v), ob.vecs[k]))
                    .collect(),
            };
            let (a, b, c) = (xs[0].clone(), xs[1].clone(), xs[2].clone());
            e.s3_zero(&a, &b, &xfe);
            e.s3_zero(&xfe, &c, &target);
        }
    }
    Built {
        arm,
        n_comparators: e.n_cmp,
        cnf: e.cnf,
        priority,
        dec,
        xchain,
    }
}

impl Built {
    /// Load into the repo CDCL solver. `perm_seed` renames variables (A/A and
    /// replicate seeds). Returns the solver and pi (pi[v] = solver variable of v).
    pub fn load(&self, perm_seed: u64) -> (crate::satlib::Solver, Vec<u32>) {
        let nv = self.cnf.nv;
        let mut pi: Vec<u32> = (0..=nv).collect();
        if perm_seed != 0 {
            let mut rng = crate::curve::Rng(perm_seed);
            for i in (2..=nv as usize).rev() {
                let j = 2 + rng.below((i - 1) as u64) as usize;
                pi.swap(i, j);
            }
        }
        let mp = |l: Lit| -> Lit {
            let v = pi[l.unsigned_abs() as usize] as Lit;
            if l < 0 {
                -v
            } else {
                v
            }
        };
        let mut s = crate::satlib::Solver::new(nv);
        for c in &self.cnf.clauses {
            s.add_clause(c.iter().map(|&l| mp(l)).collect());
        }
        for (vars, rhs) in &self.cnf.xors {
            let vs: Vec<u32> = vars.iter().map(|&v| pi[v as usize]).collect();
            s.add_xor(&vs, *rhs);
        }
        if self.cnf.root_unsat {
            s.add_clause(vec![]);
        }
        let pr: Vec<u32> = self.priority.iter().map(|&v| pi[v as usize]).collect();
        s.set_branch_priority(&pr);
        s.stats.encode_word_ops += self.cnf.encode_ops();
        (s, pi)
    }

    fn get(model: &[bool], pi: &[u32], v: u32) -> bool {
        model[(pi[v as usize] - 1) as usize]
    }

    /// Decode the tuple (as a PLAIN solution for target xr) and check it
    /// independently of the encoding: membership in V and the S_{m+1} chain.
    pub fn verify(
        &self,
        cx: &Cell,
        model: &[bool],
        pi: &[u32],
        xr: u64,
    ) -> Result<Vec<u64>, String> {
        let f = &cx.f;
        let vb = |poly: bool| if poly { &cx.vb_p } else { &cx.vb_n };
        let one_hot = |h: &[u32]| -> Result<usize, String> {
            let idx: Vec<usize> = (0..h.len())
                .filter(|&i| Self::get(model, pi, h[i]))
                .collect();
            if idx.len() == 1 {
                Ok(idx[0])
            } else {
                Err(format!("one-hot violated: {} true", idx.len()))
            }
        };
        let xs: Vec<u64> = match &self.dec {
            Decode::Plain { u, poly } => u
                .iter()
                .map(|us| {
                    us.iter().zip(vb(*poly)).fold(0u64, |a, (&v, &e)| {
                        if Self::get(model, pi, v) {
                            a ^ e
                        } else {
                            a
                        }
                    })
                })
                .collect(),
            Decode::RepShift { r, h } => {
                let mut v = vec![];
                for (rs, hs) in r.iter().zip(h) {
                    let rho = rs.iter().zip(&cx.vb_n).fold(0u64, |a, (&x, &e)| {
                        if Self::get(model, pi, x) {
                            a ^ e
                        } else {
                            a
                        }
                    });
                    v.push(f.frob(rho, one_hot(hs)? as u32));
                }
                v
            }
            Decode::Orbit { u, h } => {
                let j = one_hot(h)? as u32;
                u.iter()
                    .map(|us| {
                        let x = us.iter().zip(&cx.vb_n).fold(0u64, |a, (&v, &e)| {
                            if Self::get(model, pi, v) {
                                a ^ e
                            } else {
                                a
                            }
                        });
                        f.frob(x, cx.n - j) // sigma^-j: solution of the j-th conjugate target -> target 0
                    })
                    .collect()
            }
        };
        let vm = VMember::new(cx.n, cx.g);
        for &x in &xs {
            if !vm.contains_coords(cx.nb.coords(x)) {
                return Err("decoded x outside V".into());
            }
        }
        let ok = if cx.m == 2 {
            f.s3(xs[0], xs[1], xr, cx.b) == 0
        } else {
            let (bb, c) = (f.mul(xs[0], xs[1]), f.sq(xs[0] ^ xs[1]));
            let roots = f
                .solve_quadratic(c, bb, f.sq(bb) ^ cx.b)
                .unwrap_or_default();
            roots.iter().any(|&xx| f.s3(xx, xs[2], xr, cx.b) == 0)
        };
        if ok {
            Ok(xs)
        } else {
            Err("decoded tuple does not solve the plain target".into())
        }
    }
}
