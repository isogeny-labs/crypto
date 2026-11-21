# ECDLP index-calculus research technique matrix

This is the canonical inventory for the elliptic-curve index-calculus techniques
implemented, exercised, or queued in this repository. It maps a research name to
a concrete native implementation and states the boundary of that implementation.

Revised 2026-11-12 from a module-by-module audit of `main` (six stage
inventories, every claim read in code; see "Corrections in this revision").
The status words are defined below; a measured number is quoted from the
record it names and carries `[pcc:unverified]` unless that record shows a pcc
id or a `VALIDATION.json`.

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
| Factor base | Subspace bases; Frobenius-invariant bases (`koblitz-orbit`, `projected`); signed-orbit quotienting; trace-zero policy; Frobenius-orbit unions (`compact-orbit-scan`, `generator_orbit`); `ic search` with frozen recipes; `binary-subspace`; `glv-orbit`; PKM towers `prime-kummer`, `prime-isogeny`, `prime-interval`; `archive` import | Hashed affine-threshold bases; `norm=N` and multiplier closures; greedy and sampled selection, pruning; `strict4e`/`gated-rank`; `koblitz-closure`; cyclotomic; δ method; GG special subspace; λ base; known-log bases and the pair claw; CM scalar orbits; line bases over `F_{p^k}`; prime-orbit IC; N83 frozen bases; `explicit-x`; P-256 Dickson and bit-box dumps; small-x yield instrument; coordinate frames | General trace-zero-variety bases beyond the Koblitz subspace policy; a calibrated price for hashed-base contraction steps |
| Relation collection | Semaev `S₃`/`S₄`; `subtract`; prime and Koblitz pair tables (`mitm`, `mitm-prime-batch`, `mitm-prime-blocked`, `mitm-frobenius`); `rootfind-*`; `chained-x`; `descent-algebraic`; the F4 family (`f4-f2`, `matrix-f4`, `matrix-f5`, `inherited-f4`); symmetrised systems; target sources with the repeat guard; CADO-style filtering | `ksum`/`wsum`/`wwalk`; known-log and CM-folded MITM2; `pkm-semaev` and the tower F4 engines; line and resultant oracles; residual and cofactor-fibre collectors; one- and two-large-prime collectors; Buchberger, crossbred, SAT, FES, exhaustive and M4GB arms; WDSat and SMT; chained-`S₃` SAT; `pdp-split`; six-sum extraction; wide solvers; `n = 83` gates; degree levers and MutantXL closure; orbit pair table | Collector adapters to the common multi-large-prime layer; Joux–Vitse F4 trace reuse (G1); BooleanSolve refutation (G3); an equivariant F4 engine (G2); GPU root finding and FES on hardware; a priced solver unit for the algebraic oracles |
| Filtering | Deduplication; singleton peeling; clique removal; structured merging; Markowitz pivoting; CADO purge and merge; large-prime graph and hypergraph filtering | Collector-specific partial-relation combination; `audit_rank` receipts | None at the reusable layer |
| Linear algebra | Ten-name relation-matrix registry (dense, echelon, structured Gauss, deferred sparse Wiedemann and Lanczos); parallel block Wiedemann; block Lanczos; wide moduli to 255 bits; serial/Rayon/sharded/worker SpMV | Wide `k = 4` pipeline (`ic_wide`); Petit–Quisquater sparse elimination; sparse FGLM stage; CUDA SpMV worker | The ECC2K-130 `10⁹`-column cost (an extrapolation; the cited benchmark pack does not exist); GPU and distributed performance evidence |
| Individual logarithm | Randomized target decomposition (`ic solve`); signed-Frobenius recovery; the ledger-§23 one-target pipeline; the factor-base log database and log store | Shared full-rank log and point-only descent; batched collection with rank checks; recursive scheduler with the prime pair-table adapter and transcript replay (Experimental) | Koblitz, Diem and Gaudry adapters for the recursive scheduler; higher-genus GHS target descent |
| Higher genus and transfers | None (nothing here is an ECDLP production path) | Hyperelliptic IC (odd and binary); Gaudry–Diem `k = 3, 4, 5`; GHS smooth covers and IC; Joux–Vitse weak-class cover and sieve; plane-quartic Diem IC; cover, isogeny and Jacobian-image transfers; three-leads and APS instruments; binary large-prime references | A crossover on any non-weak class |
| Orchestration and accounting | `run_pipeline`, `ic bench`, the planner, the resumable workflow, pricing arms, the §5a band, the `S` unit and ledger, the validation gate, bounds and frontier | Degree-of-regularity forecast; relation-yield pruning | None |

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
end-to-end \(m=1\) trace descent; the smooth \(m \ge 2\) cover and the
index calculus on its Jacobian now exist in `ghs_smooth.rs` and `ghs_ic.rs`
(§6), which corrects the earlier statement here that the smooth model was
missing.

The repository does **not** contain a general trace-zero-variety ECDLP
pipeline: the trace-zero policy above is a Koblitz subspace filter, and the
GHS Jacobian path runs at toy sizes only.

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

### Automorphism-orbit bases on prime curves (`glv-orbit`) — Integrated

[`glv_invariant_base.rs`](../../src/cryptanalysis/glv_invariant_base.rs)
and the plug-in `glv-orbit:size=N[,group=auto|negation|j0|j1728][,no_fold=1]`:
take the `size` smallest abscissae, close the point set under `Aut(E)`
(`j = 0`: `x ↦ ζx`, order 6 with negation; `j = 1728`: `(x, y) ↦ (−x, iy)`,
order 4), and fold each orbit onto one column with coefficient `λ^k`, since
`log φ(P) = λ·log P` on `⟨G⟩`. `fold_by_endomorphisms` refuses a
non-invariant seed set, checks that two paths to a point agree on the
eigenvalue product, and `verify_endomorphism` checks closure, additivity
and `[λ]` on the subgroup; `no_fold=1` is the control. Type-C CM
endomorphisms of degree 2 or 3 (`D = −7, −8, −11`, Vélu maps) give no
invariant base inside a prime-order subgroup (0 images kept). The fold
divides the columns by exactly 3 (`j = 0`) and 2 (`j = 1728`) with the same
rank fraction on both arms at every size to `2^32`; "nothing about speed"
(encyclopedia A3, RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §6.1, §6.8). The
older `j = 0` ζ-orbit and Eisenstein-smooth pipeline
([`ec_index_calculus_j0.rs`](../../src/cryptanalysis/ec_index_calculus_j0.rs))
is its ancestor, measured with exponent 0.825 against 0.691 for the generic
base, "empirically falsified" (encyclopedia A8). Experimental.

### Multiplier closures: `cm-multiplier` and `binary-multiplier` — Experimental

`cm-multiplier:size=K[,norm=N][,endo=I][,mult=M]`
([`plugins.rs`](../../src/cryptanalysis/ic_framework/plugins.rs)) is the
prime-field twin of the Koblitz `Z[τ]` closure: for every
`α = a + bφ` of norm `a² + tab + db² ≤ N` it adds `[a]P + [b]φ(P)` on the
seed's column with weight `a + bλ` (and `[m]P` with weight `m` for
`m ≤ M`), `φ` being the free `j = 0`/`1728` automorphism or a Vélu
degree-2 or degree-3 endomorphism (`--family d7|d8|d11`). A precompute
trade, not a yield lever: the multiples are known linear combinations of
the seed logarithm, so the relation floor is unchanged. No dedicated
measured record for the prime plug-in; the Koblitz analogue reads 4.5× rho
at 2 columns and norm 64 (encyclopedia D3). `binary-multiplier` is the
structure-free `[m]P` form for any binary curve (see the Koblitz list
below).

### CM scalar-orbit bases (`cm-scalar-orbit`, `ic.cm_folded_*`) — Experimental

[`ic_framework/cm_orbit.rs`](../../src/cryptanalysis/ic_framework/cm_orbit.rs):
each of `columns` seeds generates `length` points under the scalar action
`λ` (`λ² − λ + q ≡ 0 mod r`, `q = 5` for `D = −19`, `11` for `D = −43`),
`mode=velu` evaluating the degree-`q` self-isogeny from rational
`q`-torsion on the twist instead (tested identical). Cofactor-one
prime-order curves only. The ecbench methods and their measured record are
in §2 (CM-folded MITM2): floor/crater 0.98 [0.87, 1.11], no systematic
difference.

### PKM towers: `prime-kummer` and `prime-isogeny` — Integrated (plug-ins), stage diagnostic at NIST size

[`ic_framework/prime_tower.rs`](../../src/cryptanalysis/ic_framework/prime_tower.rs)
builds the two Petit–Kosters–Messeng towers (PKC 2016 §3) that the
`pkm-semaev` oracle reads: `prime-kummer:t=T[,g=G]`, the base of every
point with `x ∈ g·μ_{2^t}` (the zeros of `t` chained squarings; needs
`2^t | p − 1`; `2^t` Legendre symbols and square roots charged), and
`prime-isogeny:t=T[,seed=S][,aux=ordinary|supersingular|ss-j1728|ss-j0|…]`,
the base `x(R' + ⟨Q⟩)` for `Q` of order `2^t` on an auxiliary curve, the
zeros of `t` chained Vélu 2-isogeny maps with no condition on `p`
(ordinary auxiliary curves found by a public-seeded search charged to the
factor-base phase; supersingular ones have `#E' = p + 1` and a known
2-Sylow). With any oracle other than `pkm-semaev` both behave as plain
abscissa sets. These are the two flagship families of the encyclopedia
(§0.1, §0.2): the Kummer tower was solved at `t = 11` and the P-224 prime
up to `m = 2, N = 20` with "no `S`, no relation phase" at NIST size, and
band-priced it costs 30.3 → 141.3× (`w = 0`) and 31.0 → 238.7× (`w = 1`)
the cheapest generic arm from 20 to 28 bits; the isogeny tower exists at
the P-192 prime as a construction only (search about `2^74` GAE against
rho's `2^95.8`), verdict "accounting". The interval and frame bases the
same oracle accepts, `prime-interval:t=T[,lo=L]` (Integrated plug-in) and
the digit-set, Edwards-`y`, `T0`-invariant and square-bits (2-descent)
builders in [`pkm_semaev.rs`](../../src/cryptanalysis/ic_framework/pkm_semaev.rs)
(Experimental, driven by `curve25519_family`), are Möbius frames of the
same system: frame choice, not model, moves the F4 cost 3.3–4.7× at
`k = 20` ([`research/curve25519_frame_search_20261103/`](../../research/curve25519_frame_search_20261103/README.md),
whose `VALIDATION.json` cannot pass the gate for want of a same-unit rho
row); square bits are 1.28–1.71× costlier than frame-selected bits
(closed); the greedy digit set covers 2.1–2.2× the best of 2000 random
frames (lead open, not engine-confirmed). [`pkm_criterion.rs`](../../src/cryptanalysis/pkm_criterion.rs)
is an audit score, not a base (P-224 and P-192 score 0.647, P-521 0.001).

### Line bases over `F_{p^k}` with eigenline folds — Experimental

Three extension-field families, all library plus example harness
(`examples/glv_invariant_bench.rs`, `glv_invariant_experiments.rs`), none
selectable by `ic bench` despite its listing:

- `gls-line` ([`gls_fp2.rs`](../../src/cryptanalysis/gls_fp2.rs)): on the
  GLS twist `E'/F_{p²}` the base `{P : x(P) ∈ u·s·F_p}` is stable under
  `ψ` with `ψ² = −1`, folded by `⟨−1, ψ⟩`, four signed points per column;
  column ratio 2, same rank fraction (E2); on the `k = 4` V4 line the fold
  cuts the cost 1.35–2.67× while every row stays `1.1·10⁴`–`2.7·10⁵`× rho.
- Subfield eigenline ([`subfield_fp3.rs`](../../src/cryptanalysis/subfield_fp3.rs),
  E5): `E/F_p` over `F_{p³}` with Frobenius acting as a cube root of unity
  on `⟨G⟩`, fold of order 6; column ratio 3 but relations fall only
  1.2–1.6×, and on a subfield curve rho costs `Θ(p)` while a line base
  needs `Θ(p)` relations, "at best a constant" (E16).
- FGHR Y-line ([`fghr_line.rs`](../../src/cryptanalysis/fghr_line.rs),
  E13): `Y = (x − x₀ − c)/(x − x₀ + c)` with `Y(P + T) = −Y(P)` for a
  rational 2-torsion `T`, fold `⟨−1, π, τ_T⟩` of order 12, `S₄`
  symmetrised in `D₃` invariants solved by a degree-16 conic resultant;
  `S/27` from the oracle, `S/2.52` from the base, best arm 267× rho.

### Known-log bases and the pair claw — Experimental

`known-log:size=B[,scalars=progression|random][,seed=…]`
([`plugins.rs`](../../src/cryptanalysis/ic_framework/plugins.rs)) is the
base `{±[s_i]G}` whose logarithms are known, so one verified relation
`[a]G + [b]Q = Σ ±[s_j]G` yields `k` and the matrix is skipped
(`KnownLogSolver`); "a generic precomputation table (Bernstein–Lange
shape) that cannot beat `√r`". The ecbench methods `ic.known_log_mitm2_v1`
and `ic.known_log_ksum_v1` and their records are in §2. `claw.pair_table`
([`ecbench/claw.rs`](../../src/cryptanalysis/ecbench/claw.rs), Koblitz
only) is the four-point signed-Frobenius pair claw of
aburan28/cryptanalysis#175: a known-log orbit base, a target-independent
quotient table of `M = c·√(r/n)` classes and unique query pairs, mean
`S ≈ (c + 1/c)/√n`, "a randomised baby-step giant-step on signed Frobenius
classes": 2.20× [1.90, 2.55] strong rho at the cost-minimising shape and
32.6× at the `n = 83` shape, 960 runs verified
([`research/ecbench_pair_claw_20261003/`](../../research/ecbench_pair_claw_20261003/README.md)).

