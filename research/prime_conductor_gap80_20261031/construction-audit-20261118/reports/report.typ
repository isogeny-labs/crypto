#set page(paper: "a4", margin: 16mm, footer: context align(right)[#counter(page).display()])
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.5em)
= Checking the missing construction
Frozen 80-bit ordinary source · 18 November 2026 · draft PR 4723

*Observed:* two explicit degree-2 CM endomorphisms agree with PARI on
128 images. The frozen 80-bit kernel K and its CM image α(K) have a
primitive Weil pairing. A dependent replacement rejects. The kernel line
is not CM-stable. This check evaluates endomorphisms on the source; it
does not evaluate the degree-ell isogeny. [pcc:unverified]

== An explicit source action
For E: y²=x³−140x+784 and s²=−7, put r=7+s, u=(1−s)/2
and t=3r²−140. A normalized degree-2 quotient is

  x′=x+t/(x−r),    y′=y·(1−t/(x−r)²).

Its coefficients satisfy A′=u⁴A, B′=u⁶B. Dividing coordinates by
u²,u³ gives an endomorphism α of the original source. Its trace is 1,
norm is 2, and α²−α+[2]=0. The conjugate action is 1−α.

#block(inset: 10pt, stroke: 0.7pt + rgb("#346b92"))[
  *Actual degree-2 source map:* K → α(K), both on E. \
  *Torsion basis:* (K, α(K)); the pairing is a primitive ell-th root. \
  *Action matrix:* α has columns (0,1), (−2,1) in this basis. \
  *Still missing:* quotient E′, the degree-ell map and its non-kernel images.
]

Since ell is inert in the maximal CM order of discriminant −7, the
characteristic polynomial is irreducible modulo ell. The native pairing
check confirms the frozen kernel is not a CM-invariant line. In particular,
it is not the norm-ell ideal input used by a horizontal class-group action.
This refines the method's input contract without proving that no other
construction can work.

== What passed and what did not
#table(columns: (1fr, 1.5fr), inset: 5pt, stroke: 0.4pt,
  [*Item*], [*Outcome*],
  [CM images], [128 native/PARI matches, both conjugates, kernel and q-generator panels],
  [Group identities], [Conjugate sum/product and characteristic identities pass on 32 distinct scalars per panel],
  [Weil pairing], [Exact prime order; dependent pair rejected with empty stdout],
  [Full target crossing], [No new quotient or non-kernel image],
  [Endpoint measurements], [No scalar, ECDLP or IC timing],
)

PCC-20261118-ef3abf binds the raw outputs and frozen inputs. Its producer
attestations are unsigned, standing unverified and verdict `not_covered`.
The failed signed-input attempt is retained separately. [pcc:unverified]

#pagebreak()
= A concrete limit of the single-power route
Using the frozen CM trace T, write π=A+Bα with
B²=(4p−T²)/7 and A=(T−B)/2. The checker verifies norm p and trace T.
The sign choice of B is conjugation and preserves the modular order.
The source's prior maximal-order and trace evidence are inputs here;
the ambient point count is not recomputed.

In O/2ᵐO, πᵏ=1 exactly when full E[2ᵐ] is rational over the
degree-k extension. Native ring squaring and separate PARI modular-matrix
powering agree at all 80 checked levels. Each nontrivial order includes
a rejection at half the claimed power. [pcc:unverified]

#image("torsion-orders.svg", width: 100%)

#table(columns: (1.6fr, 0.5fr, 1fr), inset: 5pt, stroke: 0.4pt,
  [*Specific requirement*], [*m*], [*Full torsion field degree*],
  [2²ᵐ ≥ 4ell+1 (cardinality condition)], [41], [2³⁹ = 549755813888],
  [2ᵐ > ell (single-power embedding)], [80], [2⁷⁸ = 302231454903657293676544],
)

These are field degrees, with no runtime extrapolation. Other powersmooth
choices and source fields remain open. Robert's representation algorithm
still needs initial images under the unknown map; the representation
theorem alone does not provide them.

== Retained failures and scope
Build A used an unavailable PARI API. Build B compiled, but the new
checker's unsigned conversion parsed −140 as zero. Attempt-a therefore
used the wrong PARI curve and is invalid for the intended source. A local
signed wrapper fixes both negative coefficients and matrix entries. Build C
passes with warnings denied; the new attempt-b and negative control pass.
Original source, executable, manifest and failed outputs are retained.
Existing parent callers already canonicalize coefficients and are unchanged.

This was fixed-input local development validation: 120-second cap per
call, 256 MB PARI stack, no paid compute or timing claim. The original
80-bit crossing and all endpoint measurements remain open. The configured
S3 store still blocks the earlier archive's verified publication. [pcc:unverified]

Sources: Robert, *Evaluating isogenies in polylogarithmic time*,
Theorems 1.2/1.3 and §5 (author PDF dated 25 August 2022);
*The Isogeny Problems*, ordinary discussion and Problem 5.2 (July 2026);
Rueger §4.4; PARI pairing reference; Sutherland Lectures 13/16.
README.md and sources.json carry exact URLs/digests. KB wrapper unavailable.
