//! `FastCdcl`: a copy of `LinCdcl` (examples/pdp_lin_cdcl.rs, sha256
//! 996770f4b4ecbc84c451203f6edce5cb17ac40b44892def19d06ca63bc0d2f47) with a different XOR engine.
//! Same constraint language (clauses, XOR rows, product gates z = x * y over F_{2^n}), same
//! search skeleton (two watched literals, first-UIP learning on every conflict, every learnt
//! clause kept, priority/activity/lowest-index branching, saved phase, no restarts by default).
//! What is decided is unchanged; how the linear part is propagated and explained is not.
//!
//! CHANGES against the original (every one of them):
//!  C1  incremental Gauss-Jordan. One reduced row-echelon matrix over ALL variables per
//!      decision level. A decision copies the current level's matrix to the next level
//!      (checkpoint stack, the undo trail); a backtrack drops levels. The original rebuilt and
//!      re-eliminated every active row at every propagation fixpoint.
//!  C2  assignments enter the matrix as unit rows (v = c) for literals that the XOR engine did
//!      not imply itself (decisions, clause propagations, level-0 facts). A variable is implied
//!      exactly when its pivot row has no other variable. The original substituted assigned
//!      values while rebuilding.
//!  C3  bit-packed rows: vw words of variables plus 1 mask word; a row operation XORs only the
//!      word range where the source row is non-zero. A column index (for each variable, the
//!      bitset of rows containing it) is kept by column operations, so eliminating a new pivot
//!      visits only the rows that contain it.
//!  C4  explanations are cut at unit literals and built lazily. Each literal that enters as a
//!      unit row above level 0 gets a slot (bit) in the mask word; bit 63 means "more than 63
//!      slots: every earlier non-implied literal". Row masks are combined by OR, and a gate
//!      row's mask is the OR of the masks of every assigned operand bit it used. So the mask
//!      of an implied literal is a superset of the decisions and clause-propagated literals
//!      that support it, which is a sound reason. The literal list is produced only inside
//!      conflict analysis, each slot at most once per analysis. The original's reason was
//!      every assigned literal occurring in the rows of an exact row combination plus the
//!      gate guards, built eagerly at every propagation; its learnt clauses therefore also
//!      contain implied literals of lower levels, where these contain decisions and
//!      clause-propagated literals only.
//!  C5  the level-0 matrix (static rows, independent of x1) is eliminated once and inherited
//!      by every level through the copies.
//!  C6  gate linearisation folds values. With known operand a of z = a * y:
//!      (i)  y fully assigned: z = a * y by one field multiplication, n unit rows;
//!      (ii) a != 0 and z fully assigned: y = z / a by one field inversion and multiplication,
//!           n unit rows (the same row space as the n gate rows reduced by z's unit rows);
//!      (iii) otherwise the n rows z_k + sum_j M(a)_kj y_j with the assigned y_j substituted.
//!      The original always injected the n unsubstituted rows and re-eliminated.
//!  C7  pivot of a new row: its highest-index variable (`pivot_high`, default) instead of the
//!      original's lowest-index order.
//!  C8  add_clause only runs clause propagation at build time; the XOR engine starts in solve().
//!  C9  watch lists are compacted in place (no allocation per visited literal).
//! NOT changed: clause learning scheme (first UIP, all clauses kept), backjumping, branching
//! rule, phase saving, gate semantics, verdict semantics. Not implemented from the candidate
//! list: Gray-code order over x1 and a learning-free leaf (both change the search itself).

#![allow(dead_code)]

use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Val {
    Undef,
    True,
    False,
}

#[derive(Clone, Copy, Debug)]
enum Reason {
    None,
    Clause(usize),
    /// Implied by the XOR engine; the mask is in `fact`.
    Xor,
}

#[derive(Clone, Debug)]
pub struct XorRow {
    pub vars: Vec<u32>,
    pub rhs: bool,
}

pub struct Gate {
    pub x: Vec<u32>,
    pub y: Vec<u32>,
    pub z: Vec<u32>,
    linearised_at: Option<u32>,
    x_run: bool,
    y_run: bool,
}

pub const N_PROF: usize = 7;
pub const PROF_NAMES: [&str; N_PROF] =
    ["clause_prop", "unit_rows", "gate_rows", "analyze", "backtrack", "pick_branch", "matrix_copy"];

#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub conflicts: u64,
    pub decisions: u64,
    pub propagations: u64,
    pub xor_propagations: u64,
    pub gate_linearisations: u64,
    pub gate_forward: u64,
    pub gate_divide: u64,
    pub gate_mixed: u64,
    pub learnt_clauses: u64,
    pub learnt_lits: u64,
    pub learnt_priority_width: Vec<u64>,
    pub row_inserts: u64,
    /// row operations (row ^= row) and the words they touched (variable range + mask word)
    pub row_xors: u64,
    pub row_xor_words: u64,
    /// column-index operations (col ^= col) and their words
    pub col_xors: u64,
    pub col_xor_words: u64,
    pub mat_copies: u64,
    pub mat_copy_words: u64,
    pub unit_overflows: u64,
    pub prof_ns: [u64; N_PROF],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Result {
    Sat,
    Unsat,
    Unknown,
}

