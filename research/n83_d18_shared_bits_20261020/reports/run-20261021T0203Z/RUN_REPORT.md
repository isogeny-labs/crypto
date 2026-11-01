# D18 construction and affected-example validation

**Observed: 23 example tests pass; all six full-size model constructions pass.
The full release-library suite and bounded Linux construction are pending.**

The native validation sequence started at 2026-10-21 02:03:11 UTC on the
shared Apple arm64 development host, with `rustc`/`cargo` 1.93.1. It uses the
separate T7 build and temporary directories after the earlier shared-SSD
space failure. Isolation is L0. No comparative solver timing is reported.

The five executed source files match commit
`78bd2cf16` (full SHA in the archived `matching-source-commit.txt`). The run
started with one uncommitted tuple-type fix relative to parent `5f76ce952`;
the same bytes were subsequently committed in `4653d9f19` and retained by
the merge. Source-file hashes and the compiled executable hashes are in the
archive. This is a validation/construction run, not a paired performance card.

## Workload and observations

Input: the pinned 16,627-class selected D18 CSV, SHA-256
`534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4`.
Curve: `icv1-f2m83-tm6151469093347-debefd74`. Both encodings use the same
public target-x coordinate at corpus index zero. No solver search is started.

| m | Original clauses | Shared-bit clauses | Variables, both | XOR rows, both |
|---|---:|---:|---:|---:|
| 2 | 2,926,660 | 732,028 | 90,258 | 1,743 |
| 3 | 4,420,949 | 1,129,001 | 145,928 | 2,822 |
| 4 | 5,915,238 | 1,525,974 | 201,598 | 3,901 |

Each entry is an actual count from one deterministic model construction.
Every preflight/actual count matches; the two policies have equal selector
and phase wire vectors. The common-bit inventory matches the independent
native inventory exactly: 66 common zero bits. There is no sampling interval.

![Actual full-size clause counts](../../counts.svg)

The construction tool issued count-identity certificate
`IDC1hbc6bfea0a569542b` on 32 points and accepted 32 extra checker points.
The separate Lean natural-number theorem is documented in the
[proof map](../../PROOF_MAP.md); neither proof alone certifies the Rust solver.

All 23 affected-example tests passed: 3 construction/control-tool tests,
5 full-panel audit tests, 2 card tests, 7 worker tests, 1 reuse-accounting test,
3 independent replay tests and 2 XOR-cache audit tests. The build produced
the three release executables. The native library's exhaustive encoding tests
belong to the still-pending complete release-library suite.

## Failures and remaining requirements

The first affected-example build failed with Rust E0282 at the construction
count tuple. The explicit `(u32, usize, usize)` annotation fixed it; the
original failure log and exit 101 are retained. Earlier space/interruption
failures remain in [validation v1](../../VALIDATION.md).

Local model construction did not use a hard container memory limit. It
establishes actual structural sizes; it does not close the protocol's 4 GiB
Linux gate. The paired 22 controls/replays and 18 natural attempts have not
started. Useful natural rank, full cold/reused IC cost, matched rho and
base selection remain open. Candidate and selected-best-runtime fields are null.

The [two-page report](../../report.pdf), [editable figure source](../../counts.typ),
[receipt](../../construction.json) and [native source](../../../../examples/n83_d18_shared_bits.rs)
describe the same check. Canonical performance dashboards and benchmark rows
are unchanged because no runtime comparison or IC pipeline ranking was measured.

## Preserved receipts

`validation-v2.tar.xz` contains 20 files: exact commands, exits, source and binary
hashes, the constructed receipt, host metadata captured after construction,
the passing example/build logs and the earlier compile failure. It excludes
the still-open full-library log. A future archive will preserve that result.

From the study directory:

```sh
shasum -a 256 -c validation-v2.sha256
mkdir /tmp/n83-shared-validation-v2-new
tar -xJf validation-v2.tar.xz -C /tmp/n83-shared-validation-v2-new
manifest="$PWD/validation-v2-files.sha256"
(cd /tmp/n83-shared-validation-v2-new && shasum -a 256 -c "$manifest")
```

The sorted ustar/xz archive uses uid/gid 0, root owner/group, file mode 0644,
and timestamp 2026-10-21 00:00 UTC. Extraction, all hashes and byte-identical
repacking were checked. Prior evidence is unchanged.
