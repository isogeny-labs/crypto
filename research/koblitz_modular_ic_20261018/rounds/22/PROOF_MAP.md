# Round22 claim-to-proof map

The source project pins Lean4.34.1 and mathlib commit
d13f23b723b8a846827a245b89c10fc7d3f11612. The final build and complete
transitive-axiom audit determine verification status. Standard foundations
propext, Classical.choice and Quot.sound are allowed. No user axiom,
sorry/admit or trusted native evaluation is allowed.

| Claim | Lean module and declarations | Scope and remaining bridge |
|---|---|---|
| Integral augmentation identification and orientation | KoblitzRound22: project_embed, embed_norm_zero, embed_project, augmentation_equivariant | All commutative coefficient rings; concrete CM and Galois interpretation written in RECOVERY.md |
| Complete commutant | cyclic_expansion, coefficient_reconstruction, action_cyclic_symmetry, commute_iterates, complete_commutant_parameterization | Every linear map commuting with the actual four-dimensional rotation; no coefficient bound |
| Complete solution set and torsion recovery | complete_solution_parameterization, normalized_torsion_recovery, injective_on_torsion | Exact module identities with explicit hypotheses; not a modular-Jacobian map |
| Actual four-map certificate and complete image lattice | KoblitzRound22Recovery: actual_integral_recovery, all commutation/binding declarations, actual_full_integer_lattice | Literal CM matrices and integer right inverse, checked by the kernel. Native U,V and two-sided inverses additionally certify the Smith kernel basis |
| Small two-map certificate | KoblitzRound22TwoMap: two_map_integer_lift, two_map_equivariant_recovery; KoblitzRound22TwoMapData: actual_two_map_recovery, smallB_commutes, smallC_commutes, actual_b_coefficients, actual_c_coefficients | Universal integer polynomial identity plus actual fixed matrices. Final checks are recorded in VALIDATION.md |
| Inherited polarization | KoblitzRound22Polarization: all three declarations | Literal J-transpose-J, determinant5 and actual weighted Gram; no final quotient or principal polarization assertion |
| Simultaneous node187 contradiction | KoblitzRound22Coupled: both_factors_have_poles, third_pole_contradicts_regular_product, coupled_proper_reduction, no_node187_valuative_model, uncoupled_valuation_control | Universal integer inequalities and additive residue groups. Proper extension, curve selection, valuations of actual Miller functions and coefficient-split geometry are written hypotheses, not a formal scheme implementation |
| Genus-one substitution and exact tower algebra | KoblitzRound22Base: quadratic_norm_integer_lift, quadratic_norm, base11_integer_lift, base11_rational_equation, linear_coordinate_recovery, char_two_square, char_two_eighth, linearized_decomposition | Universal ring/field identities. Native F2[u,v] arithmetic and Singular source bindings are separate. Complete base divisors use written smooth-curve local-parameter and norm-support arguments |
| Exceptional SS5 projection identities and Jacobian | KoblitzRound22SS: lambda_fourth_power, trace_square, relative_jacobian_determinant | Universal characteristic2 projection powers and literal determinant. Original-ideal lifts, unit checks and Frobenius residue decomposition are native CAS certificates; no formal proper normalization |

The verified R22 set has51 named declarations. The complete project build
passes9095 jobs;the final transitive audit checks1088 project theorems with0 failing
and0 trusted native results. Audit counts also include generated auxiliary
theorems and earlier rounds. A Lean proof of a universal implication does
not silently discharge its geometric hypotheses.

No declaration asserts a full quotient saturation, polynomial membership
h^k f in I, proper level77 normalization, complete lifted divisors, a prescribed
Riemann–Roch basis, the modular-factor map, or an arbitrary modular-divisor
reverse evaluator. The ordinary460/367 fibers/images remain unresolved.

Eight coordinate identity certificates bind the actual G32 integer action to
KoblitzRound22Recovery. Three universal integer-lift identity records bind the
quadratic norm, base11 relation and small recovery identity to their exact
Lean source hashes. Their finite-field AST checks supplement the independent
integer products and kernel proofs; finite sampling is not the proof over Z.
