# r38 top-form Dreg follow-up

This is the `spike38` slice of the frozen `N83-FACTOR-BASE-TOP-DREG-20261031-V2` protocol. It covers all 48 systems: three N83 families, every `m=2..9`, and natural plus planted targets. The same complete `spike38` family bases are reused across all eight `m` values. Results are original highest-homogeneous-form quotient Dreg certificates and intervals; they are not affine FFD/LFD, solving degree, Castelnuovo-Mumford regularity, solver cost, or ECDLP outcomes.

The original source-side follow-up was reconciled before resuming: the candidate-specific S3 prefix `s3://isolab-results-104186811093-us-east-2/runs/n83-pdp-top-factorbase-20261031-v2/spike38/` was empty, no local r38 input/base/top output trees existed, and no `pdp`/`top` process was active. The old `run-top.sh` targets the earlier baseline/`spike40` protocol and is not used. The saved r31 candidate commands provide the precedent; `run-spike38-topform.sh` binds this run to the current frozen v2 protocol, source commit and binaries.

The r38 base is complete: its producer index and all 384 shard joins passed the recovery audit in `../terminal-recovery-20261101T2000Z/`. All 402 noncompressed producer files were restored and SHA-256 verified. The compressed point shards remain in S3; this runner downloads and rechecks all 384 before use.

Protocol freeze SHA-256: `3d852e474eb7b129e51f68bb564f379d9d997148dbe9c3727e529102a6ef47c6`. The current protocol file SHA-256 is recorded separately in `protocol-binding.json` and matches its sidecar; the protocol retains its pre-data amendments and frozen source/binary pins.

No Dreg outcome is recorded here until the producer and fresh-process replay finish. All partial outputs remain in this fresh candidate prefix and are uploaded without deletion.
