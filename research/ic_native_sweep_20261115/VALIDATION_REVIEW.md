# Validation review: `research/ic_native_sweep_20261115/REPORT.md`

Validator (independent; did not write the code, the specs, the run or the report), 2026-11-15.
Standing of this review: `[pcc:unverified]` (no pcc claim is registered for it).

**Disposition: `validated-scoped`.**

Every number in REPORT.md's table and bullets reproduces from the raw tarball, recomputed with my own
scripts (not `analyze.py`). All 80 discrete-log certificates hold under my own elliptic-curve
arithmetic, and all 48 IC-arm identity checks hold on a stricter comparison than the producer's
script made. The code review finds the reported position set equal to the set of positions where
the CPU lookup succeeds, at `m = 2` and `m = 3`. Mutations that remove the first reported hit, the
`ε = −1` positions or the `m = 3` probe set are caught by `cargo test --release --lib rootfind_gpu`.
The scope is the one the report states: one L0 run on a 4-thread VM, wall-clock figures indicative.
Findings 1–9 are corrections or disclosures; none changes a measured number or the failed
prediction. [pcc:unverified]

## What was validated

| item | value |
|---|---|
| validated objects | `research/ic_native_sweep_20261115/` and `benchmarks/records/20261115T1740Z-ic-native-sweep.json` at `f365d11428`; byte-identical at `ea2cab1833` (dispatch head) and `036f1bc776` (origin head when checked; it adds only a `koblitz_index_calculus.rs` test change). The working tree was clean and equal to `f365d11428` on these paths (`REPORT.md` blob `6f3ec57bd1`, sha256 `2779e791d0d1287b…`) |
| code under test | `src/cryptanalysis/ic_framework/rootfind_gpu.rs` at `2ec4187be0`; `src/`, `gpu/`, `Cargo.toml` and `Cargo.lock` have no diff from `2ec4187be0` to `ea2cab1833` |
| raw tarball | `raw/out_native.tgz`: 77 674 B, sha256 `4b8351cad9fc4d488376bc57e740ff2a368acad1d95febd991fe354dcec972e9`, equal to the record's `raw[0].sha256` and the `4b8351ca…` in REPORT.md. 47 entries, no absolute or `..` paths. Extracted into scratch only |
| session bindings | For both sessions `records.jsonl`, `plan.json` and `host.json` hash to `records_sha256`, `plan_sha256`, `host_sha256` in `session.json`; seq 0–39 contiguous; no warm-ups; 40/40 records `verified`; every record L0 with the blocker "L1: no CPU reservation"; affinity CPUs 0–3 in every record |
| specs | `sessions/native2/spec.json` and `sessions/native3/spec.json` are byte-identical to `specs/native2.json` and `specs/native3.json` at `2ec4187be0` and at HEAD |
| binary | `binary_sha256 = 0d4e3842a78f…` in all 80 records, both audit receipts, the tarball's `sha256.txt` and the local `target/release/ecbench` (mtime 17:40:43Z). That binary contains the three error strings `2ec4187be0` introduced (`worker=native has no worker process`, `threads must be a count`, `worker must be spawn, daemon or native`) and not the string they replaced. `rootfind_serve` sha256 `4e51b9a2…` equals `sha256.txt` and `gpu/rootfind/rootfind_serve` |
| band files | `band-native2.json` and `band-native3.json` in Git are byte-identical to the tarball's |

## Requirements

