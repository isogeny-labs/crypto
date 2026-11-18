# Native PMU endianness correction

2026-10-29. PR2801 review comment4404274309 identifies that native-endian
serialization of low-bit masks is not the C bitfield layout on big-endian
AArch64. The existing architecture check admitted that target incorrectly.

The Linux UAPI `perf_event_attr` declares the flags as a C bitfield, unlike
the ordinary integer fields. The source was rechecked before editing:
[Linux perf_event.h](https://github.com/torvalds/linux/blob/master/include/uapi/linux/perf_event.h).
The existing design and mock attribute tests were also reviewed.

The native implementation now requires little-endian 64-bit Linux x86_64 or
AArch64 in all four complementary cfg gates. Unsupported targets report
`UnsupportedPlatform` before opening a counter; no partial endian conversion
is attempted. Little-endian request bytes and legacy counters are unchanged.
The regression queries the actual backend eligibility without opening counters
and, when unsupported, checks that the native early-exit report has no lifecycle
calls, kernel IDs, raw readings, selected attempt, or fallback attempt.

Validation is pending for this correction. The existing final validation
receipt remains immutable and describes the earlier source. No big-endian
hardware run, instruction measurement, dataset generation or cloud job is
authorized by this correction.


## Completed validation, 2026-10-29 03:32 UTC

The preceding pending status is superseded by terminal gates against clean
source `4a57e45ec74ad878a4ed0ce7ccd68abb4f4d7647`: 26 focused tests passed,
strict release Clippy passed, and the complete immutable library executable
passed **5209 tests, 0 failed, 123 ignored**, with eight test threads and no
filter (496.05 seconds, exit 0). The executable SHA-256 is
`e68264d34cac14ae8cf7188e6f29c54988128db92a259fce7ee10d03f7437f5f`.
The separate source review found no additional actionable issue.

The initial full run remains recorded: 5203 passed, six failed, 123 ignored.
Five failures could not start the absent C++ compiler on the migrated host.
Installing gcc-c++ restored all six CPU-emulation device tests. The remaining
unchanged GPU capsule lock test passed in isolation; the cause of its first
concurrent lock failure remains unconfirmed. The subsequent full run passed
without changing or excluding those tests. This validation did not exercise
big-endian hardware or establish an independent counter measurement.

`ENDIAN_VALIDATION_RECEIPTS.json` records three immutable canonical S3 objects:
source/focused/lint (37,403,961 bytes), original failures/host repair (92,815
bytes), and the final full-library gate (90,229 bytes). Each passed conditional
PUT, fresh ETag-bound GET, byte/length/hash and contained-file verification.
The source packet also passed restored bundle-head and executable comparison.
Earlier receipts remain unchanged. Hosted checks for the updated PR are a
separate gate; this record does not assert a main merge or dataset generation.
