# Retained primary N83 K2048 five-summand search at 100,000 conflicts

The frozen one-query search ended with **`UNKNOWN_solver_cap`** at exactly
100,000 SAT conflicts. It made one solver call on the first public `a=0`
fixture, found zero models and zero relations, and retained a zero-row,
rank-zero matrix with 2,049 columns. This is a censored solver observation:
it does not estimate relation yield at a larger cap or select a factor base by
complete index-calculus runtime. Both `total_index_calculus_runtime_ms` and
`selected_best_total_runtime` remain `null` in the worker and outer receipts.

The base selected for this trial is the already published 339,968-point, 2,048
signed-Frobenius-column hash-policy object at seed `2026100801`. A fresh S3
HEAD and GET matched its 10,313,647 compressed bytes, BLAKE3
`82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851`
and SHA-256
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`.
The exact URI is in the [frozen input receipt](evidence/frozen-input/input.json)
and [evidence manifest](evidence/evidence-manifest.json). Public fixture zero
produced target point `(6261116474805304534310590,
4505575220639647215230954)`, the same point used by the earlier matched
K1182/K2048 5,000-conflict gate.

The static x86-64 Linux worker SHA-256 was
`21817b5218b59b5227de5c705e44710583ef70bc8863a5c544d93853a98b5eed`.
The [build attestation](evidence/build-attestation.json) compares every
compiled source blob between build-start commit `8f971585692c702d2804668b322e2440177f80d2`
and run source commit `5f9d45cc92dc60f3b8298fa102498d58bf84c725`.
The [container inspection](evidence/run-01/container-inspect.json) recorded
the exact image ID, one CPU, 4 GiB hard memory, zero swap, no network and
read-only input mounts while the worker was running. The source, input,
binary and configuration stayed hash-identical; the container was removed
after exit.

| Measured component | Observed value |
| --- | ---: |
| Offline static Linux build | Approximately 438 s; build-only, separately attested |
| Search supervisor and cleanup | 2,846.703 s wall, under the frozen 5,400 s outer cap |
| Worker base import | 4.751 s |
| Worker solver stage | 2,840.815 s |
| Worker process CPU | 2,824.202 s |
| Worker cgroup peak | 1,160,802,304 bytes of 4 GiB |
| SAT calls / conflicts / models / unknowns | 1 / 100,000 / 0 / 1 |
| Relations / independent rows / terminal rank | 0 / 0 / 0 |

These are stage charges on one saturated shared macOS host with an emulated
Linux worker. The search's conservative envelope is 5,400 seconds, in
addition to preparation and earlier study work; observed stage wall time is
reported separately. Neither the 100,000-conflict result nor the earlier
5,000-conflict K2048 result is a full cold index-calculus measurement.

The [independent audit](evidence/run-01/audit.json) rehashed the frozen S3
bytes, manifest and public fixtures, compared retained source snapshots with
the run commit, checked binary and Docker limits, validated worker/outer
receipts, independently replayed the public query in the original curve group,
and checked container cleanup. It passed on the original run and on the
retained evidence copy. A temporary forged-success control changed both the
worker and outer success statuses and recomputed the outer worker hash;
the auditor [rejected it](evidence/negative-control.json). The
[manifest](evidence/evidence-manifest.json) hashes all 31 retained raw,
validation and input-metadata files. The large compressed base remains at its
exact S3 key;
the local build binary remains outside Git under the frozen SHA-256.

This gate does not establish repeated natural relation yield, useful rank,
column-log precomputation, linear-algebra cost, individual-log extraction or
a same-point rho comparison. A primary minimum-runtime selection needs those
stages on a frozen multi-point panel with complete cold accounting.

The [validation receipt](VALIDATION.md) retains the initial default-parallel
library failures, the passing serial replay of that group, the clean full
four-thread release-library rerun, the exporter-example and Python suites,
and the evidence-integrity checks.
