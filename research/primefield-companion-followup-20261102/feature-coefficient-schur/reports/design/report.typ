#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Feature-coefficient Schur and sparse witnesses
2026-11-03 · Derived construction / static audit · Numerical trials: zero

*Closed representation.* The preserved public character features are exactly
all subsets of a q-window of size at most three. Represent the Gram residual by
C and its right-hand side by h(x) = x − phi(x)^T u. No r-row array is allocated.

#image("pipeline.svg",width:100%)

*Exact update.* For supplied pivot rows F, compute W = C F^T, E = F W and
H = W E^(-1). Then C_new = C − H W^T and u_new = u + H h_I. Reject singular
blocks. The basis stays fixed; state is immutable. This is the established
Schur identity specialized to the preserved feature family.

*Sparse certificate.* A supplied terminal point must satisfy C phi(x)=0 and
h(x) nonzero. Normalize its coefficient by 1/h(x). Each frame lifts a tail
moment mu using y_I = −H^T mu and mu_old = mu + F^T y_I. At the root, the
invertible diagonal C implies zero feature moment. A separate checker
recomputes all feature moments and the pairing sum y_x x = 1 from support.
Internal provenance tokens are not a substitute for this final check.

#table(columns:(2fr,2fr),inset:5pt,
 [Derived quantity],[Schedule / storage],
 [State coefficients],[m squared + m field entries],
 [Frame coefficients],[2mk field entries, k = 1 or 2],
 [Pivot products],[2km squared + 2k squared m + 2km],
 [Lift products],[2mk],
 [Accepted pivot coordinates],[At most min(m,r)],
 [Certificate support],[At most m+1])

Pivot counts exclude feature evaluations and small-block inversion. One point
evaluation uses qL+J products, with L the binary-power count for (r−1)/2 and
J the total label incidence. Details and additions are in DERIVATION.md.
Native validation, copies, allocations, failed proposals and final checking
are separate. Storage is O(q^6), and need not be smaller than r.

*Next dependency.* Select useful public pivot/terminal points without a field
sweep; analyze when the residual affine functional lies in the feature span.
The current routines accept supplied points. A feature certificate still needs
its explicit target-extraction connection in a complete driver.

*Evidence.* Source bytes, pattern/power AST and syntax checked statically;
no numerical, corruption, heap or performance trial. New trial execution
requires fresh admission and frozen controls. Earlier cohorts are unchanged.

*Sources.* Preserved direct-cyclic-companion/generators.py; Higham (2023),
“What Is the Schur Complement of a Matrix?”, elimination and block inverses.
No real positive-definiteness assumptions are used in this finite-field work.