### Archive import with certified symmetry (`archive`) and `explicit-x` — Integrated / Component

[`ic_framework/archive.rs`](../../src/cryptanalysis/ic_framework/archive.rs)
(`ic bench --archive-store … --archive-manifest … --archive-symmetry
none|negation|cubic|quartic|frobenius`) imports a content-addressed
factory archive without changing its point set (manifest and shard
hashes, recipe replay), checks exact curve, subgroup, cofactor, order and
generator identity, replays every canonicalisation `C = [μ]P` and stores
`μ^{-1} mod r` as the column coefficient; `frobenius` requires a certified
closed Koblitz archive. Up to 8192 signed points, `p < 2^62`, degree
`≤ 62`; the archive's own generation cost is reported unpriced, and the
`useful_rank` audit runs automatically. Acceptance tests establish import
and accounting correctness; "no candidate has been selected by the
comparison" ([`docs/ic/ARCHIVE_FACTOR_BASE_COMPARISON.md`](ARCHIVE_FACTOR_BASE_COMPARISON.md)).
`explicit-x:xs=…` ([`ic_framework/explicit_base.rs`](../../src/cryptanalysis/ic_framework/explicit_base.rs),
ecbench only) is the exact negation-closed base from a supplied abscissa
list, the transport plug-in for externally selected bases.

### Prime-orbit index calculus (`ic prime-orbit`, `ic.prime_orbit_relations_v1`) — Experimental

[`prime_orbit_index_calculus.rs`](../../src/cryptanalysis/prime_orbit_index_calculus.rs):
a self-contained automorphism-orbit pipeline on generic, `j = 0` and
`j = 1728` prime curves. The base is `m` full `Aut`-orbits (`w·m` points,
`m` unknowns); the decomposition sweeps `α^{-1}(R) − P_o` over every
automorphism and representative with one batched inversion per block;
relations have at most two unknowns, so the system is a gain graph solved
exactly by a spanning tree with every column certified `[x_o]G = P_o`;
optional large primes; affine or 32-lane Jacobian collectors; a batched
rho baseline beside it. Scaled-down curves with certified order up to 40
bits; precomputation is about `c·r/w`, linear in `r`. The ecbench
relation-collector benchmark reads 5.42× negation rho on a 14-bit toy
curve, 4.20× cubic rho on `j = 0`, 5.79× quartic rho on `j = 1728`,
"rho-led" (e2e frontier report).

### Special prime-field bases and coordinate frames — Component

- P-256 Dickson torus ([`p256_dickson_factor_base.rs`](../../src/cryptanalysis/p256_dickson_factor_base.rs)):
  `x = w + w^{-1}` over the depth-`D` trace coset of a norm-one element
  of order `2^96` in `F_p[i]`, dumped as an exact factor-base record with
  Poisson yield metrics; refuted at 16–144 bits above rho over 24 rounds
  (encyclopedia A5b). P-256 affine bit-box
  ([`p256_bitbox_factor_base.rs`](../../src/cryptanalysis/p256_bitbox_factor_base.rs)):
  `S₃` solving degree 5 against Dickson's 4, refuted (A6).
- Small-x exact-yield instrument ([`prime_small_x.rs`](../../src/cryptanalysis/prime_small_x.rs),
  [`secg_small_x.rs`](../../src/cryptanalysis/secg_small_x.rs), `ic prime`):
  the exact probability that a random subgroup element is a signed two-sum
  of the smallest-abscissa base on every SECG curve (yield `2D/(n − 1)`,
  cross-checked by Monte Carlo and planted decompositions): two-point
  small-x loses to rho by at least `2^40.9` in trials on every curve, and
  P-192's yield is `2^{−175}`–`2^{−165}`
  ([`research/secg_small_x_20261014/`](../../research/secg_small_x_20261014/README.md)).
- Coordinate frames ([`coordinate_search.rs`](../../src/cryptanalysis/coordinate_search.rs),
  [`coordinate_quotients.rs`](../../src/cryptanalysis/coordinate_quotients.rs)):
  degree-2 coordinates are Möbius frames on the x-line; the modules detect
  pairwise symmetries, build the linearising frame and interpolate the
  summation polynomial in the group's invariants (`S₄` degrees
  `[4, 4, 4, 4] → [2, 2, 2, 2, 1]`), toy fields only, "nothing here solves a
  decomposition" (encyclopedia B3, constant factor).

### Frobenius-orbit union bases (`compact-orbit-scan`) — Integrated

The planner's default Koblitz base
([`ic_framework/plugins.rs`](../../src/cryptanalysis/ic_framework/plugins.rs),
`compact-orbit-scan:columns=K,raw_x_cap=X`): scan raw abscissae
`x = 1, 2, …`, lift each by a half-trace root, project into the order-`r`
subgroup by the cofactor, key the signed Frobenius orbit by the minimum over
`x^{2^i}`, and keep the first `K` distinct orbits, each one column with
weights `±λ^t` (`2n` points per column). Every orbit is checked closed, on
the curve, of order `r` and eigenvalue-consistent before use. Two options:
`norm=N` (lead A1, the `Z[τ]`-multiplier closure) adds the orbit of
`[a]P + [b]τ(P)` with weight `a + bλ` on the same column for every
`α = a + bτ` of norm `a² − ab + 2b² ≤ N`, so `K` seeds act as many table
points while the matrix keeps `K` unknowns; `generator_orbit=1` makes column
0 the generator's own orbit with known logarithm, so one relation fewer is
needed. Odd `n < 64` only; native container work is reported `*_uncharged`.

Measured: the band-priced frontier campaign puts the compact orbit at
3.8–15.0× rho at `w = 0` and 11.8–30.3× at `w = 1` over `n = 13…47`
(encyclopedia D2); `norm=N` at `n = 41`, `m = 3` reads 4.5–6.5× the matched
signed-Frobenius rho and cuts the one-target total 9.2× against the plain
scan, "a constant on a toy rung"
([`research/ic_leads_20261010/RESULTS_ROUND2.md`](../../research/ic_leads_20261010/RESULTS_ROUND2.md)
§A1). The generator-orbit base
([`research/ecbench_generator_orbit_20261022/`](../../research/ecbench_generator_orbit_20261022/README.md),
`VALIDATION.json` present) nets 1.26–3.20× over the baseline on seven curves
and at `n = 29` sits below the matched rho-frob across the band (S 0.546 /
0.611 / 1.196 against 1.569); its own README reads that crossing as a
Frobenius-folded BSGS at one column resting on rho's fixed set-up at
`log₂ r = 15.4`, and the gap to rho is 1.15× at `n = 31`, 1.93× at 23 and
3.89× at 37. Under the owner's 2026-10-24 track rule
([`src/bin/ic/sweep/space.rs`](../../src/bin/ic/sweep/space.rs)) every
pair-table base, this one included, is Table track: reported, never ranked
against the Algebraic track.

### Factor-base search, greedy coverage selection and frozen recipes — Integrated (search), Experimental (greedy, pruning)

`ic search` ([`koblitz_factor_base_search.rs`](../../src/cryptanalysis/koblitz_factor_base_search.rs))
scores a candidate base by expected trials `(U + 1 + extra)/p_m` from an
exact witness census over a pair-sum table, across the families `factor`,
`divisor`, `union`, `subgroup` (random abscissae kept as `[h]P`) and
`greedy`. The greedy selector
([`koblitz_greedy_base.rs`](../../src/cryptanalysis/koblitz_greedy_base.rs))
adds at each step the signed orbit covering the most new targets as two-sums,
exactly in the scalar model up to `r = 2²²`; above that
[`koblitz_greedy_sampled.rs`](../../src/cryptanalysis/koblitz_greedy_sampled.rs)
draws candidate orbits per step and tracks covered sums in a hashed set
(cap 8192 points). `greedy_prune` removes orbits while the score falls, and a
selection is saved **frozen** as `{"kind":"explicit_orbits","abscissae":[…]}`
so a recipe rebuilds in milliseconds; `TwoTorsionSaturated` and
`CofactorProjected` wrap any family. The gain is a constant that shrinks
with `r` (expected trials at the best column count fall 1.08–1.18× at
`r ≤ 2^17`; selector advantage over random is 1.09 at `r = 2^16`, 1.012 at
`2^27.8`, 1.000 at `2^39`;
[`research/greedy_factor_base_yield_20261014/`](../../research/greedy_factor_base_yield_20261014/README.md)),
so past `r ≈ 2^28` the guide says use `subgroup`. The solve-cost ranking
(encyclopedia D10) differs from the trials ranking by up to 22×, which is
why the sweeper scores both.

### Other binary and Koblitz plug-in bases — Experimental unless noted

- `koblitz-orbit:divisor=i;j` is the plug-in name of the Frobenius-invariant
  subspace base above; `projected=1` (also on `koblitz-trace-zero`) folds
  columns by the signed orbits of the cofactor projections `[h]P`, giving
  the matrix's real column count, and `no_fold=1` is the control. Invariant
  subspaces are empty at `n = 131` and `163` (dimensions 0, 1, `n − 1`, `n`
  only; encyclopedia D1), which bounds this family's reach. Integrated.
- `binary-subspace:dimension=l[,basis=…][,select=prefix|strict4e|gated-rank]`
  is the coordinate-window base on any binary curve (Integrated). The
  `strict4e` selector ([`ic_framework/subspace_select.rs`](../../src/cryptanalysis/ic_framework/subspace_select.rs))
  sieves a subspace whose abscissae all lie in `4E` (Koblitz `a = 0`, odd
  `n`, `h = 4` only), `gated-rank` keeps the best of several draws by a
  rank-aware key; both are ports of the ml-cryptanalysis cost-model
  constructors. The one record
  ([`research/fb_strict4e_20261027/`](../../research/fb_strict4e_20261027/README.md))
  is unresolved: 2.11 bits fewer counted operations at `w = 0`, the order
  flips inside the §5a band because the `2^n` sieve is `*_uncharged`, and
  rho is cheaper than both arms in every cell; its `VALIDATION.json` is
  refused by the gate (non-independent validator).
- `koblitz-closure:dimension=d` (lead L5): the Frobenius closure of the
  prefix subspace, each signed orbit one column, with the basis recorded so
  algebraic oracles solve per τ-slot tuple. Measured only negative: 1.79×
  and 2.14× hits per system against predicted 2 and 3, 50–100× the pair
  table (encyclopedia D6). No unit test of its own.
- `binary-multiplier:size=K,mult=M` (and the prime `cm-multiplier`): each
  seed contributes `[m]P` for `m ≤ M` on its own column with weight `m`, so
  `K` seeds act as `K·M` table points; as `K → 1` it is a BSGS table. At
  degree 28 the best multiplier base is 6.1× cheaper than the best subspace
  base and 3.9× cheaper than plain seeds, still 6–12× the framework rho
  ([`research/ic_improve_20261023/BINARY_README.md`](../../research/ic_improve_20261023/BINARY_README.md));
  the sweeper records that such wins "do not transfer".
- `cyclotomic:w` (lead A2) and its control `onb-weight:w`
  ([`cyclotomic_factor_base.rs`](../../src/cryptanalysis/cyclotomic_factor_base.rs)):
  for `p = 2n + 1` with `ord_p(2) = n`, the Frobenius-stable set
  `{Σ_{k∈S} γ^k : |S| = w}` of a primitive `p`-th root `γ`, which has a
  polynomial membership description. Falsified as a yield lever: 15 tests
  at `n = 23` lie inside the null of 1999 random Frobenius-stable bases
  (smallest `p` 0.363), build cost +2.2 % and +12.2 % in two of four pairs
  ([`research/cyclotomic_falsifier_20261015/`](../../research/cyclotomic_falsifier_20261015/README.md)).
  Single-word only, so `n = 83` and `131` cannot be built; ecbench only,
  not `ic bench`. The 2026-10-12 integration note's "not yet established"
  is superseded.
- `explicit-x:xs=…` ([`ic_framework/explicit_base.rs`](../../src/cryptanalysis/ic_framework/explicit_base.rs)):
  an exact abscissa list, both signs, used by ecbench claims for imported
  point-defined bases. Component.
- Binary small-x / SECG coordinate-window base
  ([`binary_small_x.rs`](../../src/cryptanalysis/binary_small_x.rs),
  `ic prime --curve sect163r2`): exact two-sum yield `2B²/(n − 1)` on the
  SECG binary curves; refuted, two-point small-x sums lose to rho on every
  curve by at least `2^40.9` in trials
  ([`research/secg_small_x_20261014/`](../../research/secg_small_x_20261014/README.md)).
- Shantz–Teske δ method ([`delta_method.rs`](../../src/cryptanalysis/delta_method.rs),
  `sage/factor_base_dor.sage --delta-sweep`): `m·l = n − δ`, trading
  `2^{−δ}` yield for an overdetermined, cheaper Weil-descent system. At
  `n = 31`, `m = 2` the measured fractions track the prediction (0.219 vs
  0.223 at `δ = +1`) and the net cost per relation is 513 / 257 / 213 ms at
  `δ = +3 / +1 / −1`; "a constant-factor tuning knob at small `n`, no
  exponent change". Not a plug-in.