| # | requirement | result | evidence |
|--:|---|---|---|
| 1 | Provenance: protocol and specs committed and pushed before the run; spec in session equals committed spec; tarball hash equals record | pass, with finding 1 | `2ec4187be0` committed 17:33:58Z; remote-tracking reflog "update by push" 17:34:01Z; record created 17:40:43Z at `2ec4187be0`, `commit_dirty: false`; command started 17:40:47Z; sessions ran 17:40:49Z–17:43:10Z (native2) and 17:45:31Z–17:46:28Z (native3); specs and hash as in the table above |
| 2 | Certificates: every recovered logarithm re-checked with independent EC arithmetic | pass (80/80) | scratch `scripts/certs.py` |
| 3 | Identity: IC arms equal the one-thread arm per workload and round | pass (48/48), with finding 2 | scratch `scripts/identity.py` |
| 4 | Every number in the table and bullets recomputed | pass, with findings 3 and 4 | scratch `scripts/table.py` |
| 5 | Code review of `NativeSweep`; mutation test | pass | sections "Code review" and "Mutations" below |
| 6 | L0 and indicative wording; failed prediction stated plainly | pass, with findings 5–8 | REPORT.md "Standing", "Results" |

### 2. Certificates

For each of the four curves I derived `a` and `b` from the generator and one target. I then checked
the following:

- `p` and `r` are prime, and the trace `p + 1 − r` equals the ICV1 trace and lies in the Hasse
  interval.
- The discriminant is non-zero, and the j-invariant equals the ICV1 `j` field.
- `r·G = O` with affine double-and-add written for this check.

Every target lies on its curve. For all 80 records, rho included, `recovered·G = target`,
`0 ≤ recovered < r`, and `recovered` equals `planted`. No failure. [pcc:unverified]

| curve | p | a | b | r |
|---|--:|--:|--:|--:|
| icv1-fp28-t7531-5f8c6474 | 187 834 783 | 178 200 012 | 134 334 653 | 187 827 253 |
| icv1-fp30-tm15113-0b67ffcf | 1 062 943 379 | 804 230 764 | 730 864 256 | 1 062 958 493 |
| icv1-fp34-t124571-a5bf1675 | 11 700 434 063 | p − 3 | 7 883 827 458 | 11 700 309 493 |
| icv1-fp36-tm325323-0291967d | 39 012 606 793 | p − 3 | 22 067 677 030 | 39 012 932 117 |

### 3. Identity

For each (session, curve, target, round), I compared each IC arm with `rf*-cpu`. The comparison
covered the following:

- status, recovered logarithm, `trials` and `counters.targets_tried`, relations, `S` and total GAE;
- `cost.unpriced`;
- every phase's `(name, adds, doubles, scalar_mults, gae)`;
- the linear-algebra native and work maps;
- all top-level counters;
- the `bitop/v1` work total and per-phase prims.

All 48 are equal. Beyond that, every relation-phase native and work counter is equal except the
`gpu_*` counters: `gpu_batches`, `gpu_targets_swept`, `gpu_targets_unused`,
`gpu_hits_confirmed_on_cpu` and `gpu_cpu_fallback_targets`. Those five are equal across the three
batched arms in every workload. The degenerate-target fallback fired in production on
`icv1-fp28` target 0, round 1: 2 targets, on all three batched arms, with identity holding.
[pcc:unverified]

### 4. Numbers

My band implementation reproduces `ecbench band`'s means to the printed digits for every arm. It
follows band.rs's rule: `cost.unpriced` is read from the top-level counters first and then from the
phase native maps, and counted `table_insert_attempts` is priced. All table cells and bullet ranges
reproduce:

| m | log2 r | S/rho (w = 0) | one thread rel. (s) | native ×1 | emulation | native (4) | native/emu | native(4)/native ×1 | solve native vs 1 thread |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 2 | 27.48 | 5 967.1 | 1.853 | 0.857× | 2.838× | 3.146× | 1.109 | 3.671 | 2.856× |
| 2 | 29.99 | 24 637.5 | 11.230 | 0.927× | 3.330× | 3.322× | 0.998 | 3.586 | 3.269× |
| 3 | 33.45 | 105.8 | 0.465 | 0.558× | 2.265× | 2.244× | 0.991 | 4.023 | 1.179× |
| 3 | 35.18 | 161.9 | 1.537 | 0.767× | 2.791× | 2.880× | 1.032 | 3.757 | 1.524× |

