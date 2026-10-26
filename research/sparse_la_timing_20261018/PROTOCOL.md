# Protocol: first measurement of the deferred sparse relation matrices in `ic.pipeline`

Frozen 2026-10-18, before any measured session. Owner request: IC leads follow-up,
lead 2 (`research/ic_leads_review_20261018/README.md` §3.3, §3.4). Code under test:
PR #2162 (merge `c8613fe57`), measured at `main` `abee07ee7` plus this directory only.
The frozen specs are `specs/SPEC-fp4-p29.json` and `specs/SPEC-prime-scaling.json`;
their SHA-256 are in the task card and in `receipts/`.

## 1. Question

Does any of the relation matrices #2162 made reachable (`sparse-cado` =
`filter+wiedemann`, `filter+lanczos`, `wiedemann`, `lanczos`) lower the
end-to-end `S` (every phase, cold) of the current best verified IC pipeline
row, on the same instances, with verified answers? What does each cost in the
linear-algebra stage alone, and how does that scale with matrix size?

## 2. Baselines (AGENTS.md "Success criterion")

| workload | baseline row | protocol / receipts | end-to-end `S` |
|:--|:--|:--|:--|
| E(F_{p⁴}) `k = 4` ladder, `fp4_torsion p = 29 seed 1 max_cofactor 16` (slug `icv1-fp5k4-tm926-7bc5b359`, registered in this PR) | `ic-d4-chains` of `research/gaudry_quartic_d4_cost_20261014/sessions/k4-p29-d4cost`: `ic.pipeline`, `fghr-y-line:zero=1`, `gaudry-d4:macaulay=26,weighted=1,chains=1,sparsest=1,replay=1,charge_unsolved=1`, `max_trials 20000`, **`linalg = incremental-gauss`** | that study's `PROTOCOL.md` (commit `2be9ad3d7`), session + audit in its `sessions/` | 1,653,062.36 (workload `Wc87e324798ff`), 1,779,983.12 (`W1016932647b2`) |
| Genus-3 after cubic GHS | round 4's `r2:lpsgebw2` (+C1, +C1C2), `examples/ic_improve4_01_genus3_rel.rs`, `research/ic_improve4_20261015/` | that round's `PROTOCOL.md`, `results/` | relative only (0.715× of `r2:lpsgebw2` at q = 2^22 for C1+C2) |

The leaderboard (`docs/ic/leaderboard.json`, tables A–C) carries no F(p⁴) or
genus-3 row; the rows above are the best verified pipelines in the files it
and the leads review cite.

**Genus-3 is not reachable from `ic.pipeline`.** ecbench has no genus-3
curve kind; the genus-3 pipeline is a standalone example with its own ten
linear-algebra arms (`inc`, `wied`, `sgelz`, `sgewd`, …, `lpsgebw<m>`),
already compared in `ic_improve2_20261014` item 01. The four names under test
cannot be applied to it without new wiring. It is recorded as not measured
(§9), not as a null.

## 3. Arms (identical in both specs; only `linalg` varies among IC arms)

| arm | `linalg` | role | completion rule |
|:--|:--|:--|:--|
| `rho-neg` | — (`rho.negation`) | matched reference | — |
| `sg` | `structured-gauss` | incumbent named by the task | stop at target pin |
| `sg-aa` | `structured-gauss` | A/A control | stop at target pin |
| `ig` | `incremental-gauss` | the baseline row's configuration (gate 0 on F(p⁴)) | stop at target pin |
| `igfr` | `incremental-gauss-full-rank` | positive control | full rank |
| `sparse-cado` | `sparse-cado` | candidate | full rank (deferred) |
| `filter-wiedemann` | `filter+wiedemann` | candidate (alias of `sparse-cado`: second A/A) | full rank (deferred) |
| `filter-lanczos` | `filter+lanczos` | candidate | full rank (deferred) |
| `wiedemann` | `wiedemann` | candidate | full rank (deferred) |
| `lanczos` | `lanczos` | candidate | full rank (deferred) |

In the prime spec every IC arm exists once per factor-base size, suffixed
`-0128 … -4096`.

## 4. Workloads

- **S1, F(p⁴) ladder** (`SPEC-fp4-p29.json`): the baseline's curve, its two
  targets (`target_seed 202610134029`) and its measurement seed
  (`202610134030`), so every arm sees the baseline's relation stream.
  `rounds 1, warmup 0, order shuffle`.
- **S2, LA scaling** (`SPEC-prime-scaling.json`): `prime_search bits 30 seed
  59297` (`icv1-fp30-t31711-b403411a`), 6 targets (`target_seed 20261018`),
  `prime-abscissa:size = 128, 256, 512, 1024, 2048, 4096`, `mitm:negation_folded=1`.
  One curve and one target set; the factor-base size (columns) is the only
  other variable. This is a stage diagnostic for scaling with matrix size; it
  has no leaderboard baseline and is not a pipeline-improvement test.

