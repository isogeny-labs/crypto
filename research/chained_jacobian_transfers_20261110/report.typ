#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.")
= Chained Jacobian transfers over F₁₀₁
2026-11-11 · crypto PR #3877 · cryptanalysis PR #1469 · queue #3904

*Standing: unverified producer assertions.* The initial Jacobian target is
isomorphic to its source. A second fixture has nonisomorphic Jacobians over
F101, distinguished by rational 2-torsion. Both complete elliptic endpoint
functions match direct elliptic shortcuts. There is no measured DLP advantage.
Full report, primary sources and raw receipts are retained beside this PDF.
[pcc:unverified]

== Initial explicit route and controls
E: v²=u³+1, P=(4,41), r=17. The degree-two cover f(x,y)=(x²,y)
from C: y²=x⁶+1 induces whole-fiber pullback into Jac(C). The ordered
splitting (x²+1)(x²+10x−1)(x²−10x−1) has determinant40.
Kunzweiler’s normalized Richelot correspondence gives a degree-four map R
to Jac(C₂), C₂: w²=41z⁶+9z⁴+9z²+41.

The first target quotient, X=41z²,V=41w, reaches
E₂: V²=X³+9X²+66X+39 and sends P to (91,84), order17. The
second target quotient annihilates this subgroup, as does the anti-invariant
source norm on f∗(E). These are explicit zero controls. [pcc:unverified]

Canonical classes use t=1/(x−10),Y=yt³ with odd quintic
F(t)=1+60t+86t²+2t³+15t⁴+60t⁵. Whole fibers include nonsplit
support, multiplicities, Weierstrass points, both original infinities and
degree drops. Unsupported input and elliptic identity are different outcomes.
All10,404 source classes are admitted; kernel4 and image2601 have equal
four-element fibers. Both sign-corrected dual compositions give[2] on the
complete source and target. Source norm gives[4] on E, inverted by13 modulo17
to recover encoded points. This does not recover unknown DLP scalars.
[pcc:unverified]

== Two retained equivalences
C₂ is F101-isomorphic to C. Put r=10,r²=−1:
x=(z−1)/(z+1),y=8r w/(z+1)³. Its inverse is
z=(1+x)/(1−x),w=−r y/(1−x)³. The aligned odd charts obey
t=56+10T,Y=79Z. Every10,404 rational class round-trips. Cleared identities
compile in Lean; projective/Picard geometry is not formalized. R becomes a
nontrivial endomorphism after model identification, not a new destination.
[pcc:unverified]

On all102 source elliptic points the full endpoint equals τ φ[2], where φ
is the historical degree-two elliptic quotient and τ(U,V)=(82U+60,98V).
Future cost comparisons must include this direct shortcut. No cost ratio
follows from finite functional equality. [pcc:unverified]

#pagebreak()
== Nonisomorphic successor and its limitation
The model-tagged backend accepts ordered quadratic splittings and rational
Weierstrass bases over *fixed F101*. It rejects mixed-curve/chart arithmetic;
it does not yet generalize fields. The preselected successor is
G₁=x²−1,G₂=x²+x+4,G₃=x²−x+4, determinant91=−10:

- Cₛ: y²=x⁶+6x⁴+9x²−16, base(1,0).
- Cₜ: w²=20z⁶+81z⁴+83z²+21, base(2,0).

Each Jacobian has9,216 rational classes; all are admitted. Transfer fibers
have size4 and both dual compositions are[2]. The source rational2-torsion
has cardinality4 and target16. The unchanged BigUint reference independently
recounts these as self-inverse classes, obtains order9,216 from Fp/Fp² point
counts, and checks paired additions and dual norms on both complete sets.
The first reference run used the development tree; immutable pin/replay is
tracked separately. [pcc:unverified]

