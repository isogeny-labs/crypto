# Full graph-wide IC campaign

Run the full matrix in [the protocol](PROTOCOL.md), preserving every vertex,
conductor, independent one-target workload, factor base and method, including
failure, timeout and unsupported cases. This card is **blocked** and is not a
launched or leased job. Its protocol is a **DRAFT** with a costed design. It
is not frozen.

[Certified input preparation](../../research/full_volcano_graphwide_ic_20261017/README.md)
covers all 24,823 vertices of the three additional completed graphs. A
separate [bounded native survey](../../research/full_volcano_graphwide_ic_20261017/execution/README.md)
uses 2 targets, 1 round, 4,096 trials and a 3 s watchdog. Its latest frozen
checkpoint covers 2,935 vertices and all 51 strata: 46,960 records, of which
30,270 verified, 14,626 exhausted and 2,064 timed out. These do not
discharge this full matrix or the original A--E.

## Why the survey timed out, and why the full matrix should not

The survey's timeouts are all the 3 s child watchdog meeting generic methods
on `three`'s 45-bit subgroup (r = 19,768,889,568,671):

- In the 99-vertex summary all 97 timeouts are on `three`: 50 `bsgs`,
  25 `rho` and 22 `rho-aa`.
- `bsgs` needs ≈ 4.4·10⁶ baby steps, and rho ≈ 3.5·10⁶ GAE.

The survey's IC arms on `three` never timed out; they exhausted. The full
matrix's watchdog is 600 s. The sizing probe's slowest run took 44 s
(`projected-128-subtract`, 10⁶ trials), so no timeouts are predicted. Any
that occur stay in the ledger as censored rows.

## Resource estimate

