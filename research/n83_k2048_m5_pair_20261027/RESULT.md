# Matched primary N83 K2048 chained-S3 model panel

The paired U/D K2048 model fits the frozen one-CPU, 4 GiB Linux container
envelope on both public targets. Both arms install the same-size five-summand
SAT encoding: 111,478 variables, 1,140,557 initial clauses and 4,980 XOR
rows. Four natural searches reached the 5,000-conflict cap and returned
`Unknown`, with all guard receipts passing. The independent
[receipt audit](evidence/panel-audit.json) verified the source/binary and input
hashes, exact target pairing, corrected run order, cgroup caps, and retained
status for every cell.
Replaying the audit against the committed evidence reproduced the same audit
bytes; changing one retained target coordinate made the audit reject the
panel before writing an output receipt.

| Cell | Point fixture | Model ms | Root ms | Search s | Conflicts | Status | Peak MiB |
| --- | ---: | ---: | ---: | ---: | ---: | --- | ---: |
| U preflight | 0 | 111.498 | 0 | 0 | 0 | `SKIPPED` | 184.60 |
| D preflight | 0 | 130.525 | 0 | 0 | 0 | `SKIPPED` | 178.45 |
| U search | 0 | 99.356 | 213.567 | 152.399 | 5,000 | `Unknown` | 299.26 |
| D search | 0 | 246.596 | 259.458 | 171.153 | 5,000 | `Unknown` | 282.76 |
| D search | 1 | 127.119 | 213.551 | 156.328 | 5,000 | `Unknown` | 292.89 |
| U search | 1 | 170.493 | 226.996 | 148.972 | 5,000 | `Unknown` | 307.57 |

These are shared-host stage times from one pass per cell, with the order
U0/D0/D1/U1 recorded in [Amendment 1](AMENDMENT_1.md). The U/D same-point
search-time ratios were 0.890 on fixture 0 and 0.953 on fixture 1. Those
ratios describe capped solver work and do not rank completed IC candidates.
There is no verified relation or useful rank in these receipts; the
factor-base selection and single-target online-speedup fields remain null.

The executed binary SHA-256 is
`5123ef012623a0f425f93a91979a918f7e2df369c1309352919a47e9eef52f06`;
the source BLAKE3 embedded in every receipt is
`a70610fb78a193bece7c387a72594dbad0f233b277ec7430fbb59213fd51f12a`.
The Linux image ID is
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.
All six worker JSON receipts and six host guard receipts are in `evidence/`.
The guard enforced one CPU, 4 GiB memory with zero swap, no network and a
300-second watchdog; the SAT worker used a 5,000-conflict and 240-second
deadline. The original [protocol](PROTOCOL.md) and its ordering amendment
are preserved.

The next gate is a versioned relation worker that either lifts and verifies
natural SAT models against the group or improves decomposition search enough
to reach one within a frozen cap. Any selection still needs novel-rank yield,
matrix and target-log recovery, and a calibrated same-point rho comparison
under the Tier 0 or Tier 1 contract.

## Native guarded replay

The [native amendment](NATIVE_AMENDMENT_1.md) supersedes the original Python
guard and receipt-audit execution path while retaining its inputs and
historical receipts. The Rust guard completed six new cells, and the
[independent Rust audit](evidence/panel-native-audit.json) passed on the
committed receipts. Capped search times were U 152.482 s versus D 166.213 s
on fixture 0, and U 159.272 s versus D 153.158 s on fixture 1. Every search
again returned `Unknown` at 5,000 conflicts. The direction of the capped
stage-time difference reverses across the two public points; this local
shared-host panel does not select a base. The
[visual report](reports/run-20261027T2012Z/REPORT.md),
[SVG](reports/run-20261027T2012Z/capped_search.svg), and
[PDF](reports/run-20261027T2012Z/REPORT.pdf) show the native result and its
exact evidence links. The earlier table above remains the historical run.