An F101 abelian-variety isomorphism induces a rational group isomorphism
commuting with[2], hence equal rational2-torsion cardinalities. Uniqueness of
reduced Mumford representatives gives a second derivation: self-inverse pairs
have v=0 and monic u dividing the odd quintic, degree≤2. The source factor
pattern is1+2+2 with both quadratics irreducible, so there are4 choices. The
target has five rational quintic roots, giving1+5+10=16 choices. Closed
quadratic support must be counted. Thus these Jacobians cannot be
F101-isomorphic; no algebraic-closure nonisomorphism is asserted.
[pcc:unverified]

The source elliptic quotient has96 points, largest odd prime factor3,
P=(48,20). Its transferred class is nonzero of order3, first target norm
(2,72), second norm zero. The normalized first target elliptic curve is
V²=X³+81X²+44X+17. This tiny subgroup is retained explicitly rather
than substituted for the original r=17 fixture. [pcc:unverified]

The successor endpoint also equals a direct shortcut on all96 source points.
Shift U=u−1, so Y²=U³+9U²+24U. The standard quotient by(0,0)
is V²=X³+83X²+86X; τ(X,V)=(100X+80,91V) identifies its target
with the endpoint. The full function equals τ φ[2]. All F101 affine
coordinate isomorphisms satisfying the coefficient identities were checked;
this sign uniquely matches. A nonisomorphic Jacobian hop still does not
establish a new elliptic reduction or a computational advantage.
[pcc:unverified]

Claims: PCC-20261111-a282a9(model equivalence), ae88cc(successor),
6e9e6b(successor shortcut); cryptanalysis21e28e(reference development).
All remain unsigned and unverified.

#pagebreak()
== Formal obligations and prime-field boundary
Core Lean4.35.0 is pinned. Ten structural lemmas cover norm composition,
Bézout recovery/injectivity, scalar relations, anti-invariant zero controls,
torsion invariance and the characteristic obstruction. Eight polynomial
lemmas cover cleared splitting, target numerator, chart and coordinate
isomorphism identities. Gates audit22+8 declarations with standard foundations
only(propext,Classical.choice,Quot.sound), no trusted native declarations.
Actual Jacobians and their dual norm identities remain explicit hypotheses
of the structural proofs. [pcc:unverified]

A unital additive map from characteristic p to q requires q dividing p.
For distinct primes this forbids field maps, not arbitrary efficiently
encoded cyclic subgroup transfers. A prime field has no proper subfield, so
ordinary Weil restriction provides no smaller-base-field descent. Lifts and
cross-prime reductions remain separate open mechanisms. [pcc:unverified]

The full projective/Picard/Richelot bridge remains open. Claims7a3fe0,
a00fb9,a97961,ced5d7 are unverified until their actual signed standing changes.
The two new proof records initially omitted theorem bindings; their initial
incomplete bytes are retained and the correction appends ledger events,
without altering any ledger history or published claim.

== Reproduction, checks and gates
Run the standalone native Cargo tests and strict Clippy, then both Lean
files through tools/lean/proof-gate.sh. Root examples
chained_jacobian_reference and chained_jacobian_identities pass16
and1 tests, including the second-hop extension. The native suite passes15
tests and strict Clippy. Two local
replay-accepted identity certificates bind the polynomial Lean source digest.
These are development checks, not signed independent validation.
[pcc:unverified]

Cryptanalysis PR1469 freezes the original baseline at crypto18db046 with
source/reference digests and a fail-closed check script. Original receipts
remain immutable. The full crypto library run is still in progress and has
reported failures in untouched historical modules. Keep failed elaboration,
missing-dependency and lint attempts; never label them passed.
[pcc:unverified]

No performance experiment has been launched. S3 checkpoint admission is
unavailable, and live IsoLab budget/capacity is unknown. The research-KB and
ecdb are unavailable; crypto-autoresearcher is vault locked without a key
binding. The current-main PCC ledger fork is a merge blocker, not something
to repair by rewriting history. The new crypto workflow requires default-branch
registration before dispatch; the reference workflow is registered. GitHub CI and required reviews still
apply. The queue names every blocker and next action.

