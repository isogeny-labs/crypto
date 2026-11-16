# Coordinator decision — XP-20261029-dg-wall

- **Decision:** approve with required edits E1–E11 (2026-10-29, `coordinator` role, before any execution of these specs), conditional on the pre-launch checks L1–L4 below. A failed pre-launch check means nothing launches and the protocol returns to the coordinator; it is not an amendment.
- **Reviewed:** `PROTOCOL.md` (sha256 `405885f3…` as given by the caller; I could not recompute it here), `spec-P24.json`, `spec-P28.json`, `spec-P32.json`; the first study's `PROTOCOL.md` and `AMENDMENT-01.md`; v3 `PROTOCOL.md` and `APPROVAL.md`; `research/ic_srenew_deferred_guard_20261029/README.md`; `docs/ecbench/README.md` §8 (wall rule); AGENTS.md "Standing workflow rules", "Success criterion", §5a, §6 and §9a; `taskq/deploy/eks-karpenter/run-ecbench`; `src/cryptanalysis/ecbench/compare.rs` (`choose_aa`, `wall_compare`, `band_report`).
- **Edits:** apply E1–E11 verbatim in the commit that adds this file. That commit is the freeze (§9 as edited by E8).

## Findings

### (a) The decision statistic

The statistic is appropriate. Three things make it so:
- On these arms the charged GAE is equal pair by pair, so `ns per GAE` of `dg / srenew` is the ratio of total solve wall over the admitted pairs, at fixed charged work. That is the quantity the question asks about.
- The A/A pair that `ecbench compare` picks for `--a srenew --b dg` is `(srenew, srenew-aa)`, with `aa_selection` "includes arm A". `method_id` hashes the parameters, so `srenew` and `dg` are not `self_aa`. I confirmed this from `choose_aa` and from the non-numeric metadata of v3 `P28/analysis/compare-srenew__srenew-small-w0.json`. The bias of arm A therefore transfers to the B/A ratio. The combined interval treats the two standard errors as independent, which is conservative here because the A/A shares arm A.
- `wall_compare` receives the unpriced records, so the wall block does not depend on `--weight`.

It is not yet fixed. Four defects:
1. Condition 7 asks for `status` "ok". The wall block's values are `admitted`, `descriptive` and `refused`; "ok" is the `ops` status.
2. "At least five admitted pairs" is the tool's floor. With every IC execution required at L2, the expected number is exactly 288 pairs and 288 A/A pairs.
3. The protocol names neither the A/A arms, nor `aa_selection`, nor `self_aa`, nor which weight file decides.
4. The 1.000 tolerance at w = 0 is not stated numerically.

E4 and E5 fix all four.

**Residual risk, accepted and disclosed.** The `t` interval treats the 288 pairs as independent, but they are 48 targets × 6 rounds. That is the repository's frozen estimator. The validator reports a target-clustered interval descriptively (E9), and that interval decides nothing.

### (b) The 28-bit baseline consequence

The consequence is legitimate. It is not relabelling or accounting, but it is admissible only with the per-curve and headline conditions in E6.

**Why it is legitimate:**
- **§5a.** §5a forbids a ranking only where the order changes inside the band. Here no weight reverses the order: `dg ≤ srenew` at w = 0 (a tie) and `<` at w = 0.1 and 1. `ecbench band` prints "unresolved inside the band" only because its `order` needs a strict inequality at every weight (`band_report`).
- **What the wall test adds.** It is direct evidence that the removed native work has positive cost. It does not reprice the band.
- **Not relabelling.** Relabelling means a headline count fell while total cost rose. No charged count moves here.
- **Not accounting.** Accounting means the numbers changed but the algorithm did not. Here the guard inserts are actually no longer executed, so `target_guard_probes_uncharged` falls.

**Conditions it needs:**
- **Per curve.** It must hold on each curve, not only pooled. The v3 28-bit verdict turned on one curve's band. A pooled rule here would apply a looser standard to the same baseline slot.
- **Strict at w = 0.** It needs `Σ S_dg ≤ Σ S_srenew` at w = 0 on each curve, not "1.000 within tolerance". If a confirming addition makes `dg` cost more at w = 0, the order does change inside the band, and no wall result can resolve that.
- **Headline.** The headline stays counted `S`, which is unchanged at w = 0. The ratchet delta is 1.000 at w = 0, plus the band ratios at w = 0.1 and 1. Wall is the practicality note and the tie-break evidence, never the headline (AGENTS.md §6).
- **Scope.** The scope is c8a.2xlarge, image `cf0dcc8b…`, ecbench `be2969ce`, the two 28-bit curves, `mitm:negation_folded=1` and `incremental-gauss`.

### (c) Disclosure and freezing

