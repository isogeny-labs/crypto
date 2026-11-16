# Round25 proof and bridge map

Lean4.34.1, mathlib d13f23b723b8a846827a245b89c10fc7d3f11612.
The source is lean/KoblitzDeck25.lean, SHA256
7bddbb6754283aa6e781b490e9398237bbadbc7512f4303573c2d2da4f32af25.
Lean02 compiles14 theorems and the transitive axiom auditor reports
0 failing,0 trusted-native. Standard foundations propext/Quot.sound only;
no sorry, custom axiom or native_decide theorem is accepted. Lean01 is
rejected and its original failed source remains in the raw archive.

| Claims | Kernel proof declarations | Other necessary evidence/bridge |
| --- | --- | --- |
| Invariant polynomial identities | s1_integer_lift,s2_integer_lift,s3_integer_lift;s1_invariant,s2_invariant,s3_invariant | Native positive04 checks the actual original source circuit |
| Order2 action and nontriviality | deck_involutive,deck_nontrivial | A nonzero in the retained generic source field; actual degree2 extension from R22 |
| Quadratic trace/norm expressions | deck_traces,deck_norms | Written identification of sigma with source-field conjugation |
| Mixed relation | mixed_integer_lift,mixed_norm_relation | Native cleared N2,N3 relations modulo the exact retained tower |
| b-unit inverse and Z relation | quotient_inverse_numerator,quotient_s1_equation | Q25 registered05: thirteen exact polynomial pullbacks |
| Invariant affine quotient, finite etale rank2 | Universal identities above | Written ring isomorphism, monic basis and derivative b-unit argument; not scheme formalized |
| Proper normalized quotient map | No new scheme theorem | Written normal-curve/function-field bridge; boundary equations/ramification open |
| Desired order11 action, arbitrary reverse evaluator, polarized embedding | None | Not established |

The Lean pairs are ordered (lambda,nu), whereas CM's coordinate action is
sometimes written (nu,lambda); their displayed translations agree exactly.
The parameter a in Lean is the original A=1+c. Universally proved algebra
does not identify a modular Jacobian or construct a polarized embedding.
No separate crypto-lean checkout/shared gate or lean4checker is available;
this is the retained pinned repository axiom gate, not a claimed shared-gate pass.
