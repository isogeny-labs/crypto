# Relation yield on a twist and a conditional IC cost theorem

2026-10-24 · `yield-20261024` · PR #2381.

[Inspected PDF](report.pdf) · [editable report](report.tex) ·
[relation/cost diagram](bridge.svg) · [exact-count control](raw-yield.svg) ·
[validation and failed attempts](../../validation/yield-20261024/README.md).

The user asks for a Lean theorem relating twist and main-curve relation yield,
and showing how higher yield and lower degree of regularity could make index
calculus faster. The result has three parts: exact transport invariance, an
exact whole-group yield comparison, and conditional cost inequalities. It does
not assume that twisting automatically improves either useful yield or regularity.

## 1. Exact relation sets and their rank

For a finite indexed factor base B and a finite set C of integer coefficient
vectors, define

`R(B,C) = { c in C : sum_i c_i B_i = O }`.

An inhomogeneous relation can be expressed by including its target point as
another indexed column. If f:G→H is an injective additive homomorphism, Lean
proves **R(f(B),C)=R(B,C)** as coefficient sets, not just an equality of counts.
Thus count/|C| is equal. The same useful-row predicate gives the same useful
yield. Over every field F, the span of the accepted coefficient vectors and
its finite rank are equal too; F may be the prime residue field used for IC.

`RelationYield.twist_relationRows`, `twist_rawYield` and `twist_rowRank`
instantiate this on the actual previously proved short-model map
psi: E_d(k)→E(K), `(X,Y) -> (X/d,Y/(d*u))`, u²=d, d≠0, and elliptic E.
The target is the mapped twist subgroup in **E(K)**. This is not an assumed
isomorphism between the two rational groups E_d(k) and E(k).
Simply transporting a matched factor base therefore cannot create a yield or
rank advantage. Different ground-field constraints, candidate distributions or
factor bases are different comparisons and must be studied separately.

## 2. A genuine whole-group yield relationship

For a finite group G of size N and an ordered prefix of length m, every prefix
has exactly one final point making its sum zero. Lean constructs that bijection.
Consequently the full relation count is N^m out of N^(m+1) candidates, and the
exact raw yield is **1/N**. This includes m=0; the formal statement here
counts zero-sum tuples.

For main/twist rational groups with actual orders N and Nt:

`raw_yield_twist / raw_yield_main = N/Nt`.

A smaller twist group therefore has higher raw whole-group yield. This is
`full_yield_ratio` / `full_yield_comparison`. It is a theorem about uniform
sampling of the **entire** groups, with the same tuple length. It is not a
distribution theorem for a small IC factor base, and raw zero counts are not
probabilities of finding new independent rows. The actual N+Nt point-count
identity belongs to the curve data; this round does not add a general formal
point-counting theorem or assume its conclusion for unmatched inputs.

## 3. When greater useful yield and lower solver degree can help

Let p_X>0 be **useful independent-row yield**, c_X the work per attempt,
m_X the required number of useful rows, and O_X all remaining work:
setup, transfer, linear algebra, descent and verification. Define the model

`C_X = O_X + m_X * c_X / p_X`.

The model uses one consistent work unit. It is neither a measured runtime nor
a proved stochastic completion time. `batch_normalization` proves that for a
finite batch with A>0 attempts, U>0 useful rows and total work W,
`(W/A)/(U/A)=W/U`. Extrapolating that average to future rank states requires
additional assumptions; a raw acceptance rate does not supply them.

The exact break-even theorem is

`C_T < C_E  iff  O_T-O_E < m_E*c_E/p_E - m_T*c_T/p_T`.

For common m, O_T≤O_E, p_T>p_E>0 and c_T≤c_E with c_E>0, Lean proves the
twist budget is strictly smaller. It also proves the complementary case of
strictly lower attempt work and nondecreasing useful yield.

For an explicit dense Macaulay work **model**, put

`M(n,D,omega) = binomial(n+D,n)^omega`.

Here n,D and omega are natural numbers; omega=3 is an allowed dense work
model. The formalization does not use a real matrix-multiplication exponent.

Lean proves M is nondecreasing in D, and strictly increasing when n>0 and
omega>0. At fixed n and omega, lower D and higher useful p therefore satisfy
the sufficient cost conditions. `faster_of_yield_and_degree` and
`faster_of_strict_degree` check those implications.

**Regularity bridge remains explicit:** D is a supplied relevant solver-degree
parameter. The code does not define an actual polynomial ideal's degree of
regularity, certify a particular twist system's D, prove D bounds F4/F5 solving
degree, or identify sparse solver runtime with this dense model. Those steps
need actual equations, field equations/rationality constraints, solver order,
variables, matrix shape, coefficient costs and independently checked work bounds.
Changing the number of variables, matrix sparsity, coefficient field or equation
encoding can change cost even when D is lower. No lower D follows merely from
the twist label. The generic cost theorem allows supplied per-attempt bounds
without identifying them with the dense formula.

To conclude an **actual** pipeline is faster, the separate
`actual_faster_of_bounds` theorem requires independently justified twist upper
and main lower work bounds. To solve the original main-curve DLP using a cheaper
twist problem, one additionally needs a scalar-preserving injective subgroup
reduction (`scalar_solution_iff`) and must charge it in O_T. A geometric twist
isomorphism over K alone does not establish that reduction between E(k) and E_d(k).

## 4. Kernel-checked control: more raw relations, no nonzero rows

The already registered curves over F5 are

