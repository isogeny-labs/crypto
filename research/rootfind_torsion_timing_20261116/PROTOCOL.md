# K13 torsion-harvest complete-call local diagnostic

Frozen on 2026-11-16 before any execution of this spec. This is an L0
engineering diagnostic for choosing the next implementation step. The frozen
ten-arm EKS L2 F4/F5/F6 study in
`../rootfind_f456_batch_combined_20261115/` retains its own source, arms,
decision rule and independent-replay obligation. A local observation under
this protocol cannot be pooled into or substituted for that study.

## Question and mechanisms

On four fixed public targets of the registered Koblitz E0 curve over
GF(2^13), does the opt-in torsion-row relation-harvest engine have lower
complete fresh-process wall time than the preceding collision-harvest engine?
How much of each arm's process time is inside the method call, and where does
F6-IC lie on the identical targets? The candidate computes the order-two
first table row from batched field inverses and seeds one torsion relation;
the prior harvest arm performs those group additions and verifies every
duplicate-key witness. The explicit A/A arm repeats the candidate method.
The prediction is narrower than an end-to-end performance claim: the
candidate's method-call wall may fall while cold-process wall remains within
A/A noise because process startup is large. [pcc:unverified]

## Fixed source, spec, inputs and controls

- Code ancestor: `f24788b1b0a2ec5ff0857bb96006b2a7c1e4c3f1`, whose
  source includes the opt-in engine. The protocol/spec/analyzer freeze commit
  and exact release binary SHA-256 will be recorded before the run. No code
  edits after the release build are admitted.
- Spec: `SPEC-K13-LOCAL.json`, SHA-256
  `8a40c31be62b0472e83f6316c76138bf6adb3e5f6b9c7f71af1227301c260457`.
  Analyzer: `analyze.py`, SHA-256
  `810ce94540c80c1669450c4039b776791602b642e0caea09ab01c0eb471059e2`.
- Exact curve: registered Koblitz E0/GF(2^13), subgroup order 2003,
  cofactor four, standard-subspace dimension five with signed Frobenius-orbit
  columns. The curve's full ICV1/EC1 identities, generator and each public
  target point come from the runner's `plan.json` and must match across arms.
  The required curve UID is
  `urn:ec-record:1:sha256:1dfa32cdf623e34fb258128415cab472abaa51695380fa5d4f5d636ed5a3da60`;
  the factor-base SHA-256 is
  `cc3c0ee034174a528849ca7efd95d70459404f6cc1e9361f0b4da7f6c3da2ae3`.
  Four public targets are the first four indices under target seed
  `202611151301`; all arms share the runner's per-target-round algorithm seed.
- Four arms: `f6-ic` m=3 degree=3 node budget 8192,
  `rootfind-negfold-batch-harvest` m=3,
  `rootfind-negfold-batch-harvest-torsion` m=3, and an independent A/A copy of
  the torsion arm. Every arm uses the same base, `max_trials=200000`, eight
  measured rounds and one warmup, alternating order. The plan has
  4 targets × 4 arms × 9 rounds = 144 executions, including 128 measured.
- Positive control: each answer must be marked `verified`, with
  `matches_target=true` for its public `[d]G=Q`. Require identical target
  point, registered curve, workload hash and algorithm seed within every
  four-arm target-round group. Keep every failure, timeout and censored row.
  The A/A pair records process-level noise on this host. [pcc:unverified]

## Measures and reading rule

Primary local diagnostic: `time.process_wall_ns` from fresh process through
the checked answer. Also retain `method_call_wall_ns`, `solve_wall_ns`, every
phase wall, total GAE, unpriced native work, peak RSS, CPU/load/isolation,
source and binary hashes. Report `process_wall_ns - method_call_wall_ns`
as a labeled residual, not as an independently measured stage. Report all
32 target-round groups, failed/censored arms and warmups; no arm is selected
after seeing data.

`analyze.py` requires all four arms, five IC phases, verified answers,
identical workload and algorithm seed, positive wall fields and L0 isolation
on every measured group. If any group or warmup is missing/invalid, it sets
`complete=false`, preserves reasons and withholds paired intervals. If
complete, report each arm's median wall and three ratios of summed wall:
torsion/harvest, torsion/F6 and torsion-A/A over torsion. Use a target-cluster
bootstrap with 5,000 resamples, seed `202611160917`, sampling four target
indices with replacement while keeping all eight rounds of each sampled
target; show percentile 95% intervals for process and method-call wall.
These intervals are exploratory L0 host-noise diagnostics, not admissible L2
confidence intervals or an attack-cost claim. [pcc:unverified]

If torsion/harvest process wall is not separated from the A/A interval, the
next implementation target is cold-process overhead or higher-cost relation
collection; preserve the method-call observation but do not call the complete
solver faster. If torsion is separated beyond A/A noise, include it as a
candidate in a new frozen L2 comparison after the account budget gate clears.
If the F6 relation changes by target or any answer fails, investigate the
specific fixture or implementation and do not infer a performance order.
No outcome here promotes a K13 or larger-curve performance claim. [pcc:unverified]

## Execution and custody

Build release `ecbench` with Rust 1.99.1 from the clean frozen source and
record its SHA-256. Validate the spec with `ecbench plan --doc`. Run the
same binary once with `ecbench run --spec SPEC-K13-LOCAL.json --out
sessions/local-diagnostic-01 --cpus none --allow-busy --settle 2`; this
explicitly requests L0 and retains busy-host telemetry. The runner's
per-execution cap is 120 seconds; an outer 600-second watchdog stops the
session without restarting it. Use a new output directory for any later
attempt and a new protocol record for any changed samples or arms. No cloud
image or EKS job is authorized by this local diagnostic; the lab budget gate
continues to bind the L2 panel.

Before launching, push a complete S3 resume checkpoint and pass
`scripts/checkpoint.sh gate`. After the session, run `ecbench verify
--replay-all`, preserve the raw session, replay receipt, analysis JSON,
commands, stdout/stderr and exact source/binary/host metadata. Produce a
read-only report with an editable graph and inspected PDF. Publish the
records on the existing rootfinding PR. Keep claims tagged unverified until
independent replay and the required L2 comparator panel exist.
