# claim-n13 replay fails on the accounting change after the recording commit, not on the host path

- **Status:** `resolved`
- **Kind:** implementation gap (counted-work definition changed after the recording); not a proved obstruction
- **Affected work:** replay of `research/ecbench_rootfind_engine_20261012/sessions/claim-n13` (96 problems: 48 in the `ic-f4-m2`/`ic-f4-m3` arms, which this record is about, and 48 in the `ic-rootfind-m2`/`ic-rootfind-m3` arms, which differ only in `phase counts` and are a separate drift, see below; all 72 rho rows reproduce); `research/validation_sweep_20261019/D-known-issues/` item 1
- **Next action owner:** none
- **Last updated:** 2026-10-28

## Problem

We cannot replay `claim-n13` on current `main` because the session was recorded at `2245b75` (Apple M4 Pro), where a host without BMI2 charged inherited-F4 specialisation lazily, and `main` charges it eagerly on every host. The cross-ISA split itself (BMI2 vs portable) no longer exists on `main`.

## Evidence and uncertainty

- **Observed (this x86 BMI2 host, `main` at a53a7e2bf, `IC_ARTIFACT_CACHE=off`):** replaying `claim-n13` gives 96 problems. Re-running the spec with the BMI2 path (eager `pext`/`pdep`) and with the portable path (eager shift runs, bit scatter for older rows) gives byte-identical `solver.ops`, counters and phase natives on all 224 records (e.g. `ic-f4-m2` first warm-up 419,828 both). The unit test `rewrite_paths_charge_identical_work_and_build_identical_rows` checks the same per specialisation step, per `InheritCost` field, and the rows.
- **Observed:** a temporary build with the lazy rewrite (`EagerMode::Off`, scatter), which is what a non-BMI2 host ran at `2245b75`, reproduces the recorded Apple `solver.ops` on all 32 `ic-f4-m2` rows (417,827, 439,059, 632,539, 618,562, ...). `ic-f4-m3` still differs (recorded 8,412,139; lazy 9,399,398; eager 10,914,662), so further code drift (for example F6-IC gate-first on by default, #2043) is mixed in.
- **Observed:** seq 58 also differs in `cost.total_gae` (741.09 recorded, 1,433.5 replayed): the recorded solver row is `priced_by: measured` (19.0 ns/op, wall 5,712,754 ns) while the replay prices it `pinned` (0.002304 ns/op), a separate pricing-table difference.
- **Derived:** lazy and eager accounting charge different word counts by construction (the unit test shows 12 vs 4 on a small chain), so no single charge reproduces both the pre-#2043 non-BMI2 record and post-#2043 sessions.
- **Suspected explanation:** the item-1 BMI2-off patch worked at `2245b75` because switching BMI2 off also switched eager to lazy there; since #2043 it switches eager Bmi2 to eager Runs and the charge no longer moves.
- **What this evidence does not establish:** Apple/aarch64 hardware was not run on `main`; the portable path was exercised on x86 by forcing it. History for `src/` was fetched shallow-since 2026-10-12 only.

## Second drift in the same session (explained 2026-10-26)

`ic-rootfind-m2` and `ic-rootfind-m3` differ in `phase counts` only, total and phase gae identical. A re-run of the same spec on current code (session `ECBS1h5f4e1659d690`, 224/224 verified, same `spec_sha256` and `plan_sha256`) paired by sequence number shows, for all 64 root-finder records, that the `oracle_setup` phase carries two native counters the committed records lack, `pair_table_kind` and `pair_table_points`, and that nothing else differs: no counter value, and no `adds`, `doubles` or `scalar_mults` in any phase. Cause: commit `c8bd952c4` (2026-10-13, negation-folded PairTable engines) added them about 21 hours after the recording; `full_price.rs` classes both as outcome counts, so they cost nothing. The audit compares the native-count map exactly, so it reports a count mismatch. Evidence and limits (one host, paired re-run, other sessions not enumerated): `research/validation_sweep_20261019/D-known-issues/item1_rootfind_drift_20261026/RESULT.md`. Not the F4 accounting drift. Open owner decision: compare native counters present on one side only as additive, as `field_ops` already is. For `claim-n13` this is moot after the 2026-10-28 re-record, whose records carry both counters; the general audit question stays open for other pre-`c8bd952c4` sessions.

## What would unblock it

- [x] An owner decision: keep the post-#2043 eager definition as the method identity (then `claim-n13` is replayable only by its recording commit and must be re-recorded or marked pre-#2043), or restore the lazy definition (then every session recorded since 2026-10-17 on x86 stops replaying). **Decided 2026-10-28 (owner): eager, version 2, is the method identity.**
- [x] Either way, a field naming the specialisation accounting version, so the audit names a cross-version replay instead of reporting bare counts. Done 2026-10-26 as `Session.accounting_versions` (`INHERITED_F4_ACCOUNTING_VERSION = 2`), not as part of the method identity: the identity hashes `{id, params}`, and a new parameter would change the id of every committed F4 method and fail every committed session's plan check. A session without the field is reported as predating versions.
- [x] Considered and rejected: making the lazy mode charge the eager amount. Lazy mode never builds the child-layout image of a row nobody reads, so the eager charge (words read plus words from the image's first nonzero word) is undefined for it without running the eager rewrite as a shadow, which removes the lazy mode's only use (a performance control), and it would not make `claim-n13` replay (its record holds the lazy charge).
- [x] The owner decision above stands as taken by `main`: the post-#2043 eager definition is version 2. `claim-n13` stays replayable only by its recording commit; re-record it or mark it pre-#2043. Done 2026-10-28, both: `sessions/claim-n13` is marked pre-#2043 in the study README and in `docs/ic/validation/replay-plan.json` (still `expect: drift`), and the same spec was re-recorded on `main` `2b9f3f72f` as `sessions/claim-n13-v2` (`ECBS1h396db1437f4a`, 224/224 verified, `accounting_versions = {inherited_f4_specialise: 2}`, same spec and plan hashes). `verify --replay-all --exit-code` run twice: 168/168 replays reproduced, 0 problems both times; the replay plan lists it `expect: identical`.

## Next discriminating step

- **Action:** replay `claim-n13` with the binary built at `2245b75` on this host (expect 96 problems on the default path, 0 with the item-1 patch, per the committed receipts) to confirm the recording side, then decide which definition is the identity.
- **Recheck trigger:** any change to `rewrite_kept`, `rewrite_step`, `rewrite_with` or `eager_mode` in `src/cryptanalysis/inherited_f4.rs`.

## Resolution and history

2026-10-23: the host-path dependence is closed on `main` (test above). The accounting-version drift is unresolved; recorded evidence under `research/` was not edited.
2026-10-26: accounting versions recorded in new sessions and named by the audit; the charge is pinned by `inherited_f4_accounting_version_pins_the_charge`. The recorded `claim-n13` session still does not replay on `main`; it now fails with its cause stated. Evidence under `research/` was not edited.
2026-10-28: resolved by the owner decision (eager, version 2, is the identity) and a re-record. `claim-n13-v2` replays with 0 problems, twice. Under version 2 the inherited-F4 arms are complete counts, no longer lower bounds: `ic-f4-m3` mean S went from 15.928 to 534.859 and `ic-f4-m2` from 16.642 to 35.758. Rho and root-finder S are unchanged. This is an accounting change, not an improvement or a regression. Report: `research/ecbench_rootfind_engine_20261012/reports/run-20261028T0403Z/`. The original session was not edited. The backend leaderboard still cites it through its pinned-binary plan.