const OVERFLOW: u64 = 1u64 << 63;
const NONE: u32 = u32::MAX;

enum Confl {
    /// literals, all false
    Clause(Vec<i32>),
    /// derivation mask, plus an optional TRUE literal that the derived row contradicts
    Xor(u64, Option<i32>),
}

#[derive(Clone, Default)]
struct Mat {
    /// nrows * (vw + 1) words: variables, then the mask word
    rows: Vec<u64>,
    rhs: Vec<bool>,
    /// non-zero word range [lo, hi) of the variable part of each row
    rng: Vec<(u16, u16)>,
    single: Vec<bool>,
    nrows: usize,
    piv_of_var: Vec<u32>,
    piv_var: Vec<u32>,
    pivmask: Vec<u64>,
    /// nv * vw words: for each variable, the rows that contain it
    cols: Vec<u64>,
}

enum Ins {
    Redundant,
    Conflict(u64),
    Added,
}

impl Mat {
    fn new(nv: usize, vw: usize) -> Mat {
        Mat {
            rows: Vec::new(),
            rhs: Vec::new(),
            rng: Vec::new(),
            single: Vec::new(),
            nrows: 0,
            piv_of_var: vec![NONE; nv],
            piv_var: Vec::new(),
            pivmask: vec![0; vw],
            cols: vec![0; nv * vw],
        }
    }
    fn copy_from(&mut self, src: &Mat) {
        self.rows.clear();
        self.rows.extend_from_slice(&src.rows);
        self.rhs.clear();
        self.rhs.extend_from_slice(&src.rhs);
        self.rng.clear();
        self.rng.extend_from_slice(&src.rng);
        self.single.clear();
        self.single.extend_from_slice(&src.single);
        self.nrows = src.nrows;
        self.piv_of_var.copy_from_slice(&src.piv_of_var);
        self.piv_var.clear();
        self.piv_var.extend_from_slice(&src.piv_var);
        self.pivmask.copy_from_slice(&src.pivmask);
        self.cols.copy_from_slice(&src.cols);
    }
    fn words(&self) -> usize {
        self.rows.len() + self.cols.len()
    }
}

/// dst ^= src on the variable words [lo, hi); the mask word is OR-ed.
#[inline(always)]
fn xor_row(dst: &mut [u64], src: &[u64], lo: usize, hi: usize, vw: usize) {
    let (d, s) = (&mut dst[..vw + 1], &src[..vw + 1]);
    for (a, b) in d[lo..hi].iter_mut().zip(&s[lo..hi]) {
        *a ^= *b;
    }
    d[vw] |= s[vw];
}

#[inline(always)]
fn is_single(row: &[u64], lo: usize, hi: usize) -> bool {
    // exactly one bit set in the range (no popcount: the default x86-64 target has none)
    let (mut one, mut more) = (0u64, false);
    for &w in &row[lo..hi] {
        if w != 0 {
            more |= one != 0;
            one = w;
        }
    }
    !more && one != 0 && one & (one - 1) == 0
}

pub struct FastCdcl {
    pub n_vars: usize,
    pub field_n: u32,
    pub irr: u64,
    assigns: Vec<Val>,
    level: Vec<u32>,
    reason: Vec<Reason>,
    /// derivation mask of each assigned variable (its row's mask, or its own unit slot)
    fact: Vec<u64>,
    trail: Vec<i32>,
    trail_lim: Vec<usize>,
    qhead: usize,
    xhead: usize,
    clauses: Vec<Vec<i32>>,
    n_original_clauses: usize,
    watches: Vec<Vec<usize>>,
    xors: Vec<XorRow>,
    gates: Vec<Gate>,
    priority: Vec<bool>,
    activity: Vec<f64>,
    var_inc: f64,
    phase: Vec<bool>,
    seen: Vec<bool>,
    pub conflict_budget: u64,
    pub stats: Stats,
    pub restarts: bool,
    /// C7: pivot = highest-index variable of the reduced row (false: lowest)
    pub pivot_high: bool,
    pub pivot_high_static: bool,
    ok: bool,
    // XOR engine
    vw: usize,
    rw: usize,
    mats: Vec<Mat>,
    tmp: Vec<u64>,
    tmp_rhs: bool,
    colbuf: Vec<u64>,
    units_out: Vec<u32>,
    unit_lits: Vec<i32>,
    unit_lim: Vec<usize>,
    started: bool,
    rb_key: u64,
    rb: [u64; 64],
}

fn lit_code(l: i32) -> usize {
    let v = l.unsigned_abs() as usize;
    2 * v + (l < 0) as usize
}

fn is_run(b: &[u32]) -> bool {
    b.iter().enumerate().all(|(j, &v)| v == b[0] + j as u32)
}

fn deg(a: u64) -> i32 {
    63 - a.leading_zeros() as i32
}

