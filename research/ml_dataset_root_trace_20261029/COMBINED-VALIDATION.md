# Combined native roots and exact-cofactor validation checkpoint

Runtime source `a0619090bd99b18e828bc16b4b508ca7dafd58e0` combines prime and binary root replay, main `78f84234e18facfc46be52741d7fa6acadc92b88` graph export, and the retained exact-cofactor context. Subsequent commits add documentation and pre-data software fixture protocols only.

Release compilation passed. All 47 focused checks passed: three exact-cofactor tests, 21 binary-root tests and 23 prime-root tests. The exact-cofactor fixture invokes root arithmetic replay on its fixed large-cofactor public instance without a private witness. Maintained strict Clippy and scoped Rust 1.98 formatting passed.

The separately invoked endpoint test passed all 360 combinations of n=23/61, m=2/4/15, three families, both twists and ten adapters. It checks structured outcomes and any returned PDP/scalar certificate; unsupported and budget-limited results are allowed. It is not a record of 360 solved instances, native root certificate replay for every endpoint, or scientific corpus production.

The frozen actual CLI fixtures passed all 44 commands and ten certificate/export gates: two prime n7 cells and eight binary/Koblitz n11/n12 cells, including both twists. No PDP success requirement or target resampling was imposed. The executable SHA-256 is `cd74dccb67d1b49f8ad4ccbb69145bedb62b5a5880e2f25dd800f11be999d3e7`.

Combined fixture archive (15,361,601 bytes):
`s3://setec-astronomy-artifacts/ml-dataset/20261029/software/cofactor-combined-cli/4ba9369a0fd6bd4f389395693e1e0203030ddeb321651f3b644bde98b6749718/fixture.tar.gz`.

Conditional PUT, fresh HEAD and ETag-bound GET, lengths, archive-byte equality and contained-file hashes passed. The downloaded executable then verified all ten downloaded datasets; every verification stdout matched its original byte for byte. The separate attestation includes the frozen source/protocol bundle:
`s3://setec-astronomy-artifacts/ml-dataset/20261029/validation/cofactor-combined-restored-replay/eb262adb7acd40044336fccca546128a7514315af438501bd5a2ea5462a9c260/attestation.tar.gz`.

The focused-test executable, source bundle, logs and source review are retained at:
`s3://setec-astronomy-artifacts/ml-dataset/20261029/validation/cofactor-focused/315063eb5296dbacec067ea31e958296c66c1888cd56580fc7cd066ef71dd1cd/checkpoint.tar.gz`.

**Full combined-source release-library validation passed: 5,309 passed, 0 failed, 123 ignored, in 1,857.75 seconds, exit 0.** The earlier binary source dd1092a passed 5,284 tests; the separate prime/current-main source dd0f10b passed 5,286 tests. Those earlier results remain distinct from the completed full suite on a0619090. Source review covers the retained runtime change; the later large-cofactor bridge test has now executed successfully as part of the focused gate.

Distinct roots of completed recorded polynomials remain separate from complete execution, total instruction cost, general PDP nonexistence, independent scalar recovery and first/last-fall-degree bounds. Both completeness/accounting verification flags remain false. Scientific cloud dataset cells remain zero. Policy integration, active exporter ownership, worker publication permissions, cumulative admission and the original budget/deadline remain unchanged.

Full-suite logs, source hashes and the immutable executable archive reference passed fresh S3 download and hash verification:
`s3://setec-astronomy-artifacts/ml-dataset/20261029/validation/cofactor-combined-full/18721757e9e1c7a447ab9bd80a244aad14530cbab5bd12114e56d326a000ceb6/full.tar.gz`.
