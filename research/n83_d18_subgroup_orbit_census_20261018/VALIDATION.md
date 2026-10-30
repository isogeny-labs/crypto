# Dimension-18 direct-subgroup census validation

| Gate | Result | Evidence |
|:--|:--|:--|
| Frozen size rule before execution | protocol committed before native run and reports | [protocol](PROTOCOL.md), commit history in the PR |
| Native example build and focused test on the census source | pass; one unit test, binary hash retained | [build](build.log), [focused test](example-tests.log), [hash](binary.sha256) |
| Full census of 2^18 x words | direct invocation exit 0; 32,643 full classes; size threshold 16,627 passed | [summary](run-20261019T0142Z/summary.json), [all rows](run-20261019T0142Z/orbit-classes.csv), [selected rows](run-20261019T0142Z/selected-representatives.csv) |
| First wrapper attempt | wrapper exit 1 after native result due sandbox-denied `sysctl kern.clockrate`; retained without treating it as a mathematical failure | [run report](reports/run-20261019T0141Z/RUN_REPORT.md), [raw attempt](run-20261019T0141Z/) |
| Repeatability and file integrity | two same-implementation CSV pairs byte-identical; 21-file SHA-256 manifest checks | [audit](audit.log), [manifest](raw-files.sha256), [check](raw-files-check.log) |
| General arithmetic spot checks | 17 selected representatives checked for curve, subgroup and Frobenius return | [native source](../../examples/n83_d18_subgroup_orbit_census.rs), [summary](run-20261019T0142Z/summary.json) |
| Full independent replay | open; the two runs use the same native implementation | [result limits](RESULT.md) |
| Materialized selected point object | open; 2,760,082 is a derived closure count | [result limits](RESULT.md) |
| Benchmark copy and report | study and benchmark PDFs, SVG and PNG are byte-identical; one-page PDFs inspected | [run report](reports/run-20261019T0141Z/RUN_REPORT.md), [result](RESULT.md) |
| Final stacked-head release library | pass: 4,395 tests passed, zero failed, 118 ignored in 193.01 s with loopback access | [raw log](cargo-release-lib-integrated.log) |
| Final stacked-head touched example | pass: one test, zero failed | [raw log](final-example-tests.log) |

The earlier `cargo test --release --lib` run on the pre-stack census source passed 4,221 tests with zero failures and 117 ignored. Its [raw log](cargo-release-lib.log) is retained separately from the final stacked-head rerun. The run records no natural relation, useful rank or complete cold pipeline comparison.
