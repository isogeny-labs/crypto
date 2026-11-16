# Six-arm CPU comparison preparation

## What was done

The protocol is frozen; no experiment has been submitted or executed. The runner
covers six arms, three existing fixtures, and one warmup plus nine measured
rounds: 180 planned rows, including 162 measured rows and 18 warmups.

| Arm | Implementation | Batch |
| --- | --- | --- |
| u, x | Original baseline `2bd4588fba60` (A/A) | 64 |
| v | Incremental hash enumeration `0d6e3840edb6` | 64 |
| w | Same candidate | 1 |
| y | Curve arithmetic context `23ff0e6bda63` | 64 |
| z | Inverse cache `5d7dcb67616c` | 64 |

The three materialization records pin committed inputs and preserved binary
hashes. Candidate inputs were compared byte-for-byte to committed blobs; all
four solver binary hashes were checked before copying to the staging bundle.
No solver was executed during preparation.

The standalone analyzer requires all cells, expected seeds/scalars and L0 labels,
valid numeric fields in warmups as well as measured rows, nonempty certificates,
and identical complete certificate bytes and recovery metadata across arms for
each fixture/round. It reports descriptive stage ranges and medians only.
Certificate byte equality is not semantic replay: the solver performs both
replays, and independent scientific review remains separate.

Six synthetic development tests pass: complete panel, missing/duplicate cells,
mismatched certificates, empty certificates, row-order independence, and invalid
fields. These inputs are generated in temporary directories, contain explicitly
synthetic certificate text, and are not algorithm results. Rust compilation with
warnings denied and shell syntax checking also pass.

Reproduce development checks (no performance experiment):

```sh
rustc --edition 2021 -D warnings -O analyze.rs -o /tmp/round18-analyze
ANALYZER=/tmp/round18-analyze python3 -W error test_analyze.py
ANALYZER=/tmp/round18-analyze python3 -W error test_export.py
bash -n run.sh
```

## Decisions and references

Reviewed `../hash-enumeration/README.md`, which records the supplied paper's
Section 3.5/Lemma 13 and the exact inverse-hash recurrence. Round18 will separate
the previously unmeasured hash-enumeration and Curve-context changes in one panel.
It retains baseline, batching reference, A/A, all three fixtures, setup, discarded
trials, both complete certificate replays, and serialization. Timers and solver
workloads are unchanged. Prior round16/round17 records remain immutable.

## Open threads

Protocol `259ecd8eec7d7c17e99fa3f15c52196aa6b07cf33ac1bf0d60ab3c36f98e26a6`
validates with zero findings and pins all source revisions, manifest hashes and
the analyzer hash. The export adapter and build manifest are complete. Three
additional synthetic packaging tests pass, including manifest mismatch and
incomplete-row rejection; the exporter refuses to overwrite an existing export.
The local bundle is assembled, but has not been uploaded. Before submission,
verify the freeze commit is pushed, eligible EKS capacity is available, then
stage and verify the bundle. Presigned submission files remain private scratch
artifacts and must never be committed. The competing `semaev-n47-rand00-d5-reuse-20261030-1600` job was
observed active during preparation; no round18 job was submitted.

L0 diagnostics cannot establish the further 10x end-to-end goal. Strict-isolation
measurement, full evidence, scale qualification and independent review remain
outstanding. No speedup is attributed to these files or to the unmeasured source
revisions.

## Errors and ownership

T-952 owns this directory and the prospective report directory. Initial tests
exposed a Python ResourceWarning; the CSV reader now closes its file. Inspection
found that warmup numeric values were not checked, now corrected with regression
coverage. PR #2979 merged during preparation; its deleted remote branch cannot
be fetched by name. Subsequent delivery uses a new branch and retains history.

Protocol and packaging completed under T-952 on 2026-10-30. The competing job
was rechecked Running on ip-10-84-78-223 with 112Gi allocated memory. No
replacement job was submitted into the previously blocked placement.
