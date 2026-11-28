# F1 protocol: cold folded-constant SAT solve seeded with warm heuristic state (frozen 2026-11-16, before any scored run)

Lane F1 of `research/ecc2k130_followups_20261116`. Rules: `../COMMON.md` (Rust only, cores 0–2,
no paid compute, no commits). Every number this protocol leads to is `[pcc:unverified]`.

## 0. Prior-art audit
- `research/ecc2k130_open7_20261114/L3-incremental-sat/` (SUMMARY.md, RESULTS.md, PROTOCOL.md,
  tool/src/main.rs, sha256 `08bfa66f…72bf`): at C3 (m=23, k=2, w=2, 39 non-decomposable
  targets) B/A0 conflicts 1.02 [0.93, 1.12]; A1/A0 2.21 (assumption-encoding cost); B/A1 0.46,
  CI/A1 0.76, CA/A1 0.69, so most of B's recovery is attributed there to heuristic state.
  Its open thread 1 is this lane's question.
- `research/sat_smt_leads_20261114/08-target-parametric/README.md`: incremental solving across
  targets on a binary Koblitz PDP, inconclusive leaning negative; no phase seeding.
- `research/sat_ladder_scaling_20261110/PROTOCOL.md`: REUSE/FRESH on the ladder formulation,
  replication pending; no phase seeding.
- A grep of `research/` and `tools/` for saved-phase seeding found no prior run. research-kb and
  the lead atlas are not reachable from this lane, so no novelty claim is made.

## 1. Instrument facts checked before freezing (selftest and one unscored timing smoke)
- cadical-sys 0.8.0 exposes `phase(lit)`/`unphase(lit)` only. In CaDiCaL 3.0.1 `phase(lit)`
  sets `phases.forced`, which `decide_phase` uses before the target and saved phases on every
  decision until `unphase`. It is a persistent forced polarity, not a seed.
- cadical-sys exposes **no getter** for saved phases and **no way to read or seed variable
  activity** (EVSIDS scores, VMTF queue order). Score seeding is therefore not run in this lane.
  It would need internal heap/queue rebuilding, which is not attempted here.
- To read and seed saved phases, the lane builds against a patched copy of the cxx bridge
  (`tool/cadical-sys-f1.patch`, 83 lines). It adds `f1_saved_phase(eidx)` (read-only) and
  `f1_seed_saved_phase(lit)` (writes `phases.saved` of the mapped internal variable), through
  CaDiCaL's own `Testing` friend accessor. CaDiCaL sources are unchanged.
  Selftest: seeding the saved phases with a model of a decomposable m=11 target gives 0
  conflicts on the first call (5 of 5 targets, both via seed and via forced `phase`). Lucky
  phases do not overwrite saved phases (`propagate.cpp`, `searching_lucky_phases`).
- **Layout-stable folded encoding.** A dual builder runs the template (R as variables) and the
  folded encoding (R as constants) side by side. A folded gate that needs a fresh variable takes
  the template index of the gate built at the same step. Compacting in increasing order
  reproduces A0's CNF exactly, so every W arm solves the identical CNF that A0 solves, and a
  template index means the same gate for every target. The equality to
  `build_system(Some(R))` is asserted per target.
- **Target-independent core.** Folded clauses whose gate output has no R in its cone (weight
  counters, lex order, the uv products for k=2, the whole first S3 link for k=3). The core set
  (template indices) is asserted identical across targets.
- Unscored timing smoke, m=23, position 0 only (W arms unseeded there), file
  `$SP/lanes/F1/dev/dev_m23_pos0_unscored.jsonl`: A0 70,734 conflicts (equals L3 A0 at
  position 0), WP/WF/WR/WT/WL equal A0 at position 0, B 126,008 (equals L3's B at position 0).
  About 46 s per target for all arms in one process.

## 2. Instance and stream
L3's generator verbatim (copied into `tool/src/main.rs`): curve y²+xy=x³+1 over GF(2^m),
type-II ONB, factor base HW(x) ≤ w, chained S3 links, lex order, Sinz counters. Target stream:
L3's seed string `L3/targets/<tag>`, extended (positions 0–39 are L3's 40 targets). No repeat
block. Every target is fully enumerated (solve, block on the projection, repeat until UNSAT), and
every arm's model set must equal brute-force ground truth (fatal otherwise).
- **C1** m=11, k=2, w=2: 80 targets.
- **C3** m=23, k=2, w=2: 80 targets (deciding cell).
- **C4** m=23, k=3, w=2: optional. Run only if C1 and C3 finish, and an unscored A0 smoke on
  position 0 finishes within 120 s. Then 20 targets with arms A0, WS, AA, WP, WR, WT, WL. Otherwise
  it is dropped and recorded.

## 3. Arms (same targets, same CNF, same solver options `factor=0, factorcheck=0`)
Budget 2,000,000 conflicts per solve call; censored calls are reported, never imputed.
- **A0** cold, R folded in as constants (current practice). After its enumeration its final
  saved phases are read (read-only).
