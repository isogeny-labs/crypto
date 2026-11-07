# N83 radix neural lead probe, local r3

The fixed prime m=2 radix census was fit once with the preregistered 6-4-1
network. The model produced no new lead: it flagged 188 of the 2,975
censored rows below degree 42, and all 188 flags violate retained certified
lower bounds. On the 35 exact rows, 30 additional full-fit predictions were
below 42; those are regression errors against exact values, not censored
candidate leads. The census retains the same point base and candidate domain;
this run generated no new factor-base points.

The five-fold exact-row MAE was 21.9038129473 for the MLP and 145919.4018577
for the censored-aware constant comparator. The comparator is dominated by
the extreme certified lower bound in the fixed grouped objective, so this
descriptive MAE comparison does not establish useful model quality. The
full-fit predictions violate 2,834 of 2,975 censored lower bounds (95.26%),
which is the decisive diagnostic for candidate ranking here.

## Verification

The run inputs bind to raw census manifest SHA-256
`5546a85f4a7e4467289bc3d167845f9054dbbfb546d75949f0a9fceec6e70e78`.
All 3,010 case files were rehashed against that manifest. The output was
downloaded from the published S3 prefix and all nine output-file hashes
matched the local run. `verify_radix_neural_r3.py` independently reconstructs
all 3,010 predictions from the saved weights (maximum absolute difference
2.85e-14), recomputes the fold assignment, MAEs, sub-42 counts, contradictions
and lower-bound violations. Its result is recorded in
`independent-verification.json`.

The run and protocol both record protocol digest `66c2eb12…ae4027c4`, but the
committed v5 protocol file bytes currently hash to
`e344d3ec…ba6dac977`. Thus the run's protocol identifier matches the protocol's
embedded freeze field, while a digest check against the file bytes does not.
This binding discrepancy is retained in the independent verification record;
the outcome is reproducible from the preserved source, binary, inputs, model
and predictions, but the recorded protocol digest is not independently
verified as the byte hash of the current protocol file.

## Run record

- Run: `n83-radix-neural-20261031-local-r3` (local existing Spot host;
  EKS r1 and r2 were cancelled before worker startup).
- S3 output: `s3://setec-astronomy-artifacts/runs/n83-radix-neural-20261031-local-r3/raw/`.
- Model source/binary SHA-256: `a94eba7295c755a1ebf2cc1452704992de8965e1e8c95cec75ae14852d9b1b5c` /
  `c363e53c2db69630714a13169253fabdff76272f9391c00094074dbb4f8dda83`.
- Model, per-row predictions, cross-validation rows, metrics, command, host,
  checksums, run manifest, and independent checker are retained beside this
  report.
- This result is scoped to the fixed prime m=2 census. It does not measure
  generalization to other curves, m values, families, or factor-base point
  sets.
