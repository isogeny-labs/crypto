# B1: factor-base size above 45 on 20-bit prime curves (`LEAD-ICP-prime-ic-constant-engineering`, residual item)

**Status: frozen (see Amendments); blockers of section 14 resolved in section 15.** Written 2026-11-14, before any
data of this study. After the freeze, results go into `README.md`, `analysis/` and `reports/`, never into
this file. Any later change is a dated amendment at the end.

Every number quoted from the repositories is `[pcc:unverified]`. Evidence classes: *cited* (a committed file),
*derived* (arithmetic on cited numbers, shown), *heuristic* (a model), *development check* (local, not a result).
This study reports measured costs and a distance to matched rho. It makes no `vs_rho` claim and no claim about the
exponent. A "gain candidate" below is a candidate for the validation gate and nothing more.

## 1. Question

The validated 2026-10-29 study moved the 20-bit pair-table oracle's factor-base size from N = 32 to N = 45 and read
S ratio 0.818 [0.773, 0.866] at w = 0 on its holdout (`research/prime_base_oracle_tuning_20261029/README.md`,
`VALIDATION.json`). At 24 and 28 bits the picked N fell at w = 0 and rose at w = 1 and was recorded as relabelling.
The lead's open item is N above 45 at 20 bits. **Question.** On fresh 20-bit prime-order curves, does any N in
{56, 64, 80, 91} have a lower counted cost than N = 45 at all three section 5a weights, w = 0, 0.1, 1, and
what is each arm's distance to matched `rho.negation`?

Why this is new data: the earlier train grid for the `mitm2` oracle stopped at N = 64 on one curve with 8 targets
(`select/band-train-P20.md`), N = 56, 80 and 91 were never run for that oracle, and the 2026-10-29 readings predate
the default of pricing counted table inserts (2026-11-08, section 6).

## 2. Hypotheses

- **H1.** For at least one N in {56, 64, 80, 91}, the pooled paired ratio S(N) / S(N = 45) has a 95% interval whose
  upper end is below 1 at each of w = 0, 0.1, 1, and its per-curve ratio of means is below 1 at each w on both curves
  (rule `gain`, section 9).
- **H0.** No N meets that rule: for every N the pooled interval contains 1 or lies above 1 at some weight.

## 3. Prediction and its source

**Predicted: H0, with the verdict `relabelling` for N = 64.**

- Source (cited): train curve `icv1-fp20-tm1195-26505147`, 8 targets, one round, `select/band-train-P20.md`,
  oracle `mitm2`:

  | N | S, w = 0 | S, w = 0.1 | S, w = 1 |
  |--:|--:|--:|--:|
  | 45 | 10.036 | 11.243 | 22.103 |
  | 64 | 10.519 | 11.413 | 19.464 |
  | 64 / 45 (derived) | **1.048** | **1.015** | **0.881** |

  The order flips across the band, so under section 5a the pair is unranked. These readings use the pre-2026-11-08
  pricing (`cost.unpriced` only). Under the primary reading of this study (section 6) the direction of the change is
  not predicted; the same sign is the working expectation, with no number.
- Scaling (derived from the README decision table): the selected sizes 45, 115, 288 at 20, 24, 28 bits grow by
  115 / 45 = 2.56 and 288 / 115 = 2.50 per 4 bits against 2^(4/3) = 2.52 for N proportional to r^(1/3). That places the
  optimum of the tuned curve at 45 and says nothing against a flat region above it.
- Secondary numbers: matched-rho distance of the validated arm, S(N = 45) / S(rho-neg) = 10.515 / 1.357 = 7.75 at
  w = 0 on the earlier holdout curve (cited, same README); a new curve is read against it, not gated by it.

## 4. Falsifier

H1 is killed if, on the pooled analysis, no N in {56, 64, 80, 91} meets rule `gain` (section 9), with every gate
of section 9 passed. H0 is killed (H1 outcome) only by one N with upper interval ends below 1 at w = 0, 0.1 and 1 and
below-1 per-curve ratios on both curves, with the positive control in range and every A/A exact. Even then the
outcome is a candidate: with four candidates tested, it must be re-measured on a further fresh pair of curves, and
replayed on a second host, before it is called a gain (section 10).

## 5. Arms