- Galbraith–Gebregiyorgis special subspace with symmetric `S₄`
  ([`koblitz_sym_s4.rs`](../../src/cryptanalysis/koblitz_sym_s4.rs)): for a
  geometric `V = θ⟨1, g, …, g^{l−1}⟩` the symmetric functions of three
  elements lie in small `W_k`, so the unknowns are `6l − 3` bits with no
  intermediate point. "Buys variables, not degree" (`D* = 4` at `n = 9`,
  `≥ 5` at `n = 15`); exact `e₃` enumeration costs 553–34,901× the table
  oracle. Component.
- λ-coordinate base `{P : λ(P) ∈ W}` (lead L1,
  [`koblitz_ic_leads.rs`](../../src/cryptanalysis/koblitz_ic_leads.rs)): an
  identity-checked halved affine base; "L1 closed in full", nothing moves an
  IC cost. Component. Quasi-subfield polynomials
  ([`quasi_subfield.rs`](../../src/cryptanalysis/quasi_subfield.rs),
  Huang–Kosters–Petit–Yeo–Yun 2020) are a feasibility census: `n = 131` is
  barren. Component.
- The N83 programme bases (frozen cofactor-projected signed orbits at
  `n = 83`, `K = 600…5,500`, built by `examples/koblitz_orbit_dlp_fast.rs`
  with its `root4…root22`, Bloom and `wide3` root-index layouts, and the
  fixed-`ℓ = 27` shared bases of
  [`research/n83_fixed_factor_base_20261028/`](../../research/n83_fixed_factor_base_20261028/))
  live outside the single-word plug-ins. Encyclopedia D8 has the record:
  K600 fresh-process IC/rho 218 [172, 286], prepared-online 1.38 [1.04,
  1.88], final tournament 28.69 ms vs 28.81 ms `BOUNDED_UNKNOWN`; wall-clock
  on shared hosts, a 53-bit subgroup, "not evidence about ECC2K-130".
- Persistent fixed parameters (`ic fixed`, `ic ecc2k130`,
  [`docs/ic/FIXED_PARAMETERS.md`](FIXED_PARAMETERS.md); 84 frozen
  `ic workflow --params` files under [`docs/ic/params/`](params/)): the
  curve, generator, base recipe (Hamming-weight 2 at `n = 131`: 3,668
  signed points, 14 orbit columns), window, seed, solver and targets as one
  parameter document with a resumable SQLite pair table. A persistence
  harness, "not a promise of a full-size hit".

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

### Prime-field decomposition oracles: subtraction and pair tables — Integrated

The `ic.pipeline` oracles on prime curves
([`ic_framework/plugins.rs`](../../src/cryptanalysis/ic_framework/plugins.rs)):

- `subtract` tests `R − P` against the base for every signed base point:
  `|F|` additions per target, `m = 2`, the baseline every table is measured
  against (S = 310…2,775 against the pair table's 10…44 on the 18–24-bit
  panel).
- `mitm` (`m = 2, 3`, `negation_folded=1`) keys every pair sum by abscissa so
  `S` and `−S` share an entry, about `|F|²/4` additions to build and one
  probe per target (or per `R − P` at `m = 3`); the generic `PairTable` of
  [`ic_boundary.rs`](../../src/cryptanalysis/ic_boundary.rs) serves prime,
  binary and Koblitz groups. `mitm-prime-batch` builds the same table with
  one Montgomery batched inversion per abscissa row; `mitm-prime-blocked`
  inserts the same entries into cache-sized one-word pieces
  (`PackedPairTable`, regions of `2^14`, load ≤ 3/4) with an optional
  one-byte membership filter (`filter=1`) and a `layout=lines` cache-line
  bucket index. A hit re-adds the stored pair, so a key hit that is not a
  decomposition is rejected. Single-word primes only (`p < 2^62`).

Measured: set-up wall of the blocked builder is 0.13–0.25 of the scalar
builder at `N = 1,024…16,384` on `icv1-fp32-t7319` with peak memory
9.15 → 4.65 GiB (engineering); the packed filter is **validated with
caveats** at 0.9601 [0.9569, 0.9633] wall per charged GAE at 32 bits
([`research/ic_packed_filter_20261029/`](../../research/ic_packed_filter_20261029/));
tuning the base size at 20 bits moves `S` 12.85 → 10.51 with a
`VALIDATION.json` filed
([`research/prime_base_oracle_tuning_20261029/`](../../research/prime_base_oracle_tuning_20261029/)).
Rho stays ahead at every size: the pair-table family law is
`S = 0.75·r^{1/6}` (ledger §11.3). The orbit-folded pair table
[`orbit_pair_table.rs`](../../src/cryptanalysis/orbit_pair_table.rs) (one
entry per automorphism orbit of pairs, keyed by an exact orbit invariant;
`j = 0`, `j = 1728`, subfield lines over `F_{p³}`) divides the pair sums by
2.95 / 2.97 / 1.98 at a probe cost of 1.00–1.11×, example-only, Experimental.
The Koblitz `PairSumTable` tiers (full 16 B per pair, compact bucketed hash
with no false negatives, folded by the signed Frobenius group) and
`FrobeniusPairTable` behind `mitm-frobenius*` are the Integrated Koblitz
counterparts ([`docs/ic/README.md`](README.md), "The pair table, folded by
the signed Frobenius group"); `native_signed_mitm.rs` is the exact 3+3
binary half-sum table of the `n = 37` native panels (Experimental,
`NO_GO`).

### Root-finding oracles (`rootfind-subtract`, `rootfind-mitm`, `rootfind-pairs`) — Integrated

The Semaev `S₃` root in the third variable is `x(R ∓ F_i)`, so an `m = 2`
sweep needs no summation polynomial and no square root:
`rootfind-subtract` ([`plugins.rs`](../../src/cryptanalysis/ic_framework/plugins.rs))
inverts every `x_R − x_i` together by Montgomery's trick (one field
inversion per target), shares the slope between `R − F_i` and `R + F_i`,
and looks each result up in the base (`m = 2`) or the folded pair table
(`m = 3`); `rootfind-mitm` and `rootfind-pairs`
([`ic_framework/rootfind.rs`](../../src/cryptanalysis/ic_framework/rootfind.rs))
are the binary and prime `m = 3` forms with `FastCurve::add_many` batching.
Charged exactly as `subtract` (operations ratio 1.000 [1.000, 1.000] over
96 pairs), about 2.5× less wall; per decomposition 3.5·10⁴–4.7·10⁴× cheaper
than a Gröbner solve on 16–24-bit primes and 383–1,535× on binary
matrix-F4 (RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md). The sweeper classes it
`Unclassified` (neither Algebraic nor Table). The CUDA kernel
([`gpu_rootfind.rs`](../../src/cryptanalysis/gpu_rootfind.rs), `gpu/rootfind/`)
has a fail-closed CPU reference and emulation (21,811 targets, 0 spurious)
but "has NOT been executed on a GPU"; the root-extraction certificate
modules [`prime_root_trace.rs`](../../src/cryptanalysis/prime_root_trace.rs),
`prime_root_bounded.rs`, `binary_root_trace.rs` serve the ML dataset
recorder, not a collector. Components.

### Prime-field Semaev specifics — Integrated (`S₃`/`S₄`), Component (diagnostics)

Beyond the one-line entry above:
[`ec_index_calculus.rs`](../../src/cryptanalysis/ec_index_calculus.rs)
finds `S₄` roots by `gcd(f, X^p − X)` and equal-degree splitting
(Cantor–Zassenhaus, `find_roots_fp_fast`; the stale comment at
`semaev_s4_in_x4` says otherwise) and runs the `S₄` three-decomposition
ladder (`a = −3`, 16-bit P-256 class: 1.0 trials per relation, 3.3 s per
relation dominated by the `B²/2` quartic sweep, no three-decomposition win).
[`semaev_prime_poly.rs`](../../src/cryptanalysis/semaev_prime_poly.rs)
builds dense `S₄`, `S₅` by evaluation and tensor interpolation (matches
PARI `polresultant`) and checks Newton-polytope fullness and irreducibility
over `F_p`: on a 40-bit curve all 125 and 6,561 coefficients are nonzero
and no planted non-global small root is recovered, "no breakthrough is
claimed". [`semaev_s3_cost.rs`](../../src/cryptanalysis/semaev_s3_cost.rs)
(`ic prime --s3-trials`) prices deciding a trial by an `S₃` solve: 2,600×
(sect113r1) to 313,000× (secp224r1) a table lookup, and collecting `B + 1`
relations by `S₃` is 52–280 bits slower than rho over 33 SECG curves.
[`semaev_higher.rs`](../../src/cryptanalysis/semaev_higher.rs) gives `S₅`
and `S₆` by resultants for the symmetrised and Diem modules.

### Higher-arity and walk tables: `ksum`, `wsum`, `wwalk`, known-log — Experimental

[`plugins.rs`](../../src/cryptanalysis/ic_framework/plugins.rs) and
[`ic_boundary.rs`](../../src/cryptanalysis/ic_boundary.rs): `ksum` stores
every signed `k`-sum of the base (`k = 3, 4, 5`, negation folded, 12 bits
per summand) so one probe decides a `k`-summand decomposition, generic
cost exponent falling from `p^{2/3}` (pairs) to `p^{5/9}` at `k = 5`,
"approaches BSGS, never below"; `free=1` hands the build's key collisions
to the loop as target-free rows. `wsum` is the coefficient-box table
(every `Σ e_c P_c` with `|e_c| ≤ coef` over `C` columns) and `wwalk` a
table of `x·√r` steps of a random-jump walk whose key collisions are
target-free rows (`tw=1|2` puts the target inside the walk). The known-log
methods `ic.known_log_mitm2_v1` and `ic.known_log_ksum_v1`
([`ecbench/known_log.rs`](../../src/cryptanalysis/ecbench/known_log.rs))
use a target-blind seeded base `±[s_i]H` whose logs are known, so a
witness gives `log_G Q` without linear algebra.

Measured: `ksum5` costs 0.700 / 0.446 / 0.324 / 0.217 of the tuned
pair-table incumbent at 20 / 24 / 28 / 32 bits at `w = 0` over 16 fresh
curves ([`research/prime_ksum_pooled_20261029/`](../../research/prime_ksum_pooled_20261029/README.md)),
a record that has been **gate-validated** since 2026-11-12
([`VALIDATION.json`](../../research/prime_ksum_pooled_20261029/VALIDATION.json):
count-identical replay of all 8 sessions on another host class and build,
and an independent recompute with `[d]G = Q` on 5,952/5,952 holdout
records). It is therefore a verified improvement over the tuned pair-table
incumbent on prime curves at these sizes, within the generic decomposition
track, and `ksum5` still costs 5.3–7.3× negation rho: the distance
remaining, bounded below by BSGS. Known-log MITM2 is 1.11 →
15.6× cheaper than the abscissa pipeline from 12 to 24 bits while rho
stays 0.43 of it at 24 bits; the precompute-regime control finds no
single-target cost below rho; `wwalk` with the target in the walk reads
1.395× [1.345, 1.446] rho at 28 bits on the Curve25519 miniatures.

### PKM tower Semaev oracle (`pkm-semaev`) — Experimental

[`ic_framework/pkm_semaev.rs`](../../src/cryptanalysis/ic_framework/pkm_semaev.rs):
the prime-field algebraic oracle. `S_{m+1}(x₁…x_m, x_R) = 0` with each
`x_i` on an algebraically described set written as a tower per summand
(`y_{i,j}² = y_{i,j+1}`), handed to F4 over the tower quotient ring
([`f4_fp_tower.rs`](../../src/cryptanalysis/f4_fp_tower.rs), `p < 2^31`;
[`f4_tower_generic.rs`](../../src/cryptanalysis/f4_tower_generic.rs) for
`Fp64`, P-224 and the Dickson torus at P-192/P-256) or to an exhaustive
grid reference (the cross-check and the cost F4 must beat). `mode=chain`
(`S₃` links with free unknowns) or `full`; membership forms Kummer
(`prime-kummer`), Vélu 2-isogeny (`prime-isogeny`), `bits`, `digits`,
square bits, Edwards bits, `T0`; `guess=G` hybrid guessing, `trace=1`
refutation-trace replay, `lifts=1|2` cofactor-fibre lifts. A refutation
ends the target, an unrefuted system is read off the grid and every
solution lifted and re-added, a budget-exceeded system is decided on the
grid, so verdicts stay exact. Nearly all work is native and priced only in
the §5a band. The multiplicative-subgroup form
[`ic_framework/subgroup_semaev.rs`](../../src/cryptanalysis/ic_framework/subgroup_semaev.rs)
(power-map bases in elementary symmetric polynomials) is driven by
`curve25519_family`; [`sig_fp_tower.rs`](../../src/cryptanalysis/sig_fp_tower.rs)
(signature-based F5/GVW) and [`hky_fp_tower.rs`](../../src/cryptanalysis/hky_fp_tower.rs)
(Huang–Kosters–Yeo fall degrees by definition) are Components.

Measured (encyclopedia §0.1–0.2, RESEARCH_PKM_TOWER_ORACLE.md): the F4
solving degree is not bounded in `N = m·t` (Kummer, `m = 3`: `D = 6, 6, 7,
7` at `N = 9…18`; `m = 4`: `D ≥ 8` at `N = 20`), growing 1.6 / 2.1 / 2.4
bits of multiply-adds per tower variable at `m = 2 / 3 / 4` against 0.50 /
0.67 / 0.75 for enumeration; end to end in ecbench the Kummer tower costs
34–141× rho from 20 to 28 bits and equals a plain base of equal size
(1.00); the matched-prime control shows `D` follows `t`, not `p`, "the line
stays closed"; `bits` membership at `m = 3` on the Curve25519 miniature is
30–200× worse than `m = 2`; the best Curve25519 PKM arm is 17.1× rho at
`w = 0` and `6·10⁴×` at `w = 0.1`.

