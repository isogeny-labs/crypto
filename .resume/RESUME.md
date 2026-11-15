# Resume: crypto @ claude/zealous-meitner-85ipb5

## Goal
Owner (2026-11-14): run the frozen degree-13 fes-f2 vs inherited-f4 measurement (research/fes_k13_validated_20261112) on Runpod instead of isolab (AWS budget exhausted). Leaderboard rows only if the validation gate passes.

## Spec
Frozen: research/fes_k13_validated_20261112/spec.json (sha256 327e0bb4...a9d76c, = protocol links.spec_sha256), protocol.json. Binary: ecbench built from e9d5dbdccd, sha256 8f9576b5bf81819274cc6049eb671c6d97f0bc6660f99d0445448edb4e5130f4 (smoke/build.json).
Route: Runpod CPU pod (ubuntu:24.04, glibc 2.39), binary+spec fetched from S3 presigned GET (s3://isolab-results-104186811093-us-east-2/runpod/fes-k13-validated-20261112-a1/), results tarball PUT back to the same prefix. Pod runs: ecbench host --json; ecbench run --spec spec.json --out session; ecbench verify --dir session --replay-all --exit-code.

## State
2026-11-15: staging inputs, then pod launch. Deviation from protocol: Runpod is not an L2-isolated host (no isolab admission); S counts are deterministic, the wall figures are not admitted.

## Next
Download results, check hashes, independent replay here (different host class) + validator report + band, VALIDATION.json, then report/PDF/benchmarks/benchrec/session log/PR. Terminate the pod after the upload.
