# Drift check: research/ic-representative-selection_20261105, frozen c6cd50bcc8 against HEAD 0c2cc1b408

Date 2026-11-14. Track A of the lead-chasing program. Every number below is
`[pcc:unverified]`: none has a pcc claim id, and none was checked by a verifier
other than the session that produced it. Nothing here is a result of the study.

## Verdict

1. **Counted quantities.** For the methods the study's specs call
   (`rho.negation`, `ic.pipeline` with `prime-abscissa:size=32` and
   `mitm:negation_folded=1`, `bsgs.negation`, plus the class generator
   `volcano-rho`), I found no change to an operation count, to S, or to the
   spec schema between c6cd50bcc8 and HEAD. Basis: exact replay of committed
   pre-freeze records on the HEAD binary reproduced identically, the HEAD
   `volcano-rho` regenerated a committed v2 class byte for byte, and all 205
   specs written by Stage 0 plan and validate under HEAD. I read the hunks on the
   study's call path; most are additions behind options that are off for these
   arms. Building at HEAD therefore gives the same counted quantities as building
   at c6cd50bcc8, **as far as this evidence reaches** (see "Limits").
2. **Two changes that are not count changes but do alter what the frozen
   protocol and its scripts say.**
   (a) Since 2026-11-08 `ecbench band` and `ecbench compare` price the IC
   table's counted inserts by default (`table_insert_attempts`); the study's
   `ic-mitm` configuration is expected to emit that counter behind
   `counted:table_insert_attempts` (a pipeline test pins it for this family; no HEAD
   record of the exact spec exists yet), so `run/06-analyse.sh`'s cross-check (`ecbench band ... --weights 0,0.1,1`, no
   flag) would price an extra term that `repsel analyse` (which reads
   `cost.unpriced` only, the pre-2026-11-08 reading) does not, and the per-arm means
   would not agree at w = 0.1 and 1 for the IC arm.
   (b) The binaries Stage 0 hashed were built at HEAD, not at the frozen commit;
   `inputs/tools.sha256` line 1 says `0c2cc1b408`.
   Neither is hidden: both go into amendment A1 (see READINESS.md).

## What the study uses (from its own files)

| used by | what | where |
|:--|:--|:--|
| `design.json` `class_search.command` | `volcano-rho` flags `--bits --depth --max-cofactor --ell --crater --seed --controls --level-caps --models --crater-models --targets --ecbench --out` | `src/bin/volcano_rho.rs` |
| `native/src/specs.rs:42-58` | arms `rho.negation` (x2), `ic.pipeline` (`prime-abscissa:size=N`, `mitm:negation_folded=1`), ctl stage adds `bsgs.negation` and `ic.pipeline` size 33 | `src/cryptanalysis/ecbench/methods.rs` (entries `ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())` line 587, `ic_framework::run_pipeline` line 1103, `ecbench::generic::bsgs_negation` line 817) |
| `native/src/specs.rs:468-480` | `ecbench.spec/v1`, `prime_explicit` workloads, `planted_shared` targets, L2, 5+1 rounds | `workload.rs`, `spec.rs`, Iso Lab contract |
| `native/src/analyse.rs:6-10,56-100` | price `S(w) = (total_gae + w (U + extra_ops)) / sqrt(r)`, U from `cost.unpriced` as `band.rs` counted it at freeze | `src/cryptanalysis/ecbench/band.rs` |
| `run/03-launch.sh`, `run/04-collect.sh` | `isolab.eks.request/v1` | `taskq/deploy/eks-karpenter/remote/validate-request`, `launch-remote`, `contracts/` |
| `run/06-analyse.sh` | `ecbench band --weights 0,0.1,1 --json` | `src/bin/ecbench.rs`, `band.rs` |

## Inventory of what changed since c6cd50bcc8 (`git diff --stat c6cd50bcc8 HEAD`)

The whole-repository diff is large (the checkout moved a long way); restricted
to what the study touches:

| path | change | affects a counted quantity? |
|:--|:--|:--|
| `research/ic-representative-selection_20261105/*` | only `protocol.json` freeze block (`status`, `protocol_sha256`, `commit`, `frozen_at`, `filed_before_affected_data`, lines 1150-1157; `git diff -w` shows 5 lines) and a one-line `README.md` note, both by commit bf3d2a2016 | no; this is the freeze itself |
| `src/bin/volcano_rho.rs` | +80 lines | no, with evidence below |
| `src/bin/ecbench.rs` | `git diff --stat`: 485 insertions, 22 deletions (the brief said about 345) | not for counts; reading change in `band`/`compare`, see below |
| `src/cryptanalysis/ecbench/{band,compare,...}.rs` | band and compare options, pooled compare, sentinels, `wcal` | pricing reading only |
| `src/cryptanalysis/ic_boundary.rs` | +12,126 lines of diff, 419 hunks, 310 of them touching existing lines | no on the study's path, with evidence below |
| `src/cryptanalysis/ic_framework/{mod,plugins,linalg,rootfind,solvers}.rs` | solver-unit pricing keys, new oracle options, new modules | no on the study's path (all off or absent for these arms) |
| `src/cryptanalysis/prime_fast.rs` | 869 lines | not on the path: `collect_relations`/`PairTable` hunks at HEAD lines 1189-1445 in `prime_fast.rs` are the fast-table variants; the replay below exercises the path |
| `taskq/deploy/eks-karpenter/{contracts,remote/validate-request,remote/launch-remote,image.json,Dockerfile.ecbench,publish-image}` | unchanged | no |
| `taskq/deploy/eks-karpenter/remote/run-request` | +3 lines, line 4-6: exports the launcher `bin/` on PATH (exit 127 fix) | no count; affects whether a launch from an older source works, see READINESS |
| `.github/workflows/isolab-experiment.yml`, `Cargo.lock` | unchanged | no |
| `tools/task_card.py`, `tools/benchrec.py` | +473 and +375 lines | no count; the card flags in `run/CARD.md` still match `benchrec.py` argparse (init `--slug --problem --entry`, run, add-raw `--s3-uri`, close `--status --summary`) |

## Evidence

### E1. Exact replay at HEAD of records written before the freeze

Source: `research/isogeny-volcano-cost-v2_20261017/sessions/l2-s1001/{spec,spec-icctl,spec-bsgs}`,
recorded by commit 156f80c821 (2026-10-17), which is older than c6cd50bcc8, with
the same method ids and parameters as the study (`rho.negation`;
`ic.pipeline` `prime-abscissa:size=32`, `mitm:negation_folded=1`; `ic-mitm-plus1`
size 33; `bsgs.negation`). Decompressed copies were audited with the HEAD
`ecbench` (sha256 `73c4f66b…e581e7`), `verify --replay 200 --replay-seed 20261114`:

| session | records | replayed | result |
|:--|--:|--:|:--|
| `spec` (rho-neg, ic-mitm) | 10,240 | 200 random | 0 problems, 200 reproduced |
| `spec-icctl` (ic size 33) | 560 | 200 random | 0 problems, 200 reproduced |
| `spec-bsgs` (bsgs-neg) | 1,280 | 200 random | 0 problems, 200 reproduced |
| `spec`, named records | seq 5120 (ic-mitm), 5121 (rho-neg), 9000 (ic-mitm), 9001 (rho-neg) | 4 | all `identical` |

Logs: `logs/verify-*.txt`, `logs/verify-spec-seq.json`. `verify` re-executes the run
and requires identical counts; it exits 1 on a difference. This is the strongest
evidence here, because it checks behaviour and not my reading of the diff.

### E2. Class generator regenerates a committed class byte for byte

