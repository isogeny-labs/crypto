//! `LinCdcl`: a CDCL solver with an XOR engine that understands field products.
//!
//! Variables are bits. Constraints are (i) clauses, (ii) XOR rows over bits, and
//! (iii) **product gates** `z = x · y` over `F_{2^n}`, each operand an `n`-bit
//! block. A gate is inert while both operands are open; as soon as one operand is
//! fully assigned the gate injects `n` XOR rows `z_k ⊕ Σ_j M(x)_{kj} y_j = 0`
//! (the multiplication-by-`x` matrix) at the current decision level, guarded by
//! the operand's literals so conflict analysis stays exact, and withdraws them on
//! backtrack. Squaring is a fixed linear map, so with the summands decided a chained
//! Semaev system becomes a linear system that Gauss–Jordan solves outright.
//!
//! The system is small (hundreds of variables), so the XOR engine simply
//! re-eliminates all active rows at every propagation fixpoint, tracking each
//! reduced row's combination of original rows; a propagated literal's reason is the
//! set of assigned literals occurring in those rows plus the guards of any dynamic
//! row involved. This is deliberately simple and exact rather than fast.
//!
//! Counted work: `conflicts` is the metric every arm is read in.

#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Val {
    Undef,
    True,
    False,
}

#[derive(Clone, Debug)]
enum Reason {
    None,
    Clause(usize),
    /// Antecedent literals (all true), from the XOR engine.
    Lits(Vec<i32>),
}

#[derive(Clone, Debug)]
pub struct XorRow {
    pub vars: Vec<u32>,
    pub rhs: bool,
}

struct DynRow {
    row: XorRow,
    guard: Vec<i32>,
    level: u32,
}

pub struct Gate {
    pub x: Vec<u32>,
    pub y: Vec<u32>,
    pub z: Vec<u32>,
    linearised_at: Option<u32>,
}

#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub conflicts: u64,
    pub decisions: u64,
    pub propagations: u64,
    pub xor_propagations: u64,
    pub gje_calls: u64,
    pub gate_linearisations: u64,
    pub learnt_clauses: u64,
    pub learnt_lits: u64,
    /// Conflicts raised while the `stage_block` (the chain's `t`) was fully assigned
    /// (the `x₃` stage) versus not (the `t` stage).
    pub conflicts_stage_assigned: u64,
    pub conflicts_stage_open: u64,
    pub forgotten_clauses: u64,
    /// Wall nanoseconds inside the XOR engine, the clause propagation loop and conflict analysis.
    pub ns_gje: u64,
    pub ns_clause_prop: u64,
    pub ns_analyze: u64,
    /// Histogram of learnt-clause literals that fall in the "priority" (summand) blocks:
    /// index = count of priority literals, value = clauses. Width `< 2ℓ` means the clause
    /// excludes more than one pair.
    pub learnt_priority_width: Vec<u64>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Result {
    Sat,
    Unsat,
    Unknown,
}

pub struct LinCdcl {
    pub n_vars: usize,
    pub field_n: u32,
    pub irr: u64,
    assigns: Vec<Val>,
    level: Vec<u32>,
    reason: Vec<Reason>,
    trail: Vec<i32>,
    trail_lim: Vec<usize>,
    qhead: usize,
    clauses: Vec<Vec<i32>>,
    n_original_clauses: usize,
    watches: Vec<Vec<usize>>, // indexed by literal code
    xors: Vec<XorRow>,
    dyn_rows: Vec<DynRow>,
    gates: Vec<Gate>,
    priority: Vec<bool>,
    activity: Vec<f64>,
    var_inc: f64,
    phase: Vec<bool>,
    seen: Vec<bool>,
    pub conflict_budget: u64,
    pub stats: Stats,
    pub restarts: bool,
    /// Literals decided first, one per level, before any search decision (incremental use).
    /// A refutation under assumptions returns `Unsat` without marking the solver inconsistent.
    pub assumptions: Vec<i32>,
    /// Block whose assignment state classifies a conflict (see `Stats`).
    pub stage_block: Vec<u32>,
    ok: bool,
}

fn lit_code(l: i32) -> usize {
    let v = l.unsigned_abs() as usize;
    2 * v + (l < 0) as usize
}

