# Semaev decomposition parameter sweeper: assessment and plan (2026-10-24)

Status: **proposed**. No code is written and no measurement is made by this
note. Every statement below is labelled *measured* (from a committed record),
*derived* (computed from recorded values) or *proposed*. Sources are named by
path and branch; where a source is not on `origin/main` that is said.

## 0. Decisions fixed by the owner (2026-10-24, in chat)

1. The sweeper is written in **Rust** (no Python driver, no Optuna).
2. Curves are **generated**, and the sweep covers every family: prime
   j = 0, prime j = 1728, generic prime, Koblitz, and binary.
3. The primary target is the **a = 0, n = 83 Koblitz curve** (81-bit prime
   subgroup). The a = 1, n = 83 curve (52.9-bit subgroup) is the control that
   can run end to end.

## 1. Goal

Search the space of summation-polynomial (Semaev) point decompositions for
m = 2…8 summands at group order about 2^83, over factor bases, large-prime
variants, symmetry reductions and degree-of-regularity reductions, with an
Optuna-style sequential optimiser: a typed search space, a trial store,
pruning, and one scalar objective.

## 2. What exists (measured / read from source)

### 2.1 Harness

- `ic bench` runs `factor base × oracle × targets × solver × linalg` through
  `run_pipeline` (`src/cryptanalysis/ic_framework/mod.rs:311`) and reports
  `S = total GAE / √r`, `verified`, and the ratio to a matched rho
  (`docs/ic/FRAMEWORK.md` §2–§4). `ic bench --sweep FILE.json` expands a
  Cartesian matrix (`SweepFile`/`expand_matrix`, `src/bin/ic/bench.rs:715-746`);
  a configuration that errors is recorded in `configurations_skipped`.
  Shipped matrices: `docs/ic/sweeps/*.json`.
- The sweep is **serial, un-resumable, and writes its report once at the end**.
  There is no Optuna, TPE, Bayesian or pruning code anywhere in the repository
  (grep over `optuna|tpe|bayesian|hyperopt|hyperband` finds nothing relevant).
- Plug-ins: 12+ factor bases (`prime-abscissa`, `binary-subspace`,
  `koblitz-orbit`, `compact-orbit-scan` with `norm=`/`mult=`, `glv-orbit`,
  `cm-multiplier`, `binary-multiplier`, `prime-kummer`, `prime-isogeny`, …);
  oracles `subtract`, `mitm`, `mitm-frobenius` (with `large_primes={0,1,2}`),
  `descent-algebraic`, `symmetrised`, `rootfind-*`; solvers `f4-f2`,
  `matrix-f4`, `matrix-f5`, `inherited-f4`, `crossbred-f2`, `fes-f2`, `xl-f2`,
  `sat-cdcl`, `exhaustive`. ecbench adds `pdp3-koblitz`, `pdp-split` (chained
  S3, plans of the form [2,1,…,1]) and `sixsum-extract`.
- Prior sweeps: `examples/fb_hyperparam_sweep.rs` (n = 8…16, **m = 2 only**,
  wall-time cost model, 208 cells) and the two branch efforts in §2.4.
- Accounting an objective must obey (`AGENTS.md` §2, §3, §5a, §8a):
  S in group-addition equivalents with every phase inside it; a row without a
  verified answer is not a result; unpriced native work priced at 0, 0.1 and
  1 GAE/op with arms ranked only where the order holds across the band;
  relabelling and accounting changes are not improvements; any claim meant to
  transfer to ECC2K-130 needs the n = 83 K_0 gate (`AGENTS.md` §8a).

### 2.2 Where the framework stops (read from source)

| limit | where | consequence |
|---|---|---|
| binary field `Gf2`, n ≤ 63 | `semaev_decomp.rs:231`; `koblitz_instance` does `group_order.to_u64()?` | `ic bench` cannot build n = 83 |
| prime field `Fp64`, p < 2^62 | `prime_fast.rs:60` | no 83-bit prime curve through the framework |
| m ≥ 4 oracle | `decompose_mitm` returns `None` for m ≥ 4 (`ic_boundary.rs:5350`) | m ≥ 4 exists only as standalone diagnostics |
| algebraic descent | `max_n_prime(m)` = ⌊64/m⌋ for m = 2, 3, else 0; `MAX_VARS = 64` | descent oracles are m = 2, 3 only |
| m = 7, 8 | no implementation found | only the chained-S3 SAT circuit reaches them |
| wide pieces | `Gf2_128`, `koblitz_wide.rs`, `wide_groebner.rs` (≤ 128 unknowns), `wide_sixsum.rs`, `binary_semaev_chain_sat.rs` (MAX_VARS 512) | exist, are **not plugged into the framework** |

