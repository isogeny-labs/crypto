# PQC platform-specific performance and leakage validation

- **Status:** open
- **Kind:** measurement and implementation gaps; hardware/tool access limits
- **Affected work:** [ML-KEM/ML-DSA assessment](../research/pqc_cross_platform_20261030/README.md)
- **Next action owner:** PQC implementation workstream
- **Last updated:** 2026-10-30

## Problem

Cross-platform speed improvements and side-channel resistance are not yet
established because target-native benchmark and leakage evidence is missing.
The request covers ML-KEM and ML-DSA across platforms, including portable and
accelerated paths. Compilation and functional tests cannot establish absence
of timing/cache leakage, power/EM leakage or fault susceptibility.

## Evidence and uncertainty

- **Observed:** source baseline `fc9af16fa1ce0654ee9df3ce63101c487ed215fb`
  contains reference/optimized ML-KEM-512/768/1024 and ML-DSA-65. The focused
  harness passes 86 existing tests before the encoder change and 88 after it.
  Five cross-target test compilations pass. The integrated release-library
  check fails on unchanged non-PQC syntax errors, reproduced on each base
  with its original encoder. The publication base's tested PQC/hash/RNG
  sources are unchanged; see the assessment's exact revisions and logs.
  The source assessment links the relevant standards, historical profiles,
  secret-derived control flow and existing proof limitations.
- **Derived or proved:** no end-to-end constant-time or physical side-channel
  theorem has been established. The encoder change removes one source branch;
  correctness checks preserve the encoded bytes.
- **Suspected explanation:** historical profiles identify potential performance
  costs; their relevance to the current source and each target is unmeasured.
- **Missing capability:** qualified PQC benchmark image/spec for the documented
  ecbench-based GitHub EKS route, or an authenticated custom-command client;
  native ARM/RISC-V/Windows/macOS execution and device-specific leakage tooling.
- **What this evidence does not establish:** speedup, a demonstrated key-recovery
  attack, an absence of leakage, full external FIPS interfaces, general embedded
  support, or runtime validation on cross-compiled targets.

## What would unblock it

- [ ] Identify the actual supported CPU/OS/compiler/backend matrix and exercise
  ACVP, malformed-input and differential correctness checks on each native target.
- [ ] Qualify the native PQC benchmark submission route with source/image/binary
  identities, output equality, L2/A/A admission and independent replay. Retain
  matched external-reference results and per-operation regressions as well as gains.
- [ ] Audit emitted code and run target-native timing/cache tests with explicit
  secret/public inputs and working positive controls. Keep statistical leakage
  tests separate from proofs and specify exactly which builds they cover.
- [ ] Define and evaluate physical devices/attacker models before claiming
  power/EM or fault resistance. Include secret-lifetime countermeasures and
  target-specific limitations.

These conditions establish bounded evidence for the tested targets. They do
not prove resistance on arbitrary devices, compilers or future platforms.

## Next discriminating step

- **Action:** qualify a source-bound PQC benchmark image/client and freeze the
  first x86 workload before making a performance change.
- **Inputs and controls:** public ACVP/known-answer fixtures for all three
  ML-KEM sets and ML-DSA-65; stateless and prepared APIs separately; baseline,
  identical-copy A/A, interleaved candidate and same-input external reference.
- **Execution limits and stop condition:** follow the existing Iso Lab resource,
  isolation, checkpoint, submission and evidence rules. Stop on output mismatch
  or failed admission; retain failed records. No run is authorized by this card
  independently of the user's implementation task.
- **Outcomes:** admitted correct measurements allow a per-target comparison;
  noise, a loss, failed admission or unavailable access remains a recorded gap.
  Passing correctness alone does not promote a speed or leakage result.
- **Recheck trigger:** qualified client/image, new native platform access or an
  affected source/compiler change.

## Resolution and history

Unresolved. The 2026-10-30 source assessment and first encoder fix do not close
the cross-platform performance or side-channel requirements.
