# P2 follow-up 1: replication test of the c32A, m=3, curve-80 yield flag (frozen before any data)

Status: written 2026-11-02 by the experiment director, before any measurement of
this follow-up. It is a **new protocol**, not an amendment of A1: the pinned P2
cards, `PROTOCOL.md` and `protocol.json` (v2) are unchanged and stay valid.
Machine-readable record: `protocol_followup1.json` (`experiment.protocol/v1`,
frozen with the same `experiment_protocol.py freeze` tool). Design table (curve
indices, iso models, seeds): `followup1_focus.json`. Parent: `PROTOCOL.md`
section 6 ("curve effect suspected ... triggers a follow-up protocol (not a
claim): rerun at fresh seeds, then test which of n, the smallest-abscissa
structure or the CM field explains it"). This protocol is the first half only:
the rerun at fresh seeds, plus the control that separates a curve effect from
the smallest-abscissa presentation. The mechanism test (n, CM field) is not
run here; it needs its own protocol and only follows a "curve effect" verdict.

## 1. Trigger and the data already seen

`runs/c32A_m3/analysis.json` (sha256 `b4e907dd0328bc41a6df402fcb75c12e5225924ef5fbb6923ad2ac348f03d1e6`,
committed, card `CARD-isogeny-ic-yield-p2-c32A-m3-20261102-3`), yield metric:

| quantity | value |
|:--|--:|
| curve (index in `classes.json`, j) | 80, 2659937662 |
| curve 80 pooled yield / grand mean (0.033006) | 0.0353999 / 1.0725 |
| relations / attempts of curve 80 | 3378 / 95424 |
| max abs z (perm null q95, perm p) | 4.147 (3.534, 0.0005; Bonferroni alpha 0.0025) |
| yield ICC (perm p) | 0.0117 raw (0.079, not significant) |
| iso control (RMS log ratio, max ratio) | 0.023, 1.042 |
| `sigma2_within` against the binomial `y(1-y)/A` (A = 5964) | 5.373e-6 against 5.352e-6 (ratio 1.004) |

Descriptive facts from the main session on existing data: in c32A m=2 curve 80
has ratio 1.010, z 0.59, rank 90/128. Curve 80 has no iso model in
`classes.json` (the 8 iso bases are 34, 96, 35, 58, 32, 86, 123, 40).
Curve 80 was selected **because** it is the maximum of 128 z-scores, so its
1.0725 is biased upward (winner's curse); this follow-up uses fresh data only
and never pools the P2 cell with its own result.

## 2. Question, claim class, prediction

Question. Does curve 80 of c32A yield more relations per attempt than the other
curves of its class under the m=3 prime-abscissa mitm pipeline on fresh seeds,
and if so, does the excess follow the curve (j) or only the smallest-abscissa
model presentation?

Claim class: comparative / statistical, one pre-specified test. It licenses only
the sentence in section 10.

Preregistered prediction (decisive): **H0, no replication.** The P2 flag is a
selection artefact. Rationale: P2's own controls (iso RMS 0.023 against a binomial
expectation of 0.025, within-cell variance equal to binomial) show no
presentation or overdispersion noise at the scale of 7%; m=2 shows no excess;
the ICC is not significant. Alternatives the data can show are in section 8.

## 3. Design (every number fixed here)

| item | value | source |
|:--|--:|:--|
| class, m | c32A (D = -84171, h = 128, p = 4127505313, n = 4127376823), m = 3 | `classes.json`, P2 flag |
| factor base | `prime-abscissa:size=470` (main), 499 (pc arm) | `runs/c32A_m3/spec.json` (rule of `PROTOCOL.md` section 3) |
| attempts A per run | 5964 (`max_trials`) | same spec; the spec generator is run **without** `--relations`, so R is its default 200 |
| expected relations per run | 5964 x 0.033006 = 196.9 | |
| oracle, linalg | `mitm:m=3`, `incremental-gauss-full-rank` | same |
| rounds, warm-up, order | 4, 1, `alternate` | same |
| targets per curve T | **12** (48 cells per curve = 4 rounds x 12 targets) | power, section 7 |
| `--aa` | 0 (as at every 32-bit cell: a deterministic duplicate) | `PROTOCOL.md` section 3 |
| isolation | L0, `ALLOW_BUSY=1` | route of amendment A1 and its note 10.1, inherited unchanged |
| **fresh seed S** | **363589115**: `--seed 363589115` gives `target_seed` 363589115 and `measurement.seed` 116 | section 4 |
| workloads | 24 curves (below), each 12 targets | |

