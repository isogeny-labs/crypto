# Native hashed ECDLP ECBench feasibility

- **Status:** pre-data; execution gates pending
- **Ready to execute:** false until actual fresh checkpoint/readback and exact
  native image/authenticated execution admission pass
- **Canonical study:** [protocol, spec, source and checks](../../research/hashed_ecdl_ecbench_20261030/README.md)
- **Driver:** actual `ic.hashed_native_v1` in the maintained native ECBench/Iso Lab path

Run the fixed four registered prime-roster cohort (10/12/14/16 field bits),
eight public targets per curve, nine arms, one warmup and five measured rounds.
Retain the original 1,800-second EKS Spot deadline, explicit five-million-step
native work bounds, all failures and zero automatic measurement retries.
Source/image/binary/spec/job/pod identities, L2, replay-all, A/A and S3 hashes
remain required. Incomplete records prevent a full cost comparison; native
arithmetic remains incompletely priced and observation overhead is included.

The new checkpoint workflow uses the existing temporary OIDC identity and
does not start a measurement. Submit only after actual upload/readback succeeds,
then exact-image publication/admission. Reconcile the original request after an
ambiguous result; never duplicate it. Do not replace this arm with the unrelated
whole-point tensor fixture or claim that development tests are this run.

This card does not replace the original frozen 60/24 A/B study, its profiling,
or the older finite-size proposal. Complete binary and schoolbook pipelines,
both challenge scalars and the independent general audit stay open.
