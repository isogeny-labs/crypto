---
title: "rootfind on the GPU: the m = 3 pair table built on the device, and a pooled multi-threaded CPU sweep"
date: 2026-11-15
---

# rootfind on the GPU: the m = 3 pair table on the device, and a pooled multi-threaded CPU sweep

User request (2026-11-15), "do these", on the open items of PR #4276: build the `m = 3` pair table on
the device; a tuned multi-threaded CPU sweep; the validator check of
`research/ic_gpu_rootfind_big_20261115`.

**Standing.** `[pcc:unverified]`: one A100 pod, measured by the session that wrote the code; no
validator yet for this report. Counted S is unchanged by design and measured unchanged in every pair;
the differences are wall time. Every record is L0 (`--cpus none --allow-busy`); the CPU arm's A/A copy
differs from it by up to 14% in solve wall. This pod's host (AMD EPYC 7543) differs from round 3's
(Intel Xeon 6240R), so wall times are compared within this session only. No scoreboard or leaderboard
row is added.

![Solve wall per arm, [pcc:unverified]](fig_pairs.svg)

## Validator check of round 3

`research/ic_gpu_rootfind_big_20261115/VALIDATION_REVIEW.md`: **validated, scoped** (all 240
certificates re-verified, the four curves' orders proved, the 51 records that need no GPU replayed on
another hardware class). Its eight corrections are applied to that report. The one that changed this
round's plan: creating threads per batch cost at most 10% of a batch, so the multi-threaded arm's
weakness had another cause (below). [pcc:unverified]

## What was built (pushed before the run; measured binary `5142615e5d`)

- **The pair table on the device** (`rf64_session_create_pairs`, `RF_LOOKUP_PAIRS_DEVICE`): one
  thread per pair of abscissae computes `x(P_ai ± P_bi)` in the CPU build's order (`rf_pair_x`); a
  device hash keeps, per abscissa, the least build index (`atomicMin`), which is the pair the CPU
  map's first insert keeps; the survivors are compacted, hashed for the probe, and each hit is
  reported with its build index.
- **`rootfind-subtract-gpu:m=3,probe_build=pairs`**: no CPU table at all. The build is charged
  exactly as `PairTable::build_negation_folded` charges it: `a(a+1)` group additions over `a`
  abscissae, `a(a−1)` general, `a` doublings and `a` identities, each case priced by one real
  addition on the ledger, plus `a²` insert attempts and `a` index reads. Each reported position is
  confirmed with `PairTable::probe`'s logic (one lookup, one charged addition, a compare) on the
  device's pair. The rare rows with `x_R` in the base search the first pair by enumeration.
- **The multi-threaded arm** (`backend=emulate`) runs on a persistent thread pool sized by the
  process's CPU budget (`RF_EMU_THREADS`, else the CPU count capped by the cgroup quota). This pod
  shows 128 threads but its cgroup quota is 13.6 CPUs (`cgroup-cpu.txt`: 1360000/100000), so the pool
  ran 14 threads; round 3's emulation ran one thread per visible host thread.
- **Checks before the run** [pcc:unverified]: 7 tests (host worker) equal to `rootfind-subtract`,
  among them the pairs build over whole pipelines and every degenerate row compared ledger for ledger;
  a worker that reports the next build index (a wrong pair) fails the pairs test (7 299 targets
  against 42). The CUDA build of the new kernels was first compiled on the pod and passed the smoke
  session.

## Run

One A100 80GB PCIe pod (`y8dxu6qiiykumx`, AMD EPYC 7543, 128 visible threads, quota 13.6 CPUs),
CUDA 12.8, 07:05Z–07:55Z 2026-11-15, about $1.75, deleted. Specs frozen and pushed before the run
(`specs/smoke.json`, `specs/rf3pairs.json` on the round-3 curves of 34–40 bits with \|F_x\| 2048 and
4096, `specs/rf2mt2.json` at 28–32 bits); one daemon (context 0.280 s); the job required CUDA in the
daemon and the worker and recorded the cgroup quota. Raw `raw/out_rpair.tgz` (sha256 `852cbf70…`);
tables `RESULTS.md`, `results.json`.

## Results [pcc:unverified]

