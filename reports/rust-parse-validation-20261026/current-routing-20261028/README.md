# Current routing validation

The exact pre-correction head 8d3b4e191a6ffd3c3b0def250df93af3f5d329b6
ran 12 method tests: 11 passed and the historical-ID test failed. Main had
already separated `ic.hyperelliptic` (the historical genus-2 arm) from
`ic.hyperelliptic_thread`; the older test and parameter-based dispatch no
longer described that registry. The actual frozen genus-2 plan still records
`ic.hyperelliptic` and ECM1ha6ff9c61b9eb, with unchanged defaults.

Resolution now follows the two explicit identifiers. The regression checks the
actual historical ID without changing its expected hash. Both planner and
solver refuse a mismatched workload or implementation entry. All 13 method
controls pass, including actual genus-2, hyperelliptic-thread and prime
workloads. The first new refusal-control setup omitted required thread
parameters and was rejected; that unsuccessful setup log is retained too.

The oversized-log review finding is separately fixed: the original 79,877-byte
compressed log and its 430,976-byte decompressed content match the current
content-addressed pointer and raw checksum manifest exactly. The oversized
blob is absent from the current index and has its generated managed ignore
entry. No original validation log, plan, raw result or scientific status is
rewritten. These software controls do not revalidate historical performance.

The current remote branch and compiler/report repair #2731 were integrated by
ordinary merges, preserving both source histories. The integrated library
Clippy run passes with -D warnings and all 13 method controls pass again.
Sparse checkout initially removed compile-time source/test fixtures during
those merges; their exact tracked patterns are now retained, and both initial
missing-input logs remain alongside the final successful checks. Earlier #2731
all-targets Clippy, adapter, subspace and report-reproduction evidence retains
its original exact-head scope. No scientific input or published report changes.