### Line and resultant Weil-descent oracles on `E(F_{p^k})` — Experimental

For a base `{P : x(P) ∈ c·F_p}`, [`line_oracle.rs`](../../src/cryptanalysis/line_oracle.rs)
(`line-resultant`) Weil-descends `S₃(ct₁, ct₂, x_R)` to `k` equations over
`F_p`, takes the resultant in `t₂` (degree ≤ 8), finds `F_p` roots by
`gcd(t^p − t, ·)` and lifts: `O(log p)` field operations per target,
complete, agreeing with `subtract` on 3,600 of 3,600 targets.
[`line_s4_oracle.rs`](../../src/cryptanalysis/line_s4_oracle.rs) is the
three-summand symmetrised `S₄` form, [`fghr_line.rs`](../../src/cryptanalysis/fghr_line.rs)
and [`fghr_full.rs`](../../src/cryptanalysis/fghr_full.rs) the FGHR
2-torsion `D_m` symmetry on the line and on the full `E(F_{p³})`. Library
and example components only (`examples/glv_invariant_experiments.rs`):
`ic bench` cannot select them. Best arm 267× rho (E13), the `S₄` line 3,344–
101,441× with fitted exponent 0.53; on a subfield curve the large prime
lives in a group of order about `p²`, so a line base can give a constant,
never an exponent (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md).

### Target sources and the repeat guard — Integrated

`Targets` ([`ic_framework/stages.rs`](../../src/cryptanalysis/ic_framework/stages.rs))
has 18 sources, not two: `random` (`[a]G + [b]Q`, two scalar
multiplications per trial), `walk` (the default, an `r`-adding walk over
16 tracked jumps, one addition per target, restarts add a pooled offset),
`walk-prime-fast` and the `walk-prime-serial-4/8/16`, `-batch-4/8/16`
synchronized lanes (same requests, serial or Montgomery-batched
inversion), `stride`, `stride-renew` (a new stride per relation, avoiding
four-row cancelling dependencies; 0.40–0.75 of `walk` on Koblitz at
`log₂ r ≤ 21`), `stride-generator` (`Q + [k]G`, needs a known-log column),
and the `residual*` family below. The repeat guard matters: presenting one
element twice pins the logarithm by a generic collision, which measures
rho, so guarded sources skip a seen target at one counted hash insert;
unguarded random `m = 2` runs are pinned that way 24.4 % of the time and
shared-jump walks 82.1 % (ledger §10.2). The lane microloop prices an
inversion at 79.5 ns against 1.3–3.8 ns for jump, lookup and guard.

### Residual and cofactor-fibre collection — Experimental

[`ic_framework/residual.rs`](../../src/cryptanalysis/ic_framework/residual.rs)
collects states `(a, b, row)` with residual `L = [a]G + [b]Q − Σ P_j`;
equal residuals up to sign give a relation over the base alone, without
an oracle. `residual-fused` folds jump and base subtraction into one
addition from `16·|F|` precomputed sums (2.005 → 1.004 additions per step
at 1.1–1.3× the steps per relation), `-dp` adds distinguished points,
`-eq` is a negation-equivariant walk, `-eq-chain` restarts at the previous
start plus one jump (null: 0.994 [0.953, 1.037]). A collision on `r`
points is a birthday event, so the second-collision floor is 1.5× ideal
rho and the arm cannot win by design
([`research/ic_rho_floor_20261109/`](../../research/ic_rho_floor_20261109/README.md)).
[`ic_framework/fiber.rs`](../../src/cryptanalysis/ic_framework/fiber.rs)
makes oracles aware that every relation is over the cofactor projection
`[h]P`, so a target stands for the fibre `{T + K : K ∈ E[h]}`: `mitm-fiber`,
`mitm-frobenius-fiber`, `descent-algebraic-fiber` with `lifts`, `closed`
(a table holding `P_i + P_j + K` for every `K`) or `combined` (one Weil
descent with the fibre polynomial adjoined). Closed tables are 0.50
[0.21, 2.36] of the blind oracle at `h ≤ 25` and 4.7× at `h ≥ 50`; the
combined descent is negative; no fibre arm beats rho (best `S ≥ 21`).

### CM-folded scalar-orbit MITM2 and the CM relation census — Experimental / Component

`ic.cm_folded_native_v1/v2`, `_recurrence_`, `_d43_`
([`ecbench/cm_folded.rs`](../../src/cryptanalysis/ecbench/cm_folded.rs)):
a compiled native backend builds a signed MITM2 table on a four-seed base
folded by a CM scalar orbit (`D = −19`, a `D = −43` degree-11 control and a
quadratic-recurrence control), probes below `r`, and every returned scalar
is replayed in Rust with separate arithmetic; `_v2` charges the replay and
names the unpriced native counters. 500 attempts verified; floor/crater
native CPU 0.98 [0.87, 1.11], "not an admitted IC/rho comparison"
([`research/conductor_gap_cm_orbits_20261015/REPORT.md`](../../research/conductor_gap_cm_orbits_20261015/REPORT.md)).
The P-192 norm-lattice relation census
([`p192_cm_norm_sieve.rs`](../../src/cryptanalysis/p192_cm_norm_sieve.rs),
[`p192_cm_relation_search.rs`](../../src/cryptanalysis/p192_cm_relation_search.rs))
factors `N(u + vπ)` over an oriented prime-ideal base and certifies class
relations by binary quadratic forms; "a relation is not an ECDLP weakness
until explicit maps, subgroup transport, evaluation costs and a fully
charged payoff all pass". No measured record. Component.

### Point-decomposition algorithms — Integrated

The Koblitz `DecompositionStrategy` has ten variants: the pair table (the
default), enumeration, Gröbner, SAT, three chained-`S₃` SAT forms, crossbred,
WDSat and MQ-FES; the prime-field oracles are listed below. Returned witnesses
are never trusted directly: the points are added again and checked against
the target before the relation is accepted.

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

### Boolean polynomial-system solvers (the `SystemSolver` registry) — Integrated (F4 family), Experimental (the rest)

[`ic_framework/solvers.rs`](../../src/cryptanalysis/ic_framework/solvers.rs)
registers twelve engines over `F₂[v]/(v² − v)`, every verdict one of
`Solved | Unsatisfiable | BudgetExceeded`, every returned solution
re-checked, and the contract "never guess Unsatisfiable": a budget
exhaustion is inconclusive. All engines emit complete `bitop/v1` tallies.