The bullet ranges from these cells are as follows:

- native/emulation: 0.991–1.109;
- native(4)/`threads=1`: 3.586–4.023;
- `threads=1` against one thread: 0.558–0.927;
- the two 4-thread arms against one thread: 2.244–3.330;
- solve: 2.856–3.269 at `m = 2` and 1.179–1.524 at `m = 3`;
- S/rho: 105.8–24 637.5.

The record's two metrics (3.38 s against a 3.373 s baseline; 0.534 s against 0.551 s) match. The
PDF and SVG carry the same numbers. [pcc:unverified]

## Code review of `NativeSweep` (`2ec4187be0`)

- **Superset (in fact equality) at `m = 2`.** The CPU lookup succeeds at position `2i + s` exactly
  when `t = R ∓ F_i` is a base point (`fb.index_of_key(key(t))`, then the confirming addition, which
  cannot fail for a correctly computed `t`). The oracle refuses a base that is not negation-closed
  (`prepare`). So `x(t)` in the base's abscissae is equivalent to success. `NativeSweep` computes
  `x(t) = λ² − x_R − x_i` with the same slopes as `decompose`: `λ = (y_R + y_i)/(x_R − x_i)` at
  even positions and `(y_R − y_i)/(x_R − x_i)` at odd ones. It then tests membership of the
  Montgomery value.
- **Montgomery equality.** `Fp64::reduce`, `add` and `sub` return fully reduced values in `[0, p)`,
  so equal field elements have equal Montgomery words. The probe set holds the reps' Montgomery `x`
  at `m = 2` and `to_mont` of the pair abscissae at `m = 3`.
- **`m = 3`.** The probe set is `PairTable::map_abscissae()`, every key of the plain map (`key >> 1
  = x + 1`). `PairTable::probe` succeeds whenever `x(t)` is a key (the stored sum is `±t`). The
  reported set is therefore the success set.
- **Hash set.** Linear probing at load ≤ 1/2, with full 64-bit comparison and sentinel `u64::MAX`.
  The sentinel cannot be a Montgomery value below `p`. There are no false positives and no misses.
- **Order.** Positions are pushed in ascending order. `par_iter().map_init().collect()` on a slice
  keeps input order, so each target's hits arrive in the CPU sweep's order.
- **Degenerate rows and identity candidates.** `t = O` only when `R = ±F_i`, that is `x_R = x_i`.
  `sweep_one` skips such rows (with `d := 1` so the batched inversion stays valid). `decompose`
  hands any target with `x_R ∈ fb.x_index` to the CPU oracle whole (`gpu_cpu_fallback_targets`).
  `R = O` returns before any sweep, and pending points at infinity are filtered out of the batch.
- **Can a missed hit change the charge?** Yes. Positions before the first reported hit are charged
  as misses (`charge_misses`: one addition, 4 multiplications, 1 squaring, 1 lookup each). That is
  exactly the CPU's charge for a candidate whose lookup fails (`charge_candidate` plus the lookup's
  `+1`), so a false positive could not change the charge. A missed first-success position makes the
  target stop at a later hit or at none, which changes the charged additions or the trials (see
  mutations 2 and 5). A missed hit after the first one is unobservable, because `decompose` returns
  at the first success and every reported position succeeds.

## Mutations

The mutations were applied in a scratch git worktree at `2ec4187be0`, switched at run time by
`VALIDATOR_MUT` (diff in scratch `mutation.diff`). The test command was `cargo test --release
--locked --lib rootfind_gpu` with a scratch `CARGO_TARGET_DIR`. Nothing was changed in the
repository's working tree. The GPU-worker tests print "skipped" and pass without
`ICX_ROOTFIND_WORKER`; `native_sweep_matches_rootfind_subtract` does not need a worker and runs.

