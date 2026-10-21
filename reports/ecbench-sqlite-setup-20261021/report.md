# ecbench CI SQLite setup repair — 2026-10-21

**Requested scope.** Continue validating the prior crypto experiments and
proof gates, including the current [proof-gate PR #2357](https://github.com/aburan28/crypto/pull/2357).
This branch makes one CI dependency-setup correction on crypto main
`51d502ba2878367d9112c2be504bf780b7069de1`. The local Rust tests below
ran on its predecessor `2f7f8538258fd81c1972c142fc6465a8086b61f0`;
the intervening main commit changed only validation-gate documentation and
Python scripts. This branch does not modify an
experiment, result, theorem, claim checker, performance graph, or dashboard.

## Observed failure and exact change

The current-head [`ecbench` harness job](https://github.com/aburan28/crypto/actions/runs/39533982254/job/119039851792)
on PR #2357 reported `audit OK: 72 records, 72 verified, 8 replays reproduced,
0 problems`, then stopped before its database-load assertions. The runner
lacked `sqlite3`; its direct `sudo apt-get install -y -qq sqlite3` returned
`E: Unable to locate package sqlite3` (exit 100). The [compressed raw job
log](evidence/ci-harness-failure.log.gz) is retained. That is a package-index
setup failure, not a theorem verdict or a failed database assertion. The
workflow file was unchanged by PR #2357.

The `Database loads, twice, strictly` step now checks whether `sqlite3` is
present. If absent, it runs `sudo apt-get update -qq` before the same install.
The two SQL imports, 72-row assertion, subsequent claim checks, and failure
behavior remain unchanged. This follows the setup pattern already used by
`.github/workflows/jacobian-certificates.yml` in this repository.

## Validation

| Check | Status and evidence | Limit |
|:--|:--|:--|
| Workflow YAML parse | **Passed** with PyYAML; `harness` and its database step were found. | A parse does not run the hosted job. |
| Extracted database-step shell | `bash -n` **passed**. | The local host lacks the GitHub runner's Ubuntu package state; hosted install remains to be observed. |
| `git diff --check` | **Passed**. | Whitespace only. |
| `cargo test --release --lib`, first attempt | **Infrastructure failure** during linking: `/tmp` tmpfs was 98% full and `cc` reported no space. [Compressed log](evidence/local-cargo-tmpfs-failure.log.gz). | No test verdict from this attempt. |
| `cargo test --release --lib`, retry with `TMPDIR=/var/tmp/crypto-ecbench-buildtmp-20261021 CARGO_BUILD_JOBS=4` | **4,473 passed, 1 failed, 119 ignored.** The sole failure, `every_cited_file_exists`, referenced a tracked report omitted by the local sparse checkout. [Compressed log](evidence/local-cargo-sparse-failure.log.gz). | The sparse omission is a local checkout issue; it does not establish a product failure. |
| Focused `every_cited_file_exists` after materializing the cited report and `isogeny_algos/` dependency | **1 passed, 0 failed.** [Compressed log](evidence/local-cargo-focused-pass.log.gz). | Focused replay only. |
| Full `cargo test --release --lib` with the corrected sparse checkout and `/var/tmp` scratch | **4,474 passed, 0 failed, 119 ignored** in 77.57 seconds. [Compressed log](evidence/local-cargo-final-pass.log.gz); raw log SHA-256 `6d899dbbe5c46cd72ca3d6acddb840e1e320f3b2a79238c2f15dfcec06bb9f44`. | Local library tests; the hosted workflow change still needs a current-head runner result. |
| `python3.12 scripts/test_validation_gate.py` on the rebased checkout | First run: 48 tests passed, one errored because tracked `benchmarks/README.md` was absent locally. After that file was added, 48 passed and one failed on 116 rows without the sparse-omitted freeze, archive, and sweep files; [log](evidence/local-validation-gate-sparse-freeze-failure.log.gz). After those files were added, 48 passed and one failed because eight tracked archive provenance files were absent; [log](evidence/local-validation-gate-sparse-provenance-failure.log.gz). | Sparse checkout omissions, not gate verdicts on a complete tree. |
| `python3.12 scripts/test_validation_gate.py` after adding those exact tracked inputs | **49 passed, 0 failed** in 6.432 seconds; [compressed log](evidence/local-validation-gate-pass.log.gz), raw log SHA-256 `fbff920a8e5e5ef20e454a0ea533a5633bec5fd498dd4264c0fb4f74b2c92314`. | Validates the rebased gate code locally; the hosted workflow remains pending. |

The sparse-checkout repair itself needed a retry: the first profile update
could not create a temporary file on the full `/tmp` filesystem. Reapplying
the tracked minimal profile with `TMPDIR` on `/var/tmp`, then adding the cited
report and dependency directory, restored the checkout before the successful
focused and full Rust runs. The Python gate's committed-tree test required
materializing its benchmark freeze, archive, sweep verdict, and eight archive
provenance paths. All were tracked files; the final run used that complete
set of referenced inputs.

No new ecbench measurement was run. The existing hosted job's successful
72-record audit and eight replays are observations from that job only; the
unreached database assertions are still unverified on this change. The
original user request remains open across the three repositories; this repair
only enables a previously blocked CI stage to execute.
