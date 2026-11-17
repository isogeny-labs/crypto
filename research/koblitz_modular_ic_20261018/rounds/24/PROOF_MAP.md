# Round24 proof map

Toolchain Lean4.34.1; mathlib d13f23b723b8a846827a245b89c10fc7d3f11612.
Module: ../../lean/KoblitzRound24Geometry.lean. Compile/audit status is
recorded in the final validation receipt; failed lean-01 remains retained.

| Report subclaim | Lean declaration in KoblitzRound24Geometry | Exact scope |
|---|---|---|
| Cubic chart clearing | infinity_base_clearing | Universal commutative-ring identity |
| Unramified transformed H | unramified_chart_identity | Universal commutative-ring identity in actual exponents |
| Tame transformed H | tame_chart_identity | Universal commutative-ring identity in actual exponents |
| Cleared toric inverses | tame_inverse_cleared | Universal polynomial identities |
| Generic toric inverse in both directions | tame_rational_inverse, tame_parameter_inverse | Arbitrary field, explicit nonzero t,Z or w,x |
| Residue tower equations | unramified_residue_equation, tame_residue_equation | Exact coefficient specializations; no geometry assumed |
| Two distinct lambda residues | two_lambda_residues | Field of characteristic2; both directions |
| Tame coordinate orders | tame_orders_from_inverse | Consequence of the two order equations; written DVR bridge supplies them |
| Proper fiber arithmetic | infinity_fiber_degree | Exact weighted sum, not the geometric fiber formula |
| Generic nu denominator order | infinity_nu_denominator | Pulled-back base valuation arithmetic; written local charts supply orders |
| Constant-degree alternatives and exclusion | constant_degree_candidates, constant_degree_one | Explicit degree divisibility and degree2-exclusion hypotheses; not field/scheme identification |

The exact b,c substitution, source polynomial divisions, irreducibility and
all actual Jacobian minors are native certificates. Smooth local-ring,
normalization, proper degree exhaustion, Hensel existence and global
constant-field/geometric-integrality bridges are written proofs in CM.md.
They are not hidden in theorem assumptions or claimed fully formalized.
The original F2 modular-factor map, its descent/order11 action, complete RR,
node187 membership and arbitrary-divisor reverse evaluator are not proved here.
