# Protocol v1: panel P, presentation dependence of the PKM Kummer relation finder (frozen before data)

**Study:** `research/pkm_present_panel_20261115/`. **Written:** 2026-11-15 (draft v0.1), revised to v1 the same day after
the pre-freeze red-team (`REDTEAM_PROTOCOL.md`, B1–B5 adopted, S1–S13 and M1–M10 adopted or answered below).
**Status:** draft until §9 names a freeze commit; no data-seed row is run before that. **Standing:** every number
produced here is `[pcc:unverified]` until a pcc claim is filed; nothing reaches a leaderboard or
`benchmarks/README.md` without the validation gate.

**Why this experiment.** The owner did not approve the 72 h panel A run
(`research/pkm_m8_sparse_panel_20261114/AMENDMENTS.md` A3) and asked for a different experiment. Panel A measured
one presentation of the m = 8 relation finder (the raw chained S_3 system with the y-tower membership), and its
KILL scope excluded "other presentations (symmetrized or elementary-symmetric variables, fewer auxiliaries)".
What the lab knows about that axis:

- `research/pkm_symmetric_tower_20261026/` (stage diagnostic, native `f4_fp`, two random targets per cell,
  m = 2, 3): the **symmetric tower** presentation (elementary symmetric unknowns with a remainder tower) has F4
  solving degree one lower than the plain Kummer chain and matrices about 10× narrower at m = 3; a **single
  S_4** without symmetrisation is the worst arm (D = 12 = its own degree). Not measured there: counted work,
  extraction, real SAT/UNSAT targets, m = 4, a second engine, direct (tower-free) membership, merge trees.
- Lead atlas (crypto-autoresearcher) `LEAD-ICE-gag-contraction-planner`: over F_{p^3}, n = m = 3, the monolithic
  S_4 used 0.110× the chained system's F4 matrix cells (run-001, `supported_within_scope`); its recorded next step
  is the prime-field m = 3 (chain, tree, monolithic) and m = 4 (chain, tree) measurement. `LEAD-ICE-semaev-chain-cost-model`
  reopens on "a solver whose degree depends on topology on measured cells". `LEAD-ICP-pkm-tower-algebraic-oracle`
  records the P-256 isogeny-chain encoding as 2×–654× slower than the dense encoding of the same set.

Panel P is the **measured follow-up of the 2026-10-26 diagnostic**, on identical real instances, with counted work,
on this container, at zero spend. It asks whether and by how much the presentation changes the per-call cost of
the relation finder at m = 3 and m = 4, which presentation is cheapest, and how each presentation's cost grows
with the base size. It decides nothing about m = 8 (§5, S8).

## 0. Sources checked (knowledge base first)

- `research-kb` MCP and `scripts/research-kb`: unavailable in this container (no executable); recorded, not waived.
- Lead atlas `--match` (unlocked checkout, 2026-11-15): `LEAD-ICE-gag-contraction-planner` (open_next_step),
  `LEAD-ICE-semaev-chain-cost-model` (cost_model obstruction, revisit on topology-dependent degree),
  `LEAD-ICE-prime-field-semaev-solving-degree` (closed_known; revisit on a bounded-degree membership base),
  `LEAD-ICP-pkm-tower-algebraic-oracle` (open_blocked), `KR-IC-955fd6` (symmetrised summation polynomials,
  extension fields, heuristic), `LEAD-ICB-decomposition-encoding-and-sat` (binary field).
- crypto: `research/pkm_symmetric_tower_20261026/{README.md,rows*.jsonl}` and `examples/pkm_symmetric_tower.rs`
  (the symmetric tower construction reused as arm `sym-tower`); `docs/ic/factor-base-encyclopedia/README.md` §0.1,
  §8 (PKM_GOALS G3 names this comparison); `research/pkm_m8_sparse_panel_20261114/` (instrument `pkm_m8_panel`,
  pilot facts on msolve); `src/cryptanalysis/semaev_higher.rs` (S_4..S_6 extractions; S_6 cannot be expanded);
  `examples/pkm_oracle_program/algebra.rs`.
- crypto-autoresearcher: `research/gag_presentation_matched_20261031/` (statistic, interval and verdict vocabulary
  reused; its review REV-…c184b3 flagged the single max-row A/A floor, repeated here as S3 and fixed).
