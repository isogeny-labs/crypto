//! Low-degree gadgets for modular arithmetic (Go: gadgets.go).
//!
//! A field element of b bits is split into n blocks of t = ceil(sqrt b) bits.
//! Every gadget is a set of vertices that each read O(b^(3/4)) bits, and the
//! gadgets use O(b) edges in total. Field multiplication checks the integer
//! identity a*b - q*p - c = 0 (with witness q). The two convolutions are
//! computed with a number-theoretic transform of length L >= 2n over a prime
//! Q, factored four-step style as L = L1*L2. That gives two stages of about
//! sqrt(L) vertices each, and each vertex sees about sqrt(L) blocks. A carry
//! chain then checks that the coefficients sum to zero.
//!
//! This is our own construction, guided by the paper's one-line description
//! ("a two-stage Fourier-transform decomposition"); the paper's gadget is not
//! reproduced here.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

use crate::schedule::{EdgeHint, Step};
use crate::tensor::{FuncVertex, Network, PredVertex, SupportVertex, Vertex};

/// Arith holds the parameters for arithmetic modulo P.
#[derive(Debug, Clone)]
pub struct Arith {
    pub p: BigUint,
    /// b
    pub bits: usize,
    /// block width t
    pub t: usize,
    /// number of blocks n
    pub n: usize,
    /// transform length, a power of two >= 2n
    pub l: usize,
    pub l1: usize,
    pub l2: usize,
    /// NTT prime, Q = 1 mod L and Q > 4 n 4^t
    pub q: BigUint,
    /// bit width of transform values
    pub wq: usize,
    /// bit width of signed carries
    pub we: usize,

    /// omega^e for e in [0, L)
    pub omega: Vec<BigUint>,
    /// omega^-e for e in [0, L)
    pub omega_inv: Vec<BigUint>,
    pub l_inv: BigUint,
    /// transform of p's blocks
    pub p_hat: Vec<BigUint>,
    /// block widths of a field element
    pub widths: Vec<usize>,
}

/// Shared handle to an `Arith`. Closures stored in vertices/hints capture a
/// clone (Go captures the pointer). `Arith::new` returns this.
pub type ArithRef = Rc<Arith>;

/// Bit length of a non-negative machine integer (Go: big.NewInt(x).BitLen()).
fn bitlen(x: u64) -> usize {
    (64 - x.leading_zeros()) as usize
}

/// 2^k - 1.
fn low_mask(k: usize) -> BigUint {
    (BigUint::one() << k) - BigUint::one()
}

impl Arith {
    /// NewArith chooses the block size, transform and NTT prime for modulus p.
    /// The NTT prime search uses `is_probable_prime(&q, 30)` (num-bigint has no
    /// primality test).
    pub fn new(p: &BigUint) -> ArithRef {
        let b = p.bits() as usize;
        let mut t = 1usize;
        while t * t < b {
            t += 1;
        }
        let n = b.div_ceil(t);
        let mut l = 1usize;
        while l < 2 * n {
            l *= 2;
        }
        let lg = l.trailing_zeros() as usize;
        let l1 = 1usize << (lg / 2);
        let l2 = l / l1;
        let mut widths = Vec::with_capacity(n);
        for i in 0..n {
            let mut w = t;
            if i == n - 1 {
                w = b - t * (n - 1);
            }
            widths.push(w);
        }

        // Q = k L + 1 > 4 n 2^(2t), prime.
        let bound = BigUint::from(4 * n) << (2 * t);
        let lbig = BigUint::from(l);
        let mut k = &bound / &lbig;
        let q = loop {
            k += 1u32;
            let q = &k * &lbig + 1u32;
            if is_probable_prime(&q, 30) {
                break q;
            }
        };
        let wq = q.bits() as usize;
        // Carries satisfy |e| < n 2^t + 2; store them in two's complement.
        let we = ((BigUint::from(n + 2) << t).bits() as usize) + 1;

        // omega of order exactly L: g^((Q-1)/L) with omega^(L/2) = -1.
        let qm1 = &q - 1u32;
        let e = &qm1 / &lbig;
        let mut g = 2u32;
        let omega = loop {
            let om = BigUint::from(g).modpow(&e, &q);
            if om.modpow(&BigUint::from(l / 2), &q) == qm1 {
                break om;
            }
            g += 1;
        };
        let oi = mod_inverse(&omega, &q).expect("omega invertible");
        let mut om_v: Vec<BigUint> = Vec::with_capacity(l);
        let mut oi_v: Vec<BigUint> = Vec::with_capacity(l);
        om_v.push(BigUint::one());
        oi_v.push(BigUint::one());
        for i in 1..l {
            let a = (&om_v[i - 1] * &omega) % &q;
            let c = (&oi_v[i - 1] * &oi) % &q;
            om_v.push(a);
            oi_v.push(c);
        }
        let l_inv = mod_inverse(&lbig, &q).expect("L invertible mod Q");
        let mut ar = Arith {
            p: p.clone(),
            bits: b,
            t,
            n,
            l,
            l1,
            l2,
            q,
            wq,
            we,
            omega: om_v,
            omega_inv: oi_v,
            l_inv,
            p_hat: Vec::new(),
            widths,
        };
        ar.p_hat = ar.ntt(&ar.blocks_of(p));
        Rc::new(ar)
    }

    /// blocksOf splits x >= 0 into L blocks of t bits (zero beyond n).
    pub fn blocks_of(&self, x: &BigUint) -> Vec<BigUint> {
        let mask = low_mask(self.t);
        (0..self.l).map(|i| (x >> (i * self.t)) & &mask).collect()
    }

    /// ntt is the direct O(L^2) transform, used for constants only.
    pub fn ntt(&self, x: &[BigUint]) -> Vec<BigUint> {
        (0..self.l)
            .map(|k| {
                let mut s = BigUint::zero();
                for (i, xi) in x.iter().enumerate() {
                    s += xi * &self.omega[(i * k) % self.l];
                }
                s % &self.q
            })
            .collect()
    }

    /// omega^(e mod L). (Go takes int and normalises negatives; every call
    /// site is non-negative, so usize.)
    pub fn w(&self, e: usize) -> &BigUint {
        &self.omega[e % self.l]
    }

    /// omega^-(e mod L).
    pub fn w_inv(&self, e: usize) -> &BigUint {
        &self.omega_inv[e % self.l]
    }
}

