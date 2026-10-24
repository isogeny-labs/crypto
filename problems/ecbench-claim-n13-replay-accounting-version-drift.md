# claim-n13 replay fails on the accounting change after the recording commit, not on the host path

- **Status:** `open`
- **Kind:** implementation gap (counted-work definition changed after the recording); not a proved obstruction
- **Affected work:** replay of `research/ecbench_rootfind_engine_20261012/sessions/claim-n13` (96 problems, all in the `ic-f4-m2`/`ic-f4-m3` arms; all 72 rho rows reproduce); `research/validation_sweep_20261019/D-known-issues/` item 1
- **Next action owner:** unassigned
- **Last updated:** 2026-10-23

## Problem

We cannot replay `claim-n13` on current `main` because the session was recorded at `2245b75` (Apple M4 Pro), where a host without BMI2 charged inherited-F4 specialisation lazily, and `main` charges it eagerly on every host. The cross-ISA split itself (BMI2 vs portable) no longer exists on `main`.

## Evidence and uncertainty

- **Observed (this x86 BMI2 host, `main` at a53a7e2bf, `IC_ARTIFACT_CACHE=off`):** replaying `claim-n13` gives 96 problems. Re-running the spec with the BMI2 path (eager `pext`/`pdep`) and with the portable path (eager shift runs, bit scatter for older rows) gives byte-identical `solver.ops`, counters and phase natives on all 224 records (e.g. `ic-f4-m2` first warm-up 419,828 both). The unit test `rewrite_paths_charge_identical_work_and_build_identical_rows` checks the same per specialisation step, per `InheritCost` field, and the rows.
- **Observed:** a temporary build with the lazy rewrite (`EagerMode::Off`, scatter), which is what a non-BMI2 host ran at `2245b75`, reproduces the recorded Apple `solver.ops` on all 32 `ic-f4-m2` rows (417,827, 439,059, 632,539, 618,562, ...). `ic-f4-m3` still differs (recorded 8,412,139; lazy 9,399,398; eager 10,914,662), so further code drift (for example F6-IC gate-first on by default, #2043) is mixed in.
- **Observed:** seq 58 also differs in `cost.total_gae` (741.09 recorded, 1,433.5 replayed): the recorded solver row is `priced_by: measured` (19.0 ns/op, wall 5,712,754 ns) while the replay prices it `pinned` (0.002304 ns/op), a separate pricing-table difference.
- **Derived:** lazy and eager accounting charge different word counts by construction (the unit test shows 12 vs 4 on a small chain), so no single charge reproduces both the pre-#2043 non-BMI2 record and post-#2043 sessions.
- **Suspected explanation:** the item-1 BMI2-off patch worked at `2245b75` because switching BMI2 off also switched eager to lazy there; since #2043 it switches eager Bmi2 to eager Runs and the charge no longer moves.
- **What this evidence does not establish:** Apple/aarch64 hardware was not run on `main`; the portable path was exercised on x86 by forcing it. History for `src/` was fetched shallow-since 2026-10-12 only.

## What would unblock it

- [ ] An owner decision: keep the post-#2043 eager definition as the method identity (then `claim-n13` is replayable only by its recording commit and must be re-recorded or marked pre-#2043), or restore the lazy definition (then every session recorded since 2026-10-17 on x86 stops replaying).
- [ ] Either way, a method-identity field naming the specialisation accounting version, so the audit refuses cross-version replay by name instead of reporting counts.

## Next discriminating step

- **Action:** replay `claim-n13` with the binary built at `2245b75` on this host (expect 96 problems on the default path, 0 with the item-1 patch, per the committed receipts) to confirm the recording side, then decide which definition is the identity.
- **Recheck trigger:** any change to `rewrite_kept`, `rewrite_step`, `rewrite_with` or `eager_mode` in `src/cryptanalysis/inherited_f4.rs`.

## Resolution and history

2026-10-23: the host-path dependence is closed on `main` (test above). The accounting-version drift is unresolved; recorded evidence under `research/` was not edited.