impl FastCdcl {
    pub fn new(n_vars: usize, field_n: u32, irr: u64) -> Self {
        let nv = n_vars + 1;
        let vw = nv.div_ceil(64);
        assert!(vw < 65535);
        FastCdcl {
            n_vars,
            field_n,
            irr,
            assigns: vec![Val::Undef; nv],
            level: vec![0; nv],
            reason: vec![Reason::None; nv],
            fact: vec![0; nv],
            trail: Vec::new(),
            trail_lim: Vec::new(),
            qhead: 0,
            xhead: 0,
            clauses: Vec::new(),
            n_original_clauses: 0,
            watches: vec![Vec::new(); 2 * nv + 2],
            xors: Vec::new(),
            gates: Vec::new(),
            priority: vec![false; nv],
            activity: vec![0.0; nv],
            var_inc: 1.0,
            phase: vec![false; nv],
            seen: vec![false; nv],
            conflict_budget: u64::MAX,
            stats: Stats::default(),
            restarts: false,
            pivot_high: true,
            pivot_high_static: true,
            ok: true,
            vw,
            rw: vw + 1,
            mats: vec![Mat::new(nv, vw)],
            tmp: vec![0; vw + 1],
            tmp_rhs: false,
            colbuf: vec![0; vw],
            units_out: Vec::new(),
            unit_lits: Vec::new(),
            unit_lim: Vec::new(),
            started: false,
            rb_key: 0,
            rb: [0; 64],
        }
    }

    pub fn row_words(&self) -> usize {
        self.rw
    }

    // ---- field helpers (n <= 62, polynomial basis, irr includes the x^n term) ----

    pub fn gf_mul(&self, a: u64, b: u64) -> u64 {
        let n = self.field_n;
        let mut r = 0u64;
        let mut a = a;
        for i in 0..n {
            if (b >> i) & 1 == 1 {
                r ^= a;
            }
            a <<= 1;
            if (a >> n) & 1 == 1 {
                a ^= self.irr;
            }
        }
        r
    }

    /// Inverse by the binary extended Euclid (a != 0).
    pub fn gf_inv(&self, a: u64) -> u64 {
        assert!(a != 0);
        let (mut u, mut v) = (a, self.irr);
        let (mut g1, mut g2) = (1u64, 0u64);
        while u != 1 {
            let mut j = deg(u) - deg(v);
            if j < 0 {
                std::mem::swap(&mut u, &mut v);
                std::mem::swap(&mut g1, &mut g2);
                j = -j;
            }
            u ^= v << j;
            g1 ^= g2 << j;
        }
        g1
    }

    /// Column `j` of the multiplication-by-`a` matrix: `a * z^j`.
    pub fn mul_matrix(&self, a: u64) -> Vec<u64> {
        (0..self.field_n).map(|j| self.gf_mul(a, 1u64 << j)).collect()
    }

    /// Column `j` of the squaring matrix: `z^{2j}`.
    pub fn sq_matrix(&self) -> Vec<u64> {
        (0..self.field_n).map(|j| self.gf_mul(1u64 << j, 1u64 << j)).collect()
    }

    // ---- constraint builders ----

    pub fn set_priority(&mut self, vars: &[u32]) {
        for &v in vars {
            self.priority[v as usize] = true;
        }
    }

    pub fn add_clause(&mut self, lits: Vec<i32>) -> bool {
        assert!(!self.started);
        if !self.ok {
            return false;
        }
        let mut lits = lits;
        lits.sort_unstable();
        lits.dedup();
        let mut out = Vec::new();
        for &l in &lits {
            if lits.contains(&-l) {
                return true;
            }
            match self.value(l) {
                Val::True => return true,
                Val::False => {}
                Val::Undef => out.push(l),
            }
        }
        match out.len() {
            0 => {
                self.ok = false;
                false
            }
            1 => {
                self.enqueue(out[0], Reason::None);
                self.ok = self.propagate_clauses().is_none();
                self.ok
            }
            _ => {
                let idx = self.clauses.len();
                self.watches[lit_code(out[0])].push(idx);
                self.watches[lit_code(out[1])].push(idx);
                self.clauses.push(out);
                self.n_original_clauses += 1;
                true
            }
        }
    }

    pub fn add_xor(&mut self, vars: Vec<u32>, rhs: bool) {
        assert!(!self.started);
        let mut vars = vars;
        vars.sort_unstable();
        let mut out: Vec<u32> = Vec::new();
        for v in vars {
            if out.last() == Some(&v) {
                out.pop();
            } else {
                out.push(v);
            }
        }
        self.xors.push(XorRow { vars: out, rhs });
    }

    /// Linear rows `z_k + sum_j M_kj y_j + c_k = 0` for a fixed matrix (columns as words).
    pub fn add_linear_map(&mut self, z: &[u32], y: &[u32], cols: &[u64], constant: u64) {
        let n = self.field_n as usize;
        for k in 0..n {
            let mut vars = vec![z[k]];
            for j in 0..n {
                if (cols[j] >> k) & 1 == 1 {
                    vars.push(y[j]);
                }
            }
            self.add_xor(vars, (constant >> k) & 1 == 1);
        }
    }

    pub fn add_gate(&mut self, x: Vec<u32>, y: Vec<u32>, z: Vec<u32>) {
        let (x_run, y_run) = (is_run(&x), is_run(&y));
        self.gates.push(Gate { x, y, z, linearised_at: None, x_run, y_run });
    }

    // ---- assignment ----

