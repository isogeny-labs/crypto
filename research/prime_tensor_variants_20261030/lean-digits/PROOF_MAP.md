# Proof map: valuation-to-digit addition

All new declarations have namespace `TernaryDigits`. The copied
`TernaryPacket` module retains its earlier 20-theorem proof map; its bytes are
unchanged. The following eight statements are the newly reported theorems.

| Statement | Lean declaration | Exact hypotheses and scope |
| --- | --- | --- |
| Digits distribute over adding a divisible integer | `digit_add_of_dvd` | Every `i:Nat`, `x,t:Int`, with `(3:Int)^i ∣ t`; signed Euclidean division |
| Digit difference identity | `digit_difference_of_dvd` | Every `i:Nat`, `x,y:Int`, with `(3:Int)^i ∣ y-x`; zero difference allowed |
| Digit identity at the actual integer valuation | `digit_difference_at_valuation` | Every `x,y:Int`; `i=padicValInt 3 (y-x)` is the actual Mathlib valuation, not an abstract assumed index |
| Maximal-power quotient is not divisible by the base | `quotient_not_dvd` | Every `p:Nat`, `p≠1`, `t:Int`, `t≠0`; no primality assumption, and negative `t` covered |
| Actual leading ternary digit is nonzero | `leadingDigit_ne_zero` | Every `t:Int`, `t≠0`; quotient cast into the actual type `ZMod 3` |
| Integer slots satisfy structural label locality | `integer_slot_locality` | Every `A:Type*` with `AddCommGroup A`, `a:Nat`, arbitrary labels `g:Fin a→Tag A`, signed `x,y:Int`, `y-x≠0`, and `padicValInt 3 (y-x)<a` |
| The finite coordinate bound follows for ordinary packet slots | `valuation_lt_of_bounded_slots` | Every `a:Nat`, `x,y:Int`, `0≤x`, `x<3^a`, `0≤y`, `y<3^a`, `y≠x` |
| Actual bounded packet slots satisfy label locality | `bounded_integer_slot_locality` | Same arbitrary group/labels and bounded distinct slots; the theorem itself constructs `i:Fin a` using the preceding checked bound |

Each theorem has an explicit `#print axioms`. Exact source/module builds,
transitive axiom audit, shared gate and replay results are recorded in the
verification receipts. Only Lean's standard foundations are allowed:
`propext`, `Classical.choice`, and `Quot.sound`.
All these checks passed, including import-inclusive `leanchecker --fresh`.

The central proof uses the actual Mathlib statements
`padicValInt_dvd`, `padicValInt_dvd_iff_of_ne_one`,
`Int.ediv_mul_cancel_of_dvd`, and
`ZMod.intCast_zmod_eq_zero_iff_dvd`. Thus a nonzero normalized leading digit
is proved, and then supplied to the copied `TernaryPacket.slot_locality`;
it is neither assumed nor replaced by a finite certificate about sample slots.

## Formal obligations remaining open

This addition closes the earlier integer-to-digit and `Fin a` index gaps.
It does not close these distinct obligations:

1. Define the Cauchy kernel and prove that its support at an integer node
   consists of slot pairs with that exact difference, then combine that
   support statement with this label-locality theorem.
2. Complete orbit-size classification, exact packet padding, graph-port
   assignments, variance/load bounds and selected-symbol marginal counts.
3. Formalize the necessary target-specific mode split, its factorization,
   and corrected `18M/D` bound.
4. Define the characteristic-three coefficient ring and lifts; prove all
   integrality, Hermite, valuation-loss and precision-recovery statements.
5. Prove parameter admission, contraction recurrence, runtime and space
   bounds, and any ECDLP reduction and its full cost.

No complete contraction or ECDLP algorithm theorem, novelty statement, or
improved exponent is claimed as Lean-verified here.
