#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
= Trace-zero relation models and tau transport
2026-11-09 · isogeny-labs/crypto · PR3744 · unsigned local evidence

*Implemented:* a basis-only decomposition oracle without a finite factor-base
table; a complete finite-base full-point chain; inverse/half-trace membership;
opt-in constant specialization; reversible tau-minus-one transport. Twenty
focused checks, ten original regressions and seven prior-lead regressions pass.
The production library type-check passes. These are correctness checks and
static constructions. No comparative experiment panel ran.

#image("construction.svg", width: 100%)

== Exact working scope
E0: y²+xy=x³+1 over F2; odd n. The full trace-zero object in the Weil
restriction has dimension n−1. Its rational point group Tₙ equals4E0.
Models here describe these finite rational points, with a separate identity
branch. They are not general scheme charts over arbitrary extension algebras.
ECC2K-130 uses n131, the original repository field, G and public Q.

The new subspace oracle takes a linear basis, target and arity. It defines
B={affine P in Tₙ : x(P) in V}, without enumerating B. Free coefficients
and inverse membership constrain each summand. All intermediate-identity
masks are kept. Partial sums are not restricted to V. Every SAT tuple is
independently lifted to both original signs and replayed in the group; all
signs must fail before that tuple is blocked. Complete exhaustion refutes;
caps or unsupported inputs remain inconclusive.

Every target of the n5/7 kernels, d0/2/3 and arities2/3 agrees with separate
exhaustive enumeration:240 calls, no point table passed to the solver.
Wide d16,m3,all-affine constructions give:

#table(columns: (auto,auto,auto,auto,auto), inset: 5pt,
 [n], [SAT variables], [S3 AND gates], [CNF clauses], [XOR rows],
 [83], [10794], [5485], [28666], [1331],
 [131], [16890], [8509], [44938], [2099],
)

Membership gates are included in total variables/clauses, excluded from the
S3 AND column. One static construction per row; no runtime or scaling claim.

#pagebreak()
== Inverse witnesses: exact x-image, degree two
For x≠0 put v=1/x and H(a)=Σ a^(2^(2i)), i=0,…,(n−1)/2. Impose
xv=1, Tr(x)=0, Tr(v)=0 and Tr(xH(v²))=0. This reuses known validation
algebra: US7257709 condition iii already supplies the last test for b=1 on
partially valid cofactor-four keys. No mathematical novelty is claimed.

Rationality is Tr(x+v²)=Tr(x)+Tr(v)=0. With s=√x and u=sH(v),
u²+su+1=0 and u²+u⁻²=x. Thus u is a half-point abscissa; its trace is
Tr(xH(v²)). Vanishing is the second halvability condition. Original point
lifts and full group trace are independently replayed.

The n3 counterexample x=z, v=z²+1 passes Tr(x) and the final trace but
fails Tr(v). Dropping rationality admits a false x. This finite image of
Tₙ under x is not all rational points of a geometric Kummer quotient.
Exhaustive x-image sizes at n3/5/7/9 are0/5/14/63. Raw/compact assignments
have one inverse per x; original point lifts have two signs.

#table(columns: (auto,auto,auto,auto,auto), inset: 5pt,
 [n131 model], [Variables], [Rows], [Degree], [AND aux],
 [Inverse compact global], [260], [132], [2], [16900],
 [Point compact global¹], [260], [131], [3], [43517],
 [Inverse compact d16], [146], [132], [2], [2080],
 [Point compact d16¹], [146], [131], [2], [2200],
)
¹ Historical counts in previous studies, unchanged. Matching x-domains
project to the same x set, with different witnesses. Inverse d16 also has
an extra row and more ANF terms (3627 versus3076). Counts do not determine
solving cost. Both wide inverse-membership singleton controls return
*UNKNOWN* at2seconds/2000conflicts: n83 has21331variables, n131 has52531.

== Compile constants before multiplication
#image("counts.svg", width: 100%)
Target-only and fixed-domain modes preserve all small projected S3 solutions.
Only fully fixed-domain controls were solved here: both SAT with field
replay, zero multiplication gates. They are x-only systems without inverse
membership and do not resolve the UNKNOWN controls above. One construction
per bar; exact recorded counts, no statistical interval or timing ratio.

#pagebreak()
== Complete point chains and a Z[tau] presentation lead
The finite-base point chain retains correlated (x,y) codes. Generic addition
has a nonzero denominator guard. Doubling, opposite points and identity-plus-
point use distinct exact branches. Each model replays every branch and the
original group sum. All100 ordered affine n5 pairs plus wrong-output controls,
all1000 triple branch paths, every small target at arities2/3/4 and sign-
restricted bases agree. Pinned n83/131 identity paths use830/1310variables
and replay G+(−G)+G=G. This completes the finite-base branch core of TZL-03;
unenumerated full-point membership is not yet attached to it.

For E0/F2, τ²+τ+2=0. Existing δ=τ−1 has norm4, and
(t−1)(−t−2)+(t²+t+2)=4. New inverse_cofactor_image applies
[4⁻¹](−τ−2) on the odd-order kernel, rejects nonkernel points and handles
the identity. Complete n5/7 checks establish bijection and the same three-
term relation existence on a signed base and its transported image. Wide
checks replay n131 G/publicQ and n83 G/17G in both directions, including
torsion-coset rejection.

Transporting both base and target preserves the number of relations. The
candidate beyond orbit compression is a different equation presentation:
Frobenius is linear in field coordinates, while a lifted δ map introduces
guarded addition. Does a direct base or its δ-preimage give cheaper complete
solving? The transported x set is not assumed linear. A charged comparison
must retain equal base sizes, corresponding targets and every failed attempt.

== Evidence, certificates and execution dependency
README.md contains exact fixture identities, formulas, source links and
reproduction commands. checks/ retains20 focused tests,10 original and7
lead regressions, root check, two corrected compile attempts and all UNKNOWN.
SOURCES.sha256 pins current code; earlier study evidence remains historical.
counts.json and plot_counts.py reproduce the graph without new computation.

Evaluated certificate IDC1h2a629cbcbb26bb0c records the degree4 integral
norm expansion behind the inverse lift; IDC1he63d40398a440139 records the
degree2 tau factorization. Both use32 issued plus32 local replay points
over2⁶¹−1. They certify the expansions only, not the Frobenius/halving
theorems, characteristic-two subgroup proof or cost. No signed independent
standing or formal proof is attached.

Flow: all listed local checks completed, no matched benchmark rows attempted.
The264-row panel is prepared at source3ae8b2124 with its original executable
hash15c4d1b193999153c6ddc6ec94959167c8f4d016be197b55b26d6993001b1126.
Checkpoint upload fails on missing credentials. The explicit, capped local
EKS/S3 exception is pending. No pilot or cloud job launched; new candidates
need their own pre-data protocol. Whole-root and benchrec gaps on PR3744
remain open. Scoreboard, timeline and performance graphs retain prior values.

Sources: Kosters–Yeo1503.08001; primary US7257709 condition iii
(https://patents.justia.com/patent/7257709); production BinaryCurve halving/
addition, previous trace-zero studies; standard Frobenius characteristic
equation (Sutherland18.783 Lecture13, reference not newly fetched here).
KB unavailable, wrapper127. Existing atlas families cover these mechanisms;
no global novelty assertion. Next deciding checks compare complete successful
work on identical bases for implicit membership, point chains and τ transport.
