# Degree proof map

Claim: saturated norm degree<=18. Lean declaration:
CalibrationFiber.saturated_norm_degree in DegreeBound.lean.
Hypotheses: arbitrary field K; A,B,f,S,Q polynomials; degree bounds15,11,7;
S nonzero of degree12; norm nonzero; exact equality A*A-f*(B*B)=S*Q.
Conclusion: Qdegree<=18. All quantifiers appear in the actual theorem.

Checked exit0 using Lean4.19.0, commit6caaee842e94, mathlib
c44e0c8ee63ca166450922a373c7409c5d26b00b, existing offline environment
/Volumes/SSD990-2/lfd-summation-systems/formal. Command: lake env lean
/absolute/path/to/DegreeBound.lean. Source config pins the same versions.
LEAN.txt prints exactly propext/Classical.choice/Quot.sound; no sorry/customaxiom.

Remaining readable proof/exact instance checks: divided-difference restriction,
coefficientwise uT³division, anchor factor multiplicities and base-field descent.
These are hypotheses in the formal degree theorem, not new formalized geometry.
Decoder correctness and full group/isogeny construction are not covered by the
Lean theorem. V4 directly checks this instance against old alignment only after
candidate selection. Shared crypto-lean integration was not performed.
