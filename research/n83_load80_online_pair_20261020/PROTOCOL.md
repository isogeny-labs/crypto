# N83 load80 one-target online comparison

This protocol measures the same-point online time of K4405, K4500, and K4549
load80 factor bases against a separately verified rho reference on the exact
a=1 curve `EC1N83Ckb1a1haacb69ee9f37`. K4500 joins after its completed
capacity control reported 0.855501 ms on the earlier public p00 point; the
two points in this protocol were sealed before that result and remain unseen
by these solvers. The primary result is each arm's first-point
`rho_online_ms / IC_online_ms`. K4500 and K4549 reverse order on the second
point. Cold base, root-index, and rank construction remain recorded in each
IC child and outside its target online interval.

## Frozen inputs and execution

- [`protocol.json`](protocol.json) fixes the current RunPod Pod, four exposed
  CPUs, source commit `f554dc9379462d47243b5b06e8ee28a2d895a131`,
  implementation flags, two public target points, algorithm order, caps, and
  source-file hashes. Its SHA-256 is
  `a913da83dd35b558f96e6b8b70a62d1eb606d798e1fde51da421db20092d2091`.
- [`fixture.json`](fixture.json) fixes deterministic scalar generation and
  point encodings. [`fresh_exclusion_audit.json`](fresh_exclusion_audit.json)
  checks both points against 33,336 distinct points in the pinned public
  inventory; the overlap is zero. The answers are verification sidecars and
  do not enter the solver input.
- Run three fixed rho calibration children before and after each point's
  comparison. For p00, run K4405 IC, K4500 IC, K4549 IC, then rho. For p01,
  run rho, K4549 IC, then K4500 IC. Every IC invocation builds an empty-cache index
  from source, then solves its one supplied point. Each rho invocation uses a
  fresh process, the frozen configuration, and its own seed. The calibration
  target is separate from the two comparison points.
- The producer has a 65-minute global cap, a 1,200-second cap per IC child,
  120 seconds per rho child, a 24 GiB child RSS cap, and a 15-minute reserve
  for verification, transfer, and provider retirement. The provider has a
  scheduled stop at `2026-10-20T08:57:30Z`; this comparison uses the already
  allocated $0.13/hour Pod and does not increase its four-hour spend upper
  bound.

## Admission and interpretation

Dispatch occurs only after the six-run capacity diagnostic records a complete
summary and its K4405/K4500/K4549 load80 p00 controls have passed process,
group-output, fixed 10 GiB root-table, and resource checks. Source,
protocol, target, binary, cgroup, affinity, and provider checks run before
the first child. A failed or capped child remains in the output and stops the
schedule; the aggregate result stays `BOUNDED_UNKNOWN`.

For each completed arm and point, independently replay the scalar and four-factor-base
point sum, compare the exact point and subgroup, verify the rho receipt and
scalar, and check the IC phase sum. The online intervals are the executable's
target-dependent `target_ms` and rho's `online_ms`; retain the cold process
wall time, excluded fixture preparation, calibration receipts, cgroup deltas,
and resource failures separately. Record p00 and p01 ratios separately and
name the candidate IDs only after their full immutable base manifests are
verified. The prior 1,536-point secondary screen remains `BOUNDED_UNKNOWN`;
this two-point study does not use that screen to choose a winner.
