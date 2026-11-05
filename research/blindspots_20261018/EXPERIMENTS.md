# Blind-spot campaign: twenty experiments (2026-10-18)

Source: the 2026-10-18 survey of the three research repositories, which named
twenty directions the program is not pursuing. This file freezes one experiment
per direction. Each item's lane turns its section into a `PROTOCOL.md` before
any measured run, may tighten but never loosen it, and records every deviation.

## Rules every item follows

- **Language.** Rust for every algorithm, harness, model and result
  (AGENTS.md "Implementation language: no Python"). External systems such as
  msolve, CryptoMiniSat or kissat are allowed only as comparison arms, with
  exact flags recorded. Shell only for thin orchestration.
- **Unit.** Counted operations (ecbench GAE, field multiplications, Macaulay
  column touches, solver conflicts/propagations). Wall time is descriptive.
  The host is a shared 4-vCPU, 15 GB container at isolation level L0.
- **Budget.** One thread per job. Address-space cap 4 GB per job
  (`prlimit --as`), 7 GB for item 7 only. At most 2 CPU-hours of measured runs
  per item, 45 minutes per job. Disk: keep build products in the shared
  `target/`; no per-item target directories; delete scratch over 200 MB.
- **Comparison.** Every candidate is measured against the incumbent component
  it would replace, and every cross-method row prices unpriced native work at
  0, 0.1 and 1 GAE per operation (AGENTS.md §5a). The gap to matched rho is
  reported as distance remaining, never as a pass/fail gate.
- **Controls.** A negative control drawn from the right null and a positive
  control proving the instrument can see the effect (controlled-experiment
  skill).
- **Outputs.** `research/blindspots_20261018/<NN>-<slug>/` holds
  `PROTOCOL.md`, `card.yaml`, `raw/`, `README.md` (verdict by the frozen rule),
  an SVG and PNG graph, and `REPORT.pdf` (research-visuals skill). Speed results
  also go to `benchmarks/20261018-blindspots-<slug>/`; the integration step
  adds the `benchmarks/README.md` rows and regenerates the leaderboard.
- **Verdict vocabulary.** ADVANCES, TRADES, MATCHES, REGRESSES, INCONCLUSIVE,
  BLOCKED, each stating its level (exponent, constant, primitive). A blocked
  item says exactly what blocked it and what would unblock it.

---

## A. Target choice and mathematics

### 01 · Gaudry k = 2 index calculus on E(F_{p²}) against rho (FourQ / GLS class)

- **Question.** On random ordinary curves over F_{p²}, how far is the best
  k = 2 index calculus from matched rho, and does the gap shrink with p?
