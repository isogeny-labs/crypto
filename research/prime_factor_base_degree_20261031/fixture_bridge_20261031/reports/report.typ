#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
#show heading: set text(fill: rgb("#20495b"))
= Concrete F17 relations and canonical degree
2026-10-31 · checked formalization continuation

The theorem #raw("FixtureGroupDegree.exact_relations_and_degree") proves
the actual four-generator system equivalent to signed factor-base relations
in Mathlib's elliptic-curve group. The same generator set has canonical
homogeneous first-vanishing degree 3, while the inherited dense comparator
has degree 5. Each valid finite target and all four signs are included.

#image("bridge.svg", width: 100%)

Curve #raw("icv1-fp5-tm1-a77cc6af"), base #raw("FB1hed5b19904cc8").
Over F17, the curve is y²=x³+2x+2, the graph is h(x)=4x²+9x+9 and
V={5,6,10,16}. These existing identities and the FB1 object are reused;
the fixture's existing unregistered status is unchanged.

The split identity is proved as a polynomial equality, rather than inferred
from finite-field function values. Every generator evaluation is proved
equal to its named secant residual. Tangents, repeated abscissae and flexes
remain covered. The identity target is separately exact: inverse pairs.

#pagebreak()
= Proof scope and verification

The headline theorem joins three checked statements:
+ For every x,z, some slope annihilates all original generators exactly when
  the signed base points have actual sum R.
+ The canonical highest-form ideal of those same generators first vanishes
  in homogeneous degree 3.
+ The inherited dense comparator first vanishes in homogeneous degree 5.

The actual total degree of each polynomial selects its highest form.
Nonzero discriminant and the concrete complete-splitting identity are
proved in Lean. Every target on the concrete curve is nonsingular. All
ordinate lifts are represented by the four sign branches.

== Checks
159 declarations in fourteen modules pass the unchanged transitive-axiom
auditor: zero failures and zero native trust. The new module passes
warnings-as-errors. The prior published three-module group bridge also
rebuilds and passes its 22-declaration audit and warnings-as-errors, closing
the earlier outage gap. The original recovered source matches the published
copy byte-for-byte.

Lean 4.19.0; Mathlib revision
#raw("c44e0c8ee63ca166450922a373c7409c5d26b00b").
Nine Git dependencies match the lockfile; thirty inputs are hashed.
The two sealed prior proof projects are unchanged relative dependencies.
All axioms are among propext, Classical.choice and Quot.sound.
lean4checker was unavailable. Seven failed development builds and all
successful receipts remain in the evidence archive.

== Interpretation limits
The degree is the first vanishing homogeneous quotient degree. No affine
F4 degree, natural yield or runtime is measured here. This continuation
does not add an independent dense S3-to-group equivalence theorem or claim
external novelty. Existing quantitative figures and leaderboard rows stay
unchanged. The figure documents formal coverage, not measured speed.

== Sources and replay
The exact declaration/hypothesis map is ../FORMALIZATION.md. Run
#raw("bash verify.sh /tmp/secant-fixture-receipts") from the project root.
See ../verification/final/axioms.log, ../verification/metadata.json and the
source hashes for the complete records.

The concrete fixture is ../../secant/fixture.json. The copied identity
certificate is IDC1h621ed1a51b5f3ca8; its evaluated-tier status is retained.
The actual group reuses Mathlib and Angdinata–Xu, ITP 2023:
#link("https://doi.org/10.4230/LIPIcs.ITP.2023.6").