The freezing mechanics are the v3 ones and are sound. The disclosure is incomplete in three ways that E7 fixes:
1. **The exact statistic already exists on six other sessions.** The `wall` blocks of `compare-srenew__dg-w{0,0.1,1}.json` in `research/ic_srenew_deferred_guard_20261029/{first-study,v3}/P{24,28,32}/analysis/` were measured on the same image and host class, and are committed in #2845. The protocol says "recorded wall time … on other curves" but does not name these files. It also does not seal them for the study's agents or say what happens to them afterwards.
2. **Rho solve-time ranges were seen.** The #2845 README prints them (84.7–996.3 µs, 547.5–933.1 µs, 140–998.5 µs, 704–965 µs). They are rho-only and unused, but they were seen, which contradicts "the only wall-time number used here is the lead's".
3. **The 28-bit consequence was written after the v3 28-bit verdict was known.** That is acceptable because the curves are fresh and the rule is fixed before data, but it must be stated.

I cannot verify the statement that no wall field was opened. Sealing the files and reporting them afterwards, whatever they show, is the mitigation.

**Coordinator's own access.** In this review I read only non-numeric fields: `schema`, `wall.status`, `aa_arms`, `aa_selection`, `self_aa` and `method_id` of v3 `P28` and `P32` `compare-srenew__srenew-small-w0.json`, and the key names of the P28 file. I opened no number from any wall block.

### (d) Underspecified, and one blocking design defect

**Blocking: the wall claim cannot earn native admission as specified.**
- §6.1 reuses v3's exit-42 clause verbatim. That clause was approved only because "wall time is not a claim" (v3 APPROVAL, and the clause's own text). Here wall time is the claim.
- With rho arms in the session, grading v3 refuses every sub-millisecond rho solve. At 24 bits that is all 672, so `run-ecbench` exits 42 at line 58 and never runs the native A/A admission at lines 59–64.
- AGENTS.md §9a ("Earn the result") and the crypto CLAUDE.md require native L2, replay and A/A admission for an L2 benchmark, with the original thresholds kept.
- Even with the L2 gate passed, the in-job admission would compare `rho-neg` against `srenew-aa`, whose A/A is the sub-ms rho pair.

**Fix (E1, E10, E11).** Split each size into two sessions on identical curves and targets:
- a wall session without rho, which must exit 0;
- a counted session that carries the matched rho, under the v3 exit-42 clause and in counted units only.

In the wall session, `walk` takes the `reference` role, a structural role used only for admission, and `srenew-aa` is the first control. `run-ecbench` then compares `walk` against `srenew-aa`; neither has a twin, so `choose_aa` takes the first-alphabetical same-method pair, `(srenew, srenew-aa)`, with 288 admitted pairs.

`srenew` cannot be the reference. `srenew` against `srenew-aa` is `self_aa`, which leaves no `aa_ci95`, and the job would exit 43.

**Other gaps, fixed in E3, E5 and E8:**
- AMENDMENT-01 step 3 embeds "the spec … at `be2969ce`", which is the wrong file for this study. The card must embed the spec at this protocol's freeze commit, with no rebuild.
- Rerun naming and what counts as a host loss.
- The exact compare commands and binary.
- An exit-43 clause for the counted session, whose own admission pair `rho-neg`/`rho-neg-aa` is `self_aa`.
- The launch-card record: freeze SHA and six spec hashes.
- The resume checkpoint.

## Pre-launch checks (read-only; on the director's card; all before the first launch)

- **L1.** `git show be2969cebf30a0cfb79088fd10094324c3efb3bb:taskq/deploy/eks-karpenter/run-ecbench` must show three things:
  - the reference is the first `role == "reference"` arm in spec order;
  - the control is the first `role == "control"` arm in spec order;
  - exit 43 is conditioned on `.wall.status=="admitted" and .wall.pairs_below_level==0 and .wall.aa_pairs>=5 and .wall.aa_ci95!=null`.

  `git show be2969ce…:src/cryptanalysis/ecbench/compare.rs` must contain `choose_aa` with the order arm A, then the reference arm, then first alphabetical, and the `self_aa` early return keyed on `method_id`. If either file differs, do not launch.
- **L2.** `ecbench plan` with binary `88070375…412c` must print the §3 curve slugs for both specs of each size, identical between the wall and counted spec of that size.
- **L3.** `jq -S .` must show that each wall spec differs from the submitted `spec-P<b>.json` only in `label`, `description`, the removal of `rho-neg` and `rho-neg-aa`, and the `walk` arm moved first with role `reference`. Each counted spec has the same `workloads` and `measurement` objects as its wall spec.
- **L4.** `.resume/RESUME.md` is filled and pushed (`scripts/checkpoint.sh init`). A Conductor task holds the `backlog/XP-20261029-dg-wall/` and `research/ic_dg_wall_20261029/` scopes.

## Edits

### E1 — replace §2 in full