- **AA** A0 with the clause list shuffled (L3's seed string): A/A noise floor.
- **WP (primary)** fresh solver, A0's CNF, saved phases of every variable present in both
  instances seeded from WP's own final saved phases on the previous target.
- **WF** as WP, but seeded through the API's forced `phase()` (persistent for the target).
- **WR (negative control)** as WP, same variables, random signs (seed `F1/WR/<tag>/<pos>`).
- **WS (positive control)** same target re-solved cold right after A0, saved phases seeded with
  A0's final saved phases of the same target.
- **WT (tracer control)** as WP, with an LRAT proof tracer attached (antecedents on).
- **WL** as WT, plus the carried target-free learnt clauses: live derived clauses whose LRAT
  antecedent cone contains only core clauses (or earlier carried clauses), added as clauses to the
  next target, accumulated, capped at 200,000. Per target, up to 30 newly carried clauses are
  checked against the core alone (core ∧ ¬clause must be UNSAT; fatal otherwise).
- **B** reference: L3's persistent solver with R as assumptions.

Arms are interleaved: the arm order rotates with the target position (A0 and WS form one unit).
C3 runs as three processes on cores 0–2, {A0+WS, AA, B}, {WP, WF, WR} and {WT, WL}, each over the
same stream. The solver is deterministic, so counted metrics do not depend on placement.

## 4. Metrics
Primary: total conflicts per target (sum over all calls of the enumeration). Secondary:
propagations, decisions, first-call conflicts, split by decomposable/non-decomposable target.
Phase-transfer diagnostics (offline, from dumped phase vectors, by template variable class
core/R-dependent): agreement of WP's seed with A0's final phases of the same target; of A0's
final phases on consecutive targets; of A0 vs AA final phases on the same target (A/A of phase
agreement); of WR's seed with A0's final phases (≈ 0.5 expected); and, for decomposable targets,
of the seed with A0's first model.

## 5. Statistic and decision rule
Over stream positions 1 to 79 (position 0 is unseeded for the W arms): geometric mean of
(c_X + 1)/(c_Y + 1), 95% percentile bootstrap over targets (2000 resamples, fixed seed).
Verdict on **C3, WP vs A0, conflicts**:
- **IMPROVES:** upper CI < 1, and the positive control passes.
- **WORSENS:** lower CI > 1.
- **NO-CHANGE:** the CI contains 1 and lies inside [0.8, 1.25].
- **INCONCLUSIVE:** anything else, or censoring on more than 10% of C3 targets in A0 or WP.
- **Positive control passes** when WS/A0 upper CI < 1 in C1 or C3. If it fails in both, any
  IMPROVES becomes INCONCLUSIVE, and a NO-CHANGE carries the note "the instrument does not see
  phase seeding even with the target's own phases".
- **Attribution:** an improvement is credited to transferred phases only if WP/WR upper CI < 1
  as well. WL is credited with a learnt-clause effect only if WL/WT upper CI < 1.
Secondary pairs (reported, not deciding): WF/A0, WR/A0, WS/A0, WT/A0, WL/A0, AA/A0, B/A0,
WP/WR, WL/WT, WT/WP, each also for propagations and by SAT/UNSAT target.

## 6. Pre-registered predictions
- P1. C3 verdict NO-CHANGE (WP/A0 within [0.9, 1.1]). Prior probability of IMPROVES 0.25.
  Reason: all C3 targets are non-decomposable, and for an UNSAT enumeration the phase mostly
  steers which part of the refutation is found first; L3's B gain was measured against the
  assumption encoding.
- P2. WS/A0 < 1 at C1 (decomposable targets, the first model is cheap); closer to 1 at C3.
- P3. WF/A0 > 1 (forced polarity on every variable removes phase saving).
- P4. WR/A0 CI contains 1.
- P5. WL/WT CI contains 1: few clauses are core-derived at k=2.
- P6. Phase agreement on consecutive targets is above 0.6 on core variables and near 0.5 on
  R-dependent variables.
- **Falsifier of P1:** WP/A0 upper CI < 1 at C3 with WP/WR upper CI < 1.

## 7. Counted-cost mapping (AGENTS.md §5a)
Conflicts and propagations are unpriced native operations. Per-target propagations are priced at
0, 0.1 and 1 GAE as a band, and two arms are ranked only where the order holds at both nonzero
prices. There is no relation-yield or linear-algebra phase here, so no end-to-end number and no
gap to matched rho: a saving would be a stage ratio of the decomposition oracle only.

## Amendment 1 (2026-11-16, after the C3 runs; rules, arms, statistic and predictions unchanged)
- **C3 process p3 {WT, WL} stopped after position 61** (`raw/rows_m23k2w2_p3.jsonl`, kept
  unchanged). Its stderr after 01:34Z is lost: at 01:34Z a repository artifacts hook wrote
  `raw/.gitignore` and `raw/.artifacts.json` and the log files stopped updating. The cause of the
  stop is not established. At position 61 WL carried about 23k clauses, so its own memory was small.
- p3 was rerun from scratch with the same frozen binary (sha256 `c9731a3f…3b87`) into a new file,
  `raw/rows_m23k2w2_p3v2.jsonl`. Its positions 0–61 are identical to the stopped file in every
  counted field (124 of 124 rows), as the solver is deterministic. The analysis uses p3v2.
- **C4 dropped** under section 2: the unscored A0 smoke at m=23, k=3, position 0
  (`raw/dev_c4_smoke_unscored.jsonl`) was censored at the 2,000,000-conflict budget after 273 s.