- **Corrected claim (S6):** the draft said no prime-field presentation measurement existed. False: the 2026-10-26
  diagnostic measured chain vs single-S_4 vs symmetric tower at m ≤ 3 on D and matrix width. What is untried, by
  grep over the four repositories and the atlas: counted paired work with extraction and construction charged,
  tower-free (direct) membership, merge trees, m = 4, msolve as a second engine, the per-presentation B-secant.

## 1. Question and claims

Instance = (curve `E/F_p` of prime order r, Kummer base `V = g·μ_{2^t}` with `B = 2^t` abscissae, real base
`V ∩ x(E(F_p))` of size `B_real`, arity m, target abscissa `x_R` with `R = aP + bQ`). A presentation is a
polynomial system over F_p whose F_p-rational solutions are (in its stated semantics) the decompositions of R into
m signed real base points.

| id | Semaev part | membership part | variables | semantics |
|---|---|---|---|---|
| `chain-tower` (baseline) | chained S_3, m−2 auxiliaries (panel A's raw system) | y-tower, t levels per leaf | m·t + (m−2) | ordered leaves, repeats allowed |
| `tree-tower` (m = 4 only) | balanced merge tree of S_3 | y-tower | m·t + (m−2) | as chain, but a decomposition whose tree-order partial sum is the identity is missed (smoke: 5 of 6 on one target) |
| `chain-direct` | chained S_3 | `x_i^{2^t} = g^{2^t}` | m + (m−2) | as chain |
| `mono-tower` (m = 3 only; **positive control**, §6: the 2026-10-26 "plain, one S_4" arm, D = 12 there and in the smoke) | S_4 by resultant | y-tower | m·t | may admit unlifted tuples (non-rational partial sums) |
| `mono-direct` (m = 3 only) | S_4 by resultant | direct | m | as mono-tower (smoke: D = 16 at t = 3) |
| `sym-tower` | S_{m+1} in e_1..e_m (2026-10-26 construction) | remainder tower `R_j = X^{2^j} mod Q`, `R_{t−1}^2 ≡ 1` | m·t | **unordered, distinct abscissae only** |
| `sym-direct` | S_{m+1} in e_1..e_m | `X^{2^t} − g^{2^t} ≡ 0 mod P_e(X)` (m equations) | m | unordered, distinct abscissae only |

Monolithic arms at m = 4 (S_5, degree 8; the smoke's mono-tower censored at 100 s with D ≥ 32) and at m = 5 (S_6 is
not expandable) are **not run**: the 2026-10-26 diagnostic already places the unsymmetrised monolithic arm last,
and S12's runtime budget goes to the untried arms. Symmetric arms run only in cells with `B_real ≥ m + 1`
(`sym_applicable`, recorded per cell); where they do not run, the cell reports them as not applicable, not as
censored (B2).

**Work** `w_A(i)` of arm A on instance i = native counted field multiplications of (solve to a reduced basis)
+ (extraction: lifting and verifying every F_p-rational solution, same `--extract-max-q` on every arm). Two
co-readings: `w^solve` (solve only; S1) and `w^+` = `w` + the per-target **construction** work (resultants,
symmetric rewrite, remainder polynomial, tower; B5), all in the same unit. Per-cell construction (independent of
the target) is reported separately and charged once per cell, never hidden.

Claims (descriptive; they decide no hypothesis about ECDLP cost and make no ranking against rho):

- **C1 (correctness gate):** on every instance, every arm's verified decompositions equal the baseline's
  (distinct-abscissa subset for the symmetric arms) and the exhaustive classifier's count; the planted
  decomposition is found by every arm that completes. A censored arm makes C1 **not evaluable** on that instance
  (B3), never a failure.
- **C2 (presentation effect, per cell and label):** for each candidate arm A, cell (m, t) and label stratum
  (real-SAT; real-UNSAT), the median paired `log2(w_A / w_chain-tower)` differs from 0 with the same sign on both
  primes and on `w^+`, by the rule of §5.
- **C3 (secant per presentation):** `γ_A(B→2B) = median_i log2(w_A(i at t+1) / w_A(i at t))` over instances
  paired by (prime, label, draw rank) on the **same curve** (nested bases, S11), reported with spread; no gate value
  is applied (S7).

**No ADVANCE is possible:** no held-out size, no priced comparison with rho. §5a: C2 is a ranking of arms by
counted native work; every arm is priced in the same unit (field multiplications), so the 0 / 0.1 / 1 GAE band
collapses to one number and is stated as such (S2). The m ≤ 4 cells **do not decide** anything about m = 8; they
inform which presentation any future m = 8 plan should use, and nothing more (S8).

## 2. Instruments

- **Native (primary):** `examples/pkm_present_panel.rs` (new; reuses `examples/pkm_m8_panel/*`,
  `examples/pkm_oracle_program/algebra.rs`, `examples/pkm_symmetric_tower.rs`), engine `crypto_lib::cryptanalysis::f4_fp`
  (grevlex), counted field multiplications (`field_ops_total`), solving degree, degree reached, basis size,
  `solve_field_multiplications`, `extract_field_multiplications`, `construction_field_multiplications` (per cell
  and per target), progress streamed every F4 step to `<out>.progress.json` so an external stop leaves a lower
  bound (B4, S12). `RAYON_NUM_THREADS=1`, `taskset -c 0`, thread count actually used recorded (probe: user 266 s vs
  77 s wall at m = 4, t = 3 without the pin). Degree bound `--max-degree 64`, non-binding by design (B4); the only
  stop is the external wall cap.
- **msolve (co-observable, F4 work only; S4):** `/usr/bin/msolve` 0.6.5-1build2 (sha256 `c2722288…59fa`),
  `msolve -t 1 -v 2 -c 0 -g 2 -f X.ms -o X.out` (Gröbner basis only, no parametrisation, genericity reruns off),
  `work_proxy` = Σ rows×cols×density of the single F4 pass (`research/pkm_m8_sparse_panel_20261114/pod/parse_msolve_log.py`),
  max round degree, CPU s. Every prime is ≥ 2^18: msolve 0.6.5 segfaulted at once on p = 65537, 65557 and 131101
  and ran at 262153 and above (M1: the observed crash band is [2^16, 2^18); smaller primes were not tried).
- **Classifier and targets:** `pkm_m8_panel targets` (exhaustive signed-multiset classifier, `--m`; records
  `real_count`, `real_count_distinct`, `algebraic_count`); planted targets drawn with distinct abscissae (B2).
- **Host:** this container (4 vCPU Intel Xeon 2.30 GHz, 15 GB, `lscpu` recorded), one call at a time. CPU seconds
  are L0-class and secondary; counted operations and the matrix proxy are host-independent.
- **Frozen environment:** no `F4_*` environment variable set (the runner clears them and records `env`); msolve
  flags as above; analysis script `analysis/build_results.py` frozen by hash in `FREEZE.json` (S10).

## 3. Cells, instances, order

**Curves (nested bases, S11):** one curve per prime serves every t: p is prime with `16 | p − 1`, not Fermat,
prime-order curve from the seeded search, and the nested bases are all eligible cells: `B_real(t = 4) ≥ 8`,
`B_real(t = 3) ≥ 4`, `B_real(t = 2) ≥ 2` (each at least half of B, the inherited eligibility rule; it also gives
the symmetric arms their `B_real ≥ m + 1` at t = 4 for m = 4 and at t = 3 for m = 3). Two primes: `p1` = least such prime ≥ 2^18, `p2` = least such prime ≥ 2^20 (a third, ≥ 2^22, for m = 3
only). Bases `g·μ_4 ⊂ g·μ_8 ⊂ g·μ_16` on the same curve with the same g. Exact commands (every flag explicit,
`--no-l1-reuse` always; the instrument records the rule it applied and refuses an L1 reuse):

```
pkm_present_panel cell --t 4 --ratio 5 --min-prime <2^18|2^20|2^22> --require-breal 8 --require-breal-nested 2:2,3:4 --seed 20261115 --slug c<k> --out cells/c<k>
pkm_present_panel cell --t 4 --ratio 5 --prime <p_k> --require-breal 8 --require-breal-nested 2:2,3:4 --seed 20261115 --slug c<k> --nested-ts 2,3 --out cells/c<k>
pkm_present_panel targets --cell cells/c<k>[/nested_t<t>] --n-sat 5 --n-unsat 5 --planted 0 --planted-distinct 1 --max-draws 1000000 --seed 20261115 --m <m> --out cells/c<k>t<t>/m<m>
```

Derived (commit `81242d990e`): c1 `p = 262321` (B_real 8 / 4 / 2 at t = 4 / 3 / 2; one prime rejected, 262193),
c2 `p = 1048721` (11 / 5 / 2; rejected 1048609), c3 `p = 4194433` (8 / 5 / 3; rejected 4194353); `g = 4` on all three.
The t = 2 cells have `B_real < m`, so their planted target is a plain planted decomposition (`--planted 1
--planted-distinct 0`): the symmetric arms do not run at t = 2 and a distinct planting is impossible there.
Symmetric arms therefore run at m3t3 and m3t4 on every curve, at m4t4 on c1 and c2, and at m4t3 on c2 only.

| cell | m | t | B | curves | arms |
|---|---|---|---|---|---|
| m3t2 | 3 | 2 | 4 | c1, c2, c3 | chain-tower, chain-direct, mono-tower, mono-direct (+ sym-* iff B_real(t=2) ≥ 4) |
| m3t3 | 3 | 3 | 8 | c1, c2, c3 | chain-tower, chain-direct, mono-tower, mono-direct, sym-tower, sym-direct (iff B_real ≥ 4) |
| m3t4 | 3 | 4 | 16 | c1, c2, c3 | same as m3t3 |
| m4t2 | 4 | 2 | 4 | c1, c2 | chain-tower, tree-tower, chain-direct |
| m4t3 | 4 | 3 | 8 | c1, c2 | chain-tower, tree-tower, chain-direct, sym-tower, sym-direct (iff B_real ≥ 5) |
| m4t4 | 4 | 4 | 16 | c1, c2 | same as m4t3; **exploratory** (probe: chain-tower at m = 4, t = 3 already costs 77 s wall) |

**Instances per (cell, curve):** 5 natural real-SAT, 5 natural real-UNSAT (seeded draw order, classifier, draw cap
10^6; a cell whose draws exhaust the cap on a curve records it and the curve is dropped from that cell, M8), 1
planted with distinct abscissae (B1, B2). Hidden `k` and decompositions in `hidden.json`, never read by a solver
step. Labels and `algebraic_count` are public for stratification (S9).

**Order (frozen):** cells in table order (m = 3 block then m = 4 block); within a cell, curves c1, c2, (c3);
within a curve, instances in draw order; within an instance, arms in the table order, then the literal A/A
(chain-tower rerun; S3, S13); for each arm the native call then the msolve call. The arm order is fixed and
disclosed (M4); the native engine is deterministic, so order cannot leak between arms. One process at a time, no
retry. The runner is resumable (rows already in `rows.jsonl` are skipped by `(cell, curve, instance, arm, engine)`);
a call interrupted by a container loss is a **lost row**, re-run once on resume and tagged `rerun: true` (S12).

**Per-call caps (frozen):** m = 3 cells: native 600 s wall (external `timeout -k 10 600`) and 10 GB address space;
msolve 600 s and 10 GB. m = 4 cells: 1,800 s and 10 GB for both engines (the smoke's chain-tower at m = 4, t = 3
censored at 100 s single-threaded, and the probe's 266 CPU-s at 4 threads says the baseline needs minutes; a cap
that censors the baseline on most instances would void C2 for the whole m = 4 block). The native `--deadline` is
86,400 s (non-binding) so that extraction is only ever stopped by the external cap and leaves its progress line. **Stop rules:** if `chain-tower` censors on more than half of a cell's instances on a curve, the remaining
cells of that m are not run (reported); the m4t4 block runs only after every other cell has finished.

**Runtime (S12):** calls = Σ_cells Σ_curves 11 instances × (arms + A/A) × 2 engines ≈ 3 × 11 × (4..6) × 2 × 3
(m = 3 block) + 2 × 11 × (4..6) × 2 × 3 (m = 4 block) ≈ 1,000–1,300 calls. From the seed-777 probe, m = 3 cells and
m4t2 take seconds per call; m4t3 about 1–3 min per call (≈ 3–7 h for the cell); m4t4 unknown, capped. Expected
8–16 h wall in total; the external caps bound the worst case at about 1,300 × 600 s = 9 days, which is why the
m4t4 block is last and the stop rules exist.

## 4. Observables (per call)

`arm, cell, curve, instance, label, algebraic_count, real_count, real_count_distinct, nvars, nequations,
max_total_degree, nterms_total, construction_field_multiplications (per target), solve_field_multiplications,
extract_field_multiplications, solving_degree, degree_reached, basis_size, threads_used, cpu_s, wall_s, max_rss_kb,
censored ∈ {no, timeout, as_rlimit, crash, not_applicable}, progress_last (ops, degree at the stop for censored
calls), verified_real_decompositions, unlifted_solutions, consistent_with_classifier`; msolve: `work_proxy,
max_round_degree, rounds, cpu_s, censored`. Per instance: the sorted, hashed decomposition set per arm (C1). Per
cell: per-cell construction work, classifier enumeration cost (analysis cost).

Censored calls carry `w_lo` = ops at the stop (from the progress file) and enter the paired statistics as
**+∞ on the candidate side** and as `−∞` when the baseline is censored (i.e. the pair's sign is known whenever one
side completed and the other's lower bound already exceeds it; a pair with both sides censored is dropped and
counted) (B3).

## 5. Analysis and decision rule (frozen)

For each cell (m, t), each candidate arm A, each label stratum L ∈ {SAT, UNSAT} (S9), pooled over the cell's curves
(B1):

- pairs `d_i = log2(w_A(i) / w_chain(i))` over the stratum's instances on all curves (n = 10 for two curves, 15 for
  three); statistic = median of `d_i` (ties at exactly 0 count as 0 and are reported); exact paired sign-flip test
  on the `d_i` (p_min = 2/2^10 ≈ 0.002); percentile bootstrap of the median (20,000 resamples, level `1 − 0.05/k`,
  k = number of candidate arms in the cell); degenerate intervals (all signs equal) are reported as such (M3).