Workloads, in this order: (1) **primary**: curve 80 (j 2659937662, the `ellfromj`
model of `classes.json`); (2) **15 comparators**: curve indices 7, 11, 19, 32, 34,
35, 40, 57, 58, 86, 96, 101, 104, 105, 123; (3) **8 isomorphic models of curve
80**, u_k from the fixed stream below. Same N, A, R and instrument on every
workload. Cells (main arm): 24 x 48 = 1152 cells, 1152 runs per arm after the
warm-up round, 2880 runs in total with the `m3-pc` arm (24 x 12 x 5 x 2).

## 4. Fixed rules, written before data (all in `followup1_focus.json`)

- **Fresh seed.** `Prg('p2-followup1-seed-c32A-m3-curve80').next() mod 10^9`
  = 363589115, with p2tool's `Prg` (u64 little-endian of the first 8 bytes of
  `sha256('<seed>|<counter>')`, counter from 0). It differs from the P2 cell's
  `target_seed` 20261102 and `measurement.seed` 103 (new 116), so targets and
  algorithm seeds are fresh. Validity condition (checked by the assessor from
  the spec): `target_seed` = 363589115 and `measurement.seed` = 116.
- **Comparators** (15, a fixed rule, no data used): the 8 iso-base curves of
  `classes.json` (34, 96, 35, 58, 32, 86, 123, 40) plus 7 by the seeded draw
  `Prg('p2-followup1-comparators-c32A-curve80')`: `i = next() mod 128`, skip 80,
  the 8 iso bases and repeats, stop at 7 draws. The draw gave 101, 11, 104, 7,
  57, 105, 19. No yield of any comparator was read to fix this rule.
- **Control arm: curve 80's u-scaled models.** `Prg('p2-followup1-iso-c32A-80')`:
  for k = 0..7, `u_k = 2 + next() mod (p - 3)`, `(a,b) -> (u^4 a, u^6 b)`,
  `G -> (u^2 x, u^3 y)`, the rule of `p2tool build` step 6. u_k =
  2607263620, 546563133, 864156487, 90595761, 861977280, 1712401383,
  1056100772, 1234643133 (models, `a`, `b` and `G` in the focus file; the
  director checked in exact integer arithmetic that each is on the curve, that
  n G = O and that j = 2659937662). Each model has its own smallest-abscissa
  factor base, and nothing else differs from curve 80.

## 5. Instrument, run plan and the one thing that does not exist yet

Instrument as `PROTOCOL.md` section 3 (`ecbench run`, `ic.pipeline`,
`prime_explicit`). Planned commands, copied into the card by the director once
the two binaries are built and hashed (a card needs their SHA-256):

1. `p2tool spec --classes classes.json --class c32A --m 3 --y 0.1 --isolation L0 --focus followup1_focus.json --targets 12 --rounds 4 --aa 0 --seed 363589115 --out <run>/spec.json`
2. `ecbench run --wait --allow-busy --spec <run>/spec.json --out <run>/session`
3. `ecbench verify --dir <run>/session --replay 12 --out <run>/audit.json`
4. `p2tool analyze-focus --spec <run>/spec.json --focus followup1_focus.json --classes classes.json --session <run>/session --out <run>/focus_analysis.json`

`p2tool spec --focus` and `p2tool analyze-focus` do not exist in the P2 build
(`p2tool spec` selects curves only by `--first K` and `--iso K`, always the
class's own 8 models; `analyze` keys curves to `classes.json` and has no
single-curve test). They are small code additions specified in section 6 and
written by a builder. The builder's commit, and the SHA-256 of the new
`spec.rs`, `analyze_focus.rs` and the two binaries, are filed in a note
(F1.1, before the first data point) that changes nothing in this document; the
frozen `analyze.rs` (sha256 `f6cc7286daae471bc40e5860ed8673f1e1676b9fadce3199b8fe67ca8d167678`)
is not edited. `ecbench` is invoked unchanged; its source is not modified.

## 6. Interface of the code additions (the specification the builder follows)

`p2tool spec --focus <followup1_focus.json>`:
- requires `--class` = `class_id`, `--m` = `m`; rejects `--first` and `--iso` with `--focus`;
- for the primary and each comparator index, takes `a, b, gx, gy` from `classes.json`
  `curves[i]`; for each iso model takes `a, b, gx, gy` from the focus file;
- re-verifies every workload in Rust before writing: `(gx,gy)` on the curve, `n G = O`,
  and for the iso models j(a,b) equals `primary.j`; stops with a nonzero exit on any failure;
- writes `workloads.curves` in the order of section 3 (24 entries), the same
  `prime_explicit` fields as today, and the same arms (`m3-main`, `m3-pc`) with
  the existing N, N', A formulas. Expected stderr line: `|F|=470 (pc 499) ... attempts=5964 curves=24`.

`p2tool analyze-focus` (statistics, section 7 and 9): reads the session's
`records.jsonl` by the same keys as `analyze.rs` (counters `targets_tried`,
`relations_found`; curve identified by `(a,b)`; arm by name; warm-up rows
excluded exactly as `analyze.rs` does), and writes `focus_analysis.json`. Its
splitmix64 seed is 20261102, permutations 20000, and every number below is
written to the JSON with the formula inputs (`K`, `A`, cell counts).

