# Native point-key and allocation accounting

The [shared driver](../../../examples/native_point_hash_study/driver.rs) now
observes the work that native root/add/meter counters omit: key construction,
logical table lookups, full-key Hash/Eq/Ord, hasher creation/finish/feed bytes,
and current-thread allocator requests/layout bytes. The analyzer reports 21
unpriced diagnostics and the direct Prepared/Ordered, Ordered/Hashed and
Hashed/Fingerprint contrasts alongside five Current comparisons. This is an
instrument improvement, with no runtime result. [pcc:unverified]

The [ordinary example](../../../examples/native_point_hash_study.rs) retains
the default allocator and plain keys in clocks mode. The separate
[accounting example](../../../examples/native_point_hash_accounting.rs) wraps
System for counts only and refuses clocks before native preparation. Both use
the same driver and lockfile; run headers pin each binary separately. Raw and
counted keys delegate the same full-point equality/order/hash. No hash match
removes exact point equality. [pcc:unverified]

Accounting spans native cold preparation through query attempts and original
arithmetic witness replay. Reporter vectors are allocated before the interval;
record serialization, the separate word checker and teardown are outside.
Allocation counts describe observed requests, which the optimizer may elide or
combine; requested layout bytes are not live/usable bytes, arenas or RSS.
Internal SwissTable probe groups and peak live bytes are explicitly unknown.
Missing allocation metrics never become zero or a selected subset of seeds.
Complete absolute totals remain when zero denominators prevent a ratio.

## Development qualification

The exact [current manifest](source.sha256) has 22 named inputs. Final local
release controls report 19 passes in the ordinary example and 20 in accounting
(19 shared, one allocator control), with strict release clippy, build and format
checks. Each binary completed the fixed 76-child F17 counts-only profile. The
native parity command compares 76 records and 292 preparation/prefix/final
snapshots: answers, base sizes and native counters match historical v1; key
counts match between the two new binaries; accounting requests are present and
ordinary allocation values remain unknown. The accounting binary's clock
request was refused. These are producer software checks, not measurement
counter authentication, signed PCC standing or independent audit.
[pcc:unverified]

Commands, source/binary hashes, flow, limitations and log hashes are in
[qualification.json](qualification.json). All retained raw development files
and three exact source stages are losslessly packaged:

| Archive | Contents |
|---|---|
| [support.tar.xz](support.tar.xz) | Sources, build/test/lint/validation logs, failures, analyses, command records and native parity receipt |
| [initial-runs.tar.xz](initial-runs.tar.xz) | Initial ordinary/accounting F17 child bodies |
| [final-runs.tar.xz](final-runs.tar.xz) | Final exact-source ordinary/accounting F17 child bodies |

Each archive is below 64 KiB. [archive-index.json](archive-index.json) pins the
archive bytes. Extract to a new directory and run `sha256sum -c SHA256SUMS`
inside each top-level directory. The final thin shell invocation is retained
as `support/qualify-final.sh`. Historical records come from the unchanged
[pilot-v1 archive](../pilot-v1/orchestration-development.tar.xz). Old source
manifests remain tied to their documented commits, rather than being rewritten.
[pcc:unverified]

The initial 240-second development deadline was refused before any child; the
corrected command uses the profile's 30-second ceiling. Strict clippy rejected
`map_or`; the final source uses a Rust 1.78-compatible `is_some_and` predicate,
without suppressions. Both raw failures and their source identities remain.
Two temporary metadata/check invocations were not redirected; their provenance
and the discarded Python parity check are disclosed in
`support/development-failures.json`. No conclusion relies on that Python check.
[pcc:unverified]

## Pilot status

The [v3 protocol draft](../pilot-v1/protocol-v3.json) registers this change before
production data; v1/v2 and all original frozen evidence remain unchanged. The
32-context/2368-attempt recipe and logical plan hash are unchanged. The readable
pinned canonical validator reports zero structural findings, but fetched current
canonical blobs are VLT1 encoded here; current canonical access and freeze remain
unresolved. No 12/16-bit decomposition batch, native clocks-mode batch, cloud
build or scientific job was run. [pcc:unverified]

The scoped [USD 10 budget override](../pilot-v1/budget-override-20261108.json)
remains active. Execution still needs authenticated native access, fresh current-
source S3 checkpoint/readback and native image/L2/host/resource admission. Full
field pricing, external references, masked canonical export, runtime/power and
independent audit remain pending. Next implement native host/resource/export
integration and complete the field inventory; do not replace these with local
timings or the unrelated deployed ECBench fixture. Auto-merge stays off for the
separate protected ledger/journal conflict on PR 2984.

Primary sources reviewed for this change are Rust's official
[GlobalAlloc](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html),
[Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html) and
[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
documentation. They motivate nonallocating/no-unwind callbacks, delegating the
same key hash/equality, pinning the toolchain and leaving internal probes
unknown. The established point-table literature remains in the original plan.
