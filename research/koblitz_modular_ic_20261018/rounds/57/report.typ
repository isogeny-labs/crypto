#set page(paper:"a4",margin:16mm)
#set text(size:10pt)
= R57: the complete prescribed-pole space has 24 sections
The actual degree-80 pole divisor gives a 24-dimensional space, obtained by
restricting all 68 sections of the retained complete degree-128 system.
All 48 local conditions were imposed; their exact rank is 44. The four
additional sections beyond the nonspecial count are retained in full.

#image("spaces57.svg",width:100%)
Recorded exact counts: space57-02 and trace57-02. No sampling or error bars.

== Complete local restriction and descent
R56 supplies D=28 R07+4 R01+7 R17+R11. Each closed point has residue
degree 2. R27 supplies D0=49 R07+7 R01+7 R17+R11. The 68 global
F4 descent identities and all 20 cusp-coordinate bindings pass. Removing
21 ramified and 3 unramified Laurent coefficients, each over F4, gives
all 48 binary conditions. Sigma partners use the same local parameter.
The invertible row change identifies the ordinary and descended kernels.

The complete certificate checks C K=0, R K=I and K R+H C=I. Thus every
permitted ambient vector belongs to the supplied 24-column kernel. All
1,360 ambient cusp evaluations and the 129-jet rank-68 independence check
pass. Controls 1,A,lambda belong; actual b is excluded in six rows.

== Actual finite-pole functions
All 24 sections are bound to source functions over F2(u), with full source
degree 32. Every divided function satisfies A × (f/A)=f. Since div(A)=P-D,
where P is the sum of four reduced degree-20 points, division by A identifies
the complete spaces L(D) and L(P). The generic inverse of A and four
actual descended-coordinate controls pass. CHECK57 independently replays
all 24 source reconstructions, all 24 division identities and matrix products.

== Four fixed classes modulo traces
The actual involution has N=sigma+I with N squared zero and rank 10.
Its fixed space has dimension 14; modulo its trace image it has dimension 4.
TRACE57 exports complete splitting/recovery matrices, four actual fixed
functions and ten actual trace preimages. All 24 source substitution identities
are checked. Constant 1 trace membership in L(D): 1.
Lambda has trace A; consequently lambda/A has trace 1 in L(P).

#pagebreak()
= Checks, failures and next construction

== Every attempted workflow
#table(columns:3,inset:4pt,[Attempt],[Terminal status],[Native wall diagnostic],
[check57-01],[accepted],[1 s],
[field57-01],[accepted],[0 s],
[match57-01],[rejected],[2 s],
[match57-02],[accepted],[2 s],
[restriction-lean57-01],[rejected],[not recorded],
[restriction-lean57-02],[accepted],[not recorded],
[space57-01],[rejected],[2 s],
[space57-02],[accepted],[75 s],
[trace57-01],[rejected],[1 s],
[trace57-02],[accepted],[0 s],
)
Native wall is the difference of retained UTC timestamps at one-second
resolution; zero means same-second endpoints. It covers the native invocation
only and is not a comparative benchmark.
CPU time and Lean compile/audit wall intervals were not recorded.

Four rejected attempts are preserved: match57-01 used unsupported whole-ideal
equality; space57-01 used reserved name denominator; restriction-lean57-01
needed explicit kernel-membership unfolding; trace57-01 trimmed trailing-zero
vector rows and used an incorrect live matrix ring label. Late native markers after errors
are excluded. Every repair was frozen before its affected data. No accepted
producer was rerun to publish results.

== Relevant Lean implications and exact premises
Four authored declarations compile and pass a separate axiom audit, zero
failing and zero trusted. They prove complete kernel recovery, coordinate
uniqueness, row-change kernel preservation and inherited involution.
Actual matrix arrays remain native premises; proper genus-61 ambient RR
completeness and local divisor geometry remain geometric premises. Given
those hypotheses, degree 80 and dimension 24 yield RR residual dimension 4.
The numerical four-dimensional fixed quotient is not a factor identification.

Native jobs use seed 57, one Singular/FLINT thread, 4 GiB virtual and a
1,200-second cap. Lean 4.34.1 and mathlib d13f23b use one worker, 4 GiB
managed memory and separate 1,200-second compile/audit caps without a
virtual limit. Sources/protocols/drivers are copied immutably and byte-bound
to Git before data. Read results.json, export.json and PROOF_MAP.md.

== Next source-bound construction
The ordinary coordinate y/x has a double pole at (0,1); written local
geometry therefore excludes its pullback from the entire simple-pole space.
Next construct complete L(2P), dimension 100, from a certified196-section
quadratic ambient and all96 restriction coefficients; see LEADS.md.

Use all 24 finite-pole sections, the ten explicit trace preimages and four
fixed-quotient representatives. Impose complete residue conditions for
smaller subsets of the four points; then test modular target identities,
F2 descent and the intended order-11 action. Keep proper polarization,
separate incidence saturation, coupled node187 divisors, arbitrary reverse
classes and larger 112/991 sources as distinct retained lanes.

Sources reviewed: R27 complete RR/cusp code; R43 descent; R50 actual field;
R55 zero quotient; R56 complete poles; Stacks RR tag 0B5B. No claim of
novel restriction linear algebra. All recorded terminal source/protocol/driver
pins are checked against their original Git revisions before publication.