- E:y²=x³+x+1, `icv1-fp3-tm3-37e7e540`, with actual point P=(0,1), order9.
- Its d=2 monic twist Y²=X³+4X+3, `icv1-fp3-t3-cec99b6b`, with U=(2,2), order3.

No new registry entries or subgroup representations are fabricated. The existing
model records give orders9 and3 but no EC1 subgroup representations. This is a
known-answer correctness control, not a cross-repository performance comparison.
The concrete coordinates, annihilators and nontriviality are checked on actual
Mathlib nonsingular points.

On the same scalar candidates n=0,...,8, Lean checks:

| Quantity | Main point P | Twist point U |
| --- | ---: | ---: |
| Candidates | 9 | 9 |
| nP=O or nU=O | 1 | 3 |
| Raw zero yield | 1/9 | 1/3 |

Every twist zero has n mod3=0. Hence **zero nonzero coefficient rows modulo3**
survive. The higher raw yield supplies no rank information about that order3
logarithm. No comparison is made between equal-size prime-order DLP instances.
The numerical cost illustrations in `YieldFixture` are separate synthetic
budgets: they show that the inequality can hold, and that overhead can reverse
it. They are not measurements or costs of these F5 curves.

## Proof map and verification

| Claim | Module / declaration |
| --- | --- |
| Same accepted coefficient set | `RelationYield.relation_iff`, `relationRows_transport` |
| Same raw and useful yields | `rawYield_transport`, `usefulRows_transport`, `usefulYield_transport` |
| Same actual coefficient span/rank | `rowSpace_transport`, `rowRank_transport` |
| Actual twist instantiation | `twist_relationRows`, `twist_rawYield`, `twist_rowRank` |
| Scalar-preserving injective reduction | `scalar_solution_iff` |
| Whole-group bijection and count | `completeRelationEquiv`, `full_relation_count` |
| Exact full-group yield and comparison | `full_factor_base_yield`, `full_yield_ratio`, `full_yield_comparison` |
| Useful finite-batch normalization | `RelationCost.batch_normalization` |
| Exact overhead thresholds | `overhead_threshold`, `overhead_threshold_general`, `budget_lt_iff_cleared` |
| Conditional useful-yield/work improvement | `faster_of_useful_yield`, `faster_of_attempt_work` |
| Degree-model monotonicity | `macaulayWork_mono`, `macaulayWork_strictMono` |
| Combined yield/degree implications | `faster_of_yield_and_degree`, `faster_of_strict_degree` |
| Actual work-bound bridge | `actual_faster_of_bounds` |
| F5 control and synthetic model cases | `YieldFixture.lean`, eleven named declarations |

There are **38 new named theorems**, bringing the study to **106 named research
theorems / 127 audited declarations**, including21 generated helpers. The
unmodified repository gate passes all ten modules with zero failing/extra
trusted native axioms. Only `propext`, `Classical.choice` and `Quot.sound`
occur; some declarations need fewer. Lean4.34.1 and Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612` remain pinned. Additional
`lean4checker` replay is unavailable; no such replay is claimed.

The polynomial part of clearing the budget denominators has a new source-bound
**IDC1h1fac46ceb41801ac**, with32+16 native points and arithmetic false-plus-one /
false-source controls. Lean separately checks positivity, counts, rank and
inequalities. The certificate asserts no actual yield, regularity, transfer
or speed improvement. The full native `run-yield.sh` byte replay and its22
backend tests pass. This round changes no curve/field arithmetic implementation.
The final full release library suite passes4,479 tests with119 existing ignores;
the four existing workflow-metadata tests also pass. Failed sparse-checkout
attempts remain in the validation record.

## Scope, provenance and reproduction

Protocol commit `65bea0a7ef8dad84453c30d61bdc9efedee28354` preceded execution.
All failed source/build attempts and final pins/hashes are retained. No old
source-bound Lean module, native result, certificate, historical Python archive
or dated report is modified. The old mixed-relation obstruction and positive
F43 pairing route remain in force; no universal no-connection goal is revived.

```sh
tools/lean/proof-gate.sh --out /tmp/twist-yield-proof research/quadratic_twist_evidence_20261021/lean
bash research/quadratic_twist_evidence_20261021/run-yield.sh /tmp/twist-yield-fresh
```

The repository-wide Lean workflow discovers the expanded existing project;
the scoped native workflow runs the certificate/control harness. The existing
global catalog, strict assessment/provenance dependencies, benchmark history,
leaderboard and conflicted claim/generated files are untouched. There is no new
admissible performance point to add to a canonical cost graph. Shared Conductor
still exits127; no reservation or replacement coordinator is claimed. All
existing CI/access, review, merge-conflict and research completion holds remain.

The ECDLP lead-atlas `--match` was executed before the new mechanism discussion.
Relevant existing leads are `LEAD-EXO-curve-model-representation`,
`LEAD-ICE-chained-semaev-dreg-vs-semiregular`, and
`LEAD-TRF-curve-parameter-transfer-audits`. Their existing scopes/dispositions
are not broadened or overwritten here. The [atlas handoff](../../validation/yield-20261024/atlas-handoff.json)
identifies this proof work for the next atlas review; it is not a new solver or
a changed research disposition. The initial sparse atlas dependency failure is
retained alongside the passing rerun.

Primary executable sources: pinned Mathlib finite-set/cardinality,
`Data/Nat/Choose/Basic.lean` and affine elliptic point/group APIs; the existing
actual twist map in `TwistGroupMap.lean`. No external theorem about F4/F5
regularity or runtime is asserted by this report. The graph plots only recorded
Lean-certified integers; Python is used solely for presentation.
