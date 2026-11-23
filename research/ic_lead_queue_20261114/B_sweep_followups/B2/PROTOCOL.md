# B2: full-group kangaroo against matched rho on the prime medium tier (`LEAD-RHO-kangaroo-interval`)

**Status: frozen (see Amendments); blockers of section 13 resolved in section 14b.** Written 2026-11-14, before any
data of this study. After the freeze, results go into `README.md`, `analysis/` and `reports/`, never into this
file. Any later change is a dated amendment at the end.

Every number quoted from the repositories is `[pcc:unverified]`. Evidence classes: *cited* (a committed file),
*derived* (arithmetic on cited numbers, shown), *heuristic* (a model), *development check* (local, not a result).
This study measures a rho-family denominator class. It is not an attack, makes no `vs_rho` claim for any index-calculus
row and reports no pass or fail.

## 1. Question

The lead says that on a prime-order group a two-kangaroo solver with distinguished points costs more group
operations than matched r-adding rho when the interval is the whole group; its advantage is confined to short
intervals (`research/lead_sweep_20261110/LEAD-RHO-kangaroo-interval/PROTOCOL.md`). The sweep run was a stdlib toy with failed
controls. The atlas records the method at 2.203 x floor, from small sizes (`research/prime_field_lead_assessment_20261107/atlas_survey.md`),
and no sealed prime medium-tier bound exists for it, although `kangaroo.vow_v2` is registered
(`src/cryptanalysis/ecbench/methods.rs`). **Question.** On the six 33- to 41-bit prime-order curves of the medium-tier
studies, what is the paired ratio R = S(`kangaroo.vow_v2`) / S(`rho.plain`) at w = 0, and is its 95% interval inside
[1.3, 2.4]? Secondary: the same against `rho.negation`.

## 2. Hypotheses

- **H1 (the lead's).** The pooled 95% interval of R = S(kang) / S(rho-plain) lies inside [1.3, 2.4] (rule
  `dominated_supported`, section 8).
- **H0.** Kangaroo is not dominated on the full group: the interval of R contains 1 or lies below it.
  (Outcomes between the two are named in section 8.)

## 3. Prediction and its source

**Predicted: R about 1.5, expected range 1.4 to 1.65, so H1.** Against `rho.negation`: about 2.06.

- ecbench declares the expected S of `kangaroo.vow` as 2.0 (`expected_s`, `src/cryptanalysis/ecbench/methods.rs`) and
  rho.plain's floor is sqrt(pi/2) = 1.2533: 2.0 / 1.2533 = 1.60 (derived).
- Measured version: kangaroo 2.203 x floor (atlas, floor sqrt(pi/4) = 0.886, so S about 1.95) and rho.plain
  1.487 [1.392, 1.577] x the negation floor (`research/prime_medium_rho_cpc_20261109/RESULTS.md`, S about 1.32):
  2.203 / 1.487 = 1.48 (derived). Against `rho.negation` at 1.067 x floor: 2.203 / 1.067 = 2.06 (derived).
- In-session control values: rho.plain / rho.negation 1.443 [1.311, 1.589] (job M1 of the same study) and
  bsgs.textbook / bsgs.negation 1.489 [1.463, 1.516] (job PC, same file; the confirmatory study read 1.4639 [1.4438,
  1.4863], `research/prime_medium_rho_p4_confirm_20261108/RESULTS.md`).

## 4. Falsifier

H1 is killed if the pooled 95% interval of R reaches below 1.3 or above 2.4, with every gate of section 8 passed.
If its lower end is at or below 1.0 the atlas reading "dominated" is withdrawn (verdict `unresolved` or
`kangaroo_cheaper`, section 8). Nothing is concluded from a run in which a gate fails.

## 5. Arms and controls

Job K1 (`spec-K1.json`, six curves):

