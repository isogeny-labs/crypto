//! Boolean tensor networks and the exact (exponential) counters
//! (Go: tensor.go).
//!
//! A Boolean tensor network: edges carry bits, vertices carry predicates on the
//! bits of their incident edges. Contracting the network means counting the
//! assignments of bits to edges that satisfy every vertex.
//!
//! Vertices come in three shapes, all of which are 0/1 tensors:
//!   - `SupportVertex` lists the satisfying rows explicitly (a sparse tensor);
//!   - `FuncVertex` holds when its output bits equal F(input bits);
//!   - `PredVertex` holds when an arbitrary predicate accepts its bits.
//!
//! The shapes only tell a contractor how to evaluate a vertex cheaply; the
//!   brute-force counter treats all three as predicates.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::schedule::Step;
use crate::{Error, Result};

/// Closure type of `FuncVertex::f`: input bits -> (output bits, ok); `None`
/// plays the role of Go's `ok == false`.
pub type FuncFn = Rc<dyn Fn(&[bool]) -> Option<Vec<bool>>>;
/// Closure type of `PredVertex::p`.
pub type PredFn = Rc<dyn Fn(&[bool]) -> bool>;

/// Vertex is a predicate on the bits of its incident edges. The Go interface
/// is closed (three shapes, always type-switched), so it is an enum.
#[derive(Clone)]
pub enum Vertex {
    Support(SupportVertex),
    Func(FuncVertex),
    Pred(PredVertex),
}

impl Vertex {
    /// Edges of the vertex in the order `holds` expects (FuncVertex: In then Out).
    pub fn edges(&self) -> Vec<usize> {
        match self {
            Vertex::Support(v) => v.e.clone(),
            Vertex::Func(v) => {
                let mut r = v.in_.clone();
                r.extend_from_slice(&v.out);
                r
            }
            Vertex::Pred(v) => v.e.clone(),
        }
    }

    /// Holds reports whether vals (aligned with `edges()`) satisfies the vertex.
    pub fn holds(&self, vals: &[bool]) -> bool {
        match self {
            Vertex::Support(v) => v.set.contains(&bits_key(vals)),
            Vertex::Func(v) => {
                let n_in = v.in_.len();
                match (v.f)(&vals[..n_in]) {
                    None => false,
                    Some(out) => {
                        for (i, b) in out.iter().enumerate() {
                            if vals[n_in + i] != *b {
                                return false;
                            }
                        }
                        true
                    }
                }
            }
            Vertex::Pred(v) => (v.p)(vals),
        }
    }
}

/// SupportVertex is satisfied exactly by the listed rows.
#[derive(Clone)]
pub struct SupportVertex {
    pub e: Vec<usize>,
    pub rows: Vec<Vec<bool>>,
    /// Set of `bits_key(row)` for fast `holds` (Go: set map[string]bool).
    pub(crate) set: HashSet<Vec<u8>>,
}

impl SupportVertex {
    /// Go: NewSupportVertex. Returns the struct by value (wrap in `Vertex::Support`).
    /// Unlike Go, duplicate rows are dropped (first occurrence kept): a
    /// support vertex is a 0/1 predicate, and a repeated row would make
    /// SparseContractor and DFSCounter count it twice but BruteForceCounter once.
    pub fn new(edges: Vec<usize>, rows: Vec<Vec<bool>>) -> SupportVertex {
        let mut set = HashSet::with_capacity(rows.len());
        let rows: Vec<Vec<bool>> = rows
            .into_iter()
            .filter(|r| set.insert(bits_key(r)))
            .collect();
        SupportVertex {
            e: edges,
            rows,
            set,
        }
    }
}

/// FuncVertex is satisfied when the Out bits equal F(In bits) and F accepts.
#[derive(Clone)]
pub struct FuncVertex {
    pub in_: Vec<usize>,
    pub out: Vec<usize>,
    pub f: FuncFn,
}

/// PredVertex is satisfied when P accepts its bits.
#[derive(Clone)]
pub struct PredVertex {
    pub e: Vec<usize>,
    pub p: PredFn,
}

/// Network is a Boolean tensor network. The vertex order is the contraction
/// schedule used by `SparseContractor`.
#[derive(Clone, Default)]
pub struct Network {
    pub num_edges: usize,
    pub vertices: Vec<Vertex>,
    /// Schedule, if non-empty (Go: non-nil), is an evaluation order for
    /// `DFSCounter` that interleaves every vertex (by index into `vertices`)
    /// with hints (see schedule.rs). It does not change what the network counts.
    pub schedule: Vec<Step>,
}