- **Identity:** smoke 36/36, rf3pairs 176/176, rf2mt2 48/48 verified; `ecbench verify --replay-all`
  reproduced all 260. 156 checks of every other IC arm against the CPU arm (by workload and round):
  equal S, total GAE, recovered logarithm, relations, targets and relation-phase additions, 0
  mismatches. The device-built pair table's charge equals the CPU build's in every record. [pcc:unverified]

### `m = 3` with the pair table on the device (median of 4)

| log2 r | \|F_x\| | S / rho | CPU solve | multi-thread | GPU, CPU table | GPU, device table | vs CPU | vs multi-thread | set-up CPU → device table |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 33.4 | 2048 | 140 | 1.06 s | 1.22 s | 0.788 s | 0.246 s | 4.3× | 5.0× | 0.511 → 0.034 s |
| 35.2 | 2048 | 166 | 1.86 s | 1.29 s | 1.02 s | 0.260 s | 7.2× | 5.0× | 0.496 → 0.028 s |
| 37.4 | 2048 | 302 | 6.25 s | 1.87 s | 0.916 s | 0.295 s | 21.2× | 6.3× | 0.501 → 0.018 s |
| 39.3 | 2048 | 569 | 21.1 s | 4.00 s | 0.949 s | **0.368 s** | **57.3×** | 10.9× | 0.505 → 0.021 s |
| 33.4 | 4096 | 242 | 5.07 s | 5.27 s | 4.75 s | 0.706 s | 7.2× | 7.5× | 4.21 → 0.124 s |
| 35.2 | 4096 | 142 | 5.41 s | 6.35 s | 5.87 s | 1.36 s | 4.0× | 4.7× | 2.94 → 0.116 s |
| 37.4 | 4096 | 172 | 11.3 s | 8.43 s | 5.05 s | 1.66 s | 6.9× | 5.1× | 3.81 → 0.109 s |
| 39.3 | 4096 | 298 | 24.8 s | 7.38 s | 5.78 s | 1.49 s | 16.7× | 5.0× | 2.92 → 0.122 s |

- **The device table removes the set-up.** Against the CPU arm's 0.50–0.51 s (2048) and 2.9–4.2 s
  (4096), the oracle set-up is 0.018–0.034 s and 0.11–0.12 s, and the worker creates the session in a
  median 0.034 s (daemon timing lines). In the same session the device-table arm is 2.6–6.7× faster
  than the GPU arm that still builds the table on the CPU. [pcc:unverified]
- **What bounds the solve now:** at 2048, the relation phase (0.02–0.16 s) and linear algebra
  (0.18–0.22 s); at 4096, linear algebra (0.53–1.50 s, the same code on every arm).
- **Distance to rho:** S is 140–569× the matched rho's mean S at w = 0, unchanged by the device. In
  wall time the device-table arm is 3–99× slower than rho's median solve (7–108 ms); at 39.3 bits and
  2048 it is 3.4× (0.368 s against 0.108 s), the closest any round has come.

### `m = 2`: the pooled multi-threaded arm against the GPU (\|F_x\| = 4096, median of 4)

| log2 r | S / rho | CPU solve | multi-thread (14 threads) | GPU | multi-thread vs CPU | GPU vs multi-thread |
|--:|--:|--:|--:|--:|--:|--:|
| 27.5 | 5 303 | 2.33 s | 0.313 s | 0.101 s | 7.5× | 3.1× |
| 30.0 | 19 501 | 12.9 s | 1.30 s | 0.158 s | 9.9× | 8.3× |
| 31.3 | 22 023 | 34.0 s | 3.24 s | 0.282 s | 10.5× | 11.5× |

- With the pool sized to the 14 CPUs the pod may use, the multi-threaded arm reaches 7.5–10.5× one
  thread at `m = 2` (round 3, a different host and 96 threads on an unrecorded quota: about 5×). It is
  still the GPU kernel's per-thread code run on the CPU, not a CPU-optimised sweep; a vectorised CPU
  implementation could do better, and that is not measured.
- At `m = 3` the multi-threaded arm is slower than one thread at the small sizes (0.85–0.96× at 33–35
  bits, 4096): its set-up builds a host hash of the 4–17M-entry probe set (3.9–6.0 s) on top of the
  CPU pair table. Its relation phase is 2.5–9.6× one thread where that phase is large.

## Not done / next

- Linear algebra now dominates the `m = 3` solve at \|F_x\| = 4096 (and is the same on every arm).
- A CPU-optimised (vectorised) multi-threaded sweep, for the strongest CPU baseline.
- A validator check of this report.