    #[inline(always)]
    fn value(&self, l: i32) -> Val {
        match self.assigns[l.unsigned_abs() as usize] {
            Val::Undef => Val::Undef,
            Val::True => {
                if l > 0 {
                    Val::True
                } else {
                    Val::False
                }
            }
            Val::False => {
                if l > 0 {
                    Val::False
                } else {
                    Val::True
                }
            }
        }
    }

    #[inline(always)]
    fn decision_level(&self) -> u32 {
        self.trail_lim.len() as u32
    }

    #[inline(always)]
    fn enqueue(&mut self, l: i32, r: Reason) {
        let v = l.unsigned_abs() as usize;
        debug_assert!(self.assigns[v] == Val::Undef);
        self.assigns[v] = if l > 0 { Val::True } else { Val::False };
        self.level[v] = self.decision_level();
        self.reason[v] = r;
        self.trail.push(l);
    }

    fn block_value(&self, block: &[u32]) -> Option<u64> {
        let mut w = 0u64;
        for (j, &v) in block.iter().enumerate() {
            match self.assigns[v as usize] {
                Val::Undef => return None,
                Val::True => w |= 1u64 << j,
                Val::False => {}
            }
        }
        Some(w)
    }

    // ---- propagation ----

    fn propagate_clauses(&mut self) -> Option<Vec<i32>> {
        while self.qhead < self.trail.len() {
            let l = self.trail[self.qhead];
            self.qhead += 1;
            self.stats.propagations += 1;
            let false_lit = -l;
            let code = lit_code(false_lit);
            if self.watches[code].is_empty() {
                continue;
            }
            let mut ws = std::mem::take(&mut self.watches[code]);
            let mut conflict: Option<Vec<i32>> = None;
            let (mut i, mut j) = (0, 0);
            while i < ws.len() {
                let ci = ws[i];
                i += 1;
                if self.clauses[ci][0] == false_lit {
                    self.clauses[ci].swap(0, 1);
                }
                let first = self.clauses[ci][0];
                if self.value(first) == Val::True {
                    ws[j] = ci;
                    j += 1;
                    continue;
                }
                let mut found = false;
                for k in 2..self.clauses[ci].len() {
                    let lk = self.clauses[ci][k];
                    if self.value(lk) != Val::False {
                        self.clauses[ci].swap(1, k);
                        self.watches[lit_code(lk)].push(ci);
                        found = true;
                        break;
                    }
                }
                if found {
                    continue;
                }
                ws[j] = ci;
                j += 1;
                if self.value(first) == Val::False {
                    conflict = Some(self.clauses[ci].clone());
                    while i < ws.len() {
                        ws[j] = ws[i];
                        j += 1;
                        i += 1;
                    }
                } else {
                    self.enqueue(first, Reason::Clause(ci));
                }
            }
            ws.truncate(j);
            self.watches[code] = ws;
            if let Some(c) = conflict {
                return Some(c);
            }
        }
        None
    }