Measured with the probes in [PROTOCOL.md §10](PROTOCOL.md#10-sizing-probes-run-for-this-draft-disclosure):
release `ecbench`, L0 on the shared host, vertex 0 of each graph, the exact
`full` arm list. These are sizing numbers, not qualified timings.

**Units of work** (*derived*):

- 24,823 vertices × 12 arms × 8 targets × 5 rounds = **11,915,040 measured
  records**.
- Adding the warmup round gives **14,297,808 child runs**: 576 per vertex.

**Per-vertex cost** (run + `verify --replay-all` + the driver's
exhausted-record replay, which re-executes each exhausted record at solve cost):

| graph | vertices | run | audit | exhausted replay | per vertex |
|:--|--:|--:|--:|--:|--:|
| `two` (r = 61,751) | 8,192 | 51.5 s | 19.9 s | 40 × 0.375 s = 15.0 s | **86.4 s** |
| `mixed` (r = 4,253) | 10,070 | 51.0 s | 27.4 s | 40 × 0.391 s = 15.7 s | **94.1 s** |
| `three` (r ≈ 2^44.2), Plan A | 6,561 | 48 × 89.1 s = 4,277 s | ≈ 160 s | 40 × 84.8 s = 3,392 s | **7,829 s = 2.17 h** |
| `three`, Plan B (IC arms 1 round) | 6,561 | 8 × 85.1 + 48 × 4.05 = 875 s | ≈ 160 s | 8 × 84.8 s = 678 s | **1,712 s** |

On `three` one target-round costs 89 s. Of that, 84.8 s is the 9 IC arms
exhausting 10⁶ trials: subtraction at B = 16/32/64/128 takes
5.8/11.3/22.2/43.9 s, and MITM ≈ 0.3 s each.

**Core-hours** (*derived*):

| | `two` | `three` | `mixed` | total |
|:--|--:|--:|--:|--:|
| Plan A (as registered) | 8,192 × 86.4 s = 197 | 6,561 × 7,829 s = 14,268 | 10,070 × 94.1 s = 263 | **≈ 14,700 core-h** |
| Plan B (decision D1) | 197 | 6,561 × 1,712 s = 3,120 | 263 | **≈ 3,600 core-h** |

`three` is 97 % of Plan A. That share buys IC rows that are predicted to
exhaust with certainty (PROTOCOL §7.1). This is the reason for decision D1.

**Peak RAM** (*measured*):

- The largest child is 171 MB (`bsgs` on `three`). The ecbench parent is
  ≤ 81 MB and the auditor ≤ 172 MB.
- One shard slot needs ≤ 0.35 GB, and 32 slots need ≤ 11 GB. Memory is not
  a constraint.

**Disk** (*derived* from measured session sizes). **Is 128 GiB right? Yes**,
with ≈ 16 % headroom; the card records 160 GB to allow 30 %:

- One session is 3.93 MB per vertex (576 records). With the audit and
  exhaustion receipt that is ≈ 4.0 MB per vertex on `two`/`mixed` and
  ≈ 4.1 MB on `three`.
- 18,262 × 4.00 MB + 6,561 × 4.11 MB = **100 GB raw (93 GiB)**.
- The driver copies `ecbench` into every shard: 256 × 25.2 MB = 6.5 GB.
- Packed shard archives add ≈ 12 GB, at the 2,935-vertex archive's ≈ 0.8 KB
  per record.
- Total ≈ **119 GB (110 GiB)**. One shard is ≈ 0.43 GB, so the per-shard
  spool guard is 1 GiB.
- This corpus does **not** need the original A/C/D/E graph storage. That
  storage is a separate requirement of the problem record.

## What fits on this host

The host is a spot instance: Xeon 6975P-C, 16 cores / 32 threads, 124 GB
RAM, 2 TB EBS (1.9 TB free).

- **RAM and disk** fit many times over.
- **CPU is the limit.** Wall-clock estimates:

| slots | Plan A | Plan B |
|:--|--:|--:|
| 32, ideal scaling | 460 h (19 d) | 112 h (4.7 d) |
| 32, assuming SMT gives ≈ 20 effective threads | 736 h (31 d) | 179 h (7.5 d) |
| 16 (sharing the host with the running multi-day experiment) | 920 h (38 d) | 224 h (9.3 d) |

The SMT factor is not measured. The first wave of shards serves as the
qualified resource pilot: execution pauses if the measured throughput or
disk per vertex is off by more than 1.5× (PROTOCOL §3, §9).

**Checkpoint and resume.**

- Each of the 256 shards is its own driver campaign, with its own
  `ledger.sqlite`, writer lock and frozen binary.
- A spot stop loses at most the in-flight vertex of each running shard:
  ≤ 2.2 core-h on `three`, ≤ 95 s elsewhere. The driver marks it
  `interrupted` and retries it into a new attempt directory.
- Each finished shard is packed (§13a) and pushed off-host before its slot
  takes another. A lost volume then costs at most one in-flight shard per
  slot: ≈ 57.5 core-h under Plan A, 14 core-h under Plan B.

Recommendation: **Plan B on 16–32 slots, about 1–1.5 weeks.** Plan A is a
month-scale spot job and should only run if the owner keeps the full rounds
on `three`.

## Before ready

1. Land code gaps G1–G8 ([PROTOCOL §11](PROTOCOL.md#11-code-that-must-land-before-freeze)).
   These are: the full-mode gate, shard selection in `plan`, `--cpus`/`--lock`
   passthrough, registration (D3), split rounds (Plan B), the native
   permutation/bootstrap analysis, a per-shard verifier binary, and the
   resume convention.
2. Settle D1–D3.
3. Re-run the control and three-root pilot gates with the frozen binary.
4. Freeze `PROTOCOL.md`, record its blob, and pin a merged `code.commit`.
   Then set `outputs.verify` and `checkpoint.resume_flag`.

The existing k=2 supersingular MOV teaching implementation is not
finite-field IC for the actual large embedding degrees. Those rows are
recorded as `unsupported` per vertex.

## Acceptance

- A per-vertex ledger matches every planned row with a verified recovery or
  a specific retained failure status.
- Full exclusive cold/online costs, raw relation yields and ranks, and
  min/max/mean distributions are auditable.
- All deterministic records replay, and every recovered scalar is
  independently checked.
- No timing is qualified without the required isolation and controls.
- No generic fallback is called IC.
- No sample is called full graph coverage.
