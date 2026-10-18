# Stopped survey checkpoint and publication staging validation

The frozen native survey advanced from 5,533 to 6,137 audited vertices before
a clean vertex-boundary pause for publication. PID 3298 finished with native
reason `stop_requested`; no new in-flight attempt was interrupted by this pause.
Earlier interrupted attempts remain retained. The actual stop receipt, native
inventory, full native summary and delivery receipts determine the final counts.

The packaging change adds `--link-sessions`. It hard-links only files beneath
`campaign/sessions/` from an immutable native snapshot into a fresh stage on
the same filesystem. Ledger/worker/other metadata are copied independently.
No fallback silently duplicates the payload on a cross-filesystem error.
Symlinks, special files, resumed/live snapshot markers, recursive destinations
and existing stages are rejected. The stage and its source must remain untouched
until the final archive is closed and verified. All archive members are stored
as regular files, even when several source paths share an inode.

Publication-file validation covers the actual packager CLI in both
modes, every fixture manifest entry, identical archive bytes, independent ledger
metadata, self-contained archive members and unsafe-input rejection. Fixtures
are deliberately synthetic file-handling data, never scientific observations.
The actual stopped campaign must separately pass native inventory and summary.

This saves one redundant session-payload copy during staging. It does not
eliminate the native snapshot, prove filesystem allocation savings, or measure
a speedup. Python only handles files/presentation; native Rust remains the
measurement auditor and aggregator. Frozen mathematical source, solver/driver
binaries, queue, acceptance gates and child budgets are unchanged.

The original full A/C/D/E graphs, full benchmark matrix, independent arithmetic,
transported-target comparisons and large-k finite-field IC remain open.
No S3 runtime identity or external larger working volume/worker is configured;
no paid cloud resources were provisioned. Existing Drive deliveries remain
immutable. Exact current download IDs and local hashes belong in delivery.json.

Raw publication receipts are archived in runs.tar.xz with its sidecar SHA-256.
Extract with `tar -xJf runs.tar.xz`; verify `sha256sum -c runs.tar.xz.sha256`
and then `(cd runs && sha256sum -c MANIFEST.sha256)`. These receipts do not
replace the external scientific archive or establish independent solver replay.

Native inventory passed: **98,192 accepted records**, comprising **63,182
verified / 30,553 exhausted / 4,457 timeout**. All 51 strata have samples;
6,137 / 24,823 vertices are audited. The per-graph coverage is 2,056 mixed,
2,038 three and 2,043 two. These counts are native output, not a fresh solver
replay or full-vertex distribution. The summary/CSV include all recorded arms.

The first packaging attempt rejected reader caches created by the native
read-only summarizer: a zero-byte WAL and a 32,768-byte SHM. That failure is
retained. The staging helper now omits only these generated direct-campaign
reader caches after requiring an empty WAL; nonempty WAL, orphaned SHM and
writer markers remain rejected. Seven file-handling tests pass. The original
checkpoint and raw session files are untouched. A separate failed test-runner
path invocation is retained alongside the successful corrected invocation.

Reproduce file-handling checks from the repository checkout:

```sh
cd research/full_volcano_graphwide_ic_20261017/execution
python -B -m unittest -v test_checkpoint_staging.py
```

The supplied test source is packaging provenance; the CLI integration fixture
uses the repository checkout and Git source paths, not a standalone extracted
`source/` folder. It performs no scientific measurement.

## Verified immutable downloads

- [Complete checkpoint archive](https://drive.google.com/file/d/1LSjoOz1tF5Jljna4pbhNunIA5MD0lA3J/view): 61,650,744 bytes; SHA-256 `1fc05eba3d271be69af41880e96634e151f8ed83b3398e2e06853c0a7581efc8`.
- [Native all-arm statistics CSV](https://drive.google.com/file/d/1usthx0SKcVgbyHaOangR4WLH_skTs6Zz/view): min/max/mean, actual n, phase/native counters and relation yields.
- [Inspected three-page report](https://drive.google.com/file/d/1SzqGABcLpJHaWMgb4i0UZhDeBhq029BY/view).
- [Complete three-class viewer with frozen 6,137-vertex overlay](https://drive.google.com/file/d/1w9qQP0rgtC40C-ioTnbEW9I91CTJyftm/view).

Fresh extraction, all 61,466 manifest entries, exact file population and regular
archive member types pass. Repacking extracted regular files gives identical
outer archive bytes. Local hashes and remote name/byte-count/parent readbacks
are retained in delivery.json; remote content hashing is not claimed. Production
staging hard-linked 30,695 session files containing 770,860,614 payload bytes,
avoiding that redundant payload copy. This is a logical file-byte count, not
measured physical allocation or a runtime result. Native totals are 156,623,072
source trials / 149,275,746 actual decomposition calls / 3,181,542 relations.
