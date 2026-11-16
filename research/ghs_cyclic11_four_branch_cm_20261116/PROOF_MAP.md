# Lean instance map for the four-branch screen

1. **Structure above.** This is a finite `List` certificate over natural
   inertia exponents, with decidable equality and modular arithmetic from
   Lean `Init`. The statement about abelian varieties is a separate paper
   implication, not formalized here.
2. **Theorems used as stated.** `decide` discharges concrete finite
   list-cardinality and Boolean equivalence obligations; the paper uses
   Ozman--Pries--Weir Lemma 2.1 and Theorem 2.3 as stated. No group or
   endomorphism law is re-derived for one curve.
3. **Object as instance.** The 910 ordered exponent triples and their
   nonzero fourth exponent are the finite input, and the two order-five
   character orbits are the finite classifier.
4. **Assembly probe.** The report's mathematical chain is conditional:
   `B(a,b,c,d)≤5`, then the published `p-rank≤B`, then at most one
   ordinary dimension-five factor. The finite Lean endpoint checks only
   the first classification. The generic implication `f≤B≤5 → f≤5`
   uses transitivity of `≤` and does not need a study theorem.
5. **Missing components.** A formal Jacobian, deck action and
   endomorphism-algebra representation are absent from this standalone
   `Init` module. The CM-field argument and elliptic trace criterion are
   written in `RESULT.md` with their assumptions. The Lean file checks
   finite inertia and trace arithmetic, never their geometric bridge.
6. **Inheritance table.** List length comes from `List.length`; the
   zero/5/10 histogram from `List.filter`; pair symmetry is a Boolean
   predicate on rows; trace equations are integer arithmetic. The paper
   bridge supplies `p-rank≤B`; the ordinary-factor consequence is outside
   Lean. No local algebraic operation shadows Mathlib or crypto-lean.

| Declaration | Kind | Owner | Hypotheses |
|---|---|---|---|
| `slice_one` ... `slice_ten`, `all_slices` | finite instances | this study | each fixed first exponent 1..10 |
| `aggregate_arithmetic` | finite instance | this study | ten identical slice counts |
| `cm_trace_three`, `cm_trace_five`, `cm_enumeration_caps` | finite instances | this study | fixed p=3,5 Hasse intervals |

The finite certificate uses only kernel `decide`; no `native_decide`,
custom axiom or increased resource limit is planned.
