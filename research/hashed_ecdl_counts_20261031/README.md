# Hashed/tensor ECDL vs rho: counted-S-only amendment (2026-10-31)

Superseding, counts-only specs and one `experiment.protocol/v1` for two panels.
The frozen originals are not edited:

| track | superseding spec | frozen original(s) | binary built from |
|---|---|---|---|
| C (tensor IC) | `spec-C-tensor-counts.json` | `docs/ecbench/specs/tensor-ic-vs-rho.json`, `tensor-ic-vs-rho-16.json`, `research/ecdl_tensor_ic_speed_20261030/protocol.json` | origin/main `4df337e64d59bbe0145c204048ab1dc69a43c187` |
| B (native hashed) | `spec-B-hashed-native-counts.json` | `frozen-source/hashed_ecdl_ecbench_20261030/{spec,protocol}.json` (byte-identical copies from `3ae1ac2bd`) | `3ae1ac2bd63d3fb0e86b7e8c0af2b5ec0f1bd09d` (`ic.hashed_native_v1` is not on main) |

Protocol: `protocol.json` (`crypto-hashed-ecdl-counts-20261031`). Hashes: `SOURCES.sha256`.

## What changes and why

Only `measurement` changes. Curves (prime roster 10/12/14/16), targets,
target seeds, arms, arm parameters, algorithm seeds and per-execution budgets
stay the same.

- `isolation_required: L0`. On EKS the run-ecbench L2 gate grades any solve
  under 1 ms as L1 (`record.rs` `MIN_SOLVE_WALL_NS_V3`) and exits 42. rho at
  10-16 bits solves in microseconds (run gh-41008826427-1: 176/192 records L1).
  ecbench documents L0 as enough for operation counts, and it is the lowest
  level the spec accepts. Run with `ecbench run --cpus none`.
- `rounds: 1, warmup: 0`. These are the minimums ecbench accepts (rounds >= 1).
  S is a deterministic function of (workload, algorithm seed) and replays
  exactly, so repeating a seed adds nothing. Warm-up only removes one-time wall
  costs, and counted S does not see those. More rounds would sample more walk
  seeds, which the expected effect does not need.
- The A/A arms share each (workload, round) seed with their partner. In counted
  S they therefore test determinism: rows must be identical. They do not
  measure a noise floor.

No runtime, wall-time or speedup number from these sessions is a result. Each
row's gap to rho is reported as the distance remaining, priced at 0, 0.1 and
1 GAE per unpriced operation on every arm (AGENTS.md §5a). Two arms are ranked
only where the order holds across that band.

**Caveat.** On 64-bit builds lambda <= 64, so root_u = lambda/131072 = 0 and
public inputs take the finite-size fallback. Nothing here exercises the
recursive subexponential kernel. `ic.tensor_contraction_v1` is an exponential
toy (protocol H1).

## Sizing

From the 2026-10-30 session-log sizing (12 bits 3 s, 14 bits 21 s, 16 bits
about 417 s per tensor solve):

- Track C: about 1.0 h. The 16-bit tensor cells take about 3340 s (0.93 h).
  That is under the 2.5 h split threshold, so 16 bits stays in spec C. ecbench
  runs curves in order 10, 12, 14, 16, so the smaller sizes finish first.
- Track B: at most about 1.4 h, assuming every native row uses its full 30 s.

The two tracks can run at the same time on separate CPUs.

## Before running

- Fill and push the resume checkpoint (`scripts/checkpoint.sh init`, `push`,
  `gate`).
- Free disk: on 2026-10-31 the host had about 0.1-0.2 GB free on `/`, which is
  not enough for either release build.
- Build both binaries and record each one's sha256 and `rustc -vV`.
- Check that `ecbench plan` gives 128 (C) and 288 (B) measured executions.
- Conductor is not installed on this host. The hooks warn and let edits through.