/// NetworkStats summarizes the shape of a network.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NetworkStats {
    pub edges: usize,
    pub vertices: usize,
    pub max_degree: usize,
    /// True when every edge has at most two endpoints, as in the tensor
    /// contraction problem (otherwise it is a hypergraph).
    pub is_graph: bool,
}

impl Network {
    pub fn stats(&self) -> NetworkStats {
        let mut endpoints = vec![0usize; self.num_edges];
        let mut st = NetworkStats {
            edges: self.num_edges,
            vertices: self.vertices.len(),
            max_degree: 0,
            is_graph: true,
        };
        for v in &self.vertices {
            let es = v.edges();
            if es.len() > st.max_degree {
                st.max_degree = es.len();
            }
            for e in es {
                endpoints[e] += 1;
            }
        }
        for k in endpoints {
            if k > 2 {
                st.is_graph = false;
            }
        }
        st
    }
}

/// Counter solves the Boolean tensor contraction problem: it counts satisfying
/// assignments, optionally with some edge bits fixed. The [Ope26] algorithm
/// would be one implementation of this trait. `&mut self` because
/// `SparseContractor` records `peak_rows`. Counts wrap like Go's uint64
/// (use wrapping_add / wrapping_mul).
pub trait Counter {
    fn count(&mut self, n: &Network, fixed: &HashMap<usize, bool>) -> Result<u64>;
}

/// BruteForceCounter enumerates all 2^NumEdges assignments. Reference only.
/// Error: "ecdl: brute force limited to 24 edges".
#[derive(Debug, Clone, Copy, Default)]
pub struct BruteForceCounter;

impl Counter for BruteForceCounter {
    fn count(&mut self, n: &Network, fixed: &HashMap<usize, bool>) -> Result<u64> {
        if n.num_edges > 24 {
            return Err(Error::new("ecdl: brute force limited to 24 edges"));
        }
        let mut count: u64 = 0;
        let mut assign = vec![false; n.num_edges];
        let mut mask: u64 = 0;
        while mask < (1u64 << n.num_edges) {
            for (e, a) in assign.iter_mut().enumerate() {
                *a = (mask >> e) & 1 == 1;
            }
            mask += 1;
            if !agrees(&assign, fixed) {
                continue;
            }
            let mut ok = true;
            for v in &n.vertices {
                let es = v.edges();
                let vals: Vec<bool> = es.iter().map(|&e| assign[e]).collect();
                if !v.holds(&vals) {
                    ok = false;
                    break;
                }
            }
            if ok {
                count = count.wrapping_add(1);
            }
        }
        Ok(count)
    }
}

fn agrees(assign: &[bool], fixed: &HashMap<usize, bool>) -> bool {
    for (&e, &b) in fixed {
        if assign[e] != b {
            return false;
        }
    }
    true
}

/// SparseContractor contracts the network vertex by vertex in network order,
/// holding the partial contraction as a sparse table over the edges that are
/// bound so far, and summing out each edge as soon as no later vertex touches
/// it. Each SupportVertex is hash-joined in, each FuncVertex extends rows by
/// computing its outputs, and each PredVertex filters rows. The schedule must
/// bind all inputs of a FuncVertex or PredVertex before that vertex.
///
/// This is an exact, ordinary contractor with no subexponential guarantee. It
/// stands in for the [Ope26] solver behind the Counter trait.
/// Errors: "ecdl: SparseContractor does not support hinted schedules; use
/// DFSCounter", "ecdl: intermediate table exceeded {} rows", and the
/// "before ... is bound" errors of apply_func / apply_pred.
#[derive(Debug, Clone, Copy, Default)]
pub struct SparseContractor {
    /// MaxRows aborts contraction when an intermediate table grows past it
    /// (0 means unlimited).
    pub max_rows: usize,
    /// PeakRows records the largest intermediate table seen by the last Count.
    pub peak_rows: usize,
}

/// Partial contraction (Go: table). `rows` key: one byte (0/1) per edge in `edges`.
struct Table {
    edges: Vec<usize>,
    pos: HashMap<usize, usize>,
    rows: HashMap<Vec<u8>, u64>,
}