| mode | mutation | result | reading |
|--:|---|---|---|
| 0 | none | 8 passed | baseline (also 8 passed on an unmodified build) |
| 1 | drop every other hit, first kept (`step_by(2)`) | 8 passed | equivalent mutant: only the first reported position per target is used |
| 2 | drop every other hit, first dropped | caught: `m=2 … walk seed 7 planted 11: relation ops`, 21 851 against 21 928 additions, same targets | a missed first hit changes the charge |
| 3 | drop every `ε = −1` position | caught: `targets` 668 against 761 | |
| 4 | `m = 3` probe set = the base's abscissae | caught at `m = 3`: `targets` 19 against 2 276 | |
| 5 | drop only the first hit | caught: `relation ops` as mode 2 | |
| 6 | remove the `x_i = x_R` skip | 8 passed | equivalent mutant: such targets go to the CPU oracle whole |
| 7 | probe: panic in the degenerate fallback under `worker=native` | panicked | the unit test does reach the degenerate-target path, so mode 6's survival is equivalence, not missing coverage |

## Findings

1. **Provenance: a second commit inside the run.** The producer committed `027ccd5fc7` (17:41:32Z)
   and `6cac7eb73d` (17:41:46Z) into the measured checkout while native2 was running. native3's
   `session.json` therefore records `git_commit` `6cac7eb73d`, not `2ec4187be0` (both
   `git_dirty: false`). Those commits touch only the benchmark record, the session log and
   `analyze.py`. Both sessions carry the same binary hash, so the code under test is unchanged. The
   report says only that the record was initialised on the clean pushed commit; it should say that
   native3 records `6cac7eb73d` and why that does not matter.
2. **The producer's identity script checks less than the report states.** `analyze.py`'s `charged`
   key reads `p.get("dbls")` and `p.get("field_ops")`. No phase of any record has either key (the
   key is `doubles`, and there is no `field_ops`), so both sides read `None`. The script therefore
   compared phase names and additions only, not "every phase's charged ledger". The claim
   nevertheless holds on my comparison (requirement 3).
3. **Prediction 2's range pools `m = 3` cells.** The protocol's prediction 2 is for `m = 2`. At
   `m = 2` the ratio of medians is 1.00–1.11× (the report's 0.99–1.11× includes the `m = 3` 0.99× and
   1.03×). Paired per workload and round, `m = 2` gives 0.95–1.15× and `m = 3` gives 0.93–1.04×. The
   prediction fails under every reading: the largest paired value, 1.15×, is below 1.5×. [pcc:unverified]
4. **The §5a band is given at `w = 0` only.** The band files hold all three weights. The distance to
   the matched rho is 106–24 637× at `w = 0`, 151–24 680× at `w = 0.1` and 293–24 875× at `w = 1`.
   At `m = 3` the `w = 1` distance is 354× (34-bit) and 293× (36-bit), against 106× and 162× at
   `w = 0`. No ordering changes, because every IC arm stays far above rho. The bullet should quote
   the band.
5. **"Because" in the batching-cost bullet is an explanation, not a tested cause.** It is
   consistent with the counters. Per workload, the native sweep evaluates 1.02–2.16× the candidates
   the one-thread arm examines (`gpu_targets_swept · 2|F_x|` against the CPU arm's relation-phase
   `lookups`). The measured `threads=1`/one-thread relation wall is 1.05–1.85×. No arm isolates the
   cause, for example a native sweep that stops at a target's first hit.