/// Fields of a `Var`.
pub struct VarInner {
    pub name: String,
    pub widths: Vec<usize>,
    /// edges written by the defining step
    pub src: Vec<Vec<usize>>,
    /// fresh edges per consumer, joined to src at Finalize
    pub uses: Vec<Vec<Vec<usize>>>,
    /// src already has a vertex endpoint
    pub producer: bool,
    pub defined: bool,
    /// schedule slot reserved for fan-out vertices
    pub copy_step: usize,
}

/// Var is a multi-bit value carried by edges, split into blocks. The Go code
/// uses `*Var` as a mutable shared handle with pointer identity (map keys,
/// mutated by Finalize); this is `Rc<RefCell<VarInner>>` with pointer
/// `PartialEq`/`Eq`/`Hash`. Cloning a `Var` clones the handle.
#[derive(Clone)]
pub struct Var(pub Rc<RefCell<VarInner>>);

impl PartialEq for Var {
    fn eq(&self, other: &Var) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for Var {}
impl Hash for Var {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (Rc::as_ptr(&self.0) as usize).hash(state)
    }
}

impl Var {
    /// Width is the total number of bits.
    pub fn width(&self) -> usize {
        self.0.borrow().widths.iter().sum()
    }

    /// Edges returns the defining edges, LSB first (valid after Finalize).
    pub fn edges(&self) -> Vec<usize> {
        let mut out = Vec::new();
        for blk in &self.0.borrow().src {
            out.extend_from_slice(blk);
        }
        out
    }

    pub fn name(&self) -> String {
        self.0.borrow().name.clone()
    }

    /// Block widths (clone of `widths`).
    pub fn widths(&self) -> Vec<usize> {
        self.0.borrow().widths.clone()
    }
}

/// Operand is a field-element input to a gadget: a Var or a constant
/// (Go: struct{V *Var; C *big.Int} with exactly one set).
#[derive(Clone)]
pub enum Operand {
    Var(Var),
    Const(BigUint),
}

/// Hint generator over unsigned values: values of the read Vars -> candidate
/// values of the written Vars (one `Vec<BigUint>` per candidate, aligned with
/// the writes). A candidate that does not fit its Var's width is dropped.
/// (Go uses *big.Int; every value the Go hints produce is non-negative.)
pub type VarHintFn = Rc<dyn Fn(&[BigUint]) -> Vec<Vec<BigUint>>>;

pub struct VarHint {
    pub reads: Vec<Var>,
    pub writes: Vec<Var>,
    pub gen: VarHintFn,
}

/// One builder step (Go: bstep with exactly one field set).
pub enum BStep {
    Vertex(Vertex),
    Hint(VarHint),
    /// fan-out vertices for this Var go here
    Fan(Var),
}

/// Builder assembles a gadget network and its schedule. Each Var is defined
/// once (by a hint or a support vertex) before it is used; Finalize joins the
/// consumers of each Var to its defining edges, with per-block copy vertices
/// when there are too many to share an edge.
#[derive(Default)]
pub struct Builder {
    pub(crate) next: usize,
    pub(crate) steps: Vec<BStep>,
    /// Exhaustive makes every hint enumerate all values of the Vars it writes
    /// instead of the computed witness. Only for testing at tiny sizes: it
    /// checks that the vertices alone pin every witness down.
    pub exhaustive: bool,
}

/// Term is coef * V in a linear check.
#[derive(Clone)]
pub struct Term {
    pub coef: i64,
    pub v: Var,
}

/// Edge renumbering used by `finalize`: aliases first, then dense ids in
/// order of first use.
struct Renum {
    alias: HashMap<usize, usize>,
    renum: HashMap<usize, usize>,
}

impl Renum {
    fn id(&mut self, e: usize) -> usize {
        let e = match self.alias.get(&e) {
            Some(&a) => a,
            None => e,
        };
        if let Some(&r) = self.renum.get(&e) {
            return r;
        }
        let r = self.renum.len();
        self.renum.insert(e, r);
        r
    }

    fn map_all(&mut self, es: &[usize]) -> Vec<usize> {
        es.iter().map(|&e| self.id(e)).collect()
    }
}