- **floor:** literal A/A `D_AA` = max_i |log2(w_AA(i)/w_chain(i))| per curve (expected 0 for a deterministic
  engine; a nonzero value is itself a finding), and the **positive control** (mono-tower at m = 3; §6).
- **supported_within_scope** for (A, cell, L): sign-flip p ≤ 0.05/k, interval excludes 0, the per-curve medians have
  the same sign on every curve of the cell, `|median| > D_AA`, the same sign holds on `w^+` (B5) and on
  `w^solve` (S1), and the msolve `work_proxy` median has the same sign (S4 co-gate). Scope: the measured (m, t,
  curves), both strata as stated, these two engines.
- **within_control_distribution:** `|median| ≤ D_AA` on every curve.
- **invalid_measurement:** C1 fails on any completed instance of the cell.
- **inconclusive:** otherwise, including fewer than 8 valid pairs in the stratum (B1), the positive control not
  detected in that cell (§6), or the co-gate disagreeing.
- Family-wise error across cells and strata is **not** controlled beyond the per-cell Bonferroni; the number of
  tests is reported with the results (M2).
- C3 per arm: `γ_A(t→t+1)` = median over pairs (same curve, same label, same draw rank) of
  `log2(w_A(t+1)/w_A(t))`, with the interquartile range; a censored numerator makes the pair a lower bound. No
  gate value is applied; the arity-matched heuristic gates the red-team derived (0, 1, 2 at m = 3, 4, 5) are
  quoted as desk, unverified, for orientation only (S7).

