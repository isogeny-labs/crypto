#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
#show heading: set text(fill: rgb("#20495b"))
= A checked secant-to-group bridge
2026-10-31 · Generic short Weierstrass curves over fields

The standalone Lean project proves that the four secant residuals vanish for
some slope exactly when the corresponding nonsingular finite points satisfy
P + Q = R in Mathlib's actual elliptic-curve group. Tangents, repeated
abscissae and flexes are included. The identity target is separately and
exactly characterized by inverse pairs, including possible two-torsion.

#image("group-law-bridge.svg", width: 100%)

Given a nonzero complete-splitting polynomial identity, the proof derives
factor-base membership and every signed ordinate lift. The union of all four
sign branches is complete. Soundness derives the canonical tangent slope.
Completeness compares a coefficient in the full intersection cubic, without
cancelling target-abscissa differences.

== Verification and custody
Lean 4.19.0 and Mathlib commit
#raw("c44e0c8ee63ca166450922a373c7409c5d26b00b").
The three-module build exited zero and twelve central axiom printouts list
only standard Lean foundations.

The worker became unavailable afterward. The published sources were recovered
from recorded file edits; no pre-outage whole-source checksum was captured.
A fresh published-byte replay and the whole-module repository audit are pending.
The direct theorem bundling the prior F17 degree fixture with this generic
bridge was not implemented. This report's PDF has not been rendered or
inspected; only this source and the diagram are supplied.

== Sources and proof map
See ../FORMALIZATION.md for every declaration and hypothesis, and
../verification/CUSTODY.md for exact commands and outstanding checks.
The output excerpt is ../verification/build03-output-recovered.txt.

Angdinata and Xu, An Elementary Formal Proof of the Group Law on Weierstrass
Elliptic Curves in Any Characteristic, ITP 2023:
#link("https://doi.org/10.4230/LIPIcs.ITP.2023.6").
Mathlib's pinned Affine.lean and Group.lean supply the verified point group.
Prior secant identities and canonical degree proof remain sealed in ../proof.

No new numerical measurement is reported; existing quantitative figures and
comparison dashboards remain unchanged.
