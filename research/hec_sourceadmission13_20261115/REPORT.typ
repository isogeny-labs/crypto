#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Source admission and rational kernel obstruction
Genus 3 · 15 November 2026
#image("flow.svg",width:100%)

A fixed 16-case cohort on the actual hyperelliptic source family yields five certified large prime subgroups. All five Weil recoveries pass independent coefficient checks. The three larger-field cases fail a necessary condition for the current rational (3,3,3)-isogeny route. This separates valid source workloads from available transfer maps.

#table(columns:4,inset:5pt,[p / constant],[Certified Jacobian N],[Prime ell],[3-kernel test],
 [127 / 31],[2057786],[1028893],[Necessary pass],
 [127 / 33],[2087686],[1043843],[Necessary pass],
 [1039 / 32],[1138476554],[569238277],[Fail],
 [2011 / 34],[8131390048],[254105939],[Fail],
 [4003 / 31],[63069089266],[31534544633],[Fail])

== Exact source certification
An Fp point count fixes a1. Bounds on a2 and a3 enclose N in an interval of width O(p²). Integer BSGS finds a point-order multiple; a nonzero prime-order witness and exactly one ell multiple in the interval certify N. Each admitted ell exceeds the proper-factor Weil bound and ell² exceeds N. These prove the stated rational subgroup properties, not absolute simplicity. Eleven other cases remain bounded unknown; all attempts are retained.

Public targets are cofactor projections of separate places, with no supplied or computed scalar relative to D1. No DLP was solved. No kernel or quotient map is constructed for a new source here.

== Weil recovery and necessary obstruction
All first 3p coefficients of f^((p−1)/2) agree between p-adic recurrence and independent polynomial multiplication. Exact lift/root-bracket checks recover L, matching certified N and counted N1. No extension-field counts were used.

For p≡1 mod3, an invariant Lagrangian W forces the Frobenius polynomial to factor as h(T)h-star(T), where h is an invertible-constant monic cubic over F3 and h-star its normalized reciprocal. All18 possibilities were checked. None works for the three larger sources. Presence is only necessary; absence excludes the required rational kernel for this route without a semisimplicity assumption.

KernelObstruction.lean verifies the three finite coefficient impossibilities. The symplectic, geometric and group-cardinality arguments remain explicit mathematical premises rather than formalized code guarantees.

== Custody and decision
Separate source freezes/checkpoints: 38958f652b07/010615Z, 2c8e3504bb59/010850Z, 852aff918417/011121Z. Each gate ran once, all statuses0 and stderr empty; unknowns and excluded cases remain visible. Raw certificates, polynomials, protocols and hashes accompany RESULTS.md.

Future workload selection must combine subgroup and kernel admission before map construction. Other isogeny degrees require their own justification. This is not a novel tenfold algorithm, timing result or new-field mapped solve. Genus two/four and the full objective remain open; the previous failed charged comparison is unchanged.