### 2.3 Large primes, symmetries, regularity (technique inventory)

- **Large primes.** Five code paths: `large_prime.rs` (Koblitz m = 3 pair-table
  collector; oracle `mitm-frobenius:large_primes=…`), `ecbench_large_prime.rs`
  / `ic large-prime`, `gaudry_cubic.rs` (`LargePrimeEliminator`),
  `large_prime_filter.rs` (graph cycles / 2LP; **no callers outside its own
  tests**), `hyperelliptic_index_calculus.rs`. Measured at n = 41
  (`research/ic_techniques_20261012/double_large_prime/`): best cell 2LP,
  |D| = 10|F|, 2.21·10⁶ GAE, **17.7× counted signed-Frobenius rho**, but only
  **2.1×** better than an equal-envelope control; single LP is 1.07–1.98×
  *worse* than that control. At n = 23, 31 every LP cell is 2.2–750× worse than
  plain.
- **Symmetry reduction.** Negation (everywhere); Frobenius/Koblitz orbit fold
  (pair table 122× fewer entries at n = 61); FGHR 2-torsion translation
  (`fghr_full.rs`: degree 64 → 16, S ÷ 27); D₄-symmetrised S5
  (`gaudry_quartic_d4.rs`); ψ and τ_T folds. Refuted: u-frame symmetrisation
  (worse at n = 31), Z/3 grading (12–70× worse), isogeny-transported bases.
- **Degree of regularity.** Weil descent (`pq_descent_symbolic.rs`), matrix
  F4/F5, inherited F4, crossbred (crosses fes-f2 at ~26 unknowns, fails at 30
  with default parameters), FES, hybrid guess + Macaulay (optimum is
  exhaustive), degree falls (D\* 4 → 2 to N = 16, then dies), Shantz–Teske δ
  (δ = +1…+2 lowers D\* by ~1 at n = 8, 9). Not found in the repo:
  Fourier–Motzkin / sparse-basis code.
- **Falsified (do not spend sweep budget):** 4/5/6-sum extraction on
  compact-orbit (`S·T² ≳ r/2n` bound), first-fall-degree measurement,
  m = 2 factor-base shape search (trace-class artefact), cyclotomic bases
  (yield-neutral), isogeny/volcano/conductor routes for Koblitz, hybrid
  slicing, single large prime with bounded D, Z/3 grading, Veronese PDP,
  merge-level caps on the F_{p³} 2LP.

### 2.4 Two prior efforts that are **not on `origin/main`** (verified this session)

Both are on remote branches only; a plan that assumes they are merged is wrong.

**N83 summand sweep, m = 2…9** (`origin/claude/sweep-n83-varied-algorithms-rovr0x`,
tip `45208c41f2`; `research/n83_msweep_20261024/`, preregistered at
`b654e95b6`). *Measured*, on the **a = 1** curve (53-bit subgroup), 4-vCPU
cloud container, wall time:

- Every Weil-descent engine (F4/F5/F6/crossbred/SAT/FES) **silently returned
  "proved UNSAT" on planted n = 83 relations** because `FieldStructure`
  truncated the field to one 64-bit word; the descent builders now refuse an
  inexact field. A sweeper must treat *refused* as invalid, never as a score.
- Chained-S3 full-width SAT is the only decomposer that runs at n = 83 for
  every m, and within a 180 s cap decides only **m = 2** (17.3 s per public
  query; derived projection 2^75.9 s = 2^75.3 × rho). All m = 3…9 cells and
  every orbit-domain cell hit the cap.
- The m = 4 root-table rootfinder (complete DLP, K = 1107): 188.5 s (root11),
  175.2 s (root22) against strong-rho 1.509 s: **116–125×** from rho.
- Companion n = 61 (exact one-word field), m = 2: native-XOR SAT fastest at
  D ≥ 12 (0.79–2.41 ms/query); inherited F4 fastest Gröbner engine; matrix F5
  0.78–1.05× of F4; **m ≥ 3 refused by the 64-variable descent mask.**