    /// Insert (`self.tmp`, `self.tmp_rhs`) into the current level's matrix, keeping it in
    /// reduced row-echelon form. Rows that end with exactly one variable go to `units_out`.
    fn insert_tmp(&mut self) -> Ins {
        let (vw, rw) = (self.vw, self.rw);
        let lvl = self.trail_lim.len();
        let m = &mut self.mats[lvl];
        let tmp = &mut self.tmp;
        let mut rhs = self.tmp_rhs;
        self.stats.row_inserts += 1;
        // reduce by existing pivots (a pivot row holds no other pivot variable)
        for w in 0..vw {
            let mut b = tmp[w] & m.pivmask[w];
            while b != 0 {
                let v = w * 64 + b.trailing_zeros() as usize;
                b &= b - 1;
                let r = m.piv_of_var[v] as usize;
                let (lo, hi) = m.rng[r];
                xor_row(tmp, &m.rows[r * rw..(r + 1) * rw], lo as usize, hi as usize, vw);
                rhs ^= m.rhs[r];
                self.stats.row_xors += 1;
                self.stats.row_xor_words += (hi - lo) as u64 + 1;
            }
        }
        // non-zero range and pivot
        let mut lo = 0;
        while lo < vw && tmp[lo] == 0 {
            lo += 1;
        }
        if lo == vw {
            if rhs {
                return Ins::Conflict(tmp[vw]);
            }
            return Ins::Redundant;
        }
        let mut hi = vw;
        while tmp[hi - 1] == 0 {
            hi -= 1;
        }
        let p = if (lvl > 0 && self.pivot_high) || (lvl == 0 && self.pivot_high_static) {
            (hi - 1) * 64 + 63 - tmp[hi - 1].leading_zeros() as usize
        } else {
            lo * 64 + tmp[lo].trailing_zeros() as usize
        };
        // eliminate p from the rows that contain it (column index)
        let cw = vw;
        let (mut klo, mut khi) = (cw, 0);
        for k in 0..cw {
            let c = m.cols[p * cw + k];
            self.colbuf[k] = c;
            if c != 0 {
                if klo == cw {
                    klo = k;
                }
                khi = k + 1;
            }
        }
        if klo < khi {
            for k in klo..khi {
                m.cols[p * cw + k] = 0;
            }
            for w in lo..hi {
                let mut b = tmp[w];
                while b != 0 {
                    let v = w * 64 + b.trailing_zeros() as usize;
                    b &= b - 1;
                    if v != p {
                        let col = &mut m.cols[v * cw..(v + 1) * cw];
                        for (a, c) in col[klo..khi].iter_mut().zip(&self.colbuf[klo..khi]) {
                            *a ^= *c;
                        }
                        self.stats.col_xors += 1;
                        self.stats.col_xor_words += (khi - klo) as u64;
                    }
                }
            }
            for k in klo..khi {
                let mut b = self.colbuf[k];
                while b != 0 {
                    let r = k * 64 + b.trailing_zeros() as usize;
                    b &= b - 1;
                    let row = &mut m.rows[r * rw..(r + 1) * rw];
                    xor_row(row, tmp, lo, hi, vw);
                    m.rhs[r] ^= rhs;
                    let (rl, rh) = m.rng[r];
                    let (nl, nh) = ((rl as usize).min(lo), (rh as usize).max(hi));
                    m.rng[r] = (nl as u16, nh as u16);
                    self.stats.row_xors += 1;
                    self.stats.row_xor_words += (hi - lo) as u64 + 1;
                    if is_single(row, nl, nh) {
                        m.single[r] = true;
                        self.units_out.push(r as u32);
                    }
                }
            }
        }
        let r = m.nrows;
        let (rk, rb) = (r / 64, 1u64 << (r % 64));
        for w in lo..hi {
            let mut b = tmp[w];
            while b != 0 {
                let v = w * 64 + b.trailing_zeros() as usize;
                b &= b - 1;
                m.cols[v * cw + rk] |= rb;
            }
        }
        m.rows.extend_from_slice(tmp);
        m.rhs.push(rhs);
        m.rng.push((lo as u16, hi as u16));
        m.nrows += 1;
        m.piv_var.push(p as u32);
        m.piv_of_var[p] = r as u32;
        m.pivmask[p / 64] |= 1u64 << (p % 64);
        if is_single(tmp, lo, hi) {
            m.single.push(true);
            self.units_out.push(r as u32);
        } else {
            m.single.push(false);
        }
        Ins::Added
    }

    /// Enqueue the literals of the rows in `units_out`.
    fn drain_units(&mut self) -> Option<Confl> {
        let (vw, rw) = (self.vw, self.rw);
        let lvl = self.trail_lim.len();
        let mut out = None;
        for i in 0..self.units_out.len() {
            let r = self.units_out[i] as usize;
            let m = &self.mats[lvl];
            let v = m.piv_var[r] as usize;
            let val = m.rhs[r];
            let mk = m.rows[r * rw + vw];
            let lit = if val { v as i32 } else { -(v as i32) };
            match self.assigns[v] {
                Val::Undef => {
                    self.fact[v] = mk;
                    self.enqueue(lit, Reason::Xor);
                    self.stats.xor_propagations += 1;
                }
                a => {
                    if (a == Val::True) != val {
                        // the assigned literal -lit is true and contradicts the row
                        out = Some(Confl::Xor(mk, Some(-lit)));
                        break;
                    }
                }
            }
        }
        self.units_out.clear();
        out
    }

    fn insert_and_drain(&mut self) -> Option<Confl> {
        match self.insert_tmp() {
            Ins::Conflict(mk) => {
                self.units_out.clear();
                Some(Confl::Xor(mk, None))
            }
            _ => self.drain_units(),
        }
    }

    /// Insert the unit row `v = val` with derivation mask `mk`.
    fn insert_single(&mut self, v: usize, val: bool, mk: u64) -> Option<Confl> {
        let (vw, rw) = (self.vw, self.rw);
        let lvl = self.trail_lim.len();
        let (w, b) = (v / 64, 1u64 << (v % 64));
        let m = &mut self.mats[lvl];
        let r = m.piv_of_var[v];
        if r != NONE && m.single[r as usize] {
            // already a unit row: redundant or contradictory
            let r = r as usize;
            if m.rhs[r] == val {
                return None;
            }
            return Some(Confl::Xor(m.rows[r * rw + vw] | mk, None));
        }
        if r == NONE && m.cols[v * vw..(v + 1) * vw].iter().all(|&c| c == 0) {
            // variable in no row: append the unit row directly
            self.stats.row_inserts += 1;
            let r = m.nrows;
            let base = m.rows.len();
            m.rows.resize(base + rw, 0);
            m.rows[base + w] = b;
            m.rows[base + vw] = mk;
            m.rhs.push(val);
            m.rng.push((w as u16, w as u16 + 1));
            m.single.push(true);
            m.nrows += 1;
            m.piv_var.push(v as u32);
            m.piv_of_var[v] = r as u32;
            m.pivmask[w] |= b;
            m.cols[v * vw + r / 64] |= 1u64 << (r % 64);
            self.units_out.push(r as u32);
            return self.drain_units();
        }
        for k in 0..rw {
            self.tmp[k] = 0;
        }
        self.tmp[w] = b;
        self.tmp[vw] = mk;
        self.tmp_rhs = val;
        self.insert_and_drain()
    }

