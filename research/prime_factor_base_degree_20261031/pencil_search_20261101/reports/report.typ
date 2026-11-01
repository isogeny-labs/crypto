#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10pt)
#show heading: set text(fill: rgb("#20495b"))
#show raw: set text(font: "DejaVu Sans")
= Boolean offsets of a quadratic ordinate base
2026-11-01 · construction and checked degree continuation

A complete two-summand family now has a Lean-checked canonical homogeneous
degree law: *2k+3*, including all 2k Boolean selector variables. The factor
base contains every rational point with y=±(a x²+b x+c+t0+sum(wᵢuᵢ)),
uᵢ²−uᵢ=0. Both slots' labels remain variables; individual shifted graphs
need not split completely. Empty fibres supply no points.

#image("construction.svg", width: 100%)

== A larger exact fixture
Curve #raw("icv1-fp5-tm1-a77cc6af") is y²=x³+2x+2 over F17.
Use h0=4x²+9x+9 and weights [2,10]. The offsets {0,2,10,12} give positive
fibre sizes [4,1,2,2]. Eighteen signed labels deduplicate to *16 signed
points at 8 abscissae*: {0,3,5,6,7,9,10,16}, twice the old fixture's sizes.
The new base is #raw("FB1h29cc024e755c"). Its full selector has canonical
degree *7*, versus *9* for the same-base dense membership/S3 presentation.

Lean proves exact finite membership and cardinalities, actual polynomial
root/group equivalence, and raw-root soundness without assumed membership.
All signs, tangents and repeated abscissae are retained. Infinity has a
separate exact inverse-pair theorem. Metadata remains unregistered; group
order is inherited, not newly proved.

This is a hand-derived fixture selected after viewing the older F17
example, not a prospective sample or natural-yield measurement. Duplicate
labels at x=9 are not extra columns. No external novelty is established.

#pagebreak()
= Degree certificate, variable tax and next experiment

The actual highest forms normalize to the core
(m², xz, x²−alpha·mx, z²−beta·mz) plus one square for every Boolean variable.
Every degree 2k+3 monomial either repeats a bit or has core degree at least 3,
so it vanishes. A degree 2k+2 witness m·x·product(bits) survives via a map to
a pure-square monomial quotient. The hypotheses are nonzero quadratic
coefficients and characteristic other than 2; the target and all lower
coefficients are arbitrary. FORMALIZATION.md maps claims to exact signatures.

== The remaining cost comparison
The seven-variable degree 7 full affine monomial envelope is
choose(14,7)=3432 columns; the two-variable dense degree 9 envelope is
choose(11,2)=55. These are *derived ambient budgets*, not measured matrix
widths. Lower canonical degree alone does not establish lower complete solving work.

Exact Boolean projection is proved even for zero weight:
exists u in {0,1}: d+l·u=0 iff d(d+l)=0. Eliminating one bit per slot removes
two variables and gives quartic residuals. Its actual canonical degree and
solving cost still need checking. The universal elimination identity is
kernel checked separately from evaluated-tier IDC1hb79cfe74b833e0be.

== Validation and execution status
The unchanged transitive axiom audit passes *240 declarations in 18 modules*,
with zero failures and zero native trust. All five new modules pass
warnings-as-errors. Thirteen native unit tests, formatting and clippy pass;
the deterministic FB1/fixture/identity export replays byte-for-byte.
Lean 4.19.0 and nine Git dependencies are pinned and checked. Forty source
inputs are hashed. Compiler failures and accepted source are preserved.
lean4checker is unavailable; no fresh checker pass is claimed.

No new timing, affine F4 degree or relation-yield measurement has run.
PROTOCOL.json records the matched panel; its full runner and controls are
not yet implemented. EKS arbitrary-command submission is not configured
here, and the inspected GitHub path accepts ecbench specifications. The old
local exception is not reused. Original P-256/17-term gates remain open.

== Sources and replay
See reports/REPORT.md, FORMALIZATION.md and verification/final/ for exact
scope, hypotheses and receipts. Run #raw("bash verify.sh NEW_RECEIPTS_DIR").
The prior 24-design register and atlas already cover learned coordinates,
digits and generic towers. Primary PKM 2016:
#link("https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf").
Actual group laws reuse Mathlib and Angdinata–Xu, ITP 2023:
#link("https://doi.org/10.4230/LIPIcs.ITP.2023.6").