`./target/release/volcano-rho --bits 24 --depth 3 --max-cofactor 4096 --ell 2
--crater split --seed 1001 --level-caps 16,4,4,12 --models 4 --crater-models 16
--aa-model0 --targets 8 --ecbench <scratch>` (the v2 `PROTOCOL.md` section 2 command)
produced `curves.jsonl` with sha256 `967912fb…a23842`, identical to
`research/isogeny-volcano-cost-v2_20261017/classes/l2-s1001/curves.jsonl`
(`cmp` exit 0), and the four `spec*.json` files equal after dropping the free-text
`label`/`description`. Log: `logs/volcano-rho-v2-regen.log`.

The `volcano_rho.rs` hunks (HEAD lines 45-100, 144-200, 626-701, 763) replace
`u128 %` by `MulModN::mul`, the `i128` Euclid by `ic_boundary::invmod`, and `%`
by compare-and-subtract `add_n`/`neg_n` guarded by `a < n`. The file's own
comment states `iters`, `k` and the log line are unchanged and only `ns` moves;
E2 is consistent with that. The hunk header also says rows from before and after
this change "must not be pooled on time". The study never pools across the two
builds (one image for all jobs), so this does not bite.

### E3. Specs and requests

All 205 job specs written by `run/02-specs.sh` pass `ecbench plan --spec` (HEAD
binary), exit 0. All 205 `isolab.eks.request/v1` requests built exactly as
`run/03-launch.sh` builds them, with a **placeholder** image receipt (zero
digests, not a real image), pass `taskq/deploy/eks-karpenter/remote/validate-request`;
the largest request is 22,628 bytes (limit 65,536) and the longest job name is 22
characters (limit 55). Scratch only; nothing was submitted.

### E4. Reading of the diff on the call path

- `rho.negation`: `rho_walk_with` (HEAD `ic_boundary.rs:2966`) calls
  `rho_walk_with_policy(..., WalkPolicy::legacy())`. The changed regions of
  `rho_walk_with_policy` (HEAD 3012-3250) are gated by `policy.chained_setup`
  (`legacy()` sets it false, line 1765), by `continue_walks` (false) and by
  `verify_ladder` (empty); in the legacy path the new `fresh_starts` counter
  equals the old `walks > 1` condition. The new `work_native: tally.work_native()`
  field (line 3247) is additive. `TunedWalker::walk` has six pure-addition hunks
  (HEAD 2778-2882) that only increment `tally.window_word_ops` and
  `tally.brent_word_ops`; these feed `work_native` (`cycle_window_word_ops`,
  `brent_word_ops`: `bitop/v1` only, not in `cost`, and `band.rs` prices only
  `table_insert_attempts` and the sibling-walk counters, lines 135 and 144). So a
  `rho-neg` record at HEAD gains a `work_native` block that a pre-2026-11-08
  record lacks, with no change to `total_gae`, `counters` or `cost.unpriced`.
  The `RhoWalk::negation()` constants (line 1712) are unchanged.
- `PhaseCost::count` now goes through `bump_counter` (HEAD 716-722); its doc
  states the same map after every call, a counter created at zero when `by` is 0.
- `PairTable` hunks (HEAD 8467-8723): new `filter_bit_sets` field, `filter=0|1`
  option (default off), `build_negation_folded_prime_blocked_with`.
  `plugins.rs:3026-3083` adds the `filter` parameter, default 0.
- `PrimeAbscissaBase` (`plugins.rs:93-179`): adds `size=auto` and `scale`; for
  `size=32` the code path is `params.u64("size")` as before, and `scale=` without
  `size=auto` is refused.
- `ic_framework/mod.rs:680-835`: solver-unit pricing keys, `pricing_qualification`
  (`skip_serializing_if` none), applied only when a unit key is pinned in
  `docs/ic/calibration.json`; the comment states no row carries one, so the
  match is the one it was.
- `generic.rs:91-123,243-281`: BSGS counters batched into `StepCounters` and
  flushed once; same counter names and values, a counter present only when
  non-zero. Only the `ctl` stage uses `bsgs.negation`.