    fn start_engine(&mut self) -> Option<Confl> {
        self.started = true;
        let rw = self.rw;
        for i in 0..self.xors.len() {
            for w in 0..rw {
                self.tmp[w] = 0;
            }
            for &v in &self.xors[i].vars {
                self.tmp[v as usize / 64] ^= 1u64 << (v % 64);
            }
            self.tmp_rhs = self.xors[i].rhs;
            if let Some(c) = self.insert_and_drain() {
                return Some(c);
            }
        }
        None
    }

    /// OR of the derivation masks of the assigned variables of a block.
    fn block_fact(&self, block: &[u32]) -> u64 {
        let mut mk = 0u64;
        for &v in block {
            if self.assigns[v as usize] != Val::Undef {
                mk |= self.fact[v as usize];
            }
        }
        mk
    }

    fn linearise(&mut self, gi: usize, known: u64, side: u8) -> Option<Confl> {
        let n = self.field_n as usize;
        self.gates[gi].linearised_at = Some(self.decision_level());
        self.stats.gate_linearisations += 1;
        let (kn, other, run) = {
            let g = &self.gates[gi];
            if side == 0 { (g.x.clone(), g.y.clone(), g.y_run) } else { (g.y.clone(), g.x.clone(), g.x_run) }
        };
        let z: Vec<u32> = self.gates[gi].z.clone();
        let guard = self.block_fact(&kn);
        // (i) forward product
        if let Some(yv) = self.block_value(&other) {
            self.stats.gate_forward += 1;
            let zv = self.gf_mul(known, yv);
            let mk = guard | self.block_fact(&other);
            for k in 0..n {
                if let Some(c) = self.insert_single(z[k] as usize, (zv >> k) & 1 == 1, mk) {
                    return Some(c);
                }
            }
            return None;
        }
        // (ii) division
        if known != 0 {
            if let Some(zv) = self.block_value(&z) {
                self.stats.gate_divide += 1;
                let yv = self.gf_mul(zv, self.gf_inv(known));
                let mk = guard | self.block_fact(&z);
                for j in 0..n {
                    if let Some(c) = self.insert_single(other[j] as usize, (yv >> j) & 1 == 1, mk) {
                        return Some(c);
                    }
                }
                return None;
            }
        }
        // (iii) rows with the assigned bits of the other operand substituted
        self.stats.gate_mixed += 1;
        if self.rb_key != known || known == 0 {
            // rb[k] bit j = coefficient k of known * z^j
            self.rb = [0u64; 64];
            let mut c = known;
            for j in 0..n {
                let mut b = c;
                while b != 0 {
                    self.rb[b.trailing_zeros() as usize] |= 1u64 << j;
                    b &= b - 1;
                }
                c <<= 1;
                if (c >> n) & 1 == 1 {
                    c ^= self.irr;
                }
            }
            self.rb_key = known;
        }
        let (mut am, mut av) = (0u64, 0u64);
        for j in 0..n {
            match self.assigns[other[j] as usize] {
                Val::Undef => {}
                Val::True => {
                    am |= 1u64 << j;
                    av |= 1u64 << j;
                }
                Val::False => am |= 1u64 << j,
            }
        }
        let mk = guard | self.block_fact(&other);
        let (vw, rw) = (self.vw, self.rw);
        for k in 0..n {
            for w in 0..rw {
                self.tmp[w] = 0;
            }
            let bits = self.rb[k] & !am;
            if run {
                let base = other[0] as usize;
                let (w, s) = (base / 64, base % 64);
                self.tmp[w] ^= bits << s;
                if s != 0 && (bits >> (64 - s)) != 0 {
                    self.tmp[w + 1] ^= bits >> (64 - s);
                }
            } else {
                let mut b = bits;
                while b != 0 {
                    let v = other[b.trailing_zeros() as usize] as usize;
                    b &= b - 1;
                    self.tmp[v / 64] ^= 1u64 << (v % 64);
                }
            }
            let zv = z[k] as usize;
            self.tmp[zv / 64] ^= 1u64 << (zv % 64);
            self.tmp[vw] = mk;
            self.tmp_rhs = (self.rb[k] & av).count_ones() & 1 == 1;
            if let Some(c) = self.insert_and_drain() {
                return Some(c);
            }
        }
        None
    }

