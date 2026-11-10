# Branch root multiplicity independent source review

Source fidelity supported; kernel unverified. Exact published revision a649d5a6f84b38a4824f07e6755c37062820bfcc matched local candidate bytes.

**branch_scalar / branch_affine_eval.** Actual affine quotient evaluation sends constants to themselves and xbar to a. Ring-hom extensionality identifies polynomial evaluation at a.

**rho_unit_iff.** The inverse branch equivalence preserves units. IsLocalization.AtPrime.isUnit_to_map_iff identifies units with the prime complement; branchIdeal is definitionally the kernel of branchEval. No arbitrary unit premise is supplied.

**polynomial_germ_unit_iff.** Combines actual evaluator transport, localization units and polynomial residue evaluation. This establishes the residual-unit bridge.

**polynomial_germ_order.** Requires g nonzero. Uses exact mathlib quotient by (X-C a)^rootMultiplicity, whose evaluation is nonzero, and the actual X-C a stalk order 2. AddValuation.map_pow is additive nsmul, then nsmul_eq_mul converts to ENat multiplication.

**polynomial_germ_order_nat.** Retains a non-top conjunct before toNat and derives the finite Nat value. Does not mask zero-germ infinity.

**support_base_multiplicity_sum.** Derives all three support polynomials nonzero by contradiction with finite stalk sum. Substitutes generic valuations, takes toNat only on finite natural casts, and cancels the factor 2 to obtain sum 1.

## Edge cases

**Zero polynomial.** Excluded by hg in generic order statements; zero factors excluded by proof in support theorem.

**Constant nonzero / higher powers.** Derivation gives valuation 0 for units and 2m for (X-C a)^m. No unsupported restriction to simple roots of g.

**Residual root.** Excluded using eval_divByMonic_pow_rootMultiplicity_ne_zero, not by assumption.

**Singular branch.** Derivative nonzero remains explicit for order claims; unit criterion itself does not need smoothness.

**Infinity and casts.** ENat equality is established first. toNat result includes non-top, and support arithmetic is over finite casts.

**Characteristic two.** Local formula does not claim a separable double cover or distinct conjugates. Those interpretations remain outside scope.

**Source specialization.** New support theorem is generic in a polynomial norm premise. Exact coefficientData source theorem can be composed with it, but this file does not add a dedicated source-specific wrapper.

Seven print-axioms commands are prospective checks, not execution evidence. No compiler or trials ran. Source inspection cannot establish elaboration, instance synthesis or tactic acceptance.
