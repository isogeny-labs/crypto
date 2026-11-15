# Protocol: panel P, presentation dependence of the PKM Kummer relation finder (frozen before data)

**Study:** `research/pkm_present_panel_20261115/`. **Date written:** 2026-11-15. **Status:** draft until §9
carries a freeze commit; no data-seed row is run before that. **Standing:** every number produced under this
protocol is `[pcc:unverified]` until a pcc claim is filed; nothing reaches a leaderboard or
`benchmarks/README.md` without the validation gate.

**Why this experiment.** The owner did not approve the 72 h panel A run (`research/pkm_m8_sparse_panel_20261114/AMENDMENTS.md`
A3) and asked for a different experiment. Panel A measured one presentation of the m = 8 relation finder, the raw
chained S_3 system with the y-tower membership. Two records say presentation matters by large factors and that the
prime-field case is unmeasured: the lead atlas `LEAD-ICE-gag-contraction-planner` (run-001, n = m = 3 over F_{p^3}:
the monolithic S_4 used 0.110× the chained system's F4 matrix cells at every prime, `supported_within_scope`; its
recorded next step is "first prime-field m = 3 (all three) and m = 4 (chain vs tree)"), and
`LEAD-ICP-pkm-tower-algebraic-oracle` (EXP-ECDLP-f49f75/502ae2: the P-256 2-isogeny chain encoding was 2×–654× slower
than the dense encoding of the same set). Panel A's own KILL scope excluded "other presentations (symmetrized or
elementary-symmetric variables, fewer auxiliaries)". This panel measures exactly that, on identical instances, at
arities and base sizes a 4-vCPU host solves in seconds to minutes, at zero spend.

## 0. Sources checked (knowledge base first)

- `research-kb` MCP and `scripts/research-kb`: unavailable in this container (no executable; recorded, not waived).
- Lead atlas (crypto-autoresearcher, unlocked 2026-11-15): `tools/lead_atlas.py --match` on the presentation idea
  returns `LEAD-ICE-gag-contraction-planner` (open_next_step, prime-field arms not run), `LEAD-ICE-semaev-chain-cost-model`
  (cost_model obstruction: variable count is topology-invariant; `revisit_when` = a solver whose degree depends on
  topology on measured cells, which is precisely what this panel measures), `LEAD-ICE-prime-field-semaev-solving-degree`
  (closed_known for naive bases; `revisit_when` names a bounded-degree membership base, i.e. the PKM tower),
  `LEAD-ICP-pkm-tower-algebraic-oracle` (open_blocked; chain vs dense encoding factors above), `KR-IC-955fd6`
  (symmetrised summation polynomials, literature over extension fields, heuristic; not measured here over F_p for a
  Kummer base), `LEAD-ICB-decomposition-encoding-and-sat` (binary field only).
- crypto-autoresearcher `research/gag_presentation_matched_20261031/` (protocol 609c701c, run-001, review REV-…c184b3):
  the statistic, interval and verdict vocabulary below are taken from it so that the prime-field rows are comparable.
- crypto: `research/pkm_m8_sparse_panel_20261114/` (instrument `pkm_m8_panel`, pilot, protocol v2.2 §2 msolve facts),
  `src/cryptanalysis/semaev_higher.rs` (prime-field S_4..S_6 extractions, test oracle), `src/cryptanalysis/koblitz_sym_s4.rs`
  (symmetrised S_4, binary field only), `examples/pkm_oracle_program/algebra.rs`.
- Not found anywhere in the four repositories: a prime-field measurement of monolithic, tree, direct-membership or
  symmetrised presentations of the PKM Kummer system. This is a "not tried" claim under the rule; it rests on the
  atlas and grep, not on the knowledge base.

## 1. Question and claims

One instance = (cell, target): a curve `E/F_p` of prime order `r`, the Kummer base `V = g·μ_{2^t}` (B = 2^t
abscissae), the real base `V ∩ x(E(F_p))` (`B_real`), a target abscissa `x_R` with `R = aP + bQ`, and the arity `m`.
A **presentation** is a polynomial system over F_p whose F_p-rational solutions are (in the stated semantics) the
decompositions of R into m signed real base points. Six presentations of the same instance:

| id | Semaev part | membership part | variables |
|---|---|---|---|
| `chain-tower` | chained S_3, m−2 auxiliaries (panel A's raw system) | y-tower, t levels per leaf | m·t + (m−2) |
| `tree-tower` | balanced merge tree of S_3 (m ≥ 4; = chain at m = 3) | y-tower | m·t + (m−2) |
| `mono-tower` | S_{m+1} (all internal wires contracted by resultants) | y-tower | m·t |
| `chain-direct` | chained S_3 | `x_i^{2^t} = g^{2^t}` | m + (m−2) |
| `mono-direct` | S_{m+1} | `x_i^{2^t} = g^{2^t}` | m |
| `sym-direct` | S_{m+1} in elementary symmetric variables e_1..e_m | `X^{2^t} − g^{2^t} ≡ 0 mod P_e(X)` (m equations) | m |

Solution semantics (stated before data): chain/tree presentations have exactly the F_p-rational decompositions
(with ordered leaves); monolithic presentations may admit extra tuples whose internal partial sums are not
F_p-rational (counted as `unlifted`, never as relations); `sym-direct` identifies permutations (unordered
solutions). Every arm's solutions are lifted and verified by point addition; only verified decompositions count.

Claims (descriptive; they decide no hypothesis about ECDLP cost):

- **C1 (correctness gate):** on every instance, every arm's verified decompositions are the same set (modulo the
  stated semantics) and equal the exhaustive classifier's count; the planted decomposition is found by every arm.
- **C2 (presentation effect, per cell):** for each candidate arm A and each cell (m, t), the median over
  instances of `log2(work_A / work_chain-tower)` differs from 0 beyond the permutation control, in the same
  direction on every prime of the cell. Work is the native engine's counted field multiplications to a reduced
  Groebner basis plus extraction (primary) and msolve's F4 matrix-cell proxy (secondary).
- **C3 (secant per presentation):** the local secant `γ_A(B→2B) = log2(work_A(2B) / work_A(B))` over
  t = 2→3 and 3→4 at fixed m, per arm, with its spread over instances; reported against the panel A gate value
  3.5 as a distance, never as a verdict about m = 8.

**No ADVANCE is possible:** the panel contains no held-out size and no priced comparison with rho; §5a is
satisfied by making no cross-method ranking at all.

## 2. Instruments

- **Native arm (primary):** `examples/pkm_present_panel.rs` (new; reuses `examples/pkm_m8_panel/*` and
  `examples/pkm_oracle_program/algebra.rs`), engine `crypto_lib::cryptanalysis::f4_fp` (grevlex, 1 thread), counted
  field multiplications as the engine names them (`field_ops_total`), solving degree, degree reached, basis size,
  extraction counted separately, CPU seconds secondary. Deterministic: the same input gives the same count.
- **msolve arm (secondary):** `/usr/bin/msolve` 0.6.5-1build2 (sha256 `c2722288…59fa`), `msolve -t 1 -v 2 -P 1`, one
  process per call; `work_proxy` = Σ rows×cols×density over all F4 passes from the `-v 2` log (parser
  `research/pkm_m8_sparse_panel_20261114/pod/parse_msolve_log.py`), max round degree, CPU seconds. Every prime is
  ≥ 2^18 (msolve 0.6.5 segfaults at once below it on this host, panel A `PILOT.md`).
- **Classifier and targets:** `pkm_m8_panel cell|targets` (exhaustive signed-multiset classifier; `--m`).
- **Host:** this container (4 vCPU Intel Xeon 2.30 GHz, 15 GB; `lscpu` recorded), one call at a time, `nice 0`;
  CPU seconds are L0-class and secondary; counted operations and the matrix proxy are host-independent.

## 3. Cells, instances, order

| cell | m | t | B | p rule | arms |
|---|---|---|---|---|---|
| m3t2 | 3 | 2 | 4 | least eligible prime ≥ 2^18 | chain-tower, mono-tower, chain-direct, mono-direct, sym-direct |
| m3t3 | 3 | 3 | 8 | least eligible prime ≥ 2^18 with 8 \| p−1 | same |
| m3t4 | 3 | 4 | 16 | least eligible prime ≥ 2^18 with 16 \| p−1 | same |
| m4t2 | 4 | 2 | 4 | as m3t2 | all six (tree-tower added) |
| m4t3 | 4 | 3 | 8 | as m3t3 | all six |
| m4t4 | 4 | 4 | 16 | as m3t4 | all six |
| m5t2 | 5 | 2 | 4 | as m3t2 | all six; **exploratory**: any arm may censor at the cap |
| m5t3 | 5 | 3 | 8 | as m3t3 | all six; exploratory |

