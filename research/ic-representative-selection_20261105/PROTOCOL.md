# PROTOCOL v1: representative selection for index calculus on ℓ = 2 volcanoes

**Status: frozen before any data** (`protocol.json`, `freeze` block: canonical
SHA-256 and commit). Nothing in this directory has been run on Iso Lab, EKS or
any cloud host. The class inputs are generated in Stage 0 (§3) by a
deterministic rule frozen here; their digest is filed as amendment A1 in
[`INPUTS.md`](INPUTS.md) before the first measured run. Any other change is a
new, dated, labelled version, and both versions are reported.

Every statement carries one label: *recorded* (a committed file), *derived*
(a calculation shown here) or *predicted* (stated before the runs). No result
here is a pcc-verified claim; every number cited from earlier studies is an
unverified assertion `[pcc:unverified]` until a claim id is attached.

## 1. Question and source

The lead is crypto-autoresearcher
`knowledge/leads/ecdlp/structure/LEAD-STR-class-representative-ic-selection.yaml`
(disposition `open_next_step`). Generic cost is class-invariant, but IC with a
small-abscissa factor base is not: which points count as "small" depends on the
curve and on its model. Two levers follow:

- **(b) vertical transport.** On ℓ = 2 volcanoes, floor curves gave fewer IC
  relations than crater curves (v2.1 H1: yield floor/crater 0.942
  [0.928, 0.956]; H5: IC S floor/crater 1.045 [1.023, 1.067], *recorded*,
  `research/isogeny-volcano-cost-v2_20261017/README.md`
  `[pcc:unverified]`). v2 did not charge the walk. Here a floor instance is
  carried to the crater by three verified 2-isogenies, every native operation
  of finding the path and mapping the points is charged, and IC runs there.
- **(c) model selection.** Isomorphic models of one curve differed in IC yield
  (v2 H4: model τ 7.2% against horizontal 1.9%; for S on ℓ = 2, τ 9.8% against
  a null 95th percentile of 7.4%, *recorded*, `analysis/v2.1-H1-l2.json`
  `[pcc:unverified]`). Here the cheapest of 16 models is chosen on calibration
  targets and measured on held-out targets, which a selection artefact cannot
  survive.

## 2. Hypotheses, claims and the discriminator

| id | hypothesis | comparison | family |
|:--|:--|:--|:--|
| **H_B24** | arm B costs less charged S than arm A, at every weight w ∈ {0, 0.1, 1}, 24-bit p | `HB24`: B vs A, ic-mitm | primary (Holm, 3) |
| **H_B28** | the same at 28-bit p | `HB28` | primary |
| **H_C24** | Csel costs less S than Cdef on held-out targets, at every weight, 24-bit p | `HC24` | primary |
| H_C28 | the same at 28-bit p, 6 classes | `HC28` | secondary (own family) |

Verdicts (fixed vocabulary, `repsel analyse` `decide`, §8): `supported_within_scope`,
`unresolved_across_band`, `vanished_within_margin`, `inconclusive`,
`invalid_measurement`.

**The lead's discriminator, as frozen.** If H_B (at 24 or 28 bits) and H_C24
are `supported_within_scope`, representative selection counts as a measured IC
gain, constant-factor only, a success under the iterative criterion
(CLAUDE.md). If H_C24 is `vanished_within_margin`, the v2 model effect was
sampling noise or selection artefact. If H_B24, H_B28 and H_C24 are all
`vanished_within_margin`, the lead closes as measured. Any other combination
is reported as it falls, with the next step named.

**Predictions** (*predicted*, before any data):

- P-B24: θ(B/A) at w = 0 in [0.93, 0.99] (v2 H5 gives 0.957). The charged
  transport is 1,034–2,125 native operations per run on the 14 v2 ℓ = 2
  classes (*recorded* by `repsel ascend` on the committed v2 class files;
  §4.2), about 1–2 S against IC S ≈ 54–80, so at w = 1 it adds 1.3–2.5% and
  the w = 1 effect is predicted near 2–3%.
