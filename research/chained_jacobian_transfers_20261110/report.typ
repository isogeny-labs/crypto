#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: true)
#set heading(numbering: "1.")
= Native chained Jacobian transfer over F₁₀₁
2026-11-11 · continuation of crypto PR #3877 · queue issue #3904

*Standing: unverified producer assertions.* Local correctness checks and kernel
proof builds are retained. There is no independent signed validation and no
solving-cost benchmark. Full report and sources: REPORT_20261111.md and
SOURCES_20261111.md beside this PDF.

== Explicit route
The source is E: v²=u³+1 with P=(4,41), order 17. The degree-two cover
f: C → E is f(x,y)=(x²,y), C: y²=x⁶+1. Its whole-fiber pullback
enters Jac(C). The ordered quadratic splitting
(x²+1)(x²+10x−1)(x²−10x−1) has determinant 40 in F₁₀₁.
Kunzweiler’s Richelot correspondence gives a degree-four (2,2)-isogeny
to Jac(C₂), C₂: w²=41z⁶+9z⁴+9z²+41.

The first quotient of C₂, with X=41z² and V=41w, reaches
E₂: V²=X³+9X²+66X+39. The local test obtains (91,84) from P.
All 17 source scalars and all 289 subgroup addition pairs are checked.
The second quotient sends this image subgroup to zero; it remains an explicit
negative control. [pcc:unverified]

== Native maps and exceptional points
The source uses the odd-degree chart t=1/(x−10), Y=yt³:
F(t)=1+60t+86t²+2t³+15t⁴+60t⁵. Canonical Mumford pairs represent
degree-zero classes based at W=(10,0). This avoids using one-infinity Cantor
arithmetic directly on a sextic.

Whole-fiber pullback handles nonsplit and ramified fibers and the degree drop
at (−1,0). Both source quotient norms accept every reduced rational class,
split quadratic support in F₁₀₁[s]/(s²−2), retain multiplicity, and handle
the two original infinities. Elliptic infinity and unsupported input are
different API outcomes. [pcc:unverified]

The Richelot evaluator computes polynomial fibers over F₁₀₁², composes
them on an odd target chart, and subtracts the source-base-point image.
The extended version admits all 10,404 rational source classes, including
Weierstrass, source infinity and degree-drop cases. Kernel size is four;
the 2,601 image classes each have four preimages in the local checks.
This is fixed-field coverage; map evaluation never calls a DLP solver.
[pcc:unverified]

== Development evidence
#table(
  columns: (1fr, 2fr), inset: 5pt, align: left,
  [Check], [Scope and evidence],
  [Native whole-fiber maps], [102 elliptic inputs; all ordered additivity pairs],
  [Source classes], [10,404 canonical classes; both norms and three translations],
  [Differential reference], [10,404 additions/inverses, unchanged cryptanalysis BigUint code],
  [Richelot chain], [17 scalars; 289 addition pairs; nonzero and zero endpoint controls],
  [Dual norm], [All 10,404 classes: dual-after-forward equals multiplication by two],
  [Elliptic shortcut], [All 102 inputs: full route equals τ φ after doubling],
  [Lean structure], [8 structural + 6 polynomial lemmas; 20+6 declarations audited],
)
These are local development checks, not independent signed validation. Both
Jacobian implementations use Cantor arithmetic. Raw receipts and failed
attempts live in evidence/; claim standings remain unverified.

#pagebreak()
== Proof map and prime-field boundary
Core Lean 4.35.0 is pinned. The two-hop norm lemma multiplies the individual
dual norm identities. A Bézout certificate and the actual source subgroup
annihilator then give recovery and injectivity. Anti-invariance yields
two-torsion; odd subgroup exponent yields zero. The existence of the geometric
objects and their norm identities is explicit input, not proved by these
structural lemmas. Six further polynomial lemmas prove the cleared source and
target identities, the F101 chart, and the elliptic endpoint isomorphism/inverse.
Only the standard foundations propext, Classical.choice and Quot.sound occur.
[pcc:unverified]

A separate lemma shows that a unital additive map from characteristic p
to characteristic q requires q dividing p. This excludes field maps for
distinct prime characteristics. It says nothing about arbitrary efficient
maps between encoded cyclic subgroups. A prime field has no proper subfield,
so ordinary Weil restriction does not provide a smaller-base-field route.
Lifting and cross-prime reduction remain separate open mechanisms.
[pcc:unverified]

Claims: PCC-20261111-ca60cb (native maps), PCC-20261111-3f548f (Richelot
fixture), PCC-20261111-7a3fe0 (conditional chain injectivity),
PCC-20261111-a00fb9 (characteristic boundary). All are unverified until a
signed verdict establishes otherwise. Geometry, exact Richelot kernel, and
the full Picard bridge are not yet formalized in this file.

== Reproduction and unresolved work
From crypto, add Cargo and Lean to PATH and run the native Cargo tests, then
tools/lean/proof-gate.sh on lean/ChainTransfer.lean. The root examples
chained_jacobian_reference and chained_jacobian_identities provide the
BigUint comparison and denominator-cleared identity certificates. The
cross-repository oracle is cryptanalysis:experiments/chained-transfer-reference.

Retain Lean attempt 01 (failed elaboration) and root-library attempts
01–03 (off-disk dependencies); never treat them as passes. The canonical queue
and resume state record the latest build outcomes and exact next actions.

No remote experiment has been launched: S3 checkpoint transport is unavailable
and the live remaining IsoLab budget/capacity is unknown. The research-KB
MCP/CLI is unavailable, and crypto-autoresearcher is vault locked without its
key binding. Local unit tests and proof development remain possible.

The full chain equals a cheaper expression on all 102 source elliptic points:
τ φ after doubling, where φ is the historical 2-isogeny and
τ(U,V)=(82U+60,98V). The reverse correspondence, with its sign corrected,
provides multiplication by two on every rational source Jacobian class.
Source norm then gives multiplication by four, invertible modulo 17. Point
recovery is distinct from solving its unknown scalar. [pcc:unverified]

Next obligations: publish the full target-side differential evidence, obtain
an independent signed validator, generalize the fixed-field backend, and freeze
a total-cost comparison. That comparison must include the collapsed elliptic
route as a separate arm. There is no measured speedup in this report.

== Primary sources
- S. Galbraith, Mathematics of Public Key Cryptography, corrected version 2.0,
  ch. 8 §8.3 and ch. 10 §§10.3–10.4, author-hosted full PDFs.
- S. Kunzweiler, Efficient computation of (2ⁿ,2ⁿ)-isogenies, Designs, Codes
  and Cryptography 92 (2024), 1761–1802. DOI 10.1007/s10623-024-01366-1,
  §3.1–3.2, Propositions 7–8 and Algorithm 1. Full publisher PDF read.

The local route diagram has no leaderboard promotion. Canonical transfer and
IC graph sources remain unchanged pending their evidence gates.

#pagebreak()
= Explicit route and zero controls
#image("route.svg", width: 100%)

All nodes and maps are over F₁₀₁. The solid path is the explicit development
fixture; dashed edges are the retained vanishing projections. No edge denotes
a measured computational advantage. [pcc:unverified]