impl Counter for SparseContractor {
    fn count(&mut self, n: &Network, fixed: &HashMap<usize, bool>) -> Result<u64> {
        self.peak_rows = 0;
        for st in &n.schedule {
            if let Step::Hint(_) = st {
                return Err(Error::new(
                    "ecdl: SparseContractor does not support hinted schedules; use DFSCounter",
                ));
            }
        }
        // Last vertex index touching each edge, for early summing-out.
        let mut last: Vec<isize> = vec![-1; n.num_edges];
        for (i, v) in n.vertices.iter().enumerate() {
            for e in v.edges() {
                last[e] = i as isize;
            }
        }
        let mut t = Table {
            edges: Vec::new(),
            pos: HashMap::new(),
            rows: HashMap::from([(Vec::new(), 1u64)]),
        };
        for (i, v) in n.vertices.iter().enumerate() {
            t = match v {
                Vertex::Support(v) => join_support(&t, v, fixed),
                Vertex::Func(v) => apply_func(&t, v, fixed)?,
                Vertex::Pred(v) => apply_pred(&t, v)?,
            };
            if t.rows.len() > self.peak_rows {
                self.peak_rows = t.rows.len();
            }
            if self.max_rows > 0 && t.rows.len() > self.max_rows {
                return Err(Error::new(format!(
                    "ecdl: intermediate table exceeded {} rows",
                    self.max_rows
                )));
            }
            t = sum_out(t, &|e| last[e] > i as isize);
            if t.rows.is_empty() {
                return Ok(0);
            }
        }
        let mut total: u64 = 0;
        for c in t.rows.values() {
            total = total.wrapping_add(*c);
        }
        // Edges touched by no vertex are free unless fixed.
        for e in 0..last.len() {
            if last[e] == -1 && !fixed.contains_key(&e) {
                total = total.wrapping_mul(2);
            }
        }
        Ok(total)
    }
}

fn new_table(edges: Vec<usize>) -> Table {
    let pos = edges.iter().enumerate().map(|(i, &e)| (e, i)).collect();
    Table {
        edges,
        pos,
        rows: HashMap::new(),
    }
}

