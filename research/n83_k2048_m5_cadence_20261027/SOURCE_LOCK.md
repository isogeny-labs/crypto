# Source and host lock before the eight natural searches

The [protocol](PROTOCOL.md) was written before compilation and any search.
The worker compiled successfully with
`cargo build --release --locked --example n83_k2048_m5_cadence`.
Its source file SHA-256 is
`6402da58e23b2b3e8badf7f8794c615d227b52b9550e3cd639ffd258eba77f03`;
the Darwin arm64 release binary SHA-256 is
`0fdbb812d25c3bb9ee78f1f8740ee05e7db1d05b52af8e1a80cf59da82d55852`.
The independent receipt audit rejects a binary or source change against
these hashes.

Host `uname -smr`: `Darwin 25.6.0 arm64`; CPU brand: `Apple M4 Pro`.
Public corpus SHA-256 readback:
`760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`.
The archived base-object SHA-256 values in the protocol are checked by the
worker. One worker process will run at a time under the same local host;
there is no isolation-tier claim for the timing columns.

The worker is a versioned copy of the native K2048 m5 model producer with
the experiment's sole algorithm setting exposed as `cadence=1|8`, and with
instrumented solver counters. It replaces the original Linux cgroup check
with explicit local-host metadata for this L0 diagnostic. The original
worker, natural-panel records, and planted controls remain unchanged.