fn var_edges(vs: &[Var]) -> Vec<usize> {
    let mut out = Vec::new();
    for v in vs {
        out.extend(v.edges());
    }
    out
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    /// Go: b.edges(k): allocate k fresh edge ids.
    pub(crate) fn alloc_edges(&mut self, k: usize) -> Vec<usize> {
        let e: Vec<usize> = (self.next..self.next + k).collect();
        self.next += k;
        e
    }

    pub fn new_var(&mut self, name: &str, widths: &[usize]) -> Var {
        let mut src = Vec::with_capacity(widths.len());
        for &w in widths {
            src.push(self.alloc_edges(w));
        }
        Var(Rc::new(RefCell::new(VarInner {
            name: name.to_string(),
            widths: widths.to_vec(),
            src,
            uses: Vec::new(),
            producer: false,
            defined: false,
            copy_step: 0,
        })))
    }

    /// FieldVar allocates a field element of ar's block layout.
    pub fn field_var(&mut self, ar: &ArithRef, name: &str) -> Var {
        self.new_var(name, &ar.widths)
    }

    /// Go: b.add(v Vertex): append a vertex step.
    pub(crate) fn add_vertex(&mut self, v: Vertex) {
        self.steps.push(BStep::Vertex(v));
    }

    fn add_func(
        &mut self,
        in_: Vec<usize>,
        out: Vec<usize>,
        f: impl Fn(&[bool]) -> Option<Vec<bool>> + 'static,
    ) {
        self.add_vertex(Vertex::Func(FuncVertex {
            in_,
            out,
            f: Rc::new(f),
        }));
    }

    fn add_pred(&mut self, e: Vec<usize>, p: impl Fn(&[bool]) -> bool + 'static) {
        self.add_vertex(Vertex::Pred(PredVertex { e, p: Rc::new(p) }));
    }

    /// Panics "ecdl: Var {name} defined twice".
    pub(crate) fn mark_defined(&mut self, v: &Var) {
        {
            let mut vi = v.0.borrow_mut();
            if vi.defined {
                panic!("ecdl: Var {} defined twice", vi.name);
            }
            vi.defined = true;
            vi.copy_step = self.steps.len();
        }
        self.steps.push(BStep::Fan(v.clone()));
    }

    /// Hint defines the writes Vars from the reads Vars. Panics "ecdl: hint
    /// reads undefined Var {name}".
    pub fn hint(
        &mut self,
        reads: &[Var],
        writes: &[Var],
        gen: impl Fn(&[BigUint]) -> Vec<Vec<BigUint>> + 'static,
    ) {
        for r in reads {
            if !r.0.borrow().defined {
                panic!("ecdl: hint reads undefined Var {}", r.name());
            }
        }
        self.steps.push(BStep::Hint(VarHint {
            reads: reads.to_vec(),
            writes: writes.to_vec(),
            gen: Rc::new(gen),
        }));
        for w in writes {
            self.mark_defined(w);
        }
    }

    /// Hint with a nil generator (Go passes `nil`): valid only in exhaustive
    /// mode or when the network is never evaluated; evaluating it in
    /// non-exhaustive mode panics, as Go's nil-func call does.
    pub fn hint_nil(&mut self, reads: &[Var], writes: &[Var]) {
        self.hint(reads, writes, |_| {
            panic!("ecdl: hint has a nil generator (non-exhaustive evaluation)")
        });
    }

    /// Support defines vars by a support vertex listing their joint values.
    /// Panics "ecdl: support value does not fit".
    pub fn support(&mut self, vars: &[Var], rows: &[Vec<BigUint>]) {
        let mut edges = Vec::new();
        for v in vars {
            edges.extend(v.edges());
        }
        let mut brows: Vec<Vec<bool>> = vec![Vec::new(); rows.len()];
        for (i, row) in rows.iter().enumerate() {
            for (j, v) in vars.iter().enumerate() {
                match uint_bits(&row[j], v.width()) {
                    Some(bs) => brows[i].extend(bs),
                    None => panic!("ecdl: support value does not fit"),
                }
            }
        }
        self.add_vertex(Vertex::Support(SupportVertex::new(edges, brows)));
        for v in vars {
            v.0.borrow_mut().producer = true;
            self.mark_defined(v);
        }
    }

    /// Go: b.use(v). Returns fresh edges, block by block, for one consumer of
    /// v. (`use` is a Rust keyword.) Panics "ecdl: use of undefined Var {name}".
    pub(crate) fn use_var(&mut self, v: &Var) -> Vec<Vec<usize>> {
        if !v.0.borrow().defined {
            panic!("ecdl: use of undefined Var {}", v.name());
        }
        let widths = v.widths();
        let blk: Vec<Vec<usize>> = widths.iter().map(|&w| self.alloc_edges(w)).collect();
        v.0.borrow_mut().uses.push(blk.clone());
        blk
    }

    /// Finalize produces the network. Vars stay valid: their Edges() refer to
    /// the final numbering. Call at most once per Builder. Panics "ecdl: Var
    /// {name} is defined by a hint but never constrained".
    pub fn finalize(&mut self) -> Network {
        let mut alias: HashMap<usize, usize> = HashMap::new();
        let mut fan_verts: HashMap<usize, Vec<FuncVertex>> = HashMap::new();
        let mut seen: HashSet<Var> = HashSet::new();
        let mut seen_order: Vec<Var> = Vec::new();
        for st in &self.steps {
            let v = match st {
                BStep::Fan(v) => v,
                _ => continue,
            };
            if seen.contains(v) {
                continue;
            }
            seen.insert(v.clone());
            seen_order.push(v.clone());
            let vi = v.0.borrow();
            let k = vi.uses.len();
            let mut ends = k;
            if vi.producer {
                ends += 1;
            }
            if k == 0 && !vi.producer {
                panic!(
                    "ecdl: Var {} is defined by a hint but never constrained",
                    vi.name
                );
            }
            if ends <= 2 {
                for u in &vi.uses {
                    for j in 0..u.len() {
                        for (i, &e) in u[j].iter().enumerate() {
                            alias.insert(e, vi.src[j][i]);
                        }
                    }
                }
                continue;
            }
            for j in 0..vi.widths.len() {
                let mut out = Vec::new();
                for u in &vi.uses {
                    out.extend_from_slice(&u[j]);
                }
                fan_verts.entry(vi.copy_step).or_default().push(FuncVertex {
                    in_: vi.src[j].clone(),
                    out,
                    f: Rc::new(move |input: &[bool]| {
                        let mut o = Vec::with_capacity(k * input.len());
                        for _ in 0..k {
                            o.extend_from_slice(input);
                        }
                        Some(o)
                    }),
                });
            }
        }

        // Renumber edges densely, applying aliases.
        let mut rn = Renum {
            alias,
            renum: HashMap::new(),
        };
        let mut n = Network::default();
        fn push(rn: &mut Renum, n: &mut Network, v: &Vertex) {
            let nv = match v {
                Vertex::Support(s) => {
                    Vertex::Support(SupportVertex::new(rn.map_all(&s.e), s.rows.clone()))
                }
                Vertex::Func(f) => Vertex::Func(FuncVertex {
                    in_: rn.map_all(&f.in_),
                    out: rn.map_all(&f.out),
                    f: f.f.clone(),
                }),
                Vertex::Pred(p) => Vertex::Pred(PredVertex {
                    e: rn.map_all(&p.e),
                    p: p.p.clone(),
                }),
            };
            n.vertices.push(nv);
            n.schedule.push(Step::Vertex(n.vertices.len() - 1));
        }
        for (i, st) in self.steps.iter().enumerate() {
            match st {
                BStep::Vertex(v) => push(&mut rn, &mut n, v),
                BStep::Hint(h) => {
                    let reads = rn.map_all(&var_edges(&h.reads));
                    let writes = rn.map_all(&var_edges(&h.writes));
                    n.schedule
                        .push(Step::Hint(self.edge_hint(h, reads, writes)));
                }
                BStep::Fan(_) => {
                    if let Some(fvs) = fan_verts.remove(&i) {
                        for fv in fvs {
                            push(&mut rn, &mut n, &Vertex::Func(fv));
                        }
                    }
                }
            }
        }
        // Vars keep the final numbering for their defining edges.
        for v in &seen_order {
            let mut vi = v.0.borrow_mut();
            for j in 0..vi.src.len() {
                let mapped = rn.map_all(&vi.src[j]);
                vi.src[j] = mapped;
            }
        }
        n.num_edges = rn.renum.len();
        n
    }

    /// Go: edgeHint. Wraps a VarHint into a bit-level EdgeHint honouring
    /// `self.exhaustive`.
    pub(crate) fn edge_hint(&self, h: &VarHint, reads: Vec<usize>, writes: Vec<usize>) -> EdgeHint {
        let exhaustive = self.exhaustive;
        let read_w: Vec<usize> = h.reads.iter().map(|r| r.width()).collect();
        let write_w: Vec<usize> = h.writes.iter().map(|w| w.width()).collect();
        let gen = h.gen.clone();
        EdgeHint {
            reads,
            writes,
            gen: Rc::new(move |input: &[bool]| {
                if exhaustive {
                    let total: usize = write_w.iter().sum();
                    let mut out = Vec::with_capacity(1usize << total);
                    for mask in 0..(1usize << total) {
                        let row: Vec<bool> = (0..total).map(|i| (mask >> i) & 1 == 1).collect();
                        out.push(row);
                    }
                    return out;
                }
                let mut vals = Vec::with_capacity(read_w.len());
                let mut off = 0;
                for &w in &read_w {
                    vals.push(bits_uint(&input[off..off + w]));
                    off += w;
                }
                let mut out = Vec::new();
                for cand in gen(&vals) {
                    let mut row = Vec::new();
                    let mut ok = true;
                    for (i, &w) in write_w.iter().enumerate() {
                        match uint_bits(&cand[i], w) {
                            Some(bs) => row.extend(bs),
                            None => {
                                ok = false;
                                break;
                            }
                        }
                    }
                    if ok {
                        out.push(row);
                    }
                }
                out
            }),
        }
    }

    /// MulCheck adds the constraint x*y = c (mod p) with 0 <= c < p. If
    /// define_c, c is defined here (and range-checked); otherwise c must
    /// already be defined and range-checked by the caller.
    pub fn mul_check(&mut self, ar: &ArithRef, x: &Operand, y: &Operand, c: &Var, define_c: bool) {
        let q = self.new_var("q", &ar.widths);
        let mut reads: Vec<Var> = Vec::new();
        for o in [x, y] {
            if let Operand::Var(v) = o {
                reads.push(v.clone());
            }
        }
        if !define_c {
            reads.push(c.clone());
        }
        let mut writes = vec![q.clone()];
        if define_c {
            writes.push(c.clone());
        }
        let p = ar.p.clone();
        {
            let reads_c = reads.clone();
            let (x, y, c) = (x.clone(), y.clone(), c.clone());
            self.hint(&reads, &writes, move |input: &[BigUint]| {
                let mut vals: HashMap<Var, BigUint> = HashMap::new();
                for (i, r) in reads_c.iter().enumerate() {
                    vals.insert(r.clone(), input[i].clone());
                }
                let prod = operand_value(&x, &vals) * operand_value(&y, &vals);
                if define_c {
                    let (qq, cc) = prod.div_rem(&p);
                    return vec![vec![qq, cc]];
                }
                let cv = &vals[&c];
                if &prod < cv {
                    return Vec::new();
                }
                let diff = prod - cv;
                let (qq, rem) = diff.div_rem(&p);
                if !rem.is_zero() {
                    return Vec::new();
                }
                vec![vec![qq]]
            });
        }
        if define_c {
            self.range_check(ar, c, &ar.p, false);
        }

        // Stage 1 for each variable operand: vertex n2 reads blocks L2*n1 + n2
        // and outputs Y[n2][k1] = w^(n2 k1) sum_n1 x[L2 n1 + n2] w^(L2 n1 k1).
        // ops[i][n2][k1]: WQ edges, None when identically zero
        let mut ops: Vec<Vec<Option<Vec<Vec<usize>>>>> = Vec::new();
        let mut const_hat: Vec<Vec<BigUint>> = Vec::new(); // transforms of constants, by position
        let mut is_const: Vec<bool> = Vec::new();
        for o in [x.clone(), y.clone(), Operand::Var(q.clone())] {
            let ov = match o {
                Operand::Const(cv) => {
                    const_hat.push(ar.ntt(&ar.blocks_of(&(cv % &ar.p))));
                    is_const.push(true);
                    ops.push(Vec::new());
                    continue;
                }
                Operand::Var(v) => v,
            };
            const_hat.push(Vec::new());
            is_const.push(false);
            let blk = self.use_var(&ov);
            let mut sy: Vec<Option<Vec<Vec<usize>>>> = vec![None; ar.l2];
            for n2 in 0..ar.l2 {
                let mut inp: Vec<usize> = Vec::new();
                let mut idx: Vec<usize> = Vec::new();
                let mut wid: Vec<usize> = Vec::new();
                for n1 in 0..ar.l1 {
                    let i = ar.l2 * n1 + n2;
                    if i < ar.n {
                        inp.extend_from_slice(&blk[i]);
                        idx.push(n1);
                        wid.push(ar.widths[i]);
                    }
                }
                if inp.is_empty() {
                    continue;
                }
                let mut ys: Vec<Vec<usize>> = Vec::with_capacity(ar.l1);
                let mut out: Vec<usize> = Vec::new();
                for _k1 in 0..ar.l1 {
                    let e = self.alloc_edges(ar.wq);
                    out.extend_from_slice(&e);
                    ys.push(e);
                }
                sy[n2] = Some(ys);
                let ar2 = ar.clone();
                self.add_func(inp, out, move |bits_in: &[bool]| {
                    let ar = &ar2;
                    let parts = split_bits(bits_in, &wid);
                    let mut res = Vec::with_capacity(ar.l1 * ar.wq);
                    for k1 in 0..ar.l1 {
                        let mut s = BigUint::zero();
                        for (j, &n1) in idx.iter().enumerate() {
                            s += bits_uint(parts[j]) * ar.w(ar.l2 * n1 * k1);
                        }
                        let s = ((s % &ar.q) * ar.w(n2 * k1)) % &ar.q;
                        res.extend(uint_bits(&s, ar.wq).expect("fits WQ"));
                    }
                    Some(res)
                });
            }
            ops.push(sy);
        }
        let const_hat = Rc::new(const_hat);
        let is_const = Rc::new(is_const);

        // Stage 2 + pointwise product + inverse stage 1: vertex k1 finishes the
        // transforms X[k1 + L1 k2], forms D = X_x X_y - X_q pHat, and outputs
        // Z[k1][r] = w^(-r k1) sum_k2 D[k1 + L1 k2] w^(-L1 r k2).
        let mut z: Vec<Vec<Vec<usize>>> = Vec::with_capacity(ar.l1); // z[k1][r]
        for k1 in 0..ar.l1 {
            let mut inp: Vec<usize> = Vec::new();
            let mut srcs: Vec<(usize, usize)> = Vec::new();
            for (oi, s) in ops.iter().enumerate() {
                if is_const[oi] {
                    continue;
                }
                for n2 in 0..ar.l2 {
                    if let Some(ys) = &s[n2] {
                        inp.extend_from_slice(&ys[k1]);
                        srcs.push((oi, n2));
                    }
                }
            }
            let mut zk: Vec<Vec<usize>> = Vec::with_capacity(ar.l2);
            let mut out: Vec<usize> = Vec::new();
            for _r in 0..ar.l2 {
                let e = self.alloc_edges(ar.wq);
                out.extend_from_slice(&e);
                zk.push(e);
            }
            z.push(zk);
            let ar2 = ar.clone();
            let is_const = is_const.clone();
            let const_hat = const_hat.clone();
            self.add_func(inp, out, move |bits_in: &[bool]| {
                let ar = &ar2;
                // yv[op][n2], None means zero
                let mut yv: Vec<Vec<Option<BigUint>>> = vec![vec![None; ar.l2]; 3];
                for (i, s) in srcs.iter().enumerate() {
                    let v = bits_uint(&bits_in[i * ar.wq..(i + 1) * ar.wq]);
                    if v >= ar.q {
                        return None;
                    }
                    yv[s.0][s.1] = Some(v);
                }
                let xf = |op: usize, k2: usize| -> BigUint {
                    if is_const[op] {
                        return const_hat[op][k1 + ar.l1 * k2].clone();
                    }
                    let mut s = BigUint::zero();
                    for (n2, y) in yv[op].iter().enumerate() {
                        if let Some(y) = y {
                            s += y * ar.w(ar.l1 * n2 * k2);
                        }
                    }
                    s % &ar.q
                };
                let mut dv: Vec<BigUint> = Vec::with_capacity(ar.l2);
                for k2 in 0..ar.l2 {
                    let a = (xf(0, k2) * xf(1, k2)) % &ar.q;
                    let b = (xf(2, k2) * &ar.p_hat[k1 + ar.l1 * k2]) % &ar.q;
                    dv.push((a + &ar.q - b) % &ar.q);
                }
                let mut res = Vec::with_capacity(ar.l2 * ar.wq);
                for r in 0..ar.l2 {
                    let mut s = BigUint::zero();
                    for (k2, d) in dv.iter().enumerate() {
                        s += d * ar.w_inv(ar.l1 * r * k2);
                    }
                    let s = ((s % &ar.q) * ar.w_inv(r * k1)) % &ar.q;
                    res.extend(uint_bits(&s, ar.wq).expect("fits WQ"));
                }
                Some(res)
            });
        }

        // Inverse stage 2: vertex r outputs d[r + L2 s] = L^-1 sum_k1 w^(-L2 s k1) Z[k1][r].
        let mut d: Vec<Vec<usize>> = vec![Vec::new(); ar.l];
        for r in 0..ar.l2 {
            let mut inp: Vec<usize> = Vec::new();
            let mut out: Vec<usize> = Vec::new();
            for k1 in 0..ar.l1 {
                inp.extend_from_slice(&z[k1][r]);
            }
            for s in 0..ar.l1 {
                d[r + ar.l2 * s] = self.alloc_edges(ar.wq);
                out.extend_from_slice(&d[r + ar.l2 * s]);
            }
            let ar2 = ar.clone();
            self.add_func(inp, out, move |bits_in: &[bool]| {
                let ar = &ar2;
                let mut zv: Vec<BigUint> = Vec::with_capacity(ar.l1);
                for k1 in 0..ar.l1 {
                    let v = bits_uint(&bits_in[k1 * ar.wq..(k1 + 1) * ar.wq]);
                    if v >= ar.q {
                        return None;
                    }
                    zv.push(v);
                }
                let mut res = Vec::with_capacity(ar.l1 * ar.wq);
                for s in 0..ar.l1 {
                    let mut v = BigUint::zero();
                    for (k1, zk) in zv.iter().enumerate() {
                        v += zk * ar.w_inv(ar.l2 * s * k1);
                    }
                    let v = ((v % &ar.q) * &ar.l_inv) % &ar.q;
                    res.extend(uint_bits(&v, ar.wq).expect("fits WQ"));
                }
                Some(res)
            });
        }

        // Carry chain: lift d_j to (-Q/2, Q/2), subtract c_j, add the carry, and
        // require divisibility by 2^t; the last position must leave zero.
        let cblk = self.use_var(c);
        let half_q: BigUint = &ar.q >> 1usize;
        let mut carry: Option<Vec<usize>> = None;
        for j in 0..ar.l {
            let mut inp = d[j].clone();
            let mut cw = 0usize;
            if j < ar.n {
                inp.extend_from_slice(&cblk[j]);
                cw = ar.widths[j];
            }
            let has_carry = carry.is_some();
            if let Some(cr) = &carry {
                inp.extend_from_slice(cr);
            }
            let ar2 = ar.clone();
            let half_q2 = half_q.clone();
            let value = move |bits_in: &[bool]| -> BigInt {
                let ar = &ar2;
                let mut v = BigInt::from(bits_uint(&bits_in[..ar.wq]));
                if v > BigInt::from(half_q2.clone()) {
                    v -= BigInt::from(ar.q.clone());
                }
                v -= BigInt::from(bits_uint(&bits_in[ar.wq..ar.wq + cw]));
                if has_carry {
                    v += bits_signed(&bits_in[ar.wq + cw..]);
                }
                v
            };
            if j == ar.l - 1 {
                self.add_pred(inp, move |bits_in: &[bool]| value(bits_in).is_zero());
                break;
            }
            let nc = self.alloc_edges(ar.we);
            carry = Some(nc.clone());
            let ar3 = ar.clone();
            self.add_func(inp, nc, move |bits_in: &[bool]| {
                let v = value(bits_in);
                let m = BigInt::one() << ar3.t;
                if !v.mod_floor(&m).is_zero() {
                    return None;
                }
                signed_bits(&v.div_floor(&m), ar3.we)
            });
        }
    }

    /// LinCheck adds the constraint out + sum coef_i v_i + konst = 0 (mod p),
    /// where out may be None. If define_out, out is defined here and
    /// range-checked to [0, p). All term Vars must already be range-checked by
    /// their definers. The witness k = (out + sum + konst)/p travels along the
    /// block chain.
    pub fn lin_check(
        &mut self,
        ar: &ArithRef,
        out: Option<&Var>,
        define_out: bool,
        terms: &[Term],
        konst: &BigInt,
    ) {
        let kc: BigUint = konst
            .mod_floor(&BigInt::from(ar.p.clone()))
            .to_biguint()
            .expect("non-negative");
        let mut sum_abs: u64 = 2;
        for t in terms {
            sum_abs += t.coef.unsigned_abs();
        }
        let wk = bitlen(sum_abs) + 1;
        let wc = bitlen(2 * sum_abs + 4) + 1;
        let k = self.new_var("k", &[wk]);

        let mut reads: Vec<Var> = terms.iter().map(|t| t.v.clone()).collect();
        let mut writes: Vec<Var> = Vec::new();
        if let Some(o) = out {
            if !define_out {
                reads.push(o.clone());
            }
        }
        if define_out {
            writes.push(out.expect("define_out requires out").clone());
        }
        writes.push(k.clone());
        {
            let p = BigInt::from(ar.p.clone());
            let coefs: Vec<i64> = terms.iter().map(|t| t.coef).collect();
            let kc = BigInt::from(kc.clone());
            let out_present = out.is_some();
            self.hint(&reads, &writes, move |input: &[BigUint]| {
                let mut s = kc.clone();
                for (i, &cf) in coefs.iter().enumerate() {
                    s += BigInt::from(cf) * BigInt::from(input[i].clone());
                }
                let o: BigInt = if define_out {
                    (-&s).mod_floor(&p)
                } else if out_present {
                    BigInt::from(input[coefs.len()].clone())
                } else {
                    BigInt::zero()
                };
                s += &o;
                let (kk, rem) = s.div_mod_floor(&p);
                if !rem.is_zero() {
                    return Vec::new();
                }
                // k is stored in two's complement but the hint layer is unsigned.
                let kb = match signed_bits(&kk, wk) {
                    Some(kb) => kb,
                    None => return Vec::new(),
                };
                if define_out {
                    vec![vec![o.to_biguint().expect("non-negative"), bits_uint(&kb)]]
                } else {
                    vec![vec![bits_uint(&kb)]]
                }
            });
        }
        if define_out {
            self.range_check(ar, out.unwrap(), &ar.p, false);
        }

        let out_blk: Option<Vec<Vec<usize>>> = out.map(|o| self.use_var(o));
        let term_blk: Vec<Vec<Vec<usize>>> = terms.iter().map(|t| self.use_var(&t.v)).collect();
        let mut k_edges = self.use_var(&k).remove(0);
        let pb = ar.blocks_of(&ar.p);
        let cb = ar.blocks_of(&kc);
        let mut carry: Option<Vec<usize>> = None;
        for j in 0..ar.n {
            let w = ar.widths[j];
            let mut inp: Vec<usize> = Vec::new();
            let mut widths: Vec<usize> = Vec::new();
            if let Some(ob) = &out_blk {
                inp.extend_from_slice(&ob[j]);
                widths.push(w);
            }
            for tb in &term_blk {
                inp.extend_from_slice(&tb[j]);
                widths.push(w);
            }
            inp.extend_from_slice(&k_edges);
            widths.push(wk);
            let has_carry = carry.is_some();
            if let Some(cr) = &carry {
                inp.extend_from_slice(cr);
                widths.push(wc);
            }
            let coefs: Vec<i64> = terms.iter().map(|t| t.coef).collect();
            let out_present = out.is_some();
            let cbj = BigInt::from(cb[j].clone());
            let pbj = BigInt::from(pb[j].clone());
            let value: Rc<dyn Fn(&[bool]) -> (BigInt, Vec<bool>)> =
                Rc::new(move |bits_in: &[bool]| {
                    let parts = split_bits(bits_in, &widths);
                    let mut v = cbj.clone();
                    let mut pi = 0;
                    if out_present {
                        v += BigInt::from(bits_uint(parts[0]));
                        pi = 1;
                    }
                    for (i, &cf) in coefs.iter().enumerate() {
                        v += BigInt::from(cf) * BigInt::from(bits_uint(parts[pi + i]));
                    }
                    let kbits = parts[pi + coefs.len()];
                    v -= bits_signed(kbits) * &pbj;
                    if has_carry {
                        v += bits_signed(parts[pi + coefs.len() + 1]);
                    }
                    (v, kbits.to_vec())
                });
            if j == ar.n - 1 {
                self.add_pred(inp, move |bits_in: &[bool]| value(bits_in).0.is_zero());
                break;
            }
            let ncarry = self.alloc_edges(wc);
            let k_next = self.alloc_edges(wk);
            carry = Some(ncarry.clone());
            let t = ar.t;
            let mut outv = ncarry;
            outv.extend_from_slice(&k_next);
            self.add_func(inp, outv, move |bits_in: &[bool]| {
                let (v, kbits) = value(bits_in);
                let m = BigInt::one() << t;
                if !v.mod_floor(&m).is_zero() {
                    return None;
                }
                let mut cbits = signed_bits(&v.div_floor(&m), wc)?;
                cbits.extend_from_slice(&kbits);
                Some(cbits)
            });
            k_edges = k_next;
        }
    }

    /// RangeCheck adds the constraint v < U (and v != 0 if nonzero), comparing
    /// block by block from the least significant end. Panics "ecdl: range
    /// bound too large".
    pub fn range_check(&mut self, ar: &ArithRef, v: &Var, u: &BigUint, nonzero: bool) {
        let blk = self.use_var(v);
        let ub = ar.blocks_of(u);
        if u.bits() as usize > ar.n * ar.t {
            panic!("ecdl: range bound too large");
        }
        let widths = v.widths();
        let mut state: Option<Vec<usize>> = None; // [lt, nz]
        for j in 0..widths.len() {
            let mut inp = blk[j].clone();
            if let Some(s) = &state {
                inp.extend_from_slice(s);
            }
            let w = widths[j];
            let has = state.is_some();
            let ubj = ub[j].clone();
            let step = move |bits_in: &[bool]| -> (bool, bool) {
                let x = bits_uint(&bits_in[..w]);
                let (mut lt, mut nz) = (false, false);
                if has {
                    lt = bits_in[w];
                    nz = bits_in[w + 1];
                }
                let cmp = x.cmp(&ubj);
                (
                    cmp == std::cmp::Ordering::Less || (cmp == std::cmp::Ordering::Equal && lt),
                    nz || !x.is_zero(),
                )
            };
            if j == widths.len() - 1 {
                self.add_pred(inp, move |bits_in: &[bool]| {
                    let (lt, nz) = step(bits_in);
                    lt && (!nonzero || nz)
                });
                break;
            }
            let st = self.alloc_edges(2);
            state = Some(st.clone());
            self.add_func(inp, st, move |bits_in: &[bool]| {
                let (lt, nz) = step(bits_in);
                Some(vec![lt, nz])
            });
        }
    }

    /// NewBit allocates a 1-bit Var.
    pub fn new_bit(&mut self, name: &str) -> Var {
        self.new_var(name, &[1])
    }

    /// IsZero defines the bit z = [v == 0], checked by an OR chain over v's
    /// blocks.
    pub fn is_zero(&mut self, v: &Var) -> Var {
        let z = self.new_bit("iszero");
        self.hint(
            std::slice::from_ref(v),
            std::slice::from_ref(&z),
            |input: &[BigUint]| vec![vec![bool_int(input[0].is_zero())]],
        );
        let blk = self.use_var(v);
        let ze = self.use_var(&z).remove(0);
        let widths = v.widths();
        let mut nz: Option<Vec<usize>> = None;
        for j in 0..blk.len() {
            let mut inp = blk[j].clone();
            if let Some(s) = &nz {
                inp.extend_from_slice(s);
            }
            let (w, has) = (widths[j], nz.is_some());
            let any_set = move |bits_in: &[bool]| -> bool {
                let x = bits_in[..w].iter().any(|&b| b);
                x || (has && bits_in[w])
            };
            if j == blk.len() - 1 {
                let mut e = inp;
                e.extend_from_slice(&ze);
                self.add_pred(e, move |bits_in: &[bool]| {
                    any_set(bits_in) != bits_in[bits_in.len() - 1]
                });
                break;
            }
            let nx = self.alloc_edges(1);
            nz = Some(nx.clone());
            self.add_func(inp, nx, move |bits_in: &[bool]| {
                Some(vec![any_set(bits_in)])
            });
        }
        z
    }

    /// flagChain checks ok(j, f, bits of edgesAt(j)) for every block j, passing
    /// the flag bit f from vertex to vertex so each vertex reads it once.
    pub(crate) fn flag_chain(
        &mut self,
        f: &Var,
        n: usize,
        edges_at: impl Fn(usize) -> Vec<usize>,
        ok: impl Fn(usize, bool, &[bool]) -> bool + 'static,
    ) {
        let ok = Rc::new(ok);
        let mut fe = self.use_var(f).remove(0);
        for j in 0..n {
            let mut inp = fe.clone();
            inp.extend(edges_at(j));
            let ok2 = ok.clone();
            let check = move |bits_in: &[bool]| ok2(j, bits_in[0], &bits_in[1..]);
            if j == n - 1 {
                self.add_pred(inp, check);
                break;
            }
            let next = self.alloc_edges(1);
            self.add_func(inp, next.clone(), move |bits_in: &[bool]| {
                if !check(bits_in) {
                    return None;
                }
                Some(vec![bits_in[0]])
            });
            fe = next;
        }
    }

    /// Select defines the field element w = f ? u : v.
    pub fn select(&mut self, ar: &ArithRef, f: &Var, u: &Operand, v: &Operand) -> Var {
        let w = self.field_var(ar, "select");
        let mut reads = vec![f.clone()];
        for o in [u, v] {
            if let Operand::Var(x) = o {
                reads.push(x.clone());
            }
        }
        {
            let reads_c = reads.clone();
            let (u, v) = (u.clone(), v.clone());
            self.hint(
                &reads,
                std::slice::from_ref(&w),
                move |input: &[BigUint]| {
                    let mut vals: HashMap<Var, BigUint> = HashMap::new();
                    for (i, r) in reads_c.iter().enumerate() {
                        vals.insert(r.clone(), input[i].clone());
                    }
                    if !input[0].is_zero() {
                        vec![vec![operand_value(&u, &vals)]]
                    } else {
                        vec![vec![operand_value(&v, &vals)]]
                    }
                },
            );
        }
        // Edges for a variable operand, constant blocks for a constant one.
        let blocks = |b: &mut Builder, o: &Operand| -> (Option<Vec<Vec<usize>>>, Vec<BigUint>) {
            match o {
                Operand::Var(x) => (Some(b.use_var(x)), Vec::new()),
                Operand::Const(c) => (None, ar.blocks_of(&(c % &ar.p))),
            }
        };
        let (ue, uc) = blocks(self, u);
        let (ve, vc) = blocks(self, v);
        let we = self.use_var(&w);
        let (u_var, v_var) = (ue.is_some(), ve.is_some());
        let ar2 = ar.clone();
        self.flag_chain(
            f,
            ar.n,
            |j| {
                let mut e = Vec::new();
                if let Some(ue) = &ue {
                    e.extend_from_slice(&ue[j]);
                }
                if let Some(ve) = &ve {
                    e.extend_from_slice(&ve[j]);
                }
                e.extend_from_slice(&we[j]);
                e
            },
            move |j: usize, flag: bool, vals: &[bool]| {
                let wd = ar2.widths[j];
                let mut off = 0;
                let mut take = |is_var: bool, c: &[BigUint]| -> BigUint {
                    if !is_var {
                        return c[j].clone();
                    }
                    let x = bits_uint(&vals[off..off + wd]);
                    off += wd;
                    x
                };
                let uj = take(u_var, &uc);
                let vj = take(v_var, &vc);
                let wj = bits_uint(&vals[off..off + wd]);
                if flag {
                    wj == uj
                } else {
                    wj == vj
                }
            },
        );
        w
    }

    /// ZeroIf adds the constraint f => v = 0.
    pub fn zero_if(&mut self, _ar: &ArithRef, f: &Var, v: &Var) {
        let ve = self.use_var(v);
        self.flag_chain(
            f,
            ve.len(),
            |j| ve[j].clone(),
            |_j: usize, flag: bool, vals: &[bool]| {
                if !flag {
                    return true;
                }
                !vals.iter().any(|&b| b)
            },
        );
    }
}

