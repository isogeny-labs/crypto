# Native guard and receipt-audit amendment

The original six-cell panel and its Python guard/auditor receipts remain
historical evidence. Their source snapshots are retained as non-executable
text under `historical/`. The active experiment harness is
`examples/n83_k2048_m5_guard.rs`; the independent receipt auditor is
`examples/n83_k2048_m5_audit.rs`. This amendment supersedes the original
Python execution path without changing the frozen curve, factor bases,
public targets, model binary, solver limits, or corrected U0/D0/D1/U1 order.

The native guard pins Linux arm64 model binary SHA-256
`5123ef012623a0f425f93a91979a918f7e2df369c1309352919a47e9eef52f06`,
both compressed object SHA-256 values, the public corpus SHA-256, and the
Linux image ID. It verifies the actual Docker CPU, memory, swap, network and
read-only-root configuration, records cgroup memory peak, OOM and exit status,
and applies the 300-second container and host watchdog. The model worker
itself retains its 5,000-conflict and 240-second deadline.

The native replay completed both preflights and all four natural search
cells. The [native audit](evidence/panel-native-audit.json) checked every
worker and guard receipt, source/binary and input hashes, same-point pairing,
run order, model dimensions, and status preservation. It passed again on the
committed copies with identical output bytes. The
[negative controls](evidence/native-negative-controls.json) rejected a
changed compressed object before Docker allocation and a changed public
target coordinate before writing an audit result.

The [source-linked report](reports/run-20261027T2012Z/REPORT.md),
[figure](reports/run-20261027T2012Z/capped_search.svg), and
[PDF](reports/run-20261027T2012Z/REPORT.pdf) present the native measurements.
The historical Python stage times are retained with their original receipts;
the native panel is the current bounded comparison. Both are shared-host
diagnostics, and neither supplies a verified natural group relation or a
completed single-target IC/rho pair.
