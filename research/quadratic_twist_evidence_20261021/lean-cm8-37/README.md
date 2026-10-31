# Actual F37 points and source-DLP reduction

The unmodified scoped gate checks **19 public named theorems / 22 audited declarations**, including three generated helpers, with no failing or extra trusted/native axioms. Foundations are only `propext`, `Classical.choice`, `Quot.sound`. Lean 4.34.1 and Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` are pinned. This is a new-module audit, not a fresh combined audit of imported projects. `lean4checker` was unavailable.

| Report statement | Declaration(s) | Explicit scope |
|---|---|---|
| Actual equations and nonsingular point maps | `forward_equation`, `dual_equation` | Finite field F37, exceptional denominators excluded; maps handle origin/kernel separately |
| Actual nonsquare monic twist | `twist_coefficients`, `twist_nonsquare` | d=35, exact models E(7,19), T(28,33) |
| Public generators and order | `generator_image`, `source_order`, `target_order` | P=(1,8), U=(32,29), order19 |
| Scalar transfer and injectivity | `scalar_transport`, `source_scalar_injective`, `target_scalar_injective` | `Fin 19` scalar representatives |
| Subgroup addition and coordinate inverse | `subgroup_additive`, `inverse_source`, `inverse_target` | Explicit cyclic19 subgroups, I=[10]D |
| Material domain failures | `source_kernel_control`, `dual_kernel_control`, `global_inverse_fails` | Nonzero2-torsion is not in odd subgroup; no global inverse claim |
| Original-source DLP equivalence | `actual_source_reduction` | Q actual E.Point with explicit existential cyclic membership, n:Fin19; no unknown index exposed to native recovery |
| Necessary coefficient obstruction | `no_short_scaling`, `no_admissible_short_change` | Written short-Weierstrass coefficient equations, u nonzero; not general isogeny classification |

All point definitions use actual Mathlib nonsingular points and coordinate formulas, not log-table maps. Kernel `decide +kernel` proves the finite statements; no trusted native evaluation is used. Full38-point addition for both maps is independently checked natively; the new Lean addition statement is only the19-subgroup statement. A generic global geometric group homomorphism/classification remains outside this module.

The first gate failed before elaboration cloning Mathlib. A committed exact manifest plus ignored existing package-cache symlinks resolved setup without changing gate/trust. Attempt02 has explicit gate exit0 and PASS/22 audit despite outer PTY exit1; unchanged-source attempt03 explicitly exits0/PASS. All sources/logs/statuses remain in [raw evidence](../validation/cm8-37-20261030/README.md).
