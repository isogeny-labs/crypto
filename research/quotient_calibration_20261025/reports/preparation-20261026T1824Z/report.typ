#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.4em)
= Quotient reconstruction: native target preparation
*isogeny-labs/crypto · 2026-10-26 · source caec72812*

The next target-side computation is implemented. *The new target frame and
32-point challenge have not run.* Their inputs and acceptance gates are unchanged.

#figure(image("flow.svg", width: 86%), caption: [Green: completed fixed-component checks. Amber: implemented but unverified on the certified anchor. Dashed: unexecuted dependent gates.])

== Verified fixed components
#table(columns: (1.2fr, 3fr), inset: 4pt, stroke: 0.4pt + luma(190),
  [*Check*], [*Observed result*],
  [Source package], [4 tests pass, including full replay of the unchanged frozen source manifest.],
  [Target package], [5 component tests and 9 canonical certificate tests pass. Strict Clippy passes.],
  [Historical targets], [3 of 3 F49 Kummer vectors reproduced; direct six-permutation and residue-algebra evaluation agree.],
  [Trace identity], [IDC1h368ff5f1ba01b556: 32 derived and 16 extra field points accepted; exact integer expansion agrees on six monomials.],
)

== Implementation and scope
Khuri-Makdisi section spaces compute divisors over the explicit F7¹⁸ field.
An L-linear trace in a length-three residue algebra evaluates Tian's symmetric
tensors without assuming their support splits in L. Full field coefficients
are preserved. Calibration, source-only evaluation and the validation oracle
have separate interfaces and fresh-only outputs.

The divisor engine's certified-anchor execution is unverified. No target
matrix, inverse fiber, recovered divisor sign, global section identity,
Profile B initialization or speedup is claimed.

== Launch prerequisite
The official S3 checkpoint gate fails without configured upload access.
Automatic approval review rejected reuse of the pasted key for the new
checkpoint payload and destination; the action was not executed or retried.
A review-only local snapshot was prepared and checkpoint approval/access
requested. *No new target experiment launched.*

After access: freeze the native execution record, checkpoint it, evaluate the
nine frame images, freeze/checkpoint M and require every one of the unchanged
32 primary and 32 successor comparisons before inversion. Preserve failures;
no refit or replacement.

#text(size: 7.5pt)[Sources: Tian, arXiv:2012.07173v1 section 3.2; Khuri-Makdisi, math/0105182 Lemmas 2.1–2.3; pinned historical reports in PROVENANCE.json. Detailed evidence, test output and limits: REPORT.md beside this PDF. No formal-prover or independent-runner review; no canonical cost-graph changes.]
