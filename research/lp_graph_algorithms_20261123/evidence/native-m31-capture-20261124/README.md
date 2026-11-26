# Native M31 retained-capture receipt

Unsigned data registration: [PCC-20261124-fcc0a3](../../../../pcc/claims/PCC-20261124-fcc0a3.json). Standing is **unverified**; no scientific verifier or signing operation was run. The attested [receipt.json](receipt.json) was published unchanged at `add6ae761780f9f06a452e0557039cdb5d4b1f01`, SHA-256 `ff8511ec06d9011a9a496f40d850f648a0f35b140cd01e79e44ec18afc01dc3e`.

## Executed scope

[Run 45309117216](https://github.com/isogeny-labs/crypto/actions/runs/45309117216), job `138160138786`, completed successfully on exact checkout `3ba271b1f7100bdfd06eafbac8af283ec9d55094`. It used the unchanged original Rust arithmetic, admission and graph consumers. Field p=2147483647; relation prime ell=2147446831. The fixed ten-query source at `577d6c2c1c4a527604d965e6fb62c44df7902f36` is an existing generic exhaustive-pair capture on a known-answer public target. No collector was rerun. [pcc:unverified]

The converter replayed all 645 equations, kept 613 rows with at most two LP terms and recorded 32 higher-arity exclusions after group replay. Every forest, pivot and MST arm accepted its prefix with zero replay failures or inconsistent rows. [pcc:unverified]

| Accepted source-order prefix | LP rank | Useful rank, each arm | Augmented rank, each arm |
|---:|---:|---:|---:|
| 256 | 125 | 131 | 131 |
| 512 | 157 | 355 | 355 |
| 613 | 161 | 452 | 452 |

Recorded native values above and in [graph-prefix-summary.json](graph-prefix-summary.json) have unsigned standing. The complete native graph reports are [256](raw/m31/graph-256.json), [512](raw/m31/graph-512.json) and [613](raw/m31/graph-613.json). Three corrupted copies of the same input each exited 1 with empty stdout: wrong field, changed query scalar and flipped signed term. [pcc:unverified]

## Bytes retained and custody limits

- `raw/` contains 334 exact compact artifact files. [PUBLICATION.json](PUBLICATION.json) records every copied file and the 20 selected oversized files omitted from Git.
- [ARTIFACT_FILES.json](ARTIFACT_FILES.json) is the complete 470-file archive inventory. Its tuple columns are declared in the file; group roots restore every original path. [SHA256SUMS](SHA256SUMS) binds the compact publication created by the metadata helper.
- [conversion-audit-without-point-tables.json](conversion-audit-without-point-tables.json) copies the native audit fields apart from the oversized derived point tables. It identifies its transformation and the original audit hash. The tables, full admitted stream and exclusions remain bound by the native manifests and full archive.
- [api-metadata.json](api-metadata.json) retains the exact execution and artifact identifiers. [job-test-result-excerpts.json](job-test-result-excerpts.json) is explicitly an excerpt of the full decoded job log, with its byte count/hash.

The full [Actions artifact 14882048762](https://github.com/isogeny-labs/crypto/actions/runs/45309117216/artifacts/14882048762) is 2,917,091 bytes, SHA-256 `fcc0a3f2946215a7bb247fc0747732352730fc03384419219f402d1b27428b6a`, expiring `2026-12-08T02:03:47Z`. The full ZIP, binaries, oversized derived records and 66,046-byte job log are retained locally and in the available Actions surfaces; durable S3 custody is not claimed. The upstream raw input remains available at its pinned Git revision. [pcc:unverified]

Consumer reconstruction of the point table is derived evidence. The original collector's full rejection trace and original binary/toolchain attestation are absent. Its counters omit separate fixture and bespoke graph arithmetic. The native replay does not establish complete cold cost, timing advantage, target identifiability or target recovery. The final study report is [here](../../reports/run-20261124T0200Z/REPORT.md).

The metadata helper copies files and native values without mathematical recomputation. Raw `.command` files deliberately retain the runner's trailing spaces; no artifact bytes were reformatted to silence whitespace diagnostics.