- P-B28: no prior. Transport is below 0.5% of S at w = 1 (*derived*: operations
  grow as log p while IC GAE grows about linearly in p, `ic-size-sweep` 24→28
  bits ×15).
- P-C24: held-out θ(Csel/Cdef) ≈ 0.95 if the true model spread σ_m is 5%,
  ≈ 0.98 if 3%, 1.00 if the v2 effect was noise (§5).
- Every IC arm stays above matched rho (v2.1 band: IC/rho 17–170× at w = 0).

## 3. Population and inputs (Stage 0, frozen rule)

- **Classes.** For tier 24 try seeds 2401, 2402, …; for tier 28, 2801, …,
  in increasing order:
  `volcano-rho --bits {24|28} --depth 3 --max-cofactor 4096 --ell 2 --crater split --seed S --controls 0 --level-caps 16,0,0,64 --models 1 --crater-models 16 --targets 8 --ecbench DIR`.
  Accept a seed when the export has at least 6 floor groups (below) and 8
  crater curves, its p is not a p of the v1 or v2 volcano studies, and
  `repsel ascend` passes on every floor curve. The first **12** accepted seeds
  per tier are its classes. Every rejected seed is logged with its reason
  (`inputs/classes/t<tier>.selection.log`). `run/01-classes.sh`.
- **Units.** *Floor groups:* every floor descendant of one crater vertex
  ascends to the same crater model and the same generator image (*recorded*:
  on all 14 v2 ℓ = 2 classes the 12 exported floor curves form groups of four,
  each landing on one crater instance). A group is therefore one unit of A, B
  and B0: up to the first **8** groups in BFS order. *Crater curves:* the first
  **16** crater vertices in BFS order, each in 16 isomorphic models
  (`--crater-models 16`; model 0 is the walked model, the default).
- **Targets.** `planted_shared`: one discrete log per target index on every
  curve of a class, so a transported workload carries the image of the same
  instance. Target seeds differ by stage (`design.json`): ab and b share one
  seed (b's 32 targets extend ab's 8), ctl, cal and hold have their own; the
  hold seed is never seen by the selection.
- **Arm (c) classes.** All 12 at tier 24; the first 6 accepted at tier 28.
- **Freeze.** `run/02-specs.sh` writes every pre-selection spec and label file
  and `inputs/MANIFEST.sha256`; its digest goes in `INPUTS.md` (amendment A1)
  and is committed and pushed before the first launch.

## 4. Arms and controls

### 4.1 Instrument

ecbench at the frozen commit, one digest-pinned image. ecbench arms in every
spec: `rho-neg` (reference, `rho.negation`), `rho-neg-aa` (control, the same
method and seeds: determinism and the Iso Lab A/A admission), `ic-mitm`
(candidate, `ic.pipeline` with `prime-abscissa:size=32` and
`mitm:negation_folded=1`, as in v2); the ctl stage adds `bsgs-neg` and
`ic-mitm-plus1` (base size 33). Measurement: L2, 5 rounds, 1 warmup,
`order: alternate`, timeout 120 s per run. Study arms are workload labels.

| study arm | role | workload | charged extra native operations |
|:--|:--|:--|:--|
| A | baseline | floor curve, model as walked (stage ab, 8 targets) | 0 |
| B | candidate | the group's ascended crater instance (stage b, 32 targets) | path finding + φ(G) + φ(Q) over 3 steps, largest in the group |
| B0 | negative control | each member's sibling floor curve, carried 2 steps (stage ab) | its path finding + maps |
| Aaa | A/A | arm A under measurement seed `aa` (stage aa) | 0 |
| Cdef | baseline | crater model 0, held-out targets (stage hold) | 0 |
| Csel | candidate | the calibration-selected model, held-out targets | 6 (u², u³, two products per point) unless model 0 |
| Crho | negative control | the rho-selected model, rho-neg, held-out targets | 6 unless model 0 |
| Cdefaa | A/A | Cdef under measurement seed `holdaa` (stage holdaa) | 0 |
| ctlA/ctlB/ctlB0 | positive controls | A, B, B0 with 2 targets (stage ctl) | as A, B, B0 |

