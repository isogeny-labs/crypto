# Frozen N83 K=16,627 phase-observable complete-witness control

**Status:** frozen before implementation or worker execution, after the capped pinned run in `research/n83_k16627_m4_planted_20261017/` was preserved. Retain all outcomes.

## Question and input

The previous exact-base m=4 pinned arm reached its 1,800-second outer wall cap without a worker or phase receipt. Determine where that wall was spent and whether the complete known x-coordinate chain can be accepted by the same full-width native SAT encoding and independently lifted into a signed-Frobenius relation.

Use the exact primary `GF(2^83)` curve `icv1-f2m83-tm6151469093347-debefd74` and the K=16,627 `public_x_hash`, seed `2026100801` retained object. Require compressed BLAKE3 `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`, point-set BLAKE3 `564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0`, the prior manifest/replay/storage receipts and byte-pinned public source. Do not rebuild or substitute an object.

Choose the first stored representative of signed-orbit columns 0, 1, 2 and 3. Compute `S2=P0+P1`, `S3=S2+P2` and `R=S3+P3` with the native primary group law. Require these sums affine and the four summand x values distinct; otherwise report `INVALID_PLANTED_CONTROL` and stop without replacing the target. Before SAT, independently replay the four stored points against `R`, calculate the signed-orbit row, confirm a fresh wide rank tracker increments to one, and check that the same row fails against `R+G`. This group preflight is reported separately from SAT acceptance.

## Exact model and controls

Build m=4, identity-mask-zero chained-S3 with the full K=16,627 Frobenius-orbit domain, at most 250,000 variables and 6,000,000 domain clauses, without pair ordering. Fix the first four x blocks to the planted point x values, intermediate block 4 to `x(S2)`, intermediate block 5 to `x(S3)` and the target block 6 to `x(R)` using a checked encoder API that names blocks rather than assuming wire offsets. The target block is already fixed by the model constructor; verify its equality without adding contradictory clauses. Freeze a 100,000-conflict SAT cap, one returned model and a 1,800-second outer wall cap.

The positive status requires SAT, original-field S3 verification, exact seven-block x match, an independently checked group lift to `R`, the signed-orbit row and rank-one control, and a rejected `R+G` tamper. `UNKNOWN_search_cap`, `UNKNOWN_wall_cap`, OOM, source mismatch, invalid model, group-lift failure and infrastructure failure remain separate outcomes. A SAT result that fails field replay is an encoder failure; UNSAT despite the known complete witness is `INVALID_UNSAT_PLANTED`, not a mathematical refutation. Do not infer natural-query yield from a planted result.

Write new, append-only phase records after exact-base import, target/group preflight, model/domain installation, seven-block pinning and before SAT entry. Flush and sync each record before continuing, with elapsed wall, current cgroup memory and source/object digests. If the solver returns, record its result, cumulative conflicts and model count before group lift. If it does not return by the outer cap, the last durable phase and the outer container state classify the stop; solver conflicts remain unknown unless a completed checkpoint explicitly records them. Do not backfill a missing phase from another run.

Use a newly created JSONL file at the CLI's `NEW_OUTPUT_JSON` path with suffix `.phases.jsonl`; fail if either output already exists. Phase names are `base_import_done`, `group_preflight_done`, `model_domain_done`, `seven_blocks_pinned`, `sat_entered`, `sat_returned`, `group_lift_done` and `rank_done`. Each record carries a monotonically increasing sequence number. The final result JSON is written only after the relevant completed phases; an outer wall exit with no result JSON is classified from the last durable phase, without treating a missing later phase as completed.

## Envelope and accounting

Build Rust code offline in the pinned arm64 `rust:1.93.1-bookworm` image digest used by the preceding controls. Pass the touched exporter tests and degree-83 source-bound/model controls before worker execution. Run a fresh Linux container with one CPU, a hard 4 GiB cgroup, zero swap, no network and a read-only root. Record binary/source/input hashes, exact commands, stdout/stderr, each phase file, exit/OOM state, peak memory, import, group preflight, model/domain, pin, SAT, field replay, group lift and rank times. Keep the earlier construction/capacity and this run's work separate; do not form a full IC total or speed ratio from mixed runs. The shared-host L0 walls are descriptive. Preserve failed attempts.

If the complete-witness positive arm passes, a separately frozen unpinned search on the same target may run with the prior 5,000-conflict, eight-model and 900-second caps. If the positive arm does not pass, the unpinned arm is not attempted. A subsequent common natural-query corpus for K=16,627, K=2,048 and structured K=2,001 is required to compare useful rank and cold cost. No outcome here selects a best factor base or updates the IC scoreboard.
