# Workstream 3: production-path parallel-rho controls

Status: **implemented but not measurement-ready** `[pcc:unverified]`.

This slice adds a clock-free `check-parallel` command to the standalone GHS
strong-rho driver. It exercises the actual curve transition with:

- domain-separated SHA-256 indexed worker starts;
- one immutable jump/transition identity shared by every worker;
- prefix-sharded distinguished-point tables;
- a bounded coordinator queue that blocks and drains without dropping work;
- independent checks of every DP equation and recovered scalar;
- same- and opposite-sign collision handling inherited from strong rho;
- explicit useful, overshoot, total and per-worker step identities; and
- byte-identical record/transcript replay with no clock fields.

The worker scheduler in `check-parallel` is deterministic round robin. It is a
production-math correctness instrument, not a latency implementation. The
`run-parallel` subcommand therefore exits 3 and says it is unavailable. A
calibration writer is implemented with raw triples, binary/source identity,
placement readback and create-new output semantics, but refuses to run without
a `BENCHREC` reference. Its schema is explicitly development-only, fixes the
isolation label to `ungraded-process-control-only`, and sets
`timing_admissible=false`; environment strings cannot upgrade it to L2.

## Development controls

Run from the repository root on four same-NUMA development CPUs:

```sh
GHS_CPUSET_P1=0 \
GHS_CPUSET_P2=0,1 \
GHS_CPUSET_P4=0,1,2,3 \
GHS_NUMA_NODE=0 \
  research/ghs_frontier_3_7_20261119/ws3_parallel_rho/check.sh
```

The script covers all 66 masked-tail cells, four shard counts, three queue
depths, four stop-delay controls, natural P=1/2/4 recovery twice each, three
exact replay pairs and four fail-closed controls. It writes only to a fresh
temporary directory and reports no timing.

## Remaining implementation obligations

| Control | Current status | Exact remaining gate |
|:--|:--|:--|
| D00 build/selftest | exact pushed-source development pass; source/compiler/binary hashes bound in `DEVELOPMENT_RECEIPT.txt` | cross-materialization reproducibility and an admitted image build remain |
| D01 serial-to-P1 bridge | blocked | define and implement a bridge preserving the legacy serial start stream, or prospectively version the changed start law; full trajectory and count equality are absent |
| D02 indexed starts | partial | SHA-256 derivation and replay exist; the frozen first-1,000,000-start uniqueness corpus for P=1/2/4 is unexecuted |
| D03 shared transition | partial | every record binds one shared transition digest; the 10,000 sealed-state cross-thread/process vector corpus is unexecuted |
| D04 masked tails | partial: exact-commit 66/66 no-crash/accounting cells; every nonempty cell recovers and verifies the injected scalar | scalar-oracle trajectory/count parity and sanitizer/UB remain |
| D05 DP sharding | exact-commit development pass for H=1/2/4/8 | adversarial routing/tamper corpus remains |
| D06 bounded queues | development pass for depths 1/2/64 | real threaded saturation and deadlock/forced-close controls remain |
| D07 stop accounting | development pass for delays 0/1/3/8 | real threaded forced-stop and per-thread overshoot readback remain |
| D08 collision signs | partial | live same/opposite-sign equations are implemented; committed same-y, opposite-y, sterile-denominator and tampered-record fixtures remain |
| D09 known answers | partial | exact-commit seed 1 natural P=1/2/4 replays twice; seeds 2/3 remain |
| D10 failure preservation | partial | unsupported timing, missing BENCHREC, pre-existing calibration output and invalid placement fail closed; forced queue close, worker panic and malformed-DP partial records remain |

Threaded workers, per-thread affinity readback, NUMA memory-policy readback,
total/per-worker CPU accounting, exclusive wall phase intervals and real
contention are not implemented. This is why `run-parallel` remains
unavailable. Calibration has not been executed under benchmark custody, and
no EKS/L2 result, worker-scaling ratio or frontier update follows.
