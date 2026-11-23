# Weighted Weil decomposition lacks complete native pricing and a rho reference

- **Status:** open
- **Kind:** implementation gap; measurement gap
- **Affected work:** [native weighted study](../research/weil_weighted_decomposition_20261019/README.md), [protocol](../research/weil_weighted_decomposition_20261019/PROTOCOL.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-19

## Problem

We cannot establish an end-to-end weighted-slot IC speedup because the
native pair solver does not emit complete work counters and this cubic
field workload has no matched one-target ecbench rho measurement here.
The current suite uses prime-order synthetic cubic-extension curves,
base primes 31 through 1009, coefficients 1/2/4, two construction seeds
and identical public targets within each curve.

## Evidence and uncertainty

- **Observed:** the [primary run](../research/weil_weighted_decomposition_20261019/runs/native-20261019-primary/records.jsonl) completed all 24 planned recoveries and 11,370 independent PDP decisions. Its byte-identical replay and PARI fixture checks are archived beside the study reports.
- **Derived or proved:** every returned decomposition satisfies its group equation; replacing the final factor by a known scalar multiple adds no new factor-base log unknown. No complexity exponent or performance theorem is established.
- **Suspected explanation:** more distinct weighted sums reduce permutation collisions; its quantitative causal contribution is not isolated by this suite.
- **Missing capability:** complete polynomial add/divide/lookup, RNG/allocation and reference/setup accounting; a compatible ecbench cubic-field instance and strong rho reference; pinned pricing conversions; independent-host replay; external algebraic solver measurements (Magma/msolve/FGb were not run).
- **What this does not establish:** a full-cost gain, a win over rho, publication novelty, cryptographic-size recovery or transfer to binary fields.

## What would unblock it

- [ ] Emit complete native counters for cold setup, relation collection,
  linear algebra and recovery, with explicit validation costs and peak memory.
- [ ] Run matched one-target ecbench IC and strong rho on exactly the same
  public fixtures, using the required pricing band and verified scalar output.
- [ ] Replay on another host and compare the strongest available external
  algebraic solver, or explicitly bound that missing comparison.

These checks would allow evaluation of a performance claim; passing
correctness alone is insufficient. A gain must be measured, not assumed.

## Next discriminating step

- **Action:** instrument the existing native S3 solver and port one frozen
  cubic-field fixture into ecbench before collecting a full matched panel.
- **Inputs and controls:** committed curve models, targets, factor bases and
  seeds; c=1 reference, c=1 A/A, c=2 and c=4; independent dictionary checks.
- **Execution limits:** preserve the study's 10,000-residual and
  5,000,000-pair-query stops until a new protocol is frozen.
- **Outcomes:** complete counts permit pricing; an observed regression is
  retained; absent counters or comparison measurements remain inconclusive.
- **Recheck trigger:** native accounting/instance support becomes available.

## Resolution and history

Unresolved. The tracking PR and verified stage experiments do not resolve
the complete-cost research question.