| engine | backing | budget honoured | note |
|:--|:--|:--|:--|
| `f4-f2` | [`pq_f4_f2.rs`](../../src/cryptanalysis/pq_f4_f2.rs): batched pairs, symbolic preprocessing, bit-packed M4RI-style elimination | wall | unit "word XORs (elimination only)", so the runner prices it by wall |
| `buchberger-f2` | [`pq_groebner_f2.rs`](../../src/cryptanalysis/pq_groebner_f2.rs) | wall; `BudgetExceeded` above 26 variables (extraction is by enumeration) | reports the solving degree |
| `matrix-f4`, `matrix-f5`, `inherited-f4` | [`koblitz_groebner.rs`](../../src/cryptanalysis/koblitz_groebner.rs), [`matrix_f5_f2.rs`](../../src/cryptanalysis/matrix_f5_f2.rs), [`inherited_f4.rs`](../../src/cryptanalysis/inherited_f4.rs) | node budget (default 4096) and matrix caps only, no wall clock | Macaulay cap `max_degree` 3, DPLL(GB) splitting rules `auto|lowest|highest|frequent|mom`, forced-variable propagation; `inherited-f4` is the default Koblitz Gröbner engine (children specialise the parent's reduced basis) |
| `crossbred-f2` | [`crossbred.rs`](../../src/cryptanalysis/crossbred.rs) (Joux–Vitse: Macaulay left kernel at degree `D`, `2^k` bit-sliced linear solves) | own caps; wall ignored | also `DecompositionStrategy::Crossbred` |
| `xl-f2` | [`pq_xl.rs`](../../src/cryptanalysis/pq_xl.rs) | none; registry cap 10 unknowns (about 150 s per call against 7 ms for Buchberger at 12) | Component, as before |
| `sat-cdcl` | [`sat.rs`](../../src/cryptanalysis/sat.rs) (in-house CDCL: two watched literals, 1-UIP, VSIDS, Luby restarts, native XOR Gauss–Jordan) with [`semaev_sat.rs`](../../src/cryptanalysis/semaev_sat.rs) encodings | conflict budget (200,000); wall ignored | first solution only; unit "conflicts" |
| `fes-f2`, `fes-f2-wide` | [`mq_fes.rs`](../../src/cryptanalysis/mq_fes.rs) (libfes-lite Gray code; wide = 8 AVX2 or 16 AVX-512 sub-cubes in 32-bit lanes) | exact, no budget | quadratic, `n ≤ 32 (+log₂ lanes)`; the strongest reference arm |
| `exhaustive` | every equation at every point, cap 26 variables | exact | "the reference, not a strawman" |
| `m4gb-f2` | [`m4gb_oracle.rs`](../../src/cryptanalysis/m4gb_oracle.rs): external M4GB (Makarim–Stevens 2017) behind `M4GB_BINARY`, GPL, never linked | wall only, no operation count | a reference arm, never a leaderboard row ([`docs/ic/M4GB_INTEGRATION_ASSESSMENT_20261110.md`](M4GB_INTEGRATION_ASSESSMENT_20261110.md)) |

Measured (stage diagnostics, pre-gate): the hybrid F4 stage cut counted
word XORs 1.60× and wall 2.84× across the frozen ladder deciding every
target identically ([`research/groebner_stage_20260915/`](../../research/groebner_stage_20260915/));
F4 per-target cost grows 8.6–9.3 bits per base-dimension rung against
0.33–0.75 for enumeration and the pair table, "F4 is measured to be the
wrong tool on both tracks" ([`research/pdp_root_finder_requirements_20261012/`](../../research/pdp_root_finder_requirements_20261012/README.md));
crossbred wins the oracle but loses the pipeline, exhaustive enumeration
beating every algebraic oracle 3.4–1,800× end to end
(RESEARCH_ECC2K130_CROSSBRED.md); M4GB overtakes the in-house `f4-f2` from
`n ≈ 19` at `m = 2` but stays 7–176× behind `fes-f2`, and at `m = 3`
`inherited-f4` beats both ([`research/m4gb_descent_ladder_20261110/`](../../research/m4gb_descent_ladder_20261110/RESULTS.md));
the native-XOR SAT reading of 0.854× rho at `n = 17` becomes 7.34× at
0.1 GAE per conflict and 51× at 1 GAE
([`research/sat_pdp_pricing_band_20261111/`](../../research/sat_pdp_pricing_band_20261111/README.md)).
The Möbius and Monica enumerators in `mq_fes.rs`/`mq_monica.rs`, the
GPU/Metal FES kernels ([`fes_gpu.rs`](../../src/cryptanalysis/fes_gpu.rs),
`gpu/fes/`, reachable only through `icx fes`), the unwired Redis
[`groebner_cache.rs`](../../src/cryptanalysis/groebner_cache.rs) and the
opt-in [`algebra_cache.rs`](../../src/cryptanalysis/algebra_cache.rs) /
[`polynomial_reuse.rs`](../../src/cryptanalysis/polynomial_reuse.rs)
template (cache overhead erases the saving at frozen sizes) are
Components.

### External solvers: WDSat, SMT, CryptoMiniSat, msolve — Experimental (WDSat strategy), Component (others)

[`wdsat_oracle.rs`](../../src/cryptanalysis/wdsat_oracle.rs) is the one
external tool wired as a decomposition strategy: it emits ANF in the
Trimoska benchmark format, writes a per-instance capacity config, shells
out, lifts any model through the group check, and treats a nonzero exit,
empty output, timeout or capacity failure as exhausted, never refuted
(child-status protocol); WDSat's static allocation crashes (`rc = −11`)
above about 24 M table entries. [`smt_oracle.rs`](../../src/cryptanalysis/smt_oracle.rs)
runs cvc5 or Z3 on the identical Weil-descended `S₄` system (`QF_UF` and
`QF_BV` encodings, blocking-assertion enumeration): the parity-gap
hypothesis failed at `n = 13` (Z3 at 0.76× native conflicts), `n = 19`
hit the cap. [`prime_field_smt.rs`](../../src/cryptanalysis/prime_field_smt.rs)
encodes the prime two-point decomposition with `x < B` as `QF_BV`/`QF_FF`
(cvc5 "unknown" at 20 bits in 120 s). CryptoMiniSat appears only as
exports and the `pycryptosat` adapter of [`groebner_compare/`](../../groebner_compare/README.md),
the Python comparison protocol over SymPy, Sage/PolyBoRi, Groebner.jl,
M5GB and hybrid guess arms, which claims no speedup. msolve is used in
`F_p` parity runs and GF(2) degree cells only. M4GRI appears nowhere in
the tree.

### Koblitz decomposition encodings beyond the pair table — Integrated (`chained-x`, `descent-algebraic`), Experimental (the rest)

The Koblitz `DecompositionStrategy` has ten variants, the pair table being
the default and Gröbner opt-in: `Groebner`, `Enumerate`, `Sat`,
`ChainedS3`, `ChainedS3FrobeniusOrbit`, `ChainedS3FrobeniusOrbitOrdered`,
`PairTable`, `Crossbred`, `Wdsat`, `MqFes`.

- `chained-x` ([`plugins.rs`](../../src/cryptanalysis/ic_framework/plugins.rs)):
  the chained x-frame descent, `m − 1` Semaev `S₃` links with `(m − 2)·n`
  free intermediate abscissae (`m·ℓ + (m − 2)·n` unknowns, cap 64), solved
  by the hybrid engines directly (`engine=inherited-f4|matrix-f4|matrix-f5`,
  `max_degree`, `node_budget`, `trace`); end to end through `ic bench`
  with relations re-summed and refutations matched against enumeration.
  `inherited-f4` reads 1.105 / 1.582 / 2.091× rho at `n = 13 / 19 / 23`
  on the `E₀` ladder as a lower bound (leaderboard, pre-gate).
- `descent-algebraic` (`m = 2 → S₃`, `m = 3 → S₄`; `slots=all` for the
  ordered τ-slot variant): the Petit–Quisquater Weil descent built
  symbolically by [`pq_descent_symbolic.rs`](../../src/cryptanalysis/pq_descent_symbolic.rs)
  and handed to any registered `SystemSolver` (`solver=f4-f2|buchberger-f2
  |sat-cdcl|exhaustive|…`); the one oracle through which every engine
  reaches a verified logarithm.
- Chained `S₃` SAT for 4–15 summands
  ([`binary_semaev_chain_sat.rs`](../../src/cryptanalysis/binary_semaev_chain_sat.rs)):
  a full-width native-XOR circuit per intermediate-identity pattern with
  exact domain constraints (explicit coordinates, representative plus
  Frobenius phase, ordered); a capped pattern is inconclusive. At `n = 83`
  public targets are refuted with 0 conflicts at `m = 5…8` (0.8–77 s) and
  cap at `m = 9`; planted targets 0/2 at every `m`
  ([`research/n83_chain_sat_meet_20261101/`](../../research/n83_chain_sat_meet_20261101/README.md)).
  [`semaev_sat.rs`](../../src/cryptanalysis/semaev_sat.rs) (Tseitin vs
  native XOR, summand-bit branching, symmetry breaking: 74.2 → 17.8 s at
  `n19l6`) and the matched `n = 17` SAT pipeline record sit here.
- `pdp-split` (ecbench; [`pdp_split.rs`](../../src/cryptanalysis/pdp_split.rs)):
  cut an `m`-summand decomposition into blocks joined by full-field
  auxiliary abscissae and guess `g` unknowns (Bettale–Faugère–Perret
  hybrid; plans `[3]`, `[2,1]`, `[2,2]`, …). Guessing four unknowns cuts F4
  XORs 10–128× per branch, every branch charged, yet the projected `S`
  stays `1.5·10¹`–`5.7·10⁶`× strong rho and the F4 cost per branch grows
  2.7–8.7× per residual unknown against 2× for enumeration, so the hybrid
  optimum is full enumeration.
- Six-sum extraction ([`koblitz_sixsum_extract.rs`](../../src/cryptanalysis/koblitz_sixsum_extract.rs),
  oracle `sixsum-extract` in `ic_run`): 4-, 5- and 6-sum relations over
  the Frobenius pair table (kernels A, B, B2, C). Only B2 converts
  multiplicity into fewer solves, but one `S₄` root solve costs 20–60 `S₃`
  solves (1–8× more work); the result corrected the feasibility note's
  §4.7 accounting ([`IC_TECHNIQUE_INTEGRATION_20261012.md`](IC_TECHNIQUE_INTEGRATION_20261012.md) §7).
- F6-IC (`groebner_decompose_f6_ic`), the wide solvers
  [`wide_groebner.rs`](../../src/cryptanalysis/wide_groebner.rs) (`u128`
  monomials, 128 unknowns, branching on whole summands) and
  [`wide_sixsum.rs`](../../src/cryptanalysis/wide_sixsum.rs) (512
  variables, the direct `n = 83` six-summand chain: planted control exact,
  ordinary target "root gate inconclusive"), the rotated complete-point
  gate [`koblitz_rotated_chain.rs`](../../src/cryptanalysis/koblitz_rotated_chain.rs)
  (`n = 83`, ten rotated subspaces, public target timed out at the 120 s
  CaDiCaL cap), and the blinded PDP panels of
  [`koblitz_pdp_phase_a.rs`](../../src/cryptanalysis/koblitz_pdp_phase_a.rs)
  (Component) are the `n = 83` stage gates: none has produced a natural
  relation.
- Symmetrised direct `S₄` ([`koblitz_sym_s4.rs`](../../src/cryptanalysis/koblitz_sym_s4.rs);
  quadratic after descent on a geometric base, `6ℓ − 3` unknowns) agrees
  with the chain and brute force; the symmetrised end-to-end arms read
  987–5,767× rho at `n = 17` ([`research/koblitz_symmetrised_e2e_20260927/`](../../research/koblitz_symmetrised_e2e_20260927/RESULTS.md)).

### Degree-of-regularity levers, Macaulay closure and complexity tooling — Experimental / Component

[`degree_reduction.rs`](../../src/cryptanalysis/degree_reduction.rs) (lever
L3, hybrid slicing of the solving degree `D*`) and
[`degree_reduction_anf.rs`](../../src/cryptanalysis/degree_reduction_anf.rs)
(lever L2, symmetrisation at `m = 3`) are measurement harnesses whose own
headers record hybrid slicing and composition as killed and the `Δ_low`
screen as a size proxy. [`partial_enlargement.rs`](../../src/cryptanalysis/partial_enlargement.rs)
implements MutantXL fixed-degree closure (Buchmann–Cabarcas–Ding–Mohamed
2010) with checkpoints: it refutes the descent systems at degree 3–4
where XL needs 5–7 (92 of 92 unsatisfiable draws), and the degree-5
closure at `m = 3`, `n = 19`, `l = 7` refutes in 31,632 s and 52.6 GB
where the dense pass aborted at 424 GB; a memory diagnostic, no IC cost
change. [`sparse_macaulay.rs`](../../src/cryptanalysis/sparse_macaulay.rs)
answers "is 1 in the row space" by structured sparse elimination on
degree-6 matrices and argues Wiedemann gives no reduced basis;
[`koblitz_groebner/checkpoint.rs`](../../src/cryptanalysis/koblitz_groebner/checkpoint.rs)
is the resumable state of the long eliminations. The first-fall-degree
predictors `descent_algebraic.rs` (Hilbert-defect), `descent_expansion.rs`
(spectral expansion, refuted as predictor), `descent_lowgamma.rs`,
`descent_treewidth.rs` (constant: the descended primal graph is complete)
are diagnostics of the FFD programme, not oracles.
[`ic_descent_degrees.rs`](../../src/cryptanalysis/ic_descent_degrees.rs)
(`ic descent --solver`) reproduces Petit–Quisquater Table 2 quantities on
the repository's own systems. [`matmul_exponent.rs`](../../src/cryptanalysis/matmul_exponent.rs)
carries `ω` with provenance for the cost models; [`lopsided_thin_product.rs`](../../src/cryptanalysis/lopsided_thin_product.rs)
screens the Alman–Vassilevska Williams thin product as an insertion point
without implementing it (Backlog as a technique).

### The three 2026-10-12 WIP items, re-read on 2026-11-12

- **G1 Joux–Vitse F4 trace reuse**: not implemented for binary systems; the
  `matrix-f4:trace=replay` toggle named in the lead ladder does not exist.
  Two nearby caches landed off by default and measured null:
  `KIC_F4_ROOT_REPLAY=1` (0.954–0.983) and `KIC_F4_PREFIX_ROOT=1`
  (0.989–1.006). Backlog.
- **G3 sparse Macaulay plus Krylov (BooleanSolve) refutation**: no code in
  the tree; the `n = 41` falsifier numbers do not exist. Backlog.
- **G2 Faugère–Svartz equivariant F4**: landed as example probes and a
  one-pass diagnostic (block RREF over `F_{2^d}` with inverse transform,
  bit-identical to the full `F₂` RREF at `n = 7…17`, 44–73× CPU at
  `n = 11…17`; [`research/ic_next10_20261014/06-equivariant-f4-m4/`](../../research/ic_next10_20261014/06-equivariant-f4-m4/README.md)),
  explicitly not wired into `reduce_system`. Experimental; an engine is
  Backlog.

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

### CADO-style purge and merge filter — Integrated

`FilterStrategy::Cado` in [`koblitz_sparse_la.rs`](../../src/cryptanalysis/koblitz_sparse_la.rs)
(the default of `ic logs --filter-strategy cado`): purge spends the excess
over up to 50 steps by deleting whole connected components of rows joined
by weight-two columns, heaviest first under the `Ω₃₁` weight, then cascades
singletons; merge is the Markowitz spanning-tree merge of light columns.
`legacy` is the earlier singleton-plus-star-merge filter. Tests include a
solve modulo the ECC2K-130 order. (Bouvier, CADO-NFS filtering, 2013.)
### Group-verified rank accounting (`audit_rank`) — Component

[`ic_framework/useful_rank.rs`](../../src/cryptanalysis/ic_framework/useful_rank.rs)
(`ic bench --audit-rank`, automatic for the `archive` base) checks every
row entering the matrix as a group equation, counts verified, invalid,
independent, dependent and duplicate rows, tracks rank with and without the
target column, and keeps a bounded per-row receipt transcript that `replay`
re-feeds to a fresh elimination and requires to agree (schema
`ic-useful-rank/v1`). Duplicate normalisation is reported unpriced.
Incremental Gaussian solvers only, up to 4096 columns.

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

### Relation-matrix registry and completion rules — Integrated

`run_pipeline` takes its matrix from the `RelationSolver` registry in
[`ic_framework/linalg.rs`](../../src/cryptanalysis/ic_framework/linalg.rs)
(`ic bench --linalg <name>`), which ships ten names, not the three the
framework guide lists: `incremental-gauss` (dense reduced row echelon),
`incremental-gauss-full-rank` and `-full-rank-checked` (the latter
point-checks one representative per base column after full rank, both
scalar multiplications charged), `incremental-gauss-echelon` (sparse
forward elimination with an `O(1)` target pin), `structured-gauss`
(Markowitz lightest-column pivot, "a few per cent, not a factor" over dense
in row operations), and the deferred sparse family `sparse-cado`
(= `filter+wiedemann`), `filter+lanczos`, `wiedemann`, `lanczos`, which
buffer rows until every column is covered and then hand the batch to
`koblitz_sparse_la` with the CADO-style or keep-all filter and block
Wiedemann or Lanczos, accepting an answer only when a second,
independently seeded solve agrees. Two completion rules: `TargetPinned`
(stop when the target column is determined; default) and `FullRank` (every
base column too; forced for the deferred family). Known-log bases bypass
the matrix through `KnownLogSolver`. Work is counted in `row_ops`, one
multiply-subtract on a non-zero entry, a lower bound that omits the dense
column scans and the block arithmetic; it is priced in `S` only where the
curve has a pinned `ns_per_row_op`. Pipeline moduli are `u64`; wider
subgroups go through the wide path below.

Measured, all null for `S`: the deferred sparse matrices cost more end to
end than `structured-gauss` because full-rank completion draws 1.75–10.5×
more relations ([`research/sparse_la_timing_20261018/`](../../research/sparse_la_timing_20261018/README.md));
`sparse-cado` and dense full rank give identical `S` at `|F| = 1,024…16,384`
while `sparse-cado` counts 1.5× the row operations and spends 2–4× less
linear-algebra wall time, with the `|F|²` oracle setup at 84–89 % of wall
([`research/ic_linalg_pipeline_20261019/`](../../research/ic_linalg_pipeline_20261019/README.md));
the CADO-style filter is 10.40× [9.99, 11.06] faster than the legacy filter
at 262,144 columns and shrinks the core 26,750 → 3,424 rows, with linear
algebra at 0.11–1.9 % of a real `ic logs` run
([`research/la_engines_20261014/`](../../research/la_engines_20261014/README.md)).
None is validated for the gate.
### Wide-modulus sparse linear algebra — Integrated algorithm, Experimental at scale

`koblitz_sparse_la` is generic over a modular word (`u64` fast path, then
`Wide<2|3|4>` limbs for odd moduli below `2^{255}`; `MAX_MODULUS_BITS =
255`), so the same filter, block Wiedemann and block Lanczos run on
ECC2K-130's 129-bit `r`; tests cover 127-, 190-, 191- and 250-bit moduli and
the ECC2K-130 order on planted systems. The Koblitz driver's own incremental
echelon solver ([`koblitz_relation_solver.rs`](../../src/cryptanalysis/koblitz_relation_solver.rs))
has a matching word and `BigUint` pair. The `10⁹`-column ECC2K-130 cost
"remains an extrapolation", and the benchmark pack the 2026-10-12
integration note cites (`research/ic_techniques_20261012/wide_sparse_la/`)
does not exist in the tree. The wide `k = 4` pipeline
[`ic_wide.rs`](../../src/cryptanalysis/ic_wide.rs) (`u128` scalars,
`bin-t-line` + `gaudry-d4-bin` + `walk` + scalar `Wiedemann<S>`) is the
one end-to-end consumer past one word; its crossover study ran on source
not on `main` and none of its sessions was replayed (see §6).
### Petit–Quisquater sparse elimination — Component

[`pq_sparse_la.rs`](../../src/cryptanalysis/pq_sparse_la.rs): structured
Gaussian elimination with Markowitz pivoting over `Z/N` (composite `N`
tolerated), used only by the Petit–Quisquater path and
`pq_wiedemann.rs`; "demonstrates the right algorithm structure even if we
never run it at production scale".
### Sparse FGLM linear stage after F4 — Integrated inside the Joux–Vitse route

[`zero_dim_la.rs`](../../src/cryptanalysis/zero_dim_la.rs) reads the
`F_p` points of a zero-dimensional Gröbner basis (`n ≤ 8`, `p < 2^63`) with
one transposed Krylov pass serving every coordinate, Berlekamp–Massey, roots
by `gcd(f, x^p − x)` and a rational univariate read-out, falling back to the
dense stage when it gives up. Default in `jv_cover` (`JV_LIN=sparse`):
linear stage 4.87·10⁵ → 2.30·10⁵ multiplications at `p = 53`, end-to-end
`S` 1.28× cheaper, class engineering
([`research/jv_cover_sparse_fglm_20261018/`](../../research/jv_cover_sparse_fglm_20261018/README.md)).
The dense `F₂` kernel [`gf2_elim.rs`](../../src/cryptanalysis/gf2_elim.rs)
(Method of Four Russians, AVX2/AVX-512 row updates, counted in word XORs)
is the elimination inside the Macaulay/F4/F5 oracle stage, not the
relation matrix.

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

### Factor-base logarithm database and one-target descent (`ic logs`, `ic solve`) — Integrated

`ic logs` precomputes the Koblitz factor-base logarithm database
(relation collection, `koblitz_sparse_la` filter, block Wiedemann or
Lanczos; `--linear-algebra sparse|dense`, `--sparse-solver`, `--spmv
auto|…|worker|sharded`, `--filter-strategy`, `--block-size 1..64`,
`--log-store`), and `ic solve` reloads it, re-verifies every column
`[x]G = R`, or reads a shared store keyed by the SHA-256 of curve, generator
and materialised base ([`src/bin/ic/log_store.rs`](../../src/bin/ic/log_store.rs),
runs labelled amortised by `log_store::provenance`), then descends one
target with `IndividualLogSolver` (`m = 2…4`, pair-table solver), accepting
only `[d]G = Q`. Degree `3…127`, subfield degree `≤ 8`, synthetic
known-answer or random targets.

### One-target Koblitz pipeline, ledger §23 (`ic.koblitz_descent`, `ic price --single-target`) — Integrated

[`ecbench/koblitz_descent.rs`](../../src/cryptanalysis/ecbench/koblitz_descent.rs)
assembles the seven stages exactly as `ic price` does: subgroup-orbit base,
`PairSumTable` tier by probe budget, **aimed** `m = 3` windowed collection
(`collect_aimed`, column coverage, `unit_trials × units`), sparse logs with
every column point-checked, descent set-up, a walked `m = 2` descent with
an online interval, and verification; the `rule_v8` parameter value loads
the frozen rows of `research/ic_tool_program/rule/v8/`. Every stage is
charged by counted point additions and doublings, the rest reported
`*_uncharged`. It costs 1.41–1.49× the `ic.pipeline` plan at `k41/k47/k53`
(descent/rho 13.45 / 24.10 / 33.81, pipeline/rho 9.46 / 16.19 / 23.99, all
96 runs verified;
[`research/ic_koblitz_descent_routing_20261026/`](../../research/ic_koblitz_descent_routing_20261026/README.md)),
so the planner keeps routing to the pipeline.

### Target-blind shared full-rank log and point-only descent (`ic.shared_rank`) — Experimental

[`ic_framework/shared_rank.rs`](../../src/cryptanalysis/ic_framework/shared_rank.rs):
stage one, blind to the targets, builds the compact-orbit base and the
folded `mitm-frobenius-counted:m=3` table and collects relations over `[a]G`
alone until the matrix has full column rank, point-checking every solved
column and hashing the base; stage two queries `Q` directly, then residual
points `Q + [a]G`, and recovers `d` with a full-point replay. Koblitz,
`n ≲ 62`; "native work remains visible and unpriced". At `n = 37` it
verified 5/5 at cold `S/rho` 13.59 and 0.0619 ms online against rho's
0.744 ms; the `n = 41/53` panel's decision is
`frozen_parameters_do_not_transfer` (K8 verified 0/80 on both sizes).

### Batched collection with rank checks (`ic.koblitz_batched`) — Experimental

[`ecbench/koblitz_batched.rs`](../../src/cryptanalysis/ecbench/koblitz_batched.rs)
reproduces the tournament worker (`examples/ic_tournament_worker.rs`):
relation collection in batches of `batch_trials` queries with a sparse
solve attempted after every batch once rows reach columns, then descent.
The L0 screen found the counted cost falls monotonically as the batch
shrinks to 1 (batch 16 at 0.69–0.80× batch 64, 1600 records, 1280 replays
reproduced), "not promotable"; the isolated L2 protocol is proposed and not
run.

### Recursive point decomposition and descent adapters — Experimental (one prime adapter), Component (scheduler alone)

[`recursive_descent.rs`](../../src/cryptanalysis/recursive_descent.rs)
implements the budgeted scheduler (memoisation, depth/node/relation caps,
cycle rejection, verified relations, a canonicalisation hook returning the
representative and the multiplier transporting its logarithm). Until
2026-11-12 it had no caller outside its own two toy tests; the earlier
"Integrated scheduler" wording was corrected to Component then.

It now has one curve adapter,
[`ic_framework/recursive_adapter.rs`](../../src/cryptanalysis/ic_framework/recursive_adapter.rs)
(`PrimePairTableDescent`), on prime curves over the counted group:
nodes are subgroup points canonicalised by negation only (multiplier 1 or
`r − 1`); a node is a leaf when it is a factor-base point whose certified
column logarithm is supplied (the adapter does not know how the logs were
obtained); relations are a two-summand pair-table decomposition through
the existing `PairTable::build_negation_folded` / `decompose_mitm`, and
one-large-prime relations `node = P_i + L` with `L` outside the base
returned as a child, so the scheduler recurses on `L` (bounded by
`max_large_prime_probes` per attempt). Every relation is re-checked as
`[a]·node = [c]G + Σ [b_i]·child_i` in the counted group after the witness
indices are matched against the base, and every logarithm by `[d]G = node`.
`replay_transcript` re-verifies a recorded descent (every step's relation,
every child log as a base leaf or an earlier step, every step result, the
final logarithm) without re-running the search: the frozen termination
transcript.

Tests (`cargo test --lib recursive`, 6 passed): on a 20-bit prime-order
curve (`r = 1,046,999`) with a 200-column known-log base, a planted target
that is neither a base point nor a pair-table hit is recovered through a
large-prime child and its transcript replays; the replay rejects a wrong
final logarithm, a tampered coefficient, a tampered child, a tampered
child log and a dropped step; `max_depth = 1` solves a direct pair and
refuses large-prime recursion (the scheduler checks depth before leaves,
so `max_depth = 0` admits only a target that is itself a base point); a
node budget of 1 fails with `NodeBudget`, never a wrong log. In the
end-to-end test the descent spent 25,011 additions, 675 doublings and 38
scalar multiplications beside the 40,200-addition table build, almost all
of it on failed large-prime branches (8,256 probes, 8,250 children, 8,204
relations verified, 0 rejected; 164 nodes, maximum depth 3); nothing was
tuned, and no comparison with the one-relation `ic solve` descent or with
rho has been run. Experimental: a working bounded adapter with replayable
transcripts, not registered as an `ic bench` oracle and not on any
production path. Koblitz (signed Frobenius), Diem and Gaudry adapters
remain to be written; the other descent modules are diagnostics:
[`coordinate_descent.rs`](../../src/cryptanalysis/coordinate_descent.rs)
ranks the Gaudry, sign-symmetrised and Klein-invariant systems on one
engine at `p^k ≲ 5000`, `m = 2`, recovering no scalar;
[`diem_descent.rs`](../../src/cryptanalysis/diem_descent.rs) is the toy
Diem `(p, k) = (5, 2)` with brute-force decomposition, "not subexponential";
[`ghs_descent.rs`](../../src/cryptanalysis/ghs_descent.rs) is end to end
for `m = 1` only.

## 6. Higher-genus index calculus, covers and transfers

Every route in this section leaves the elliptic curve. The matched reference
is rho on the source curve, priced on both sides under AGENTS.md §5a. No
entry here is a statement about a prime-order or deployed curve: every
sub-rho number below is on a structured weak class or a toy non-elliptic
Jacobian, and only one record
([`research/ic_cover_weakclass_20261023/`](../../research/ic_cover_weakclass_20261023/))
carries a `VALIDATION.json`. All other numbers are `[pcc:unverified]`.

### Hyperelliptic (Adleman–DeMarrais–Huang / Gaudry) index calculus — Experimental

[`hyperelliptic_index_calculus.rs`](../../src/cryptanalysis/hyperelliptic_index_calculus.rs)
runs the classical Jacobian method on genus 2–4 over odd-characteristic
fields: a factor base of degree-one places `[P] − [∞]` folded by `±`
(optionally a Gaudry reduced base), relations from `R = aD₁ + bD₂` or the
default factor-base walk, smoothness by a counted oracle (Legendre symbol at
degree ≤ 2, `gcd(u, x^p − x)` or Cantor–Zassenhaus above), sparse Markowitz
elimination modulo the prime order, and a verified `D₂ = [k]D₁`. Optional
single large primes pair partial relations. The word-size engine
[`hyperelliptic_word.rs`](../../src/cryptanalysis/hyperelliptic_word.rs)
(genus ≤ 4, `p < 256`) backs the ecbench ids `ic.hyperelliptic`,
`ic.hyperelliptic_v2` (verification and factor-base scan charged) and
`ic.hyperelliptic_thread` ([`ecbench/hyper_ic.rs`](../../src/cryptanalysis/ecbench/hyper_ic.rs)),
which refuses `r² | #J`. The characteristic-two companion
[`binary_hyperelliptic_index_calculus.rs`](../../src/cryptanalysis/binary_hyperelliptic_index_calculus.rs)
(`hecx run --field binary`) has end-to-end tests but no ecbench id and no
measured record. [`hec_dlp.rs`](../../src/cryptanalysis/hec_dlp.rs) is the
front door (Pohlig–Hellman plus BSGS, index calculus only in the large prime
component); [`hyperelliptic_ic_bench.rs`](../../src/cryptanalysis/hyperelliptic_ic_bench.rs)
is the matched Jacobian-rho harness.

Measured: in its own in-situ unit genus 2 sits at about 1× rho, genus 3 at
0.66 → 0.23 and genus 4 at 0.15 (`p = 61`); re-priced in `bitop/v1`
([`research/hec_bitop_20261020/`](../../research/hec_bitop_20261020/README.md),
leaderboard section outside tables A–C), genus 4 at `p = 61` reads 0.114
[0.096, 0.138] of rho and only genus 4 has a falling slope (−0.23 per
doubling of `log N`, against +0.27 and +0.10 for genus 2 and 3). In ecbench
against `rho.negation`
([`research/ecbench_jacobians_transfers_20261025/`](../../research/ecbench_jacobians_transfers_20261025/README.md)),
genus 2 costs 1.47 [1.16, 1.87]× rho and stays above it at every band
weight. Toy Jacobians only; these curves are not elliptic, so this is a
reference for the cover routes, not an ECDLP route by itself.

### Gaudry–Diem extension-field decomposition (`k = 3, 4, 5`) — Experimental

Decomposition of `R` into `k` points of the subfield-abscissa base
`{P : x(P) ∈ F_p}` on `E(F_{p^k})`, with the Weil-restricted summation
polynomial solved by Macaulay/F4 elimination:

- `k = 3`: [`gaudry_cubic.rs`](../../src/cryptanalysis/gaudry_cubic.rs)
  (random curves over `F_{p³}`, meet-in-the-middle and algebraic `S₄` solve,
  `LargePrimeEliminator`, rho on the same group; also the shared polynomial
  and Wiedemann kernels the other files reuse). Measured (encyclopedia B1):
  MITM 1,057× rho at `2^24.2` rising to 2,240× at `2^33.1`; the
  double-large-prime arm has exponent 0.424 but 1,989× rho, parity
  extrapolated past `2^230`.
- `k = 4`, odd characteristic: [`gaudry_quartic.rs`](../../src/cryptanalysis/gaudry_quartic.rs)
  measures one `S₅` solve (`C₄ ≈ 1.2·10¹²` field multiplications, Macaulay
  degree 29); [`gaudry_quartic_d4.rs`](../../src/cryptanalysis/gaudry_quartic_d4.rs)
  applies the FGHR 2-torsion fold (`D₄` of order 192, degree 14, weighted
  four-unknown form at `1.41·10⁹` multiplications per solve, about 850×
  below plain, derived handover `2^118.9`); the ecbench method `ic.k4_gaudry`
  ([`ecbench/fp4.rs`](../../src/cryptanalysis/ecbench/fp4.rs)) runs the
  x-line or `D₄` Y-line base with an exact enumeration oracle, optional
  double large primes, filtering and Wiedemann, and charges each trial the
  measured solve cost `c₄` while its table and probe work stay
  `*_uncharged`, so its records are lower bounds that "can promote nothing".
  The fp4 ladder ([`research/ic_next10_20261014/01-fp4-ladder/`](../../research/ic_next10_20261014/01-fp4-ladder/README.md))
  is "NEGATIVE for a crossover with rho at any reachable size", handover
  extrapolated to `2^126`–`2^137`.
- `k = 4`, characteristic two: [`gaudry_quartic_bin.rs`](../../src/cryptanalysis/gaudry_quartic_bin.rs)
  (`t = β/(x + β)`, 105 invariant monomials, weighted degree 26),
  [`ecbench/xline_bin.rs`](../../src/cryptanalysis/ecbench/xline_bin.rs)
  (standard or twisted all-lift line, used by `ic.pipeline`) and the wide
  end-to-end pipeline [`ic_wide.rs`](../../src/cryptanalysis/ic_wide.rs)
  (`m ≥ 16`, `u128` scalars). The crossover study read 3.6× rho at `m = 15`,
  0.95× at 17 and 0.45× at 18 of rho's theoretical cost, but the red team
  ([`research/gaudry_k4_char2_redteam_20261026/`](../../research/gaudry_k4_char2_redteam_20261026/REPORT.md))
  classed the crossover "NOT SURVIVED (unsupported), not refuted": no clean
  measurement at `m ≥ 16`, two targets.
- `k = 4, 5` with `(k − 1)`-point decompositions (Joux–Vitse / Vitse):
  [`jv_quartic.rs`](../../src/cryptanalysis/jv_quartic.rs),
  [`jv_quintic.rs`](../../src/cryptanalysis/jv_quintic.rs),
  [`jv_quintic_edwards.rs`](../../src/cryptanalysis/jv_quintic_edwards.rs),
  [`joux_vitse.rs`](../../src/cryptanalysis/joux_vitse.rs),
  [`joux_vitse_edwards.rs`](../../src/cryptanalysis/joux_vitse_edwards.rs):
  the `k = 4` variant is flat at 6,945× rho; the `k = 5` Edwards 2-torsion
  symmetry cuts the solve constant 416× with parity derived at `2^233`.
- Components: [`glv_gaudry.rs`](../../src/cryptanalysis/glv_gaudry.rs)
  (ψ-orbit quotient and `Z/3`-graded Macaulay study on `j = 0`),
  [`f6_wide_geometry.rs`](../../src/cryptanalysis/f6_wide_geometry.rs)
  (four-summand pair closure on two-word binary curves),
  [`ext_curve.rs`](../../src/cryptanalysis/ext_curve.rs) (`F_{p²}`, `F_{p³}`
  counted groups).

### GHS Weil-descent covers of binary curves — Experimental

The GHS chain `E(F_{2^N}) → Jac(C)(F_{2^l})` and index calculus on the
descended Jacobian is complete at toy sizes: [`ghs_smooth.rs`](../../src/cryptanalysis/ghs_smooth.rs)
derives the **smooth** hyperelliptic model and the conorm–norm homomorphism
in Mumford arithmetic for any `y² + xy = x³ + ax² + b` with `a ∈ F_{2^l}`,
and [`ghs_ic.rs`](../../src/cryptanalysis/ghs_ic.rs) runs Gaudry-style
index calculus on it (degree-one factor base, dense or sparse relation walk,
incremental echelon modulo `ℓ`, verified on `E`); tests recover planted logs
at `N = 12` with `(l, n) = (2, 6)` (genus 8 over `F₄`) and at `q = 16`,
genus 4. This corrects the earlier matrix statement that the smooth
`m ≥ 2` model was missing. Around it: [`ghs_descent.rs`](../../src/cryptanalysis/ghs_descent.rs)
(end-to-end `m = 1` trace descent, symbolic `m = 2`),
[`ghs_quadratic.rs`](../../src/cryptanalysis/ghs_quadratic.rs) (explicit
`n = 2` genus-2 descent with a matched rho CLI),
[`ghs_full_attack.rs`](../../src/cryptanalysis/ghs_full_attack.rs) (glue:
isogeny walk → descent → HCDLP; the isogeny lift step is not implemented),
and the Component screens [`ghs_screen.rs`](../../src/cryptanalysis/ghs_screen.rs)
(magic number, genus, cover degree for every `N = n·l`),
[`ghs_generalized.rs`](../../src/cryptanalysis/ghs_generalized.rs) (Hess
profiles over every proper subfield),
[`ghs_class_coverage.rs`](../../src/cryptanalysis/ghs_class_coverage.rs)
(isogeny-class coverage of small magic numbers) and
[`ghs_transport.rs`](../../src/cryptanalysis/ghs_transport.rs).

Measured: the R7.2 end-to-end run (RESEARCH_ISOGENY_FIBERS_ECDLP.md §20)
has "IC ≥ rho at every instance", 1.91 [1.72, 2.11]× to 131 [123, 141]× the
best generic over 30 instances at `N = 12…28`, with a crossing extrapolated
near `N ≈ 32` and not built;
[`research/ghs_generalized_20261024/`](../../research/ghs_generalized_20261024/REPORT.md)
finds "no calibrated GHS advantage over rho". For ECC2K-130 the large horn
is at best `2^70.19`× rho and the small horn empty (encyclopedia E4); the
cover-genus floor study puts every GHS cover of `E/F_{p^k}` with
`F_p(j) = F_{p^k}` at genus ≥ 9, 49, 41 for `k = 6, 10, 14`. The
sect113r1-class screen is pinned in `tests/sect113r1_ghs_evidence.rs`.

### Joux–Vitse cover-and-decomposition on the weak class — Experimental

[`jv_cover.rs`](../../src/cryptanalysis/jv_cover.rs) implements Joux–Vitse
(Eurocrypt 2012): for `E : y² = x(x − α)(x − σα)` over `F_{q³}`, `q = p²`,
the conorm–norm map into the genus-3 `H : y² = F(x)N(x)` over `F_q`, Nagao
decomposition into six factor-base places by a six-equation quadratic
system, a sparse FGLM-style linear stage (Wiedemann, Berlekamp–Massey) and
negation / distinguished-point rho references on `E`, all counted in `F_p`
multiplications. [`jv_sieve.rs`](../../src/cryptanalysis/jv_sieve.rs) is the
JV12 §3.2 sieving variant (arms P, H, U), and
[`jv_isogeny_walk.rs`](../../src/cryptanalysis/jv_isogeny_walk.rs) prices
the walk from a non-weak curve into the class (weak-class test
`N_{F_{q³}/F_q}(cross-ratio) = 1`). [`jv_rho_strong.rs`](../../src/cryptanalysis/jv_rho_strong.rs)
and [`jv_bitops.rs`](../../src/cryptanalysis/jv_bitops.rs) are the reference
and accounting components.

This is the only index calculus in the repository measured below rho, and
only on the weak class (full rational 2-torsion, never prime order; about
55–63 % of full-2-torsion classes contain a weak curve). The validated-with-
caveats record ([`research/ic_cover_weakclass_20261023/`](../../research/ic_cover_weakclass_20261023/),
`VALIDATION.json` present) gives IC/rho 0.0244 [0.0206, 0.0292] pooled for
arm P and 0.0306, 0.0115, 0.0081 for arm U at `p = 1009, 1511, 1823`,
still below rho across the §5a band (P pooled 0.024 / 0.164 / 0.172). With
the isogeny-walk charge
([`research/jv_walk_charge_20261111/`](../../research/jv_walk_charge_20261111/))
the pooled ratios are 0.0155 / 0.0858 / 0.0900, and a random admitted start
reaches a weak curve in 3 of 60 cases. Against a strong rho (negation,
batched inversion) the crossover study
([`research/jv_cover_crossover_20261101/`](../../research/jv_cover_crossover_20261101/README.md),
no `VALIDATION.json`) reads 1.35, 1.06 and 0.91× at `p = 1301, 1451, 1601`,
"not resolved".

### Plane-quartic (Diem) index calculus on the GHS-weak genus-3 family — Experimental

[`plane_quartic_pic.rs`](../../src/cryptanalysis/plane_quartic_pic.rs)
provides canonical `Pic⁰` arithmetic on a smooth plane quartic (classes as
`x + D_∞` in `Pic³`, echelon bases of conics, no root finding in the group
law; about 3,523 multiplications and 44 inversions per addition, flat in
`q` up to `2³¹ − 1`), [`diem_quartic.rs`](../../src/cryptanalysis/diem_quartic.rs)
runs Diem's line-section relation collection (full, single- and
double-large-prime arms), linear algebra modulo `ℓ` and a priced rho on
`E₀/F_{q³}`, and [`bielliptic_quartic.rs`](../../src/cryptanalysis/bielliptic_quartic.rs)
is a hard-bounded tiny-field diagnostic on `v⁴ = x³ + ax + b`.
[`ecbench/plane_quartic.rs`](../../src/cryptanalysis/ecbench/plane_quartic.rs)
is the Jacobian workload the transfers use.

