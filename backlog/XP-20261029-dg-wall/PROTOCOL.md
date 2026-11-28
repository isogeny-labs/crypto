# Prime-field IC: wall time per operation of the deferred repeat guard — protocol

**Status:** drafted 2026-10-29, before any execution of these specs.

## 1. Question and class

`targets = stride-renew` probes the repeat guard on every trial: one hash insert, unpriced (`target_guard_probes_uncharged`). `targets = stride-renew-deferred-guard` (#2831) probes it only on targets that decompose. At the same factor base the two arms do the same charged work: in XP-20261029-srenew-deferred-guard-v3, `dg / srenew` is exactly 1.000 at w = 0 at 24, 28 and 32 bits, 288/288 pairs.

The lead `LEAD-ICP-prime-ic-constant-engineering` (crypto-autoresearcher atlas, reviewed 2026-10-29) names the remaining constant of the prime relation trial as "the hash-table floor of the relation trial (guard insert plus oracle probe, about 137 ns of a ~318 ns trial)". The deferred guard removes the guard insert from almost every trial. The question is whether that is visible as a lower **wall-clock cost per charged operation**.

**Question.** At b ∈ {24, 28, 32}, with the incumbent factor base N_inc, is the ecbench wall statistic `ns per GAE` of `dg` below that of `srenew`, after the A/A bias correction?

Class: engineering (constant), measured at L2.

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

## 3. Workloads

Each size uses two fresh prime-order curves from `prime_search`, with seeds `202610297<b>3` and `202610297<b>4`:

| b | curves (`ecbench plan`, binary 88070375…412c) |
|--:|:--|
| 24 | `icv1-fp24-tm1297-6f714a52`, `icv1-fp24-tm2281-0469151f` |
| 28 | `icv1-fp28-t14677-349132c0`, `icv1-fp28-t22749-960afa7f` |
| 32 | `icv1-fp32-t16357-dd50743c`, `icv1-fp32-tm94829-d5c5f997` |

None of these curves appears in `research/*.md`, `backlog/`, `docs/ic/*.json` or `benchmarks/*.md` on `origin/main` or on the #2845 branch (`git grep`, 2026-10-29).

Measurement, per size:
- 24 public targets per curve (seed `202610297<b>6`);
- 1 warm-up and 6 measured rounds, `alternate` order (seed `202610297<b>8`);
- L2 isolation, 1800 s timeout.

That gives, per size, 1,680 executions in the wall session and 1,344 in the counted session (336 per arm), and 288 matched pairs per arm pair.

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

## 6. Decision rule (per size, fixed now)

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

## 7. Predictions (before any data)

The model has three parts:
- a relation trial costs about 318 ns, and the guard insert about half of the 137 ns hash floor, i.e. 55–70 ns;
- the oracle set-up, N(N+1) additions with table inserts, is unchanged;
- the set-up's share of `srenew` GAE is taken from the v3 counted records (measured rounds): 0.522 at 24 bits, 0.546 at 28 and 0.520 at 32. So the guard can touch only about half of the work, and the predicted ratio is about 1 − 0.48 × (0.17 to 0.22) × (the relation phase's share of wall per GAE).

| b | predicted corrected `dg / srenew` ns/GAE |
|--:|:--|
| 24 | 0.89–0.97 |
| 28 | 0.89–0.97 |
| 32 | 0.89–0.97 |

Model risk:
- If the guard insert hits a cache-resident table, its cost may be far below 55 ns, and the ratio is null at 24 bits first, where the guard table is smallest.
- The wall statistic includes set-up. A null may therefore hide a relation-phase gain. The online windows (`online.phases_ns`) are reported descriptively and decide nothing.

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

## 9. Freezing

This file, `APPROVAL.md` and the six specs (`spec-P<b>.json` and `spec-P<b>-rho.json`) are frozen by the commit that adds them.

Before the first job launches:
- that commit is pushed;
- its SHA and the six spec sha256 are recorded on every launch card.

Each job's request embeds its spec byte for byte. No amendment is allowed after the first launch of any of the six jobs.

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
