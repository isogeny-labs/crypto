# RunPod run of PROTOCOL.md (R1'), launched 2026-11-16

- Pod: `md3iv02hk7l6sa` (`replead-r1-20261116`), `cpu5m`, 16 vCPU, 128 GB, image
  `runpod/base:1.0.2-ubuntu2404`, 1.04 USD/h; owner-authorized route (2026-11-16).
- Script: `runpod/pod_run.sh` at commit b2d0535197 (sha256 62122d99...b4f6a6), passed by env.
- Source: tarball of this worktree at b2d0535197 (excluding .git, target, .resume, systems.jsonl),
  sha256 1a9e28178bcbcc3a4d202af0ab980b1318dadde4b6c143ba40235a73a09ed2c8, 137 294 806 bytes,
  at s3://isolab-results-104186811093-us-east-2/runpod/replead-r1-20261116/src.tgz.
- The pod builds derive/ and examples/replead_closure from that source, runs both test suites,
  regenerates the systems and stops unless their sha256 is
  251302b560bfb05502e5e455997ce6f207a119faa0c959a7480da197ab94b86c (PROTOCOL.md).
- Parameters: PAR=4 cells in parallel, dense call timeout 1800 s, sparse call timeout 3600 s,
  overall budget 36000 s; a timed-out or capped degree is recorded as unmeasured.
- I/O: presigned GET for the source; presigned PUT for `pod.log` (heartbeat every 300 s) and
  `results.tgz` in the same S3 prefix. No credentials on the pod.
- Resume checkpoint: s3://isolab-results-104186811093-us-east-2/checkpoints/crypto/claude__r1prime-closure-adapter-20261114/20261116T160621Z-c71d2ff58cec/
