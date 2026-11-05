# Offline replay verification

This is a post-data exact verification of saved CAS output, authorized by the
parent after the scientific panels finished. It is not a replacement for the
original capped extractor and does not add a scientific run.

## Source and environment

The original instrument source/runtime revisions were
`de5a3cfdc6bfd4744a15fa301a1a0b21c872e1b1` and
`ff9d54f3ad4030dbe4e8426a02a2c5e5fa806b2c` respectively. The immutable
v1 source SHA-256 remains
`e201eda231b41820b1733eefd725841036075aadceb9d4cada92926893025842`.
The companion's `build.rs` enforces this hash before including its helpers.

Builds used Rust/Cargo 1.94.1, `CARGO_HOME=/workspace/n83-cargo` and
`RUSTUP_HOME=/workspace/n83-rustup`. The checked-in Cargo lockfile freezes
dependencies. Native build output under `target/` is ignored.

## Executed command

From `/workspace/crypto`:

```sh
set -o noclobber
timeout --signal=TERM --kill-after=5s 300s bash -c '
  ulimit -v 2097152
  exec research/prime_factor_base_degree_20261031/parallel_20261031/f4/replay_v2/target/release/secant-f4-offline-replay-v2 run /workspace/scratch/secant-parallel-runs/run-20261031T201044Z /workspace/scratch/secant-parallel-runs/replay-v2-20261031T201044Z 240
' > /workspace/scratch/secant-parallel-runs/replay-v2-20261031T201044Z.stdout 2> /workspace/scratch/secant-parallel-runs/replay-v2-20261031T201044Z.stderr
```

One native process, no worker threads, 2 GiB address-space cap, internal
240-second cap, outer 300-second cap. The command exited 0. The companion
manifest reports 14,220/14,220 verified, zero failed/unattempted, and all 26
v1 failed/capped cases verified by the companion. Original panel counts
reconcile 14,220 requested/attempted and zero unattempted. New CAS calls: 0;
coordinate-cube searches: 0. No separate measured elapsed time is claimed.

## Exact checks and scope

Each replay checks canonical input reconstruction, complete signed-group
reference generation, original-generator normal forms, every Buchberger
S-pair, and every reference root in both input and exported basis. Producer
references and available v1 root lists must agree with the independent
reference. Input/output/trace linkage hashes must match the original result.
The two-inclusion argument is detailed in [README.md](README.md).

The independent reviewer approved the exact
proof route before execution, including diagonal/tangent/flex handling,
complete branch orientation and the distinction between basis verification
and operational extraction. The separate review record is maintained by the
parent at [REPLAY_V2_REVIEW.json](../../review/REPLAY_V2_REVIEW.json).
The subsequent independent inventory/hash review checked all 14,220 unique
companions, 71,110 manifest/companion/raw links, 14,194 available v1 root
lists, and all 14,220 trace/input-degree metrics with zero errors. All 26
original caps were preserved.

The verifier reuses hash-locked v1 field/model/group primitives. It is an
alternate exact verification route, not an independent implementation of
those primitives. It certifies finite-field root-set equality; equality of
ideals or multiplicities is not claimed. Group enumeration is verification
work and is not credited as algebraic relation discovery.

The original 26 root-extraction caps and their errors remain intact. Neither
the discovery cost nor an end-to-end solver result is promoted by this replay.

## Development checks

Four artificial tests, strict Clippy, formatting, release build and
`git diff --check` passed. The retained development log records final
source checks only; it is separate from all curve-panel data. During initial
development a large JSON literal exceeded Rust's default macro recursion
limit, then strict Clippy found an unused import. Both were fixed without
changing the frozen source or weakening algebraic checks.
