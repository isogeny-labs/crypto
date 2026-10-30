# PORTING.md: Go `ecdltensor` (package `ecdl`) -> Rust `ecdl-tensor-ic`

The skeleton in `src/` is the contract. Porting agents fill in `todo!()` bodies
and add `#[cfg(test)]`/`tests/` code. **Do not change any public signature, type,
field or module placement**; if you believe one is wrong, report it instead.
Private helpers local to one module may be added freely.
`lib.rs` has `#![allow(unused_variables, unused_mut, dead_code)]` as a skeleton
aid; the last agent removes it and fixes the warnings.

Go source: `scratchpad/go-src` (read-only). Crate-wide: `crate::Error(String)`
(Display, std Error, `Error::new(msg)`) and `crate::Result<T>`; messages stay
the Go "ecdl: ..." strings. Rust panics replace Go `panic`.

## Global conventions

| Go | Rust |
|---|---|
| `int` (edge ids, counts, widths, lengths) | `usize` (`i64` only for `Term.coef`) |
| `uint64` | `u64` (counts wrap: use `wrapping_add/mul`, as Go does) |
| `*big.Int` unsigned | `num_bigint::BigUint`; signed (carries, `konst`, `bitsSigned`) `BigInt` |
| `(T, bool)` / nil results | `Option<T>` (`DecodePoint`, `LeftKernelVector`, `FindWitness`, `FuncVertex.F`) |
| `(T, error)` | `crate::Result<T>` |
| `*rand.Rand` | `&mut rand::rngs::StdRng` (`SeedableRng::seed_from_u64`) |
| `rng.Int63n(n)` | `field::rand_below(rng, n: u64) -> u64` (`gen_range(0..n)`) |
| `rng.Intn(n)` | `rng.gen_range(0..n)` |
| `rng.Read(key)` | `rng.fill_bytes(&mut key)` |
| `map[int]bool` fixed | `HashMap<usize, bool>` |
| `*big.Int.ProbablyPrime(30)` | `gadgets::is_probable_prime(&q, 30)` (must be written; num-bigint has none) |
| `big.Int.ModInverse` | `gadgets::mod_inverse(&a, &m) -> Option<BigUint>` |
| closures `func(...)` stored in vertices | `Rc<dyn Fn(...)>` (single-threaded; nothing is `Send`) |

Design choices:

* **Vertex = enum**, not a trait: Go always type-switches over exactly
  `SupportVertex | FuncVertex | PredVertex`, and `Finalize` rebuilds them.
  `Vertex::{Support, Func, Pred}` hold the plain structs by value.
  `Vertex::edges()` returns a fresh `Vec<usize>` (FuncVertex = In then Out).
* **Counter = trait** with `fn count(&mut self, &Network, &HashMap<usize,bool>) -> Result<u64>`
  (`&mut` because `SparseContractor.peak_rows`). `Solver.counter: Box<dyn Counter>`.
  Go `DFSCounter{}.Count(...)` is `DFSCounter.count(...)` (unit struct, `Default`).
* **Schedule steps reference vertices by index**: `Step::Vertex(usize)` indexes
  `Network.vertices`; Go stored the same pointer in both. `Builder::finalize`
  pushes vertex `i` and `Step::Vertex(i)` together. `Network.schedule` empty
  == Go `nil` (DFSCounter then uses every vertex in order).
* **Hints**: `EdgeHint.gen: Rc<dyn Fn(&[bool]) -> Vec<Vec<bool>>>` (bit level),
  `VarHintFn: Rc<dyn Fn(&[BigUint]) -> Vec<Vec<BigUint>>>` (Var level, all
  values non-negative, as in Go; `lin_check` encodes the signed `k` via
  `bits_uint(signed_bits(..))` exactly like Go). `Builder::hint` takes
  `impl Fn(..) + 'static`; Go's `nil` gen (GadgetShapes) becomes `|_| Vec::new()`.
  Exhaustive mode (`Builder.exhaustive`) enumerates all `2^total` bit patterns
  of the written Vars, unchanged.