**Algebraic IC ladder** (`origin/claude/ic-m2-linear-ladder-20261024`, tip
`96e016772e`; `research/ic_m2_linear_ladder_20261024/`). *Measured*: the
linear Weil-descent of S3 over `V = span{1..z^{d-1}}` solved 52/52 one-target
DLPs on seven rungs n = 13…41; work slope **0.647 [0.639, 0.655]** log₂
trials per bit of n against **1.079** for the F4 arm; n = 41 takes ≈ 8 min and
is **36,440×** strong rho in wall time, widening with n. Its report records an
**owner criterion dated 2026-10-24: an index-calculus pipeline counts only
when its relations come from solving summation-polynomial systems over an
algebraically defined factor base, without table collisions** (table
pipelines — PDP4root, `engine=rootfind`, "aimed m = 3", pair enumeration — are
excluded). That sentence is not yet in `AGENTS.md` on `origin/main` (grep
found nothing); §6 turns it into an explicit decision.

![distance to rho](distance_to_rho.svg)

Graph data and source: `graph_data.json`, `plot.py`. Recorded values only.

## 3. Curves

Koblitz curves are `y² + xy = x³ + a x² + 1`. Frobenius has order n and acts
as λ on the prime subgroup; the signed orbit has size 2n.

| id | family | n | log₂ r | cofactor | status |
|---|---|--:|--:|--:|---|
| a = 0, modulus `z^83+z^45+z^2+z+1` (`known_n83_k0()`; `icv1-f2m83-tm6151469093347-debefd74`) | Koblitz | 83 | **81.0** | 4 | **primary**; registered in `docs/curves/registry.json` (order 9671406556923184866742756) but with **no generator recorded** (`ec1_unresolved`: `KoblitzCurve` covers n ≤ 63); rho only (~2^37.5 steps) |
| a = 1, n = 83 (`icv1-f2m83-t6151469093347-9eba6cd5`) | Koblitz | 83 | 52.9 | 1,128,547,018 | **control**; IC runs end to end |
| a = 0, n = 97 (`icv1-f2m97-t378251973071909-f27192f3`) | Koblitz | 97 | 95.0 | 4 | registered, no runs |
| a = 1 n = 79 / 85 / 89 / 97 | Koblitz | — | 69.7 / 64.5 / 64.7 / 87.4 | 634 / 1.4·10⁶ / 2.1·10⁷ / 778 | computed in audit, unregistered, no runs |
| prime ≈ 83 bits | any | — | — | — | **none exists**; nearest `fp-j0-b80` has a 32-bit subgroup |

Two curve names for degree 83 must never be merged by degree
(`research/n83_evaluation_20261012/verification/CURVE_ID_ERRATUM.md`); the
repository's curve-name lint (`AGENTS.md` §11, `scripts/check_curve_names.py`)
applies to everything the sweeper emits.

**Generation constraints (derived; to be confirmed as each generator lands).**

- *Binary j = 0 / 1728.* In characteristic 2, 1728 ≡ 0, so j = 0 = 1728 is a
  single class and is **supersingular** (embedding degree ≤ 4). It is not an
  ECDLP target. It is included only as a labelled MOV/Frobenius control, not
  as a sweep objective.
- *Prime j = 1728* (`y² = x³ + ax`) always has the point (0,0) of order 2, so
  prime order is impossible: target cofactor 2 (or 4). *Prime j = 0*
  (`y² = x³ + b`, `4p = t² + 3v²`, six twists) can reach prime order.
- *Random binary* curves always have a 2-torsion point (b ≠ 0): the minimum
  cofactor is 2. Point counting over F_{2^83} needs Satoh/AGM/SEA-type code;
  whether the repository has it (candidates: `ai_schoof.rs`,
  `binary_velu.rs`) is **unverified**.
- *Prime ≈ 83-bit* needs a wide prime field: `Fp64` stops at 62 bits. The
  Montgomery code in `certicom131-native-*` is a candidate to reuse.
- Koblitz counts are cheap (trace recurrence) and already done for the table
  above.

## 4. Objective

At n = 83 no end-to-end `S` exists (a = 0: no relation, rank or DLP solve;
a = 1: only the root-table pipeline completes). So the objective cannot be an
`S` measured at n = 83.

