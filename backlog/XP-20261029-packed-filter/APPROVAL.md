# Coordinator decision — XP-20261029-packed-filter

## Status

**Approved with edits** (2026-10-29, `coordinator` role, before any execution of these specs). The approval requires edits E1–E10 and the pre-launch checks P1–P6 below.

- **Protocol reviewed.** The protocol is `backlog/XP-20261029-packed-filter/PROTOCOL.md`, title line `# Prime-field IC: wall time per operation of a membership filter in front of the packed pair table — protocol`, as drafted. I had no shell, so I could not compute its sha256. The binding text is the file as it stands in **the freeze commit that adds this file**, with E1–E10 applied. Launch cards record that commit's SHA and the sha256 of `PROTOCOL.md`, `APPROVAL.md` and the six specs.
- **Edits.** Apply E1–E10 verbatim in the commit that adds this file. That commit is the freeze (§9).
- **Pre-launch checks.** P1–P6 fail closed. If any one fails, nothing launches and the protocol comes back to the coordinator. A failed check is not an amendment.
- **Reviewed:**
  - `PROTOCOL.md`, the six specs and the six `plan-P*.txt` of this directory;
  - from `pilot/`, only `spec28.json`, `run32/.artifacts.json` and `run32/.gitignore`. I opened no `records.jsonl`, no `session.json` and no wall field;
  - the candidate code in the `/dev/shm/src-pf` working tree:
    - `src/cryptanalysis/ic_boundary.rs` 5719–6098 (`PackedPairTable` with its filter, `PairTable::build_negation_folded_prime_blocked_with`) and the unit tests at 12134–12200;
    - `src/cryptanalysis/ic_framework/plugins.rs` 2193–2299 (parsing of `filter`);
    - `src/cryptanalysis/ecbench/work.rs` 387–393;
    - `docs/units/BITOPS.md` §"2026-10-29 (packed pair-table filter)" and the `word_op` row;
    - `src/cryptanalysis/ecbench/band.rs` 35–152 and `methods.rs` 4617–4716 (how `cost.unpriced` is built for `ic.pipeline`);
    - `src/cryptanalysis/ecbench/record.rs` 1280–1480 (the grading-v3 L2 blockers);
    - `taskq/deploy/eks-karpenter/run-ecbench` 54–67;
  - from the prior study (`/dev/shm/wt-chain/research/ic_dg_blocked_20261029/`): `validation/frozen/PROTOCOL.md`, `validation/frozen/APPROVAL.md`, `cards/RULING-P2.md` and `VERDICT.md`;
  - `/dev/shm/wt-chain/AGENTS.md` §3, §5a and §6 (`/dev/shm/src-pf` has no `AGENTS.md`).
- **Limits of this review.**
  - I could not run git. I have not read the diff of `3271a25d02` as a diff, and I have not confirmed that the working tree equals `3271a25d02`. P1 covers both.
  - I have not confirmed the image digest, the ecbench sha256 or `build-info.json`. P2 covers these.
- **Verdict on the design.** The design is sound:
  - it is the dg-blocked layout with the oracles shifted by one, a layout that has already been validated;
  - the arms differ only by `,filter=1`;
  - the decision rule is fixed and three-way.

  The edits fix six things:
  1. the noise-rerun trigger is not mechanical. It does not exclude wrong-answer exits, and its job name collides with the host-loss rerun (E1, E9);
  2. condition 8 narrows the prior study's per-phase identity check to `work_native` only (E2);
  3. the packed-layout condition A1 of the prior study is missing for the unfiltered arms (E3);
  4. the filter's bit sets are native work that the §5a band does not price (E4);
  5. the cache-size figures in §7 are wrong (E5);
  6. disclosure gaps (E6–E8, E10).

  None of the edits changes an arm, a seed, a spec, a budget, the decision threshold or a predicted range.

## Edits

### E1 — §4: replace the paragraph that begins `**Noise rerun (fixed now, before any data).**` in full

```markdown
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
```

### E2 — §6.1 condition 8: replace the list that follows `Each match must agree on:` in full

```markdown
     - the recovered logarithm;
     - `total_gae`;
     - every unpriced-operation count that `ecbench band` prices;
     - for every phase, matched by `name`: `adds`, `doubles`, `scalar_mults`, `gae`, and every key and value of `native`;
     - for every phase, every key and value of `work_native` **except** `table_filter_bit_sets`. That counter must meet three requirements:
       - it is absent on every `dgb` and `dgb-aa` record;
       - it is present on every `dgbf` and `dgbf-edge` record, in the `oracle_setup` phase only;
       - its value is the same on all of a curve's `dgbf` and `dgbf-edge` records. It is the packed table's entry count, which the curve and N fix.
```

