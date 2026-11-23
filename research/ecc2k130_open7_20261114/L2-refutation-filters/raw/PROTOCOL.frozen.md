# L2 refutation filters: protocol (frozen 2026-11-14, before any measured run)

Lane L2 of `research/ecc2k130_open7_20261114` (rules: `../COMMON.md`).
All statements here and in the results are `[pcc:unverified]`.

## Prior art (audit done before this freeze; details in REPORT.md)

- In-solver early abort at the refutation degree (InheritedF4): mode 1 NO-CHANGE, mode 2
  NEGATIVE (`crypto:research/lit2026_w4_early_abort_20261011/README.md`).
- Trace / Kosters-Yeo class chi(P)=Tr(x+a): homomorphism, kernel 2E, zero on <G>; the
  linear form L is in the degree-2 span for m=2 and adds exactly one bit for m=3
  (`crypto:research/trace_homomorphism_tests_20261109/README.md`, T1, T2, T3, T5).
- t=2 (m=2) Semaev systems are refuted at Macaulay degree 2 or 3 (154/200 at 2)
  (`crypto:research/semaev_t2_trace_20261102/REPORT.md`).
- ML UNSAT predictor from bits of x_R, their pairwise products and early telemetry:
  AUC within noise of 0.5; perfect-predictor ceiling 5.2-9.0x; SAT oracle refutes at
  ~1 conflict per sorted base triple (`crypto:research/pdp_ml_sat_ceiling_20261027/README.md`).