impl LinCdcl {
    pub fn new(n_vars: usize, field_n: u32, irr: u64) -> Self {
        LinCdcl {
            n_vars,
            field_n,
            irr,
            assigns: vec![Val::Undef; n_vars + 1],
            level: vec![0; n_vars + 1],
            reason: vec![Reason::None; n_vars + 1],
            trail: Vec::new(),
            trail_lim: Vec::new(),
            qhead: 0,
            clauses: Vec::new(),
            n_original_clauses: 0,
            watches: vec![Vec::new(); 2 * (n_vars + 1) + 2],
            xors: Vec::new(),
            dyn_rows: Vec::new(),
            gates: Vec::new(),
            priority: vec![false; n_vars + 1],
            activity: vec![0.0; n_vars + 1],
            var_inc: 1.0,
            phase: vec![false; n_vars + 1],
            seen: vec![false; n_vars + 1],
            conflict_budget: u64::MAX,
            stats: Stats::default(),
            restarts: false,
            assumptions: Vec::new(),
            stage_block: Vec::new(),
            ok: true,
        }
    }

    // ---- field helpers (n ≤ 62, polynomial basis, irr includes the x^n term) ----

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

    /// Column `j` of the multiplication-by-`a` matrix: `a · z^j`.
    pub fn mul_matrix(&self, a: u64) -> Vec<u64> {
        (0..self.field_n)
            .map(|j| self.gf_mul(a, 1u64 << j))
            .collect()
    }

    /// Column `j` of the squaring matrix: `z^{2j}`.
    pub fn sq_matrix(&self) -> Vec<u64> {
        (0..self.field_n)
            .map(|j| self.gf_mul(1u64 << j, 1u64 << j))
            .collect()
    }

    // ---- constraint builders ----

    pub fn set_priority(&mut self, vars: &[u32]) {
        for &v in vars {
            self.priority[v as usize] = true;
        }
    }