/// Go: bitsUint. Reads an unsigned LSB-first integer.
pub fn bits_uint(vals: &[bool]) -> BigUint {
    let mut bytes = vec![0u8; vals.len().div_ceil(8)];
    for (i, &b) in vals.iter().enumerate() {
        if b {
            bytes[i / 8] |= 1 << (i % 8);
        }
    }
    BigUint::from_bytes_le(&bytes)
}

/// Go: uintBits. Writes 0 <= x < 2^w as w bits, LSB first; `None` if it does
/// not fit.
pub fn uint_bits(x: &BigUint, w: usize) -> Option<Vec<bool>> {
    if x.bits() as usize > w {
        return None;
    }
    Some((0..w).map(|i| x.bit(i as u64)).collect())
}

/// Go: bitsSigned. Reads a two's-complement LSB-first integer.
pub fn bits_signed(vals: &[bool]) -> BigInt {
    let mut x = BigInt::from(bits_uint(vals));
    if !vals.is_empty() && vals[vals.len() - 1] {
        x -= BigInt::one() << vals.len();
    }
    x
}

/// Go: signedBits. Writes -2^(w-1) <= x < 2^(w-1) in two's complement; `None`
/// if out of range.
pub fn signed_bits(x: &BigInt, w: usize) -> Option<Vec<bool>> {
    let lim = BigInt::one() << (w - 1);
    if x >= &lim || x < &(-&lim) {
        return None;
    }
    let mut y = x.clone();
    if y.is_negative() {
        y += BigInt::one() << w;
    }
    uint_bits(&y.to_biguint().expect("non-negative"), w)
}

