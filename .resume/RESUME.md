# Resume: replication of the binary-field k = 4 crossover (red-team plan, #2628)

- **Goal:** execute `research/gaudry_k4_char2_redteam_20261026/REPORT.md` §3 (merged in #2628) in order: (1) full replay of the claim's clean sessions; (2) build full pricing for f2m4 / gaudry-d4-bin, a fully priced `rho-strong` (negation, distinguished points, batched inversion, canonicalisation charged) and a measured binary-field unit price, behind new method ids; (3) freeze the protocol, then 8 fresh targets per side at m = 14–17, both units, §5a band; (4) m = 18–20 with large primes only if the crossover survives.
- **Branch:** `claude/k4-char2-replication-20261026`, based on the claim commit `e403626cb` (Chase 1's code; do not touch `claude/k4-char2-crossover-*`).
- **Study dir:** `research/gaudry_k4_char2_replication_20261026/`. `claim-snapshot/` is a read-only copy of Chase 1's sessions at `b94e81c8a`; only clean sessions count (d-ladder, e-m15, e-m17-rho, e-m17b-rho, f-lp), never `*.attemptN-killed`.
- **Step 1 runner:** `replay.sh` (resumable; receipts in `audits/replay-*.json`, status in `logs/replay.status`). Uses `target/release/ecbench` built from `e403626cb`.
- **New builds:** use `CARGO_TARGET_DIR=target-repl` so the replay binary is never overwritten mid-run.
- **Conductor:** T-3 (local control plane).
- **S3 checkpoint:** not possible in this container (no upload token, no AWS CLI); this file and every finished session are committed and pushed to git instead.
- **Session doing the work:** session_01JZkty38GW5FcgAcudQ45ph (parent of the chase sessions).

## Status 2026-10-27 (protocol frozen)

- Step 1 done: d-ladder replay 18/18 reproduced. Step 2 done after four review rounds (`REVIEW-1..3.md`, round-4 verify in the session log of the parent session).
- Step 3: `PROTOCOL.md`, `specs/` (96, from `make_specs.py`), `jobs.txt` and `run_queue.sh` are frozen in the commit that adds this note. To resume after a restart: `setsid nohup bash research/gaudry_k4_char2_replication_20261026/run_queue.sh &`. It skips `done/` jobs and renames interrupted sessions `.attemptN-killed`.
- Build the binary first if `target-repl/release/ecbench` is missing: `CARGO_TARGET_DIR=target-repl cargo build --release -j2 --bin ecbench`.