* **Var** = `Rc<RefCell<VarInner>>` handle with pointer identity (`Eq`/`Hash`),
  because Go mutates it in `Finalize` and keys maps by `*Var`. Cloning shares.
  `Var::edges()` works after `finalize` with final numbering.
* **Arith** is built by `Arith::new(&BigUint) -> ArithRef` where
  `ArithRef = Rc<Arith>`; every gadget takes `&ArithRef` because closures clone
  it. `Arith.w(e)`/`w_inv(e)` take `usize` (all Go call sites are non-negative).
* **Operand** is an enum `Var(Var) | Const(BigUint)` (Go struct with one nil).
* **FieldPoint.inf: Option<Var>** (Go nil `*Var`).
* **Curve is `Copy`** so closures (`FuncVertex.F` in `relation_network`,
  gadget hints) capture an owned copy. `Point` is `Copy + Eq + Hash`
  (used as a `HashMap` key; `INFINITY` is canonical x=y=0).
* **Solver owns** `curve: Curve`, `rng: StdRng`, `counter: Box<dyn Counter>`.
  `Solver::solve` returns `Result<(xi, Stats)>`; Go's partial `Stats` on error
  are dropped. `logf: Option<Box<dyn Fn(&str)>>` receives the formatted message.
* **Shared helpers are `pub` (some `#[doc(hidden)]`-able) rather than
  `pub(crate)`** so integration tests in `tests/` (which replace the in-package
  Go tests) can use them: `bit`, `bits_key` (tensor), `bits_uint`, `uint_bits`,
  `bits_signed`, `signed_bits`, `split_bits`, `concat`, `operand_value`,
  `bool_int` (gadgets), `bi64` (pointgadget), `mul_mod/add_mod/sub_mod/pow_mod/inv_mod/rand_below` (field).
* Edge numbering, schedule order, `Rows` order, hint candidate order must be
  kept as in Go (counts do not depend on them but debugging parity does).
  HashMap iteration order never affects results (sums only).
* Go's `math/rand` streams are not reproducible; `StdRng::seed_from_u64` is used.

## Module ownership and identifier map

### `field` (field.go)
| Go | Rust |
|---|---|
| `mulMod(a,b,m uint64) uint64` | `pub fn mul_mod(a: u64, b: u64, m: u64) -> u64` (u128) |
| `addMod`, `subMod` | `pub fn add_mod / sub_mod(a: u64, b: u64, m: u64) -> u64` (addMod must handle `a+b` overflow like Go) |
| `powMod(a,e,m)` | `pub fn pow_mod(a: u64, e: u64, m: u64) -> u64` |
| `invMod(a,m)` | `pub fn inv_mod(a: u64, m: u64) -> u64` |
| `sqrtMod(a,p) (uint64,bool)` | `pub fn sqrt_mod(a: u64, p: u64) -> Option<u64>` (Tonelli–Shanks) |
| `IsPrime(n)` | `pub fn is_prime(n: u64) -> bool` (re-exported at crate root) |
| (new; Go `uint64(rng.Int63n(int64(n)))`) | `pub fn rand_below(rng: &mut StdRng, n: u64) -> u64` |

### `curve` (curve.go)
| Go | Rust |
|---|---|
| `Curve{P,A,B,Order uint64; G Point}` | `Curve { p, a, b, order: u64, g: Point }` (`Copy, Default`) |
| `Point{X,Y uint64; Inf bool}` | `Point { x, y: u64, inf: bool }` (`Copy, Eq, Hash, Default`) |
| `Infinity` | `pub const INFINITY: Point` |
| `(*Curve).FieldBits() int` | `fn field_bits(&self) -> usize` (`64 - (p-1).leading_zeros()`) |
| `rhs(x)` | `pub(crate) fn rhs(&self, x: u64) -> u64` |
| `OnCurve(pt)` | `fn on_curve(&self, pt: Point) -> bool` |
| `Neg(pt)` | `fn neg(&self, pt: Point) -> Point` |
| `Add(s,t)` | `fn add(&self, s: Point, t: Point) -> Point` |
| `Mul(k,pt)` | `fn mul(&self, k: u64, pt: Point) -> Point` |
| `Points()` | `fn points(&self) -> Vec<Point>` |
| `WindowPoints(xBound)` | `fn window_points(&self, x_bound: u64) -> Vec<Point>` (sqrt per x) |
| `RandomPrimeOrderCurve(rng,b)` | `fn random_prime_order_curve(rng: &mut StdRng, b: usize) -> Result<Curve>` |

