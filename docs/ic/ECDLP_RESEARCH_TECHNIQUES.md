# ECDLP index-calculus research technique matrix

This is the canonical inventory for the elliptic-curve index-calculus techniques
implemented, exercised, or queued in this repository. It maps a research name to
a concrete native implementation and states the boundary of that implementation.

This page is an inventory, not a performance claim. A component or stage result
does not establish an ECDLP speedup. Any comparison with Pollard rho still has to
follow [the repository's full-cost measurement rules](README.md), use the same
one-target workload and resource envelope, recover and verify the scalar, and
price every stage.

## Status vocabulary

| Status | Meaning |
|:--|:--|
| **Integrated** | Used by a native end-to-end or resumable ECDLP index-calculus path and covered by correctness checks. |
| **Experimental** | A working bounded research implementation or harness, but not a general production path. |
| **Component** | Reusable native code exists, but it is not connected to the main ECDLP pipeline for this role. |
| **Backlog** | No complete native implementation currently exists for this role. |

“Integrated” says that the path exists; it does not say it beats rho or scales to
a deployed curve.

## Stage summary

| Stage | Integrated | Experimental or component | Explicit backlog |
|:--|:--|:--|:--|
| Factor base | Subspace bases; Frobenius-invariant bases; signed-orbit quotienting; orbit representatives; trace-zero subspace policy | Hashed affine-threshold bases (`hashed-affine`, `ic.tensor_contraction_*`); higher-genus GHS/trace-zero experiments | General trace-zero-variety factor bases beyond the Koblitz subspace policy; a calibrated price for hashed-base contraction steps |
| Relation collection | Semaev systems; point decomposition; Weil restriction; F4/F5 and hybrid solving; symmetrised systems | XL; single/double/multi-large-prime experiments | Collector-specific adapters to the common multi-large-prime layer |
| Filtering | Deduplication; singleton peeling; clique removal; structured merging; Markowitz pivoting; large-prime graph/hypergraph filtering | Collector-specific partial-relation combination | None at the reusable filtering layer |
| Linear algebra | Parallel block Wiedemann; block Lanczos; black-box Krylov methods; serial/Rayon/sharded/worker SpMV | CUDA SpMV worker (hardware validation required) | Performance evidence for distributed/GPU deployments |
| Individual logarithm | Randomized target decomposition; signed-Frobenius recovery; budgeted recursive scheduler with large-prime children and orbit canonicalisation | Curve-specific recursive-descent adapters | Higher-genus GHS target descent |

## 1. Factor-base construction

### Subspace factor bases — Integrated

[`FactorBaseSpec::StandardSubspace`](../../src/cryptanalysis/koblitz_factor_base_search.rs)
and the divisor-family search provide reproducible linear-subspace factor
bases. The Diem demonstration independently provides the base-field
construction for extension-field curves in
[`diem_descent.rs`](../../src/cryptanalysis/diem_descent.rs).

### Frobenius-invariant bases — Integrated

[`koblitz_index_calculus.rs`](../../src/cryptanalysis/koblitz_index_calculus.rs)
constructs kernels of linearised polynomials obtained from factors of
\(x^n-1\), as well as Frobenius closures of seed spaces.
[`koblitz_factor_base_search.rs`](../../src/cryptanalysis/koblitz_factor_base_search.rs)
searches the divisor lattice and nonlinear Frobenius unions using measured
coverage and column counts.

### Quotient by \(P\mapsto -P\) and orbit representatives — Integrated

The Koblitz pipeline stores one unknown per signed Frobenius orbit. A point
\(\pi^k(Q)\) contributes \(\lambda^k\) to the representative column, while
negation changes the coefficient sign. The factor-base recipes preserve the
canonical orbit representatives needed to replay the same column assignment.

This is a representation quotient, not permission to drop relation witnesses:
every accepted decomposition is still re-added and verified in the curve group.

### Trace-zero constructions — Integrated for Koblitz subspace bases

The `koblitz-trace-zero` framework policy constructs a Frobenius-invariant
abscissa subspace, proves that every element is in the absolute-trace kernel,
and refuses a divisor containing a trace-one element before collection starts.
The factor-base search also detects this property and records its yield. See
[`ic_framework/plugins.rs`](../../src/cryptanalysis/ic_framework/plugins.rs)
and [`koblitz_factor_base_search.rs`](../../src/cryptanalysis/koblitz_factor_base_search.rs).
The GHS code in
[`ghs_descent.rs`](../../src/cryptanalysis/ghs_descent.rs) also provides an
end-to-end \(m=1\) trace descent and structural \(m=2\) machinery.

The repository does **not** yet contain a general trace-zero-variety ECDLP
pipeline. In particular, higher-genus GHS descent still needs the explicit
smooth model before its Jacobian path is end to end.

### Hashed factor bases (factor-base hashing) — Experimental

Membership is decided by a keyed hash, not by algebraic shape. A slot base is

\[
F_i = \{P \in \langle G\rangle : h_i(P) < \tau\},\qquad
h_i(P) = (\alpha_i\cdot\mathrm{encode}(P) + \beta_i) \bmod s,
\]

with \(s\) the first prime in \([2p, 4p]\), one independent \((\alpha_i, \beta_i)\)
per ordered slot, \(m\) slots, and \(\tau\) chosen so that
\(\mathbb{E}|F_i| = \max(\lceil r^{1/m}\rceil, 10m)\) (Bochum v0.2, Lemma 13).
A relation is \(R = P_1 + \dots + P_m\) with \(P_i \in F_i\). The base is
built by inverting the hash: for each \(t < \tau\) decode
\(z = \alpha^{-1}(t - \beta) \bmod s\) to a point and keep it if it lies in the
subgroup, so construction costs \(O(\tau)\) decodes with no \(O(p)\) scan and no
square-root table. Membership is one affine map and one comparison, which is
what makes it cheap to express as a low-degree predicate inside a Boolean
tensor network or a CNF. The price is that there is no smoothness notion:
decomposition becomes an unstructured \(m\)-list problem, and a hashed base
decomposes like a random set of its size (yield conservation, encyclopedia
§7 item 1), so the hash buys an encoding, not yield.

Four native implementations exist, all prime-field:

- **IC-framework plug-ins** in
  [`ic_framework/hashed.rs`](../../src/cryptanalysis/ic_framework/hashed.rs):
  the `hashed-affine` factor base (union of `slots` = 2..=8 slot bases, one
  column per negation orbit, parameters `seed`, `tau`, `s`, `steps`; limited
  to \(p < 2^{61}\)) and the `hashed-tensor` decomposition oracle (exact
  Boolean tensor contraction over the same slots). The oracle refuses a base
  built with different parameters, maps every witness back to base indices,
  and the shared relation loop re-adds and verifies it in the counted group.
  Selectable as `ic bench --factor-base hashed-affine --oracle hashed-tensor`;
  test [`tests/hashed_ic_pipeline.rs`](../../tests/hashed_ic_pipeline.rs).
  [`FRAMEWORK.md`](FRAMEWORK.md) ("Hashed-affine / hashed-tensor") has the
  parameter contract.
- **Reference reduction**
  [`hashed_ecdl.rs`](../../src/cryptanalysis/hashed_ecdl.rs) and the vendored
  [`crates/hashed-ecdl-tensor`](../../crates/hashed-ecdl-tensor/README.md):
  Bochum v0.2 §3.2–3.5 and Figure 2 with the October 25 manuscript's
  Appendix D.4 recursion, arbitrary precision, exact primality, replayable
  certificates. The algorithm source is mirrored byte-identically in
  `cryptanalysis` (`cryptanalysis::hashed_ecdl::tensor`;
  `cryptanalysis/docs/hashed-ecdl.md`).
- **Whole-point tensor IC**
  [`crates/ecdl-tensor-ic`](../../crates/ecdl-tensor-ic/README.md) (port of
  the Go `ecdltensor` prototype): keyed SHA-256 membership predicates by
  default, a random affine hash for the gadget path, an optional coordinate
  window \(x(A) < p \gg k\), forward or meet-in-the-middle contraction order.
  Registered in ecbench as `ic.tensor_contraction_v1` and
  `ic.tensor_contraction_cofactor_v1` with policy `DiagnosticOnly`
  ([`ecbench/optimization.rs`](../../src/cryptanalysis/ecbench/optimization.rs)).
- **CPU meet-in-the-middle oracle**
  [`crates/hashed-ecdl-cpu`](../../crates/hashed-ecdl-cpu/README.md) over the
  same ordered signed slot bases: persistent tuple-sum tables, Montgomery batch
  inversion, JSON certificates with independent replay
  ([`docs/ecbench/cpu-reduction.md`](../ecbench/cpu-reduction.md)).

**Boundary.** `run_pipeline` lists this arm's unpriced work by name:
`hashed_tensor_steps` is an uncalibrated charged-primitive count and the
tensor-curve certification is omitted from setup, so the `S` of a
`hashed-affine + hashed-tensor` row is a stage diagnostic, not a comparable
end-to-end cost. The recursive subexponential kernel has never been exercised:
every run falls back to direct summation (`root_u = 0`), because Appendix D.4's
admission needs \(\lambda \ge 2^{17}\), i.e. networks of at least
\(2^{131072}\) edges. The asymptotic claim is conditional on OpenAI's
Theorem D.1 and has not been audited here.

**Measured** (every number `[pcc:unverified]` unless the source says otherwise):

- Counts-only comparison, frozen protocol v2
  ([`research/hashed_ecdl_counts_20261031/`](../../research/hashed_ecdl_counts_20261031/results/RESULTS-v2-superseding.md)):
  `ic.tensor_contraction_v1` at \(m = 2\) against `rho.negation`, priced at 0,
  0.1 and 1 GAE per unpriced operation, costs 4.3·10³–9.5·10³× rho at 10 bits,
  2.7·10⁴–5.0·10⁴× at 12, 1.7·10⁵–3.0·10⁵× at 14 and 6.7·10⁵–1.2·10⁶× at 16;
  the ratio grows about 5.1× per two bits (log-log slope 1.17, 95% interval
  1.00–1.34). The native hashed arms (`ic.hashed_native_v1`, on branch
  `codex/hashed-ecdl-ecbench-e2e`, not on `main`) completed only three cells,
  at 5·10²–2.8·10³× rho (10 and 12 bits); the rest exhausted their budgets
  and are unknown, not worse. No complete cell is cheaper than rho or than the
  plain `prime-abscissa:size=32` + `mitm` baseline (14.8× down to 5.2× rho).
  The validator replayed all 416 executions and checked \([d]G = Q\) on the
  306 verified rows; the record stays unvalidated for the gate (no
  `VALIDATION.json`, binary chain of custody not established).
- CPU oracle, L0 shared-host timing
  ([`research/hashed_ecdl_cpu_20261030/`](../../research/hashed_ecdl_cpu_20261030/README.md)):
  complete recovery on the public fixture \(p = 65521\), \(m = 3\),
  \(\tau = 80\) fell from a 45.99 ms median to 3.90 ms with persistent tables
  and batch-64 inversion, 120 executions per panel with byte-identical
  certificates across arms. Engineering on its own baseline; no rho arm.
- Desk audit, 2026-11-07
  ([`research/hashed_ecdl_tensor_audit_20261107/`](../../research/hashed_ecdl_tensor_audit_20261107/README.md)):
  the reduction is correct; the encoding has \(s = a\cdot m\cdot b\) edges with
  \(a \ge 5.3\cdot 10^3\) at \(b = 131\); the composed exponent sits above
  rho by at least 120 bits at \(b = 128\)–\(256\) even with \(a = 1\); the
  linear algebra alone exceeds rho for every \(m \le 6\). The LEAD-ICB
  obstruction does not apply, since \(s\) is linear in \(b/m\); the closure
  comes from admission and constants. The lead card
  ([`research/pdp_strategy_evaluation_20261031/cards/e4.md`](../../research/pdp_strategy_evaluation_20261031/cards/e4.md))
  closes it as an experimental lead and keeps the preprint as a watch item,
  to be revisited on an explicit-constants Theorem D.1 whose saving starts
  below about \(10^6\) edges.

**Status.** Experimental, diagnostic only: not Integrated, because no path
yields a priced end-to-end `S`; not Backlog, because the code paths exist,
are registered, and are tested. The hash tables that the Integrated paths use
for membership (`point_index`/`x_index`, the Frobenius orbit-key lookup, the
compact bucketed hash of `PairSumTable`, which has no false negatives) are
implementation of those paths, not this technique; §5a prices them as hash
probes.

## 2. Relation collection

### Semaev summation polynomials — Integrated

- Prime-field \(S_3\) and recursively constructed \(S_4\):
  [`ec_index_calculus.rs`](../../src/cryptanalysis/ec_index_calculus.rs).
- Higher and binary-field systems:
  [`semaev_higher.rs`](../../src/cryptanalysis/semaev_higher.rs),
  [`binary_semaev.rs`](../../src/cryptanalysis/binary_semaev.rs), and
  [`binary_semaev_s4.rs`](../../src/cryptanalysis/binary_semaev_s4.rs).
- Pair-and-root \(S_4\) decomposition over binary subspaces:
  [`semaev_decomp.rs`](../../src/cryptanalysis/semaev_decomp.rs).

### Point-decomposition algorithms — Integrated

The Koblitz pipeline exposes enumeration, meet-in-the-middle pair tables,
Gröbner, SAT, WDSat, and MQ-FES decomposition strategies. Returned witnesses are
never trusted directly: the points are added again and checked against the
target before the relation is accepted.

### Weil restriction — Integrated

The Koblitz Gröbner and SAT paths descend extension-field Semaev equations to
Boolean systems over the selected subspace. The symbolic Petit–Quisquater
construction is in
[`pq_descent_symbolic.rs`](../../src/cryptanalysis/pq_descent_symbolic.rs);
the bounded degree and cost measurements are in
[`ic_descent_degrees.rs`](../../src/cryptanalysis/ic_descent_degrees.rs).

### Gröbner F4/F5, XL, and hybrid guessing

- **Integrated:** native F4 and the Koblitz hybrid matrix-F4,
  matrix-F5, and inherited-F4 engines are registered through
  [`ic_framework/solvers.rs`](../../src/cryptanalysis/ic_framework/solvers.rs).
- **Component:** Boolean XL is implemented in
  [`pq_xl.rs`](../../src/cryptanalysis/pq_xl.rs) and registered as a bounded
  reference solver. It is deliberately capped where its matrix becomes
  impractical.
- **Integrated as hybrid solvers:** the matrix engines combine bounded
  Macaulay reduction, propagation, and splitting/guessing. Budget exhaustion is
  reported as inconclusive, never as “no decomposition.”

### Symmetrised polynomial systems — Integrated

[`symmetrized_semaev.rs`](../../src/cryptanalysis/symmetrized_semaev.rs)
implements elementary-symmetric coordinates. The Koblitz implementation in
[`koblitz_symmetrised.rs`](../../src/cryptanalysis/koblitz_symmetrised.rs)
constructs the torsion-closed factor base, solves the symmetrised systems, lifts
roots back to points, and verifies their group sum. `koblitz-symmetrised` plus
the `symmetrised` oracle is selectable through the IC framework and has an
end-to-end logarithm-recovery test.

### Large-prime variants — Experimental

Three distinct research paths exist:

- residual collisions analogous to the large-prime variation:
  [`residual_walk.rs`](../../src/cryptanalysis/residual_walk.rs);
- bounded single-large-prime Jacobian experiments:
  [`hyperelliptic_index_calculus.rs`](../../src/cryptanalysis/hyperelliptic_index_calculus.rs)
  and [`hyperelliptic_ic_large_prime.rs`](../../examples/hyperelliptic_ic_large_prime.rs);
- bounded Gaudry cubic runs with configurable large-prime count and merge level:
  [`gaudry_cubic.rs`](../../src/cryptanalysis/gaudry_cubic.rs).

The common [`large_prime_filter.rs`](../../src/cryptanalysis/large_prime_filter.rs)
layer accepts all three shapes. It preserves source coefficients for replay,
peels the large-prime hypergraph, closes graph cycles, and performs bounded
Markowitz elimination on arbitrary hyperedges. Collector adapters remain
explicit so different point encodings cannot be mixed silently.

Since 2026-10-12 the Koblitz pair-table pipeline has an integrated one- and
two-large-prime collector
([`ic_framework/large_prime.rs`](../../src/cryptanalysis/ic_framework/large_prime.rs),
`mitm-frobenius:m=3,large_primes={1,2},lp_domain_factor=c`, or
`ic bench --large-primes --lp-domain-factor`). The domain `D` is the next
`c·|F|` signed points of the compact-orbit raw-x scan after the base; the
three-summand loop keeps its classic table and adds a second Frobenius-folded
table over `F ∪ D` without the `F × F` pairs, so a probe `T = R − P_i` yields
`R = P_i + L` (orbit-key lookup), `R = P_i + P_j + L` or `R = P_i + L_1 + L_2`.
Partials are merged online by
[`koblitz_large_prime::WideLargePrimeEliminator`](../../src/cryptanalysis/koblitz_large_prime.rs)
(exact `±λ^t` orbit coefficients modulo `r`, every partial and every completed
row replayed as a point equation) and each merged row is checked again in the
counted group before it enters the matrix through the `Relation::Row` seam of
the shared relation loop. Measured on `K_0` at n = 23, 31 and 41 with 5/10/20
compact-orbit columns, `|D|/|F| ∈ {3, 10, 30}`, 4–8 planted targets per cell,
all 260 targets verified, no partial rejected and no merged row failing
replay (`research/ic_techniques_20261012/double_large_prime/`): at n = 23 and
31 (`r` ≈ 2²¹ and 2^20.5, where the plain table already decomposes almost
every trial) every large-prime cell costs 2.2–750× the plain oracle end to
end and the `F ∪ D` table saturates at the subgroup's `r/2n` orbits; at n =
41 (`r` = 2³⁹), 10 columns, the single large prime cuts trials per complete
relation 4.1–10.3× and the double 8.7–138×, but against the plain oracle on a
base the size of `F ∪ D` (40 and 110 columns) the single is 1.07–1.98× worse
and the double 2.1× better at equal table size (one-target total 2.21·10⁶ GAE
against 4.59·10⁶; still 17.7× the counted rho). One rung with headroom, no
exponent fitted: a constant-factor device of the pair-table family,
classified engineering; the README there has the cells and the verdict
against `LEAD_LADDER_METHODOLOGY_20261010.md` §7 item 4.

## 3. Relation filtering

### Duplicate removal and singleton peeling — Integrated

[`koblitz_sparse_la.rs`](../../src/cryptanalysis/koblitz_sparse_la.rs)
deduplicates rows, removes singleton columns with their rows, and records
back-substitution data so every eliminated logarithm can be reconstructed.

### Cycle/clique selection and peeling — Integrated for the main sparse matrix

Surplus rows are removed with a clique rule designed to cascade through
weight-two columns. This is matrix filtering after complete relations have been
formed. It should not be confused with a general graph or hypergraph engine for
combining arbitrary multi-large-prime partial relations.

### Structured merging and Markowitz-style elimination — Integrated

The sparse logarithm solver merges light columns under a fill-in bound.
The generic research framework also provides a lightest-column,
Markowitz-style incremental solver in
[`ic_framework/linalg.rs`](../../src/cryptanalysis/ic_framework/linalg.rs).

### Large-prime graphs/hypergraphs — Integrated reusable layer

[`large_prime_filter.rs`](../../src/cryptanalysis/large_prime_filter.rs) accepts
single, double, and arbitrary multi-large-prime relations with coefficients
modulo the subgroup order. It deduplicates, recursively peels degree-one
vertices, closes cycles, and uses fill-bounded Markowitz pivots on the
hypergraph two-core. Every complete row carries its exact sparse combination of
source relation ids, and the replay helper reconstructs the row before the
collector independently verifies its curve-group witness.

## 4. Sparse linear algebra

### Parallel block Wiedemann and black-box Krylov — Integrated

[`koblitz_sparse_la.rs`](../../src/cryptanalysis/koblitz_sparse_la.rs)
implements block Wiedemann over the filtered relation core. It forms a block
Krylov sequence using only sparse matrix-times-block products, computes a
shifted minimal approximant basis, reconstructs eliminated variables, checks
every original relation, and leaves the final group certification to the
caller. Sparse products run in parallel over rows.

### Sequential Wiedemann — Component

[`pq_wiedemann.rs`](../../src/cryptanalysis/pq_wiedemann.rs) provides the
scalar black-box algorithm and sparse matrix-vector primitives. The main
Koblitz path uses the block implementation instead.

### Block Lanczos — Integrated

[`koblitz_sparse_la.rs`](../../src/cryptanalysis/koblitz_sparse_la.rs) provides
a selectable finite-field block-Lanczos/conjugate-direction backend. It applies
the symmetric congruence `A^T D A` only over the odd prime subgroup order,
retries deterministic random diagonals on breakdown, and accepts a result only
after checking every original equation `A x = b`. Tests cross-check it against
the dense reference and exercise selection through the full sparse solver.

### Distributed/GPU sparse matrix-vector multiplication — Integrated backend

The ECDLP CSR operator now selects serial, Rayon, deterministic row-sharded, or
external-worker products. The stable `SPMV1` contract and native host/CUDA
workers live in [`gpu/spmv`](../../gpu/spmv). External results are recomputed
and compared with the portable CPU product before use, so an accelerator can
only cause a fallback, never a false logarithm. A cluster launcher may split
the same independent row shards. No GPU or distributed speed claim is made
until matched hardware measurements and replay evidence exist.

## 5. Individual logarithm

### Randomized target decomposition — Integrated

The `ic solve` path samples
\(R=[a]G+[b]Q\) until one decomposition is found, substitutes certified
factor-base logarithms, recovers \(d=\log_G Q\), and accepts it only after
checking \([d]G=Q\). See the factor-base-log and descent section of
[the IC framework guide](README.md).

### Signed-Frobenius-orbit recovery — Integrated

For Frobenius-invariant bases, descent relations are rewritten into the signed
orbit columns used by the precomputed logarithm database. This is an orbit-aware
one-relation descent, not a claim that every target decomposition has an
independent Frobenius symmetry.

### Recursive point decomposition — Integrated scheduler, experimental adapters

[`recursive_descent.rs`](../../src/cryptanalysis/recursive_descent.rs) implements
the common budgeted scheduler: memoisation, depth/node/relation caps, randomized
attempts, cycle rejection, verified relations, and a replay transcript. The
repository also contains degree, coordinate, and algebraic adapters and
experiments, including
[`coordinate_descent.rs`](../../src/cryptanalysis/coordinate_descent.rs),
[`descent_algebraic.rs`](../../src/cryptanalysis/descent_algebraic.rs), and
[`diem_descent.rs`](../../src/cryptanalysis/diem_descent.rs). Curve-specific
adapters decide which decomposition oracle supplies each recursion level.

### Large-prime descent — Integrated scheduler, experimental collectors

Large primes are ordinary recursive children in the scheduler. Their relations
can be reduced by the common hypergraph filter, while the bounded Gaudry,
residual-walk, and Jacobian collectors provide concrete experimental sources.

### Frobenius-orbit recursive descent — Integrated proof boundary

The recursive scheduler exposes canonicalisation as an explicit proof boundary:
an adapter returns a representative and the multiplier transporting its
logarithm. Signed Frobenius adapters use `±lambda^k`; adapters without a proven
action must return the identity. Every lifted relation and final logarithm is
verified independently, so an invalid per-target symmetry is rejected rather
than assumed.

## Integration order for the remaining items

1. Add collector-specific converters for the common large-prime relation type,
   retaining each collector's independent curve-group verifier.
2. Add curve-specific recursive-descent adapters and frozen termination
   transcripts for the Koblitz, Diem, and Gaudry paths.
3. Validate block Lanczos against the larger frozen block-Wiedemann corpus.
4. Validate the CUDA worker and a cluster launcher on named hardware without
   changing the `SPMV1` transcript.
5. Attempt general trace-zero or higher-genus GHS integration only after the
   explicit smooth-model map is available and independently checked.
6. Calibrate `hashed_tensor_steps` (nanoseconds per charged primitive step on
   the calibration host, pinned in [`calibration.json`](calibration.json)) or
   charge the contraction in GAE, so the hashed arm gets a priced `S` and can
   leave `DiagnosticOnly`; until then it stays a stage diagnostic.

Each item is a separate research iteration. Before measuring, it requires a
versioned protocol with frozen inputs, a matched baseline, a derived boundary,
success and stop conditions, exclusive cost accounting, and verified
one-target recovery. Stage-only results remain labelled stage diagnostics.

## References already represented by the code

- Semaev, summation polynomials (2004).
- Faugère–Perret–Petit–Renault and Petit–Quisquater, Weil-descent systems
  and algebraic solving (2012).
- Faugère–Gaudry–Huot–Renault, symmetry in elliptic-curve index calculus
  (2014).
- Gaudry and Diem, extension-field and abelian-variety index calculus.
- Galbraith–Granger–Merz–Petit, Frobenius-invariant factor bases for
  subfield curves (2020).
- Wiedemann, Coppersmith, Kaltofen, and Montgomery, sparse black-box
  linear algebra.
- Akshima, Bacho, Huang, Kiltz, Nishaburi, Nooripoor, Tittelbach, and
  Wiedijk, *A Subexponential Time Algorithm for Elliptic Curve Discrete
  Logarithm*, v0.2 (2026), hashed factor bases; conditional on the exact
  tensor-contraction theorem of OpenAI, *The Exponential Time Hypothesis is
  False*, Appendix D (2026).