The rest of condition 8 stands as drafted. That covers the `ops` bullet, the `ecbench work` bullet with `384 × table_filter_bit_sets` in the `oracle_setup` phase, and "Any difference makes the size undecidable". The `ecbench work` figures are computed with the image's binary (sha256 `21701be6…4503`).

### E3 — §6.1: add condition 10 after condition 9, and renumber references

Add:

```markdown
10. **Packed layout (XP-20261029-dg-blocked `cards/RULING-P2.md`, A1).** Unfiltered `mitm-prime-blocked:m=2` silently builds its small-maps layout when p ≥ 2³² or the base has more than 2¹⁶ signed points. `filter=1` refuses that case instead. For every `dgb`, `dgb-aa`, `dgbf` and `dgbf-edge` record, warm-ups included, both of the following hold:
    - the curve's p, read from the record's ICV1 curve string, is ≤ 4,294,967,295;
    - the factor base's signed-point count is ≤ 65,536. It is read from the record's `signed_points` field. If records lack that field, it is twice the record's `abscissae` field. If records carry neither, it is read from `ecbench plan --doc` for the frozen spec, and the analysis states which source it used.

    No wall field is used. A failure makes the size undecidable and is never waived after data.
```

Then make these replacements:
- in §6.2, replace `the wall session's §6.1 conditions 1–9 hold` with `the wall session's §6.1 conditions 1–10 hold`;
- in §6.1 "Counted session", after `in the fields listed in wall condition 8 (`table_filter_bit_sets` included)`, add `, and condition 10 holds for them`.

### E4 — §5: replace the paragraph `**bitop/v1.** …` in full

```markdown
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
```

### E5 — §7: replace the `**Probe.**` bullet in full; the predicted ranges are not changed

```markdown
- **Probe.** At N_inc almost every probe misses. The filter's load k = entries / bits is about 0.100, 0.079 and 0.062 at 24, 28 and 32 bits. So a clear bit, a fraction e^(−k), ends 90–94% of misses: one bit read replaces the region scan. The saving per probe is large only when the slot array does not sit in the core's L2 (1 MiB per core on c8a):
  - at 24 bits, the packed table (about 26k entries in one region of 2¹⁶ slots, 512 KiB) and its filter (2¹⁸ bits, 32 KiB) both sit in L2, and little changes;
  - at 28 bits, the table (about 165k entries in 16 regions of 2¹⁴ slots, 2 MiB) spills to L3, while the filter (2²¹ bits, 256 KiB) stays in L2;
  - at 32 bits, the table (about 1.05M entries in 64 regions of 2¹⁵ slots, 16 MiB) is more than the slice of L3 one core can rely on, and the filter (2²⁴ bits, 2 MiB) is L3-resident.

  The figures were corrected at the coordinator's review. The draft's 1.3 MiB and 8 MiB counted entries, not slots; each region is sized at the next power of two above 4/3 of its entries (`PackedPairTable::build`). The ranges below were written before the correction and are not changed.
```

### E6 — §7: add two model-risk bullets after the last one

```markdown
- the filter test is a branch that is taken on a set bit: about 6–10% of misses, plus every hit. Mispredictions on those probes add a few cycles per probe on average, which the model above omits;
- the expected outcome at 24 bits is **undecidable**. XP-20261029-dg-blocked's P24 wall session had 2 of its 1,344 executions below the 1 ms minimum solve (`solve too short to admit wall time`, `VERDICT.md`). This study uses the same N, target rule and design, and its candidate is predicted to be no slower. The noise rerun of §4 does not cover that blocker. P24 is run for its counted session and for descriptive wall figures; a P24 undecidable result is the expected one and is not a defect of the run.
```

### E7 — §8: add these bullets to "Seen before drafting" and "Earlier wall data"; replace the last bullet

Under **Seen before drafting**, after the pilot bullet, add:

```markdown
  - The pilot also ran the arm `dg` (`mitm:negation_folded=1`). The author states here whether the pilot's `dgb / dg` and `dgbf / dg` wall ratios were computed or seen, and gives their values if so: `<filled by the author before the freeze commit>`.
  - XP-20261029-dg-blocked's `VERDICT.md`, including the reasons P24 and P32 were undecidable: sub-millisecond solves at 24 bits, and one execution with 3 foreign task slices at 32 bits. They are the basis of §4's noise-rerun clause and of §7's 24-bit risk.
```

Under **Earlier wall data of this oracle pair**, add:

```markdown
  - Before the freeze commit, the author runs `git grep -l 'filter=1'` on every open PR head (fetched for the purpose) and lists here every file that also contains `mitm-prime-blocked`: `<filled by the author>`. Any session found there that runs `filter=1` at L2 is sealed for every agent on this study (author, director, executor and validator) until this study's verdict commit is pushed. Afterwards it is reported descriptively and never pooled.
```

Replace the bullet `- **The coordinator's review** records what the coordinator read in `APPROVAL.md`.` with:

```markdown
- **The coordinator's review.** `APPROVAL.md` lists what the coordinator read. It opened no pilot records, no session file and no wall field. It read the prior study's `VERDICT.md`, which the author had already seen.
- **The pilot's files.** In `pilot/run28/` and `pilot/run32/`, `records.jsonl` is git-ignored and kept in the artifact store. Its sha256 is in each directory's `.artifacts.json` (`run32`: `3716d9f4…0be2`). In each pilot spec, the seeds are 909090 for the curve, 909091 for the targets and 909092 for the order.
- **The baseline binary.** This study's `dgb` runs in the `3271a25d02` binary, not in XP-20261029-dg-blocked's `be2969ce` binary. In this binary, the unfiltered lookup also tests whether a filter is present (`PackedPairTable::get`). Both arms of this study run the same binary, so that test is common to both and does not confound the decision.
```

### E8 — §6.2: in the paragraph `**Composition with XP-20261029-dg-blocked (descriptive).**`, replace

`It is labelled as a product across sessions on different curves; it decides nothing and enters no table.`

with

`It is labelled as a product across sessions on different curves and from different binaries (`be2969ce` and `3271a25d02`; §8, "The baseline binary"). It decides nothing and enters no table.`

### E9 — §10: rerun directory and validator scope

- Replace `(and `P<b>/wall-r2/` after a noise rerun)` with `(and `P<b>/wall-nr/` after a noise rerun, with `P<b>/P<b>-RERUN.md`)`.
- In **Independent validation**, replace `- rechecks §6.1 for both sessions, and the noise-rerun trigger if one was used.` with:

```markdown
- recomputes the §5 "filter's bit sets in the §5a band" ratios;
- checks condition 10 for every IC record of both sessions;
- rechecks §6.1 for both sessions. If a noise rerun was used, it re-evaluates the §4 trigger with its own code, from the first session's `job-exit.json` and `l2-gate.json`, and confirms that `P<b>-RERUN.md` was committed before the rerun's launch receipt.
```

### E10 — §3: replace the first sentence of the pilot paragraph

Old:

`**Local pilot (seen; descriptive only).** One run per size at 28 and 32 bits (pilot seeds 909090/909091/909092, one curve, 3 targets, arms `dg`, `dgb`, `dgbf`, L0 on a shared 4-vCPU VM, `--cpus none --allow-busy`), with a glibc release build of `3271a25d02` (ecbench sha256 `b00eea70fc528030007b3946c9f3e5a01614e9bcc07e997b3f4d447c49bc6733`, not the image's binary).`

New:

`**Local pilot (seen; descriptive only).** One run per size at 28 and 32 bits: curve seed 909090, target seed 909091, order seed 909092; one curve, 3 targets, 1 round; arms `dg`, `dgb`, `dgbf`; L0 on a shared 4-vCPU VM, `--cpus none --allow-busy`. It used a glibc release build of `3271a25d02` (ecbench sha256 `b00eea70fc528030007b3946c9f3e5a01614e9bcc07e997b3f4d447c49bc6733`), not the image's binary; P2 of `APPROVAL.md` repeats its count checks with the image's binary.`

## Pre-launch checks

These checks are read-only. They go on the director's card and all run before the first launch. Each one fails closed.

