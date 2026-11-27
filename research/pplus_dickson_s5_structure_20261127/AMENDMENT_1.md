# Prospective amendment 1: close v1 replay-coverage gaps

This amendment is frozen after preserving the first bounded packet and before
editing or rerunning the candidate, verifier or analyzer. It changes no field,
curve, depth, Dickson root, target, target order, zero predicate, promotion
threshold or stop rule from `PROTOCOL.md`.

## Status of the first packet

The packet in `evidence/` passed all arithmetic comparisons implemented by the
v1 verifier: its separately constructed resultant matched every emitted kernel
entry and the signed witness sets matched. However, a protocol-to-source audit
found three missing proof obligations:

1. the candidate emitted multiplication-channel ranks and a self-check boolean,
   but not the frozen channel matrices and their sum for independent comparison;
2. the divisor recurrence covered every distinct divisor of degrees one through
   four, while the evidence did not bind that recurrence and direct `S_5` zeros
   to each of the five multiplicity partitions for every target; and
3. the analyzer omitted the required unordered-pair count and the explicit
   comparison between matrix singularity/rank deficiency and an actual zero
   entry.

Therefore the v1 line `exactness_ok=1` describes only its implemented checks.
It does not meet the complete frozen algebra/structure pass and is superseded by
the evidence status `COVERAGE_INVALID_STOP`. [pcc:unverified]

## Required v2 repair

Before another execution, v2 must:

- emit and independently compare every entry of the difference channel,
  sum/reflection channel and total multiplication matrix through `n=64`;
- enumerate all four-root multisets at `n=4,8,16`, classify each as `4`, `3+1`,
  `2+2`, `2+1+1` or `1+1+1+1`, and for every target independently compare the
  state count, divisor recurrence result and direct `S_5` zero count;
- emit `N`, `N^2`, the unordered-pair and four-multiset counts, and whether each
  singularity/rank-deficiency predicate agrees with the actual zero predicate;
  and
- make the verifier reject a missing record or mismatched field and make the
  analyzer condition its verdict on this strengthened verifier.

The first packet remains immutable. V2 uses a new source manifest, runner and
`evidence-v2/` directory. A v2 failure is preserved as `INVALID_STOP`; it may not
trigger a changed target or a third run.
