# Coordinator decision — XP-20261029-dg-blocked

## Status

**Approved with edits** (2026-10-29, `coordinator` role, before any execution of these specs). The approval requires edits E1–E10 and the pre-launch checks P1–P5 below.

- **Edits.** Apply E1–E10 verbatim in the commit that adds this file. That commit is the freeze (§9).
- **Pre-launch checks.** P1–P5 fail closed. If any check fails, nothing launches and the protocol returns to the coordinator. A failed check is not an amendment.
- **Reviewed:**
  - `PROTOCOL.md` and the six specs of this directory, as drafted;
  - `backlog/XP-20261029-dg-wall/` (`PROTOCOL.md`, `APPROVAL.md` with E1–E11, the six specs);
  - from `research/ic_dg_wall_20261029/`: `README.md`, `VALIDATION.json`, and parts of `validation/validator-report.json` (listed in §8 as edited by E7);
  - `research/ic_oracle_setup_blocked_20261020/README.md`, read from the `/home/user/crypto` working tree. I could not run git, so I have not confirmed that this copy equals `origin/main`.
  - AGENTS.md §5a, §6, §8, §9a, §10, "Success criterion" and the validation gate;
  - the working-tree copies of `taskq/deploy/eks-karpenter/run-ecbench` and `src/cryptanalysis/ecbench/compare.rs`. The `compare.rs` there is older than `be2969ce`: it has no `aa_selection` field. Because of that, P1 has to confirm the tool's behaviour at the frozen revision.
- **Main defect.** As drafted, the design does not do what its §2 says it does, for two reasons:
  - the decision statistic would carry exactly the position effect that the `dgb-lead` arm is meant to expose;
  - every size would fail its own condition 6.

  E1, E3, E4 and E9 fix both by reordering and renaming the arms. No arm, oracle, seed or budget changes.

## Edits

### E1 — replace §2 in full

```markdown
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
```

### E2 — replace the pilot paragraph of §3

Old:

```markdown
A local pilot (pilot seed 909090, one 32-bit curve, 3 targets, L0, not a study curve) checked only that the two oracles run and charge the same operations (`ops` ratio 1.0). Its wall ratio was seen; it is descriptive at L0 and plays no part here.
```