fn join_support(t: &Table, v: &SupportVertex, fixed: &HashMap<usize, bool>) -> Table {
    let mut shared: Vec<usize> = Vec::new(); // indices into v.e bound in t
    let mut shared_pos: Vec<usize> = Vec::new(); // their positions in t
    let mut fresh: Vec<usize> = Vec::new(); // indices into v.e not yet bound
    for (i, e) in v.e.iter().enumerate() {
        if let Some(&p) = t.pos.get(e) {
            shared.push(i);
            shared_pos.push(p);
        } else {
            fresh.push(i);
        }
    }
    // Group the vertex rows (restricted to fixed bits) by their shared part.
    let mut groups: HashMap<Vec<u8>, Vec<Vec<u8>>> = HashMap::new();
    for r in &v.rows {
        let mut ok = true;
        for (i, e) in v.e.iter().enumerate() {
            if let Some(&b) = fixed.get(e) {
                if b != r[i] {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        let sk: Vec<u8> = shared.iter().map(|&i| bit(r[i])).collect();
        let fk: Vec<u8> = fresh.iter().map(|&i| bit(r[i])).collect();
        groups.entry(sk).or_default().push(fk);
    }
    let mut edges = t.edges.clone();
    for &i in &fresh {
        edges.push(v.e[i]);
    }
    let mut out = new_table(edges);
    let mut sk = vec![0u8; shared.len()];
    for (key, &c) in &t.rows {
        for (j, &p) in shared_pos.iter().enumerate() {
            sk[j] = key[p];
        }
        if let Some(g) = groups.get(&sk) {
            for fk in g {
                let mut nk = key.clone();
                nk.extend_from_slice(fk);
                let slot = out.rows.entry(nk).or_insert(0);
                *slot = slot.wrapping_add(c);
            }
        }
    }
    crate::opcount::bump("contraction_rows_uncharged", out.rows.len() as u64);
    out
}

fn apply_func(t: &Table, v: &FuncVertex, fixed: &HashMap<usize, bool>) -> Result<Table> {
    let mut in_pos = Vec::with_capacity(v.in_.len());
    for &e in &v.in_ {
        match t.pos.get(&e) {
            Some(&p) => in_pos.push(p),
            None => {
                return Err(Error::new(format!(
                    "ecdl: schedule reaches a function vertex before input edge {} is bound",
                    e
                )))
            }
        }
    }
    let mut out_pos: Vec<isize> = Vec::with_capacity(v.out.len()); // -1 when the output edge is new
    let mut edges = t.edges.clone();
    for &e in &v.out {
        if let Some(&p) = t.pos.get(&e) {
            out_pos.push(p as isize);
        } else {
            out_pos.push(-1);
            edges.push(e);
        }
    }
    let mut out = new_table(edges);
    let mut inp = vec![false; v.in_.len()];
    for (key, &c) in &t.rows {
        for (i, &p) in in_pos.iter().enumerate() {
            inp[i] = key[p] == 1;
        }
        let res = match (v.f)(&inp) {
            Some(r) => r,
            None => continue,
        };
        let mut ext: Vec<u8> = Vec::with_capacity(v.out.len());
        let mut ok = true;
        for (i, &b) in res.iter().enumerate() {
            let e = v.out[i];
            if let Some(&fb) = fixed.get(&e) {
                if fb != b {
                    ok = false;
                    break;
                }
            }
            if out_pos[i] >= 0 {
                if (key[out_pos[i] as usize] == 1) != b {
                    ok = false;
                    break;
                }
            } else {
                ext.push(bit(b));
            }
        }
        if ok {
            let mut nk = key.clone();
            nk.extend_from_slice(&ext);
            let slot = out.rows.entry(nk).or_insert(0);
            *slot = slot.wrapping_add(c);
        }
    }
    crate::opcount::bump("contraction_rows_uncharged", out.rows.len() as u64);
    Ok(out)
}

fn apply_pred(t: &Table, v: &PredVertex) -> Result<Table> {
    let mut pos = Vec::with_capacity(v.e.len());
    for &e in &v.e {
        match t.pos.get(&e) {
            Some(&p) => pos.push(p),
            None => {
                return Err(Error::new(format!(
                    "ecdl: schedule reaches a predicate vertex before edge {} is bound",
                    e
                )))
            }
        }
    }
    let mut out = new_table(t.edges.clone());
    let mut vals = vec![false; v.e.len()];
    for (key, &c) in &t.rows {
        for (i, &p) in pos.iter().enumerate() {
            vals[i] = key[p] == 1;
        }
        if (v.p)(&vals) {
            let slot = out.rows.entry(key.clone()).or_insert(0);
            *slot = slot.wrapping_add(c);
        }
    }
    crate::opcount::bump("contraction_rows_uncharged", out.rows.len() as u64);
    Ok(out)
}

/// sumOut projects the table onto the edges for which keep is true.
fn sum_out(t: Table, keep: &dyn Fn(usize) -> bool) -> Table {
    let mut keep_idx: Vec<usize> = Vec::new();
    let mut edges: Vec<usize> = Vec::new();
    for (i, &e) in t.edges.iter().enumerate() {
        if keep(e) {
            keep_idx.push(i);
            edges.push(e);
        }
    }
    if edges.len() == t.edges.len() {
        return t;
    }
    let mut out = new_table(edges);
    let mut buf = vec![0u8; keep_idx.len()];
    for (key, &c) in &t.rows {
        for (j, &i) in keep_idx.iter().enumerate() {
            buf[j] = key[i];
        }
        let slot = out.rows.entry(buf.clone()).or_insert(0);
        *slot = slot.wrapping_add(c);
    }
    out
}

/// FindWitness turns a counting solver into a search solver by
/// self-reduction: it fixes the given edges one bit at a time, keeping the
/// branch whose count stays positive. It returns `Ok(None)` if there is no
/// solution (Go: nil map, nil error).
pub fn find_witness(
    c: &mut dyn Counter,
    n: &Network,
    edges: &[usize],
) -> Result<Option<HashMap<usize, bool>>> {
    let mut fixed: HashMap<usize, bool> = HashMap::new();
    let total = c.count(n, &fixed)?;
    if total == 0 {
        return Ok(None);
    }
    for &e in edges {
        fixed.insert(e, false);
        let k = c.count(n, &fixed)?;
        if k == 0 {
            fixed.insert(e, true);
        }
    }
    Ok(Some(fixed))
}

/// Go: bit(b bool) byte.
pub fn bit(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

/// Go: bitsKey. One byte (0/1) per bool.
pub fn bits_key(vals: &[bool]) -> Vec<u8> {
    vals.iter().map(|&b| bit(b)).collect()
}
