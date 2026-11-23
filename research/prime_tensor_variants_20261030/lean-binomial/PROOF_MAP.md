# Exact proof map

All nine reported theorem names have namespace `LevelwiseBinomial` and are
proved in `LevelwiseBinomial.lean`.

| Statement | Lean declaration | Exact hypotheses |
| --- | --- | --- |
| The actual rational binomial polynomial evaluates to the generalized integer binomial | `binomialPolynomial_eval_int` | Every `n:Nat`, `t:Int`; equality in `Q` using the integer cast |
| Natural binomial parity equals the selected binary digit | `choose_nat_pow_two_digit` | Every `n,i:Nat`, including zero; equality in actual `ZMod 2` |
| Signed generalized binomial parity equals the selected Euclidean binary digit | `choose_int_pow_two_digit` | Every `t:Int`, `i:Nat`, including negative/zero `t`; no valuation assumption |
| Dividing by the maximal base power removes base divisibility | `quotient_not_dvd` | Every `p:Nat`, `p≠1`, `t:Int`, `t≠0`; no primality assumption |
| Generalized signed binomial at the actual valuation power is odd | `choose_int_valuation_odd` | Every `t:Int`, `t≠0`; actual `padicValInt 2 t` |
| Oddness with the selected index named explicitly | `choose_int_at_valuation_odd` | Every `t:Int`, `t≠0`, `i:Nat`, `padicValInt 2 t=i` |
| Actual level-family polynomial value equals the cast integer power | `levelPolynomial_eval_int` | Every `i,p:Nat`, every signed `t:Int`; equality in `Q` |
| Every actual level polynomial has origin one | `levelPolynomial_origin` | Every `i,p:Nat`, including `p=0` |
| The level polynomial's integer node value is divisible by `2^p` | `level_integer_value_divisible` | Every `i,p:Nat`, `t:Int`, `t≠0`, `padicValInt 2 t=i`; node value/constant coefficient only |

The relevant definitions are actual Mathlib objects: `Ring.choose`,
`Polynomial Q`, `ZMod 2`, integer Euclidean division, and `padicValInt`.
`binomialPolynomial n := Ring.choose X n` is connected to integer values by
the proved ring-homomorphism identity, not an assumed matching condition.
No quantified result is replaced by a finite arithmetic certificate.

All reported theorems have explicit `#print axioms` commands. Verification
receipts record actual builds and transitive audit/replay results. Allowed
foundations are only `propext`, `Classical.choice`, and `Quot.sound`.
All listed checks passed, including the unmodified shared gate and
import-inclusive `leanchecker --fresh` replay. The audit checked all nine
theorems with zero failing and zero native-trusted proofs.

## Open formal obligations for the full counterfamily

The following parts of the written argument remain unformalized and are not
assumed as axioms or hypotheses purporting to prove the full result:

1. Define the local coefficient ring `Z_(2)` as rationals with odd
   denominator, or an equivalent actual localization, and define the scaled
   polynomial chart `f.comp (C t + C (2^(a+1))*X)`.
2. Prove the coefficientwise statement, for every signed integer `t` and
   `a≥i`, that the chart of `B_(2^i)` is integral and its difference from
   the constant `B_(2^i)(t)` has every coefficient in
   `2^(a+1-i)*Z_(2)`. This requires the actual polynomial Vandermonde identity
   and integral/divisible coefficients of the scaled binomial factors.
3. Combine that stronger chart statement with the checked signed-node
   oddness to prove **every** coefficient of `H_(i,p)(t+alpha*z)` belongs
   to `2^p*Z_(2)` at nodes of valuation `i`. The checked node-value
   divisibility proves only the constant coefficient and is not a substitute.
4. Prove degree bounds under the actual tapered schedule, all ceiling and
   parameter-admission inequalities, and charged rational description bounds.
5. Define the generic level-dependent modular jet-data interface and prove
   that the constructed family and the zero family give identical supplied
   data but different origins. This full obstruction theorem is still open
   in Lean, even though the written proof is independently reviewed.

The construction's realizability by actual packet-network families is
neither asserted nor formalized. There is no ECDLP impossibility theorem,
general algorithm lower bound, or code-aware hardness claim here.
