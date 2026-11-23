# Formal proof map

Central scalar theorem: `ConicDescent.centered_conic` in Centering.lean.
Exact hypotheses: arbitrary field R with 2 nonzero, x=e-d+beta,
2beta=2d-k, e+a+b+r=2k. Conclusion: x=3k/2-(a+b+r).
`centered_conic_doubled` proves the identity without the 2-nonzero hypothesis.
Here r is the SUM of the three residual point logs; R is the scalar log field,
not the curve coordinate field. Applying this to F_ell requires an established
log homomorphism in the checked odd prime subgroup.

Lean 4.19.0, commit6caaee842e94; mathlib revision
c44e0c8ee63ca166450922a373c7409c5d26b00b. Existing offline environment:
/Volumes/SSD990-2/lfd-summation-systems/formal . No dependency update.
Command: lake env lean /absolute/path/to/Centering.lean.
LEAN.txt and EXIT-LEAN.txt are the authoritative final build result.
LEAN-FIRST.txt retains a failed proof-script attempt: field_simp had already
closed the goal, so the extra ring command was rejected. The theorem statement
was unchanged; removing the redundant command is a proof-script correction.

Boundary: Lean formalizes the CONDITIONAL SCALAR identity. It does not
formalize the plane quartic geometry, the log-kernel construction, divisor
linear equivalence, centering construction, public sign proof or implementation.
Those hypotheses have readable derivations in RESULTS.md and executed exact
checks in RUN.txt/LOGS.txt. No blanket formal-verification claim is made.
The theorem's printed axiom list must be inspected for unexpected axioms/sorry.
The shared crypto-lean integration is not performed by this local scalar check.

Final checked outcome: exit0. Printed axioms are exactly propext,
Classical.choice and Quot.sound; no sorryAx or custom axiom. Both declarations
elaborate in the pinned environment. The two geometric equations remain
explicit hypotheses rather than claimed formal geometric results.
