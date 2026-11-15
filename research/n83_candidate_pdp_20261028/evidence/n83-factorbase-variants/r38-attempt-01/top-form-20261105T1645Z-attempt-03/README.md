# spike38 top-form attempt 02

This fresh attempt runs all 48 frozen V2 systems for candidate spike38 through the PDP system generator and top-form quotient degree-of-regularity producer/replay. It uses the same complete factor bases at m=2..9 and natural/planted targets. The previous attempt is retained; it failed at its initial S3 prefix check before generating a PDP system or certificate.

This output reports top-form quotient Dreg certificates or intervals only. It does not establish affine FFD/LFD or a factor-base ranking. The 384 compressed factor-base shards are bound by the verified recovery audit and the `f990221ecd91d7a7a8bc3eadf89529aec5f06c7ed8fa3cad40f31dd245337765` S3 file index. The exact source hashes, binary hashes, base manifests, run limits, budget and launch command are in `preflight.json`; `PRE-RUN-FILE-LAYOUT.json` gives the expected output structure.

S3 output prefix: `s3://isolab-results-104186811093-us-east-2/runs/n83-pdp-top-factorbase-20261031-v2/spike38/attempt-02/`. Compute hard stop: 13,000 seconds plus 45-second termination grace, one CPU, 8 GiB, four concurrent shard downloads, no retries. Estimated maximum EC2 cost is USD 1.93 plus USD 0.25 artifacts.


Static attempt inputs and launcher were published under `attempt-03/capsule-v2/`; scientific outputs are written only to the fresh `attempt-03/results/` prefix. The earlier static capsule at the attempt root is preserved as capsule version 1.


The exact build completed before scientific launch in 108 seconds. The launch rechecks both binary hashes and pins the entire execution tree to one allowed CPU. The scientific phase has a 12,850-second hard cap plus 45-second termination grace; including the completed build, total planned process time stays below the USD 1.93 compute ceiling. Final static capsule v3 records these execution details; v1/v2 copies remain preserved.


AWS CLI uses `AWS_MAX_ATTEMPTS=1` for data fetches and uploads, matching the no-retry rule.
