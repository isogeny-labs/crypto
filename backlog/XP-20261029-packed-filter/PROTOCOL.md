# Prime-field IC: wall time per operation of a membership filter in front of the packed pair table — protocol

**Status:** drafted 2026-10-29, before any execution of these specs.

## 1. Question and class

The oracle `mitm-prime-blocked:m=2` (`dgb`) probes a packed, cache-blocked pair table (`PackedPairTable`, p < 2³², at most 2¹⁶ signed points) once per relation trial. XP-20261029-dg-blocked found it a wall-clock improvement over `mitm:negation_folded=1` at 28 bits, corrected `dgb / dg` 0.9300 [0.9266, 0.9333] (validator VAL-20261029-33a286, pass with caveats), and undecidable at 24 and 32 bits (one execution below L2 in each wall session). Its target-PDP phase rose by about a third against `dg` at 28 bits (1,062 → 1,428 ms phase sum), so the probe on the packed table costs more than the hash-map probe it replaced.

The design note `research/prime_ic_program_20261029/hashtable_prefetch/` (origin/main, design only) gives the reason: a miss in the hash map reads only its small control array, while a miss in a packed table reads the large slot array, and at N_inc almost every probe misses (hit rate about 1/1,024 at 28 bits). The lead `LEAD-ICP-prime-ic-constant-engineering` names this hash-table floor as the remaining constant of a relation trial.

Commit `3271a25d02dcef6da148293fb8f9e2149ce59858` adds the opt-in parameter `filter=1` to `mitm-prime-blocked`:
- after the slots are filled, a one-bit-per-slot membership filter is built: 8 bits per entry, rounded up to a power of two, at least 2¹² bits, one multiplicative hash per key;
- a lookup whose filter bit is clear returns "absent" without reading the slot array;
- a set bit falls through to the unchanged slot lookup, so there are no false negatives and every lookup returns what the unfiltered table returns;
- each bit set is charged as the new `bitop/v1` counter `table_filter_bit_sets` (6 word ops, `docs/units/BITOPS.md`, "2026-10-29 (packed pair-table filter)"); GAE is unchanged;
- `filter=1` refuses the small-maps layout (p ≥ 2³² or more than 2¹⁶ signed points) instead of falling back.

**Question.** At b ∈ {24, 28, 32}, with the incumbent factor base N_inc and `targets = stride-renew-deferred-guard`, is the ecbench wall statistic `ns per GAE` of the filtered oracle (`dgbf`) below that of the unfiltered packed oracle (`dgb`), after the A/A bias correction?

Class: engineering (constant), measured at L2. Charged GAE is identical by construction; this is a component speed-up, never a counted-unit claim. `bitop/v1` work rises by the filter's bit sets (§6.1 condition 8); the README reports that rise beside every wall figure.

## 2. Arms (two sessions per size)

Each size runs two jobs on the same curves, targets and seeds, split as in XP-20261029-dg-blocked §2:
- the **wall session** (`spec-P<b>.json`, job `ic-dgbf-p<b>-20261029`);
- the **counted session** (`spec-P<b>-rho.json`, job `ic-dgbf-rho-p<b>-20261029`).

Wall session, in spec order:

| index | arm | role | oracle | position in a forward / reversed block | reads |
|--:|:--|:--|:--|:--|:--|
| 0 | `dgb` | baseline | `mitm-prime-blocked:m=2` | 0 / 3 | decision (arm A); edge arm of the position control |
| 1 | `dgb-aa` | control | = `dgb` | 1 / 2 | wall A/A of the decision file; first control in spec order |
| 2 | `dgbf` | candidate | `mitm-prime-blocked:m=2,filter=1` | 2 / 1 | decision (arm B) |
| 3 | `dgbf-edge` | reference | = `dgbf` | 3 / 0 | position control (descriptive). The role is structural: `run-ecbench` compares the first `reference` arm with the first `control` arm for native A/A admission. |

Counted session: `rho-neg` (reference, `rho.negation`), `dgb` (baseline), `dgbf` (candidate), `rho-neg-aa` (control). No wall field of the counted session is read.

Every IC arm uses `targets = stride-renew-deferred-guard`, factor base `prime-abscissa:size=N_inc` (N_inc 162, 407 and 1024 at 24, 28 and 32 bits) and XP-20261029-dg-wall's `max_trials`. The other parameters are the `ecbench plan` defaults.

