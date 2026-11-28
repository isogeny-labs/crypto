# Scaling-audit instance gate (before implementation)

1. Structure above: Mathlib's linear ordered field `Rat`, lattice `max`,
   and Core `Nat` arithmetic. The cost envelope is a proposed scalar model;
   its assumptions are not a theorem about a decomposition algorithm.
2. Laws used as stated: `le_max_left`, `le_max_right`, `le_trans` and
   ordered-field arithmetic; closed values by `norm_num`. Core decidable
   arithmetic supplies the separate concrete modulus/cardinality inequalities.
   No probability, solver-cost or generic complexity law is rederived.
3. Instances: the four-summand model at gamma=1/2 and six-summand model at
   gamma=3/2, with rational base-size exponent beta; actual Curve25519
   scalar order and current torus order for the integer obligations.
4. Assembly probe: anonymous `Rat` max-bound example applies `le_max_left`
   verbatim. Each numeric-model endpoint bounds one max component using
   arithmetic and applies the same order laws. The endpoints do not prove
   that a real algorithm has the proposed exponent.
5. Missing components: the actual success/yield lower bound, complete solver
   and preprocessing upper bounds, sparse-kernel implementation and cost,
   admissible growing family, and a real-power/asymptotic bridge. Generic
   versions of the complexity model would belong in crypto-lean first.
6. Inheritance: rational arithmetic/order/lattice from existing Mathlib
   instances; linear arithmetic discharged by its tactics; natural powers,
   division and order from Core. No new field/group operations or axioms.

Proof sketch: compare beta with the declared kink/balance point. In each
case, a stated max component is at least the target; use `le_trans` into
the full max. Closed substitution establishes attainment. The four-summand
balance is beta=2/9, exponent4/9; the six-summand saturated kink is beta=1/6,
exponent5/12. The unsaturated formula at beta=2/11 is separately evaluated
to expose its missing probability cap.

| Statement | Kind | Owner | Scope |
|---|---|---|---|
| Anonymous max example | Assembly probe | No named generic law | Existing `le_max_left` |
| Four/six lower bounds and attained values | Numeric model instance | Study | Exact rational envelopes only |
| Six naive-balance substitution | Numeric model instance | Study | Shows the cap changes this model value |
| Concrete root/base-size/budget inequalities | Arithmetic instance | Study | Current integer parameters only |

Dependencies: reuse crypto-lean5f2ca251cb34806f69fd5455175434cac3567dfa
and its Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2 / Lean4.35.
These sources already own the arithmetic/order laws. No shared-library edit
is needed for these numeric instances. No limit override or native exception.