### 4.2 Arm (b): the charged carrier (`repsel ascend`, `native/src/ascend.rs`)

Heights count from the floor (0) to the crater (3). Vélu's 2-isogeny with
kernel (x₀, 0) sends the other 2-torsion points to abscissa −2x₀, so the
backtracking edge is known for free. Charged: at height 0 the one rational
root of the cubic from gcd(x^p − x, f); at height h ≥ 1 the two
non-backtracking kernels from one square root; candidate c₁ descends iff a
non-backtracking walk of h − 1 steps from its codomain ends on a floor curve
(one Legendre symbol), since an ascending candidate cannot reach the floor in
fewer than h + 1 steps (Fouquet–Morain). φ(G) and φ(Q) cost one inversion and
four multiplications per step. Every multiplication and inversion is counted;
additions and small-constant multiples are not. *Uncharged verification*:
images on their codomains, of order r, additive (φ(G + 2G) = φ(G) + φ(2G)),
and the height of every vertex on the path recomputed by BFS (B: 0,1,2,3;
B0: 0,1,0). On the committed v2 fixture class every B endpoint is an exported
crater vertex and the first step reaches the BFS parent (`cargo test`).

### 4.3 Controls

- **A/A.** `Aaa` vs `A` and `Cdefaa` vs `Cdef` (independent measurement seeds):
  the 95% CI at w = 0 must contain 1, and its half-width is the noise floor
  reported beside every effect. `rho-neg-aa` vs `rho-neg` (same seeds): every
  pair identical (`check:determinism`).
- **Positive.** Planted factor base: yield ratio `ic-mitm-plus1`/`ic-mitm` on
  ctl workloads must have a 95% CI excluding 1 (predicted (33/32)² = 1.0635;
  v2 1.062 [1.060, 1.064]). BSGS exactness: exactly one bsgs-neg cost per
  (tier, class, group, target) across ctlA, ctlB, ctlB0, and equal planted
  scalars (`check:transport:<tier>`), which binds every transported instance
  to its floor instance. Analysis power: `PWB`/`PWC` scale X by 0.96 on the
  A/A data; p_iut must be below 0.0167.
- **Negative.** `Crho`: the selection procedure applied to rho, which has no
  expected model effect (v2 rho model p = 0.08); its held-out 95% CI must
  contain 1, or H_C is invalid (selection leak). `B0`: the same carrier
  landing on the same level; `MECH` (B vs B0) isolates the level change from
  re-modelling and transport.
- **Matched rho.** `GAP*`: each IC arm over rho-neg on the instance's source
  curve (floor curve for A/B/B0, crater model 0 for C), at every weight: the
  distance remaining, not a gate. `RHOLVL` (rho on B vs A) replicates v2 H3.

### 4.4 Arm (c): calibration, selection, held-out

Calibration (stage cal): every crater curve in all 16 models × 8 targets × 5
rounds. Selection (`repsel select`): per crater curve, the model with the
smallest mean S (w = 0) over its 40 calibration runs, ties to the lower index;
a model with a missing or unverified run is not eligible. The same rule on
rho-neg gives the Crho model. `selection.json` is hashed, committed and
pushed before any hold job runs. Held-out (stage hold): model 0, the IC pick
and the rho pick, 8 held-out targets × 5 rounds. The calibration GAE is
reported as one-time precomputation with the break-even target count;
in-sample gains (`in_sample_ratio`) are reported only as the winner's-curse
contrast, never as the C result.

## 5. Sample size and power (derived from v2.1)