Measured ([`research/plane_quartic_program_20261018/CLOSING.md`](../../research/plane_quartic_program_20261018/CLOSING.md)):
the best pipeline (double large primes, structured Gaussian elimination,
block Lanczos) crosses negation rho at `q* = 555 [389, 752]` in `F_q`
multiplication equivalents at `w = 0`, is below rho at every size from
`q = 991` across the pricing band, and costs 0.005 [0.004, 0.008] of
negation rho at `q = 2,097,091` with an exponent gap of 0.560
[0.510, 0.600]. The note calls this "our measurements of a known result,
not a new attack": synthetic known-answer instances of the GHS-weak family
only, no target descent, no `VALIDATION.json`.

### Cover and isogeny transfers with generic inner solvers — Component

DLP-preserving maps into another group, followed by a generic solver and a
check on the source curve:

- `transfer.ghs_cover` ([`ecbench/ghs_cover.rs`](../../src/cryptanalysis/ecbench/ghs_cover.rs)):
  conorm–norm into the Joux–Vitse genus-3 Jacobian, inner `rho.negation`,
  everything counted in `F_p` multiplications and divided by the measured
  331 multiplications per affine addition on `E(F_{p⁶})`: 6.62 [5.13, 8.54]×
  rho, above rho at every weight.
- `transfer.quartic_cover` and `_v2` ([`ecbench/cover_quartic.rs`](../../src/cryptanalysis/ecbench/cover_quartic.rs)):
  pullback along the bielliptic double cover `V⁴ = X³Z + aXZ³ + bZ⁴ → E`
  into a plane-quartic Jacobian isogenous to `E × Prym`: 15,292
  [11,065, 21,134]× rho.