**Pre-registered expectations, which the data may contradict:** from the 2026-10-26 diagnostic and the seed-777
smoke (development data, D only), `sym-tower` below `chain-tower`, `mono-tower` and `mono-direct` above it;
`sym-direct` (D = 7 vs 6 in the smoke), `chain-direct` and `tree-tower` undecided.
**Non-improvement sentence (S8):** whatever C2 finds, no presentation effect at m ≤ 4 is a measured improvement
of the index-calculus baseline; the panel changes no leaderboard row and makes no claim about m = 8.

## 6. Controls and variable ledger

| control | arm | expected | failure means |
|---|---|---|---|
| C1 identical verified relations | all completed arms, every instance | same decomposition set (semantics-adjusted); planted found | exporter/extractor fault; instance invalid |
| classifier gate | exhaustive `real_count` / `real_count_distinct` vs chain-tower's verified set | equal | classifier or solver fault; cell invalid |
| per-arm variety gate (S5) | F_p-rational solution count of each arm vs the count its semantics predicts from the classifier (ordered with repeats / distinct unordered) | equal; monolithic arms ≥ with `unlifted` explaining the excess | exporter fault: arm invalid in that cell |
| positive (detector sees solutions) | planted instance per (cell, curve), every arm | planted decomposition recovered | arm invalid in that cell |
| negative (no false relations) | real-UNSAT instances | zero verified decompositions; algebraic solutions counted | false relation: invalid run |
| **positive control for the C2 statistic (S13)** | `mono-tower` at m = 3 (the 2026-10-26 "plain, one S_4" arm, D = 12, reproduced in the smoke) | `supported_within_scope` with positive sign in every m = 3 cell where it completes | the statistic cannot detect a known effect: the cell's other C2 verdicts are inconclusive |
| literal A/A (S3, S13) | chain-tower rerun on every instance | identical counts (D_AA = 0) | engine nondeterminism; D_AA becomes the floor |
| engine cross-check (S4) | msolve `-g 2` proxy sign | same sign as native | engine-specific effect: inconclusive |
| construction charged (B5) | `w^+` sign agreement | same sign | "presentation" was precomputation moved offline: inconclusive |
| identities (pre-run) | `cargo test`: resultant vs `semaev_higher`, symmetric rewrite, remainder criterion both directions, sym-tower vs the 2026-10-26 builder, chain-tower byte-identical to panel A's exporter | pass before any data | exporter fault: nothing runs |
| blinding | `hidden.json` never read by the pipeline; the analyst runs `analysis/build_results.py` only after the plan finished | | |
| not run, stated | random-base null (presentation vs Kummer structure), Magma arm, enumeration reference row (S13) | — | listed as limits of scope, not controls |

