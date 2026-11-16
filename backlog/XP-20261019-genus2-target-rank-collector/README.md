# L4: Target-useful rank rather than first-cycle genus-2 collection

Status: **draft, not running**. This is an experiment specification, not a measured result. The executable producer/verifier and checkpoints marked TODO in the card do not yet exist.

## Question and decision

Can target-aware relation selection reduce work to a useful dependence in the verified genus-2 transfer pipeline?

In the exact-order cohort 1,244/1,260 solves needed a dependence after the first graph cycle. The proposal is to spend relation and elimination work on target-useful rank, rather than treating graph closure as a solve. This is a pipeline improvement question, with no new exponent claimed.

## First discriminating experiment

Instrument the existing verified 1,260 exact-order targets: label graph cycle, algebraic dependence, sign anchoring, and nonzero target coefficient separately. First replay the incumbent unchanged. Design on separate training targets, then freeze one candidate that prioritizes useful components without inspecting unknown scalars or future draws, and run the full 1,260 held-out cases.

## Controls

Controls: incumbent v2 collector, an equal-cost random selection policy, independent relation trials and paired unchanged draws. Solver receives only P,Q,r. Recompute every recovered kP on the source and charge rejected candidates, data structures, scalar arithmetic and restarts.

## Acceptance and negative outcomes

Pass requires a complete cold paired cost reduction against the incumbent beyond an A/A interval, retaining all failures, and comparison with both negation and plain source rho. Report remaining gap to rho even if this improves IC. No first-cycle stopping shortcut or generic collision fallback may be counted as a relation solve.

## Execution readiness

Freeze exact identities, seeds, protocol, runtime and resource budget before execution. Keep setup, online work and verification separate; preserve complete cold totals and failed attempts. Native Rust performs all mathematics. The card remains draft until code, independent checker and resume path exist; estimates are intentionally unknown. No cloud job has been started for this card.

Source: [https://www.ams.org/journals/mcom/2007-76-257/S0025-5718-06-01854-6/](https://www.ams.org/journals/mcom/2007-76-257/S0025-5718-06-01854-6/). This source motivates the mechanism; the specific proposed experiment above is this search round's design. See the [search report](../../research/new_leads_20261019/README.md).