- `transfer.isogeny` ([`ecbench/isogeny.rs`](../../src/cryptanalysis/ecbench/isogeny.rs)):
  a rational Vélu `ℓ`-isogeny (`ℓ = 2, 3`) to an isogenous curve, solved
  there: 0.934 [0.744, 1.172]× rho, unresolved across the band, as expected
  for a cost-neutral transport.
- `jacobian_image.rho3` / `.bsgs` (+ `_field_tally`): certified degree-three
  same-field pullback of a binary curve into a genus-3 Jacobian
  ([`research/n83_genus3_image_solver_20261031/`](../../research/n83_genus3_image_solver_20261031/)):
  destination rho costs 9.8, 24.2, 125, 365× the source at `n = 9, 13, 19,
  23`, a ratio that grows with `n`; "rejects an exact `n = 83` generic-solver
  attempt".
- [`chained_transfer/`](../../src/cryptanalysis/chained_transfer/) (`E → E'
  → J(C)` by Vélu then bielliptic `φ^*`, with a genus-2 index calculus on
  `J(C)`): 9.8× rho at `p = 211` widening to 42.9× at `p = 1619`
  ([`research/chained_prime_transfer_20261111/RESULTS.md`](../../research/chained_prime_transfer_20261111/RESULTS.md)).
- The genus-2 `three_leads` methods (`ic.three_leads_genus2_*`, Scholten
  family; [`ecbench/three_leads.rs`](../../src/cryptanalysis/ecbench/three_leads.rs))
  and the APS single-relation instruments `ic.aps_v1/v2`
  ([`aps_single_relation.rs`](../../src/cryptanalysis/aps_single_relation.rs),
  a prime-field correctness instrument with symmetric-coordinate encodings,
  "not a performance or scaling claim") are Experimental counted methods
  that lose to rho in every record read (genus-2 transfer 3.8–7.9× rho over
  twelve arithmetic prices, "rho wins 12/12").

All transfers are correct and priced; none has an index-calculus inner
method that beats rho, and the plane-quartic double cover cannot by
construction. Isogeny tooling (the `isogeny_walk/` BFS with verified Elkies
kernels, `volcano/`, [`binary_velu.rs`](../../src/cryptanalysis/binary_velu.rs),
the Smith `(2,2,2)` modules, [`isogeny_class_search.rs`](../../src/cryptanalysis/isogeny_class_search.rs),
[`koblitz_isogeny_cost.rs`](../../src/cryptanalysis/koblitz_isogeny_cost.rs))
produces curves and certificates, not logarithms. The isogeny-fibre analysis
(RESEARCH_ISOGENY_FIBERS_ECDLP.md §0) shows relation generation is
isogeny-equivariant with zero lifting cost, so "decompose on `E₂` then lift"
equals decomposing on `E₁` with the pulled-back base; conductor gaps move
only `#Aut`, rational torsion or monomial support, never an exponent.
ECC2K-130's class has about `2^65.06` curves and `ℓ ∈ {2, 3}` never moves a
Koblitz class.

## 7. Orchestration, planning, forecasting and cost accounting

These are not decomposition techniques; they decide what runs, how it is
priced and what may be claimed. They are listed because every technique
above is measured through them, and because "relabelling and accounting
changes are not improvements" needs the accounting named.

### Staged pipeline and plug-in bench (`run_pipeline`, `ic bench`) — Integrated