| arm | role | method and parameters | purpose |
|:--|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` | matched rho on the same targets, the denominator of every distance |
| `rho-neg-aa` | control (A/A, same seeds) | `rho.negation` | determinism and the wall-noise floor the L2 gate needs |
| `inc-n32` | **positive control** | `ic.pipeline`, `prime-abscissa:size=32`, `mitm:negation_folded=1`, `max_trials=1410236` | the arm the validated study beat; `n45 / inc-n32` must be seen (section 9, G3) |
| `n45` | baseline | same, `size=45` | the validated baseline; the denominator of every candidate ratio |
| `n45-aa` | **negative control and A/A** | same as `n45` | same method, same seeds: paired counts must be identical, so the expected effect is exactly nothing |
| `n56`, `n64`, `n80`, `n91` | candidates | same, `size=N` | the grid; spacing 56, 64 chosen to bracket the old edge of the train grid, 80 and 91 to extend to the sizes tried for the three-summand oracle at 20 bits |

All other parameters are the defaults visible in `PLAN.txt` (`linalg incremental-gauss`, `targets walk`, empty solver).
`max_trials = 1410236` is the frozen holdout value of the earlier study. One binary serves every arm.

## 6. Pricing, fixed before data

- **Primary pricing: the current default**, table work priced: every `ecbench compare` and `ecbench band` call of the
  decision carries `--price-table-work` (the default since 2026-11-08, `band::Pricing::default()`, stated in
  `src/bin/ecbench.rs` and `docs/ecbench/README.md`). The IC oracles here record `table_insert_attempts` behind the
  `counted:` marker, which this reading adds to the unpriced work `U`.
- **Secondary pricing: the pre-2026-11-08 reading**, `--no-price-table-work`. It is reported for every comparison, and it
  is the reading under which the positive control's expected value was measured (section 9, G3). The two readings are
  never mixed within one decision.
- Weights: w = 0, 0.1, 1 GAE per unpriced native operation (AGENTS.md section 5a), `--weights 0,0.1,1` in `band`,
  `--weight W` in `compare`. The ranking of two arms is claimed only where the order holds at all three weights.
- The 2026-10-29 gain was measured under the secondary reading and has not been re-priced; the positive control
  re-prices it as a by-product (reported, ungated, under the primary reading).

## 7. Curves, targets, rounds, seeds

- **Curves.** Two fresh 20-bit curves from `{"kind": "prime_search", "bits": 20, "seed": s}`, with `s` from
  `seeds.json` (rule `seed(tag) = int(SHA-256(prefix + tag)[0:12 hex], 16)`, prefix
  `ic_lead_queue_20261114/B1/v1/`, tags `curve_seed/c1`, `curve_seed/c2`: 211747386521066 and 145169970934094).
  The seeds are fixed by the tag strings, so no curve can be chosen after seeing data. `ecbench plan` resolves them to
  `icv1-fp20-t691-5e9cb4d2` (r = 2^19.12) and `icv1-fp20-tm155-ce03b9ba` (r = 2^19.53) (`PLAN.txt`), neither of which
  is a curve of the earlier study (`tm565-6d7d1d32` holdout, `tm1195-26505147` train). The plan notes that both slugs are not in
  `docs/curves/registry.json`; they must be registered before they are named in prose (AGENTS.md section 11), as with
  the earlier holdout.
- **Targets.** 96 public targets per curve (`hash_to_subgroup_v1`), `target_seed` 242005234523932. One target per
  workload, never pooled across workloads.
- **Rounds.** `rounds 6`, `warmup 1`, `order alternate`, `seed` 270048475138877, `isolation_required L2`,
  `timeout_seconds 300`. Six is the L2 gate's usual setting (at least five, `alternate` needs an even count).
- **Size.** 2 curves x 96 targets = 192 workloads; 192 x 6 = 1,152 measured pairs per arm; 9 arms; `ecbench plan` reports
  12,096 executions including the warm-up round (`PLAN.txt`).

## 8. Sample size and precision (from the earlier variance only)

The earlier holdout had 48 workloads x 2 rounds and read 0.818 [0.773, 0.866] at w = 0, 0.798 [0.752, 0.847] at
w = 0.1, 0.726 [0.677, 0.778] at w = 1: half-widths of 5.7%, 5.9%, 7.0% of the ratio (derived). Half-widths scale as
1 / sqrt(workloads) for a workload-cluster bootstrap, so 192 workloads give about 2.9%, 3.0% and 3.5% (derived,
assuming the same per-workload variance; candidates differ from `n45` by a few percent only, so their paired
ratios should be at least as tight as `n45 / inc-n32`). Consequence: a shift of about 3% is resolvable per weight, the
predicted +4.8% for N = 64 at w = 0 sits 1.6 half-widths from 1, and the predicted -11.9% at w = 1 sits about 3.4
half-widths from 1. A true ratio within 3% of 1 will read as `null`, and the study cannot say more than that. No
adaptive stopping: the sample is fixed.

## 9. Estimator, gates and decision rule (frozen)

Analysis is native only (`ecbench`; no script). `D` is the session directory, `B` = 171294647853004 the bootstrap seed
(`--seed B`, `--resamples 4000`).

```
ecbench verify  --dir D --replay-all --exit-code
for N in n56 n64 n80 n91; do for W in 0 0.1 1; do
  ecbench compare --dir D --a n45 --b $N --weight $W --price-table-work    --seed B --save   # primary: ratio = S(N)/S(n45)
  ecbench compare --dir D --a n45 --b $N --weight $W --no-price-table-work --seed B --save   # secondary
