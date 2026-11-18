#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
#set par(justify: true)

= Primary N83 D18 K2048 construction and overlap
Run 20261027T1915Z · 27 October 2026

The sorted direct-subgroup D18 prefix produced *2,048 signed-Frobenius
columns and 339,968 distinct subgroup points* on
`EC1N83Ckb0h252dacc102a4`. The matched U K2048 archive has the same
counts. A native full-row audit found *zero shared points and zero shared
orbits*.

#image("factor_base_overlap.svg", width: 100%)

_Figure 1._ Complete subgroup point counts in the U and D K2048 archives;
intersection from `evidence/structural-overlap-native.json`. One fixed
archive per arm, so no sampling interval applies.

== Method and resource envelope

The protocol used the first 2,048 rows of the fully hashed 16,627-row D18
census selection. Exporter source commit:
`a1e6b778dcee8f4ec72f359df7ce3728b302b78a`. Linux arm64 binary SHA-256:
`a0f9b7fe67ffea4816d077b8017e3ed4512cc36fd04689e3eae1257975831a5d`.
The Darwin arm64 host ran the pinned Linux image
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`
with one CPU, 4 GiB memory, zero swap, no network and a 900-second watchdog.
This is a shared-host correctness run, without a CPU wall-time comparison tier.

Export and independent replay were both requested and completed. Historical
guard receipts recorded `PASS` with outer durations 5.594 s and 3.602 s.
A later native-guard rerun also passed at 4.623 s and 3.240 s. These measured
setup durations come from separate shared-host runs; they are not a paired
performance result. The native rerun reproduced compressed object SHA-256
`2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7`
and point-set BLAKE3
`2b5ac37ec3779026278f9debf1a3be6fdfdcff6ff7838f78f1eed4bee2f04b0b`.

== Verification and next gate

The native S3 verifier checked the archived and fresh objects and the S3
HEAD length and AES256 encryption. Native and historical custody and overlap
result fields matched. The original source snapshots and receipts are
retained for provenance; Rust executables are the active verification path.
The raw records are in `research/n83_d18_k2048_20261027/evidence/` and the
protocol and amendment beside this report.

The point-set disjointness makes U and D an equal-size controlled comparison.
Natural relation yield, rank, matrix solving and verified single-target
IC/rho timing remain open. The next experiment pairs natural queries on the
same public points under explicit solver caps. The canonical scoreboard and
progress timeline were checked and left unchanged because no completed
candidate or online-speed result was produced.
