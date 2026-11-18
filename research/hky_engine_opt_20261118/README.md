# Optimisation round on the hky3 closure engine for the no-1 production closures

Date: 2026-11-18. Worktree `wt/engine-opt` (sparse checkout of isogeny-labs/crypto), local commits only.
Engine: `src/cryptanalysis/hky_fp_tower_fast2.rs` (`--engine hky3`), driven by `examples/pkm_tower_pilot.rs`,
built through `harness/` (a copy of `research/rand_vs_struct_fb_20261118/harness/`, which compiles only the modules
the pilot needs). Every number is `[pcc:unverified]` (no pcc claim filed; no study, leaderboard or
`benchmarks/README.md` row is claimed). Labels: **measured** (this machine, a command of this directory),
**given** (pod records of `research/hky_pod_runs_20261117/`, or the earlier READMEs), **estimate**.

The machine: Intel Xeon @ 2.1 GHz, 4 vCPU, L2 2 MiB/core, L3 260 MiB, 16 GiB, shared with five other workers'
builds and runs: the 1-minute load was **10 to 24 on 4 vCPU** throughout, so every process got about one core.
Wall times here are therefore noisy and roughly single-core; the deterministic counters are the primary evidence,
and CPU seconds of the process are reported beside wall.

SUMMARY_PLACEHOLDER