```markdown
## 2. Arms (two sessions per size)

Each size runs two jobs on the same curves, targets and seeds:
- the **wall session** (`spec-P<b>.json`, job `ic-dgw-p<b>-20261029`) holds every arm whose wall time is read;
- the **counted session** (`spec-P<b>-rho.json`, job `ic-dgw-rho-p<b>-20261029`) holds the matched rho reference, in counted units only.

Grading v3 never admits the wall time of a solve under 1 ms. A rho arm in the wall session would therefore make `run-ecbench` exit 42 and skip its native A/A admission. Splitting the sessions lets the wall session earn native L2, replay and A/A admission (AGENTS.md §9a).

Wall session:

| arm | role | targets | factor base | reads |
|:--|:--|:--|:--|:--|
| `walk` | reference | `walk` | N_inc | descriptive; the role is structural (the arm `run-ecbench` compares with the first control for native A/A admission) |
| `srenew` | baseline | `stride-renew` | N_inc | decision (denominator) |
| `dg` | candidate | `stride-renew-deferred-guard` | N_inc | decision (numerator) |
| `srenew-aa` | control | = `srenew` | N_inc | wall A/A (bias and noise); first control in spec order |
| `dg-small` | control | `stride-renew-deferred-guard` | N_b | descriptive (24/32-bit baseline of #2845) |

Counted session:

| arm | role | targets | factor base | reads |
|:--|:--|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` | — | matched rho, counted units |
| `srenew` | baseline | `stride-renew` | N_inc | counted distance to rho |
| `dg` | candidate | `stride-renew-deferred-guard` | N_inc | counted distance to rho |
| `rho-neg-aa` | control | = `rho-neg` | — | A/A, counted units |

No wall field of the counted session is read.

N_inc and N_b are those of XP-20261029-srenew-deferred-guard: 162 and 130 at 24 bits, 407 and 267 at 28, 1024 and 741 at 32. `max_trials` is unchanged. The oracle is `mitm:negation_folded=1`; the other parameters are the `ecbench plan` defaults.
```

### E2 — replace the last line of §3

Old: `That gives 2,352 executions per size and 288 matched pairs per arm pair.`

New:

```markdown
That gives, per size, 1,680 executions in the wall session and 1,344 in the counted session (336 per arm), and 288 matched pairs per arm pair.
```

### E3 — replace §4 in full

```markdown
## 4. Route

The image, binary and launcher are those of `backlog/XP-20261029-srenew-deferred-guard/AMENDMENT-01.md`. Nothing is rebuilt.
- Image `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c`.
- ecbench sha256 `88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c`, built from `be2969cebf30a0cfb79088fd10094324c3efb3bb`.
- Launch with `taskq/deploy/eks-karpenter/remote/launch-remote`, using an `isolab.eks.request/v1` request:
  - `job.build = {image_uri@digest, source_revision, ecbench_sha256}`;
  - `provenance.harness = "claude-code-local"`;
  - c8a.2xlarge, a 3,600 s deadline, L2 isolation.
- Each request embeds, byte for byte, the spec file at this protocol's freeze commit (§9). It does not use a file at `be2969ce`.
- There are six jobs: `ic-dgw-p<b>-20261029` (wall) and `ic-dgw-rho-p<b>-20261029` (counted), for b ∈ {24, 28, 32}.
- Frozen `experiment-director` cards are run unchanged by `executor-mechanical`. Each card records the freeze-commit SHA, the spec sha256, the image digest and the ecbench sha256.

**Host loss.** A host loss is a pod lost before `job-exit.json` is written, for example by a Spot interruption. It reruns that job whole under the same name with the suffix `-r2` (then `-r3`), and the rerun is recorded. A job that wrote `job-exit.json` with a non-zero code is not a host loss and is not rerun.
```

### E4 — replace §5 in full

```markdown
## 5. Unit and statistic

**Wall.** The statistic is the `wall` block of

    ecbench compare --dir <wall session> --a srenew --b dg --weight 0 --json

It is run with the image's ecbench binary (sha256 `88070375…412c`) at its default `--resamples` and `--seed`, and the file's `resample_seed` is recorded. The block's fields:
- `wall.ns_per_gae_a` and `wall.ns_per_gae_b`: each arm's `Σ solve_wall / Σ total_gae` over the admitted pairs, where both runs are at L2 or L3;
- `wall.ratio_b_over_a` (`dg / srenew`), with `wall.ci95` (`ci_method` `linearised_t_pairs`);
- `wall.aa_arms` = `["srenew", "srenew-aa"]`, `wall.aa_selection` = `"includes arm A"`, and `wall.aa_ratio` (`srenew-aa / srenew`);
- `wall.corrected_ratio` = `ratio_b_over_a / aa_ratio`, with `wall.corrected_ci95` and `wall.outside_noise`.