New (the author fills in the three angle-bracket fields from the pilot's own files before the freeze commit):

```markdown
A local pilot (pilot seed 909090, one 32-bit curve, 3 targets, L0, not a study curve) checked only that the two oracles run and charge the same operations (`ops` ratio 1.0). It ran with ecbench sha256 `<pilot binary sha256>`, and its output is kept at `<pilot output path>`. Its wall ratio `dgb / dg`, `<value as seen>`, was seen. That ratio is descriptive at L0 and plays no part here.
```

### E3 — replace §5 in full

```markdown
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
```

### E4 — replace §6.1 in full

```markdown
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
```

### E5 — replace §6.2 in full

```markdown
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
```

### E6 — §7: add a third model-risk bullet

After the bullet that ends `a null at 24 bits is the first failure the model allows.`, add:

```markdown
- the 2026-10-20 study's whole-run blocked/scalar wall ratio at N = 1,024 was 0.91. That figure is descriptive, mostly L1, from an Intel 4-vCPU VM with the default targets rule. It is above the roughly 0.84 that the study's own means imply: set-up 216 ms → 55 ms, against about 0.8 s of relations. If the packed probe accounts for the difference, the 32-bit ratio here will lie above the predicted range. The predictions above do not use this figure; it was seen before drafting (§8).
```

### E7 — replace §8 in full

```markdown
## 8. Disclosure

- **Seen before drafting:**
  - XP-20261029-dg-wall's results: its verdict, README and validator report, including the phase sums used in §7.
  - The 2026-10-20 blocked-table study, including every wall figure in its README. At N = 1,024 those are the set-up ratio blocked/scalar of 0.25 [0.15, 0.43] and the whole-run ratios 0.91 against scalar and 1.03 against batched. All are descriptive, mostly L1, from an Intel 4-vCPU VM with the default targets rule.
  - The local pilot's L0 wall ratio (§3).
- **Earlier wall data of this oracle pair.**
  - The 2026-10-20 sessions (`sessions/A-1k-8k`, `sessions/B-16k-r2`) are descriptive. They have no admitted wall pair, and they differ from this study in targets rule, host and N. They are never pooled with this study and decide nothing.
  - Before the freeze commit, the author runs `git grep -l 'mitm-prime-blocked' origin/main -- research benchmarks backlog docs/ic`, and the same command on each open PR branch, and lists the output here.
  - Any session found there that pairs `mitm-prime-blocked` with `mitm:negation_folded=1` at L2 is named here. It is sealed for every agent on this study (author, director, executor and validator) until this study's verdict commit is pushed. Afterwards it is reported descriptively and never pooled, as in XP-20261029-dg-wall §8.
- **The coordinator's review.** In this review the coordinator read:
  - XP-20261029-dg-wall's README and VALIDATION.json;
  - parts of its validator report: the P24 descriptive wall blocks with `walk` as arm A, the memory-and-scheduling means, `arm_positions_alternate_order`, and the A/A selection labels;
  - the 2026-10-20 README.

  The coordinator opened no pilot output and no data of this study.
- **Arm order.** The arm order (§2) was set by the coordinator from XP-20261029-dg-wall's position table, before any data of this study.
- **Curves** are fresh. **N_inc** is fixed by the earlier studies, not chosen here.
```

### E8 — replace §10 in full

```markdown
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
```

### E9 — replace `spec-P24.json`, `spec-P28.json`, `spec-P32.json` in full

`spec-P24.json`:

```json
{
 "schema": "ecbench.spec/v1",
 "label": "XP-20261029-dg-blocked P24 wall session: wall time per operation, prime-blocked pair table against the folded pair table at N 162",
 "description": "Frozen protocol (backlog/XP-20261029-dg-blocked/PROTOCOL.md and APPROVAL.md): ns per GAE of ic.pipeline targets stride-renew-deferred-guard with oracle mitm-prime-blocked:m=2 (dgb) against the same with oracle mitm:negation_folded=1 (dg), same factor base prime-abscissa:size=162 (identical charged work), A/A-corrected by dg-aa, on two fresh 24-bit prime-order curves. Arm order dg, dg-aa, dgb, dgb-edge puts dgb and dg-aa at the same middle positions of the alternate order; dgb-edge (a copy of dgb; structural reference role for native A/A admission) and dg share the edge positions and give a descriptive position control. No rho arm: the matched rho is in spec-P24-rho.json. Public synthetic curves and targets; no security claim.",
 "workloads": {
  "curves": [
   {"kind": "prime_search", "bits": 24, "seed": 202610295243},
   {"kind": "prime_search", "bits": 24, "seed": 202610295244}
  ],
  "targets_per_curve": 24,
  "target_seed": 202610295246,
  "target_kind": "public"
 },
 "arms": [
  {"name": "dg", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm:negation_folded=1", "max_trials": "4243777", "targets": "stride-renew-deferred-guard"}}},
  {"name": "dg-aa", "role": "control", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm:negation_folded=1", "max_trials": "4243777", "targets": "stride-renew-deferred-guard"}}},
  {"name": "dgb", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm-prime-blocked:m=2", "max_trials": "4243777", "targets": "stride-renew-deferred-guard"}}},
  {"name": "dgb-edge", "role": "reference", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=162", "oracle": "mitm-prime-blocked:m=2", "max_trials": "4243777", "targets": "stride-renew-deferred-guard"}}}
 ],
 "measurement": {"rounds": 6, "warmup": 1, "order": "alternate", "seed": 202610295248, "isolation_required": "L2", "timeout_seconds": 1800}
}
```

`spec-P28.json`:

```json
{
 "schema": "ecbench.spec/v1",
 "label": "XP-20261029-dg-blocked P28 wall session: wall time per operation, prime-blocked pair table against the folded pair table at N 407",
 "description": "Frozen protocol (backlog/XP-20261029-dg-blocked/PROTOCOL.md and APPROVAL.md): ns per GAE of ic.pipeline targets stride-renew-deferred-guard with oracle mitm-prime-blocked:m=2 (dgb) against the same with oracle mitm:negation_folded=1 (dg), same factor base prime-abscissa:size=407 (identical charged work), A/A-corrected by dg-aa, on two fresh 28-bit prime-order curves. Arm order dg, dg-aa, dgb, dgb-edge puts dgb and dg-aa at the same middle positions of the alternate order; dgb-edge (a copy of dgb; structural reference role for native A/A admission) and dg share the edge positions and give a descriptive position control. No rho arm: the matched rho is in spec-P28-rho.json. Public synthetic curves and targets; no security claim.",
 "workloads": {
  "curves": [
   {"kind": "prime_search", "bits": 28, "seed": 202610295283},
   {"kind": "prime_search", "bits": 28, "seed": 202610295284}
  ],
  "targets_per_curve": 24,
  "target_seed": 202610295286,
  "target_kind": "public"
 },
 "arms": [
  {"name": "dg", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm:negation_folded=1", "max_trials": "26479085", "targets": "stride-renew-deferred-guard"}}},
  {"name": "dg-aa", "role": "control", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm:negation_folded=1", "max_trials": "26479085", "targets": "stride-renew-deferred-guard"}}},
  {"name": "dgb", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm-prime-blocked:m=2", "max_trials": "26479085", "targets": "stride-renew-deferred-guard"}}},
  {"name": "dgb-edge", "role": "reference", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=407", "oracle": "mitm-prime-blocked:m=2", "max_trials": "26479085", "targets": "stride-renew-deferred-guard"}}}
 ],
 "measurement": {"rounds": 6, "warmup": 1, "order": "alternate", "seed": 202610295288, "isolation_required": "L2", "timeout_seconds": 1800}
}
```

`spec-P32.json`:

```json
{
 "schema": "ecbench.spec/v1",
 "label": "XP-20261029-dg-blocked P32 wall session: wall time per operation, prime-blocked pair table against the folded pair table at N 1024",
 "description": "Frozen protocol (backlog/XP-20261029-dg-blocked/PROTOCOL.md and APPROVAL.md): ns per GAE of ic.pipeline targets stride-renew-deferred-guard with oracle mitm-prime-blocked:m=2 (dgb) against the same with oracle mitm:negation_folded=1 (dg), same factor base prime-abscissa:size=1024 (identical charged work), A/A-corrected by dg-aa, on two fresh 32-bit prime-order curves. Arm order dg, dg-aa, dgb, dgb-edge puts dgb and dg-aa at the same middle positions of the alternate order; dgb-edge (a copy of dgb; structural reference role for native A/A admission) and dg share the edge positions and give a descriptive position control. No rho arm: the matched rho is in spec-P32-rho.json. Public synthetic curves and targets; no security claim.",
 "workloads": {
  "curves": [
   {"kind": "prime_search", "bits": 32, "seed": 202610295323},
   {"kind": "prime_search", "bits": 32, "seed": 202610295324}
  ],
  "targets_per_curve": 24,
  "target_seed": 202610295326,
  "target_kind": "public"
 },
 "arms": [
  {"name": "dg", "role": "baseline", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm:negation_folded=1", "max_trials": "167871873", "targets": "stride-renew-deferred-guard"}}},
  {"name": "dg-aa", "role": "control", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm:negation_folded=1", "max_trials": "167871873", "targets": "stride-renew-deferred-guard"}}},
  {"name": "dgb", "role": "candidate", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm-prime-blocked:m=2", "max_trials": "167871873", "targets": "stride-renew-deferred-guard"}}},
  {"name": "dgb-edge", "role": "reference", "method": {"id": "ic.pipeline", "params": {"factor_base": "prime-abscissa:size=1024", "oracle": "mitm-prime-blocked:m=2", "max_trials": "167871873", "targets": "stride-renew-deferred-guard"}}}
 ],
 "measurement": {"rounds": 6, "warmup": 1, "order": "alternate", "seed": 202610295328, "isolation_required": "L2", "timeout_seconds": 1800}
}
```

### E10 — counted specs: name the approval in the description

In each of `spec-P24-rho.json`, `spec-P28-rho.json` and `spec-P32-rho.json`, replace the substring

`Frozen protocol (backlog/XP-20261029-dg-blocked/PROTOCOL.md):`

with

`Frozen protocol (backlog/XP-20261029-dg-blocked/PROTOCOL.md and APPROVAL.md):`

Nothing else in the counted specs changes.

## Pre-launch checks

These checks are read-only, sit on the director's card, and all run before the first launch. Each fails closed.

- **P1 — tool behaviour at the frozen revision.**
  - `git show be2969cebf30a0cfb79088fd10094324c3efb3bb:taskq/deploy/eks-karpenter/run-ecbench` must show three things:
    - the reference is the first `role == "reference"` arm in spec order, and the control is the first `role == "control"` arm in spec order;
    - exit 43 is conditioned on `.wall.status=="admitted" and .wall.pairs_below_level==0 and .wall.aa_pairs>=5 and .wall.aa_ci95!=null`;
    - nothing requires the reference to be at index 0.
  - `git show be2969ce…:src/cryptanalysis/ecbench/compare.rs` must show four things:
    - `choose_aa` tries arm A's same-method twin first, then the reference arm's, then the first alphabetical pair;
    - `self_aa` is keyed on `method_id`;
    - `method_id` is derived from the method id and params, not from the arm name;
    - `corrected_ratio` is `ratio_b_over_a / aa_ratio`.
  - The `alternate` order at `be2969ce` must run each measured block either in spec order or reversed, 144 blocks each per size.
- **P2 — the oracle.**
  - `git merge-base --is-ancestor e59331c96 be2969ce` must succeed.
  - At `be2969ce`, `src/cryptanalysis/ic_framework/plugins.rs` must route two strings:
    - `mitm-prime-blocked:m=2` to `PairTable::build_negation_folded_prime_blocked` / `PackedPairTable`;
    - `mitm:negation_folded=1` to the scalar `PairTable::build_negation_folded`.
  - The blocked oracle must refuse, with an error rather than a silent fallback, a field with p ≥ 2³² or more than 2¹⁶ signed points.
  - If E2's pilot binary is not `88070375…412c`, run the pilot spec once locally with that binary at L0. Read only `outcome.status`, `outcome.matches_target` and the `ops` block, and no wall field.
- **P3 — the specs.**
  - `ecbench plan` with binary `88070375…412c` must exit 0 on all six specs. It must print the §3 curve slugs, identical between the wall and counted spec of each size, and list the arms in spec order with their roles.
  - `jq -S` must show the following for each wall spec:
    - the arms are `dg`, `dg-aa`, `dgb`, `dgb-edge`, with roles baseline, control, candidate and reference;
    - `dg-aa`'s params equal `dg`'s, and `dgb-edge`'s equal `dgb`'s;
    - `dgb`'s params differ from `dg`'s only in `oracle`;
    - `dg`'s params equal those of the `dg` arm of `backlog/XP-20261029-dg-wall/spec-P<b>.json`.
  - For each counted spec:
    - its `dg` and `dgb` params equal the wall spec's;
    - its `rho-neg` and `rho-neg-aa` arms equal those of `backlog/XP-20261029-dg-wall/spec-P<b>-rho.json`;
    - its `workloads` and `measurement` objects equal the wall spec's.
- **P4 — checkpoint and scope.** `.resume/RESUME.md` is filled and pushed (`scripts/checkpoint.sh init`). A Conductor task holds the `backlog/XP-20261029-dg-blocked/` and `research/ic_dg_blocked_20261029/` scopes.
- **P5 — edits applied.**
  - `grep -n 'dgb-lead'` on `PROTOCOL.md` and the six specs returns nothing.
  - E2's three fields are filled.
  - E7's `git grep` output is listed.

## Reasons

### 1. Design soundness

**What is sound:**
- **Single variable.** Every IC param is equal except `oracle`, and so are curves, targets, seeds, `max_trials` and N_inc. I checked this in all six specs.
- **Identical charged work.** This is the right framing. The 2026-10-20 study found 456/456 executions identical in counts across the builders. Condition 8 re-establishes it record by record, under this study's target rule. That also satisfies AGENTS.md §6's requirement to cross-check a new oracle on every input.
- **The session split** is carried over from XP-20261029-dg-wall. A wall session without rho can earn native L2, replay and A/A admission (AGENTS.md §9a).

**Blocking defect 1: the corrected decision ratio absorbs a position effect.**
- **How `alternate` places arms.** The validator's `arm_positions_alternate_order` for XP-20261029-dg-wall (five arms at {0,4}, {1,3}, {2}, {1,3} and {0,4}, 144 each) shows that `alternate` runs blocks forward and reversed.
- **Where the drafted order puts the arms:**
  - `dgb-lead` at {0, 3};
  - `dg` at {1, 2};
  - `dgb` at {2, 1};
  - `dg-aa` at {3, 0}.
- **What follows:**
  - The protocol's claim that `dgb-lead` "runs before the baseline in every block" is false: in reversed blocks it runs last, after `dg`.
  - On identical charged work and fully admitted pairs, `corrected_ratio` equals Σ wall(`dgb`) / Σ wall(`dg-aa`) exactly. You can check that `corrected_ratio = ratio_b_over_a / aa_ratio` holds in the earlier study's numbers, for example its P24 `walk__srenew` block.
  - The drafted decision therefore compared the middle positions (`dgb`) with the edge positions (`dg-aa`). That is the cold-cache and one-time-cost contrast AGENTS.md §10 warns of. It is also the confound the new arm was meant to address, now moved into the decision statistic.
  - The drafted uncorrected `dgb / dg` was position-balanced. Deciding on it would drop the A/A noise floor from the decision interval, so I rejected that option.
- **The fix (E1, E9)** reorders the wall arms to `dg`, `dg-aa`, `dgb`, `dgb-edge`.
  - `dgb` and `dg-aa` share the middle positions, alternate precedence, and are balanced in the oracle of their within-block predecessor.
  - The edge pair gives a second, position-matched oracle estimate.
  - Nothing else changes: same arms, oracles, seeds and budget.
  - Condition 9 (E4) checks the identity that the design relies on.

**Blocking defect 2: native admission would select a different A/A than §6.1 requires.**
- `run-ecbench` compares the first `reference` arm with the first `control` arm. As drafted, that is `--a dgb-lead --b dg-aa`.
- `dgb-lead` and `dgb` have identical params, so they share a `method_id` and are twins.
- `choose_aa` tries arm A's twin first. The approval of XP-20261029-dg-wall read this order from the `be2969ce` source. Its validator report agrees: `walk__srenew` was labelled "first alphabetical" even though arm B had a twin, and `srenew__dg-small` was labelled "includes arm A".
- So `control-comparison.json` would carry `aa_arms` {`dgb`, `dgb-lead`}. The job exits 0, because the exit-43 test reads only status, pairs and `aa_ci95`.
- But drafted §6.1 maps dg-wall's condition 6 to `aa_arms` `["dg", "dg-aa"]`. Every size would therefore have been undecidable.
- The chained rename "`srenew` as `dg` … and `dg` as `dgb`" is also ambiguous if applied in sequence.
- E4 writes §6.1 out in full, and it expects {`dgb`, `dgb-edge`} with "includes arm A". That pair is the correct one, and it is admissible: two different arms with one method, 288 L2 pairs, a non-null `aa_ci95`, and `self_aa` false.
- I could not read `compare.rs` at `be2969ce`, so P1 confirms this. Conditions 6 and 7 fail closed if it differs.

### 2. Spec consistency

**Checked and correct:**
- curve seeds 202610295`<b>`3 and 202610295`<b>`4, target seed `<b>`6, order seed `<b>`8;
- 24 targets per curve, 6 rounds plus 1 warm-up, `alternate` order, L2, 1800 s timeout;
- N_inc 162, 407 and 1024;
- `max_trials` 4243777, 26479085 and 167871873, equal to the frozen XP-20261029-dg-wall specs;
- `targets` `stride-renew-deferred-guard`;
- oracle strings `mitm:negation_folded=1` and `mitm-prime-blocked:m=2`, the name the 2026-10-20 README uses;
- roles accepted by `run-ecbench`'s spec gate;
- 1,344 executions per session and 288 pairs per arm pair;
- rho arms `{"id": "rho.negation"}` as in the dg-wall counted specs;
- wall and counted workloads and measurement identical per size.

**Defects:**
- the drafted arm order and the false description of `dgb-lead` (E1, E9);
- the counted specs omit "and APPROVAL.md" from their provenance line (E10, cosmetic).

### 3. Decision rule and predictions

- The rule (corrected upper bound below 1, per size, three-way reading) is fixed before data, and the predictions are stated per size with a model. The arithmetic is right to rounding: 1 − share × (0.5–0.75) gives 0.8125–0.875, 0.7675–0.845 and 0.7825–0.855.
- The model omits one figure that was seen. The 2026-10-20 whole-run ratio of 0.91 at N = 1,024 sits above what that study's own set-up means imply, and it bears directly on the protocol's "probe cost taken as equal" assumption. E6 adds it as model risk. The predictions still decide nothing, and they are not changed.
- The order caveat is rewritten (E5) for the corrected design, because the drafted version compared a contaminated interval with an edge-versus-middle one.

### 4. Native A/A admission

See Reasons §1, defect 2. With E1 and E9:
- `run-ecbench` compares `dgb-edge` with `dg-aa`;
- `choose_aa` takes arm A's twin `dgb`, giving `aa_arms` {`dgb`, `dgb-edge`} with "includes arm A";
- that comparison has 288 admitted pairs and a non-null `aa_ci95`, so the job exits 0.

The decision file `--a dg --b dgb` takes arm A's twin `dg-aa` ("includes arm A"), as condition 7 requires.

`dg` cannot be the reference, for the same reason `srenew` could not be in XP-20261029-dg-wall: `dg` against `dg-aa` is `self_aa`, which leaves no `aa_ci95`, so the job would exit 43.

### 5. Disclosure

**Missing from the draft:**
- the 2026-10-20 whole-run wall ratios (0.91 against scalar and 1.03 against batched at N = 1,024), which were seen but not stated;
- the pilot's binary, its output path, and the value of the pilot wall ratio that was seen;
- a search for earlier wall data scoped to the oracle pair rather than to the pairing with this target rule, with sealing if any is found;
- the coordinator's own access.

E2 and E7 fix these.

### 6. Residual risks, accepted and disclosed

- **24-bit sub-millisecond solves.** In XP-20261029-dg-wall the 24-bit IC means were about 3.6–3.9 ms, and the predicted `dgb` is about 3 ms. A solve under 1 ms would make the job exit 42, and the size undecidable. It would be reported and not rerun.
- **The interval treats the 288 pairs as independent.** They are 48 targets × 6 rounds. The validator's target-clustered interval is descriptive.
- **Coverage.** Two curves per size, and one host class.
- **Carry-over across blocks.** At block boundaries this is not balanced for the edge arms. Only the descriptive position control depends on it.

### 7. Rules and roadmap

- **Success criterion.** This is not relabelling or accounting. The counted `S` is identical by construction and stays the headline. A faster executed data structure is a real component speed-up, placed outside tables A–C (E8).
- **AGENTS.md §8.** No end-to-end counted speedup is claimed, so §8's total-operations requirement does not bind. Its runtime clause (paired reruns on matched hardware, with a 95% paired interval) is met by the design.
- **Why approve constant engineering now.** It is constant engineering, not an exponent mechanism. I approve it because it is cheap (nothing is rebuilt, six jobs on an existing route), because it closes the constant that `LEAD-ICP-prime-ic-constant-engineering` names, and because no exponent claim rides on it.

## Not changed

- §1, §4 and §9 stand as written.
- §3's curves and seeds stand, apart from E2.
- §7's predictions stand, apart from the added model-risk bullet.

## Decision record

The decision record is not yet in the ledger. I had no shell in this engagement, so I could not allocate a `DEC-*` id with `tools/allocate_id.py`. The session that commits this file must allocate the id, confirm it with `--check`, and archive the record below as `ledger/decisions/DEC-20261029-<tok>.yaml`.

```yaml
type: coordinator_decision
id: DEC-20261029-<allocate>
date: 2026-10-29
subject: backlog/XP-20261029-dg-blocked/PROTOCOL.md (+ spec-P{24,28,32}.json, spec-P{24,28,32}-rho.json)
decision: approve_with_edits
edits: [E1, E2, E3, E4, E5, E6, E7, E8, E9, E10]
conditions: [P1, P2, P3, P4, P5]   # fail-closed: no launch, return to coordinator
rationale:
  - "Single-variable change (oracle only) with identical charged work checked record by record. The engineering/component class is correct, with no counted or baseline claim."
  - "As drafted, alternate order puts dgb in the middle and dg-aa at the block edges. Since corrected_ratio = sum wall(dgb) / sum wall(dg-aa) on identical work, the decision would carry the middle-versus-edge position effect. The reorder dg, dg-aa, dgb, dgb-edge (E1, E9) puts dgb and dg-aa at the same positions; condition 9 checks the identity."
  - "As drafted, run-ecbench's admission compare (dgb-lead vs dg-aa) selects A/A {dgb, dgb-lead} by arm A's twin, but section 6.1 condition 6 required [dg, dg-aa]; every size would have been undecidable. E4 writes 6.1 in full."
  - "Disclosure must state the seen 2026-10-20 whole-run ratios and the pilot value, and seal any earlier L2 wall data of the oracle pair (E2, E6, E7)."
evidence_refs:
  - research/ic_dg_wall_20261029/validation/validator-report.json (arm_positions_alternate_order; aa_selection labels)
  - research/ic_dg_wall_20261029/README.md
  - backlog/XP-20261029-dg-wall/APPROVAL.md (choose_aa reading at be2969ce)
  - research/ic_oracle_setup_blocked_20261020/README.md (working-tree copy; not verified against origin/main)
  - taskq/deploy/eks-karpenter/run-ecbench (lines 59-64, working-tree copy)
scope: "prime-field IC, b in {24, 28, 32}, N_inc, stride-renew-deferred-guard, mitm-prime-blocked:m=2 vs mitm:negation_folded=1, incremental-gauss, c8a.2xlarge, image cf0dcc8b, ecbench be2969ce; engineering (constant); no exponent claim"
next_actions:
  - "Author: apply E1-E10 verbatim; fill E2's fields and E7's git grep list. The director runs P1-P5. Commit and push the freeze; record the SHA and the six spec sha256 on the cards."
```