- **P1 — the candidate commit.**
  - `git show --stat 3271a25d02` and `git show 3271a25d02` must show that every change under `src/` belongs to the filter. That means:
    - the `PackedPairTable` filter fields, their build, and the guard in `get`;
    - `PairTable::filter_bit_sets` and its `work_counters` entry;
    - `build_negation_folded_prime_blocked_with` and its refusal;
    - parsing of `filter` in `plugins.rs`;
    - the `table_filter_bit_sets` rule in `work.rs`;
    - tests.

    Beyond `src/`, only `docs/units/BITOPS.md` may change.
  - On the unfiltered path, the only changes may be the `if !self.filter.is_empty()` guard in `PackedPairTable::get` and the `filtered = false` argument.
  - The working tree used for this review must equal `3271a25d02` in `src/cryptanalysis/ic_boundary.rs`, `ic_framework/plugins.rs` and `ecbench/work.rs` (`git diff --quiet 3271a25d02 -- <paths>`).
  - At `3271a25d02`, `src/cryptanalysis/ecbench/record.rs` must carry the grading-v3 format strings behind E1's seven regular expressions verbatim. If one differs, the trigger list comes back to the coordinator before any launch.
  - At `3271a25d02`, `taskq/deploy/eks-karpenter/run-ecbench` must match lines 54–67 as read in this review. Specifically:
    - it writes `l2-gate.json` with `invalid_outcomes` and `below_l2[] = {seq, arm, isolation}`;
    - it exits 42 before it writes `control-comparison.json`;
    - it takes the first `reference` and the first `control` arm in spec order.
- **P2 — image and binary.**
  - Pull the image by its digest `…@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b`.
  - Its `ecbench` must have sha256 `21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503`.
  - The bundle's `build-info.json` must name `3271a25d02dcef6da148293fb8f9e2149ce59858`.
  - `validate-request` must accept all six requests.
  - Run `pilot/spec32.json` once with the image's binary at L0. Read only `outcome.status`, `outcome.matches_target`, the `ops` block, `phases[oracle_setup].work_native.table_filter_bit_sets`, and the `ecbench work` totals; read no wall field. Every outcome must be verified. Counted `ops` `dgbf / dgb` must be exactly 1. The `dgbf − dgb` bitops difference must be exactly 384 × `table_filter_bit_sets`.
- **P3 — the specs.**
  - `ecbench plan` with the image's binary must exit 0 on all six specs. It must print the §3 slugs, the same in the wall and counted spec of each size. It must list the arms in spec order with their roles.
  - `jq -S` must show, for each wall spec:
    - the arms are `dgb`, `dgb-aa`, `dgbf`, `dgbf-edge`, with roles baseline, control, candidate and reference;
    - `dgb-aa`'s params equal `dgb`'s, and `dgbf-edge`'s equal `dgbf`'s;
    - `dgbf`'s params differ from `dgb`'s only in `oracle`, and only by the suffix `,filter=1`;
    - `dgb`'s params equal those of the `dgb` arm of `backlog/XP-20261029-dg-blocked/spec-P<b>.json`.
  - For each counted spec:
    - its `dgb` and `dgbf` params equal the wall spec's;
    - its `rho-neg` and `rho-neg-aa` arms equal those of `backlog/XP-20261029-dg-blocked/spec-P<b>-rho.json`;
    - its `workloads` and `measurement` objects equal the wall spec's.
- **P4 — fresh curves and seeds.**
  - Run `git grep -lF` for each of the six §3 slugs and each of the twelve seeds `202610296<b>{3,4,6,8}`. Run it on `origin/main` and on every open PR head, all fetched for this check.
  - Every hit must lie under `backlog/XP-20261029-packed-filter/` or on this branch. Any other hit comes back to the coordinator.
- **P5 — checkpoint and scope.**
  - `.resume/RESUME.md` is filled and pushed (`scripts/checkpoint.sh init`).
  - A Conductor task holds the `backlog/XP-20261029-packed-filter/` and `research/ic_packed_filter_20261029/` scopes.
- **P6 — edits applied.**
  - `grep -n 'wall-r2'` on `PROTOCOL.md` returns nothing.
  - `grep -c 'conditions 1–10'` on `PROTOCOL.md` returns at least 1.
  - E7's two angle-bracket fields are filled.
  - The six specs are byte-identical to the drafts reviewed here: E1–E10 do not touch them.

## Reasons

### 1. Arms isolate the filter

