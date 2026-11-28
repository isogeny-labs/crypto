# Prime-field IC: wall time per operation of the cache-blocked packed pair table — protocol

**Status:** drafted 2026-10-29, before any execution of these specs.

## 1. Question and class

The incumbent prime-field oracle `mitm:negation_folded=1` builds its negation-folded pair table with the scalar builder (`PairTable::build_negation_folded`) into one `FastMap` (a `HashMap` of 16-byte entries) and probes that map once per relation trial. The oracle `mitm-prime-blocked:m=2` (e59331c96, 2026-10-20) builds **the same table** (the same keys, the same witness for every key, the same probe, the same counted operations) with batched inversions into cache-sized open-addressed regions of one-word entries (`PackedPairTable`, p < 2³²), and probes that packed table.

`research/ic_oracle_setup_blocked_20261020` showed 456/456 executions identical in counts across the builders and a set-up wall ratio blocked/scalar of 0.25 [pair range 0.15, 0.43] at N = 1,024. Every wall figure there was descriptive (most runs L1), N below 1,024 was not measured, and the relation phase's probe on the packed table was never compared at L2.

In XP-20261029-dg-wall the oracle set-up is about a quarter to a third of `dg`'s solve wall at N_inc (phase sums of that study's wall session, read for this prediction only). The lead `LEAD-ICP-prime-ic-constant-engineering` names the hash-table floor of the relation trial as the remaining constant; the packed table is the "table with exposed slot addresses" the lead asks for.

**Question.** At b ∈ {24, 28, 32}, with the incumbent factor base N_inc and `targets = stride-renew-deferred-guard`, is the ecbench wall statistic `ns per GAE` of the packed-table oracle (`dgb`) below that of the incumbent oracle (`dg`), after the A/A bias correction?

Class: engineering (constant), measured at L2. Charged work is identical by construction; this is a component speed-up, never a counted-unit claim.

## 2. Arms (two sessions per size)

Each size runs two jobs on the same curves, targets and seeds. The split is the one in XP-20261029-dg-wall §2, so that the wall session earns native L2, replay and A/A admission:
- the **wall session** (`spec-P<b>.json`, job `ic-dgb-p<b>-20261029`);
- the **counted session** (`spec-P<b>-rho.json`, job `ic-dgb-rho-p<b>-20261029`).

Wall session, in spec order:

| index | arm | role | oracle | position in a forward / reversed block | reads |
|--:|:--|:--|:--|:--|:--|
| 0 | `dg` | baseline | `mitm:negation_folded=1` | 0 / 3 | decision (arm A); edge arm of the position control |
| 1 | `dg-aa` | control | = `dg` | 1 / 2 | wall A/A of the decision file; first control in spec order |
| 2 | `dgb` | candidate | `mitm-prime-blocked:m=2` | 2 / 1 | decision (arm B) |
| 3 | `dgb-edge` | reference | = `dgb` | 3 / 0 | position control (descriptive). The role is structural: `run-ecbench` compares the first `reference` arm with the first `control` arm for native A/A admission. |

Counted session: `rho-neg` (reference, `rho.negation`), `dg` (baseline), `dgb` (candidate), `rho-neg-aa` (control). No wall field of the counted session is read.

Every IC arm uses:
- `targets = stride-renew-deferred-guard` (the `dg` arm of XP-20261029-dg-wall);
- factor base `prime-abscissa:size=N_inc`, with N_inc 162, 407 and 1024 at 24, 28 and 32 bits;
- that study's `max_trials`.

The other parameters are the `ecbench plan` defaults.

