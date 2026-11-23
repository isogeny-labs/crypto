# Affine replay and CLI integration validation

The corrected production source passed compilation and all functional affine tests in the 89-test batch. Batch result:88 passed/1 failed, exact source and binary unchanged. The final negative profile test incorrectly unwrapped malformed input which strict decoding rejected. Its reviewed test-only correction accepts rejection at decoding or typed profile validation while retaining real valid positives. This later correction is not retroactively included in the 88/1 result.

Seven exact mapped affine positive/fault tests passed, satisfying the independently reviewed raw test-contract amendment gate. Two explicit raw cap/ownership negative regressions replace unreachable raw positives; production raw expansion remains unchanged and original caps remain fixed. Historical failed results are retained.

Complete release-library validation is running in affine-row-full-release-gate-20261105, including the new raw controls and corrected malformed-profile test. The pending result must pass before push. Command-level correctness acceptance is separately prepared and reviewed; it has not run. No new scientific dataset or cloud experiment is claimed.

S3 receipts identify complete logs, retained binary, source proposal/reviews and correction evidence, verified by conditional upload and fresh hash-checked download.
