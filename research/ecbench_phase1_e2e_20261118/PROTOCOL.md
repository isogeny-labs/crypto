# Protocol: Phase-1 cold ECDLP qualification on K1-23 (2026-11-19)

Frozen before any execution of the spec below. Class: **baseline
qualification** for the phased ecbench program; no end-to-end speedup or
security claim. Every eventual result is `[pcc:unverified]` until the repository
PCC process gives the exact claim standing.

The accepted plan is `ECS1hbe36807b8450`, 1,344 executions. It was expanded by
the exact changed-source release binary with SHA-256
`9e81bc233220aecb3e0bb9aac03e18c099d4b08abd7a22ef4fbb311d07c27103`.
That local build is a qualification receipt only. The measured run must use an
immutable image built from the published exact commit and must record its own
binary hash.

## 1. Question and inherited evidence

Does the best previously measured cold generator-orbit index-calculus layout
beat the strongest fully accounted single-target rho reference on the
registered K1-23 cell?

The inherited PR #4651 audit measured `gen-c4 / rho-v5` as 3.198 / 3.735 /
6.050 at band weights 0 / 0.1 / 1. Those are prior L0 results, not results of
this protocol. They make `gen-c4` a baseline likely to be pruned; the purpose of
this qualification is to establish the current exact-source denominator and
failure margin without selecting only favorable targets or costs.

## 2. Frozen workload and arms

Spec: `specs/k1-23-qualification.json`.

- Curve: K1-23, `icv1-f2m23-tm5197-1f85e9e1`, registered EC1 identity
  `EC1N23Ce1heec6a3619e3f`, prime subgroup `r = 4196903` (`log2 r = 22.00`),
  cofactor 2 and signed-Frobenius automorphism count 46.
- Targets: 32 public targets, seed `202611190501`; no planted logarithm is
  available to an arm. Every recovered scalar is checked by `[d]G = Q`.
- Measurement: six measured rounds, one warmup, alternating order, algorithm
  seed `202611190502`, L2 required, 3,600-second execution timeout.

| Arm | Method and role |
|:--|:--|
| `rho-v6`, `rho-v6-aa` | `rho.signed_frobenius_v6`; counted matched reference and A/A wall control |
| `rho-v5-trajectory` | frozen v5 identity; proves v6 changes only the partition-mix ledger |
| `rho-strong-v2` | independently implemented strong walk with scaled, exactly counted setup |
| `rho-strong` | historical strong reference and implementation-diversity control |
| `gen-c4` | cold `ic.pipeline`, generator-orbit compact scan with four columns and counted Frobenius-layout m=2 oracle |

No arm, target, seed, round count, timeout, cost weight or gate may change after
the first execution. A changed design gets a new spec identity and is reported
as a separate round.

## 3. Execution and custody

1. Commit and push this protocol, spec, source and registration hashes to PR
   #4785. The cloud build must name that exact published commit.
2. Before spending, preserve a complete `.resume/RESUME.md` checkpoint, verify
   live account budget and `cpu-l2-spot` capacity, and save the publication and
   pool receipts. One c8a.2xlarge Spot attempt, at most 4,800 seconds, 8 GiB,
   zero automatic or agent retry, and a USD 5 all-in attempt ceiling. A failed,
   timed-out or ambiguous attempt is evidence, not permission to resubmit.
3. Run through the maintained EKS ecbench contract. Success requires the exact
   image/source/binary match, accepted all-record L2 gate, immutable Job and Pod
   identities, S3 payload hashes, and A/A admission.
4. Run `ecbench verify --replay-all`, `ecbench band --weights 0,0.1,1 --ranks`,
   and paired `ecbench compare` for `gen-c4` against `rho-v6`,
   `rho-strong-v2`, and `rho-strong`, plus `rho-v6-aa / rho-v6` and
   `rho-v5-trajectory / rho-v6`. Use 4,000 target-clustered resamples and seed
   `20261002`. Preserve method-call and process-wall fields even if the small
   solve is correctly rejected as too short for an L2 wall claim.
5. Obtain an independent-host replay receipt before promoting any result.

## 4. Predeclared reads and gates

- **Q0 — accounting identity:** on every paired v5/v6 execution, recovered
  scalar, exhaustion state, charged phases, GAE and all counters except
  `partition_hashes_uncharged` are identical. The v6 counter is positive and at
  least `walk_operations`. Failure stops the round.
- **Q1 — correctness and completeness:** every measured solve in every arm
  verifies, the session is complete, all 1,344 planned executions are present,
  and replay-all reports zero problems. Failure stops the round.
- **Q2 — controls and isolation:** operations A/A is exactly 1; the admitted
  wall A/A interval contains 1; every result used for wall conclusions reaches
  L2. A sub-millisecond blocker is retained and means no Phase-1 wall claim.
- **Q3 — strongest reference:** the denominator is the cheaper admissible arm
  between `rho-v6` and `rho-strong-v2` at each predeclared band weight. The
  historical strong arm is still reported; it cannot substitute for a cheaper
  valid reference.
- **Q4 — cold promotion:** `gen-c4` promotes only if its paired cold ratio has a
  95% upper bound below 1 against both `rho-v6` and `rho-strong-v2` at all three
  weights. Any lower-bound or unpriced-work caveat blocks promotion. Otherwise
  it is pruned with its observed margin.
- **Q5 — phased-program promotion:** passing Q0-Q4 qualifies only this K1-23
  cell. It permits work on the other mandatory Phase-1 traits; it does not
  establish a general ECDLP speedup. A full claim additionally needs the
  remaining Phase-1 cells, held-out later phases, complete cold and online cost
  accounting, admitted wall evidence at sizes long enough to time, and the
  repository's independent replay and signed PCC gates.

## 5. Stop conditions and non-claims

Stop on source/image mismatch, unavailable budget or pool receipt, first
terminal EKS result, any Q0/Q1 failure, or the finite deadline. Do not weaken a
reference, remove failed targets, lower rounds, turn an error into a survivor,
or extrapolate this 22-bit cell to 60/80-bit curves.

The relation-yield pruning instrumentation already merged separately; this
protocol neither reclaims it as a speedup nor treats pruning telemetry as a
complete ECDLP solve. No end-to-end ECDLP speedup is established at freeze time.