/// Go: splitBits(vals, widths...). Cuts vals into consecutive pieces of the
/// given widths.
pub fn split_bits<'a>(vals: &'a [bool], widths: &[usize]) -> Vec<&'a [bool]> {
    let mut out = Vec::with_capacity(widths.len());
    let mut off = 0;
    for &w in widths {
        out.push(&vals[off..off + w]);
        off += w;
    }
    out
}

/// Go: concat(es ...[]int).
pub fn concat(parts: &[&[usize]]) -> Vec<usize> {
    let mut out = Vec::new();
    for p in parts {
        out.extend_from_slice(p);
    }
    out
}

/// Go: operandValue. Constant, or the value of the Var in `vals`.
pub fn operand_value(o: &Operand, vals: &HashMap<Var, BigUint>) -> BigUint {
    match o {
        Operand::Var(v) => vals[v].clone(),
        Operand::Const(c) => c.clone(),
    }
}

/// Go: boolInt.
pub fn bool_int(x: bool) -> BigUint {
    if x {
        BigUint::one()
    } else {
        BigUint::zero()
    }
}

/// Miller-Rabin with `rounds` bases (fixed small primes first, then pseudo-
/// random-free deterministic bases); replaces Go's `ProbablyPrime(30)`
/// (num-bigint has none). Must be correct for the NTT primes (< ~2^64 at
/// toy sizes, larger at scale).
pub fn is_probable_prime(n: &BigUint, rounds: usize) -> bool {
    const SMALL: [u32; 40] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
    ];
    let two = BigUint::from(2u32);
    if n < &two {
        return false;
    }
    for &sp in &SMALL {
        let spb = BigUint::from(sp);
        if n == &spb {
            return true;
        }
        if (n % &spb).is_zero() {
            return false;
        }
    }
    // n - 1 = d 2^s with d odd.
    let nm1 = n - 1u32;
    let s = nm1.trailing_zeros().unwrap_or(0);
    let d = &nm1 >> s;
    let nbases = rounds.clamp(1, SMALL.len());
    'base: for &a in SMALL.iter().take(nbases) {
        let mut x = BigUint::from(a).modpow(&d, n);
        if x.is_one() || x == nm1 {
            continue;
        }
        for _ in 1..s {
            x = (&x * &x) % n;
            if x == nm1 {
                continue 'base;
            }
        }
        return false;
    }
    true
}

