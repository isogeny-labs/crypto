#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:true)
= Small-prime transfers and an algebraic shortcut
2026-11-11 · supplement to crypto PR3877 · queue3904

*Standing: unsigned producer assertions.* The opt-in small-prime backend
supports checked odd primes≤4096, with exhaustive enumeration capped at101.
It imports the unchanged repository Fp2 context; this is explicit arithmetic
reuse, not independent validation of that layer. There is no benchmark or
cryptographic-size field claim. [pcc:unverified]

== Fixed correctness controls
#table(columns:(0.5fr,0.6fr,0.8fr,1fr,0.8fr,1fr),inset:4pt,
 [p],[Nonresidue],[Each Jac order],[Rational 2-torsion],[Source E order],[r; generator],
 [13],[2],[180],[4 / 4],[10],[5; (10,6)],
 [29],[2],[960],[4 / 16],[40],[5; (24,15)],
 [31],[3],[1008],[16 / 4],[28],[7; (19,2)],
 [101],[2],[9216],[4 / 16],[96],[3; (48,20)],
)
The original13/29/101 fields were fixed before execution. The separate31
control varies the automatic nonresidue and tests a descending2-torsion case.
Every rational class, both dual norms, equal fibers and odd-subgroup scalar
relations pass. Both F101 hops agree exhaustively with the previous model
backend. Equal2-torsion counts at13 do not establish an isomorphism.
[pcc:unverified]

The final feature suite passes25 tests and strict Clippy; the root default
example import check passes. The unchanged BigUint reference compares all
eight complete class sets, independently counts Jacobian orders and
self-inverse elements, and checks additions and dual norms. Its chart adapter
uses BigUint directly. Independent elliptic quotient orders are10/18,40/24,
28/36,96/96. The Fp2 counter shares the existing algorithm lineage, so no
signed independence is inferred. [pcc:unverified]

Receipts are frozen in evidence/, with runtime inputs in
small-prime-inputs.sha256. Claimscd7c2f and cryptanalysisdb1613 are unverified.
The first family reference run is development evidence; an immutable source
pin/replay remains a separate step. Earlier reports and raw failures remain
retained with their original scope.

== Algebraic endpoint control
For the fixed c=4 source, in every characteristic except2,3,5:
Cₛ:y²=(x²−1)(x²+x+4)(x²−x+4), and
Cₜ:w²=(z⁶−102z⁴+393z²−4)/500. Normalize the target quotient by
X=z²/500,V=w/500. Shift source U=u−1; its standard2-isogeny is
phi(U,v)=(U+9+24/U,v(1−24/U²)). The codomain isomorphism is
tau(X,V)=(X/100+1/125,−V/1000). The written argument gives
n R f∗ = tau phi [2]. It includes the actual base-image translation by
(1/125,0). [pcc:unverified]

Seven cleared ring lemmas compile with the standard axiom gate. Total proof
inventory is25 written lemmas,22+8+7 audited declarations. Full projective
and Picard geometry is a written argument using the primary sources, not a
Lean construction. Claims ec195a and6f6202 remain unsigned. This result
concerns the first elliptic projection; it is not asserted as a collapse of
the later Jacobian-valued two-hop map. [pcc:unverified]

#pagebreak()
= The fiber calculation and remaining work
On the generic correspondence fiber put a=x²−1,b=x²+x+4,c=x²−x+4,
A=x²−x−6,D=9x²+x−4,M=2x³+19x²+7x+74,
N=8x³+x²−47x−4. The fiber equation is A z²+10b z−D=0. Its projected
two elliptic points lie on V=mX+n with
m=aM/(10yA),n=−aN/(5000yA).

The Lean-checked identities simplify their sum to
X₀=1/125−3a/(20bc),V₀=−3y(a²−24)/(200b²c²). The source base’s
correspondence image projects to T=(1/125,0), so it must be subtracted.
After translation by T the coordinates become
X=1/125+bc/(100a),V=−y(a²−24)/(1000a²), exactly tau phi(f(P)).
Agreement on this dense open set extends along the actual morphisms of smooth
projective curves. Linearity on complete degree-two fibers then gives
n R f∗ = tau phi [2]. The extension/Picard step is explained in
GLOBAL_SUCCESSOR_COLLAPSE.md and uses the cited primary geometry; it is not
established by the ring lemmas alone. [pcc:unverified]

Native checks cover every rational affine curve point, both original
infinities, the actual base image and all elliptic whole fibers over
13,29,31,101. The second target norm is zero on these whole-fiber images.
These controls retain degree drops and ramification; finite agreement is
separate from the written geometric argument. [pcc:unverified]

== Reproduction
Use native Cargo feature small-prime, strict Clippy and the repository proof
gate on ChainTransfer.lean,PolynomialIdentities.lean,SuccessorCollapse.lean.
The native package remains dependency-free but imports the external unchanged
src/prime_hyperelliptic/fp2.rs file. CI checkouts and consumer source guards
must include that source. The focused crypto workflow is not yet registered
on main; do not claim a feature-branch dispatch ran there.

Cryptanalysis PR1469/1471 are merged. PR1473 freezes the prior two-hop
checkpoint at aabae197, with28 local tests and1 existing ignored benchmark.
The small-prime feature requires a later explicit dependency/source pin.
No field-size extrapolation follows from the four bounded fixtures.

== Open gates and next tasks
S3 checkpoint admission credentials and live remaining IsoLab budget/capacity
are unavailable. No benchmark has run. KB/ecdb access is unavailable and
crypto-autoresearcher is vault locked; its handoff remains outside the vault.
Crypto’s current-main ledger-prefix fork and historical missing benchmark
receipts still block merge. Owner rules require extending the existing PR
and preserving both histories, so no alternate ledger lineage or rule bypass
has been introduced. The long full-library run remains separate from the
passing focused checks and has reported untouched historical failures.

Next: immutable feature oracle replay; examine the simple quadratic cover’s
restriction for elliptic full groups of odd order and test an alternative
odd-degree cover if warranted. Prime-field descent and computational
advantage remain open. The explicit shortcut is a required comparison arm;
functional equivalence itself is not a measured speed ratio.

Primary sources: Galbraith corrected book ch.8 §8.3,ch.10 §§10.3–10.4/10.6;
Kunzweiler,DCC92(2024),1761–1802,DOI10.1007/s10623-024-01366-1,
Propositions7–8. No novelty claim is made.