The wall block reads charged `total_gae` and does not depend on `--weight`, so the w = 0.1 and w = 1 files carry the same block.

Because `dg` and `srenew` do the same charged work (§6.1 condition 8), this ratio is the ratio of total solve wall over the admitted pairs. `total_wall_ratio`, `median_ratio` and the `legacy_*` fields are reported and decide nothing.

Both arms carry unpriced native work, so each ns/GAE is an upper bound on cost per operation. The verdict records this, and it applies equally to both arms.

Descriptive wall comparisons decide nothing and record their own `aa_selection`. They are, in the wall session:
- `--a walk --b srenew`;
- `--a walk --b dg`;
- `--a walk --b dg-small`;
- `--a srenew --b dg-small`.

**Counted.** `S = total GAE / √r`, cold, whole pipeline, priced at w = 0, 0.1 and 1 (`--weight`), unchanged from the earlier studies. Each pair below is compared at the three weights, and each session gets `ecbench band --ranks`:
- wall session: `srenew`/`dg`, `srenew`/`srenew-aa`, `walk`/`srenew`, `walk`/`dg`, `walk`/`dg-small`, `srenew`/`dg-small`;
- counted session: `rho-neg`/`rho-neg-aa`, `rho-neg`/`srenew`, `rho-neg`/`dg`.
```

### E5 — replace §6.1 in full

```markdown
### 6.1 Instrument

**Wall session.** All of the following must hold, read from the job's own files after the `SHA256SUMS` check:
1. **Exit:** `job-exit.json` records `exit_code` 0 from a single pod. Any other code makes the size undecidable, including 42 (an execution below L2) and 43 (native A/A admission refused). The exit-42 clause of the v3 protocol does not apply to the wall session.
2. **Audit:** `audit.json` is `ok`, the session is `complete`, there are 0 problems, and every measured replay is `identical`.
3. **Counted A/A:** `srenew-aa / srenew` is exactly 1.000 at w = 0, 0.1 and 1.
4. **Spec and build:** `input-spec.json` is JSON-equal to the frozen `spec-P<b>.json`, and `build-info.json` names `be2969ce` and ecbench sha256 `88070375…412c`.
5. **Gate:** `l2-gate.json` has `accepted` true, every execution is graded L2 or L3, and `invalid_outcomes` is empty.
6. **Native A/A admission:** `control-comparison.json` is present, with `wall.status` `"admitted"`, `wall.pairs_below_level` 0, `wall.aa_pairs` ≥ 5, a non-null `wall.aa_ci95`, and `wall.aa_arms` `["srenew", "srenew-aa"]`.
7. **Decision block:** in the w = 0 `srenew`/`dg` file:
   - `wall.status` is `"admitted"`;
   - `wall.pairs` is 288 and `wall.pairs_below_level` is 0;
   - `wall.self_aa` is false;
   - `wall.aa_arms` is `["srenew", "srenew-aa"]` and `wall.aa_selection` is `"includes arm A"`;
   - `wall.aa_pairs` is 288;
   - `wall.corrected_ci95` is non-null.

   The w = 0.1 and w = 1 files carry an identical `wall` block.
8. **Same charged work:** in the `srenew`/`dg` files, two things hold:
   - `ops.ratio_b_over_a` at w = 0 satisfies |ratio − 1| < 0.0005;
   - the upper bound of `ops.ci95` is below 1 at w = 0.1 and at w = 1.

**Counted session.** Conditions 1–6 of `XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md` §6.1 apply, with the A/A in condition 2 being `rho-neg-aa / rho-neg` only, and with condition 4 read as "every `ic.pipeline` execution". The exit-42 clause applies verbatim, including its reason: no wall figure of the counted session is read.

Exit 43 also satisfies the route if, and only if:
- `l2-gate.json` has `accepted` true;
- conditions 1–6 hold;
- `control-comparison.json` has `wall.self_aa` true, because its pair `rho-neg`/`rho-neg-aa` is itself an A/A.

A counted-session failure does not change the wall decision. It blocks the validation gate (no matched rho) for that size, and it is reported.
```

### E6 — replace §6.2 in full

```markdown
### 6.2 Decision

The deferred guard is a **wall-clock improvement at N_inc** at size b if and only if the wall session's §6.1 conditions 1–8 hold and the upper bound of `wall.corrected_ci95` for `dg / srenew` is below 1.

| result at b (wall-session §6.1 holds) | reading |
|:--|:--|
| corrected upper bound < 1 | improvement: lower wall cost per operation, same charged work |
| corrected interval contains 1 | null at this sample size |
| corrected lower bound > 1 | regression |

If wall-session §6.1 fails, the size is undecidable.