**Why this order.**
- **How `alternate` places arms.** The `alternate` order runs a block either in spec order or reversed. XP-20261029-dg-wall's validator report (`arm_positions_alternate_order`) shows this: its five arms sat at positions {0,4}, {1,3}, {2}, {1,3} and {0,4}, 144 blocks each. With four arms, indices 0 and 3 share the edge positions {0, 3}, and indices 1 and 2 share the middle positions {1, 2}.
- **What the decision statistic measures.** All four arms charge identical work pair by pair (§6.1 condition 8). So the decision statistic `wall.corrected_ratio` of `--a dg --b dgb` equals Σ wall(`dgb`) / Σ wall(`dg-aa`) over the 288 pairs (§5). It is free of a position effect only if `dgb` and `dg-aa` occupy the same positions.
- **What this order gives.**
  - `dgb` and `dg-aa` both run in the middle, each at positions 1 and 2 in 144 blocks.
  - Each runs before the other in half the blocks.
  - Inside a block, each follows a `mitm:negation_folded=1` run in one block direction and a `mitm-prime-blocked` run in the other.
  - The edge arms `dg` and `dgb-edge` give a second estimate of the oracle effect, position-matched at the block edges. It decides nothing.
  - `dg-aa / dg` (the decision file's A/A ratio) and `dgb / dgb-edge` are middle-over-edge position contrasts for each oracle. They are descriptive, and they measure the one-time costs that AGENTS.md §10 says land on whichever case runs first.
- **Why the drafted order fails.** This order replaces the drafted one (`dgb-lead`, `dg`, `dgb`, `dg-aa`).
  - Under `alternate`, the drafted order runs `dgb-lead` last, after `dg`, in every reversed block.
  - It puts `dgb` in the middle and `dg-aa` at the edges, so the corrected decision ratio would have carried the middle-versus-edge effect.

## 3. Workloads

Each size uses two fresh prime-order curves from `prime_search`, with seeds `202610295<b>3` and `202610295<b>4`:

| b | curves (`ecbench plan`, binary 88070375…412c) |
|--:|:--|
| 24 | `icv1-fp24-t1973-f876d133`, `icv1-fp24-tm995-ee6c519b` |
| 28 | `icv1-fp28-t9805-a382b8a8`, `icv1-fp28-tm3463-6d2f5699` |
| 32 | `icv1-fp32-tm43175-08b04f34`, `icv1-fp32-tm81759-3ad5e8c5` |

None of these curves appears in `research/`, `backlog/`, `docs/ic/` or `benchmarks/` on `origin/main` or on the #2849 branch (`git grep`, 2026-10-29).

Measurement, per size: 24 public targets per curve (seed `202610295<b>6`); 1 warm-up and 6 measured rounds, `alternate` order (seed `202610295<b>8`); L2 isolation, 1800 s timeout. That gives 1,344 executions per session and 288 matched pairs per arm pair.

A local pilot (pilot seed 909090, one 32-bit curve, 3 targets, L0, not a study curve) checked only that the two oracles run and charge the same operations (`ops` ratio 1.0). It ran with ecbench sha256 `88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c` (the image's binary), and its output is kept at `backlog/XP-20261029-dg-blocked/pilot/` (copied from the local run directory before the freeze). Its wall ratio `dgb / dg`, 0.9146 (ns per GAE 196.6 → 179.8, 3 pairs), was seen. That ratio is descriptive at L0 and plays no part here.

## 4. Route

The image, binary and launcher of XP-20261029-dg-wall §4, unchanged; nothing is rebuilt:
- image `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:cf0dcc8ba2f6e399f3c16aec8dda0eaccb65dd0f8b75a2f90daf2b543ffb620c`;
- ecbench sha256 `88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c`, built from `be2969cebf30a0cfb79088fd10094324c3efb3bb` (which contains `mitm-prime-blocked`);
- `taskq/deploy/eks-karpenter/remote/launch-remote` with an `isolab.eks.request/v1` request, `job.build = {image_uri@digest, source_revision, ecbench_sha256}`, `provenance.harness = "claude-code-local"`, c8a.2xlarge, 3,600 s deadline, L2;
- each request embeds its spec byte for byte from this protocol's freeze commit (§9);
- six jobs: `ic-dgb-p<b>-20261029` (wall) and `ic-dgb-rho-p<b>-20261029` (counted);
- frozen `experiment-director` cards run unchanged by `executor-mechanical`, each recording the freeze-commit SHA, the spec sha256, the image digest and the ecbench sha256.

**Host loss** is handled as in XP-20261029-dg-wall §4 (rerun the job whole as `-r2`, recorded; a job that wrote `job-exit.json` with a non-zero code is not rerun).

## 5. Unit and statistic

**Wall.** The statistic is the `wall` block of

    ecbench compare --dir <wall session> --a dg --b dgb --weight 0 --json

It is run with the image's ecbench binary (sha256 `88070375…412c`) at its default `--resamples` and `--seed`, and the file's `resample_seed` is recorded. The block's fields:
- `wall.ns_per_gae_a` and `wall.ns_per_gae_b`: each arm's `Σ solve_wall / Σ total_gae` over the admitted pairs, where both runs of a pair are at L2 or L3;
- `wall.ratio_b_over_a` (`dgb / dg`), with `wall.ci95`;
- `wall.aa_arms` = `["dg", "dg-aa"]`, `wall.aa_selection` = `"includes arm A"`, and `wall.aa_ratio` (`dg-aa / dg`);
- `wall.corrected_ratio` = `ratio_b_over_a / aa_ratio`, with `wall.corrected_ci95` and `wall.outside_noise`.

The block reads charged `total_gae` and does not depend on `--weight`. The w = 0.1 and w = 1 files therefore carry the same block.

**What the corrected ratio is.** Two facts hold for the 288 (curve, target, round) pairs: all four arms charge identical work on each pair (§6.1 condition 8), and every pair is admitted. Together they give:
- `corrected_ratio` = Σ wall(`dgb`) / Σ wall(`dg-aa`). This compares two arms at the same block positions (§2).
- `ratio_b_over_a` = Σ wall(`dgb`) / Σ wall(`dg`). This compares middle positions with edge positions, and it decides nothing.

In this design `aa_ratio` carries the baseline's middle-over-edge contrast as well as the A/A noise. Dividing by it is what takes the position effect out of the decision.

The combined interval treats the two standard errors as independent. That is conservative, because both share `dg`.

`total_wall_ratio`, `median_ratio` and the `legacy_*` fields are reported and decide nothing.

Both arms carry unpriced native work, so each ns/GAE is an upper bound on cost per operation, for both arms alike.

**Descriptive wall comparisons.** In the wall session these decide nothing, and each records its `aa_arms`, `aa_selection` and `self_aa`:
- `--a dg-aa --b dgb`: `ratio_b_over_a` with `ci95`. This is the decision's middle-position contrast as a direct paired interval. Its use is the identity check of §6.1 condition 9.
- `--a dg --b dgb-edge`: `ratio_b_over_a` with `ci95`, uncorrected. This is the **position control**, the oracle effect at the block edges. Its corrected fields mix positions and are not read.
- `--a dgb-edge --b dgb`: `self_aa` is true. `ratio_b_over_a` with `ci95` is the candidate's middle-over-edge contrast.
- The decision file's `aa_ratio` with `aa_ci95`: the baseline's middle-over-edge contrast.

**Counted.** `S = total GAE / √r`, cold, whole pipeline, priced at w = 0, 0.1 and 1. Each pair below is compared at the three weights, and each session gets `ecbench band --ranks`:
- wall session: `dg`/`dgb`, `dg`/`dg-aa`, `dgb-edge`/`dgb`, `dg`/`dgb-edge`, `dg-aa`/`dgb`;
- counted session: `rho-neg`/`rho-neg-aa`, `rho-neg`/`dg`, `rho-neg`/`dgb`.

## 6. Decision rule (per size, fixed now)

### 6.1 Instrument

**Wall session.** All of the following must hold. Each is read from the job's own files after the `SHA256SUMS` check.

1. **Exit:** `job-exit.json` records `exit_code` 0 from a single pod. Any other code makes the size undecidable, including 42 (an execution below L2) and 43 (native A/A admission refused). The v3 exit-42 clause does not apply to the wall session.
2. **Audit:** `audit.json` is `ok`, the session is `complete`, there are 0 problems, and every measured replay is `identical`.
3. **Counted A/A:** `ops.ratio_b_over_a` is exactly 1.000 at w = 0, 0.1 and 1 for both `--a dg --b dg-aa` and `--a dgb-edge --b dgb`.
4. **Spec and build:** `input-spec.json` is JSON-equal to the frozen `spec-P<b>.json`, and `build-info.json` names `be2969ce` and ecbench sha256 `88070375…412c`.
5. **Gate:** `l2-gate.json` has `accepted` true, every execution is graded L2 or L3, and `invalid_outcomes` is empty.
6. **Native A/A admission:** `control-comparison.json` is present. It is `run-ecbench`'s `--a dgb-edge --b dg-aa` comparison. It must show:
   - `wall.status` `"admitted"`;
   - `wall.pairs` 288 and `wall.pairs_below_level` 0;
   - `wall.aa_pairs` 288 and a non-null `wall.aa_ci95`;
   - `wall.self_aa` false;
   - `wall.aa_arms` equal, as a set, to {`dgb`, `dgb-edge`}, and `wall.aa_selection` `"includes arm A"`.
7. **Decision block:** the w = 0 `dg`/`dgb` file must show:
   - `wall.status` `"admitted"`;
   - `wall.pairs` 288 and `wall.pairs_below_level` 0;
   - `wall.self_aa` false;
   - `wall.aa_arms` `["dg", "dg-aa"]` and `wall.aa_selection` `"includes arm A"`;
   - `wall.aa_pairs` 288;
   - a non-null `wall.corrected_ci95`.

   The w = 0.1 and w = 1 files carry an identical `wall` block.
8. **Identical charged work.** AGENTS.md §6 requires a new oracle to be cross-checked against the one it replaces on every input of a full run. Two things must hold:
   - In the `dg`/`dgb` files, `ops.ratio_b_over_a` is exactly 1 at w = 0, 0.1 and 1.
   - The analysis pairs records from `records.jsonl`. Each `dgb` and `dgb-edge` record, warm-ups included, is matched with the `dg` record of the same curve, target and round. Each match must agree on:
     - the recovered logarithm;
     - `total_gae`;
     - every unpriced-operation count that `ecbench band` prices;
     - every per-phase operation count.

   Any difference makes the size undecidable.
9. **Identity:** in the w = 0 `--a dg-aa --b dgb` file, `wall.pairs` is 288, and `wall.ratio_b_over_a` equals the decision file's `wall.corrected_ratio` to within a relative 1e-9.

**Counted session.** It is handled as in XP-20261029-dg-wall §6.1 "Counted session":
- conditions 1–6 of v3 §6.1, with `rho-neg-aa / rho-neg` as the A/A and condition 4 read as "every `ic.pipeline` execution";
- the exit-42 clause verbatim;
- the exit-43 clause: exit 43 satisfies the route if, and only if, `l2-gate.json` is accepted, conditions 1–6 hold, and `control-comparison.json` has `wall.self_aa` true.

In addition, the counted session's `dg` and `dgb` records must equal the wall session's records of the same arm, curve, target and round, in the fields listed in wall condition 8.

A counted-session failure does not change the wall decision. It blocks the validation gate for that size, and it is reported.

### 6.2 Decision

The packed-table oracle is a **wall-clock improvement at N_inc** at size b if, and only if, both of the following hold:
- the wall session's §6.1 conditions 1–9 hold;
- the upper bound of `wall.corrected_ci95` for `dgb / dg` in the w = 0 file is below 1.

| result at b (wall-session §6.1 holds) | reading |
|:--|:--|
| corrected upper bound < 1 | improvement: lower wall cost per operation, identical charged work |
| corrected interval contains 1 | null at this sample size |
| corrected lower bound > 1 | regression |

If wall-session §6.1 fails, the size is undecidable.

**No baseline changes at any size.** Counted `S` is identical (condition 8). An improvement is therefore a component speed-up of the oracle (AGENTS.md §3). It is not a counted improvement, and the headline stays counted `S` (AGENTS.md §6). Every size reports `dg/rho-neg` and `dgb/rho-neg` from the counted session at the three weights, as the distance remaining.

**Position control (fixed now; decides nothing).** At each size the README reports, from the wall session:
- the edge-position estimate `dgb-edge / dg` (`ratio_b_over_a` with `ci95`, uncorrected);
- the decision's corrected ratio, measured at the middle positions;
- the two middle-over-edge contrasts: `aa_ratio` of the decision file, and `dgb / dgb-edge`.

If the edge-position interval does not overlap the decision's `corrected_ci95`, the decision is still read by the table above. The README then states two things:
- the oracle's effect depends on block position, by at least the gap between the two intervals;
- the block positions at which each interval was measured.

**Scope.** The scope is:
- c8a.2xlarge (AMD), image `cf0dcc8b…`, ecbench `88070375…412c` built from `be2969ce`;
- the six curves of §3;
- N_inc, `targets = stride-renew-deferred-guard`, `incremental-gauss`.

Not measured: other host classes (Intel, Arm), other factor bases, other target rules, and the batched builder.

## 7. Predictions (before any data)

Model: the relation phase is unchanged (the probe cost on the packed table is unknown and taken as equal), and the set-up falls to 0.25–0.5 of the scalar build (the 2026-10-20 study's range at N = 1,024, widened because smaller N was not measured). The set-up share of `dg`'s solve wall at N_inc in XP-20261029-dg-wall's wall session (`phase_sums-stdout.txt`, Σ `phases.oracle_setup.wall_ns` / Σ `time.solve_wall_ns`, read by hand) is 0.25 at 24 bits, 0.31 at 28 and 0.29 at 32. The predicted ratio is 1 − share × (0.5 to 0.75).

| b | predicted corrected `dgb / dg` ns/GAE |
|--:|:--|
| 24 | 0.81–0.87 |
| 28 | 0.77–0.84 |
| 32 | 0.78–0.85 |

Model risk:
- the packed probe touches a smaller region per key and may be faster (ratio below the range) or, through the extra region lookup, slower (ratio above it); the online windows are reported descriptively;
- at 24 bits the table is small (N 162, about 26k entries) and the scalar build may already be cache-resident, so the gain may be smallest there; a null at 24 bits is the first failure the model allows.
- the 2026-10-20 study's whole-run blocked/scalar wall ratio at N = 1,024 was 0.91. That figure is descriptive, mostly L1, from an Intel 4-vCPU VM with the default targets rule. It is above the roughly 0.84 that the study's own means imply: set-up 216 ms → 55 ms, against about 0.8 s of relations. If the packed probe accounts for the difference, the 32-bit ratio here will lie above the predicted range. The predictions above do not use this figure; it was seen before drafting (§8).

## 8. Disclosure

- **Seen before drafting:**
  - XP-20261029-dg-wall's results: its verdict, README and validator report, including the phase sums used in §7.
  - The 2026-10-20 blocked-table study, including every wall figure in its README. At N = 1,024 those are the set-up ratio blocked/scalar of 0.25 [0.15, 0.43] and the whole-run ratios 0.91 against scalar and 1.03 against batched. All are descriptive, mostly L1, from an Intel 4-vCPU VM with the default targets rule.
  - The local pilot's L0 wall ratio (§3).
- **Earlier wall data of this oracle pair.**
  - The 2026-10-20 sessions (`sessions/A-1k-8k`, `sessions/B-16k-r2`) are descriptive. They have no admitted wall pair, and they differ from this study in targets rule, host and N. They are never pooled with this study and decide nothing.
  - Before the freeze commit, the author runs `git grep -l 'mitm-prime-blocked' origin/main -- research benchmarks backlog docs/ic`, and the same command on each open PR branch, and lists the output here.
  - Any session found there that pairs `mitm-prime-blocked` with `mitm:negation_folded=1` at L2 is named here.
  - **Output of that search (author, 2026-10-29, before the freeze commit).**
    - `origin/main` (at its 2026-10-29 fetch): 112 files. The only specs that pair `mitm-prime-blocked` with `mitm:negation_folded=1` are the 2026-10-20 study's (`research/ic_oracle_setup_blocked_20261020/specs/A-1k-8k.json`, `B-16k.json`, and their session copies), already disclosed above. The planner studies (`ic_planner_validation_20261020`, `ic_planner_weight_rule_20261021`, `ic_planner_exhaustive_20261026`) and `ic_prime_residual_lp_20261026` use `mitm-prime-blocked` against other oracles and never `mitm:negation_folded=1`; the remaining hits are figures, tables, method listings and leaderboard text.
    - Open PR branches: all 98 open PR heads were fetched (2026-10-29 about 12:30Z). Files beyond `origin/main` with the string are on #2380, #2462, #2783, #2806, #2807 and #2844 (93 files, listed in `prior-data-search.txt`, under `research/ecbench_rho_lanes_20261023/`, `research/ic_wide_ext_20261028/` and `research/twist_routes_1_3_20261021/`). No spec or session among them contains `mitm:negation_folded=1`. #2849 (this branch's base) adds none.
    - So no earlier session pairs the two oracles at L2, and nothing is sealed.
  - **Related result seen before the freeze (author).** PR #2844 (`research/ic_table_floor_20261029`, lead `ICP-prime-ic-constant-engineering`) reports a descriptive micro-benchmark on a shared machine: a one-region flat packed pair table probes 2.3–2.9× slower on misses than the std map (1.3–1.5× with prefetch), on the batched prime lane-walk tables. It ran no ecbench session. It differs from this study in layout (one region against cache-sized regions), code path (lane walk) and level (not L2). It bears on the "probe cost taken as equal" assumption of §7, as E6's bullet does, and it decides nothing here. It is sealed for every agent on this study (author, director, executor and validator) until this study's verdict commit is pushed. Afterwards it is reported descriptively and never pooled, as in XP-20261029-dg-wall §8.
- **The coordinator's review.** In this review the coordinator read:
  - XP-20261029-dg-wall's README and VALIDATION.json;
  - parts of its validator report: the P24 descriptive wall blocks with `walk` as arm A, the memory-and-scheduling means, `arm_positions_alternate_order`, and the A/A selection labels;
  - the 2026-10-20 README.

  The coordinator opened no pilot output and no data of this study.
- **Arm order.** The arm order (§2) was set by the coordinator from XP-20261029-dg-wall's position table, before any data of this study.
- **Curves** are fresh. **N_inc** is fixed by the earlier studies, not chosen here.

## 9. Freezing

This file, `APPROVAL.md` and the six specs are frozen by the commit that adds them. Before the first job launches that commit is pushed and its SHA and the six spec sha256 are recorded on every launch card. Each request embeds its spec byte for byte. No amendment is allowed after the first launch of any of the six jobs.

## 10. Outputs

`research/ic_dg_blocked_20261029/` holds, per size, `P<b>/wall/` and `P<b>/counted/`. Each holds:
- the launch receipts and the S3 copy with hashes;
- the `compare` JSON for every pair of §5, at the three weights;
- the band table.

It also holds a README with a figure and a PDF (research-visuals). The README's position table gives, per size, each arm's block positions (from `records.jsonl`) and the position-control figures of §6.2.

**Independent validation.** The validator, using its own code on the wall session's `records.jsonl`:
- recomputes `ns per GAE`, `ratio_b_over_a`, `aa_ratio`, `corrected_ratio` and `corrected_ci95`, and the identity of condition 9;
- checks condition 8 record by record, warm-ups included;
- checks that the counted session's `dg` and `dgb` records equal the wall session's;
- tabulates each arm's block positions and its predecessor within the block, and confirms that `dgb` and `dg-aa` have the same position distribution (144 blocks at position 1 and 144 at position 2);
- reports, descriptively, the per-curve and target-clustered intervals and the position control;
- rechecks §6.1 for both sessions.

**Before any leaderboard or `benchmarks/` change**, an improvement passes the validation gate. `VALIDATION.json` must name:
- an independent replay of both sessions;
- the validator report;
- the matched rho from the counted session, with its §5a band.

**Placement.**
- At each size with an improvement, the result goes in the pair-table component section of `scripts/build_ic_leaderboard.py` (`pair_table_component`, outside tables A–C), labelled "whole-solve wall per charged GAE at N_inc".
- If that section holds set-up figures only, it goes instead in a new component section outside tables A–C, named for the oracle. The evidence review records which.
- It also goes in `benchmarks/<YYYYMMDD>-<slug>/`, with a row in `benchmarks/README.md`.
- A null or a regression is reported in the README only.

Wall-time figures enter only for the IC arms of the wall session. Rho wall time, and every wall field of the counted session, are never used.
