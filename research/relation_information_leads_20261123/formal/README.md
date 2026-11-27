# Formalization status

Both declarations are source-only attempts, never reported as checked Lean proofs. This environment has neither lean nor lake on PATH. No Mathlib checkout/lock is installed in this study. Upstream crypto-lean revision ff94d4295f592c679074034cf926695c76de4ddc (aburan28/crypto-lean) names Lean4v4.35.0; its toolchain file was fetched at that exact revision. The isogeny-labs spelling returned404; the documented original repository resolves.

A compatible crypto-lean/Mathlib dependency checkout and the named toolchain are needed to build these declarations, inspect both #print axioms outputs and run lean4checker --fresh where available. No kernel/axiom verdict exists here. The finite-field identity certificates cover two polynomial building blocks only, not these quantified Lean statements.

Remaining bridges: valid prime-order subgroup relations to Rz=a+d*b, finite-dimensional annihilator/rank equivalence, sparse dependency extraction with b-dot nonzero, actual point identities for geometric rows, and complete Macaulay closure/ideal-preservation certificates. None is assumed as a new checked theorem.