**What holds:**
- In all six specs, `dgbf`'s params differ from `dgb`'s only by `,filter=1` on the `oracle` string. Curves, targets, seeds, `max_trials`, N_inc and measurement are equal. The plans give `dgb` and `dgbf` distinct method ids at each size; for example, at 32 bits they are `ECM1h1ece5a06377d` and `ECM1h201ac28dd4c3`. `dgb-aa` and `dgbf-edge` share their twin's id.
- Both arms run one binary. `filter=1` changes only two things:
  - after the slots are filled, one pass sets one bit per stored entry;
  - in `get`, a clear bit returns `None` before the region scan.

  Keys, witnesses and group operations come from the same `PackedPairTable::build` (`ic_boundary.rs` 5762–5837). The unit test at 12134 asserts equal entries, inserts, lookups, group and field operations, and every answer.
- The arm order is the prior study's validated layout. The candidate and the A/A twin both sit at the middle positions {1, 2}, and the edge pair gives the position control.
- The A/A selection follows the same `choose_aa` order as before (arm A's twin first):
  - the decision file `--a dgb --b dgbf` takes `["dgb", "dgb-aa"]`;
  - the admission compare `--a dgbf-edge --b dgb-aa` takes {`dgbf`, `dgbf-edge`}, with `self_aa` false.

**What is missing:** the guard against the unfiltered arm's silent layout fallback. The prior study needed condition A1 for this (`RULING-P2.md`). `filter=1` refuses the small-maps layout, but `dgb` does not. On these curves the packed layout holds: p < 2³², and at most 2,048 signed points. E3 checks this record by record instead of assuming it.

### 2. Condition 8

**Correct.**
- `word_op` is 64 bit operations (`BITOPS.md`), and the rule is `6 · word_op` (`work.rs` 393). So 384 × `table_filter_bit_sets` is the exact `bitop/v1` difference, and it does not depend on the curve. The pilot's +402,498,432 at 32 bits equals 384 × 1,048,173.
- The counter is `PackedPairTable::entries`, which the curve and N fix.

**Not sufficient as drafted. Three gaps:**
- The drafted list narrows the prior study's "every per-phase operation count" to `phases[*].work_native`. That drops `adds`, `doubles`, `scalar_mults`, `gae` and `native`, which is where a filter bug that moved work between phases would show. E2 restores them.
- The `ops` ratio of exactly 1 at w = 0.1 and 1 is true by construction of the unit, not because the filter adds no native work:
  - `table_filter_bit_sets` is not in `cost.unpriced` (`methods.rs` 4698–4716), so `band.rs` never prices it;
  - AGENTS.md §5a asks that native work left out of GAE be priced at 0, 0.1 and 1 on the arm that does it, and the drafted rho distances for `dgbf` would leave it out.

  E4 adds a hand-priced band. It decides nothing, but it stops a wall-clock speed-up being shown beside a counted figure that hides the work bought for it. That is the relabelling the success criterion excludes.
- The `bitop/v1` rise is build-side only (E4 states this).

### 3. Decision rule and predictions

- **The rule is fixed, falsifiable and per size.** It requires conditions 1–10 and a corrected upper bound below 1, with a three-way reading, and the statistic is the same as the validated prior study's.
- **The predictions are stated per size with a model and model risks.** They decide nothing. The pilot values were seen before they were written, and the protocol says so.
- **The cache sizes in the model were wrong.**
  - Each region is sized at the next power of two above 4/3 of its entries. That gives 2¹⁶ slots (512 KiB) at 24 bits, 16 × 2¹⁴ (2 MiB) at 28 bits and 64 × 2¹⁵ (16 MiB) at 32 bits, not 1.3 and 8 MiB.
  - The filter sizes (32 KiB, 256 KiB, 2 MiB) and the qualitative reading were right.
  - E5 corrects the figures and keeps the ranges, because changing a range now, after the pilot, would be the worse fault.
- **E6 adds two model risks:**
  - branch mispredictions on the filter test;
  - a predicted undecidable result at 24 bits. The prior study's 2 sub-ms solves out of 1,344 make one or more such executions likely again (about 0.86 under a Poisson rate of 2), and that blocker is correctly excluded from the noise rerun.

### 4. Noise-rerun clause

**What holds.** The principle is sound and bounded:
- it is fixed before data;
- it allows one rerun per size;
- it reruns the whole session with the same spec;
- the decision reads the rerun alone, and the first session is never pooled;
- it never applies to the counted session;
- a second 42 makes the size undecidable.

**Four defects:**
1. The drafted trigger is not mechanical. "Host-scheduling blockers (foreign task slices or run-queue contention …)" and "never a blocker naming the arm" leave the classification of 13 grading-v3 L2 blockers to judgement: steal, sibling ticks, involuntary switches, interrupts, memory PSI, missing observations, loose preflight and others. A Haiku executor cannot apply that unchanged. E1 gives a closed list of anchored patterns and fails closed on anything else. It also excludes memory stall, because an arm's own allocations can cause it.
2. `run-ecbench` also exits 42 when `invalid_outcomes` is non-empty, which means a wrong or unverified answer. The drafted trigger could be read as vacuously true there. E1 requires `invalid_outcomes` to be `[]` and every refused execution to be L1.
3. The drafted rerun name `-r2` collides with the host-loss rerun name. E1 and E9 use `-nr`, and they define how a host loss interacts with the clause.
4. Blinding. On exit 42, `run-ecbench` writes no `control-comparison.json`. But `records.jsonl` exists and holds every wall time. E1 seals it, and every derived figure, until the rerun has exited. It also requires the trigger file to be committed before the rerun launches.

### 5. Curves, seeds, specs

**Checked and correct:**
- curve seeds `202610296<b>3` and `<b>4`, target seed `<b>6`, order seed `<b>8`, at all three sizes;
- the 6th-digit index (6) differs from XP-20261029-dg-blocked's (5);
- 24 targets per curve, 6 rounds plus 1 warm-up, `alternate`, L2, 1800 s;
- 1,344 executions per session, and 288 pairs per arm pair;
- N_inc and `max_trials` equal the prior study's;
- wall and counted workloads and measurement are identical per size;
- the rho arms are `rho.negation`;
- the spec descriptions already name `PROTOCOL.md and APPROVAL.md`.

**Freshness.** The drafted search covered `origin/main` and four branches, which is more than the prior study's curve search. P4 extends it to every open PR head and to the seeds, because another study of the same date could take the same index.

### 6. Disclosure

**Complete:**
- the prior study's results;
- the design note;
- the pilot's binary and its dgbf/dgb ratios;
- PR #2844's status;
- the `filter=1` search on `origin/main`.

**Missing** (E7, E10):
- whether the pilot's `dg` comparisons were seen;
- that the pilot's records live in the artifact store;
- the roles of the pilot seeds;
- a `filter=1` search over open PR heads;
- that this study's `dgb` runs in a different binary from the prior study's `dgb`, with an extra guard in its lookup. This matters only for the descriptive composition product (E8).

### 7. Rules and roadmap

- **Success criterion.** A faster executed data structure at identical charged GAE is a real component speed-up. It is placed outside tables A–C, next to `dg_blocked_component`, which exists in `scripts/build_ic_leaderboard.py`. With E4, it is shown together with the native work it adds. It is not relabelling.
- **Validation gate.** The gate, L2 admission (conditions 2, 5 and 6) and the A/A control are as in the validated prior study.
- **Why approve.** This is constant engineering, not an exponent mechanism. I approve it for three reasons:
  - it is cheap: six jobs on an established route;
  - it addresses the probe cost that the prior study measured (target-PDP phase up by about a third);
  - it closes the remaining hash-table constant named by `LEAD-ICP-prime-ic-constant-engineering`, with no exponent claim riding on it.

## Not changed

- §1, §2, §5's wall statistic, §6.1 conditions 1–7 and 9, §6.2's decision table, §9 and the six specs stand as written.
- §7's predicted ranges stand.

## Decision record

The decision record is not yet in the ledger. I had no shell in this engagement, so I could not allocate ids with `tools/allocate_id.py`. The session that commits this file must do four things:
- allocate both ids and confirm each with `--check`;
- archive the decision below as `ledger/decisions/DEC-20261029-<tok>.yaml`;
- archive the handoff below as `ledger/handoffs/TASK-20261029-<tok>.yaml`;
- record the sha256 of this file and of the edited `PROTOCOL.md` on the launch cards.

```yaml
type: coordinator_decision
id: DEC-20261029-<allocate>
date: 2026-10-29
subject: backlog/XP-20261029-packed-filter/PROTOCOL.md (+ spec-P{24,28,32}.json, spec-P{24,28,32}-rho.json)
decision: approve_with_edits
edits: [E1, E2, E3, E4, E5, E6, E7, E8, E9, E10]
conditions: [P1, P2, P3, P4, P5, P6]   # fail-closed: no launch, return to coordinator
rationale:
  - "Single-variable change: dgbf differs from dgb only by ',filter=1' in one binary; arm order is XP-20261029-dg-blocked's validated layout shifted by one; A/A selection and native admission map exactly as there."
  - "Condition 8's bitop identity (384 x table_filter_bit_sets; word_op = 64, rule 6 word_op) is correct, but the drafted field list narrowed the per-phase identity to work_native; E2 restores adds/doubles/scalar_mults/gae/native."
  - "Unfiltered mitm-prime-blocked can silently fall back to small maps; the prior study's A1 packed-layout check is added as condition 10 (E3)."
  - "table_filter_bit_sets is native work outside cost.unpriced, so the ecbench band never prices it; E4 adds a hand-priced section 5a band for dgbf and requires it wherever counted dgbf figures are shown."
  - "Noise-rerun clause is sound in principle but not mechanical, admits exit 42 from invalid outcomes, collides with the host-loss -r2 name and leaves the first session's records unsealed; E1/E9 fix all four."
  - "Section 7 slot-array sizes were entries not slots (2 MiB and 16 MiB, not 1.3 and 8 MiB); corrected with ranges unchanged (E5). P24 is predicted undecidable from the prior study's sub-ms solves (E6)."
evidence_refs:
  - src/cryptanalysis/ic_boundary.rs 5719-6098, 12134-12200 (working tree of claude/ic-packed-filter-20261029; equality with 3271a25d02 checked by P1)
  - src/cryptanalysis/ecbench/work.rs 387-393; docs/units/BITOPS.md "2026-10-29 (packed pair-table filter)"
  - src/cryptanalysis/ecbench/band.rs 35-152; src/cryptanalysis/ecbench/methods.rs 4617-4716
  - src/cryptanalysis/ecbench/record.rs 1280-1480 (grading-v3 L2 blockers)
  - taskq/deploy/eks-karpenter/run-ecbench 54-67
  - research/ic_dg_blocked_20261029/VERDICT.md; research/ic_dg_blocked_20261029/cards/RULING-P2.md; research/ic_dg_blocked_20261029/validation/frozen/{PROTOCOL.md,APPROVAL.md}
scope: "prime-field IC, b in {24, 28, 32}, N_inc, stride-renew-deferred-guard, mitm-prime-blocked:m=2,filter=1 vs mitm-prime-blocked:m=2 (packed layout only), incremental-gauss, c8a.2xlarge, image 32144a29, ecbench 21701be6 from 3271a25d02; engineering (constant); no exponent or counted claim"
knowledge_promotion: "not_warranted: protocol approval, no evidence yet"
next_actions:
  - "Author: apply E1-E10 verbatim and fill E7's fields; commit PROTOCOL.md, APPROVAL.md and the six specs as the freeze; push and open or refresh the PR."
```

```yaml
type: handoff
id: TASK-20261029-<allocate>
date: 2026-10-29
from: coordinator
to: experiment-director
objective: "Run the read-only pre-launch checks P1-P6 of backlog/XP-20261029-packed-filter/APPROVAL.md against the freeze commit, and write the frozen launch cards for the six jobs only if all pass."
inputs:
  - backlog/XP-20261029-packed-filter/PROTOCOL.md and APPROVAL.md at the freeze commit (worker context: APPROVAL.md P1-P6 and PROTOCOL.md sections 4, 6.1, 9)
  - backlog/XP-20261029-packed-filter/spec-P{24,28,32}{,-rho}.json
  - backlog/XP-20261029-dg-blocked/spec-P{24,28,32}{,-rho}.json (P3 comparison only)
constraints:
  - "Read-only checks; open no pilot records.jsonl and no wall field."
  - "Any failed check stops the task and returns to the coordinator; no edits to the frozen directory."
deliverables:
  - research/ic_packed_filter_20261029/cards/prelaunch-checks.md (each check, its command, its output, pass/fail)
  - research/ic_packed_filter_20261029/cards/launch-P{24,28,32}{,-rho}.md (frozen cards citing freeze SHA, spec sha256, image digest, ecbench sha256), only if P1-P6 all pass
completion_gate: "prelaunch-checks.md committed and pushed with every check marked pass, or the first failure reported to the coordinator; no job launched by this task."
archival_task: "one snapshot archive of research/ic_packed_filter_20261029/cards/ only"
budget: {}
```