- Degree-2 Macaulay rows handed to SAT as preprocessing (implied rows, 2026-09-08)
  (`crypto:research/notes/ecc2k130/RESEARCH_KOBLITZ_SCALING_TARGET.md`, "Algebraic
  preprocessing").
- research-kb and the lead atlas are unavailable in this container; no novelty is claimed.

## Residual questions (what prior work leaves open)

- **Q-a (workload).** Exact UNSAT fraction per cell (n, m, d) on targets in <G>, and the
  share of oracle cost spent on UNSAT targets, in counted units, for the oracles whose
  per-target cost is known (rootfind: constant probes; enumerator-SAT: prior conflicts;
  XL linearisation: row operations measured here).
- **Q-b1 (cheap target filters).** Does ANY polynomial of degree <= k (k = 1, 2, 3) in the
  n bits of x(R) vanish on every decomposable target in <G> without vanishing on all of
  <G>? Such a polynomial is exactly a zero-false-negative pre-solve filter of cost
  O(C(n,<=k)) bit operations; the trace conditions are the k=1 special case. This is an
  exact, exhaustive question at toy n (prior ML work only tested statistical signal).
- **Q-b2 (partial-descent filters).** Fraction of UNSAT targets refuted by pure
  linearisation of the Weil-descent system at Macaulay degree D (D = deg of the
  equations, i.e. no multiplication, "linear-part rank test"), and at D+1, for m=2
  (S3 form) and m=3 (split S3 chain with a free intermediate u in F_2^n).
- **Q-c (net saving).** Pruning rate and net counted cost (filter included) against the
  unfiltered oracle on identical target streams.

## Instrument

Pure Python 3 (stdlib only), exact arithmetic, file `l2filters.py` in this directory.

- Fields F_2^n, n in {11, 13, 19} (prime, 2 primitive mod n, mirroring 131), fixed
  irreducible polynomials checked irreducible in code.
- Curve y^2 + xy = x^3 + a x^2 + 1, a = 0 (ECC2K-130 form) primary; a = 1 secondary at n=11,13.
- <G> = largest prime-order subgroup; universe U = x-coordinates of <G> \ {O}.
- Base F(d) = all points with x in span{1, z, ..., z^{d-1}} (repo's
  `koblitz-standard-subspace:dimension=d`), x != 0.
- Decomposable set D = {x(P_1+...+P_m) : P_i in F(d), multiset} computed exhaustively.
- Cells: m in {2, 3}; d in {ceil(n/m) - 1, ceil(n/m), ceil(n/m) + 1} (skipped when
  enumeration exceeds 4e6 sums).

## Arms

- **Baseline arm**: unfiltered oracle on the target stream (every x in U, or a seeded
  sample for Q-b2).
- **Test arm T1 (Q-b1)**: degree-<=k vanishing filter, k = 1, 2, 3: the space
  N_k = {p : deg p <= k, p|D = 0} modulo {p : p|U = 0}; its dimension
  dim_k = rank(eval on U) - rank(eval on D cap U) and its exact pruning (targets u in U\D
  with p(u) = 1 for some p in N_k).
- **Test arm T2 (Q-b2)**: linearisation refutation at Macaulay degree D0 (= equation
  degree) and D0+1, with row-XOR and word-op counts.

## Controls

- **Positive control PC1** (detector sees a filter): m = 1 (D = liftable part of V): the
  detector must find dim_1 > 0 and prune >= 1 - 2^(d-n+1) of U.
- **Positive control PC2** (genuine m >= 2 structure): universe U_all = x of all of E
  \ {O}, base restricted to F(d) cap 2E (chi = 0). Sums then lie in 2E, so the trace
  filter Tr(x) = Tr(a) must be found at k = 1 and prune exactly the x-coordinates of
  E \ 2E (with 2-torsion caveats recorded).
- **Negative control NC1 (null)**: a uniformly random subset of U of size |D cap U|
  (seeded): its dim_k is the null expectation.
- **Negative control NC2 (contrarian, required by the brief)**: T2's filter applied in
  front of an XL/linearisation oracle whose own first step is the same degree-D0
  elimination: saving must be exactly 0 and filter cost pure overhead. Also: every T1
  filter (if any) is re-evaluated on targets that survive the solver's own D0 step.
- **Soundness gate (hard)**: zero false negatives. Every filter is evaluated on every
  decomposable target in the cell (exhaustive); any rejection of a decomposable target
  voids the filter.
- **Determinism / A/A**: every cell's ranks and counts are recomputed twice from scratch
  (independent runs, same seed) and must be bit-identical; field arithmetic and group law
  self-tests (associativity, #E*P = O, order of G) must pass.

## Counted metrics

- UNSAT fraction = 1 - |D cap U| / |U| (exact).
- dim_k, pruning_k (exact counts).
- Linearisation: refuted fraction among UNSAT targets at D0 and D0+1; row XORs and
  word ops (row XOR x ceil(columns/64)) per target.
- Oracle costs per target: rootfind m=2: 1 table probe; m=3: |F| probes (each 1 group
  add + lookup) (repo `pdp3-koblitz:engine=rootfind`); enumerator-SAT: prior conflicts.
- Net saving per target = pruning x (oracle cost of a refuted target) - filter cost, with
  filter word ops priced at 0, 0.1 and 1 GAE per word op (COMMON.md / AGENTS.md §5a band).

## Pre-registered predictions

- P1: UNSAT fraction rises with n at fixed m and d = ceil(n/m) and is >= 0.6 at m = 3
  (expected ~ 1 - exp(-|F|^m / (m! |U|)) heuristic).
- P2: dim_k = 0 (no useful filter) for k = 1, 2, 3 in every cell with |D cap U| > 4 x
  number of monomials; PC1/PC2 detect their filters.
- P3: m=2 linearisation at D0 = 2 refutes a majority of UNSAT targets (consistent with the
  t=2 study); m=3 split form at D0 = 3 refutes < 10 %.
- P4: against an XL oracle (NC2) the saving is exactly 0.

## Decision rule (per test arm)

- **IMPROVES**: zero false negatives AND net saving > 0 at every price in the band
  {0, 0.1, 1} GAE per word op on at least two n values, against a baseline oracle that
  does not already perform the filter's work (i.e. NC2 does not absorb it).
- **NO-CHANGE**: zero false negatives but no pruning (dim_k = 0) or the saving is
  absorbed by the oracle's own first step (NC2).
- **NEGATIVE**: zero false negatives, pruning > 0, but net saving < 0 at every price
  including 0 (impossible by construction) or < 0 at 0.1 and 1 with no regime where the
  baseline oracle lacks the work.
- **INCONCLUSIVE**: controls fail, determinism fails, or band prices disagree in sign
  (reported as a trade with the break-even price per word op).
- **Falsifier of the whole-lane "no cheap filter" reading**: any cell with dim_k > 0 on
  U for k <= 3 beyond the null (NC1), with zero false negatives.

## Not in scope

Symmetrised S4 / Gaudry e-variable encodings (only m=2 S3 and the m=3 split chain are
built); n = 131 itself; wall time (noted only).