done; done
for W in 0 0.1 1; do
  ecbench compare --dir D --a inc-n32 --b n45 --weight $W --no-price-table-work --seed B --save   # G3: ratio = S(n45)/S(inc-n32)
  ecbench compare --dir D --a inc-n32 --b n45 --weight $W --price-table-work    --seed B --save   # reported, ungated
done
ecbench compare --dir D --a n45 --b n45-aa --weight 0 --price-table-work --seed B   # G2
ecbench compare --dir D --a rho-neg --b rho-neg-aa --weight 0 --seed B              # G2
ecbench band    --dir D --weights 0,0.1,1 --ranks --json D/band.json                # primary pricing (default)
ecbench band    --dir D --weights 0,0.1,1 --ranks --no-price-table-work             # secondary (flag accepted, development check)
ecbench table   --dir D --reference rho-neg                                          # per-curve S, distance to rho
```

`compare` reports the ratio of total cost over the pooled workloads of both curves. Per-curve ratios of means come
from `ecbench table`.

**Gates (any failure makes the verdict `inconclusive`; the study still reports everything).**
- G1. `ecbench verify --replay-all` reports 0 problems and every record verified; the session status is `complete`.
- G2 (A/A, negative control). `n45` against `n45-aa` and `rho-neg` against `rho-neg-aa`: paired counts identical in every
  pair (ratio exactly 1.0000). A mismatch voids the session.
- G3 (positive control). Under `--no-price-table-work`, the point estimate of S(n45) / S(inc-n32) is inside
  [0.73, 0.90] at w = 0, [0.70, 0.89] at w = 0.1 and [0.63, 0.82] at w = 1, and its upper end is below 1 at all three.
  Ranges are the earlier holdout intervals widened by about 0.04 (derived from 0.818 [0.773, 0.866],
  0.798 [0.752, 0.847], 0.726 [0.677, 0.778]). Failure means the instrument or the curves do not reproduce the known
  effect, and no `gain`, `relabelling` or `null` is read.
- G4. At least 90% of the pairs are complete and no arm has a censored share above 1% (`max_trials`).

**Verdict per N (primary pricing, pooled), then overall.** Let L, U be the 95% interval ends of S(N) / S(n45) at w.
- `gain`: U < 1 at w = 0, 0.1 and 1, and the per-curve ratio of means is below 1 at all three on both curves.
- `relabelling`: some weight has U < 1 and some weight has L > 1.
- `null`: otherwise.
Overall: `gain` if any N is `gain` (if several, the N with the lowest pooled ratio at w = 0.1; ties by smaller N),
else `relabelling` if any N is, else `null`. The secondary reading is reported beside it and does not decide.

## 10. What each outcome changes

- `gain` (H1): a gain candidate for the 20-bit baseline. Next, not in this study: one further fresh pair of curves
  with only the selected N and `n45` (confirmation of a selected arm), a second-host replay on isolab-eks, the validator
  report and a matched-rho band, as `docs/VALIDATION_GATE.md` requires, before any leaderboard row. The atlas
  disposition (`open_next_step`, constant factor only) is unchanged.
- `relabelling` (predicted): recorded as work moved between native operations and priced operations; N = 45 stays the
  baseline; nothing else changes.
- `null`: N = 45 is bracketed from above; nothing changes.
- `inconclusive` (a gate failed): no reading; report the failed gate, rerun once if the cause is infrastructure.
- In every case the report states S(N) / S(rho-neg) at the three weights and the distance, never a pass or fail.

## 11. Variable ledger

| factor | treatment | how |
|:--|:--|:--|
| instrument | fixed | one binary and image for every arm; its sha256 is recorded in the session |
| curve | randomized, then fixed | two curves from the seeds in section 7; both enter every comparison |
| target | randomized | 96 per curve, one seed |
| algorithm seed | randomized | one per (workload, round), shared by every arm (paired) |
| arm order | randomized | `alternate`, balanced over six rounds |
| host, Spot | measured | counts do not depend on the host; an interrupted session is incomplete and rerun once |
| analyst | fixed | the commands of section 9, no free parameter after data |
| pricing | crossed | w and the two table-work readings, section 6 |

## 12. Execution and cost

- **Route.** Iso Lab on EKS Spot (AGENTS.md section 9a, `isolab-eks`), queue `cpu-l2-spot` (8 vCPU / 32 GiB),
  `c8a.2xlarge`, one job requesting 4 CPU, 4 Gi, `seconds 3600`, no checkpoint (a lost job is an incomplete
  session). Request shape: `research/prime_medium_rho_cpc_20261109/requests/M1.request.json` with the spec of
  `spec.json` and `job.build` bound to the frozen image. This study is counts-only; wall time is not quoted.
- **Cost (estimate).** Under 0.1 CPU-hours: the earlier 20-bit runs took a median of 1.7 to 2.5 ms and the development
  check ran 36 records in 3.2 s including start-up. 12,096 executions are about 1 minute of compute at that rate
  (estimate); pod provisioning dominates (about 300 s assumed in the earlier studies). Cost well under USD 1 at the
  recorded Spot price of USD 0.099 to 0.158 per hour; proposed ceiling USD 5 including one rerun.
  Local 4-core host: minutes (L0, development only).
- **Before launch** (main session): commit and push, freeze, `scripts/checkpoint.sh init` and push, a `bench.record/v1`
  record (`tools/benchrec.py init`), a Conductor task and scope check.

## 13. Validation gate and records

Counts-only, same-binary replay only (`ecbench verify --replay-all`). No row is proposed for the IC leaderboard or
`benchmarks/README.md`: that needs a `VALIDATION.json` (replay certificate, validator report, matched rho with the band),
and a second-host replay is not part of this study. Raw data to S3 by SHA-256 and the progress report per
`docs/benchmark-records.md`.

## 14. Sources checked, and blockers to freezing

- **Knowledge base.** `scripts/research-kb status` on 2026-11-14 printed `research-kb: no executable found; set
  RESEARCH_KB_BIN or put research-kb on PATH`; `RESEARCH_KB_BIN` is unset and the research-kb MCP tools are not exposed to
  this session. Not done. Fallback: repository paths read, listed here, with no `source_id` recorded:
  `research/prime_base_oracle_tuning_20261029/{README.md,VALIDATION.json,specs/holdout-P20.json,select/band-train-P20.md}`,
  `research/prime_field_lead_assessment_20261107/{README.md,atlas_survey.md,crypto_survey.md}`,
  `research/prime_ic_leads_20261110/README.md` (section 7 row 5, prefetch obsolete),
  `research/ic_e222_m221_20261106/CAMPAIGN_SUMMARY.md`, `src/bin/ecbench.rs`, `docs/ecbench/README.md` (table-work pricing),
  `research/prime_medium_rho_p4_confirm_20261108/PROTOCOL.md` (house format),
  `research/ic_lead_queue_20261114/B_sweep_followups/EXPERIMENTS.md` (spec 1). The KB pass, including a negative-form query on
  "factor-base size optimum", is owed before launch.
- **Blockers.**
  1. KB pass owed (above).
  2. `PLAN.txt` was produced with the local `target/release/ecbench` (sha256 73c4f66b...81e7, built at HEAD 0c2cc1b408
     by track A), not with the production image. The image pinned in the earlier requests is ecbench `a18af09f...` at source
     `dbd587a01a`. Whether it carries `prime-abscissa`, `mitm:negation_folded` and the `counted:` marker must be checked and
     `ecbench plan` re-run inside the chosen image; the arm identity ids (`ECM1h...`) must match `PLAN.txt` (the
     `rho.negation` id `ECM1hefea4e0ebe79` already equals the medium-tier study's).
  3. Both curve slugs need registering (section 7).
  4. The `experiment.protocol/v1` record (`protocol.json`) and its freeze are not written here; the main session produces
     them with the repository tool.
  5. Launch prerequisites of section 12 (checkpoint, benchrec, Conductor).
- **Not blockers, stated for honesty.** The study has no curve-equal estimator beyond `compare` pooling and `table`
  per-curve means (the house analysis scripts are Python and cannot be reused; AGENTS.md "no Python"). The two curves are
  nearly equal in size, so total-cost weighting and curve-equal weighting differ little (r = 2^19.12 and 2^19.53).

## 15. Freeze status (updated 2026-11-14, before the freeze; resolves the blockers of section 14)

- **Image and instrument.** The image is the `publish_only` build of source revision
  `0c2cc1b408e33a43e55ee31cb75307e90df9d619` (the same source the local `PLAN.txt` was made at). Its digest and the
  ecbench sha256 are recorded at launch from the build receipt, as an amendment-free input; `ecbench plan` is re-run inside
  that image and its spec and arm ids must equal `PLAN.txt` (`ECS1h8f29f99dcec4`) before launch. This replaces blocker 2.
  The earlier-study image (`a18af09f...`, `dbd587a01a`) is not used.
- **Knowledge base.** Unavailable in this session as well; the fallback of section 14 stands. This replaces blocker 1.
- **Curves.** Registration is staged, not complete: `docs/curves/sources/specs.txt` gains `prime:20:211747386521066` and
  `prime:20:145169970934094` and `docs/curves/sources/generated.json` gains their two Rust-computed entries
  (`curve_id_generated`, which resolves both seeds to the slugs of `PLAN.txt`). `docs/curves/registry.json` and its two derived
  files are not changed: a full `python3 scripts/build_curve_registry.py` on this checkout rewrites the registry (+414k and -231k
  lines, 7,365 further curves, because the committed registry predates sources already tracked), so it is left to the committing
  session. The two entries it will produce are in `registry/new-curve-entries.json` (ICV1 `ICV1:fp-571759:691:571069:376997:unk:unk:r:5e9cb4d25344`
  and `ICV1:fp-754483:-155:754639:258553:unk:unk:r:ce03b9ba3257`). Until then the curves are named by their icv1 slugs only.
- **Records.** Bench record `benchmarks/records/20261114T1957Z-b1-prime-ic-n-ext-fp20.json` (status open, created with
  `tools/benchrec.py init`; `BENCHREC` names it for the run).
- **Analysis script.** `analysis.sh` (thin `ecbench` orchestration of section 9, checked against the development smoke; its sha256 is
  bound in `protocol.json`).
- **Protocol record.** `protocol.json` (`experiment.protocol/v1`), validated with 0 findings, frozen with
  `experiment_protocol.py freeze`. It binds the sha256 of `spec.json`, `seeds.json`, `analysis.sh` and of this file as it stood
  before the freeze entry under Amendments was appended (the entry records the byte length to check with `head -c`).


## Amendments

(none)

- **2026-11-14, freeze record (not a change to the design).** frozen a18877e5c32cfd837001eff8df322721465ec2a58e757811eafee9832d8a9be3 at 2026-11-14T20:00:39Z, before any data
  (`protocol.json`, `experiment.protocol/v1`, validated with 0 findings; `freeze.commit` a9f62b745f3d2c69fc833d292c440c623832762c is the
  repository HEAD at the freeze, the files themselves are committed afterwards by the main session). The record binds the sha256 of this file
  as it stood before this entry (d536b92fc5072ba2de4e8f421afeb6b9a1ae2aa696cbcce7e5eb1fcc84af02d8, first 19533 bytes, check with `head -c 19533 PROTOCOL.md | sha256sum`), of the spec JSON files, of
  `seeds.json` and of `analysis.sh`. The image digest and the ecbench sha256 are filled at launch from the `publish_only` build
  receipt of 0c2cc1b408e33a43e55ee31cb75307e90df9d619; they are inputs, not amendments.