Variable ledger: instrument commit and binary sha256; msolve sha256 and flags; `RAYON_NUM_THREADS=1`, `taskset`;
cleared `F4_*` environment; one host; fixed order; seeds derived from 20261115 and the explicit cell slugs (S10:
each (curve, t) has its own slug, so no two cells share an RNG stream); caps fixed; no retry; no cap raised after
seeing results. Confounders isolated: same instance across arms (paired), same engine settings, same prime band and
the same curve across t (nested bases), label strata separated, construction charged, extraction uniform.

## 7. Budget, route and stop rules

Local container, zero spend, one call at a time. Route reason (M9): the isolab monthly budget is exhausted
(1,808.95 of 1,300 USD on 2026-11-14) and the owner declined a paid host; counted operations do not need an
isolated host, and CPU seconds are secondary. Resume checkpoint (`.resume/RESUME.md`) filled and pushed before the
run (M6; no waiver needed: the S3 token works in this container). Stop rules in §3. A follow-up at m ≥ 5 or on a
bigger host is a new protocol version.

## 8. Records and deliverables

`cells/<curve>/{cell.json}` and `cells/<curve>t<t>/{targets.json,hidden.json}` with `cells/MANIFEST.json` (ICV1
curve slugs recorded, M7); `systems/<cell>/<curve>/<instance>/<arm>.{ms,system.json}`; `plan.json` (frozen order);
`results/<stamp>/{rows.jsonl,calls/*,analysis/}`; `RESULTS.md` (tables of §4–§5, C1/C2/C3, the number of tests,
the seed-777 smoke disclosed as development data, M10); `figures/presentation_work.svg|png` (per-arm work vs B,
log y, per m and label, censoring marks); `REPORT.typ` (`REPORT.pdf` if Typst is permitted); bench record
`benchmarks/records/20261115T0345Z-pkm-present-panel.json`; raw tarball to S3 by SHA-256; pcc claims for the cell
derivation, C1 and each C2 verdict; session-log entries; PR with auto-merge. The metric is not comparable with the
F_{p^3} predecessor's matrix cells and the report says so (M5).