Every arm in a workload shares the algorithm seed (ecbench derives one seed
per workload and round), so the relation stream is common to all IC arms
and the pairing is exact.

## 5. Units and pricing

- **Primary:** ecbench `S` = GAE / √r, all phases, cold. Counts, not time.
- **Unpriced native work** (AGENTS.md §5a): the relation matrices' `row_ops`
  (and each arm's other `*_uncharged` / native counters, rho's
  `canonicalisations_uncharged` included) are priced at **0, 0.1 and 1 GAE**
  per operation on every arm alike with `ecbench band`. Two arms are ranked
  only where the order holds at all three weights; otherwise "unresolved".
- **Known under-count:** the deferred solvers' `row_ops` omit the `block × block`
  approximant / Lanczos-recurrence arithmetic (as `#2162` states). Their LA
  count is therefore a lower bound; the dense and structured methods' column
  scans are likewise uncounted. A sparse arm that *loses* on `row_ops` loses at
  least that much; a sparse arm that wins on `row_ops` is reported with this
  caveat and the wall cross-check.
- **Secondary:** wall time per phase. This VM is shared (4 vCPU): isolation
  tier **L0**, descriptive only, never ranked.

## 6. Gates and controls (all must pass, else no result)

- **G0 (baseline reproduction).** On S1, `ig`'s `S`, `relations_found`,
  `targets_tried` and recovered logs equal the baseline session's records on
  both workloads.
- **Verification.** Every IC and rho run `verified` (`[d]G = Q`, equal to the
  planted `d`); `ecbench verify --replay-all` OK with every replay identical.
- **A/A.** `sg-aa` equals `sg` exactly in every count on every workload;
  `filter-wiedemann` equals `sparse-cado` exactly in every count.
- **Positive control.** Prediction: `igfr` collects at least as many relations
  as `ig` on every workload, and strictly more on every S2 workload (a pilot,
  §8, saw 310 vs 1,625 relations at size 512). If `igfr` is not strictly
  above `ig` on S2, the detector for the completion-rule effect has failed
  and the session is not interpreted.

## 7. Analysis (fixed now)

1. Per workload and arm: `S` by phase (`factor_base`, `oracle_setup`,
   `relations`, `linear_algebra`, `verify`), `relations_found`,
   `targets_tried`, LA `row_ops`, LA wall.
2. **End-to-end:** totals ratio `Σ S_cand / Σ S_base` over paired workloads,
   paired bootstrap 95 % interval (`ecbench compare`), at each band weight
   (`ecbench band`). Base = `ig` (baseline row) and, separately, `sg`
   (incumbent). An **improvement** is a candidate whose interval is below 1 at
   every band weight on S1 (`ig` as base), every answer verified.
3. **LA stage alone:** ratio of LA `row_ops`, candidate / `sg`, and candidate /
   `igfr` (the like-for-like full-rank matrix), per workload. A stage-only
   win (LA lower, end-to-end not) is reported in its own section.
4. **Scaling:** per arm, fit `log row_ops` and `log relations_found` against
   `log columns` over the six S2 sizes (least squares, slope with bootstrap
   interval over targets), and report the fit beside the dense/structured arms'.
5. **Gap to matched rho:** per row, `S_arm / S_rho-neg` at each band weight,
   reported as distance remaining, not as a gate.

Inadmissible after freezing: changing workloads, targets, seeds, sizes, arms,
weights, the base arm, or the improvement rule; dropping a failed or timed-out
run (a timeout is reported as censored).

## 8. Pilot (disclosed, excluded)

Before freezing, two feasibility sessions on S2's curve (one target, sizes
512, 2048, 4096; scratch space, not committed) set sizes and timeouts. They
showed: deferred arms 1,625 rows vs 310 for the target-pin arms at size 512
(end-to-end `S` 100.8 vs 25.7); at 4096 columns `incremental-gauss-full-rank`
1.4·10⁵ `row_ops`, `filter+lanczos` 2.1·10⁵, unfiltered Lanczos / Wiedemann
1.7·10⁹ / 2.7·10⁹; peak RSS 0.72 GB, longest run 10.5 s. These shape the
predictions in §6 and are not results.

## 9. Stop conditions and scope

- Any gate failing stops interpretation; the failure is reported.
- A run hitting its timeout (S1 3,600 s, S2 900 s) is censored and reported.
- No rented compute. This container only.
- Genus-3: not measured (§2). Its next step is wiring the pipeline's
  relation-matrix registry into the genus-3 example, or a genus-3 curve kind
  in ecbench.