Each cell runs on **two primes**: the least eligible prime ≥ 2^18 and the least eligible prime ≥ 2^20 (same t
rule), so that C2's "same direction on every prime" has two primes; a third prime (≥ 2^22) is added for m = 3
only (cheap). Eligible = prime, `2^t | p−1`, not Fermat, prime-order curve from the seeded search, `B_real ≥ B/2`
under `pkm_m8_panel`'s g rule. Cells are derived by `pkm_m8_panel cell --t T --ratio R --master-seed 20261115`
(ratio chosen so that `2^(t·ratio)` is the prime floor: t = 2 → ratios 9, 10, 11; t = 3 → 6, 7, 8 (8 | p−1 is the
instrument's rule for t = 3); t = 4 → ratios 5, 5 with `--min-prime` 2^18 / 2^20 since 2^(4·5) = 2^20 already; the
instrument records every rule it applied).

**Instances per (cell, prime):** 3 natural real-SAT, 3 natural real-UNSAT (drawn in seeded order with the
classifier, draw cap 10^6) and 1 planted; `targets --m M --n-sat 3 --n-unsat 3 --planted 1`. The hidden `k` and
the decompositions stay in `hidden.json`, never read by any solver step; labels are public for stratification.

**Order (frozen):** cells in the table order; within a cell, prime ascending; within a prime, instances in draw
order; within an instance, arms in the table order, then the permutation control; native call then msolve call.
One process at a time. No retry. A censored call stays censored.

**Per-call caps (frozen):** native 600 s wall and 10 GB address space (`prlimit --as`); msolve 600 s wall and 10 GB;
`max-degree` 16 for the native engine (a run that needs more is censored `degree_cap`). The exploratory m = 5 cells
use the same caps; they are reported but excluded from C2's cell-level verdicts if any arm censors on more than
half of the instances.

## 4. Observables (per call)

`arm, cell, prime, instance, label, algebraic_count, nvars, nequations, max_input_degree, solving_degree,
degree_reached, basis_size, field_multiplications (solve), field_multiplications (extraction), cpu_s, wall_s,
max_rss_kb, censored ∈ {no, timeout, as_rlimit, degree_cap, crash}`, `verified_real_decompositions`,
`unlifted_solutions`, `consistent_with_classifier`; msolve: `work_proxy, max_round_degree, rounds, passes, cpu_s,
distinct_solutions (from -P 1), censored`. Per instance: the set of verified decompositions per arm (sorted,
hashed) for C1. Per cell: classifier enumeration cost (analysis cost, listed separately).

**Work** `w_A(instance)` = native `field_multiplications(solve) + field_multiplications(extraction)`. The msolve
proxy `π_A` is the co-observable. Censored calls carry lower bounds and are excluded from medians (and the
exclusion is counted and reported; a cell with any censored chain-tower call has no C2 verdict).

## 5. Analysis and decision rule (frozen)

For each cell (m, t), each prime, each candidate arm A ≠ chain-tower, over the natural instances (SAT and UNSAT
pooled, and also reported split by label):

- statistic `d_A = median_i log2( w_A(i) / w_chain(i) )`; interval: percentile bootstrap of the median, 20,000
  resamples, level `1 − 0.05/k` with k = number of candidate arms in the cell (Bonferroni); exact sign-flip test
  on the paired log-ratios.
- **permutation control** `d_perm`: chain-tower on the same instance with the leaves in a seeded random order
  (the system is identical up to variable relabelling; a counting engine should give the same work up to tie
  breaking). `D_perm = max_i |log2(w_perm(i)/w_chain(i))|` per prime.
- **supported_within_scope** for arm A in cell (m, t): at every prime of the cell, the interval excludes 0, the
  sign-flip p ≤ 0.05/k, `|d_A| > D_perm`, and the sign agrees across the cell's primes; also the msolve proxy's
  `d` has the same sign at every prime (co-gate). The scope is the measured (m, t, p) cells and these two engines.
- **within_control_distribution:** `|d_A| ≤ D_perm` at every prime.
- **invalid_measurement:** C1 fails on any instance of the cell (any arm misses a verified decomposition that
  another arm finds, or finds one the classifier does not have), or a prime has fewer than 4 valid instances.
- **inconclusive:** otherwise.
- C3: `γ_A(B→2B)` = median over matched instances (same prime rule, same draw index) of
  `log2(w_A(t+1) / w_A(t))`, with the instance spread; reported per arm with the distance to 3.5 as a number.
  A censored call at the larger t makes that γ a lower bound.

The pre-registered expectations (from the F_{p^3} run-001 and the P-256 isogeny-chain record), which the data may
contradict: `mono-*` below `chain-*`; `*-direct` below `*-tower`; `sym-direct` lowest when `2^t` is small relative
to the degree budget; `tree-tower` indistinguishable from `chain-tower` at m = 4. The m = 8 question is not decided
by any outcome here; a strong presentation effect at m ≤ 5 is the reason to re-plan panel A's presentation before
any large host is bought, and no effect is the reason not to.

## 6. Controls and variable ledger

| control | arm | expected | failure means |
|---|---|---|---|
| C1 identical verified relations | all arms, every instance | same decomposition set (semantics-adjusted); planted found | exporter/extractor fault; instance invalid |
| classifier gate | `pkm_m8_panel targets` exhaustive count vs verified decompositions of chain-tower | equal | classifier or solver fault; cell invalid |
| positive | planted instance per (cell, prime) | planted decomposition recovered by every arm | arm invalid in that cell |
| negative | real-UNSAT instances | zero verified decompositions on every arm (algebraic/unlifted solutions may exist and are counted) | false relation: invalid run |
| permutation (noise floor of a counting engine) | chain-tower with seeded leaf permutation | `D_perm` small; recorded | sets the floor for C2 |
| engine cross-check | msolve proxy sign agreement | same sign as native | engine-specific effect, not a presentation effect: C2 inconclusive |
| resultant / rewrite identities | `cargo test` of the instrument (S_{m+1} vs `semaev_higher`, symmetric rewrite, remainder criterion) | pass before any data | exporter fault: nothing runs |
| blinding | `hidden.json` never read by the pipeline; analyst reads the results table only after the whole plan finished | | |

Variable ledger: instrument commit and binary sha256, msolve package sha256, flags, one thread, one host (lscpu
recorded), fixed order, seeds derived from 20261115 and the cell slug, caps fixed, no retry, no cap raised after
seeing results. Confounders isolated: same instance across arms (paired), same engine settings, same prime band,
interleaving of arms within an instance, permutation control for ordering effects.

## 7. Budget, route and stop rules

Local container, 1 thread, one call at a time; expected total 1–4 h (probe on seed 777: m ≤ 4, t ≤ 3 calls take
0.1–3.2 s on both engines; m = 5 and t = 4 unknown, capped). Zero spend. Stop rules: a cell whose chain-tower
censors on more than half of its instances stops the remaining cells of that m (reported). No isolab, no pod.
A follow-up at larger m or on a bigger host is a new protocol version.

## 8. Records and deliverables

`cells/<cell>-p<k>/{cell.json,targets.json,hidden.json}`, `systems/<cell>-p<k>/<instance>/<arm>.{ms,system.json}`,
`plan.json` (frozen order), `results/<stamp>/{rows.jsonl,calls/*,analysis/}`, `RESULTS.md` (tables per §4–§5,
C1/C2/C3), `figures/presentation_work.svg|png` (per-arm work vs B, log y, per m), `REPORT.typ`
(`REPORT.pdf` if Typst is permitted), bench record (`tools/benchrec.py`), raw tarball to S3 by SHA-256, pcc claims
for the cell derivation, C1 and each C2 verdict, session-log entries, PR with auto-merge. `.resume/RESUME.md` filled
before the run.

## 9. Freeze

Frozen by the commit recorded in `FREEZE.json` (written in the following commit with the SHA-256 of this file,
`protocol.json`, `plan.json`, the instrument binary and `cells/MANIFEST.json`). Changes after the freeze go to
`AMENDMENTS.md` with a timestamp and a disclosure of what had been seen.