**Consequence at 28 bits (fixed now).** At 28 bits `srenew` (N 407) stays the baseline after #2845. `dg` (N 407) replaces it as the 28-bit prime-field IC baseline if, and only if, all of the following hold at 28 bits in the wall session:
1. 6.2 gives an improvement.
2. On each curve, the `curves[]` row of the w = 0 `srenew`/`dg` file has `ratio_b_over_a` ≤ 1. That is, `Σ S_dg ≤ Σ S_srenew`, which the analysis recomputes from the totals.
3. On each curve, the `curves[]` rows of the w = 0.1 and w = 1 files have a `ci95` upper bound below 1.
4. On each curve, `dg`'s `target_guard_probes_uncharged` is below `srenew`'s.

Reading:
- The counted order `dg ≤ srenew` then holds at every weight on each curve. It is a tie at w = 0, so no weight in the §5a band reverses it.
- The wall result is the measured evidence that the removed native work has positive cost.
- This is neither relabelling (no charged count moves) nor accounting (the guard inserts are no longer executed).

The record states four things:
- the headline counted `S` is unchanged at w = 0;
- the ratchet delta is 1.000 at w = 0, and the measured band ratios at w = 0.1 and 1;
- the wall ratio is the practicality note and the tie-break evidence, never the headline (AGENTS.md §6);
- the scope is c8a.2xlarge, image `cf0dcc8b…`, ecbench `be2969ce`, the two 28-bit curves of §3, `mitm:negation_folded=1` and `incremental-gauss`.

If any of 1–4 fails, `srenew` stays the 28-bit baseline, and the w = 0 tie stays unresolved in counted units.

At 24 and 32 bits an improvement is reported as a component speed-up of the deferred guard, in its own section outside tables A–C. It changes no baseline: `dg-small` there is #2845's. Every size reports `srenew/rho-neg` and `dg/rho-neg` from the counted session at the three weights, as the distance remaining.
```

### E7 — replace §8 in full

```markdown
## 8. Disclosure

