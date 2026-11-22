# Lean status of the no-subexponential-ECDLP goal

Date: 2026-11-18
Status: the implication from semantic rank premises is refuted by a
kernel-checked countermodel; the universal ECDLP lower-bound goal remains open

## Requested goal

The requested conclusion is that no correct ECDLP solver has subexponential
binary work.  The Lean definition deliberately takes the predicate
`SolvesECDLP : (Nat -> Nat) -> Prop` as a parameter: defining real algorithm
semantics, correctness on every admitted prime-field curve, and a complete
cost model is part of the missing lower-bound theorem, not notation that can
be assumed.

For a natural-valued work function `W`, the formal operational criterion is

```text
IsSubexponentialBinaryWork W :=
  for every a,b > 0, eventually W(n)^b < 2^(a*n).
```

This expresses growth below every positive rational exponential rate.  The
formal goal is

```text
NoSubexponentialECDLPGoal SolvesECDLP :=
  for every W, SolvesECDLP(W) -> not IsSubexponentialBinaryWork(W).
```

## What Lean establishes

The pinned project kernel-checks two complementary statements.

First, the sufficient bridge is explicit:

```text
(for every W, SolvesECDLP(W) -> HasPositiveExponentialLowerBound(W))
  -> NoSubexponentialECDLPGoal(SolvesECDLP).
```

This is theorem `noSubexponentialECDLPGoalOfExponentialLowerBound`.  It reduces
the requested goal to the substantive obligation: establish a positive
exponential work lower bound for every correct ECDLP solver in the selected
machine and cost model. Claim `PCC-20261118-63eb26` binds this theorem; its
local verifier passes and signed standing is unverified.
[pcc:unverified]

The endpoint `linearRankBridgeNotSubexponential` connects this definition to
the earlier registered architecture chain.  If a family has a positive-slope
exponential rank floor, every admitted exact computation factors through the
linear separator, and work is at least message dimension, then that named
backend's work is not subexponential.  The semantic family does not satisfy
the required rank premise, and the low-level BTNIR family premise remains
open. Claim `PCC-20261118-ed3d35` binds this endpoint; its local verifier
passes and signed standing is unverified. [pcc:unverified]

Second, theorem
`semanticRankPremisesDoNotEstablishNoSubexponentialECDLPGoal` refutes the
attempt to obtain the goal from the current semantic premises alone.  Its
one-element-seed countermodel has zero rank and zero solution count, satisfying

```text
rankCount(seed) <= solutionCount(seed),
sum_seed solutionCount(seed) <= 2 * number_of_seeds.
```

The independent solver predicate accepts constant-one work, which Lean checks
is subexponential.  Therefore no theorem quantified only by those semantic
rank/count premises can establish the requested conclusion for an otherwise
unconstrained solver predicate. Claim `PCC-20261118-06e657` binds this
countermodel; its local verifier passes and signed standing is unverified.
[pcc:unverified]

The constant-one predicate is a logical countermodel, not a claim that a real
constant-time ECDLP algorithm exists.  Its role is to show exactly which
connection is absent: a theorem forcing every correct ECDLP solver to expose
the registered boundary matrix, an exact linear message, and corresponding
work.

## Why the broader goal remains open

The existing checked chain applies only after all of these hypotheses are
supplied:

```text
uniform family rank floor
  -> exact linear separator dimension floor
  -> work floor for the named backend.
```

It does not quantify over arbitrary algorithms.  In addition, the semantic
slot matrix cannot provide the first premise uniformly over all source seeds
or with nonnegligible seed probability: its expected rank is below two.  The
low-level BTNIR edge matrix is representation-specific and has no established
rank-equivalence with the semantic matrix.

Consequently, the Lean development disproves the proposed implication to the
universal goal; it does not disprove the existence of a real subexponential
ECDLP algorithm. [pcc:unverified]

## Requirement status

| Requested item | Status | Evidence or exact gap |
| --- | --- | --- |
| State the no-subexponential-ECDLP goal in Lean | Complete at parameterized-model scope | `NoSubexponentialECDLPGoal` |
| Prove the sufficient asymptotic bridge | Complete at locally kernel-checked scope | `noSubexponentialECDLPGoalOfExponentialLowerBound` |
| Decide whether semantic rank/count premises imply the goal | Refuted at locally kernel-checked scope | `semanticRankPremisesDoNotEstablishNoSubexponentialECDLPGoal` |
| Establish the universal real-ECDLP lower bound | Open | Define the admitted algorithm/cost model and prove every correct solver has a positive exponential work floor |
| Cover nonlinear, randomized, approximate, alternate-encoding, or separator-avoiding solvers | Open | No universal solver-to-rank/work theorem exists |

## Reproduction

The declarations are in
[`lean/TensorBoundaryBounds.lean`](lean/TensorBoundaryBounds.lean).  Their
six-field instance analysis and inheritance table are in
[`../ic_yield_rank_frontier_20261118/PROOF_MAP.md`](../ic_yield_rank_frontier_20261118/PROOF_MAP.md).
The project pins Lean 4.35.0, Mathlib, and `crypto-lean`; validation commands
and axiom receipts are recorded in [`VALIDATION.md`](VALIDATION.md).