- **Why.** Gaudry's k = 2 attack is Õ(p) like rho's Õ(p) = Õ(√#E); this is
  the only deployed family in the tie regime.
- **Arms.** (a) Plain Gaudry–Diem k = 2: base F = {P : x(P) ∈ F_p}, S₃ Weil
  restricted to two equations over F_p, root finding, sparse LA.
  (b) (a) + 2-torsion (FGHR) symmetrisation where E has a rational 2-torsion
  point. (c) (b) + single and double large primes. (d) Matched rho with the
  negation map; for GLS curves also rho folded by the order-4 GLS
  endomorphism, and IC folded the same way (arm e).
- **Code.** `src/cryptanalysis/diem_descent.rs` (k = 2 toy),
  `src/cryptanalysis/gls_fp2.rs`, `examples/glv_gaudry_bench.rs`, ecbench.
- **Cells.** p prime with log₂ #E ∈ {24, 28, 32, 36, 40}; 6 random curves of
  prime order (or cofactor ≤ 4) per size plus 3 GLS curves per size; one
  target per curve. Positive control: arm (a) recovers the planted log on every
  instance. Negative control: arm (a) on a shuffled base of equal size.
- **Metric.** S = cost / √r in GAE per arm; fitted exponent α in cost ∝ r^α
  with bootstrap interval; gap to rho per size; extrapolated gap at
  p = 2¹²⁷ − 1 with the interval propagated.
- **Rule.** ADVANCES (exponent) if any IC arm's α interval lies below rho's;
  MATCHES if intervals overlap; report the extrapolated FourQ gap either way.

### 02 · Pricing binary curves at deployed sizes under the measured degree model

- **Question.** Under each measured solving-degree model, at which extension
  degree n does the m = 4 (and m = 3, 5) Weil-descent route cross rho?
- **Why.** The "constant F4 degree 4" reading puts n = 131 at +7.6 bits; the
  cost scales like 2^{n/m} against 2^{n/2}, so the sign can flip at the
  deployed sect233–sect571 sizes, which are never priced.
- **Arms.** Degree models: D = 3, D = 4 (F4, measured to n ≤ 15), D = 6
  (XL), "levelled at 7", D = ℓ + 4. Arities m ∈ {3, 4, 5}; weights w ∈ {1, 2}.
- **Code.** Port `research/ic_next10_20261014/06-equivariant-f4-m4/cost_d4.py`
  into `examples/descent_degree_cost.rs` (it already reproduces the D = 6
  rows), add an n sweep and the column-touch-per-group-op constant K as a
  parameter with K ∈ {2⁰, 2⁴, 2⁸}.
- **Cells.** n ∈ {131, 163, 233, 283, 409, 571} with each curve's real
  cofactor; best ℓ per cell; memory column alongside time.
- **Positive control.** The ported tool reproduces the committed 2^76.31 and
  2^88.05 rows bit for bit. Negative control: m = 2 must never cross rho.
- **Metric.** log₂(total) − log₂(rho) per (model, m, n); crossover n per model.
- **Rule.** A model "flips" if its gap is ≤ 0 at some deployed n. The output
  decides whether item 07's cells are worth the machine: if no plausible model
  flips by n = 571, say so.

### 03 · Polynomial-method solving (Dinur 2021; Lokshtanov et al. 2017) on descended Semaev systems

- **Question.** What exponent c does the polynomial method achieve on the
  Boolean Weil-descended S₃/S₄ systems, against FES and F4 on the same inputs?
- **Arms.** (a) Dinur's multiple-parity-counting algorithm, native Rust,
  (b) Lokshtanov–Paturi–Tamaki–Williams–Yu probabilistic-polynomial
  algorithm at small parameters, (c) incumbent FES (`mq_fes.rs`),
  (d) incumbent F4 (`koblitz_groebner.rs`), (e) msolve if buildable.
- **Cells.** Descended systems from the existing generators at
  (n, ℓ, m) with N = m·ℓ unknowns ∈ {12, 16, 20, 24, 28, 32}, 8 targets each;
  plus random dense quadratic systems of equal N and equation count as the
  positive control (published exponents must be reproduced within the band).
- **Metric.** Bit operations per instance; fitted c in 2^{cN}; ratio to FES.
- **Rule.** Needed for an IC win: c < 1/2 − 1/m relative to the decomposition
  regime. ADVANCES (constant) if (a) or (b) beats FES at the largest N with
  intervals separated; the exponent statement is reported whatever the verdict.
  Expected negative; a clean negative closes the paradigm.

### 04 · Presentation search for the m-sum variety in the binary descent

- **Question.** Does the descended solving degree D depend on the presentation
  of the summation variety, at fixed base and target?
- **Arms.** (a) Semaev S_{m+1} in x (incumbent), (b) unprojected chain with
  y-coordinates and the affine addition law, (c) binary Edwards
  (Bernstein–Lange–Rezaeian Farashahi) with its own summation polynomial,
  (d) Möbius frame u = 1/(x+1) with the 2-torsion symmetrisation, (e) the
  chained S₃ with intermediates (Semaev 2015) as reference.
- **Cells.** (n, ℓ, m) ∈ {(17, 4, 3), (19, 4, 3), (13, 3, 4), (15, 3, 4),
  (17, 4, 4)}; 8 targets per cell; same F₂-subspace base across arms.
- **Metric.** Solving degree D, max Macaulay matrix size, column touches;
  solutions cross-checked equal across arms (positive control: every arm
  recovers a planted decomposition).
- **Rule.** ADVANCES (primitive) if any arm lowers D by ≥ 1 on ≥ 4/5 cells
  with identical solution sets; MATCHES otherwise.

### 05 · Monomial, variable and block orders tuned to the descent structure

- **Question.** Does a structure-aware order lower D or Macaulay size on the
  descended systems relative to grevlex?
- **Arms.** grevlex (incumbent); block order leaves ≻ intermediates;
  intermediates ≻ leaves; weighted degree with Frobenius-conjugate weights;
  variable orders by (i) min-degree heuristic, (ii) treewidth elimination order
  from `descent_treewidth.rs`, (iii) random (negative control, 8 draws).
- **Cells.** Same cells as item 04 arm (a), plus (n, ℓ, m) = (31, 15, 2).
- **Metric.** D, largest matrix rows × columns, column touches, time.
- **Rule.** ADVANCES (constant) if an arm cuts column touches ≥ 1.5× on ≥ 4/6
  cells with the random-order band excluded; ADVANCES (primitive) if D drops.

### 06 · Signature-based Gröbner engine (F5/GVW) with Joux–Vitse trace replay

- **Question.** Does a signature-based Boolean engine eliminate the reductions
  to zero that dominate the degree-4 steps, and does trace replay amortise
  symbolic preprocessing across same-shape targets?
- **Arms.** (a) incumbent inherited F4, (b) new signature-based engine
  (GVW or F5 with rewritten-criterion, Boolean field equations), (c) (a) with
  recorded-trace replay (F4Remake), (d) (b) with replay, (e) msolve.
- **Cells.** m = 3 and m = 4 cells reaching degree 4: (13, 3, 4), (15, 3, 4),
  (15, 5, 3), (17, 4, 3); 16 targets per cell, trace recorded on a held-out
  target.
- **Metric.** Reductions to zero, column touches, peak memory per solve;
  basis equality across arms as positive control.
- **Rule.** ADVANCES (constant) if (b) or (d) beats (a) by ≥ 1.5× column
  touches on ≥ 3/4 cells with identical bases; the replay ratio is reported
  separately.

## B. AI and HPC levers

### 07 · Out-of-core / blocked Boolean F4 to measure the deciding degree cells

- **Question.** What is the solving degree at (m, ℓ) = (4, 5) and (4, 6), and
  at (3, 6), using a memory-blocked Macaulay elimination?
- **Why.** The binary verdict rests on extrapolating D from n ≤ 15, ℓ ≤ 5;
  the next cells died at 18–50 GB.
- **Arms.** (a) Blocked F4 elimination: the degree-d Macaulay matrix streamed
  in column panels from disk (M4RI-style panel reduction, row blocks memory
  mapped), single node; (b) the G2 equivariant pass where the system has the
  Frobenius symmetry; (c) msolve as reference where it fits.
- **Cells.** Ladder (11, 4, 4) → (13, 4, 4) → (15, 5, 4) → (17, 5, 4) →
  (19, 6, 4) and (19, 6, 3); stop at the first cell that exceeds the job cap.
- **Positive control.** (a) reproduces the committed degree and basis at
  (11, 4, 4).
- **Metric.** Solving degree D, rows × columns per degree, bytes streamed,
  peak RSS.
- **Rule.** Record D per cell. If D stays at 4 at ℓ = 5 and 6, item 02's
  D = 4 rows become measured to that ℓ; if D ≥ 5 the growth model returns.
  BLOCKED cells name the projected memory and disk needed.

### 08 · Learned branching for WDSat / XOR-CDCL on Semaev CNFs

- **Question.** Can a learned variable-selection policy cut WDSat or CDCL work
  on m = 3 Semaev instances relative to the static vertex-cover order?
- **Arms.** (a) WDSat with its static order (incumbent, `wdsat_oracle.rs`),
  (b) WDSat with a learned scorer: a Rust gradient-boosted tree or linear model
  over per-variable features (occurrence in XOR vs ANF clauses, degree,
  Frobenius orbit index, current propagation counts), trained on solved
  instances at n ≤ 29, (c) learned cube selection for cube-and-conquer,
  (d) random order (negative control), (e) CryptoMiniSat with XOR as external.
- **Cells.** Train n ∈ {23, 25, 27, 29}; test held-out targets at n ∈ {31,
  33, 35, 37}; 32 targets per test size, same factor bases.
- **Metric.** Decisions + propagations per instance, solved within cap.
- **Rule.** ADVANCES (constant) if (b) or (c) beats (a) on held-out sizes by
  ≥ 1.3× median with the paired bootstrap interval excluding 1.

### 09 · Learned F4 selection and pivot policy, with per-shape trace transfer

- **Question.** Does a learned critical-pair-degree selection or pivot policy
  in Boolean F4 cut column touches on Semaev systems, and does a trace
  recorded on one target transfer to others of the same shape?
- **Arms.** (a) incumbent F4 selection (normal strategy, all minimal-degree
  pairs), (b) learned subset selection: a Rust policy (linear or small MLP,
  trained by imitation from an oracle that knows which pairs produced new
  leading terms on recorded runs), (c) pivot-order policy for the M4RI
  elimination, (d) trace transfer: replay target A's selected rows on target B.
- **Cells.** (17, 4, 3), (19, 4, 3), (21, 5, 3), (13, 3, 4); train on 32
  targets, test on 32 held-out per cell.
- **Metric.** Column touches and rows built per solve; basis equality.
- **Rule.** ADVANCES (constant) if (b), (c) or (d) beats (a) by ≥ 1.3× on
  held-out targets with identical bases on ≥ 3/4 cells.

### 10 · Program search over the decomposition kernel, fitness = counted oracle cost

- **Question.** Can an automated propose-evaluate loop find a decomposition
  kernel or factor-base predicate with lower counted oracle cost than the
  incumbent at fixed yield?
- **Arms.** (a) incumbent kernel (`semaev_decomp.rs` pair-and-root S₄ over
  binary subspaces), (b) search: a Rust mutation engine over a small
  kernel DSL (filter order, early-abort thresholds, table layout, root-finder
  choice, base predicate among subspace / Hamming / cyclotomic families)
  evaluated by ecbench counted cost on training targets, (c) random search of
  equal evaluation budget (negative control).
- **Cells.** n ∈ {31, 37, 41} Koblitz; 256 evaluation budget; held-out
  64 targets per n.
- **Metric.** Counted oracle cost per relation at equal relation count;
  yield reported to confirm conservation (KN-FIND-007).
- **Rule.** ADVANCES (constant) if the best (b) program beats (a) on held-out
  targets with interval excluding 1 and beats (c)'s best.

### 11 · Learned necessary-condition prefilter on x_R for subspace bases

- **Question.** Is there a cheap function of x_R that predicts "no
  m-decomposition over this base" better than the trace-zero baseline?
- **Arms.** (a) no filter, (b) trace filter where it applies, (c) learned
  classifier (Rust logistic regression and gradient-boosted trees) over
  features: ONB/polynomial-basis bits, absolute traces of x_R^k and 1/x_R^k
  for small k, Hilbert 90 indicators, membership of x_R in shifted subspaces,
  half-trace bits, (d) label-permutation null, (e) positive control: a planted
  label (Tr(x_R) parity) that the classifier must recover.
- **Cells.** n ∈ {17, 19, 23, 29}, m ∈ {2, 3}, ℓ at the transition size;
  20,000 labelled targets per cell from the exact decomposer, class-split by
  base.
- **Metric.** AUC and refutation rate at 0 false refutations on held-out
  targets; filter cost in field operations.
- **Rule.** ADVANCES (primitive) if (c) refutes ≥ 5 points more than (b) at
  zero false refutations with the null band excluded; any such feature is
  then proven algebraically before use.

### 12 · Surrogate for the solving-degree surface

- **Question.** Can a model fitted on all measured toy cells predict D and
  Macaulay size for unmeasured cells well enough to rank experiments?
- **Data.** Every committed degree and matrix-size measurement in
  `research/dreg_*`, `research/ic_next10_20261014/06-*`, `research/f6_*`,
  `research/ffd_*`, `research/fall_degree_bounds*`, and items 04, 05, 07 if
  finished first.
- **Arms.** (a) semi-regular (Fröberg) prediction, (b) "levelled" constant
  model, (c) Rust gradient-boosted regressor with split conformal intervals,
  (d) leave-one-family-out validation.
- **Metric.** Interval coverage at 90 %, mean absolute degree error, rank
  correlation on held-out cells.
- **Rule.** Useful if (c) beats (a) and (b) on held-out error with coverage in
  [0.85, 0.95]; output a ranked list of the next five cells to buy.

### 13 · Rho reference: the negation-map shortfall (H-RHO-bc6ea6) and an ECC2K-130 completion plan

- **Question A.** Over prime fields, does the negation map reach √2 once
  fruitless-cycle escape is charged, and where does it cross 1.0?
- **Arms A.** Teske r-adding baseline; negation walk with escape rules
  {doubling escape, Bos–Kleinjung–Lenstra min-of-2, look-ahead, periodic
  detection with period ∈ {16, 32, 64}}; r ∈ {16, 32, 128, 2048}.
- **Cells A.** log₂ r ∈ {40, 44, 48}; 64 targets per cell; field
  multiplications counted.
- **Rule A.** Report the net gain per rule with intervals; support the
  hypothesis if no rule reaches 1.35×; locate the crossing if one exists.
- **Question B.** What is the full cost of completing ECC2K-130 with the
  repository's best iteration function? Measure the CPU per-iteration rate of
  the current `ecc2k130/` client natively, combine with the committed GPU rate,
  and write a costed plan (GPU-hours, distinguished-point storage, cost at
  current cloud prices). No claim beyond the measured rate.

## C. Known, recommended, still undone

### 14 · First prime-field arity m = 4 cost cells

- **Question.** What does one m = 4 (S₅) decomposition attempt cost at a
  small-x base, with each available oracle, and where does the measured cost
  put the m = 4 route against rho?
- **Arms.** (a) enumeration / MITM over a triple table (generic reference),
  (b) F4 on S₅ restricted by the base membership polynomial, (c) the PKM tower
  oracle where it runs, (d) Coppersmith lattice at the measured reach.
- **Cells.** log₂ p ∈ {16, 20, 24, 28}; B at the §2.4 optimum (p·T_dec)^{1/5};
  16 targets per cell; full IC to completion at log₂ p ≤ 20.
- **Metric.** T_dec per oracle, total S, gap to rho.
- **Rule.** Record the first measured m = 4 cells; ADVANCES if any oracle's
  T_dec grows slower than p^{1/4}.

### 15 · Genus-3 linear-algebra exponent (replaces "D₄ + double large primes", done in ic_improve rounds 1–2)

- **Question.** Can the linear-algebra phase of genus-3 double-large-prime IC
  (now 65 % of cost, ≈ q^1.52) be brought toward the q^{4/3} relation phase?
- **Arms.** (a) incumbent `lpsgebw2` (structured Gaussian elimination + block
  Wiedemann), (b) block Wiedemann with larger blocking (4,4), (c) block
  Lanczos, (d) filtering with merge-weight limits tuned per q, (e) relation
  oversampling with clique removal.
- **Cells.** q ∈ 2^{16..22}; 4 curves per size; same relation sets across arms.
- **Metric.** LA operations and fitted exponent; whole-solve S.
- **Rule.** ADVANCES (exponent) if an arm's LA exponent interval lies below
  (a)'s; ADVANCES (constant) at ≥ 1.2× whole-solve.

### 16 · Size-gated equivariant oracle (replaces "wire the equivariant pass", measured TRADES in ic_improve round 1)

- **Question.** The equivariant pass loses at n = 5, 7 and wins 5.1×–15.5×
  above; does a size-gated dispatcher advance the production oracle?
- **Arms.** (a) `solve_boolean_system` (incumbent), (b) equivariant pass
  always, (c) gate: equivariant when the barrel system has ≥ N₀ unknowns,
  N₀ tuned on training sizes.
- **Cells.** Koblitz n ∈ {41, 47, 53, 61} production IC; 32 held-out targets.
- **Rule.** ADVANCES (constant) if (c) beats (a) whole-solve with interval
  excluding 1 and never regresses a size by more than its interval.

### 17 · Cyclotomic base: cost arm at n = 83 (replaces "run A2", yield-neutral in cyclotomic_falsifier_20261015)

- **Question.** The cyclotomic base is yield-neutral; does its algebraic
  membership test lower the decomposition oracle's solving degree or cost at
  the n = 83 rung?
- **Arms.** (a) Hamming-weight ONB base, (b) compact-orbit base (incumbent),
  (c) cyclotomic γ-weight-w base with its degree-4 membership system.
- **Cells.** n ∈ {23, 47, 83}; matched column counts; 16 targets.
- **Rule.** ADVANCES (constant) if (c)'s counted oracle cost beats (b) with the
  interval excluding 1; otherwise record A2 closed.

### 18 · Execute the frozen ECDLP contracts in crypto-autoresearcher

- **Scope.** List the frozen ECDLP contracts with zero run directories
  (REVIEW-UNADDRESSED-20260807 §3), classify each as runnable on this host
  within 45 minutes and 4 GB, and run the runnable ones exactly as frozen
  through the `run` skill's contract. Write run records only; the Coordinator
  alone changes status. Output: a table of contract, runnable, run id, exit,
  and the reason for each one not run.

### 19 · Negative-results paper and Lean statements

- **Scope.** Draft `papers/ic-negative-results/` (LaTeX, IACR format) stating
  with proofs or exact evidence links: compact-orbit Koblitz IC is generic and
  sits on or above S·T² ≳ r/|Aut|; isogeny oracles add nothing in the generic
  model (fibers note Thm 15.1); mean decomposition yield is conserved under
  base geometry; the Semaev cover monodromy is C₂^{m−2}; Nagao's Lemma 4 is
  false; Semaev 2015 Assumption 1 fails off the diagonal. Formalise Thm 15.1's
  combinatorial core and the yield-conservation identity as Lean statements
  with `sorry`-free proofs where feasible, else stated targets marked open.
  Every number links to its evidence file. No submission (iacr-eprint stops
  before submit).

### 20 · Classical pre- and post-processing for quantum ECDLP

- **Question.** How much does the classical side change the quantum cost of
  ECDLP on P-256 and secp256k1: Ekerå–Håstad short-log variants, the
  classical post-processing lattice for partial measurement success, and
  windowed arithmetic tables?
- **Arms.** Extend `quantum_estimator.rs` with (a) Ekerå's one-run
  post-processing success model, (b) tolerance to failed runs, (c) windowed
  scalar arithmetic; measure the classical post-processing cost natively by
  simulating the lattice step on toy group sizes.
- **Rule.** Report logical-qubit × Toffoli changes against the committed
  baseline estimate; no claim about hardware timelines.