## 9a. Pre-freeze clarifications (v1.1, 2026-11-15; seen: the seed-777 probe and smoke, runner and analysis stub tests; no data row)

Resolved while the runner and analysis were built (`runner/`, `analysis/`), before the freeze:

1. Layout: `cells/<curve>/cell.json` (curve derived at t = 4); `cells/<curve>t<t>/m<M>/{targets.json,hidden.json}`
   (targets depend on m and t); systems under `systems/<cell>/<curve>/<instance>/<arm>.{ms,system.json}`.
2. Instance order within a (cell, curve): by `draw_index`, planted last. In m4t2 the symmetric arms are
   `not_applicable` (B_real ≤ 4 < 5) by construction.
3. Strata are the **real** labels (SAT, UNSAT); the split by `algebraic_count` (0 vs > 0) is descriptive.
4. Sign-flip statistic: the **sum** of the paired `d_i` (Fisher's exact randomisation over sign flips; at n = 10
   the smallest attainable p is 2/1024), with ±∞ imputed lexicographically; exact zeros count as 0 in the median
   and are dropped from the test. The verdict's median condition and the test are separate clauses.
5. Censored pairs enter as +∞ (candidate censored) or −∞ (baseline censored) regardless of the bound; pairs whose
   lower bound does not already exceed the completed side are counted and reported (`n_lb_not_exceeding`), and
   the analysis is re-run with them dropped as a sensitivity reading.