Inputs (*recorded*, v2.1): per-run CV of IC S within a curve-model 0.37–0.44
(classes l2-s1001, s1005, s1010; v2 `sessions/*/spec*/records.jsonl.gz`),
0.40–0.48 overall; S floor/crater 1.045, SE 0.0107 on the log scale (95% CI
[1.023, 1.067]); S model τ 9.8% raw (DerSimonian–Laird), null p95 7.4%; S
horizontal τ 2.0%. Take CV = 0.45, a class-level effect spread of 2%, and a
×1.25 allowance for the ratio estimator and skew. Holm's smallest threshold
with three tests is α = 0.0167 (z = 2.13); normal approximation.

- **H_B.** Unit = floor group; A has up to 4 × 40 runs, B 1 × 160 runs.
  Per-unit log-ratio variance 0.45²(1/160 + 1/160) + σ²(1 + 1/4), with σ the
  curve-model spread. 96 units per tier → SE 0.0129 (σ = 6%) or 0.0171
  (σ = 10%). Effect at w = 0 ln 1.045 = 0.044: power **0.90** (σ = 6%) or
  0.67 (σ = 10%). At w = 1 at 24 bits the predicted net effect is 2–3%:
  power 0.43–0.56, so a 24-bit result significant at w = 0 but not at w = 1 is
  reported `unresolved_across_band`, never `vanished`. At 28 bits the
  transport is negligible: ≈ 0.90 if the floor effect persists.
- **H_C.** Unit = crater curve; ≈ 12 per class → 144 at 24 bits. Expected
  held-out gain of the best of K = 16 models with n = 40 calibration runs each:
  G = σ_m² / √(σ_m² + 0.45²/40) × E[max of 16 N(0,1)] (1.766): 5.1% at
  σ_m = 5%, 2.1% at 3%, 7.6% at 6.4% (the v2 noise-corrected guess
  √(0.098² − 0.074²)). SE ≈ 0.0126 → power **0.97** at σ_m = 5%, 0.31 at 3%.
  H_C28 (6 classes, ≈ 72 units) is secondary for that reason.
- **Resolution.** 10,000 sign-flip permutations: minimum one-sided p 1e-4 <
  0.0167. 4,000 bootstrap resamples for the intervals.

## 6. Execution (frozen route)

- **Route** (AGENTS.md §9a, `isolab-eks`). Build once: dispatch
  `isolab-experiment.yml` on `main` with `source_revision=<frozen commit>` and
  `publish_only=true`; its receipt (image digest, source revision, ecbench
  SHA-256) is `inputs/image.json`. Each job: `run/03-launch.sh <job>`, which
  writes an `isolab.eks.request/v1` (c8a.2xlarge, 3,600 s, 4 GiB, the inline
  spec, `job.build` pinned), runs `remote/validate-request` and
  `remote/launch-remote` with `AWS_REGION=us-east-2`,
  `ISOLAB_BUCKET=isolab-results-104186811093-us-east-2`,
  `ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher`, and a fresh receipt
  directory. Then `run/04-collect.sh <job>` downloads the payload, checks
  `SHA256SUMS`, the L2 gate, the audit (replay-all), the binary hash and the
  spec identity. One job at a time (the L2 pool holds one 8-vCPU worker), in
  `jobs.tsv` order, tier 24 first; hold jobs after their class's selection.
- **Model tiers.** The experiment-director compiles `run/CARD.md` into a task
  card per batch, the main session relays it verbatim to executor-mechanical,
  the director assesses each receipt. Every launch runs under
  `tools/benchrec.py run <record> --` (one `bench.record/v1` per job).
- **Size** (*derived*, conservative per-run estimates in `design.json`): about
  234 jobs, each ≤ 1,400 s of measurement (doubled by the harness's
  `--replay-all`); about 56 job-hours at tier 24 and 88 at tier 28. Budget
  ceiling **USD 150** (compute, launcher builds, retries), deadline 10 days from
  the first launch; stop and queue when it or the daily allowance is exhausted
  or unknown (`COST_CONTROLS.md`).
