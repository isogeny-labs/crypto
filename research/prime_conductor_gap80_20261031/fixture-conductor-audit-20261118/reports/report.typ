#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#set heading(numbering:none)
= Conductor audit of retained explicit maps
Native PARI/GP correctness certification; 18 November 2026.

Four fixed ordinary-curve maps are classified using their actual endpoint
orders. The largest isogeny degree in this panel does not change conductor.
The explicit map on the 202-bit target field changes conductor from one to
two. This is a bounded lower milestone, not the original minimum-80-bit
crossing or an exhaustive record survey. [pcc:unverified]

#image("conductor.svg",width:100%)

#table(columns:(2fr,1fr,0.7fr,0.7fr,0.8fr,0.8fr),inset:5pt,stroke:0.35pt,
 [*Retained fixture*],[*Degree*],[*Source f*],[*Target f*],[*Ratio*],[*Inputs*],
[degree19], [19], [1], [1], [1], [6],
[degree10009], [10009], [3], [3], [1], [36],
[field202-degree2], [2], [1], [2], [2], [35],
[field96-degree5], [5], [1], [1], [1], [35],
)
All 112 retained map inputs match fresh direct classical Vélu sums;
98 matched known scalar pairs are reverified on both endpoints.
Counts include repeated, kernel and infinity inputs. Six negative controls
reject degree-as-gap, composite degree, wrong kernel/codomain/image and a
supersingular input. Pre/post source/input/executable hashes match.
No logarithm solve, scalar timing or ECDLP performance session ran.
[pcc:unverified]

== What is actually certified
For the two small fields, both endpoint point counts are recomputed. In the
large fields, fresh counts and Frobenius conductors are deliberately absent:
the order argument uses explicit CM endomorphisms, normalized quotient scaling,
ordinary reduction and the degree2 volcano alternatives. The supplied large
ambient cardinalities are not independently recounted. Prime subgroup orders,
generators, disjoint kernels and transported target identities are checked.
[pcc:unverified]

#table(columns:(1.2fr,2.4fr),inset:4pt,stroke:0.35pt,
 [*Requirement*],[*Disposition*],
 [Four retained maps/orders],[Qualified at the labelled fixture scope; GP arithmetic plus the standard order argument],
 [At least 80-bit conductor crossing],[Open: target quotient, non-kernel images and endpoint certificates],
 [Endpoint measurements],[Open: isolated construction/transport/scalar/ECDLP/IC and admitted comparisons],
 [Full costs],[Construction, conversion, failed work, verification, memory, destination and recovery all unmeasured],
)
Producer evidence PCC-20261118-d00763 is unsigned/not covered by a signed
independent verdict; no Lean proof or official ledger promotion is claimed.
The original four maps and previous null conductor fields remain unchanged;
this adds separate certificates. No scientific job is active.
[pcc:unverified]

#pagebreak()
= Why the orders, not the degrees, decide
For ordinary curves, write Δπ = t² − 4p = fπ² DK. Then
Z[π] ⊆ End(E) ⊆ OK, and fE divides fπ. A prime-degree ell edge can change
only the ell valuation of the conductor, by at most one. Thus ell not dividing
fπ forces a horizontal edge. Geometric degree is not an evaluation cost.
[pcc:unverified]

== The degree10009 case
Fresh trace 686 gives fundamental field discriminant -2910680
and Frobenius conductor 3. Hence actual conductors
are one or three, and degree10009 cannot change them. To determine the orders
exactly, set β = (π − 1)/3. Its integral trace and norm give fundamental DK,
so Z[β] = OK. The full three-torsion is rational iff β is an endomorphism:
use the separable quotient property of [3]. This happens exactly at conductor
one. The three-division polynomial and rational ordinates give 3
rational three-torsion points on each endpoint, cross-checked against group
invariant factors. Both conductors are therefore 3.
[pcc:unverified]

== The degree2 positive case
On y² = x³ − 140x + 784, take s² = −7 and u = (1 − s)/2. The normalized
quotient with kernel (7 + s, 0), followed by inverse scaling, is an endomorphism
of differential u. Its degree is two and u² − u + 2 = 0. Trace/norm and the
integral trace bound force trace one, hence fundamental discriminant minus
seven and a maximal source order. Splitting of the field prime gives ordinary
CM reduction. [pcc:unverified]

The audited quotient instead uses kernel (−14, 0). Its target has conductor
one or two. The class-number-one maximal order has unique j = −3375; the
target j differs. Consequently the target conductor is two. The independent
direct-map panel checks the actual rational quotient and subgroup transport.
[pcc:unverified]

== The degree5 control and preservation
The pilot small map has equal endpoint j invariants. Its explicit scaling
produces a degree5 endomorphism with absolute trace one and fundamental
discriminant minus19. Both ordinary geometric orders are maximal. This small
map is distinct from the original large pilot kernel. [pcc:unverified]

Each selected subgroup has prime order coprime to the map degree. Thus the
kernel intersection is trivial. Both generators have exact prime order,
and every frozen [k]G and [k]φ(G) is verified. There is no ECDLP eigenvalue,
GLV basis, speedup, cost ranking or timing inferred from these checks.
[pcc:unverified]

== Sources and reproducibility
Galbraith, ANTS submission, introduction and section 2:
#link("https://antsmath.org/ANTSXVI/papers/Galbraith.pdf")[ordinary order sandwich and conductor gap].
Galbraith–Gilchrist–Robert, section 2.1:
#link("https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/articles/climbing_volcanoes.pdf")[prime-degree volcano levels].
The actual reviewed PDF versions/hashes are in sources.json. See DERIVATION.md
for the full kernel/CM/order argument, and qualify-v2.sh for exact native
calls into fresh output paths under the bounded correctness cap. KB unavailable;
no claim of latest literature or an exhaustive world-record search.
[pcc:unverified]

Does degree10009 imply conductor gap10009? No; both actual conductors are three.
Does this meet the requested80-bit criterion? No; the certified ratio in this
fixed panel is two, with no endpoint timings. [pcc:unverified]