### `tensor` (tensor.go)
| Go | Rust |
|---|---|
| `Vertex` interface | `enum Vertex { Support(SupportVertex), Func(FuncVertex), Pred(PredVertex) }`, `fn edges(&self) -> Vec<usize>`, `fn holds(&self, &[bool]) -> bool` |
| `SupportVertex{E,Rows,set}` / `NewSupportVertex` | `struct SupportVertex { e: Vec<usize>, rows: Vec<Vec<bool>>, pub(crate) set: HashSet<Vec<u8>> }`, `SupportVertex::new(edges, rows) -> SupportVertex` |
| `FuncVertex{In,Out,F}` | `struct FuncVertex { in_: Vec<usize>, out: Vec<usize>, f: FuncFn }`, `FuncFn = Rc<dyn Fn(&[bool]) -> Option<Vec<bool>>>` |
| `PredVertex{E,P}` | `struct PredVertex { e: Vec<usize>, p: PredFn }`, `PredFn = Rc<dyn Fn(&[bool]) -> bool>` |
| `Network{NumEdges,Vertices,Schedule}` | `struct Network { num_edges: usize, vertices: Vec<Vertex>, schedule: Vec<Step> }` (`Default`) |
| `NetworkStats` | `struct NetworkStats { edges, vertices, max_degree: usize, is_graph: bool }` (`Copy, Default`) |
| `(*Network).Stats()` | `fn stats(&self) -> NetworkStats` |
| `Counter` | `trait Counter { fn count(&mut self, n: &Network, fixed: &HashMap<usize,bool>) -> Result<u64>; }` |
| `BruteForceCounter` | `struct BruteForceCounter` (unit) impl `Counter` |
| `agrees` | private `fn agrees(&[bool], &HashMap<usize,bool>) -> bool` |
| `SparseContractor{MaxRows,PeakRows}` | `struct SparseContractor { max_rows: usize, peak_rows: usize }` impl `Counter` |
| `table`, `newTable`, `joinSupport`, `applyFunc`, `applyPred`, `sumOut` | private `Table`, `new_table`, `join_support`, `apply_func`, `apply_pred`, `sum_out` (signatures in file; body-private, may be reshaped) |
| `FindWitness(c,n,edges)` | `fn find_witness(c: &mut dyn Counter, n: &Network, edges: &[usize]) -> Result<Option<HashMap<usize,bool>>>` |
| `bit(b) byte` | `pub fn bit(b: bool) -> u8` |
| `bitsKey(vals) string` | `pub fn bits_key(vals: &[bool]) -> Vec<u8>` |

