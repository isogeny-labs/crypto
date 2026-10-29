# Checked phase integration validation — in progress

Runtime and CLI source: `c59a5dd8a8a81c4749540b92ebcdf7fb24da4a59`.
CLI SHA256: `74b23756aa776d85085bbfeef03400b68a354994bc3a8a2bea6c1d19735414d1`.
The subsequent source delta `7ace206adc9` adds only comments and `#[cfg(test)]`
to the registry classifier: it has no production callers and remains available
to the existing registry regression. Strict release Clippy passes at that source.
Other subsequent commits freeze and correct fixture documents/scripts.

The runtime release build and 79 focused tests passed: collector26, capture12,
checked telemetry12, report11, prime-chain18. The prior runtime f6fdcb3 failed
one equation-persistence test; its failure remains archived. The correction stages
canonical equations before trying the bounded live sink and does not invoke native
elimination when that initial persistence fails.

The full unfiltered library suite is still running on the immutable c59a5dd test
executable. No full-suite pass or current-head CI/merge claim is made here.

## Actual CLI and independent consumer

The v3 frozen fixtures passed54 CLI commands across10 fixed cells: two prime
and eight binary/Koblitz, both twists. Each exercised plan/preflight, generation,
native verification, status, export and report. Four starts/four metrics are
retained per cell. Reports contain eight phase-v2 rows plus one native solver
counter row. Restricted data are excluded from the public feature projection.

The separately implemented retained consumer independently audited/exported all10
cells and invoked the pinned native verifier successfully. It checked10 completed
distinct-root sets and3 positive certificates. Its runtime executable SHA256 is
`2512b52b8d4061febd068746b0e15fffb72cc5c1bfb8e4f54f45469236d8523c`.
Capture-limit semantics remain producer-verified, not independently implemented
in that consumer. Full mathematical verification, execution completeness,
whole-worker instruction counts, hardware/platform admission, training-cost
eligibility and scientific corpus completion remain false or unknown.

The v1 fixtures stopped at preflight because freeze metadata lacked required
fields; zero cells generated. The v2 fixtures generated, verified and exported
the first prime and binary cells, then stopped on a test assertion that wrongly
expected eight total report rows. The v3 amendment checks all eight phase rows
and the distinct solver-counter row. Inputs/binary/limits remained fixed; every
stopped attempt and its output is preserved, not relabeled as passing.

## Durable artifact packet

s3://setec-astronomy-artifacts/ml-dataset/20261029/software/checked-phase-cli/55ad175c4ea290301f4297c2746ae39ba2f4eb63f3ed4dac332dad3b445ffa37/fixture.tar.gz

SHA256: `55ad175c4ea290301f4297c2746ae39ba2f4eb63f3ed4dac332dad3b445ffa37`; bytes: 17032560.
Conditional PUT, HEAD, fresh ETag-bound GET, byte comparison and all contained
hash checks passed. Packet includes source bundle, producer/consumer binaries,
all three fixture versions' outputs, independent audits, CLI/lint logs and
precise source identities. Additional restored-native replay is recorded separately
when complete. These are software fixtures, not scientific cloud cells.

No new cloud worker, IAM change, budget increase or deadline extension. Original
USD200 total cap and the incomplete broad corpus objective remain unchanged.
