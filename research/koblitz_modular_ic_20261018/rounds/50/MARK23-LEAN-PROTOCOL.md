# Marked23 algebra proofs, before compilation
Compile and audit Marked23_50 with the original pinned Lean4.34.1/mathlib
d13f23b source and dependency-cache roots. Same one thread,4096MiB,1200s
per compile/audit; no native_decide. Prove the exact binary-prefix additive
identity and the abstract prime-order implication from nonidentity and23P=0.
Actual characteristic2 field arrays, Weierstrass addition implementation and
its agreement with the abstract curve group are native/written hypotheses,
not formalized by this module. Preserve every failed compile/audit; accept
only compile.exit0 and audit.exit0 with zero failing/trusted declarations.