**Why this order.** It is XP-20261029-dg-blocked's order with the oracles shifted by one step, for the same reasons (that protocol's §2, set by the Coordinator from XP-20261029-dg-wall's position table):
- under `alternate`, indices 0 and 3 share the edge positions {0, 3}, and indices 1 and 2 share the middle positions {1, 2}; XP-20261029-dg-blocked's validator confirmed 144 blocks at each middle position for its middle arms;
- all four arms charge identical GAE pair by pair (§6.1 condition 8), so the decision statistic `wall.corrected_ratio` of `--a dgb --b dgbf` equals Σ wall(`dgbf`) / Σ wall(`dgb-aa`) over the 288 pairs, two arms at the same block positions;
- the edge arms `dgb` and `dgbf-edge` give a second, position-matched estimate at the block edges that decides nothing;
- `dgb-aa / dgb` (the decision file's A/A ratio) and `dgbf / dgbf-edge` are each oracle's middle-over-edge contrast, descriptive.

## 3. Workloads

Each size uses two fresh prime-order curves from `prime_search`, with seeds `202610296<b>3` and `202610296<b>4`:

| b | curves (`ecbench plan`) |
|--:|:--|
| 24 | `icv1-fp24-t217-e1fbffd2`, `icv1-fp24-tm567-4accd0e8` |
| 28 | `icv1-fp28-tm295-85243301`, `icv1-fp28-tm8273-f1778659` |
| 32 | `icv1-fp32-t89141-95879fe3`, `icv1-fp32-tm68303-f902d2f0` |

None of these curves appears anywhere on `origin/main` or on the branches of #2826, #2845, #2849 and #2855 (`git grep -l <slug>`, 0 files each, 2026-10-29). `ecbench plan` gives 1,344 executions per session.

Measurement, per size: 24 public targets per curve (seed `202610296<b>6`); 1 warm-up and 6 measured rounds, `alternate` order (seed `202610296<b>8`); L2 isolation, 1800 s timeout. That gives 1,344 executions per session and 288 matched pairs per arm pair.

**Local pilot (seen; descriptive only).** One run per size at 28 and 32 bits: curve seed 909090, target seed 909091, order seed 909092; one curve, 3 targets, 1 round; arms `dg`, `dgb`, `dgbf`; L0 on a shared 4-vCPU VM, `--cpus none --allow-busy`. It used a glibc release build of `3271a25d02` (ecbench sha256 `b00eea70fc528030007b3946c9f3e5a01614e9bcc07e997b3f4d447c49bc6733`), not the image's binary; P2 of `APPROVAL.md` repeats its count checks with the image's binary. It checked that the filter runs, that `verify --replay-all` reproduces 9/9 records per session, that counted `ops` `dgbf / dgb` is exactly 1.0000, and that `ecbench work` prices `table_filter_bit_sets` (+402,498,432 bitops per record at 32 bits, 384 per bit set). Its wall ratios `dgbf / dgb` were seen: 0.9765 at 28 bits (ns per GAE 141.10 → 137.79) and 0.8789 at 32 bits (183.51 → 161.29), 3 pairs each. They are descriptive at L0 and play no part here. Its output is kept at `backlog/XP-20261029-packed-filter/pilot/`.

## 4. Route

A new image is needed, because the filter is new code.

1. **Build.** `taskq/deploy/eks-karpenter/build-bundle`, taken from `3271a25d02` itself, run on a sparse checkout of `3271a25d02` (Cargo manifests, `src/`, `tools/factor-base/`, every target path in `Cargo.toml`; `git status --porcelain --untracked-files=no` empty, as the script checks). Flags as the script sets them: `cargo +1.99.1 build --locked --release --no-default-features --target x86_64-unknown-linux-musl --bin ecbench`, `RUSTFLAGS='-C target-cpu=x86-64'`, `CARGO_BUILD_JOBS=2`. Its `build-info.json` records ecbench sha256 `21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503`.
2. **Image.** The steps of `publish-image` at `3271a25d02`: the same three-file context and `Dockerfile.ecbench`, `docker build --platform linux/amd64`, pushed to `isolab/tasks` with tag `local-20261029-pf-3271a25d02dcef6da148293fb8f9e2149ce59858`; digest resolved from ECR: `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b`. This is the route of XP-20261029-srenew-deferred-guard `AMENDMENT-01.md`, used because the GitHub workflow's OIDC role does not trust `isogeny-labs/crypto` (open owner decision).
3. **Launch.** `taskq/deploy/eks-karpenter/remote/launch-remote` with an `isolab.eks.request/v1` request:
   - `job.build = {image_uri@digest, source_revision 3271a25d02dcef6da148293fb8f9e2149ce59858, ecbench_sha256}`, checked by `validate-request`;
   - `provenance.harness = "claude-code-local"`, c8a.2xlarge, 3,600 s deadline, L2;
   - each request embeds its spec byte for byte from this protocol's freeze commit (§9);
   - six jobs: `ic-dgbf-p<b>-20261029` (wall) and `ic-dgbf-rho-p<b>-20261029` (counted), one size at a time;
   - frozen `experiment-director` cards run unchanged by `executor-mechanical`, each recording the freeze-commit SHA, the spec sha256, the image digest and the ecbench sha256. A job is launched once; an ambiguous submission is checked for an existing build, job and S3 prefix before anything is resubmitted.

**Host loss** is handled as in XP-20261029-dg-wall §4: the job is rerun whole as `-r2` and the loss recorded. A job that wrote `job-exit.json` with a non-zero code is not rerun, except under the noise-rerun clause below.

**Noise rerun (fixed now, before any data).** XP-20261029-dg-blocked lost its 32-bit size to one wall-session execution graded L1 by foreign task slices (`research/ic_dg_blocked_20261029/VERDICT.md`). This clause allows one rerun of a wall session lost to host scheduling, and nothing else.

- **Trigger.** A wall session qualifies if, and only if, all of the following hold. The session is the first launch `ic-dgbf-p<b>-20261029`, or its host-loss rerun `ic-dgbf-p<b>-20261029-r2` when the first launch wrote no `job-exit.json`. The conditions are read from that session's `job-exit.json` and `l2-gate.json` alone, after the `SHA256SUMS` check:
  1. `job-exit.json` records `exit_code` 42 from a single pod.
  2. In `l2-gate.json`, `invalid_outcomes` is `[]` and `below_l2` is non-empty.
  3. Every `below_l2[]` entry has `isolation.level` equal to `"L1"`, so it carries no `L1:` blocker.
  4. Each string in each `below_l2[].isolation.blockers` takes one of two forms:
     - it begins with `L3: `; or
     - it is `L2: ` followed by a string that matches one of these anchored regular expressions. They are the grading-v3 strings of `src/cryptanalysis/ecbench/record.rs` at `3271a25d02`, and `µ` is U+00B5 as in that file:
       - `^[0-9]+ ticks of foreign work on the run CPU during the run$`
       - `^[0-9]+ ticks of hypervisor steal on the run CPU$`
       - `^[0-9]+ busy ticks on the run CPU.s idle siblings and the rest of the reservation$`
       - `^run-queue delay [0-9]+ µs is [0-9.]+% of the solve$`
       - `^[0-9]+ involuntary context switches (during a [0-9.]+ ms solve|over the process) \(at most [0-9.]+\)$`
       - `^[0-9]+ interrupts on the run CPU during the solve \(.*\)$`
       - `^[0-9]+ foreign task slices on the run CPU during the solve \([0-9]+ µs\)$`

  Any other `L2:` blocker fails the trigger. That includes `solve too short to admit wall time`, memory stall (`… of memory stall system-wide`), a missing observation (`no schedstat …`, `no memory PSI`, `no /proc/stat ticks …`), and `preflight thresholds looser than the defaults`.
- **Blinding.**
  - Until the rerun has written its `job-exit.json`, no agent opens the first session's `records.jsonl`, runs `ecbench compare`, `band` or `work` on it, or reads any wall field of it.
  - The director's card evaluates the trigger with one `jq -e` over `l2-gate.json` that implements conditions 2–4.
  - The card writes the command, its output, the sha256 of the two files and the verdict to `research/ic_packed_filter_20261029/P<b>/P<b>-RERUN.md`. That file is committed and pushed before the rerun launches.
- **Rerun.**
  - If the trigger holds, the wall session is rerun once, whole, as job `ic-dgbf-p<b>-20261029-nr`.
  - The rerun uses the same frozen spec, image, binary and request fields. Only the job name changes.
  - If the rerun loses its host, it is rerun under the host-loss rule as `ic-dgbf-p<b>-20261029-nr-r2`. That is not a second noise rerun.
  - The decision reads the rerun alone. The first session is kept and reported descriptively. It is never pooled.
- **Limits.**
  - There is at most one noise rerun per size.
  - Any non-zero exit of the rerun, 42 included, makes the size undecidable.
  - The counted session is never rerun under this clause.

## 5. Unit and statistic

**Wall.** The statistic is the `wall` block of

    ecbench compare --dir <wall session> --a dgb --b dgbf --weight 0 --json

run with the image's ecbench binary (sha256 `21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503`) at its default `--resamples` and `--seed`; the file's `resample_seed` is recorded. The block's fields:
- `wall.ns_per_gae_a` and `wall.ns_per_gae_b`: each arm's `Σ solve_wall / Σ total_gae` over the admitted pairs, where both runs of a pair are at L2 or L3;
- `wall.ratio_b_over_a` (`dgbf / dgb`), with `wall.ci95`;
- `wall.aa_arms` = `["dgb", "dgb-aa"]`, `wall.aa_selection` = `"includes arm A"`, and `wall.aa_ratio` (`dgb-aa / dgb`);
- `wall.corrected_ratio` = `ratio_b_over_a / aa_ratio`, with `wall.corrected_ci95` and `wall.outside_noise`.

The block reads charged `total_gae` and does not depend on `--weight`. The w = 0.1 and w = 1 files therefore carry the same block.

**What the corrected ratio is.** All four arms charge identical GAE on each pair (§6.1 condition 8), and every pair is admitted, so:
- `corrected_ratio` = Σ wall(`dgbf`) / Σ wall(`dgb-aa`), two arms at the same block positions (§2);
- `ratio_b_over_a` = Σ wall(`dgbf`) / Σ wall(`dgb`), middle over edge positions; it decides nothing.

`aa_ratio` carries the baseline's middle-over-edge contrast as well as the A/A noise; dividing by it takes the position effect out of the decision. The combined interval treats the two standard errors as independent, which is conservative because both share `dgb`. `total_wall_ratio`, `median_ratio` and the `legacy_*` fields are reported and decide nothing. Both arms carry unpriced native work, so each ns/GAE is an upper bound on cost per operation, for both arms alike.

**Descriptive wall comparisons** (wall session; each records its `aa_arms`, `aa_selection` and `self_aa`):
- `--a dgb-aa --b dgbf`: `ratio_b_over_a` with `ci95`, the decision's middle-position contrast as a direct paired interval; used for the identity check of §6.1 condition 9;
- `--a dgb --b dgbf-edge`: `ratio_b_over_a` with `ci95`, uncorrected: the **position control**, the filter's effect at the block edges;
- `--a dgbf-edge --b dgbf`: `self_aa` true; `ratio_b_over_a` with `ci95` is the candidate's middle-over-edge contrast;
- the decision file's `aa_ratio` with `aa_ci95`: the baseline's middle-over-edge contrast.

**Phase sums (descriptive).** Per arm, Σ `phases.<name>.wall_ns` over the 288 measured records for every phase, so the README can say where the time moved (oracle set-up, which now also builds the filter, against the target PDP phase, which carries the probes).

**Counted.** `S = total GAE / √r`, cold, whole pipeline, priced at w = 0, 0.1 and 1. Each pair below is compared at the three weights, and each session gets `ecbench band --ranks`:
- wall session: `dgb`/`dgbf`, `dgb`/`dgb-aa`, `dgbf-edge`/`dgbf`, `dgb`/`dgbf-edge`, `dgb-aa`/`dgbf`;
- counted session: `rho-neg`/`rho-neg-aa`, `rho-neg`/`dgb`, `rho-neg`/`dgbf`.

**bitop/v1.** `ecbench work` on both sessions, with the image's binary. For each arm it reports Σ `work.bitops` and the ratio `dgbf / dgb`, beside every wall figure. It decides nothing. Three limits of this figure are stated with it:
- the rise it shows is build-side only (`table_filter_bit_sets`, 6 `word_op` each);
- the probe-side filter test (one multiply-shift and one bit read per lookup) is not counted separately, because `bitop/v1` prices a lookup at a fixed cost whatever the table does;
- the filter build's pass over the empty slots (slots − entries word tests) is not counted.

**The filter's bit sets in the §5a band.** `table_filter_bit_sets` is a `bitop/v1` counter (`work_native`), not a `cost.unpriced` counter. So `ecbench band` and `compare --weight` do not price it, and condition 8's `ops` ratio of exactly 1 at w = 0.1 and 1 only says that everything else is equal. It is native work that the filter adds, and AGENTS.md §5a prices it on the arm that does it.

The analysis therefore also reports, for each size and at w = 0, 0.1 and 1:
- `S'_w = (priced_gae_w + w · table_filter_bit_sets) / √r` per record, where `priced_gae_w` is `band.rs`'s priced GAE at w (the figure `compare --weight w` uses) and `table_filter_bit_sets` is 0 on every arm without the filter;
- the ratios Σ `S'_w(B)` / Σ `S'_w(A)` over the matched pairs, for `dgb`/`dgbf` in the wall session, and for `rho-neg`/`dgb` and `rho-neg`/`dgbf` in the counted session.

The analysis computes these from `records.jsonl`, and the validator recomputes them with its own code.

Wherever the README, `benchmarks/` or the leaderboard gives `dgbf / rho-neg` or a counted `dgbf / dgb`, it uses these figures. It also says three things there:
- `dgbf`'s counted cost exceeds `dgb`'s at w > 0, by that ratio;
- the band leaves `table_insert_attempts` unpriced on every IC arm. That count is identical between `dgb` and `dgbf`, and the gap was there before this study;
- these figures decide nothing.

## 6. Decision rule (per size, fixed now)

### 6.1 Instrument

**Wall session.** All of the following must hold. Each is read from the job's own files after the `SHA256SUMS` check. (Under the noise-rerun clause of §4, "the wall session" is the rerun.)

1. **Exit:** `job-exit.json` records `exit_code` 0 from a single pod. Any other code makes the size undecidable, including 42 (an execution below L2) and 43 (native A/A admission refused), except that exit 42 can trigger the one noise rerun of §4. The v3 exit-42 clause does not apply to the wall session.
2. **Audit:** `audit.json` is `ok`, the session is `complete`, there are 0 problems, and every measured replay is `identical`.
3. **Counted A/A:** `ops.ratio_b_over_a` is exactly 1.000 at w = 0, 0.1 and 1 for both `--a dgb --b dgb-aa` and `--a dgbf-edge --b dgbf`.
4. **Spec and build:** `input-spec.json` is JSON-equal to the frozen `spec-P<b>.json`, and `build-info.json` names `3271a25d02dcef6da148293fb8f9e2149ce59858` and ecbench sha256 `21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503`.
5. **Gate:** `l2-gate.json` has `accepted` true, every execution is graded L2 or L3, and `invalid_outcomes` is empty.
6. **Native A/A admission:** `control-comparison.json` is present. It is `run-ecbench`'s `--a dgbf-edge --b dgb-aa` comparison. It must show:
   - `wall.status` `"admitted"`;
   - `wall.pairs` 288 and `wall.pairs_below_level` 0;
   - `wall.aa_pairs` 288 and a non-null `wall.aa_ci95`;
   - `wall.self_aa` false;
   - `wall.aa_arms` equal, as a set, to {`dgbf`, `dgbf-edge`}, and `wall.aa_selection` `"includes arm A"`.
7. **Decision block:** the w = 0 `dgb`/`dgbf` file must show:
   - `wall.status` `"admitted"`;
   - `wall.pairs` 288 and `wall.pairs_below_level` 0;
   - `wall.self_aa` false;
   - `wall.aa_arms` `["dgb", "dgb-aa"]` and `wall.aa_selection` `"includes arm A"`;
   - `wall.aa_pairs` 288;
   - a non-null `wall.corrected_ci95`.

   The w = 0.1 and w = 1 files carry an identical `wall` block.
8. **Identical charged work; the filter's only trace.** AGENTS.md §6 requires a changed oracle to be cross-checked against the one it replaces on every input of a full run. All of the following must hold:
   - In the `dgb`/`dgbf` files, `ops.ratio_b_over_a` is exactly 1 at w = 0, 0.1 and 1.
   - The analysis pairs records from `records.jsonl`. Each `dgbf` and `dgbf-edge` record, warm-ups included, is matched with the `dgb` record of the same curve, target and round. Each match must agree on:
     - the recovered logarithm;
     - `total_gae`;
     - every unpriced-operation count that `ecbench band` prices;
     - for every phase, matched by `name`: `adds`, `doubles`, `scalar_mults`, `gae`, and every key and value of `native`;
     - for every phase, every key and value of `work_native` **except** `table_filter_bit_sets`. That counter must meet three requirements:
       - it is absent on every `dgb` and `dgb-aa` record;
       - it is present on every `dgbf` and `dgbf-edge` record, in the `oracle_setup` phase only;
       - its value is the same on all of a curve's `dgbf` and `dgbf-edge` records. It is the packed table's entry count, which the curve and N fix.
   - From `ecbench work`, for each match: `dgbf.work.bitops − dgb.work.bitops` = 384 × `table_filter_bit_sets` exactly (6 word ops of 64 bits per bit set), and that difference lies entirely in the oracle set-up phase.

   Any difference makes the size undecidable.
9. **Identity:** in the w = 0 `--a dgb-aa --b dgbf` file, `wall.pairs` is 288, and `wall.ratio_b_over_a` equals the decision file's `wall.corrected_ratio` to within a relative 1e-9.
10. **Packed layout (XP-20261029-dg-blocked `cards/RULING-P2.md`, A1).** Unfiltered `mitm-prime-blocked:m=2` silently builds its small-maps layout when p ≥ 2³² or the base has more than 2¹⁶ signed points. `filter=1` refuses that case instead. For every `dgb`, `dgb-aa`, `dgbf` and `dgbf-edge` record, warm-ups included, both of the following hold:
    - the curve's p, read from the record's ICV1 curve string, is ≤ 4,294,967,295;
    - the factor base's signed-point count is ≤ 65,536. It is read from the record's `signed_points` field. If records lack that field, it is twice the record's `abscissae` field. If records carry neither, it is read from `ecbench plan --doc` for the frozen spec, and the analysis states which source it used.

    No wall field is used. A failure makes the size undecidable and is never waived after data.

**Counted session.** It is handled as in XP-20261029-dg-wall §6.1 "Counted session":
- conditions 1–6 of XP-20261029-srenew-deferred-guard-v3 §6.1, with `rho-neg-aa / rho-neg` as the A/A and condition 4 read as "every `ic.pipeline` execution";
- that protocol's exit-42 clause verbatim;
- the exit-43 clause: exit 43 satisfies the route if, and only if, `l2-gate.json` is accepted, conditions 1–6 hold, and `control-comparison.json` has `wall.self_aa` true.

In addition, the counted session's `dgb` and `dgbf` records must equal the wall session's records of the same arm, curve, target and round, in the fields listed in wall condition 8 (`table_filter_bit_sets` included), and condition 10 holds for them.

A counted-session failure does not change the wall decision. It blocks the validation gate for that size, and it is reported.

### 6.2 Decision

The filter is a **wall-clock improvement at N_inc** at size b if, and only if, both of the following hold:
- the wall session's §6.1 conditions 1–10 hold;
- the upper bound of `wall.corrected_ci95` for `dgbf / dgb` in the w = 0 file is below 1.

| result at b (wall-session §6.1 holds) | reading |
|:--|:--|
| corrected upper bound < 1 | improvement: lower wall cost per operation, identical charged GAE, `bitop/v1` higher by the reported ratio |
| corrected interval contains 1 | null at this sample size |
| corrected lower bound > 1 | regression |

If wall-session §6.1 fails, the size is undecidable.

**No baseline changes at any size.** Counted `S` is identical (condition 8). An improvement is a component speed-up of the oracle (AGENTS.md §3), not a counted improvement, and the headline stays counted `S` (AGENTS.md §6). Every size reports `dgb/rho-neg` and `dgbf/rho-neg` from the counted session at the three weights, as the distance remaining.

**Composition with XP-20261029-dg-blocked (descriptive).** Where both studies decided an improvement at the same size, the README may give the product of the two corrected ratios as an indicative `dgbf / dg`. It is labelled as a product across sessions on different curves and from different binaries (`be2969ce` and `3271a25d02`; §8, "The baseline binary"). It decides nothing and enters no table.

**Position control (fixed now; decides nothing).** At each size the README reports, from the wall session:
- the edge-position estimate `dgbf-edge / dgb` (`ratio_b_over_a` with `ci95`, uncorrected);
- the decision's corrected ratio, measured at the middle positions;
- the two middle-over-edge contrasts: `aa_ratio` of the decision file, and `dgbf / dgbf-edge`.

If the edge-position interval does not overlap the decision's `corrected_ci95`, the decision is still read by the table above, and the README states that the filter's effect depends on block position, by at least the gap between the two intervals, with the positions at which each interval was measured.

**Scope:** c8a.2xlarge (AMD), image `32144a29…817b`, ecbench `21701be6…4503` built from `3271a25d02`; the six curves of §3; N_inc, `targets = stride-renew-deferred-guard`, `incremental-gauss`. Not measured: other host classes, other factor bases, other target rules, other filter densities, the small-maps layout.

## 7. Predictions (before any study data; the pilot of §3 was seen)

Model: the filter changes only the probe of a missing key and the set-up.
- **Probe.** At N_inc almost every probe misses. The filter's load k = entries / bits is about 0.100, 0.079 and 0.062 at 24, 28 and 32 bits. So a clear bit, a fraction e^(−k), ends 90–94% of misses: one bit read replaces the region scan. The saving per probe is large only when the slot array does not sit in the core's L2 (1 MiB per core on c8a):
  - at 24 bits, the packed table (about 26k entries in one region of 2¹⁶ slots, 512 KiB) and its filter (2¹⁸ bits, 32 KiB) both sit in L2, and little changes;
  - at 28 bits, the table (about 165k entries in 16 regions of 2¹⁴ slots, 2 MiB) spills to L3, while the filter (2²¹ bits, 256 KiB) stays in L2;
  - at 32 bits, the table (about 1.05M entries in 64 regions of 2¹⁵ slots, 16 MiB) is more than the slice of L3 one core can rely on, and the filter (2²⁴ bits, 2 MiB) is L3-resident.

  The figures were corrected at the coordinator's review. The draft's 1.3 MiB and 8 MiB counted entries, not slots; each region is sized at the next power of two above 4/3 of its entries (`PackedPairTable::build`). The ranges below were written before the correction and are not changed.
- **Set-up.** One extra pass over the slots with a bit set per entry, against the packed build's batched inversions: below 2% of `dgb`'s solve wall at every size.
- **Shares** (from XP-20261029-dg-blocked, seen): the target-PDP phase is the larger part of `dgb`'s solve wall at N_inc.

| b | predicted corrected `dgbf / dgb` ns/GAE |
|--:|:--|
| 24 | 0.97–1.01 |
| 28 | 0.92–0.99 |
| 32 | 0.82–0.94 |

Model risk:
- the probe may be bound by the hash and the key arithmetic rather than by memory, in which case the gain is smaller at every size;
- a set bit costs one more dependent load than before (filter, then slot), so if hits and false positives are more frequent than modelled the ratio rises;
- at 24 bits a small regression (the extra branch and set-up pass with nothing saved) is the first failure the model allows.
- the filter test is a branch that is taken on a set bit: about 6–10% of misses, plus every hit. Mispredictions on those probes add a few cycles per probe on average, which the model above omits;
- the expected outcome at 24 bits is **undecidable**. XP-20261029-dg-blocked's P24 wall session had 2 of its 1,344 executions below the 1 ms minimum solve (`solve too short to admit wall time`, `VERDICT.md`). This study uses the same N, target rule and design, and its candidate is predicted to be no slower. The noise rerun of §4 does not cover that blocker. P24 is run for its counted session and for descriptive wall figures; a P24 undecidable result is the expected one and is not a defect of the run.

## 8. Disclosure

- **Seen before drafting:**
  - XP-20261029-dg-blocked's results: its verdict, README, phase sums and validator report (the source of §1's figures and §7's shares).
  - XP-20261029-dg-wall's results, as listed in XP-20261029-dg-blocked §8.
  - `research/prime_ic_program_20261029/hashtable_prefetch/` (design only, no measurement).
  - The local pilot's L0 wall ratios (§3). They were seen before §7's ranges were written, and the 28- and 32-bit ranges contain them.
  - The pilot also ran the arm `dg` (`mitm:negation_folded=1`). The author states here whether the pilot's `dgb / dg` and `dgbf / dg` wall ratios were computed or seen, and gives their values if so: not computed and not seen. The pilot's only wall comparison was `ecbench compare` with `--a dgb --b dgbf` (the ratios quoted in §3); no comparison involving `dg` was run or read, and no `dg` wall field was opened (author, from this session's command log, 2026-10-29).
  - XP-20261029-dg-blocked's `VERDICT.md`, including the reasons P24 and P32 were undecidable: sub-millisecond solves at 24 bits, and one execution with 3 foreign task slices at 32 bits. They are the basis of §4's noise-rerun clause and of §7's 24-bit risk.
  - PR #2844's table-floor micro-benchmark is no longer sealed (XP-20261029-dg-blocked's verdict commit is pushed); it was read descriptively, and it decides nothing.
- **Earlier wall data of this oracle pair.** `filter=1` exists only on branch `claude/ic-packed-filter-20261029` from commit `3271a25d02`; the only sessions that ran it are the pilot of §3. On `origin/main`, `git grep -l 'mitm-prime-blocked:m=2,filter=1' -- research benchmarks backlog docs/ic` returns 0 files, and `git grep -l 'filter=1'` over the same paths returns 1 file, an unrelated NFS script (`research/nist_skip_criterion_20261025/factor_followup_20261026/run_nfs_continue.sh`, `minimum_relations_first_filter=1000000`) (author, 2026-10-29, before the freeze commit).
  - Before the freeze commit, the author runs `git grep -l 'filter=1'` on every open PR head (fetched for the purpose) and lists here every file that also contains `mitm-prime-blocked`: none. On 2026-10-29 the author fetched the heads of all 112 open PRs (trees only) and `origin/main`. Two checks: (a) at every head, `src/cryptanalysis/ic_framework/plugins.rs` contains no `"filter"` parameter, so no binary built from any open PR head accepts `filter=1` (the parser refuses unknown keys); (b) of the files each PR changes against its merge base, every spec, `PROTOCOL` and request file under `research/`, `benchmarks/`, `backlog/`, `docs/ic/` and `taskq/` (1,490 distinct blobs) was read in full, and 0 contain both `filter=1` and `mitm-prime-blocked`. The same scan finds `mitm-prime-blocked` in XP-20261029-dg-blocked's specs at #2855's head (positive control). The scan did not read the other changed files of each PR (records, reports, code outside the parser), which check (a) makes unable to hold a `filter=1` session. Any session found there that runs `filter=1` at L2 is sealed for every agent on this study (author, director, executor and validator) until this study's verdict commit is pushed. Afterwards it is reported descriptively and never pooled.
- **Curves** are fresh (§3). **N_inc** is fixed by the earlier studies, not chosen here.
- **The coordinator's review.** `APPROVAL.md` lists what the coordinator read. It opened no pilot records, no session file and no wall field. It read the prior study's `VERDICT.md`, which the author had already seen.
- **The pilot's files.** In `pilot/run28/` and `pilot/run32/`, `records.jsonl` is git-ignored and kept in the artifact store. Its sha256 is in each directory's `.artifacts.json` (`run32`: `3716d9f4…0be2`). In each pilot spec, the seeds are 909090 for the curve, 909091 for the targets and 909092 for the order.
- **The baseline binary.** This study's `dgb` runs in the `3271a25d02` binary, not in XP-20261029-dg-blocked's `be2969ce` binary. In this binary, the unfiltered lookup also tests whether a filter is present (`PackedPairTable::get`). Both arms of this study run the same binary, so that test is common to both and does not confound the decision.

## 9. Freezing

This file, `APPROVAL.md` and the six specs are frozen by the commit that adds them. Before the first job launches that commit is pushed, and its SHA and the six spec sha256 are recorded on every launch card. Each request embeds its spec byte for byte. No amendment is allowed after the first launch of any of the six jobs; the noise rerun of §4 is part of this protocol, not an amendment.

## 10. Outputs

`research/ic_packed_filter_20261029/` holds, per size, `P<b>/wall/` and `P<b>/counted/` (and `P<b>/wall-nr/` after a noise rerun, with `P<b>/P<b>-RERUN.md`). Each holds:
- the launch receipts and the S3 copy with hashes;
- the `compare` JSON for every pair of §5, at the three weights;
- the band table, the phase sums and the `ecbench work` totals.

It also holds a README with a figure and a PDF (research-visuals). The README's position table gives, per size, each arm's block positions (from `records.jsonl`) and the position-control figures of §6.2.

**Independent validation.** The validator, using its own code on the wall session's `records.jsonl`:
- recomputes `ns per GAE`, `ratio_b_over_a`, `aa_ratio`, `corrected_ratio` and `corrected_ci95`, and the identity of condition 9;
- checks condition 8 record by record, warm-ups included, including the `table_filter_bit_sets` and bitop clauses;
- checks that the counted session's `dgb` and `dgbf` records equal the wall session's;
- tabulates each arm's block positions and its predecessor within the block, and confirms that `dgbf` and `dgb-aa` have the same position distribution (144 blocks at position 1 and 144 at position 2);
- reports, descriptively, the per-curve and target-clustered intervals and the position control;
- recomputes the §5 "filter's bit sets in the §5a band" ratios;
- checks condition 10 for every IC record of both sessions;
- rechecks §6.1 for both sessions. If a noise rerun was used, it re-evaluates the §4 trigger with its own code, from the first session's `job-exit.json` and `l2-gate.json`, and confirms that `P<b>-RERUN.md` was committed before the rerun's launch receipt.

**Before any leaderboard or `benchmarks/` change**, an improvement passes the validation gate. `VALIDATION.json` must name an independent replay of both sessions, the validator report, and the matched rho from the counted session with its §5a band.

**Placement.**
- At each size with an improvement, the result goes in a component section of `scripts/build_ic_leaderboard.py` outside tables A–C, next to XP-20261029-dg-blocked's `dg_blocked_component`, labelled "whole-solve wall per charged GAE at N_inc, filtered against unfiltered packed table", with the `bitop/v1` ratio beside it.
- It also goes in `benchmarks/<YYYYMMDD>-<slug>/`, with a row in `benchmarks/README.md`.
- A null or a regression is reported in the README only.

Wall-time figures enter only for the IC arms of the wall session. Rho wall time, and every wall field of the counted session, are never used.
