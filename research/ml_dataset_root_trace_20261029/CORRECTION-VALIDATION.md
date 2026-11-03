# Corrected prime replay validation checkpoint

Runtime source `5a3202c0a9fb22d471bcced0afd63d8bf29ca1c6` corrects both findings in REPLAY-REVIEW.md. A separate agent reviewed the stable correction and found no actionable defect. That was source review, not executable validation. The native arithmetic module remains unchanged.

Completed current-source checks: release compilation, 23 focused root tests (zero failures), maintained strict Clippy, and release CLI build. The unfiltered release library suite also passed: 5,264 passed, zero failed, 123 ignored, in 1,447.86 seconds. This result belongs to runtime source `5a3202c0a9fb22d471bcced0afd63d8bf29ca1c6`; the earlier 5,259-pass result belongs to `6542d87`.

The original frozen NFS CLI fixture ran all10 commands with exit0, but correctly failed acceptance: twisted shard63 reached its20-second worker timeout without sealing a root certificate. The complete failed attempt and executable are preserved at `s3://setec-astronomy-artifacts/ml-dataset/20261029/software/root-corrected-cli-timeout/72f245905ac539efa8922411fbfbce246b3df70f43b4d99e8fb519b900df617e/fixture.tar.gz`. The timeout cause is not established; no automatic retry or retroactive pass is claimed.

Commit `50dd148960b` froze a separate local-XFS software follow-up before its data. Inputs, binary,20-second worker limit,45-second command limit and300-second outer limit were unchanged. The follow-up passed all10 commands and both certificate/export checks. It supports correctness of those cases, not a filesystem or source performance comparison. The untwisted fixture has a verified PDP witness; the twisted fixture has a complete bounded root call without a PDP witness. Neither is independent scalar recovery.

Follow-up archive: `s3://setec-astronomy-artifacts/ml-dataset/20261029/software/root-corrected-local-cli/61696ce4011d845dcbf50a6b402e6efb4f816288cdd796182fee17564bf2af1c/fixture.tar.gz`,14,848,292 bytes. Conditional PUT, fresh HEAD/ETag-bound GET, length, archive hash and contained-file hashes passed. The downloaded executable subsequently verified both downloaded datasets; both outputs were byte-identical to the original verifies.

Executable SHA-256: `ceb6b787bce11e8b1a090f59a11ce8666fbf56e314b6fc7bf2f8f256443d0034`. Corrected source/review checkpoint: `s3://setec-astronomy-artifacts/ml-dataset/20261029/checkpoints/root-replay-correction/091c4784697e24eeef20e6469814d6fe3fed9abbc1d26f598c8d0fae7784893e/source.tar.gz`.

All these cells remain software fixtures. Scientific cloud cells remain zero. No worker IAM grant, new cloud job, extra budget, first/last-fall theorem, full execution completeness or complete instruction accounting is established. PR delivery remains pending; binary integration is proceeding in a separate checkout. Newer main-branch changes require separate integration validation.

The corrected full-suite executable, logs, source hashes, focused checks and independent source review are preserved at `s3://setec-astronomy-artifacts/ml-dataset/20261029/validation/root-prime-corrected-full/6122a990a79f31c9aa92f458761a5b02c3b90f78a1d4bb69896e3af64b7c5516/full.tar.gz`. The 41,861,048-byte archive passed conditional upload, fresh HEAD and ETag-bound GET, byte comparison, archive hashing, and all contained-file hash checks.
