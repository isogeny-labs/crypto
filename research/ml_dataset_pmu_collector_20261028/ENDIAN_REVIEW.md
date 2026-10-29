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