- **Earlier results seen before drafting.** This protocol was drafted after the first study and v3 were analysed. Those results include v3's counted `dg / srenew` (1.0000 at w = 0, 0.956–0.959 at w = 0.1, 0.756–0.766 at w = 1) and v3's 28-bit verdict (`srenew` kept on one curve's band). The 28-bit consequence in §6.2 was written knowing that verdict.
- **Earlier wall data of this exact comparison exist, and are sealed.**
  - Six earlier sessions measured `dg` against `srenew` at N_inc on the same image and host class: the first study and v3, at 24, 28 and 32 bits.
  - Their `wall` blocks are committed in `research/ic_srenew_deferred_guard_20261029/{first-study,v3}/P{24,28,32}/analysis/compare-*.json` (#2845), and their `session/records.jsonl` carry `time.solve_wall_ns`.
  - The author states that no `wall` block, `wall_ns` field or `time` field of those sessions was opened, and that their compare files were read for `ops` only.
  - Until this study's verdict commit is pushed, no agent working on this study opens them: author, director, executor and validator alike.
  - After that commit, the analysis reports each earlier session's `srenew`/`dg` `wall.corrected_ratio` and `wall.corrected_ci95`, whatever they show. They are reported next to these results, descriptively. They are never pooled and decide nothing: those sessions exited 42 and had no native A/A admission.
- **Wall figures that were seen.** The #2845 README prints rho solve-time ranges: 84.7–996.3 µs, 547.5–933.1 µs, 140–998.5 µs and 704–965 µs. They are rho-only and are not used. The only wall-time figures used here are the lead's 318 ns and 137 ns, for the predictions.
- **The coordinator's review.** The coordinator read only non-numeric fields: `schema`, `wall.status`, `aa_arms`, `aa_selection`, `self_aa` and `method_id` of v3 `P28` and `P32` `compare-srenew__srenew-small-w0.json`, and the key names of the P28 file. The aim was to confirm the A/A selection. No number of any wall block was opened.
- **Curves.** All six curves are fresh.
- **Factor bases.** N_inc and N_b are fixed by the earlier studies, not chosen here.
```

### E8 — replace §9 in full

```markdown
## 9. Freezing

This file, `APPROVAL.md` and the six specs (`spec-P<b>.json` and `spec-P<b>-rho.json`) are frozen by the commit that adds them.

Before the first job launches:
- that commit is pushed;
- its SHA and the six spec sha256 are recorded on every launch card.

Each job's request embeds its spec byte for byte. No amendment is allowed after the first launch of any of the six jobs.
```

### E9 — replace §10 in full

```markdown
## 10. Outputs

`research/ic_dg_wall_20261029/` holds, per size, `P<b>/wall/` and `P<b>/counted/`. Each holds:
- launch receipts and the S3 copy with hashes;
- the `compare` JSON for every pair named in §5, at the three weights;
- the band table.

The directory also holds a README with a figure and a PDF (research-visuals).

**Independent validation.** The validator, using its own code on the wall session's `records.jsonl`:
- recomputes `ns per GAE`, `ratio_b_over_a`, `aa_ratio`, `corrected_ratio` and `corrected_ci95`;
- reports, descriptively, the per-curve corrected ratios and an interval clustered on targets.

It also rechecks §6.1 for both sessions, and the four per-curve conditions of §6.2 at 28 bits.

**Before any leaderboard or `benchmarks/` change**, an improvement goes through the validation gate. `VALIDATION.json` needs:
- an independent replay of both sessions;
- the validator report;
- the matched rho from the counted session.

**Placement.** A 24/32-bit improvement enters the leaderboard as a component speed-up section outside tables A–C. A 28-bit baseline change enters as specified in §6.2. Performance figures are copied to `benchmarks/<YYYYMMDD>-<slug>/` with a row in `benchmarks/README.md`.

Wall-time figures enter only for the IC arms of the wall session. Rho wall time, and every wall field of the counted session, are never used.
```

### E10 — replace `spec-P24.json`, `spec-P28.json`, `spec-P32.json` in full (wall sessions)

`spec-P24.json`:

```json
{
 "schema": "ecbench.spec/v1",
 "label": "XP-20261029-dg-wall P24 wall session: wall time per operation, deferred guard against the eager guard at N 162",
 "description": "Frozen protocol (backlog/XP-20261029-dg-wall/PROTOCOL.md and APPROVAL.md): ns per GAE of ic.pipeline targets stride-renew-deferred-guard against stride-renew at the same factor base prime-abscissa:size=162 and oracle mitm:negation_folded=1 (identical charged work), A/A-corrected by srenew-aa, on two fresh 24-bit prime-order curves; walk (structural reference role for native A/A admission) and dg-small (N 130) are descriptive. No rho arm: the matched rho is in spec-P24-rho.json. Public synthetic curves and targets; no security claim.",
 "workloads": {
  "curves": [
   {"kind": "prime_search", "bits": 24, "seed": 202610297243},
   {"kind": "prime_search", "bits": 24, "seed": 202610297244}
  ],
  "targets_per_curve": 24,
  "target_seed": 202610297246,
  "target_kind": "public"
 },
 "arms": [
  {"name": "walk", "role": "reference", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm:negation_folded=1", "max_trials": "4243777", "targets": "walk"}}},
  {"name": "srenew", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm:negation_folded=1", "max_trials": "4243777", "targets": "stride-renew"}}},
  {"name": "dg", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm:negation_folded=1", "max_trials": "4243777", "targets": "stride-renew-deferred-guard"}}},
  {"name": "srenew-aa", "role": "control", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm:negation_folded=1", "max_trials": "4243777", "targets": "stride-renew"}}},
  {"name": "dg-small", "role": "control", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=130", "oracle": "mitm:negation_folded=1", "max_trials": "4243777", "targets": "stride-renew-deferred-guard"}}}
 ],
 "measurement": {"rounds": 6, "warmup": 1, "order": "alternate", "seed": 202610297248, "isolation_required": "L2", "timeout_seconds": 1800}
}
```

`spec-P28.json`:

```json
{
 "schema": "ecbench.spec/v1",
 "label": "XP-20261029-dg-wall P28 wall session: wall time per operation, deferred guard against the eager guard at N 407",
 "description": "Frozen protocol (backlog/XP-20261029-dg-wall/PROTOCOL.md and APPROVAL.md): ns per GAE of ic.pipeline targets stride-renew-deferred-guard against stride-renew at the same factor base prime-abscissa:size=407 and oracle mitm:negation_folded=1 (identical charged work), A/A-corrected by srenew-aa, on two fresh 28-bit prime-order curves; walk (structural reference role for native A/A admission) and dg-small (N 267) are descriptive. No rho arm: the matched rho is in spec-P28-rho.json. Public synthetic curves and targets; no security claim.",
 "workloads": {
  "curves": [
   {"kind": "prime_search", "bits": 28, "seed": 202610297283},
   {"kind": "prime_search", "bits": 28, "seed": 202610297284}
  ],
  "targets_per_curve": 24,
  "target_seed": 202610297286,
  "target_kind": "public"
 },
 "arms": [
  {"name": "walk", "role": "reference", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm:negation_folded=1", "max_trials": "26479085", "targets": "walk"}}},
  {"name": "srenew", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm:negation_folded=1", "max_trials": "26479085", "targets": "stride-renew"}}},
  {"name": "dg", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm:negation_folded=1", "max_trials": "26479085", "targets": "stride-renew-deferred-guard"}}},
  {"name": "srenew-aa", "role": "control", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm:negation_folded=1", "max_trials": "26479085", "targets": "stride-renew"}}},
  {"name": "dg-small", "role": "control", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=267", "oracle": "mitm:negation_folded=1", "max_trials": "26479085", "targets": "stride-renew-deferred-guard"}}}
 ],
 "measurement": {"rounds": 6, "warmup": 1, "order": "alternate", "seed": 202610297288, "isolation_required": "L2", "timeout_seconds": 1800}
}
```

`spec-P32.json`:

```json
{
 "schema": "ecbench.spec/v1",
 "label": "XP-20261029-dg-wall P32 wall session: wall time per operation, deferred guard against the eager guard at N 1024",
 "description": "Frozen protocol (backlog/XP-20261029-dg-wall/PROTOCOL.md and APPROVAL.md): ns per GAE of ic.pipeline targets stride-renew-deferred-guard against stride-renew at the same factor base prime-abscissa:size=1024 and oracle mitm:negation_folded=1 (identical charged work), A/A-corrected by srenew-aa, on two fresh 32-bit prime-order curves; walk (structural reference role for native A/A admission) and dg-small (N 741) are descriptive. No rho arm: the matched rho is in spec-P32-rho.json. Public synthetic curves and targets; no security claim.",
 "workloads": {
  "curves": [
   {"kind": "prime_search", "bits": 32, "seed": 202610297323},
   {"kind": "prime_search", "bits": 32, "seed": 202610297324}
  ],
  "targets_per_curve": 24,
  "target_seed": 202610297326,
  "target_kind": "public"
 },
 "arms": [
  {"name": "walk", "role": "reference", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm:negation_folded=1", "max_trials": "167871873", "targets": "walk"}}},
  {"name": "srenew", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm:negation_folded=1", "max_trials": "167871873", "targets": "stride-renew"}}},
  {"name": "dg", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm:negation_folded=1", "max_trials": "167871873", "targets": "stride-renew-deferred-guard"}}},
  {"name": "srenew-aa", "role": "control", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm:negation_folded=1", "max_trials": "167871873", "targets": "stride-renew"}}},
  {"name": "dg-small", "role": "control", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=741", "oracle": "mitm:negation_folded=1", "max_trials": "167871873", "targets": "stride-renew-deferred-guard"}}}
 ],
 "measurement": {"rounds": 6, "warmup": 1, "order": "alternate", "seed": 202610297328, "isolation_required": "L2", "timeout_seconds": 1800}
}
```

### E11 — add `spec-P24-rho.json`, `spec-P28-rho.json`, `spec-P32-rho.json` (counted sessions)

`spec-P24-rho.json`:

```json
{
 "schema": "ecbench.spec/v1",
 "label": "XP-20261029-dg-wall P24 counted session: matched rho reference for srenew and dg at N 162",
 "description": "Frozen protocol (backlog/XP-20261029-dg-wall/PROTOCOL.md and APPROVAL.md): counted units only (S = total GAE / sqrt r at w = 0, 0.1, 1) of rho.negation against ic.pipeline targets stride-renew and stride-renew-deferred-guard at prime-abscissa:size=162, oracle mitm:negation_folded=1, on the same two 24-bit curves and targets as spec-P24.json. No wall figure of this session is read. Public synthetic curves and targets; no security claim.",
 "workloads": {
  "curves": [
   {"kind": "prime_search", "bits": 24, "seed": 202610297243},
   {"kind": "prime_search", "bits": 24, "seed": 202610297244}
  ],
  "targets_per_curve": 24,
  "target_seed": 202610297246,
  "target_kind": "public"
 },
 "arms": [
  {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
  {"name": "srenew", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm:negation_folded=1", "max_trials": "4243777", "targets": "stride-renew"}}},
  {"name": "dg", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm:negation_folded=1", "max_trials": "4243777", "targets": "stride-renew-deferred-guard"}}},
  {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}
 ],
 "measurement": {"rounds": 6, "warmup": 1, "order": "alternate", "seed": 202610297248, "isolation_required": "L2", "timeout_seconds": 1800}
}
```

`spec-P28-rho.json`:

```json
{
 "schema": "ecbench.spec/v1",
 "label": "XP-20261029-dg-wall P28 counted session: matched rho reference for srenew and dg at N 407",
 "description": "Frozen protocol (backlog/XP-20261029-dg-wall/PROTOCOL.md and APPROVAL.md): counted units only (S = total GAE / sqrt r at w = 0, 0.1, 1) of rho.negation against ic.pipeline targets stride-renew and stride-renew-deferred-guard at prime-abscissa:size=407, oracle mitm:negation_folded=1, on the same two 28-bit curves and targets as spec-P28.json. No wall figure of this session is read. Public synthetic curves and targets; no security claim.",
 "workloads": {
  "curves": [
   {"kind": "prime_search", "bits": 28, "seed": 202610297283},
   {"kind": "prime_search", "bits": 28, "seed": 202610297284}
  ],
  "targets_per_curve": 24,
  "target_seed": 202610297286,
  "target_kind": "public"
 },
 "arms": [
  {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
  {"name": "srenew", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm:negation_folded=1", "max_trials": "26479085", "targets": "stride-renew"}}},
  {"name": "dg", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm:negation_folded=1", "max_trials": "26479085", "targets": "stride-renew-deferred-guard"}}},
  {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}
 ],
 "measurement": {"rounds": 6, "warmup": 1, "order": "alternate", "seed": 202610297288, "isolation_required": "L2", "timeout_seconds": 1800}
}
```

`spec-P32-rho.json`:

```json
{
 "schema": "ecbench.spec/v1",
 "label": "XP-20261029-dg-wall P32 counted session: matched rho reference for srenew and dg at N 1024",
 "description": "Frozen protocol (backlog/XP-20261029-dg-wall/PROTOCOL.md and APPROVAL.md): counted units only (S = total GAE / sqrt r at w = 0, 0.1, 1) of rho.negation against ic.pipeline targets stride-renew and stride-renew-deferred-guard at prime-abscissa:size=1024, oracle mitm:negation_folded=1, on the same two 32-bit curves and targets as spec-P32.json. No wall figure of this session is read. Public synthetic curves and targets; no security claim.",
 "workloads": {
  "curves": [
   {"kind": "prime_search", "bits": 32, "seed": 202610297323},
   {"kind": "prime_search", "bits": 32, "seed": 202610297324}
  ],
  "targets_per_curve": 24,
  "target_seed": 202610297326,
  "target_kind": "public"
 },
 "arms": [
  {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
  {"name": "srenew", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm:negation_folded=1", "max_trials": "167871873", "targets": "stride-renew"}}},
  {"name": "dg", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm:negation_folded=1", "max_trials": "167871873", "targets": "stride-renew-deferred-guard"}}},
  {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}
 ],
 "measurement": {"rounds": 6, "warmup": 1, "order": "alternate", "seed": 202610297328, "isolation_required": "L2", "timeout_seconds": 1800}
}
```

## Not changed

- §1 and §7 are unchanged; the predictions stand as written and decide nothing.
- §3's curves and seeds are unchanged.

## Decision record

The decision record is not yet in the ledger. In this engagement I had no shell to allocate a `DEC-*` id with `tools/allocate_id.py`, and I was asked not to run anything. The session that commits this file must allocate the id, confirm it with `--check`, and archive the record below as `ledger/decisions/DEC-20261029-<tok>.yaml`.

```yaml
type: coordinator_decision
id: DEC-20261029-<allocate>
date: 2026-10-29
subject: backlog/XP-20261029-dg-wall/PROTOCOL.md (+ spec-P{24,28,32}.json)
decision: approve_with_edits
edits: [E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11]
conditions: [L1, L2, L3, L4]   # fail-closed: no launch, return to coordinator
rationale:
  - "The wall statistic (ecbench compare wall.corrected_ci95 upper bound < 1, A/A srenew/srenew-aa 'includes arm A') fits a same-charged-work comparison. It is fixed by E4 and E5: status 'admitted', not 'ok'; 288 pairs; the w = 0 file decides."
  - "Reusing the v3 exit-42 clause for a wall claim fails AGENTS.md §9a, which requires native L2, replay and A/A admission. E1, E10 and E11 split each size into a wall session without rho (exit 0 required) and a counted rho session."
  - "The 28-bit baseline consequence is legitimate under §5a: weak dominance with a tie at w = 0 only, and wall as measured evidence that the removed native work has positive cost. It needs per-curve counted conditions and a counted headline (E6)."
  - "The disclosure must name and seal the six earlier dg/srenew wall blocks in #2845, and report them afterwards (E7)."
evidence_refs:
  - research/ic_srenew_deferred_guard_20261029/README.md
  - backlog/XP-20261029-srenew-deferred-guard-v3/PROTOCOL.md
  - backlog/XP-20261029-srenew-deferred-guard-v3/APPROVAL.md
  - docs/ecbench/README.md (§8, wall rule)
  - taskq/deploy/eks-karpenter/run-ecbench (lines 52-64)
  - src/cryptanalysis/ecbench/compare.rs (choose_aa, wall_compare, band_report)
scope: "prime-field IC, b in {24, 28, 32}, N_inc, mitm:negation_folded=1, incremental-gauss, c8a.2xlarge, image cf0dcc8b, ecbench be2969ce; engineering (constant); no exponent claim"
next_actions:
  - "Author: apply E1-E11 verbatim. The director runs L1-L4. Commit and push the freeze; record the SHA and the six spec sha256 on the cards."
  - "Experiment director: write six launch cards (three wall, three counted) and one analysis card per size; executor-mechanical runs them unchanged."
```
