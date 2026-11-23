# Prime31 development evidence

This follow-up applies the requested Fp, p = 2^31 - 1, to actual S3/F4
extraction and structured public-input recovery in PR #4922.

- [PLAN.md](PLAN.md): inputs, acceptance conditions and scope set before calls.
- [development-receipt.json](development-receipt.json): commands, source and
  binary provenance, raw artifact hashes and observed outcomes.
- [evidence/attempt1/](evidence/attempt1/): separate build, pre-call and
  post-call records. The copied binaries themselves are local build products;
  their hashes and the reproducible source manifests are retained here.
- [../REPORT.md](../REPORT.md), section 7: human-readable outcome and limits.
- [../report.pdf](../report.pdf): the refreshed report and diagram.
- [registration/README.md](registration/README.md): unsigned statement
  `PCC-20261122-95c625`, binding this receipt to source/evidence commit
  `26f6880421d5fcb85cb33c68ec2f56aedfae6803`.

The source-bound Rust suite recorded 12 passing tests. The focused Prime31
CLI recorded nine identical complete finite-chart solutions for original F4
and all fresh/shared preconditioning arms, followed by k=7 recovery modulo
2147446831 in each of the Integer, FMA and Split16 preconditioner arms.
Each recovery used three F4 relations plus the explicit G anchor and replayed
kG=Q. These are unsigned producer observations. [pcc:unverified]

This is a structured target/base correctness check. The solver receives public
Q, not its expected scalar; that separation does not turn the chosen input into
a uniform random target. F4 and relation Gaussian elimination remain the
existing integer code. The finite affine chart and incomplete cost coverage
remain explicit. No comparison of speed or complete cost was performed.

All initial evidence in the study's older `evidence/` directory is retained
unchanged. Test-harness elapsed times in raw output are diagnostic framework
text and are not admitted performance measurements. The root-library build
gate remains open after its previously recorded compiler SIGKILL.
