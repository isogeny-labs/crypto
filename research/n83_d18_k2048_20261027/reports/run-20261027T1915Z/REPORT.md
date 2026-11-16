# Primary N83 D18 K2048 construction and overlap · run 20261027T1915Z

**Result.** The sorted direct-subgroup D18 prefix produced 2,048
signed-Frobenius columns and 339,968 distinct subgroup points on
`EC1N83Ckb0h252dacc102a4`. The matched U K2048 archive has the same counts.
A native full-row audit found **zero shared points and zero shared orbits**.
The [figure](factor_base_overlap.svg) plots the complete point counts and
intersection; its [data](factor_base_overlap.data.json) comes from the
[native overlap receipt](../../evidence/structural-overlap-native.json).

The frozen [protocol](../../PROTOCOL.md) used the first 2,048 rows of the
fully hashed 16,627-row selected census. The exporter source commit was
`a1e6b778dcee8f4ec72f359df7ce3728b302b78a`, and the Linux arm64 binary
SHA-256 was
`a0f9b7fe67ffea4816d077b8017e3ed4512cc36fd04689e3eae1257975831a5d`.
The host was Darwin arm64 running the pinned Linux container image
`sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`.
The container envelope was one CPU, 4 GiB memory, zero swap, no network and a
900-second watchdog. This construction/replay is a shared-host correctness
run; it does not earn a CPU wall-time comparison tier.

Two phases were requested and completed: export and independent replay.
The historical [export guard](../../evidence/export-guard.json) and
[replay guard](../../evidence/replay-guard.json) both recorded `PASS`; their
outer wall times were 5.594 s and 3.602 s. A subsequent run through the
[native guard](../../evidence/export-native-guard.json) and its
[replay receipt](../../evidence/replay-native-guard.json) also passed, with
outer times 4.623 s and 3.240 s. These are measured setup durations from
separate shared-host runs, not paired performance results. The native rerun
reproduced the compressed object SHA-256
`2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7`
and point-set BLAKE3
`2b5ac37ec3779026278f9debf1a3be6fdfdcff6ff7838f78f1eed4bee2f04b0b`.
The [native manifest](../../evidence/manifest-native-rerun.json) and
[native replay](../../evidence/replay-native-rerun.json) retain that check.

The [native S3 custody verifier](../../evidence/s3-custody-native.json)
checked the archived and freshly downloaded objects and the S3 HEAD length
and AES256 encryption; it matched the earlier custody receipt's result
fields. The native overlap verifier matched the earlier full-row audit's
result fields. The original Python source snapshots and receipts remain in
`historical/` and `evidence/` for provenance; the native executables are the
active verification path, as described in the
[native amendment](../../NATIVE_AMENDMENT_1.md).

The chart has no sampling interval: every row in each fixed archive was
examined. It establishes a controlled change of point set at equal base
size. Natural relation yield, useful rank, matrix solving, and a verified
single-target IC/rho pair are still unmeasured. The canonical
`docs/index-calculus-scoreboard.html` and `docs/ic/progress-timeline.json`
were checked and left unchanged because this run supplies no completed
candidate or online-speed result. The next test is the paired natural-query
panel with same public points and explicit solver caps.

The [PDF version](REPORT.pdf) contains this report's result and figure.