Next: immutable second-hop replay, checked small-prime field support,
independent signed validation and an admitted total-cost
comparison including both direct elliptic shortcuts. Canonical transfer and
leaderboard graphs receive no promotion from this unsigned fixed-field work.

Primary sources: Galbraith, corrected Mathematics of Public Key Cryptography,
ch.8 §8.3 and ch.10 §§10.3–10.4,10.6; Kunzweiler, DCC92(2024),1761–1802,
DOI10.1007/s10623-024-01366-1, Proposition1 and §§3.1–3.2. Full PDFs were
read; retrieval gaps and exact hashes are in SOURCES_20261111.md.

#pagebreak()
= Explicit routes and controls
#image("route.svg", width: 100%)

All objects and maps use F101. Solid edges are actual development maps;
dashed edges are zero controls; dotted edges are complete finite endpoint
shortcuts. The successor’s different rational2-torsion counts obstruct a
base-field Jacobian isomorphism. No edge denotes a measured DLP advantage.
[pcc:unverified]

#pagebreak()
= A second nonbacktracking Jacobian hop
The next pairing was fixed before execution as
(z−2)(z−18), (z+2)(z−28), 20(z+18)(z+28), preserving Cₜ’s exact
scale20 and base(2,0). Its determinant is67 and first target factor
23z²+66z+26 has roots11,52; the fixed minimum-root rule chooses11.
The next curve is Cᵤ: a²=8b⁶+17b⁵+86b⁴+62b³+29b²+9b+38.

#image("chain.svg", width: 100%)

Both rational class sets have9,216 elements. The next map has equal
four-element fibers and both dual norms[2]. Target rational2-torsion has
cardinality8. The pre-existing BigUint reference independently obtains
point-count order9,216, self-inverse count8, paired additions and dual norm
on all9,216 classes. The three counts4,16,8 distinguish all three Jacobians
over F101, without asserting algebraic-closure nonisomorphism. These are
unsigned development assertions; immutable third-model replay is next.
[pcc:unverified]

The next kernel intersects the previous dual kernel only at zero. Both
four-element2-torsion kernels are rational, so this checks the full
intersection and excludes an immediate reverse hop. Under the standard
separable Richelot geometry, the degree16 geometric composite kernel is
killed by4 and has2-torsion cardinality4, hence is (Z/4Z)². The *rational*
composite kernel has only8 elements,4 self-inverse, hence Z/4Z×Z/2Z.
Not all geometric4-torsion need be rational. The counts are execution
assertions; the geometry and finite-group classification are not fully
formalized here. [pcc:unverified]

For source P=(48,20), the final class based at(11,0) is
u(T)=T²+86T+27,v(T)=34T+7, nonzero of order3. Reversing both maps and
applying the source norm gives[8]P, inverted by2 modulo3. All three scalar
relations are checked with no DLP solver used in evaluation. Point recovery
does not reveal the unknown scalar. No elliptic quotient for this final
non-even sextic has been implemented; the earlier elliptic shortcut results
are not claimed as identities of the final Jacobian-valued map.
[pcc:unverified]

The new core Lean lemma composite_kernel_on_torsion states the abstract
kernel-intersection step with actual maps/norms as hypotheses. Proof inventory
is now18 written lemmas,10 structural and8 polynomial; axiom gates audit22+8
declarations with standard foundations only. Final native suite passes15
tests and strict Clippy. The full geometric bridge, signed standing and a
measured computational advantage remain open. [pcc:unverified]

Receipts: native-second-hop-01(failed compilation),02(first passed fixture),
final-01(full15-test suite), lean-second-hop-01. Claims53535c and795c4d;
cryptanalysisad0e63. The independent successor replay is published in
cryptanalysis PR1471 at c6bc590, pinning cryptob280efc,27 tests/1 ignored
benchmark; focused CI run42986170752 was dispatched on that exact head.
Source report and queue retain actual final-head CI status separately.