- `record.rs:742-768`: a static-DH oracle is passed only for methods that need
  one (`needs_static_dh_oracle`); `None` otherwise.
- `workload.rs:1551-1625`: `hash_to_subgroup_counted` wraps the old derivation
  ("the result is the same function of the inputs"); this is the `planted_shared`
  target derivation, and E1 covers it in practice.

### E5. The pricing-reading change (the item that does not match the frozen text)

- `band.rs:29-44,91-135` and `ecbench.rs:415-433,460-475,1909-1926,2100-2116`:
  `Pricing::table_work` is on by default since 2026-11-08 (owner); `--no-price-table-work`
  restores the earlier reading and reproduces every band file saved before that date.
- `ic_framework/mod.rs:1332-1337` (a test that pins this very configuration:
  `prime-abscissa`, `mitm` with `negation_folded=1`, `Targets::Walk`; the test
  passes plain `size=24`, `m=2` through `MitmOracle`, not the Iso Lab
  `PrimeBatchMitmOracle`, so this is evidence for the family, not for the exact
  oracle object the spec selects) asserts that the oracle-setup phase carries
  `counted:table_insert_attempts` and a non-zero `table_insert_attempts` count.
  Whether the spec's `mitm:negation_folded=1` resolves to the same marker at run
  time is confirmed only by a HEAD-built record; Stage 0 produces none. The committed v2 records, written before the
  marker existed, carry no `work_native` at all (checked on seq 5120).
- PROTOCOL.md section 7 fixes `U` "counted exactly as `ecbench band` does" at the
  freeze, and `native/src/analyse.rs:6-10` implements the freeze-time reading
  (`cost.unpriced` and the non-operation byte exclusions). `06-analyse.sh` calls
  `ecbench band` without a flag, which at HEAD means the new default.
- Size, *derived* and unmeasured: the folded pair table for a base of 32 abscissae
  has on the order of 10^3 inserts per run; at 24 bits sqrt(r) is about 1,470, so the
  extra term at w = 1 is on the order of 0.4 to 0.8 S against an IC S of 54-80
  (PROTOCOL.md section 2), the same table in every IC arm. It would shift the
  `ecbench band` cross-check, not `repsel analyse`.
- Fix to describe (frozen file, not edited): either add `--no-price-table-work` to
  the `ecbench band` call in `run/06-analyse.sh` (keeps the frozen reading), or
  file an amendment that adopts the 2026-11-08 default and teaches `repsel analyse`
  to add the phase sum of `table_insert_attempts` to U. This is a decision for the
  protocol owner, not a mechanical edit.

## Limits (what this check does not establish)

- I did not build at c6cd50bcc8 itself (no worktree: free space was 3.3 GB at
  start, and the instruction forbade it). "Same counted quantities" rests on E1
  (records made by an older commit replay identically on HEAD) and E4 (reading),
  not on running both binaries on the same specs. A binary at c6cd50bcc8 and a
  `cmp` of its records against HEAD's on one job spec would close this; the replay
  of older records is a one-directional check (old behaviour reproduced by new code).
- The `ic_boundary.rs` diff is 12,126 lines. I read the functions on the study's
  call path (E4) and listed every modified-hunk function name by count; I did not
  read hunks in unrelated functions (`rho_batch_with`, `ic_residual_batch_with`,
  `FrobeniusPairTable`, walk-table tests, and similar), which these arms do not call.
- A stale binary from 2026-11-07 (`target/release/ecbench`, sha256 `2522feb8…969c49`,
  commit unknown) was overwritten by the Stage 0 build; I kept a copy in the
  session scratchpad and did not use it as evidence.
- The `protocol_sha256` (`dcee3b0f…a8188`) was not recomputed: its tool is Python
  (`crypto-autoresearcher/tools/experiment_protocol.py`), which this track does not run.
  `protocol.json` on HEAD has sha256 `6c263a04…e334f` as a file (that includes the
  freeze block and is not the canonical protocol hash).