- **Preemption.** Ordinary request-v1 is not resumable: an interrupted or
  failed job is kept and re-submitted once as `<job>-r2`; counted units of an
  interrupted session are never pooled. A job over its deadline is split by
  units under a labelled amendment filed before its data.
- **Resume checkpoint.** `.resume/RESUME.md` is filled; its push to S3 must
  succeed before any launch (the hooks refuse launchers otherwise).

## 7. Frozen analysis (`native/src/analyse.rs`, `comparisons.json`)

- Price: S(w) = (total_gae + w·(U + extra_ops))/√r, U counted exactly as
  `ecbench band` does (memory-size counters excluded); a record with a pinned
  calibration or an uncounted unpriced counter is refused.
- Statistic: θ = exp(mean over classes of ln(mean over units of X / mean over
  units of Y)), unit means as ratios of sums.
- Interval: hierarchical bootstrap (classes; units within class; runs within
  unit and arm), 4,000 resamples, seed 20261105; 95% and 90% percentiles.
- Test: unit-level sign flip, 10,000 permutations, one-sided add-one p; p_iut
  = max over w; Holm over H_B24, H_B28, H_C24; H_C28 its own family.
- Masking: `repsel mask` (nonce drawn after the inputs freeze) codes every arm
  label; `analysis-masked.json` is hashed before `repsel unmask`.
- Cross-check: `ecbench band` on every session; its per-arm means must agree
  with the analysis summary for the same records.
- Export: `experiment.export/v1` per session with
  `experiment_protocol.py from-ecbench`, `check-export --verify-files`.

## 8. Decision rule

For each hypothesis, in order: `invalid_measurement` if a gate fails
(determinism; `check:transport` for H_B; the A/A CI at w = 0 excludes 1; the
planted-yield CI contains 1; for H_C, the Crho CI excludes 1);
`supported_within_scope` if Holm rejects on p_iut; `unresolved_across_band`
if the w = 0 p alone is below its Holm threshold; `vanished_within_margin` if
the power control passed and the 90% CI lower bound of θ at w = 0 exceeds 0.97
(gain below 3% with 95% confidence); otherwise `inconclusive`.

## 9. Does not count

- Changing classes, units, targets, seeds, K, the held-out split, the selection
  rule, the charging, the weights, the margin or the correction after any
  measured IC value is seen; re-running a job because of its result; dropping
  a unit, class or run outside §6.
- Reporting in-sample gains as the C result; reporting B at w = 0 alone as a
  ranking; reading C as a cold one-target gain without its selection
  cost and break-even.
- Any timing claim (counted S is the metric). Any security statement.

## 10. What happens after

A supported result lands with its report, graphs and PDF (`research-visuals`),
`benchmarks/<date>-ic-representative-selection/`, `bench.record/v1` records
and the progress report, and reaches `benchmarks/README.md` or the IC
leaderboard only with a `VALIDATION.json` (independent replay certificate,
validator recompute, matched rho with the §5a band) and a pcc claim. The next
step on support is a cheap selection proxy (the selection costs about
640 IC runs per curve); on `vanished` the lead closes and the atlas is
updated.

## Literature checked

Repository: the v2 study (README, PROTOCOL, analysis), the volcano census
REPORT, pair46 floor pullback, RESEARCH_ISOGENY_FIBERS_ECDLP.md, ecbench
`isogeny.rs` (`transfer.isogeny`) and `band.rs`, the Iso Lab harness. External,
recalled and not re-fetched in this session (bibliographic details stated from
memory, a gap): Fouquet–Morain (ANTS-V 2002) for direction finding on
volcanoes; Sutherland, "Isogeny volcanoes" (ANTS X, 2013); Vélu (1971);
Cawley–Talbot (JMLR 2010) on selection bias; Holm (1979). No prior work was
found in the repository on charging a volcano ascent inside an IC cost.
