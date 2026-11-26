# Aggregate wrapper correction and final invocation

The final command check found a target-selection error in `check.sh`: the
original wrapper requested graph `--all-targets`, while the qualified graph
gate and its README use `--test graph_suite`. Inherited graph unit tests need
the root crate's complete `jv_bitops` telemetry interface, which this standalone
comparator does not supply. That invocation returned 101; its command, complete
diagnostics and terminal status are in `wrapper-compatibility-v1`.
[pcc:unverified]

The correction lets each component specify its Cargo target. Graph runs its
seven integration tests, which also compile and exercise the comparator CLI;
structured and RR5 retain `--all-targets`. No algorithm or existing qualified
test changed. The corrected wrapper was executed sequentially with one build
job, one test thread, offline locked dependencies and a 180-second cap per
component. Graph, structured and RR5 each returned zero, with 7, 13 and 50 tests
passing. These repeat the original 70 distinct tests. [pcc:unverified]

`wrapper-final-v1` contains the outer invocation, toolchain/environment capture,
exact per-gate commands, complete stdout/stderr, each exit and `overall.exit`.
It reused existing Cargo caches, including temporary target-directory symlinks
that were removed after execution. This is a cached development run.

`wrapper-final-v1/receipt.json` pins the corrected wrapper and all of those
artifacts. Its source base is `42cbbf529176ef49767575e7415520647dd41959`, with
the explicitly identified working-tree wrapper change. It checks every entry
of the original source manifest: only `check.sh` differs. The original
`verification.json`, `SOURCE_SHA256SUMS`, report, PDF and claim
`PCC-20261124-db5264` remain unchanged records of source commit `709a68ad`.
Supplemental producer claim `PCC-20261124-9f9ff4` binds the wrapper receipt and
the CI precondition observation. Neither claim has a signed verdict.
[pcc:unverified]

## Automatic CI observation

At head `42cbbf529176ef49767575e7415520647dd41959`, the automatic
[Round123 run](https://github.com/isogeny-labs/crypto/actions/runs/45323167283)
and [Round124 run](https://github.com/isogeny-labs/crypto/actions/runs/45323167354)
stopped at source-precondition assertions before compilation or benchmark
execution. Round123 expects one cached-inverse expression and finds zero.
Round124 expects two Barrett-specific tokens in `field_value`; both are absent.
The scripts, exact input source files and workflows are byte-identical to base
`ceba74bb64854027c4dd16f00428bf676383bfdb`. Exact blobs and predicate results
are in `ci-preconditions-v1.json`. No baseline CI run or manual rerun was made.
[pcc:unverified]

The passing component wrapper does not establish a whole-repository pass.
The previously recorded root-library compiler/build gate remains incomplete,
and automatic CI remains subject to its actual final-head results. Preserve
the no-merge and no-paid-compute holds.