    fn propagate(&mut self) -> Option<Confl> {
        loop {
            let t0 = Instant::now();
            let c = self.propagate_clauses();
            self.stats.prof_ns[0] += t0.elapsed().as_nanos() as u64;
            if let Some(c) = c {
                return Some(Confl::Clause(c));
            }
            // unit rows for literals the XOR engine did not imply itself
            let t0 = Instant::now();
            while self.xhead < self.trail.len() {
                let lit = self.trail[self.xhead];
                self.xhead += 1;
                let v = lit.unsigned_abs() as usize;
                if matches!(self.reason[v], Reason::Xor) {
                    continue;
                }
                let mut mk = 0u64;
                if self.level[v] > 0 {
                    let slot = self.unit_lits.len();
                    self.unit_lits.push(lit);
                    if slot < 63 {
                        mk = 1u64 << slot;
                    } else {
                        mk = OVERFLOW;
                        self.stats.unit_overflows += 1;
                    }
                }
                self.fact[v] = mk;
                if let Some(c) = self.insert_single(v, lit > 0, mk) {
                    self.stats.prof_ns[1] += t0.elapsed().as_nanos() as u64;
                    return Some(c);
                }
            }
            self.stats.prof_ns[1] += t0.elapsed().as_nanos() as u64;
            if self.qhead < self.trail.len() {
                continue;
            }
            // gate linearisation
            let t0 = Instant::now();
            let mut added = false;
            for gi in 0..self.gates.len() {
                if self.gates[gi].linearised_at.is_some() {
                    continue;
                }
                let (known, side) = match self.block_value(&self.gates[gi].x) {
                    Some(a) => (a, 0u8),
                    None => match self.block_value(&self.gates[gi].y) {
                        Some(b) => (b, 1u8),
                        None => continue,
                    },
                };
                added = true;
                if let Some(c) = self.linearise(gi, known, side) {
                    self.stats.prof_ns[2] += t0.elapsed().as_nanos() as u64;
                    return Some(c);
                }
                if self.qhead < self.trail.len() {
                    // let clause propagation catch up first
                    break;
                }
            }
            self.stats.prof_ns[2] += t0.elapsed().as_nanos() as u64;
            if !added && self.qhead == self.trail.len() {
                return None;
            }
        }
    }

    // ---- search ----

    fn pick_branch(&mut self) -> Option<i32> {
        let mut best: Option<(bool, f64, usize)> = None;
        for v in 1..=self.n_vars {
            if self.assigns[v] != Val::Undef {
                continue;
            }
            let key = (self.priority[v], self.activity[v], v);
            match best {
                None => best = Some(key),
                Some(b) => {
                    if (key.0, key.1) > (b.0, b.1) || ((key.0, key.1) == (b.0, b.1) && key.2 < b.2) {
                        best = Some(key);
                    }
                }
            }
        }
        best.map(|(_, _, v)| if self.phase[v] { v as i32 } else { -(v as i32) })
    }

    fn backtrack(&mut self, lvl: u32) {
        if self.decision_level() <= lvl {
            return;
        }
        let start = self.trail_lim[lvl as usize];
        for i in (start..self.trail.len()).rev() {
            let l = self.trail[i];
            let v = l.unsigned_abs() as usize;
            self.phase[v] = l > 0;
            self.assigns[v] = Val::Undef;
            self.reason[v] = Reason::None;
        }
        self.trail.truncate(start);
        self.trail_lim.truncate(lvl as usize);
        self.qhead = self.trail.len();
        self.xhead = self.trail.len();
        self.unit_lits.truncate(self.unit_lim[lvl as usize]);
        self.unit_lim.truncate(lvl as usize);
        for g in &mut self.gates {
            if g.linearised_at.is_some_and(|l| l > lvl) {
                g.linearised_at = None;
            }
        }
    }

    fn bump(&mut self, v: usize) {
        self.activity[v] += self.var_inc;
        if self.activity[v] > 1e100 {
            for a in &mut self.activity {
                *a *= 1e-100;
            }
            self.var_inc *= 1e-100;
        }
    }

    #[inline(always)]
    fn visit(&mut self, q: i32, cur_level: u32, path: &mut usize, learnt: &mut Vec<i32>) {
        // q is a false literal
        let v = q.unsigned_abs() as usize;
        if !self.seen[v] && self.level[v] > 0 {
            self.seen[v] = true;
            self.bump(v);
            if self.level[v] >= cur_level {
                *path += 1;
            } else {
                learnt.push(q);
            }
        }
    }

    /// Visit the unit literals of a derivation mask that were not expanded yet in this
    /// analysis. `before` is the trail position of the implied literal (overflow case).
    fn visit_mask(
        &mut self,
        mk: u64,
        before: usize,
        done: &mut u64,
        ovf_upto: &mut usize,
        cur_level: u32,
        path: &mut usize,
        learnt: &mut Vec<i32>,
    ) {
        let mut b = mk & !*done & !OVERFLOW;
        *done |= b;
        while b != 0 {
            let slot = b.trailing_zeros() as usize;
            b &= b - 1;
            let lit = self.unit_lits[slot];
            self.visit(-lit, cur_level, path, learnt);
        }
        if mk & OVERFLOW != 0 && before > *ovf_upto {
            // sound fallback: every earlier literal the XOR engine did not imply itself
            for i in *ovf_upto..before {
                let lit = self.trail[i];
                if !matches!(self.reason[lit.unsigned_abs() as usize], Reason::Xor) {
                    self.visit(-lit, cur_level, path, learnt);
                }
            }
            *ovf_upto = before;
        }
    }

