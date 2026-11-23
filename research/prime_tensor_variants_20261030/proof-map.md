# Claim-to-proof map

The actual Lean declarations, exact quantifiers and hypotheses are listed in
[lean/PROOF_MAP.md](lean/PROOF_MAP.md) and the versioned
[integer-slot addition](lean-digits/PROOF_MAP.md). Checked means built at the pinned
toolchain, inspected with `#print axioms`, audited transitively by the shared
gate and replayed in the official fresh Lean kernel. It does not mean the
entire contraction theorem has been formalized. No declaration assumes the
global research conclusion as an axiom or a circular hypothesis.

| Report claim | Formal declaration or evidence | Status / concrete remaining formal obligation |
| --- | --- | --- |
| T2: order-three action and inverse | `rot_thrice`, `rot_invRot`, `invRot_rot` | Checked over every `AddCommGroup`; separate evaluated identity record in the study root |
| T2: digit action composes | `pairAction_add`, `tagAction_add`, `slotAction_add` | Checked for actual `ZMod 3` digits |
| T2: selected-component row/column locality | `TernaryPacket.digit_locality`, `slot_locality`; `TernaryDigits.bounded_integer_slot_locality` | Checked for arbitrary labels over an `AddCommGroup` and actual distinct signed integer slots `0<=x,y<3^a`; the new theorem proves the nonzero leading digit and constructs its `Fin a` coordinate |
| T2: integer valuation/digit bridge and finite index | Eight `TernaryDigits` theorems, especially `digit_difference_at_valuation`, `leadingDigit_ne_zero`, `valuation_lt_of_bounded_slots` | Checked using actual Mathlib `padicValInt 3`; exact signed-integer division, nonzero difference and range hypotheses are in the addition's proof map |
| T2: uniform positive reached fibers and total slots | `orbitFiberEquiv`, `orbit_fiber_uniform`, `orbit_fiber_pos`, `slot_count` | Checked; exact orbit-size classification and `L/|orbit|` padding construction remain unformalized |
| T2: fixed points over F67 | `rot_fixed_iff`, `invRot_fixed_iff`, `tagAction_one_fixed_iff`, F67 specializations | Checked; the general results expose `NoThreeTorsion` |
| T2: graph variance, common load, port marginals | Written packet/global reviews; native finite packings | Open formal graph/probability/port definitions and structural bounds; finite matching units violate full recursive degree admission |
| T3: Cauchy support, integral local charts | Written precision/global reviews; exact native chart checks | Open formal coefficient extension, integer support and valuation/integrality proofs |
| T4: tapered Hermite recovery with precision buffers | Written review; full fixed one-level reconstruction | Open formal rational Hermite identity and coordinatewise error bound for arbitrary parameters |
| T5: target-specific modes, `18M/D`, group factorization | Written reviews; exact local negative control and one-level child sums | Open formal ports, coefficient-extraction identity and weighted mode enumeration |
| T6: canonical creation-precision discipline | Written global review | Open formal immutable entry descriptions/lifts; miniature has only one active level |
| T1/T6: complete deterministic complexity and polynomial space | Written global argument | Open full recursive machine/model and cost/space formalization; no general native implementation |
| T7: application to prime-field ECDLP | Existing pinned paper plus written scope discussion | Open formal reduction/complete cost; inherited reduction is not claimed original |
| Sharper taper claims | [Taper review](reviews/taper.md) | Written analysis; no Lean theorem supplied for band enumeration, taper maximum principle or arbitrary-node symmetrization. Full claim certification remains open |
| Product deformation/code discussion | [Route notes](route-decisions.md), [counterexamples](reviews/multivariate-code-correction.md) | Written scoped arguments; no formal general mode-rank bridge. Proposed universal occupancy inference is explicitly withdrawn |
| Scalar verifier and rational-map restrictions | [Scalar](reviews/compact-scalar.md), [division](reviews/division-polynomials.md) reviews | Written constructions/bounds; no Lean circuit-size/copy bridge or algebraic-geometry formalization supplied |
| Heavy-predicate restrictions | [Main](reviews/heavy-predicates.md) and [independent](reviews/heavy-variance-independent.md) reviews | Written ordered-pair variance/incidence proofs and sparse-packet example; graph probability, tuple support and aggregation remain unformalized |
| Dense Fourier width | [Width review](reviews/fourier-width.md) | Written fixed-representation cut argument; formal weighted-tree separator and graph/path boundary bridge remain open |
| Matched-level origin recovery and moment bounds | [Matched-level analysis](reviews/matched-level.md) and its independent notes | Written application of source D.5 plus scoped slot/moment counting; no Lean multivariate Hermite or recursive complexity theorem supplied |
| Levelwise binomial parity, origin and node divisibility | Nine `LevelwiseBinomial` declarations in [the exact map](lean-binomial/PROOF_MAP.md) | Checked for all signed integers using actual `Ring.choose`, rational polynomials and `padicValInt`; full chart-coefficient integrality/divisibility, degree schedule and data-collision theorem remain unformalized |

The formalization effort prioritized the ternary action, actual bounded-integer
structural locality, reached-fiber equivalences and field-specific fixed-point
assumptions. There are 28 unique checked theorems: 20 in the unchanged original
module and eight in the versioned addition. Both projects pass builds, audits,
the shared gate and fresh replay. Failed Lean attempts are retained. The checked
statements are explicitly separated from the stronger written results; the
global theorem and the other scoped analyses remain open at the formal gate.

The binary counterfamily project adds nine different supporting theorems, for
37 named declarations across the study. This count does not certify the full
counterfamily or a contraction algorithm; its exact missing obligations are
listed separately. The original 20-theorem source is copied, unchanged, in
the digit project, and is not counted twice.

The formalization uses only `propext`, `Classical.choice` and `Quot.sound`.
Ordinary finite `decide` for digit arithmetic/F67 torsion is kernel checked;
there is no `native_decide`, `sorry`, admitted theorem or custom research axiom.