## 7. Statistics (frozen)

Groups of main-arm cells: **P** = curve 80, **C** = the 15 comparators pooled,
**I** = the 8 iso models pooled. For a group g: `A_g` = sum of `targets_tried`,
`K_g` = sum of `relations_found`, `y_g = K_g / A_g`. Ratios are `r = y / y_C`.

- **T1 (the one pre-specified test): curve 80 against the pooled comparators.**
  `l = ln(y_P / y_C)`, `SE = sqrt(1/K_P + 1/K_C)` (delta method, binomial counts; the
  P2 data show the binomial within-cell variance, ratio 1.004), `z = l/SE`,
  `p_z` two-sided normal. `p_perm`: 20000 random reassignments of the 48 curve-80
  cell labels among the 48 + 15 x 48 cells, statistic `|l|`, add-one p. **Decision
  p = max(p_z, p_perm)**. Reported with the 95% interval `exp(l +- 1.96 SE)`.
  **Replicated** := `r_P >= 1.05` and decision p < 0.05.
- **T2: iso models against the comparators**, same form with I for P (`z`-based).
  **Present in iso** := `r_I >= 1.05` and `p_z < 0.05`.
- **T3: curve 80 against its iso models**, `ln(y_P / y_I)`, `SE = sqrt(1/K_P + 1/K_I)`.
- Per-iso-model ratio to C and the RMS of ln(y_I,k / y_P) are reported
  (descriptive; a presentation spread beyond binomial noise is visible there).
- **Positive control (power of the machinery).** The `m3-pc` cells of curve 80
  (factor base 499, expected yield 1.2 x) against the main cells of C, same
  `l`, `z`. The run is **informative only if** this ratio is >= 1.10 with
  `p_z < 0.05`; otherwise the verdict is "not informative" whatever T1 says.
- **Negative control / null calibration.** Each comparator `c` against the other
  14 comparators pooled, `z_c` as in T1. Reported: the number with `|z_c| > 1.96`
  (expectation 0.75 of 15), max `|z_c|`, and `D = mean(z_c^2)`. The run is
  informative only if `D <= 2.0` (under the binomial model D has mean 1, sd 0.37;
  2.0 is a 0.3% event). If `D > 2.0` the binomial SE is declared unreliable and
  the verdict is "inconclusive, overdispersed".
- Counted metrics only decide. `rel_gae_per_attempt` and wall are not computed.

## 8. Decision rule (exhaustive, applied in this order)