### `schedule` (schedule.go)
| Go | Rust |
|---|---|
| `Step{Vertex,Hint}` | `enum Step { Vertex(usize), Hint(EdgeHint) }` |
| `EdgeHint{Reads,Writes,Gen}` | `struct EdgeHint { reads, writes: Vec<usize>, gen: HintGenFn }`, `HintGenFn = Rc<dyn Fn(&[bool]) -> Vec<Vec<bool>>>` |
| `DFSCounter` | `struct DFSCounter` (unit) impl `Counter` |
| `dfs`, `bind`, `undo`, `read`, `run` | private `Dfs<'a>` with those methods (must undo the trail on every exit of `run`, as Go's `defer`) |

### `relation` (relation.go)
| Go | Rust |
|---|---|
| `(*Curve).PointWidth() int` | `impl Curve { fn point_width(&self) -> usize }` (in relation.rs) |
| `EncodePoint(pt)` | `fn encode_point(&self, pt: Point) -> Vec<bool>` |
| `DecodePoint(v) (Point, bool)` | `fn decode_point(&self, v: &[bool]) -> Option<Point>` |
| `FactorBases` | `struct FactorBases { sets: Vec<Vec<Point>>, points: Vec<Point>, index: HashMap<Point,usize>, keys: Vec<[u8;16]>, mods: Vec<u64>, affine: bool, alpha, beta, gamma, bound: Vec<u64>, p: u64 }` (all pub, `Default`) |
| `NewFactorBases(c,m,rng)` | `fn new_factor_bases(c: &Curve, m: usize, rng: &mut StdRng) -> FactorBases` |
| `NewAffineFactorBases` | `fn new_affine_factor_bases(c: &Curve, m: usize, rng: &mut StdRng) -> FactorBases` (delegates to the windowed constructor with k = 0) |
| `NewWindowedFactorBases(c,m,rng,k)` | `fn new_windowed_factor_bases(c: &Curve, m: usize, rng: &mut StdRng, k: usize) -> FactorBases` |
| `XBound []uint64`, `WindowBits int` | `x_bound: Vec<u64>`, `window_bits: usize` |
| `(*FactorBases).list(c)` / `listAll` / `listWindow` | `pub(crate) fn list` / `list_all` / `list_window` |
| `WindowCandidates(c,i)` | `fn window_candidates(&self, c: &Curve, i: usize) -> Vec<Point>` |
| `AffineHash(i,pt)` | `fn affine_hash(&self, i: usize, pt: Point) -> u64` |
| `Member(i,pt)` | `fn member(&self, i: usize, pt: Point) -> bool` (sha2::Sha256) |
| `RelationNetwork(c,fb,R)` | `fn relation_network(c: &Curve, fb: &FactorBases, r: Point) -> (Network, Vec<Vec<usize>>)` |
| (new, Rust only) | `fn mitm_find_relation(c: &Curve, fb: &FactorBases, r: Point, ...) -> Option<Vec<Point>>`: meet-in-the-middle elimination order; `Solver::mitm`, `ecdldemo -mitm`; `Solver::threads` / `ecdldemo -threads N` for parallel search, disabled while `opcount::active()` |

### `linalg` (linalg.go)

| Go | Rust |
|---|---|
| `LeftKernelVector(rows []map[int]uint64, ncols int, r uint64) ([]uint64, bool)` | `fn left_kernel_vector(...) -> Option<Vec<u64>>` (dense reference, same contract) |
| (Rust only) | `fn left_kernel_vector_sparse(...)`, used by the solver: sorted sparse rows, smallest-first pivots, deterministic |

### `solver` (solver.go)
| Go | Rust |
|---|---|
| `Relation{U,V,Points}` | `struct Relation { u, v: u64, points: Vec<Point> }` |
| `Stats{FactorBaseSize,Relations,Attempts,Restarts,Network,Elapsed}` | `struct Stats { factor_base_size, relations, attempts, restarts: usize, network: NetworkStats, elapsed: Duration }` |
| `Solver{Curve,M,Counter,Rng,MaxRestarts,Gadgets,Logf,arith}` | `struct Solver { curve: Curve, m: usize, counter: Box<dyn Counter>, rng: StdRng, max_restarts: usize, gadgets: bool, logf: Option<Box<dyn Fn(&str)>>, pub(crate) arith: Option<ArithRef> }` + `Solver::new(curve, m, counter, rng)` |
| `(*Solver).Solve(Q) (uint64, Stats, error)` | `fn solve(&mut self, q: Point) -> Result<(u64, Stats)>` |
| `findRelation(fb,R)` | `pub(crate) fn find_relation(&mut self, fb: &FactorBases, r: Point) -> Result<(Option<Relation>, NetworkStats)>` |
| `logf(format, args...)` | `pub(crate) fn logf(&self, msg: &str)` (callers `format!`) |
| `setSizes` | `pub(crate) fn set_sizes(fb: &FactorBases) -> Vec<usize>` |

In the gadget path `find_relation` builds `A` edges as
`concat(&[&pt.x.edges(), &pt.y.edges()])`, decodes with
`bits_uint(&vals[..b]).to_u64()` (use `to_u64_digits`/`ToPrimitive`).

### `gadgets` (gadgets.go)
| Go | Rust |
|---|---|
| `Arith` | `struct Arith { p: BigUint, bits, t, n, l, l1, l2: usize, q: BigUint, wq, we: usize, omega, omega_inv: Vec<BigUint>, l_inv: BigUint, p_hat: Vec<BigUint>, widths: Vec<usize> }` (all pub) |
| `NewArith(p)` | `Arith::new(p: &BigUint) -> ArithRef` (`ArithRef = Rc<Arith>`) |
| `blocksOf`, `ntt` | `fn blocks_of(&self, &BigUint) -> Vec<BigUint>`, `fn ntt(&self, &[BigUint]) -> Vec<BigUint>` |
| `w(e)`, `wInv(e)` | `fn w(&self, e: usize) -> &BigUint`, `fn w_inv(&self, e: usize) -> &BigUint` |
| `Var` (+ fields) | `struct Var(Rc<RefCell<VarInner>>)`; `VarInner { name, widths, src, uses, producer, defined, copy_step }`; methods `width()`, `edges()`, `name()`, `widths()` |
| `Operand{V,C}` | `enum Operand { Var(Var), Const(BigUint) }` |
| `varHint`, `bstep` | `VarHint { reads, writes: Vec<Var>, gen: VarHintFn }`, `enum BStep { Vertex(Vertex), Hint(VarHint), Fan(Var) }` |
| `Builder{next,steps,Exhaustive}` | `struct Builder { pub(crate) next, pub(crate) steps: Vec<BStep>, pub exhaustive: bool }` (`Default`, `Builder::new()`; tests use `Builder { exhaustive: true, ..Default::default() }`) |
| `b.edges(k)` | `pub(crate) fn alloc_edges(&mut self, k: usize) -> Vec<usize>` |
| `NewVar(name,widths)` | `fn new_var(&mut self, name: &str, widths: &[usize]) -> Var` |
| `FieldVar(ar,name)` | `fn field_var(&mut self, ar: &ArithRef, name: &str) -> Var` |
| `b.add(v)` | `pub(crate) fn add_vertex(&mut self, v: Vertex)` |
| `markDefined` | `pub(crate) fn mark_defined(&mut self, v: &Var)` |
| `Hint(reads,writes,gen)` | `fn hint(&mut self, reads: &[Var], writes: &[Var], gen: impl Fn(&[BigUint]) -> Vec<Vec<BigUint>> + 'static)` |
| `Support(vars,rows)` | `fn support(&mut self, vars: &[Var], rows: &[Vec<BigUint>])` |
| `use(v)` | `pub(crate) fn use_var(&mut self, v: &Var) -> Vec<Vec<usize>>` |
| `Finalize()` | `fn finalize(&mut self) -> Network` |
| `edgeHint` | `pub(crate) fn edge_hint(&self, h: &VarHint, reads: Vec<usize>, writes: Vec<usize>) -> EdgeHint` |
| `MulCheck(ar,x,y,c,defineC)` | `fn mul_check(&mut self, ar: &ArithRef, x: &Operand, y: &Operand, c: &Var, define_c: bool)` |
| `Term{Coef int; V *Var}` | `struct Term { coef: i64, v: Var }` |
| `LinCheck(ar,out,defineOut,terms,konst)` | `fn lin_check(&mut self, ar: &ArithRef, out: Option<&Var>, define_out: bool, terms: &[Term], konst: &BigInt)` |
| `RangeCheck(ar,v,U,nonzero)` | `fn range_check(&mut self, ar: &ArithRef, v: &Var, u: &BigUint, nonzero: bool)` |
| `NewBit(name)` | `fn new_bit(&mut self, name: &str) -> Var` |
| `IsZero(v)` | `fn is_zero(&mut self, v: &Var) -> Var` |
| `flagChain` | `pub(crate) fn flag_chain(&mut self, f: &Var, n: usize, edges_at: impl Fn(usize)->Vec<usize>, ok: impl Fn(usize,bool,&[bool])->bool + 'static)` |
| `Select(ar,f,u,v)` | `fn select(&mut self, ar: &ArithRef, f: &Var, u: &Operand, v: &Operand) -> Var` |
| `ZeroIf(ar,f,v)` | `fn zero_if(&mut self, ar: &ArithRef, f: &Var, v: &Var)` |
| `bitsUint`/`uintBits` | `fn bits_uint(&[bool]) -> BigUint` / `fn uint_bits(&BigUint, w: usize) -> Option<Vec<bool>>` |
| `bitsSigned`/`signedBits` | `fn bits_signed(&[bool]) -> BigInt` / `fn signed_bits(&BigInt, w: usize) -> Option<Vec<bool>>` |
| `splitBits(vals, widths...)` | `fn split_bits<'a>(vals: &'a [bool], widths: &[usize]) -> Vec<&'a [bool]>` |
| `concat(es ...[]int)` | `fn concat(parts: &[&[usize]]) -> Vec<usize>` |
| `operandValue` | `fn operand_value(o: &Operand, vals: &HashMap<Var, BigUint>) -> BigUint` |
| `boolInt` | `fn bool_int(x: bool) -> BigUint` |
| (new) | `fn is_probable_prime(n: &BigUint, rounds: usize) -> bool`, `fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint>` |

Hint-closure notes for `mul_check`/`lin_check`: values that can go negative
(`prod - c`, `s`) are computed in `BigInt` inside the closure (Euclidean
div/mod as Go's `DivMod`), converted to `BigUint` only after the sign check;
`lin_check` writes `k` as `bits_uint(&signed_bits(kk, wk)?)`.

### `pointgadget` (pointgadget.go)
| Go | Rust |
|---|---|
| `FieldPoint{X,Y,Inf}` | `struct FieldPoint { x: Var, y: Var, inf: Option<Var> }` (`Clone`) |
| `bi64(x)` | `pub fn bi64(x: u64) -> BigUint` |
| `AddGeneric(ar,p1,p2)` | `Builder::add_generic(&mut self, &ArithRef, p1: &FieldPoint, p2: &FieldPoint) -> FieldPoint` |
| `ValidityCheck(ar,c,pt)` | `Builder::validity_check(&mut self, &ArithRef, c: &Curve, pt: &FieldPoint)` |
| `EqualsConst(ar,v,k)` | `Builder::equals_const(&mut self, &ArithRef, v: &Var, k: &BigUint)` |
| `MembershipCheck(ar,fb,i,pt)` | `Builder::membership_check(&mut self, &ArithRef, fb: &FactorBases, i: usize, pt: &FieldPoint)` |
| `CompleteAdd(ar,c,p1,p2)` | `Builder::complete_add(&mut self, &ArithRef, c: &Curve, p1: &FieldPoint, p2: &FieldPoint) -> FieldPoint` |
| `GadgetRelationNetwork(c,ar,fb,R)` | `fn gadget_relation_network(c: &Curve, ar: &ArithRef, fb: &FactorBases, r: Point) -> (Network, Vec<FieldPoint>)` |
| `GadgetShapes(ar)` | `fn gadget_shapes(ar: &ArithRef) -> (NetworkStats, NetworkStats, NetworkStats)` (mul, add, point) |

(`impl Builder` blocks for these live in `pointgadget.rs`; they use only
`pub`/`pub(crate)` Builder API.)

### Binaries
`src/bin/ecdldemo.rs` (flags `-bits` (14), `-m` (2), `-seed` (now in ns), `-q`,
`-gadgets`) and `src/bin/ecdlscale.rs` (flag `-bits`, default
`"64,256,1024,4096,16384"`): `main` is `todo!()`; flag parsing helpers
(`parse_flags`, `parse_bits_flag`) are declared as `todo!()` too. Accept Go
flag syntax (`-x v`, `-x=v`, `--x v`, bare bool flags). The demo output lines
and format strings are copied from `cmd/*/main.go`.

### Tests (to port into `tests/`, Go names -> `tests/<file>.rs`)
`ecdl_test.go` -> `tests/ecdl.rs`; `gadget_test.go` -> `tests/gadgets.rs`.
`countWith` uses `DFSCounter.count` with `fixed` built from `Var::edges()` and
`uint_bits`. `randomNetwork` etc. are local test helpers.
