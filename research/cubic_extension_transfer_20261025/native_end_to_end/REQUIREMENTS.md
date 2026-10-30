# Qualified native cold-transfer benchmark continuation

Requested outcome: execute and qualify the complete elliptic-to-genus-three
pipeline, including transfer construction and inversion, against matched direct
source controls. Restored compute access alone does not qualify a measurement.
Historical and prior native stage diagnostics remain unchanged.

## Acceptance requirements

1. Input the original public source curves and subgroup generators from the
   p=109 and p=157 fixtures. Generate public known-answer targets separately;
   withhold their scalars from every solver API. Actual transported points must
   replace the earlier synthetic target-stage inputs.
2. Implement all missing execution paths natively: real 3-isogeny preprocessing,
   obstruction evaluation, quartic construction, divisor-side Weil functions,
   section calibration, inversion, divisor recovery, sign alignment and target
   solving. Verify the final scalar on the original source curve.
3. Charge all cold setup, failed attempts, retries, transfer, target work,
   recovery and verification. Record genuinely reusable setup separately; never
   replace cold cost by amortized or stage-only cost.
4. Establish exact-fixture correctness before timing. Route new measurements
   through the repository's frozen experiment.protocol/v1 and isolated-runner
   workflow, with L2 or better earned from observations. Do not retry the known
   under-qualified local host and relabel it isolated.
5. Include direct source rho with and without negation, full-transfer IC,
   a full-transfer target-rho control and A/A controls, with common workload generation,
   interleaved order, independent nuisance replicates, at least eight targets
   per curve and five measured rounds if wall time is reported. Retain one
   warmup, failed runs and the exact stop rule. Every cold row starts in a fresh
   process without retained transfer or factor-base setup; warmup must not
   populate a cache used by a cold row. Two field sizes do not support a scaling
   or asymptotic claim. Establish the strongest applicable source rho variant
   before freezing the comparison; the two existing variants alone are not a
   certificate that all applicable automorphisms have been covered.
6. Bind source, binary, inputs, protocol, host, counted units, pressure,
   affinity/reservation, context switches, faults, clock validity, attempts,
   verification and replay. Complete native-work pricing and applicable
   external/independent checks before performance admission.

## Current verified starting point

PR #2545 in isogeny-labs/crypto, permanent repository ID 1221029056.
Published substantive commit c6c28772f4b91329db062dd73cc32fb42c737f67;
published handoff 376b380302af950452c4d7feea8783dbd0ff3b2e.
The original partial import checkout is at f505f0d and contains the same native
execution sources. A new blobless sparse continuation checkout starts from
376b380302af950452c4d7feea8783dbd0ff3b2e. Its root Cargo files are present;
off-disk workspace sources must be materialized before root validation.

The native follow-up contains exact torsion-action calculations, obstruction
replay, projective divisor encoding, fixed-quartic stage solvers and source
rho controls. It does not contain a complete native forward/inverse transfer.
The earlier 58 diagnostic answers and 2,492 divisor checks do not discharge
the requirements above.

## Execution prerequisites

- Run the official S3 resume-checkpoint gate before every experiment launch.
  Do not bypass it or create replacement credentials.
- Read-only runner checks must establish access to an existing compatible
  isolab NATS hub or EKS backend. Direct connections require their documented
  identity and network route. The documented Actions/CodeBuild bridge is an
  alternative only if its actual authorization and job contract are verified.
  The deployed EKS default executable does not change when a caller supplies
  source_revision as provenance alone. The current bridge supports a verified
  per-run job.build image; the reviewed main-branch GitHub workflow can build
  the requested commit before submission. Either path must include the actual
  native map. NATS can build pinned source when its connection exists.
- Freeze the final instrument/protocol only after the native map passes exact
  correctness controls. This file is a requirements/runbook record, not a
  frozen measurement protocol or evidence that a job has run.

Initial correctness-run limits: one thread, 4 GiB, 30 minutes per command.
No paid job or new capacity is launched by this preparation. Runner placement,
estimated duration and a bounded cost must be recorded before submission.

## Sources and implementation contracts

Start from the frozen historical research_lead/inverse_map.py,
polynomial_inverse.py, transfer_pipeline.py and FOUR_TRACK_METHOD.md, preserving
their mathematical contract while replacing execution with native code. The
native torsion-action and divisor code and the pinned PR #2170 quartic backend
are existing components, not a completed transfer. Literature: Tian's cubic
cover construction, the cited algebraic obstruction method and Diem--Thome
target index calculus. Primary-source review of any new construction remains
required; no novelty claim is proposed by this runbook.

The continuation reviewed Tian, *Cover attacks for elliptic curves with prime
order*, [arXiv:2012.07173](https://arxiv.org/abs/2012.07173), Algorithm 2 and
section 3.2. These explicitly require divisor-side Riemann--Roch/Weil-function
normalization, eight-section conversion, polynomial inversion and resolution of
the two opposite Jacobian classes. Existing native stage solvers implement none
of that missing map contract. The historical files above are archived under
`../historical/extension-field-experiments/research_lead/`.

See [the observed access status](ACCESS_STATUS.md) before attempting a run.

2026-10-28 access continuation: PR #2545 has merged. The official checkpoint
gate now succeeds with the supplied lab-account AWS identity and a privately
installed restricted upload token; the first uploaded resume was verified by
readback. All five original artifacts are published and hydrated. The current
EKS skill supports commit-built images through its reviewed GitHub path, so the
old deployment limitation must not be generalized to that newer path. Direct
AWS-bridge status polling is denied to this identity. No real native instrument,
frozen comparison or qualified run follows from these infrastructure checks.