[`ic_framework/mod.rs`](../../src/cryptanalysis/ic_framework/mod.rs) runs
factor base → oracle precompute → relations (the shared loop in
`ic_boundary`) → matrix → verification, every phase in one unit with
exclusive phase clocks ([`ic_measurement.rs`](../../src/cryptanalysis/ic_measurement.rs)),
and reports per-phase cost, `total_gae`, `S = total_gae/√r`, the matched
rho `S`, their ratio and a `cost_qualification` naming every unpriced
counter. `ic bench --list` prints every plug-in and parameter; sweep
files and `--auto` drive it. `u64` scalars (`r < 2^64`); the wide `k = 4`
path is the only consumer past one word.

### Planner (`ic plan`, `ic bench --auto`, `ic-planner/4`) — Integrated

[`ic_framework/planner.rs`](../../src/cryptanalysis/ic_framework/planner.rs)
chooses every stage from the curve class with a rule, a reason, evidence
sources and an extrapolation flag, and predicts the gap to matched rho:
prime → `prime-abscissa` at `round(#E^{1/3}/2)` with
`mitm-prime-blocked:m=2` and `incremental-gauss`; Koblitz odd `n < 64` →
`compact-orbit-scan` at `max(1, round(0.6·(r/n³)^{1/3}))` columns with
`mitm-frobenius-counted:m=2`, `stride-renew` below `log₂ r = 21` and `walk`
above; random binary → `binary-subspace` at `round(log₂ #E / 3)`; others →
rho only. The weight rule moves the Koblitz optimum to `0.71·c₀` columns
at `w ≥ 0.3`. "A plan is a hypothesis until a session confirms it":
validation on six prime curves read `S(plan)/S(rho)` 12.6–25.8 against
predicted 11.5–24.5 (leaderboard, outside tables A–C).

### Resumable workflow, frozen parameters and checkpoints — Integrated

`ic workflow` (select → collect in work units → logs → solve, with a
BLAKE3-digested `state.json`, atomic artifacts, `--collect-units a-b` for
distributed collection, every merged relation re-verified, every column
certified), `ic fixed` (the persistent Python ONB workflow through degree
131), `icx run --resume` (a smaller analogue of the named family through
the same entry points, never the named curve; `ICXCKPT1` checkpoints),
`ic corpus` (the Trimoska-style `S₄` SAT corpus with certified labels),
and the application checkpoints of the long Macaulay eliminations
([`closure_checkpoint.rs`](../../src/cryptanalysis/closure_checkpoint.rs),
`koblitz_groebner/checkpoint.rs`). The track rule of 2026-10-24
([`src/bin/ic/sweep/space.rs`](../../src/bin/ic/sweep/space.rs)): a
pipeline counts in the Algebraic track only if relations come from solving
summation-polynomial systems over an algebraically defined base without
table collisions; pair tables, MITM and large primes on a table are the
Table track, reported but never ranked against Algebraic; `rootfind-*` is
Unclassified. `ic sweep` is the typed TPE sweeper over base-size exponent
and descent dimension ([`docs/ic/SWEEPER.md`](SWEEPER.md)).

### Degree-of-regularity forecast and relation-yield pruning — Experimental / Component

[`dreg_forecast.rs`](../../src/cryptanalysis/dreg_forecast.rs)
(`dreg_forecast/v1`; `factor-base-sweep plan|run|triage|refine`, the
`dreg-triage` skill): each solve is a draw of the solving degree, decided
solves observe it and exhausted ones are right-censored at the highest
Macaulay degree built, so a Kaplan–Meier product limit gives the predicted
degree of regularity with a 95 % band against the Bardet–Faugère–Salvy
semi-regular yardstick; the cost side projects relation-phase work against
the rho floor at 1, 10 and 100 multiplications per group addition and
advises `promising|undecided|doubtful|hopeless_at_bound`. It never stops a
run and never certifies a degree; `m = 2` only in the sweeper.
[`relation_yield_pruning.rs`](../../src/cryptanalysis/relation_yield_pruning.rs)
(`icprog relation-prune`) is the write-once, risk-controlled checkpoint
analyser for candidate tournaments (futility stops from capped verified
yield, Bonferroni-corrected looks, Clopper–Pearson bounds, fails closed);
producer wiring remains follow-on. Component.

### Pricing arms, the §5a band and the `S` unit — Integrated instruments

`ic.pipeline` prices calibrated native work with ratios pinned in
[`calibration.json`](calibration.json) and lists the rest `*_uncharged`;
`ic.pipeline_full` re-prices every counter from
[`ecbench/full_prices.json`](../../src/cryptanalysis/ecbench/full_prices.json)
(one host's measured ratios to an affine addition: doublings,
Artin–Schreier solves, lookups, row operations, Frobenius maps,
canonicalisations, field operations, word XORs); `ic.pipeline_wordram`
prices them by the CurveSmash/HashSmash 256-bit word-RAM budgets
([`wordram_price.rs`](../../src/cryptanalysis/ecbench/wordram_price.rs),
host-independent, "one point to quote beside the band"); `ic.pipeline_counted`
rebuilds the relation charge from counted work without solver wall
pricing. The §5a band ([`ecbench/band.rs`](../../src/cryptanalysis/ecbench/band.rs),
`ecbench compare --weight`) charges every unpriced and every
calibration-priced native operation at `w ∈ {0, 0.1, 1}` GAE and ranks two
arms only where the order holds at every weight; since 2026-11-08 counted
table work is priced by default. `S` itself is total operations over `√r`,
one affine addition the unit, with the generic floor `√(π/2A)` and the
counting ceiling, as defined in [`ic_boundary.rs`](../../src/cryptanalysis/ic_boundary.rs)
and measured by `ic boundary` (the ledger,
[`RESEARCH_IC_BOUNDARY_LEDGER.md`](../../research/notes/index-calculus/RESEARCH_IC_BOUNDARY_LEDGER.md),
24 sections). `audit_rank` (§3) adds group-verified rank receipts.

### Validation gate, bounds and the frontier — Integrated instruments

No row reaches the leaderboard or `benchmarks/README.md` without a replay
certificate, an independent validator report and a matched rho reference
with the band, named by the evidence directory's `VALIDATION.json`
([`docs/VALIDATION_GATE.md`](../VALIDATION_GATE.md)); pre-gate rows render
"unvalidated (pre-gate)". Across the records this inventory cites, the
gate-validated ones are the Joux–Vitse weak-class sieve, the
generator-orbit base, the packed pair-table filter, the prime base-size
tuning and three Macaulay pivot studies; every other number above is
pre-gate. `ecbench bound fit|check` ([`ecbench/bounds.rs`](../../src/cryptanalysis/ecbench/bounds.rs))
seals a re-derivable bound (constant, exponent with intervals, per-phase
shares, unpriced work) from a committed arm, `ecbench frontier build`
keeps [`docs/bounds/FRONTIER.md`](../bounds/FRONTIER.md) of undominated
bounds, and `challenge` replaces one by a paired spec (skill
`ecbench-bounds`). The registry audit of 2026-11-01 ran 73 of 73 runnable
methods with 854 of 854 eligible replays reproduced
([`research/ecbench_registry_optimization_20261101/`](../../research/ecbench_registry_optimization_20261101/README.md)),
and the e2e frontier coverage sheet dispositions every method
(`current_primary`, `diagnostic_only`, `pricing_sensitivity`,
`specialized_domain`, `legacy_control`); "a disposition is not a
performance verdict".

## Integration order for the remaining items

1. Add collector-specific converters for the common large-prime relation type,
   retaining each collector's independent curve-group verifier.
2. Measure the prime recursive-descent adapter against the one-relation
   `ic solve` descent and matched rho on a frozen ladder, and add the Koblitz
   (signed-Frobenius canonicalisation), Diem and Gaudry adapters; the prime
   adapter and its replayable transcript landed on 2026-11-12.
3. Validate block Lanczos against the larger frozen block-Wiedemann corpus,
   and produce the wide-modulus benchmark pack the 2026-10-12 note cites.
4. Validate the CUDA SpMV worker, the GPU root finder and the FES kernels on
   named hardware without changing their transcripts.
5. Done 2026-11-12 for the pooled `ksum` study (validated). Still open: the
   external-tool comparison its README owes, and a second-host timed (L2)
   replay if a wall-time claim is wanted.
6. Pin `ns_per_hashed_tensor_step` on the calibration host (the key exists
   since 2026-11-12; protocol in
   [`calibration-solver-units.md`](calibration-solver-units.md)) and charge
   the omitted tensor-curve certification work; only then can the hashed arm
   leave `DiagnosticOnly`. Until pinned it stays a stage diagnostic.
7. Pin the solver-unit keys added on 2026-11-12 (`ns_per_conflict`,
   `ns_per_monomial_op`, `ns_per_monomial_test`, `ns_per_vector_xor_8x32` /
   `_16x32`, `ns_per_f4_muladd`, `ns_per_fp_mul`) per
   [`calibration-solver-units.md`](calibration-solver-units.md), which first
   needs its three missing measurement tools (an `ic bench` report reader, an
   `F_p` multiplication microbenchmark, a contraction-step timer). Until a
   curve's row pins a key, its unit stays a lower bound exactly as before;
   `word operations (partial)` (crossbred) and `monomial operations
   (modelled)` (XL) stay refused even when pinned, because their counts do
   not cover the whole run. Four pkm counters (`pkm_trace_learn_muladds`,
   `pkm_t0_*_inversions`, `pkm_crosscheck_fp_mults`) are neither priced nor
   listed as unpriced: listing them changes frozen ids' unpriced lists and
   needs its own versioned id.
8. Pursue G1 (binary F4 trace reuse) and G3 (BooleanSolve refutation) only
   with a preregistered falsifier; both are Backlog with no code.
9. Attempt general trace-zero or higher-genus GHS integration only after the
   explicit smooth-model map is checked at a non-toy size; the Jacobian path
   itself exists.

Each item is a separate research iteration. Before measuring, it requires a
versioned protocol with frozen inputs, a matched baseline, a derived boundary,
success and stop conditions, exclusive cost accounting, and verified
one-target recovery. Stage-only results remain labelled stage diagnostics.

## Corrections in this revision (2026-11-12)

- The trace-zero entry said the smooth `m ≥ 2` GHS model was missing;
  `ghs_smooth.rs` builds it and `ghs_ic.rs` runs index calculus on it (§6).
- `recursive_descent.rs` was listed as an Integrated scheduler; it had no
  caller outside its own tests and no curve adapter, so it was Component;
  the prime adapter `PrimePairTableDescent` with transcript replay now gives
  it one Experimental path (§5).
- `docs/ic/IC_TECHNIQUE_INTEGRATION_20261012.md` cites a wide-modulus
  benchmark pack (`research/ic_techniques_20261012/wide_sparse_la/`) that is
  not in the tree; the wide code is real and tested (§4). Its §8 verdict on
  the cyclotomic base ("not yet established") was settled yield-neutral on
  2026-10-15 (§1). Its three WIP rows G1–G3 are re-read in §2.
- `docs/ic/FRAMEWORK.md` §9 listed three relation-matrix plug-ins; the
  registry ships ten (§4; the guide is updated in the same change).
- `ic bench --list` prints `gls-line`, which the prime dispatch cannot run,
  and omits the `scalars`/`seed` parameters of `known-log` and the `mult`
  parameter of `cm-multiplier`; `cyclotomic`, `onb-weight`, `explicit-x`,
  `prime-interval` and `cm-scalar-orbit` are ecbench-only.
- The doc comment of `semaev_s4_in_x4` still says Cantor–Zassenhaus is not
  implemented; `find_roots_fp_fast` implements it.

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
- Gallant–Lambert–Vanstone (2001) and Duursma–Gaudry–Morain (1999),
  automorphism folds; Galbraith–Lin–Scott (2009, 2011), GLS curves.
- Petit–Kosters–Messeng (PKC 2016), prime-field towers and bit membership;
  Amadori–Pintore–Sala (2017, 2018), summation-polynomial systems and the
  single-relation instrument; Huang–Kosters–Yeo, fall degrees.
- Joux–Vitse (Eurocrypt 2012; J. Cryptology 2013) and Vitse (2011), cover
  and decomposition, crossbred, `(k − 1)`-point decompositions; Nagao,
  Jacobian decomposition.
- Gaudry–Hess–Smart (2002), Hess (2004), Maurer–Menezes–Teske (2002), GHS
  Weil descent; Diem (2006) and Diem–Thomé (2008), plane-quartic index
  calculus; Smith (2008/2009), the `(2,2,2)` isogeny.
- Adleman–DeMarrais–Huang (1994), Gaudry (2000), Thériault (2003),
  hyperelliptic index calculus and large primes; Cantor (1987).
- Bettale–Faugère–Perret, hybrid guessing; Buchmann–Cabarcas–Ding–Mohamed
  (2010), MutantXL; Makarim–Stevens (ISSAC 2017), M4GB; Bouillaguet et al.,
  libfes; Trimoska–Ionica–Dequen, WDSat and the benchmark corpus;
  Soos–Nohl–Castelluccia (2009), XOR-aware CDCL; Bardet–Faugère–Salvy,
  semi-regular degrees; Shantz–Teske (2013); Galbraith–Gebregiyorgis.
- Pomerance (1996), LaMacchia–Odlyzko (1990), structured Gaussian
  elimination; Bouvier (2013), CADO-NFS filtering; Bernstein–Lange (2013),
  preprocessing tables; Huang–Kosters–Petit–Yeo–Yun (2020), quasi-subfield
  polynomials; Alman–Vassilevska Williams (2026), the thin product (not
  implemented).
