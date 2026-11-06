# Round21 proof map

Every formal result uses the pinned Lean4.34.1/mathlib manifest. The final
full build and transitive audit receipts determine verification status;
a failed build's automatically introduced sorryAx is never a passing
proof. No user axioms, trusted native evaluation, sorry or admit are allowed.

| Report claim | Lean declaration(s) | Exact scope / missing bridge |
|---|---|---|
| Order-of2 permutation modulo19 | KoblitzRound21.doubling_bijective, doubling_mod19 | Exact18-element permutation, kernel checked |
| Additive18-fold Frobenius trace, root trace identities | trace18_add, trace18_root_reindex, geometric19, trace18_nontrivial_root, trace18_one | Any commutative characteristic2 ring for additivity; field and nineteenth power1/nontrivial for root trace; geometric identity universal |
| Five nineteenth roots cannot sum to zero | missing_among_five, five_primitive_powers_ne_zero, five_roots_ne_zero | Any characteristic2 field with a supplied primitive19th root; every five-tuple including repetitions; root adjunction geometric bridge written |
| Ordinary infinity chart factorization | ordinary_chart_factorization | Universal commutative-ring polynomial identity; DVR pole orders/local parameter semantics written |
| Full source trace-zero descent implication | trace_zero_from_actual_descent | Arbitrary additive commutative group and additive sigma, with all four actual difference-descent identities; elliptic/split coefficient interpretation written |
| Actual Q34 support/U gate | KoblitzRound21TangentData.actual_support0..3, actual_U_gate0..3 | Literal actual program and scalar model bindings; native S polynomial degree/leading checks and group geometry not reimplemented as Lean elliptic DAG semantics |
| Actual equivariant averaged recovery | KoblitzRound21DescentData.actual_averaged_recovery, actual_equivariance, actual_rows_equivariance | Literal matrices over pair representation of Z[w], w²+w+2=0; CM-morphism/descent interpretation written |
| Sharp denominator5 obstruction | KoblitzRound21Equivariant.source_norm_zero, actual_fixed_source, target_norm_coordinates, fixed_target_is_norm, intertwine_iterates, equivariant_scalar_recovery_zero, no_equivariant_retraction | All stated linear maps over a commutative characteristic5 ring, regular target cycles and explicit augmentation source action; nontrivial ring for no retraction. Integral CM reduction and denominator clearing written |
| Q36/Q38/Q40 complete repaired scalar models | KoblitzRound21Boundary / FinalBoundary / LastBoundary: actual_remaining_model0..3, cube_zero0..3, nonzero0..3, actual_new_linear0..3, all_new_linear0..3 | Every retained finite Boolean gate/constraint family and all added zeros/linear pairs; Q35's geometric legitimacy is a separate written bridge. No formal full-scheme saturation claim |
|32 exact coupled resultant tops and residues | KoblitzRound21Separators.actual_top_shapeG_F, actual_scalar_residueG_F, actual_U_gateG_F | G=0..3,F=0..7; exact finite selected scalar polynomial terms/gate bindings; full F360 norm matrices and denominator-cleared substitution native |
|20 pure-source families/1330 relations | KoblitzRound21Family.all_actual_topsG_I | G=0..3,I=0,2,4,6,8; kernel checked finite polynomial tops; exact full field elimination and univariate substitution native |

The Q35 source-arc exclusion remains partial formalization: curve
selection, proper elliptic extension, splitting the coefficient algebra,
valuation scaling and formal-group leading-term extraction are written
in SATURATION.md. Q37/Q39's native arithmetic is independently replayed,
not called a full Lean proof of the finite-field implementation.
The source embedding is distinct from the final polarized quotient.
No new theorem asserts an integral level77 map,243 quotient sections,
a whole saturated ideal, or IC efficiency.

Source-bound universal identity records:
[IDC1h83eef58c80bbbbbf](identity-geometric-series19.json) for geometric19;
[IDC1hf0e17a52634561b4](identity-ordinary-infinity-chart.json) for the
ordinary chart factorization. Both pass64 issuer checks plus128 extra
parsed checks. Their fixed odd-prime AST semantics are appropriate to
these universal identities;the root/trace characteristic2 theorem and
mod5 representation obstruction are proved separately in Lean.