6. k for Bonferroni = the candidate arms applicable in the cell, `mono-direct` included.
7. The positive control (`mono-tower`) gates per (cell, stratum) at m = 3; the m = 4 cells have no control arm
   and their verdicts are reported without that gate, stated in RESULTS.md.
8. `|median| > D_AA` uses the largest per-curve `D_AA`; `within_control_distribution` uses per-curve medians.
9. The stop rule counts native chain-tower censoring only.
10. Per-arm variety gate: equality of `fp_rational_solutions` within the chain group (chain-tower, tree-tower,
    chain-direct, aa) and within the symmetric pair, and against the classifier's ordered / distinct counts where
    the instrument records them; the monolithic arm's excess is explained by `unlifted_solutions`.
11. Decompositions are exchanged as sorted lists of `[x, sign]`; "planted found" means the public
    `planted_abscissae` set of the target appears in the arm's verified list (hidden.json is never read).
12. `construction_field_multiplications` is per-target only; the per-cell part is a separate field and is
    reported per cell, not added to `w^+`.
13. C3 pairs instances by (curve, label, draw rank) across t on the same curve; the targets themselves differ
    between t (a target's SAT status depends on the base), which is why the pairing is by rank and the spread is
    reported. "Nested bases" means the same curve and g, not the same target.
14. The native `--deadline` is 86,400 s (non-binding); the external 600 s wall cap is the only stop.
15. Failure classification without a marker (exit 134/137/139 with RSS ≥ 50 % of the cap → `as_rlimit`,
    otherwise `crash`) is a recorded heuristic (`censor_basis`).
16. `verify-msolve` is not run (`-g 2` has no parametrisation); the msolve arm is the F4 proxy only.
17. (v1.2, after the instrument smoke) Positive control is `mono-tower`, not `mono-direct`: the prior study's
    "plain, one S_4" arm is S_4 with the y-tower; `mono-direct` is a second monolithic arm (D = 16 in the smoke).
    Both run at m = 3 only. m = 4 caps 1,800 s. `tree-tower` semantics noted. The instrument's construction counter
    also counts the scaling inside subtractions (over-counts negations); stated with `w^+`. Symmetric arms at
    m = 4, t = 3 run only on curves with `B_real(t = 3) ≥ 5` (the curve rule guarantees it at t = 4 only).
18. (v1.2) The wrapper's `--min-prime` was not honoured in its first build (every curve landed on p = 1048609);
    fixed in `81242d990e` before any target was used, and the curves re-derived from scratch under the rule above.
19. (v1.2) C1 is semantics-adjusted for the ordered arms, by a frozen predicate on the decomposition list: a
    decomposition is *tree-missable* (m = 4) when every partition of its summands into two pairs contains a
    cancelling pair (same abscissa, opposite sign), and *chain-missable* when every ordering has an identity
    prefix sum. tree-tower's expected set is the baseline's minus the tree-missable ones; chain-direct and the
    A/A must equal the baseline; a monolithic arm must contain the baseline, its extras must be chain-missable,
    and its count must equal `real_count`; where a monolithic arm ran its set stands in for the classifier's
    (only counts are public), and the baseline must equal it minus the chain-missable ones; at m = 4 a baseline
    shortfall against `real_count` is reported (`classifier_bound_only`), not judged. Exclusion counts are
    reported per cell.

## 9. Freeze

Frozen by the commit recorded in `FREEZE.json` (written in the following commit, with the SHA-256 of this file,
`protocol.json`, `plan.json`, `analysis/build_results.py`, the instrument binary and `cells/MANIFEST.json`, and the
disclosure of what had been seen: the seed-777 probe and smoke only). Changes after the freeze go to
`AMENDMENTS.md` with a timestamp and a disclosure of what had been seen.
