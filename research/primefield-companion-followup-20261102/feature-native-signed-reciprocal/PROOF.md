# Proof map

`Norm.lean`: `SignedReciprocal.norm_identity`, checked with Lean4.34.1, commit5045d0056413266e57c625dcd7c365b10e377c52. Imports only bundled Std; no Mathlib dependency. Toolchain pin: leanprover/lean4:v4.34.1.

Statement: for any type carrying Lean.Grind.CommRing, if v*vi=1 and v+vi=w, then (a+v*b)*(a+vi*b)=a*a+w*a*b+b*b. This exactly establishes the conditional algebraic norm identity. Checked axioms are propext, Classical.choice, Quot.sound; no sorry or custom axioms. Command/output retained in validation03. A separate lean4checker fresh replay was not run.

Written proof only: binomial coefficient reversal, roots-of-unity weighted projection, signed-product identity including finite-field Euler criterion, and quotient/gcd equivalence. These are not covered by the conditional Lean theorem. Formalization still requires their finite-field, polynomial and stream definitions; the Lean theorem is not a formal proof of the runtime filter or scalar recovery. Deterministic Rust controls cover the finite domains described in README.md.