    /// First-UIP learning.
    fn analyze(&mut self, confl: Confl) -> (Vec<i32>, u32) {
        let mut learnt: Vec<i32> = vec![0];
        let mut path = 0usize;
        let mut idx = self.trail.len();
        let cur_level = self.decision_level();
        let mut done = 0u64;
        let mut ovf_upto = 0usize;
        match confl {
            Confl::Clause(c) => {
                for &q in &c {
                    self.visit(q, cur_level, &mut path, &mut learnt);
                }
            }
            Confl::Xor(mk, extra) => {
                let end = self.trail.len();
                self.visit_mask(mk, end, &mut done, &mut ovf_upto, cur_level, &mut path, &mut learnt);
                if let Some(t) = extra {
                    self.visit(-t, cur_level, &mut path, &mut learnt);
                }
            }
        }
        assert!(path > 0, "conflict without a literal of the current level");
        loop {
            loop {
                idx -= 1;
                if self.seen[self.trail[idx].unsigned_abs() as usize] {
                    break;
                }
            }
            let lit = self.trail[idx];
            let v = lit.unsigned_abs() as usize;
            self.seen[v] = false;
            path -= 1;
            if path == 0 {
                learnt[0] = -lit;
                break;
            }
            match self.reason[v] {
                Reason::Clause(ci) => {
                    for k in 0..self.clauses[ci].len() {
                        let q = self.clauses[ci][k];
                        if q != lit {
                            self.visit(q, cur_level, &mut path, &mut learnt);
                        }
                    }
                }
                Reason::Xor => {
                    let mk = self.fact[v];
                    self.visit_mask(mk, idx, &mut done, &mut ovf_upto, cur_level, &mut path, &mut learnt);
                }
                Reason::None => unreachable!("decision on the implication path"),
            }
        }
        for &l in &learnt {
            self.seen[l.unsigned_abs() as usize] = false;
        }
        let mut bt = 0;
        if learnt.len() > 1 {
            let mut max_i = 1;
            for i in 2..learnt.len() {
                if self.level[learnt[i].unsigned_abs() as usize] > self.level[learnt[max_i].unsigned_abs() as usize] {
                    max_i = i;
                }
            }
            learnt.swap(1, max_i);
            bt = self.level[learnt[1].unsigned_abs() as usize];
        }
        (learnt, bt)
    }

    pub fn solve(&mut self) -> Result {
        if !self.ok {
            return Result::Unsat;
        }
        if self.start_engine().is_some() || self.propagate().is_some() {
            self.ok = false;
            return Result::Unsat;
        }
        let mut conflicts_since_restart = 0u64;
        let mut restart_limit = 100u64;
        loop {
            if let Some(confl) = self.propagate() {
                self.stats.conflicts += 1;
                conflicts_since_restart += 1;
                if self.decision_level() == 0 {
                    self.ok = false;
                    return Result::Unsat;
                }
                let t0 = Instant::now();
                let (learnt, bt) = self.analyze(confl);
                self.stats.prof_ns[3] += t0.elapsed().as_nanos() as u64;
                let t0 = Instant::now();
                self.backtrack(bt);
                let pw = learnt.iter().filter(|&&l| self.priority[l.unsigned_abs() as usize]).count();
                if self.stats.learnt_priority_width.len() <= pw {
                    self.stats.learnt_priority_width.resize(pw + 1, 0);
                }
                self.stats.learnt_priority_width[pw] += 1;
                self.stats.learnt_clauses += 1;
                self.stats.learnt_lits += learnt.len() as u64;
                let assert_lit = learnt[0];
                if learnt.len() == 1 {
                    self.enqueue(assert_lit, Reason::None);
                } else {
                    let idx = self.clauses.len();
                    self.watches[lit_code(learnt[0])].push(idx);
                    self.watches[lit_code(learnt[1])].push(idx);
                    self.clauses.push(learnt);
                    self.enqueue(assert_lit, Reason::Clause(idx));
                }
                self.var_inc *= 1.0 / 0.95;
                self.stats.prof_ns[4] += t0.elapsed().as_nanos() as u64;
                if self.stats.conflicts >= self.conflict_budget {
                    return Result::Unknown;
                }
            } else {
                if self.restarts && conflicts_since_restart >= restart_limit {
                    conflicts_since_restart = 0;
                    restart_limit = restart_limit * 3 / 2;
                    self.backtrack(0);
                    continue;
                }
                let t0 = Instant::now();
                let pb = self.pick_branch();
                self.stats.prof_ns[5] += t0.elapsed().as_nanos() as u64;
                match pb {
                    None => return Result::Sat,
                    Some(l) => {
                        self.stats.decisions += 1;
                        let t0 = Instant::now();
                        let lvl = self.trail_lim.len();
                        if self.mats.len() <= lvl + 1 {
                            let m = Mat::new(self.n_vars + 1, self.vw);
                            self.mats.push(m);
                        }
                        let (lo, hi) = self.mats.split_at_mut(lvl + 1);
                        hi[0].copy_from(&lo[lvl]);
                        self.stats.mat_copies += 1;
                        self.stats.mat_copy_words += lo[lvl].words() as u64;
                        self.stats.prof_ns[6] += t0.elapsed().as_nanos() as u64;
                        self.trail_lim.push(self.trail.len());
                        self.unit_lim.push(self.unit_lits.len());
                        self.enqueue(l, Reason::None);
                    }
                }
            }
        }
    }

    pub fn model_block(&self, block: &[u32]) -> u64 {
        self.block_value(block).expect("model incomplete")
    }
}