| arm | role | method | purpose |
|:--|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` | matched rho, the secondary denominator |
| `rho-plain` | baseline | `rho.plain` | the lead's own comparator, the denominator of R |
| `kang` | candidate | `kangaroo.vow_v2` | the numerator of R |
| `rho-neg-aa` | control (A/A) | `rho.negation` | same method, same seeds |
| `kang-aa` | **negative control and A/A** | `kangaroo.vow_v2` | same method, same seeds: paired counts must be identical, so the expected effect is exactly nothing |

Positive controls (the instrument must see an effect of this size):
- **In session (K1):** `rho-plain` against `rho-neg`, expected 1.44 (cited above), the same estimator and workloads as R.
  It shows that this harness resolves a ratio of 1.3 to 1.6 against 1.2 with this sample.
- **Separate job PC (`spec-PC.json`, same six curves, 16 fresh targets per curve):** `bsgs.textbook` against
  `bsgs.negation` (arms `bsgs-tb`, `bsgs-neg`, A/A `bsgs-neg-aa`), expected about 1.49: the only registry pair with an exactly
  derived ratio (1.5 up to O(log r / sqrt r) of set-up, `methods::expected_s`), the same layout and estimator as K1.

All parameters are at their registered defaults (`cap_multiple 64`; `PLAN.txt`). One binary serves both jobs.

## 6. Curves, targets, rounds, seeds

- **Curves.** The six curves of `research/prime_medium_tier_rho_20261107/curves.json` (sha256
  6ccc0acb15d99a5593fcfce6b9dc57ddfd6141f87b35e2a96f41043569ad3680), the list used by `prime_medium_rho_cpc_20261109`:
  `icv1-fp33-t65189-eee04389` (log2 r 32.37), `icv1-fp33-tm43469-ac81a6df` (32.74), `icv1-fp37-tm218953-61268c9f` (36.03),
  `icv1-fp37-t160405-998ed0ab` (36.43), `icv1-fp41-tm405553-81153b6e` (40.44), `icv1-fp41-tm886935-baa24912` (40.81)
  (`PLAN.txt`, all `registered=true`). They are reused, not drawn, so there is no curve choice to make; only the
  targets are fresh.
- **Targets.** Planted targets, 24 per curve in K1 (`target_seed` 262101475107005) and 16 per curve in PC
  (`target_seed` 21344195110516). Seeds: `seed(tag) = int(SHA-256(prefix + tag)[0:12 hex], 16)` with prefix
  `ic_lead_queue_20261114/B2/v1/` (`seeds.json`); both differ from the cpc study's target seeds (4704550062423,
  259652123255010), so the workloads are new.
- **Rounds.** K1 and PC: `rounds 6`, `warmup 1`, `order alternate`, `isolation_required L2`, `timeout_seconds 300`; measurement
  seeds 64010579100382 (K1) and 35920352509606 (PC).
- **Size.** K1: 6 curves x 24 targets = 144 workloads, 864 measured pairs per arm, 5 arms; `ecbench plan` reports 5,040
  executions with the warm-up round. PC: 6 x 16 workloads, 2,016 executions.

## 7. Precision (from the earlier variance only)

Job M1 of the cpc study had 48 workloads and read rho.plain / rho.negation 1.443 [1.311, 1.589], a half-width of
9.6% (derived). With 144 workloads a workload-cluster bootstrap gives about 5.6% (9.6% x sqrt(48 / 144), derived). Kangaroo's
per-workload variance has not been measured here; allowing 1.5 times the rho.plain variance gives at most 8.4%. At the
predicted R of 1.48 to 1.5 that is an interval of about [1.36, 1.62] at worst and [1.40, 1.58] at best, which clears the
lower bound 1.3. If the true R is below about 1.43 the interval may reach under 1.3; that outcome is named
`dominated_below_range` and is not an error of the design. No adaptive stopping.

## 8. Pricing, estimator, gates and decision rule (frozen)

**Pricing.** The primary reading is w = 0, as in the medium-tier studies. The band weights 0.1 and 1 are reported
(AGENTS.md section 5a, `--weights 0,0.1,1`). Neither kangaroo nor rho records counted table inserts, so the
2026-11-08 default of pricing table work does not change these arms; the primary reading is the default, stated
explicitly with `--price-table-work` and fixed for every call. Unpriced native work (rho.negation's
canonicalisations, kangaroo's restart work) is priced alike at w. Ratios are ranked only where the order holds at all
three weights.

**Estimator.** Native only. `compare` reports the ratio of total cost over the pooled workloads of all six curves, so the
41-bit curves weigh most. R is expected to be size-independent (all arms are sqrt(r) methods), and the per-curve ratios of
means from `ecbench table` are reported as a secondary check. This differs from the curve-equal estimator of the earlier
studies, whose analysis scripts are Python and cannot be reused (AGENTS.md "Implementation language: no Python"); it is
fixed here before data. `B` = 275055567897761.

```
ecbench verify  --dir K1 --replay-all --exit-code
ecbench verify  --dir PC --replay-all --exit-code
ecbench compare --dir K1 --a rho-plain --b kang    --weight 0 --price-table-work --seed B --resamples 4000 --save   # R, primary
ecbench compare --dir K1 --a rho-neg   --b kang    --weight 0 --price-table-work --seed B --resamples 4000 --save   # secondary
ecbench compare --dir K1 --a rho-neg   --b rho-plain --weight 0 --price-table-work --seed B --save                  # G3(a)
ecbench compare --dir K1 --a rho-plain --b kang    --weight 0.1 --price-table-work --seed B --save
ecbench compare --dir K1 --a rho-plain --b kang    --weight 1   --price-table-work --seed B --save
ecbench compare --dir K1 --a kang      --b kang-aa --weight 0 --price-table-work --seed B                           # G2
ecbench compare --dir K1 --a rho-neg   --b rho-neg-aa --weight 0 --price-table-work --seed B                        # G2
ecbench compare --dir PC --a bsgs-neg  --b bsgs-tb --weight 0 --price-table-work --seed B --save                    # G3(b)
ecbench compare --dir PC --a bsgs-neg  --b bsgs-neg-aa --weight 0 --price-table-work --seed B                       # G2
ecbench band    --dir K1 --weights 0,0.1,1 --ranks --json K1/band.json
ecbench table   --dir K1 --reference rho-neg
```

`compare --a X --b Y` reports S(Y) / S(X).

**Gates (any failure makes the verdict `inconclusive`; everything is still reported).**
- G1. Both verify calls report 0 problems, every record verified, both sessions `complete`.
- G2 (A/A, negative control). `kang` against `kang-aa`, `rho-neg` against `rho-neg-aa` and `bsgs-neg` against `bsgs-neg-aa`:
  paired counts identical in every pair (ratio exactly 1.0000).
- G3 (positive controls). (a) In K1, `rho-plain / rho-neg` has a 95% lower end above 1.2 and a point estimate inside
  [1.25, 1.60] (cited reading 1.443 [1.311, 1.589]). (b) In PC, `bsgs-tb / bsgs-neg` has a 95% lower end above 1.2 and a point
  estimate inside [1.43, 1.55] (cited readings 1.489 and 1.4639).
- G4. At least 90% of the pairs are complete, and no arm has a censored share above 1% (`cap_multiple 64`).

**Verdict (R = S(kang) / S(rho-plain), pooled, w = 0, interval [L, U]).**
- `dominated_supported` (H1): 1.3 <= L and U <= 2.4.
- `dominated_below_range`: 1.0 < L < 1.3 and U <= 2.4. Kangaroo is dominated by less than predicted.
- `dominated_above_range`: L > 1.0 and U > 2.4 with L >= 1.3.
- `dominated_wide`: L < 1.3 and U > 2.4 with L > 1.0 (dominated, size unresolved).
- `unresolved`: the interval contains 1.
- `kangaroo_cheaper`: U < 1.
Any gate failure: `inconclusive`.

## 9. What each outcome changes

- `dominated_supported`: the atlas row stays `closed_dominated`, now with a sealed medium-tier number, and the
  rho-family frontier can take one more bound (`ecbench bound fit`, a later step). The IC baseline is untouched.
- `dominated_below_range`, `dominated_above_range`, `dominated_wide`: the lead's range [1.3, 2.4] is corrected from data;
  the disposition stays `closed_dominated`.
- `unresolved` or `kangaroo_cheaper`: reopens the atlas row. A full-group method at or below matched rho on a prime
  curve would need the whole validation gate (replay certificate, validator report, matched rho with the band, a second host)
  before it is described as anything.
- `inconclusive`: no reading; report the failed gate; rerun once if the cause is infrastructure.

## 10. Variable ledger

| factor | treatment | how |
|:--|:--|:--|
| instrument | fixed | one binary and image for both jobs |
| curve, size | fixed | the six curves; pooled by total cost, per-curve means reported |
| target | randomized | fresh blocks K1 and PC, seeds in `seeds.json` |
| algorithm seed | randomized | one per (workload, round), shared by every arm (paired) |
| arm order | randomized | `alternate`, balanced over six rounds |
| host, Spot | measured | counts do not depend on the host; an interrupted session is incomplete and rerun once |
| analyst | fixed | the commands of section 8 |
| pricing | crossed | w = 0 decides, 0.1 and 1 reported |

## 11. Execution and cost

- **Route.** Iso Lab on EKS Spot (AGENTS.md section 9a, `isolab-eks`), queue `cpu-l2-spot` (8 vCPU / 32 GiB),
  `c8a.2xlarge`; two jobs (K1, PC), each 4 CPU, 4 Gi, `seconds 3600`, no checkpoint. Request shape:
  `research/prime_medium_rho_cpc_20261109/requests/M1.request.json` with the spec of the job and `job.build` bound to the
  frozen image. Counts-only; wall time is not quoted.
- **Cost (estimate).** K1 about 0.4 CPU-hours: the development check measured the median wall of `rho.negation` at 32.37 bits as
  27.6 ms; scaling by sqrt(r) gives about 0.45 s at 40.8 bits; the five arms of one workload-round sum to roughly 3.3 s there
  (kangaroo taken at about 2 x rho, estimate), over 336 top-size runs plus about 25% of that at 36 bits and little at 33 bits,
  about 1,400 s in all. PC is modelled at 459 s by the confirmatory study (same spec size). Pod provisioning of about 300 s
  per job is added. Spot price USD 0.099 to 0.158 per hour (recorded, cpc study), so well under USD 1; proposed ceiling USD 5
  including one rerun. Local 4-core host: about 25 minutes (L0, development only).
- **Before launch** (main session): commit and push, freeze, `scripts/checkpoint.sh init` and push, a `bench.record/v1`
  record, a Conductor task and scope check.

## 12. Validation gate and records

Counts-only, same-binary replay only. No row for the IC leaderboard or `benchmarks/README.md`, which needs a
`VALIDATION.json` and a second-host replay; none is part of this study. Raw data to S3 by SHA-256 and the progress report
per `docs/benchmark-records.md`.

## 13. Sources checked, and blockers to freezing

- **Knowledge base.** `scripts/research-kb status` on 2026-11-14 printed `research-kb: no executable found; set
  RESEARCH_KB_BIN or put research-kb on PATH`; `RESEARCH_KB_BIN` is unset and the research-kb MCP tools are not exposed to this
  session. Not done. Fallback: repository paths read, no `source_id` recorded:
  `research/prime_medium_rho_cpc_20261109/{RESULTS.md,specs/M1.json,specs/PC.json,requests/M1.request.json}`,
  `research/prime_medium_rho_p4_confirm_20261108/{PROTOCOL.md,RESULTS.md}`, `research/prime_medium_tier_rho_20261107/{RESULTS.md,curves.json}`,
  `research/prime_field_lead_assessment_20261107/atlas_survey.md`, `research/ecbench_frontier_v2_20261023/README.md`,
  `research/lead_sweep_20261110/LEAD-RHO-kangaroo-interval/{PROTOCOL.md,RESULT.md}`, `src/cryptanalysis/ecbench/methods.rs`,
  `src/bin/ecbench.rs`, `research/ic_lead_queue_20261114/B_sweep_followups/EXPERIMENTS.md` (spec 2). A KB pass, including the
  negative form ("kangaroo not dominated by rho on the full group"), is owed before launch.
- **Blockers.**
  1. KB pass owed (above).
  2. `PLAN.txt` was produced with the local `target/release/ecbench` (sha256 73c4f66b...81e7, HEAD 0c2cc1b408), not the
     production image pinned in the cpc requests (ecbench `a18af09f...`, source `dbd587a01a`). Whether that image carries
     `kangaroo.vow_v2` must be checked and `ecbench plan` re-run in the chosen image; arm ids must match `PLAN.txt`
     (`rho.negation` `ECM1hefea4e0ebe79` and the `bsgs.*` ids are expected to match the cpc study's).
  3. The `experiment.protocol/v1` record (`protocol.json`) and its freeze are produced by the main session with the repository tool.
  4. Launch prerequisites of section 11.
- **Not blockers, stated for honesty.** The estimator differs from the curve-equal one of the earlier studies (section 8). The
  kangaroo variance is unmeasured (section 7). The 41-bit pair carries most of the weight in the pooled ratio.

## 14b. Freeze status (updated 2026-11-14, before the freeze; resolves the blockers of section 13)

- **Image and instrument.** The image is the `publish_only` build of source revision
  `0c2cc1b408e33a43e55ee31cb75307e90df9d619` (the same source the local `PLAN.txt` was made at). Its digest and the
  ecbench sha256 are recorded at launch from the build receipt, as an amendment-free input; `ecbench plan` is re-run inside that
  image and the spec ids must equal `PLAN.txt` (`ECS1h73ec912e684a`, `ECS1h838d79f8b2b8`) before launch. This replaces blocker 2;
  the cpc-study image is not used.
- **Knowledge base.** Unavailable in this session as well; the fallback of section 13 stands. This replaces blocker 1.
- **Records.** Bench record `benchmarks/records/20261114T1957Z-b2-kangaroo-medium-tier.json` (status open, created with
  `tools/benchrec.py init`; one record covers jobs K1 and PC).
- **Analysis script.** `analysis.sh` (thin `ecbench` orchestration of section 8, checked against the development smokes; sha256 bound
  in `protocol.json`).
- **Protocol record.** `protocol.json` (`experiment.protocol/v1`), validated with 0 findings, frozen with `experiment_protocol.py freeze`.
  It binds the sha256 of `spec-K1.json`, `spec-PC.json`, `seeds.json`, `analysis.sh` and of this file as it stood before the freeze entry
  under Amendments was appended (the entry records the byte length to check with `head -c`).


## Amendments

(none)

- **2026-11-14, freeze record (not a change to the design).** frozen 10be9a4e1613fc59d4c7d60b9a4b09007b2ded9981509875f1cada8e26bbda86 at 2026-11-14T20:00:39Z, before any data
  (`protocol.json`, `experiment.protocol/v1`, validated with 0 findings; `freeze.commit` a9f62b745f3d2c69fc833d292c440c623832762c is the
  repository HEAD at the freeze, the files themselves are committed afterwards by the main session). The record binds the sha256 of this file
  as it stood before this entry (1a217c40381786a3f3e06c68dc5d5792d0d58cafe4f08ac7c9b740936851fccd, first 16952 bytes, check with `head -c 16952 PROTOCOL.md | sha256sum`), of the spec JSON files, of
  `seeds.json` and of `analysis.sh`. The image digest and the ecbench sha256 are filled at launch from the `publish_only` build
  receipt of 0c2cc1b408e33a43e55ee31cb75307e90df9d619; they are inputs, not amendments.