6. **The native arm's own setup work at `m = 3` is not named.** The arm builds a 2²³-slot probe set
   (64 MiB) from about 4.19 million pair-table abscissae (`n_probe = 4 192 870` in the emulation
   worker's stderr; the same `probe` vector). No counter records it. The native arm's median
   `oracle_setup` wall is 0.726 s and 0.783 s, against the one-thread arm's 0.599 s and 0.575 s. The
   `m = 3` solve-wall explanation names the pair table and linear algebra but not this extra setup.
   The same holds for the emulation arm's device-side table.
7. **Pool sizes are asserted, not recorded.** The native pool comes from `available_parallelism()`,
   and every record's affinity is CPUs 0–3. The emulation pool comes from `rf_cpu_budget()`
   (`hardware_concurrency` capped by a cgroup quota). This container's cgroup v1 quota is −1 today,
   and spawn mode prints no thread count. Both are consistent with 4, and the 3.6–4.0×
   all-threads/`threads=1` ratio is consistent with 4 workers. Neither size is in the evidence.
8. **The host was loaded during the wall measurements.** The 1-minute load average before each run
   was 2.10–3.37 in native2 and 1.30–1.68 in native3, on 4 CPUs, and CPU PSI avg10 reached 7.9% in
   native2. The paired native/emulation ratios spread over 0.93–1.15×. "Equal within 11%" (a ratio
   of medians) is therefore the right strength, and the 1.11× at 28 bits is not resolved at this
   isolation. The report already calls the wall figures indicative.
9. **Record metadata.** `reports.pdf` names `REPORT.md`, not `REPORT.pdf`. `raw[0].uploaded` is
   `false`, and the 77 674 B archive sits in Git, above the 64 KiB data-file threshold of AGENTS.md
   §13. The report discloses the S3 waiver. `validation` is `null`, as expected before this review.

Not findings, for the record:

- The IC arms' `S` is the CPU path's charge, re-derived in `decompose` from the hit positions, so
  "the same on every IC arm" is an accounting identity. The arms' extra executed work is outside
  `S`: full sweeps, unused targets and the `m = 3` probe set. The report says so ("by design"; the
  `gpu_*` counters).
- The replays in both audit receipts ran on the run's own host, with the same binary and the same
  environment class. The report does not claim a cross-host audit.

## Wording (requirement 6)

The report stays within the L0, indicative wording:

- the standing is `[pcc:unverified]`;
- it is one local L0 run with `--cpus none --allow-busy`;
- the wall figures are labelled indicative;
- no scoreboard or leaderboard row is added;
- it says explicitly that nothing is a GPU measurement.

Prediction 2's failure is the first result bullet and is stated plainly, with the predicted 1.5×.
"What this changes" is scoped to "on one host" and "at least on this host", and the 96-thread
question is left untested, as it should be. The corrections it needs are findings 1, 3, 4 and 6
(disclosure and ranges); finding 5 is a wording softening ("consistent with" for "because").

## Not checked

- No independent replay with a binary built by me, and no replay on another host class. The audit
  receipts are same-host and same-binary. I built only the library test binary, from a sparse
  scratch worktree.
- No wall-clock re-measurement.
- The target-generation law (`uniform_scalar_sha256_v1`) was not recomputed from its seeds;
  certificates check `recovered·G = target` and `recovered = planted` only.
- The emulation worker's C++ (`gpu/rootfind/`) was not reviewed beyond its thread-budget function.
- Round 3's 96-thread per-core figure was not re-examined; only its presence in
  `research/ic_gpu_rootfind_big_20261115/REPORT.md` was confirmed.
- The push time of `2ec4187be0` comes from the local remote-tracking reflog ("update by push",
  17:34:01Z), not from the server.

## Scratch evidence (not committed)

`/tmp/claude-0/-home-user/648c3177-6621-55ff-b9d8-09ead07ab678/scratchpad/validator-native/`:

- `scripts/certs.py`, `scripts/identity.py`, `scripts/table.py` and `scripts/mutate.py`;
- `mutation.diff`;
- `test-baseline.log`, `test-mut{0..6}.log` and `test-mut{0,7}-b.log`;
- the extracted tarball in `data/out_native/`.

Process note: `git sparse-checkout init` in the scratch worktree wrote `core.sparseCheckout = true`
into the shared `.git/config`. I removed it about a minute later. The main working tree stayed
clean, and there is no `.git/info/sparse-checkout`. The scratch worktree and its build directory
were removed after the tests (`git worktree remove`), so the logs and `mutation.diff` are what
remain.