- **Primary scalar: projected S at the target size.** Fit log₂ S against n on
  a ladder of at least four rungs (`AGENTS.md` §5 requires ≥ 4 sizes), and
  optimise the fitted value extrapolated to n = 83 — for the a = 0 curve,
  with its own rungs (a = 0, n = 13, 19, 23, 41, … primes with #E = 4·prime) —
  plus the n = 83 diagnostics measured directly (decided fraction, seconds per
  query, capacity). *Derived, not measured*; reported as such.
- **Scale-relative knobs.** Optimising S at one toy size selects
  configurations that only work at that size, because the base size scales
  like r^{1/3}. Every size-like knob is expressed relative to scale: base size
  as a multiple of the family optimum `(#E·t/k)^{1/3}`, `lp_domain_factor` as
  a ratio, descent dimension d as d/n.
- **Band-robust scalar.** Score a trial by the worst case of
  `S_trial / S_baseline` over unpriced-work weights w ∈ {0, 0.1, 1} (§5a), so a
  winner holds across the band by construction. The baseline is named and
  frozen before the search starts.
- **Two tracks, both reported, one counted.** Track A (algebraic: relations
  from solving summation-polynomial systems, no table collisions) is the
  counted track if the owner criterion in §2.4 governs this work. Track B
  (table / rootfinder / pair table / `mitm-frobenius` large primes) is run as
  the reference and context track and is never ranked against Track A as an
  improvement. This split is decision D1 in §6.
- **Determinism.** Budgets are operation or trial counts, never
  `solver_budget_seconds > 0` (non-deterministic). Counts stay single-threaded;
  parallelism is across trials, one process each. Only 25 instances have pinned
  calibration (`docs/ic/calibration.json`); elsewhere a solver row is
  `measured`, comparable only on the same host, and a `S` is a lower bound.
- **Failure classes.** Every trial ends in `ok` (verified), `exhausted`
  (budget hit, count recorded), `refused` (the model cannot represent the
  instance, e.g. inexact field, 64-variable mask), or `invalid` (verification
  failed). Only `ok` is scored. `refused` and `invalid` are logged and never
  silently dropped or treated as "no solution" (the n = 83 UNSAT bug in §2.4).

## 5. Plan

**P0 — Setup and hygiene.**
Branch from `origin/main` (`ec77beacc9`). The two efforts in §2.4 are on
unmerged branches: build on a branch that can later rebase onto them, and
re-read their cells before running anything so nothing is repeated
(m = 2…9 on a = 1 n = 83; the m = 2 linear ladder n = 13…41). New cargo target
directory, not an existing `cargo-target-ic-*`/`-sixsum-s4`. Avoid files held by
live worktrees: `ghs_generalized.rs`, `ghs_quadratic.rs`,
`binary_ecc/operation_tally.rs`, `f2m.rs`, `binary_ecc/mod.rs`,
`cryptanalysis/mod.rs`. Register a Conductor task first (the control plane was
unreachable from this session, §7).

**P1 — Trial interface (Rust).** A subcommand `ic sweep-trial` taking one
configuration as JSON and printing one JSON record: failure class, `s` over the
band, `verified`, per-stage counts, native counters, solver `priced_by`.
Built on `run_pipeline`, so the objective is defined in one place and `ic bench`
stays unchanged. Includes the §4 failure classes and an operation-count budget.

**P2 — Search space.** One typed schema, conditional where a parameter exists
only for some choices:

| group | dimensions |
|---|---|
| instance | family ∈ {prime-j0, prime-j1728, prime-generic, koblitz-a0, koblitz-a1, binary}; n or log₂ r |
| arity | m ∈ {2…8}; split plan ([2,1,…,1] chain, [2,2], [3], [2,1]) |
| factor base | kind (§2.1); size relative to family optimum; `norm=`, `mult=`, `divisor=i;j`, subspace dimension d/n, basis (polynomial / orbit / coordinate / random) |
| symmetry | negation, Frobenius-orbit fold, FGHR torsion invariants, GLV/GLS/CM group, D₄ / ψ folds |
| large primes | none / single / double; domain ratio; partial-relation cap; merge depth |
| solver | engine; D, k (crossbred); max_degree, split, node budget (matrix/inherited F4); XOR encoding (SAT) |
| linear algebra | incremental, structured, sparse |

Known-dead regions (§2.3 falsified list) are excluded and cited in the schema.

**P3 — Optimiser (in-tree Rust, no new dependency).** TPE over a conditional
categorical/log-integer space is small enough to write directly. `rand` and
`rayon` are already in `Cargo.lock`; the trial store is an append-only JSONL
file (resumable, mergeable across hosts, no SQLite dependency). Pruning:
successive halving on the rung ladder — evaluate at the two smallest rungs
first, promote only trials whose early S/S_baseline is within a margin of the
running median. Parallel workers are separate OS processes. Deterministic given
a seed. The optimiser is a library module with tests; it does not touch the
framework traits.

**P4 — Calibration tier (n ≤ 41), where S is measured.** Run m = 2 and 3
first, over every family that exists. Extend `fb_hyperparam_sweep` to m = 3
(its report names this as the next arm). Fit exponents over ≥ 4 rungs against a
frozen baseline per family. Replay the ladder's seven rungs as a regression
test of the trial interface. Expected outcome (*derived from prior fits*): the
sweep finds constants, not a changed exponent; both measured IC exponents
(0.59–0.62) sit above rho's (0.44), so a verified constant-factor gain with
the gap to rho reported as distance remaining is the realistic success under
the owner's criterion.

**P5 — Wide tier (n = 61…83).** Lift the u64 caps so `S` is measured above
n = 63: plug `Gf2_128`/`koblitz_wide`/wide mod-r arithmetic into the framework
(the `InstanceCtx r: u64`, `CountedGroup::mul k: u64` and linalg u64 limits are
the known blockers). Descent builders must keep refusing inexact fields. For
prime curves, add a wide prime field. n = 83 on the a = 0 curve gets
diagnostics only (decided fraction, seconds per query, capacity per query),
labelled as stage diagnostics (§4).

**P6 — m = 4…8.** Cheapest route: wrap `pdp-split` chained-S3 plans
([2,1,…,1]) as a framework oracle; check the longest plan it accepts before
promising m = 7, 8 (it needs b = 1 Koblitz and refuses unsplit S5). The n = 61
companion shows m ≥ 3 algebraic descent is blocked by the 64-variable mask, so
a precondition is moving the descent builders to the wide (≤ 128-variable)
monomial type from `wide_groebner.rs`. The summand-sweep result (all m ≥ 3 SAT
cells capped at n = 83) is the prior to beat, not to repeat.

**P7 — Curves.** One generator per family behind a common trait, each emitting
the curve, its group order, prime-subgroup order, cofactor and a primality
certificate; registered in `docs/curves/registry.json` with full ICV1 ids. The
a = 0 n = 83 curve is already registered but has no generator or subgroup
record; completing that record (generator, prime-subgroup order, EC1
representation) is the first P7 item. A delegated audit reported it as
unregistered; that was wrong and is corrected here.

- Prime j = 0: CM construction (4p = t² + 3v²), choose the twist with prime or
  smallest-cofactor order; wide `Fp` required.
- Prime j = 1728: cofactor 2 or 4 (§3).
- Generic prime and random binary: needs a point counter first (§3).
- Binary j = 0/1728: supersingular, labelled control.
- Ladders per family at sizes spaced to give ≥ 4 rungs below 62 bits, then the
  83-bit instance.

**P8 — Reporting and landing.** Every run leaves a report, a graph and a PDF
(`research-visuals` skill). Performance gains also copy into `benchmarks/`.
No leaderboard row without a replay certificate, validator report and matched
rho with the §5a band (validation gate, 2026-10-19). Each landed trial batch
is a preregistered protocol plus raw records plus fit, mirroring the ladder's
layout (`PROTOCOL.md`, `raw/*.json`, `fit.json`).

## 6. Decisions still open

- **D1 (needs the owner).** Does the 2026-10-24 algebraic-only criterion govern
  the sweeper's counted track? It excludes the large-prime collector that
  exists today (`mitm-frobenius:large_primes`) and every pair-table arm. If it
  does, double-large-prime has to be rebuilt on the algebraic side (the
  `gaudry_cubic.rs` eliminator and the unwired `large_prime_filter.rs` are the
  starting points). The plan above defaults to two tracks, one counted.
- **D2.** Are n = 61 companion curves (unregistered by design, for registry
  size reasons) acceptable as calibration rungs for the a = 0 family?
- **D3.** Is a supersingular binary j = 0 control wanted at all, given that it
  cannot be an ECDLP target?

## 7. Risks, and what this note does not establish

- Both measured IC exponents exceed rho's, and the algebraic arm is 36,440×
  rho at n = 41 and widening. A sweeper cannot change that slope; it can
  shrink constants and find the regions where an engine is least bad.
- Everything at n = 83 on the a = 0 curve is a diagnostic until a decomposition
  oracle decides queries there (it does not today: every attempt ended
  UNKNOWN, `BOUNDED_*` or at the cap).
- Several ratios in the cited documents are weight-0 lower bounds recorded
  before the §5a rule.
- The Conductor control plane was unreachable from this session (404), and the
  sandbox blocked creating `.mcp.json` in the worktree. The lead atlas named in
  `CLAUDE.md` was not found in the sibling `crypto-autoresearcher` checkout
  (an old commit), so it was **not consulted**.
- The "Optimal-shift" session works in `crypto-autoresearcher`
  (`tools/smallroot_reach.py`); it reports its reach calculator has a sign
  error (S3 reach ≈ 0.28, not 0.8408). Nothing here relies on those figures.
- Sources read by delegated read-only audits and not re-verified line by line:
  the technique and curve inventories, and the leaderboard values in
  `graph_data.json` panel B.
