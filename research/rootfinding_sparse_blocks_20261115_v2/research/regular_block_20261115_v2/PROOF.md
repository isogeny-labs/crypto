# Regular doubled-source free-k norm fiber, with exact sparse incidence audit

This is a conditional binary norm component, not a global original-root solver,
a novelty claim, or a qualified performance comparison. Classical Hermite,
Frobenius-linear and sparse-graph identities supply the reduction.

Let K=GF(2^n), E:y²+xy=x³+Ax²+B, B!=0, V an F2 source subspace,
and T=(z,yT) a rational target with z outside V. Fix h, a nonzero a in V,
and a rational ordinate ya of a. Fix b=h²+h+z in V. Set G=hX+k,
F=X³+rX²+sX+t, t=z³+rz²+sz+(hz+k)yT, and
N=F²+XGF+(X³+AX²+B)G²=(X+z)H. The last equality follows by
substitution of the target curve equation; H is monic degree5.

Restrict to G(a)!=0. N(a)=0 and F(a)=G(a)ya specify the selected rational
lift, and formal differentiation gives N'=kF+XG(X²+s)+X²G². Dividing
N'(a)=0 by aG(a) gives
s=(k/a)ya+a²+a(ha+k).
Substitution into F(a)=G(a)ya and t gives
r=[(ha+k)ya+a³+z³+s(a+z)+(hz+k)yT]/(a+z)².
All denominators are fixed nonzero. These conditions are necessary and
sufficient for the selected regular double-root/lift branch. Since a!=z,
H=(X+a)²Q exactly. Q is monic cubic with first coefficient b.

Both r and s are K-affine in the SAME k. Thus F=F0+kF1 and G=G0+k,
so N has only constant,k,k² terms. Squaring is F2-linear, and division by
the fixed monic factors X+z and (X+a)² is linear. Every coefficient of Q
is globally F2-affine in k. Basis-column replay audits, rather than proves,
this fact. The worker directly divides the literal norm and checks zero
remainders. No formal Jacobian sparsity is used: a squared dependency is
present in the exact binary column whenever it acts nontrivially.

For source-basis product spans Wj=span(products of j basis elements),
Q=X³+bX²+TX+U can be supported in V only if T in W2,U in W3.
For actual n83/d18 these have dimensions35,52. At most79 independent
binary conditions act on83 bits of k. A consistent family therefore has
nullity AT LEAST4, never an assumed upper bound. Gaussian row-combination
proofs certify pivots/contradictions, particular and complete independent
kernel. Enumerate only nullity<=12; otherwise preserve UNKNOWN and family.
The unique forbidden k=ha is excluded from this regular branch and remains
an unresolved common-factor branch. Zero a and other h/a/ya remain separate.

The sparse audit uses the exact row-column incidence graph. Connected
components give a certified block DIAGONAL permutation. Solving every local
component (including isolated free columns) and preserving zero-RHS
contradictions is equivalent to the dense system. A complete independent
sparse kernel of the same dimension plus a valid particular proves affine
family equality. A single connected component disproves this sufficient
separation shortcut but does NOT disprove a finer matched block TRIANGULAR
permutation. General BTF/SCC is explicitly unimplemented and queued.

For exact support use L_V(X)=product_{v in V}(X+v), represented by the
linearized recurrence L_{W+<v>}=L_W²+L_W(v)L_W. Compute only its remainder
R modulo Q, plus sparse linearized coefficients for scalar evaluations.
All roots of Q, with multiplicities<=3, lie in V iff R^4=0 in K[X]/Q.
Indeed each irreducible primary factor then divides a power of the squarefree
split polynomial L_V; conversely fourth power covers every multiplicity.
This handles repeated roots. A complete distinct-root isolator then divides
out every multiplicity, reconstructs Q, adds the two fixed copies of a and
emits every distinct source ordering. A capped factorization remains UNKNOWN;
it cannot certify an empty source set or complete norm fiber. Original
ordinates/auxiliaries and polynomial ghost branches are not restored here.

Storage: O(n²) bits for coefficient constraints/kernel plus O(n³) bits for
row-combination provenance, O(d) field elements for linearized L_V and
O(2^min(nullity,12)) conditional outputs; each supported k has <=120 source
orderings. Work: n+1 constant-degree literal norms, polynomial-time Gaussian
elimination and incidence scan, then at most4096 constant-degree support/
factor/order checks. Full outer h/a/ya search is still unsearched and potentially
large. Profiling repeats run in one process; setup irreducibility cache may be
warm, not cold matched timings. Peak self RSS is not aggregate process-tree RAM.

Controls use independent exhaustive r/s/k norm divisions and source multisets
on declared GF4/GF8/GF16 fixtures. Both selected ordinates are exercised via the
first two nonzero on-curve source points. Actual positive coverage is measured,
not assumed; a required total positive check prevents vacuous admission.
90-second process budget charges startup/final output externally; internal80s
work checks and84s serialized-report demotion remove exact counts/completion
on lateness. The coordinator preserves raw observations even when demoted.