1. Positive control not met, or `D > 2.0`: **not informative / inconclusive** (reported with the numbers; no replication claim either way).
2. Not (`r_P >= 1.05` and decision p < 0.05): **not replicated: chance (winner's curse)**; the P2 flag for c32A m=3 stands as a false positive. This does not exclude a true effect below 5%; the interval of T1 is reported as the size still compatible.
3. Replicated, and iso present (`r_I >= 1.05`, `p_z < 0.05`): **curve effect, escalate to the mechanism test** (n, smallest-abscissa structure, CM field). Not a claim about any mechanism.
4. Replicated, `r_I < 1.05`, and T3 `p_z < 0.05` with `y_P > y_I`: **factor-base presentation effect** (the smallest-abscissa set of curve 80's `ellfromj` model, not the curve).
5. Anything else (replicated, neither 3 nor 4): **inconclusive**, numbers reported.

Nothing here is a claim about ECCp-131, security, speed, a cross-method cost or
an improvement over the baseline; no rho reference or section 5a band applies
(one method on curves of one class). A false positive or a null is a closed lead.

## 9. Power (computed before data, all inputs from the P2 cell)

Inputs: A = 5964, grand mean yield 0.033006, so 196.85 expected relations per
cell; 48 cells per curve: `K_P` = 9449, `K_C` = 141732, `K_I` = 75590;
`SE(T1)` = 1.06%, `SE(T2)` = 0.45%, `SE(T3)` = 1.09%. Test two-sided, alpha 0.05.

| quantity | T = 12 (chosen) |
|:--|--:|
| power, T1 `p < 0.05` alone, true +7% | > 0.999 |
| power, **T1 replicated** (`r >= 1.05` and `p < 0.05`), true +7% | **0.962** (0.977 at the observed 7.25%) |
| power, replicated, true +5% | 0.50 (the 1.05 floor is binding) |
| false replication under H0 (true ratio 1) | 2.2e-6 (the floor, not alpha, binds) |
| power, T2 iso present, true +7% | > 0.999 |
| power, T3 `p < 0.05` at a true presentation-only 7% | > 0.999 |

The requested "power >= 0.9 for a 7% deviation of one curve at alpha 0.05" is
met by the p-only test already at T = 4 (0.957) and by the stricter replicated
rule (with the 5% floor) at T = 7 (0.895 at 6, 0.926 at T = 8); T = 12 keeps
margin against the winner's curse (a true effect below the observed 7.25%).
Monte Carlo (40000 draws per scenario, normal approximation of the binomial
counts, T = 12) of the whole decision rule (steps 2 to 5):

| true state | not replicated | curve effect | presentation | inconclusive |
|:--|--:|--:|--:|--:|
| no effect | 1.000 | 0 | 0 | 0 |
| curve effect +10% (also in iso) | 0 | 1.000 | 0 | 0 |
| curve effect +7% | 0.033 | 0.967 | 0 | 0 |
| presentation-only +7% | 0.032 | 0 | 0.968 | 0 |
| curve effect +5% | 0.500 | 0.275 | 0.018 | 0.207 |

The last row is the stated limit: a true effect near the 5% floor is classified
at random, as the floor was chosen to ignore effects below it.

## 10. What a result licenses

Exactly: "On fresh seeds, curve 80 of c32A has m=3 prime-abscissa mitm yield
ratio r (95% interval) against 15 other curves of the class, its 8 u-scaled
models have ratio r_I, and the pre-specified rule classifies the P2 flag as
<verdict>." It is a structural statement about this pipeline on this scaled
analogue, and not about ECCp-131, security, speed or the index-calculus baseline.
m=2 and other classes are not retested; the P2 multiplicity family (20 tests,
alpha 0.0025) is not reopened, and this follow-up adds one pre-specified test.

## 11. Cost and route

Measured per-run cost in the P2 c32A m=3 cell: 4949 s for 5440 runs = 0.91 s
per run on the shared 4-core host (L0, load). This follow-up has 2880 runs:
about 2600 s (44 min) for `ecbench run`, 12 replays (< 1 min), analysis
(< 1 min): **expected 45 to 60 minutes**, background card, per-command timeouts
spec 120 s, run 7200 s, verify 3600 s, analyze 900 s.
Route: the local backend of amendment A1, L0, `ALLOW_BUSY=1`, counted metrics
decide, wall descriptive only. `ecbench run` holds the per-host benchmark lock
and `--wait` queues behind the running P2 queue; the follow-up is run **after the P2
queue has finished**, from a fresh detached worktree at the builder's commit
(`git worktree add --detach /home/user/crypto-p2fu <commit>`; the pinned P2 worktree
`/home/user/crypto-p2run` stays untouched). A different route (isolab-eks) needs a
validated request and tested image from the main session and a note here first.

## 12. Variable ledger and stopping

All variables as `PROTOCOL.md` section 7, except: seeds are the fresh S above;
the curve set is the 24 workloads above; T = 12. Stopping: fixed n (1152 cells
per arm), then stop; a session that does not complete is reported as not run,
never filled; an `ecbench verify` failure invalidates the session. No seed,
comparator, threshold or T is changed after a look; any change is a new
protocol filed before the data it affects.

## 13. Notes recorded now

- **N1 (R discrepancy, not repaired).** `PROTOCOL.md` section 3 and the README state
  R = min(100, 0.6 N), but the P2 spec generator's default is `--relations 200` and
  the executed spec has `max_trials` 5964 = ceil(200 / 0.03354). This follow-up copies
  the executed spec (A = 5964), as "same N, A, R" requires; it does not pass `--relations`.
- **N2.** The P2 analysis file has no single-curve test; T1 is therefore new code
  (section 6), and the permutation p-value uses the same splitmix64 generator as `analyze.rs`.

## 14. Hashes (sha256)

| file | hash |
|:--|:--|
| `classes.json` | `7e79c642399cdda52e0e6666123cf8abcad27dbf68c9ff7dda14f0d3316adb47` |
| `followup1_focus.json` | `0340494ad36574f7d27283617096cdfe4a9bf8160cad3810c0405484f73a92d7` |
| `PROTOCOL.md` (P2 parent) | `9e3a39f398c91128eb564a74f8fa71dedfba336debd1500ea12a8d03bce354d6` |
| `protocol.json` (P2 v2) | `8b6fc89c3c60352b7e5203bcd1120cc10babf28ce91c59b3fce77c12976a14e0` |
| `runs/c32A_m3/analysis.json` (trigger) | `b4e907dd0328bc41a6df402fcb75c12e5225924ef5fbb6923ad2ac348f03d1e6` |
| `runs/c32A_m3/spec.json` (A, N, seeds copied) | `9efa85db1cc8470d49cf0bf30d4c8d75dc491ccf2adbf3b4186495b887aa1926` |
| `p2tool/src/analyze.rs` (unchanged) | `f6cc7286daae471bc40e5860ed8673f1e1676b9fadce3199b8fe67ca8d167678` |