/// Replaces Go's `ModInverse`: a^-1 mod m via extended Euclid; `None` when
/// gcd(a, m) != 1 (m need not be prime).
pub fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    if m.is_zero() {
        return None;
    }
    let mi = BigInt::from(m.clone());
    let (mut r0, mut r1) = (mi.clone(), BigInt::from(a.clone()).mod_floor(&mi));
    let (mut t0, mut t1) = (BigInt::zero(), BigInt::one());
    while !r1.is_zero() {
        let qt = &r0 / &r1;
        let r2 = &r0 - &qt * &r1;
        r0 = std::mem::replace(&mut r1, r2);
        let t2 = &t0 - &qt * &t1;
        t0 = std::mem::replace(&mut t1, t2);
    }
    if !r0.is_one() {
        return None;
    }
    t0.mod_floor(&mi).to_biguint()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schedule::DFSCounter;
    use crate::tensor::Counter;

    #[test]
    fn mul_check_counts_pairs() {
        let p = 13u32;
        let ar = Arith::new(&BigUint::from(p));
        let mut b = Builder::new();
        let x = b.field_var(&ar, "x");
        let y = b.field_var(&ar, "y");
        let rows: Vec<Vec<BigUint>> = (0..p).map(|v| vec![BigUint::from(v)]).collect();
        b.support(std::slice::from_ref(&x), &rows);
        b.support(std::slice::from_ref(&y), &rows);
        let c = b.field_var(&ar, "c");
        b.mul_check(&ar, &Operand::Var(x), &Operand::Var(y), &c, true);
        let net = b.finalize();
        let cnt = DFSCounter.count(&net, &HashMap::new()).unwrap();
        assert_eq!(cnt, (p * p) as u64);
    }

    #[test]
    fn prime_and_inverse() {
        assert!(is_probable_prime(&BigUint::from(1_000_000_007u64), 30));
        assert!(!is_probable_prime(&BigUint::from(1_000_000_007u64 * 3), 30));
        let inv = mod_inverse(&BigUint::from(3u32), &BigUint::from(11u32)).unwrap();
        assert_eq!(inv, BigUint::from(4u32));
    }
}