    pub fn add_clause(&mut self, lits: Vec<i32>) -> bool {
        if !self.ok {
            return false;
        }
        let mut lits = lits;
        lits.sort_unstable();
        lits.dedup();
        // tautology / satisfied / falsified at level 0
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
                self.ok = self.propagate().is_none();
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
        let mut vars = vars;
        vars.sort_unstable();
        // cancel duplicate variables (x ⊕ x = 0)
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

    /// Linear rows `z_k ⊕ Σ_j M_kj y_j ⊕ c_k = 0` for a fixed matrix (columns as words).
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

    /// Rows `z_k ⊕ Σ_j M_kj (y_j ⊕ y'_j) = 0`: a linear map of a sum of two blocks.
    pub fn add_linear_map_sum(&mut self, z: &[u32], y: &[u32], y2: &[u32], cols: &[u64]) {
        let n = self.field_n as usize;
        for k in 0..n {
            let mut vars = vec![z[k]];
            for j in 0..n {
                if (cols[j] >> k) & 1 == 1 {
                    vars.push(y[j]);
                    vars.push(y2[j]);
                }
            }
            self.add_xor(vars, false);
        }
    }

    pub fn add_gate(&mut self, x: Vec<u32>, y: Vec<u32>, z: Vec<u32>) {
        self.gates.push(Gate {
            x,
            y,
            z,
            linearised_at: None,
        });
    }

    // ---- assignment ----

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

    fn decision_level(&self) -> u32 {
        self.trail_lim.len() as u32
    }

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

    fn block_lits(&self, block: &[u32]) -> Vec<i32> {
        block
            .iter()
            .map(|&v| {
                if self.assigns[v as usize] == Val::True {
                    v as i32
                } else {
                    -(v as i32)
                }
            })
            .collect()
    }

    // ---- propagation ----

    /// Returns a conflict as (clause-like literal set all false) or None.
    fn propagate(&mut self) -> Option<Vec<i32>> {
        loop {
            // 1. clause propagation (two watched literals)
            let t_cp = std::time::Instant::now();
            while self.qhead < self.trail.len() {
                let l = self.trail[self.qhead];
                self.qhead += 1;
                self.stats.propagations += 1;
                let false_lit = -l;
                let code = lit_code(false_lit);
                let ws = std::mem::take(&mut self.watches[code]);
                let mut keep = Vec::with_capacity(ws.len());
                let mut conflict: Option<Vec<i32>> = None;
                let mut i = 0;
                while i < ws.len() {
                    let ci = ws[i];
                    i += 1;
                    if conflict.is_some() {
                        keep.push(ci);
                        continue;
                    }
                    // make sure false_lit is at position 1
                    if self.clauses[ci][0] == false_lit {
                        self.clauses[ci].swap(0, 1);
                    }
                    let first = self.clauses[ci][0];
                    if self.value(first) == Val::True {
                        keep.push(ci);
                        continue;
                    }
                    // look for a new watch
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
                    keep.push(ci);
                    if self.value(first) == Val::False {
                        conflict = Some(self.clauses[ci].clone());
                    } else {
                        self.enqueue(first, Reason::Clause(ci));
                    }
                }
                self.watches[code] = keep;
                if let Some(c) = conflict {
                    self.stats.ns_clause_prop += t_cp.elapsed().as_nanos() as u64;
                    return Some(c);
                }
            }
            self.stats.ns_clause_prop += t_cp.elapsed().as_nanos() as u64;
            // 2. gate linearisation
            let mut added = false;
            for gi in 0..self.gates.len() {
                if self.gates[gi].linearised_at.is_some() {
                    continue;
                }
                let (xv, yv) = (
                    self.block_value(&self.gates[gi].x),
                    self.block_value(&self.gates[gi].y),
                );
                let (known, other, guard) = match (xv, yv) {
                    (Some(a), _) => (
                        a,
                        self.gates[gi].y.clone(),
                        self.block_lits(&self.gates[gi].x),
                    ),
                    (None, Some(b)) => (
                        b,
                        self.gates[gi].x.clone(),
                        self.block_lits(&self.gates[gi].y),
                    ),
                    (None, None) => continue,
                };
                let cols = self.mul_matrix(known);
                let z = self.gates[gi].z.clone();
                let lvl = self.decision_level();
                let n = self.field_n as usize;
                for k in 0..n {
                    let mut vars = vec![z[k]];
                    for j in 0..n {
                        if (cols[j] >> k) & 1 == 1 {
                            vars.push(other[j]);
                        }
                    }
                    vars.sort_unstable();
                    self.dyn_rows.push(DynRow {
                        row: XorRow { vars, rhs: false },
                        guard: guard.clone(),
                        level: lvl,
                    });
                }
                self.gates[gi].linearised_at = Some(lvl);
                self.stats.gate_linearisations += 1;
                added = true;
            }
            // 3. XOR engine
            let r = self.gje();
            match r {
                GjeOutcome::Conflict(expl) => return Some(expl),
                GjeOutcome::Propagated => continue,
                GjeOutcome::Quiet => {
                    if added {
                        continue;
                    }
                    return None;
                }
            }
        }
    }

    fn gje(&mut self) -> GjeOutcome {
        let t0 = std::time::Instant::now();
        let r = self.gje_inner();
        self.stats.ns_gje += t0.elapsed().as_nanos() as u64;
        r
    }

    fn gje_inner(&mut self) -> GjeOutcome {
        self.stats.gje_calls += 1;
        let nv = self.n_vars + 1;
        let vw = nv.div_ceil(64);
        let n_static = self.xors.len();
        let n_rows = n_static + self.dyn_rows.len();
        if n_rows == 0 {
            return GjeOutcome::Quiet;
        }
        let rw = n_rows.div_ceil(64);
        // Build reduced rows over unassigned vars; rhs folds in assigned values.
        let mut mat: Vec<Vec<u64>> = Vec::with_capacity(n_rows);
        let mut rhs: Vec<bool> = Vec::with_capacity(n_rows);
        let mut comb: Vec<Vec<u64>> = Vec::with_capacity(n_rows);
        for ri in 0..n_rows {
            let row = if ri < n_static {
                &self.xors[ri]
            } else {
                &self.dyn_rows[ri - n_static].row
            };
            let mut bits = vec![0u64; vw];
            let mut r = row.rhs;
            for &v in &row.vars {
                match self.assigns[v as usize] {
                    Val::Undef => bits[v as usize / 64] ^= 1u64 << (v % 64),
                    Val::True => r ^= true,
                    Val::False => {}
                }
            }
            mat.push(bits);
            rhs.push(r);
            let mut c = vec![0u64; rw];
            c[ri / 64] |= 1u64 << (ri % 64);
            comb.push(c);
        }
        // Gauss–Jordan elimination.
        let mut pivot_row = 0usize;
        let mut pivots: Vec<(usize, usize)> = Vec::new(); // (row, var)
        for var in 1..nv {
            if pivot_row >= n_rows {
                break;
            }
            let (w, b) = (var / 64, 1u64 << (var % 64));
            let mut sel = None;
            for r in pivot_row..n_rows {
                if mat[r][w] & b != 0 {
                    sel = Some(r);
                    break;
                }
            }
            let Some(sel) = sel else { continue };
            mat.swap(pivot_row, sel);
            rhs.swap(pivot_row, sel);
            comb.swap(pivot_row, sel);
            let (prow, prhs, pcomb) = (
                mat[pivot_row].clone(),
                rhs[pivot_row],
                comb[pivot_row].clone(),
            );
            for r in 0..n_rows {
                if r != pivot_row && mat[r][w] & b != 0 {
                    for k in 0..vw {
                        mat[r][k] ^= prow[k];
                    }
                    rhs[r] ^= prhs;
                    for k in 0..rw {
                        comb[r][k] ^= pcomb[k];
                    }
                }
            }
            pivots.push((pivot_row, var));
            pivot_row += 1;
        }
        // Conflict rows: all-zero with rhs = 1.
        for r in 0..n_rows {
            if rhs[r] && mat[r].iter().all(|&w| w == 0) {
                let expl = self.explain(&comb[r], n_static);
                // clause form: every literal false
                return GjeOutcome::Conflict(expl.into_iter().map(|l| -l).collect());
            }
        }
        // Unit rows: exactly one variable.
        let mut propagated = false;
        for &(r, var) in &pivots {
            let cnt: u32 = mat[r].iter().map(|w| w.count_ones()).sum();
            if cnt == 1 && self.assigns[var] == Val::Undef {
                let lit = if rhs[r] { var as i32 } else { -(var as i32) };
                let expl = self.explain(&comb[r], n_static);
                self.enqueue(lit, Reason::Lits(expl));
                self.stats.xor_propagations += 1;
                propagated = true;
            }
        }
        if propagated {
            GjeOutcome::Propagated
        } else {
            GjeOutcome::Quiet
        }
    }

    /// Assigned literals in the original rows of a combination, plus dynamic-row guards.
    fn explain(&self, comb: &[u64], n_static: usize) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
        for (w, &word) in comb.iter().enumerate() {
            let mut bits = word;
            while bits != 0 {
                let t = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                let ri = w * 64 + t;
                if ri < n_static {
                    for &v in &self.xors[ri].vars {
                        match self.assigns[v as usize] {
                            Val::True => out.push(v as i32),
                            Val::False => out.push(-(v as i32)),
                            Val::Undef => {}
                        }
                    }
                } else {
                    let d = &self.dyn_rows[ri - n_static];
                    for &v in &d.row.vars {
                        match self.assigns[v as usize] {
                            Val::True => out.push(v as i32),
                            Val::False => out.push(-(v as i32)),
                            Val::Undef => {}
                        }
                    }
                    out.extend_from_slice(&d.guard);
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
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
                    if (key.0, key.1) > (b.0, b.1) || ((key.0, key.1) == (b.0, b.1) && key.2 < b.2)
                    {
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
        self.dyn_rows.retain(|d| d.level <= lvl);
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

    /// First-UIP learning. `confl` is a set of literals that are all false.
    fn analyze(&mut self, confl: Vec<i32>) -> (Vec<i32>, u32) {
        let mut learnt: Vec<i32> = vec![0]; // slot 0 = asserting literal
        let mut path = 0usize;
        let mut idx = self.trail.len();
        let mut p: Option<i32> = None;
        let mut cur: Vec<i32> = confl;
        let cur_level = self.decision_level();
        loop {
            for &q in &cur {
                if Some(q) == p.map(|x| -x) {
                    continue;
                }
                // q is false here; in a clause reason, lits other than the implied one are false.
                let v = q.unsigned_abs() as usize;
                if !self.seen[v] && self.level[v] > 0 {
                    self.seen[v] = true;
                    self.bump(v);
                    if self.level[v] >= cur_level {
                        path += 1;
                    } else {
                        learnt.push(q);
                    }
                }
            }
            // next literal on the trail that is seen
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
            p = Some(lit);
            cur = match &self.reason[v] {
                Reason::Clause(ci) => self.clauses[*ci]
                    .iter()
                    .copied()
                    .filter(|&l| l != lit)
                    .collect(),
                Reason::Lits(ls) => ls.iter().map(|&l| -l).filter(|&l| l != -lit).collect(),
                Reason::None => unreachable!("decision on the implication path"),
            };
        }
        for &l in &learnt {
            self.seen[l.unsigned_abs() as usize] = false;
        }
        // backtrack level = max level among the other literals
        let mut bt = 0;
        if learnt.len() > 1 {
            let mut max_i = 1;
            for i in 2..learnt.len() {
                if self.level[learnt[i].unsigned_abs() as usize]
                    > self.level[learnt[max_i].unsigned_abs() as usize]
                {
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
        self.backtrack(0);
        if self.propagate().is_some() {
            self.ok = false;
            return Result::Unsat;
        }
        let budget_end = self.stats.conflicts.saturating_add(self.conflict_budget);
        let mut conflicts_since_restart = 0u64;
        let mut restart_limit = 100u64;
        loop {
            if let Some(confl) = self.propagate() {
                self.stats.conflicts += 1;
                conflicts_since_restart += 1;
                if !self.stage_block.is_empty() {
                    if self.block_value(&self.stage_block).is_some() {
                        self.stats.conflicts_stage_assigned += 1;
                    } else {
                        self.stats.conflicts_stage_open += 1;
                    }
                }
                if self.decision_level() == 0 {
                    self.ok = false;
                    return Result::Unsat;
                }
                let t_an = std::time::Instant::now();
                let (learnt, bt) = self.analyze(confl);
                self.stats.ns_analyze += t_an.elapsed().as_nanos() as u64;
                self.backtrack(bt);
                let pw = learnt
                    .iter()
                    .filter(|&&l| self.priority[l.unsigned_abs() as usize])
                    .count();
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
                if self.stats.conflicts >= budget_end {
                    return Result::Unknown;
                }
            } else {
                if self.restarts && conflicts_since_restart >= restart_limit {
                    conflicts_since_restart = 0;
                    restart_limit = restart_limit * 3 / 2;
                    self.backtrack(0);
                    continue;
                }
                // assumptions first, one level each
                let mut next_assumption: Option<i32> = None;
                while (self.decision_level() as usize) < self.assumptions.len() {
                    let p = self.assumptions[self.decision_level() as usize];
                    match self.value(p) {
                        Val::True => self.trail_lim.push(self.trail.len()),
                        Val::False => return Result::Unsat,
                        Val::Undef => {
                            next_assumption = Some(p);
                            break;
                        }
                    }
                }
                if let Some(p) = next_assumption {
                    self.trail_lim.push(self.trail.len());
                    self.enqueue(p, Reason::None);
                    continue;
                }
                match self.pick_branch() {
                    None => return Result::Sat,
                    Some(l) => {
                        self.stats.decisions += 1;
                        self.trail_lim.push(self.trail.len());
                        self.enqueue(l, Reason::None);
                    }
                }
            }
        }
    }

    /// Backtrack to level 0 (public reset between incremental solves).
    pub fn reset(&mut self) {
        self.backtrack(0);
    }

    /// Drop learnt clauses that mention any variable of `block` (e.g. the target block, whose
    /// clauses are specific to one target) and rebuild the watch lists. Clauses that are the
    /// reason of a current assignment are kept. Returns the number dropped.
    pub fn forget_learnts_mentioning(&mut self, block: &[u32]) -> usize {
        let set: std::collections::HashSet<u32> = block.iter().copied().collect();
        let locked: std::collections::HashSet<usize> = self
            .trail
            .iter()
            .filter_map(|&t| match self.reason[t.unsigned_abs() as usize] {
                Reason::Clause(c) => Some(c),
                _ => None,
            })
            .collect();
        let mut removed = 0;
        for ci in self.n_original_clauses..self.clauses.len() {
            if self.clauses[ci].is_empty() || locked.contains(&ci) {
                continue;
            }
            if self.clauses[ci]
                .iter()
                .any(|l| set.contains(&l.unsigned_abs()))
            {
                self.clauses[ci].clear();
                removed += 1;
            }
        }
        for w in &mut self.watches {
            w.clear();
        }
        for ci in 0..self.clauses.len() {
            if self.clauses[ci].len() >= 2 {
                let (a, b) = (self.clauses[ci][0], self.clauses[ci][1]);
                self.watches[lit_code(a)].push(ci);
                self.watches[lit_code(b)].push(ci);
            }
        }
        self.stats.forgotten_clauses += removed as u64;
        removed
    }

    pub fn live_learnts(&self) -> usize {
        self.clauses[self.n_original_clauses..]
            .iter()
            .filter(|c| !c.is_empty())
            .count()
    }

    pub fn model_block(&self, block: &[u32]) -> u64 {
        self.block_value(block).expect("model incomplete")
    }
}

enum GjeOutcome {
    Conflict(Vec<i32>),
    Propagated,
    Quiet,
}

/// The chained-S₃ PDP instance for `y² + xy = x³ + ax² + b` with `b = 1` (the a-free
/// S₃), summands in the subspace `x < 2^ℓ`, target abscissa `x_r` outside the base.
/// Blocks: x1, x2, x3, t, p = x1·x2, q = x1 ⊕ x2, v = q·t, w = p·t, a = t·x3.
/// S₃(x1,x2,t) = p² ⊕ v² ⊕ w ⊕ b;  S₃(t,x3,x_r) = (a ⊕ x_r·t ⊕ x_r·x3)² ⊕ x_r·a ⊕ b.
pub struct ChainInstance {
    pub solver: LinCdcl,
    pub x1: Vec<u32>,
    pub x2: Vec<u32>,
    pub x3: Vec<u32>,
    pub t: Vec<u32>,
    /// Present when `x_R` is a variable block set per target through assumptions.
    pub xr: Option<Vec<u32>>,
}

impl ChainInstance {
    /// Fixed target: `x_R` folded into the constants (one solver per target).
    pub fn build(n: u32, l: u32, irr: u64, x_r: u64, b: u64, lex_pairs: bool) -> ChainInstance {
        Self::build_inner(n, l, irr, Some(x_r), b, lex_pairs)
    }

    /// Parametrised target: `x_R` is a block fixed per solve by `set_target` (assumptions), so one
    /// solver and its learnt clauses serve many targets of the same factor base.
    pub fn build_param(n: u32, l: u32, irr: u64, b: u64, lex_pairs: bool) -> ChainInstance {
        Self::build_inner(n, l, irr, None, b, lex_pairs)
    }

    /// Fix the target for the next solve. With `forget`, learnt clauses that mention the
    /// `x_R` block (specific to the previous target) are dropped first; the rest, the learnt
    /// table of root-less pairs, is kept.
    pub fn set_target(&mut self, x_r: u64, forget: bool) {
        let xr = self.xr.as_ref().expect("parametrised instance").clone();
        if forget {
            self.solver.reset();
            self.solver.forget_learnts_mentioning(&xr);
        }
        let xr = self.xr.as_ref().expect("parametrised instance");
        self.solver.assumptions = xr
            .iter()
            .enumerate()
            .map(|(j, &v)| {
                if (x_r >> j) & 1 == 1 {
                    v as i32
                } else {
                    -(v as i32)
                }
            })
            .collect();
    }

    fn build_inner(
        n: u32,
        l: u32,
        irr: u64,
        x_r: Option<u64>,
        b: u64,
        lex_pairs: bool,
    ) -> ChainInstance {
        let nn = n as usize;
        let mut next = 1u32;
        let mut block = || {
            let v: Vec<u32> = (next..next + n).collect();
            next += n;
            v
        };
        let (x1, x2, x3, t, p, q, v, w, a) = (
            block(),
            block(),
            block(),
            block(),
            block(),
            block(),
            block(),
            block(),
            block(),
        );
        // parametrised target: blocks xr, b2 = t·xr, c = x3·xr, e = a·xr
        let param = if x_r.is_none() {
            Some((block(), block(), block(), block()))
        } else {
            None
        };
        let n_vars = (next - 1) as usize + if lex_pairs { nn } else { 0 };
        let mut s = LinCdcl::new(n_vars, n, irr);
        let sq = s.sq_matrix();
        let mr = s.mul_matrix(x_r.unwrap_or(0));
        // domain: summand bits ≥ ℓ are zero
        for blk in [&x1, &x2, &x3] {
            for j in l as usize..nn {
                s.add_clause(vec![-(blk[j] as i32)]);
            }
        }
        // q = x1 ⊕ x2
        for k in 0..nn {
            s.add_xor(vec![q[k], x1[k], x2[k]], false);
        }
        // gates
        s.add_gate(x1.clone(), x2.clone(), p.clone());
        s.add_gate(q.clone(), t.clone(), v.clone());
        s.add_gate(p.clone(), t.clone(), w.clone());
        s.add_gate(t.clone(), x3.clone(), a.clone());
        // S3a: sq(p) ⊕ sq(v) ⊕ w ⊕ b = 0
        for k in 0..nn {
            let mut vars = vec![w[k]];
            for j in 0..nn {
                if (sq[j] >> k) & 1 == 1 {
                    vars.push(p[j]);
                    vars.push(v[j]);
                }
            }
            s.add_xor(vars, (b >> k) & 1 == 1);
        }
        if let Some((xr, b2, c, e)) = &param {
            // S3b with x_R a block: sq(a ⊕ b2 ⊕ c) ⊕ e ⊕ b = 0, gates b2 = t·xr, c = x3·xr, e = a·xr
            s.add_gate(t.clone(), xr.clone(), b2.clone());
            s.add_gate(x3.clone(), xr.clone(), c.clone());
            s.add_gate(a.clone(), xr.clone(), e.clone());
            for k in 0..nn {
                let mut vars = vec![e[k]];
                for j in 0..nn {
                    if (sq[j] >> k) & 1 == 1 {
                        vars.push(a[j]);
                        vars.push(b2[j]);
                        vars.push(c[j]);
                    }
                }
                s.add_xor(vars, (b >> k) & 1 == 1);
            }
        } else {
            // S3b: sq(a ⊕ Mr t ⊕ Mr x3) ⊕ Mr a ⊕ b = 0
            // inner_j = a_j ⊕ Σ_i Mr_ji t_i ⊕ Σ_i Mr_ji x3_i ; row k: Σ_j sq_kj inner_j ⊕ Σ_j Mr_kj a_j = b_k
            for k in 0..nn {
                let mut vars = Vec::new();
                for j in 0..nn {
                    if (sq[j] >> k) & 1 == 1 {
                        vars.push(a[j]);
                        for i in 0..nn {
                            if (mr[i] >> j) & 1 == 1 {
                                vars.push(t[i]);
                                vars.push(x3[i]);
                            }
                        }
                    }
                    if (mr[j] >> k) & 1 == 1 {
                        vars.push(a[j]);
                    }
                }
                s.add_xor(vars, (b >> k) & 1 == 1);
            }
        }
        // optional lex ordering x1 ≤ x2 on the ℓ low bits (high bit most significant),
        // via a standard comparator chain with aux "equal-so-far" variables e_j.
        if lex_pairs {
            let e: Vec<u32> = (next..next + n).collect();
            let l = l as usize;
            // e[l] (virtual) = true. For j from l-1 down to 0:
            //   eq_j := e_{j+1} ∧ (x1_j == x2_j); constraint: e_{j+1} → (x1_j ≤ x2_j) i.e. ¬(x1_j ∧ ¬x2_j)
            let mut prev: Option<u32> = None; // None = true
            for j in (0..l).rev() {
                let (a1, a2) = (x1[j] as i32, x2[j] as i32);
                match prev {
                    None => {
                        s.add_clause(vec![-a1, a2]);
                    }
                    Some(ep) => {
                        s.add_clause(vec![-(ep as i32), -a1, a2]);
                    }
                }
                // e_j ↔ prev ∧ (x1_j ↔ x2_j)
                let ej = e[j] as i32;
                s.add_clause(vec![-ej, -a1, a2]);
                s.add_clause(vec![-ej, a1, -a2]);
                if let Some(ep) = prev {
                    s.add_clause(vec![-ej, ep as i32]);
                    s.add_clause(vec![ej, -(ep as i32), -a1, -a2]);
                    s.add_clause(vec![ej, -(ep as i32), a1, a2]);
                } else {
                    s.add_clause(vec![ej, -a1, -a2]);
                    s.add_clause(vec![ej, a1, a2]);
                }
                prev = Some(e[j]);
            }
        }
        let mut prio: Vec<u32> = x1[..l as usize].to_vec();
        prio.extend_from_slice(&x2[..l as usize]);
        s.set_priority(&prio);
        s.stage_block = t.clone();
        ChainInstance {
            solver: s,
            x1,
            x2,
            x3,
            t,
            xr: param.map(|p| p.0),
        }
    }
}

/// This file is a module shared by `b0_pdp_sat_baseline.rs` through `#[path]`; cargo also
/// compiles every file under `examples/` as an example binary, so it needs an entry point.
#[allow(dead_code)]
fn main() {}
