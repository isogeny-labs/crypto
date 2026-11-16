#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.4em)
= Quotient calibration: validator review correction
*isogeny-labs/crypto · PR 2641 · 2026-10-30*

The projective validator now rejects incomplete coordinates before target
arithmetic. *21 focused tests pass. No new target frame, challenge image or
inverse divisor has been computed.* The F343/order-373/kernel-153 inputs,
certified anchor and acceptance criteria are unchanged.

#figure(image("review.svg", width: 75%), caption: [Green: fixed development checks. Amber: unresolved execution prerequisites. Dashed: unexecuted target experiment.])

== Correctness and fixed validation
The former zip comparison could accept a one-coordinate prefix. The repair
rejects unequal lengths and checks every prediction row before constructing
any target divisor. All 32 case IDs, source points and P+G inputs must match.
Each primary, successor and control vector has exactly eight field coordinates
and is nonzero; field coefficients lie in 0..6.

#table(columns: (1.2fr, 3fr), inset: 4pt, stroke: 0.4pt + luma(190),
  [*Check*], [*Observed result*],
  [Native focused tests], [8 target components + 9 certificate tests + 4 original source tests pass. Strict all-target Clippy passes.],
  [Historical targets], [3 of 3 existing F49 vectors match both existing paths. No new target is evaluated.],
  [Public identity proof], [IDC1h368ff5f1ba01b556 and the exact integer expansion replay byte-for-byte. No credential/TLS content.],
  [Root library checks], [Earlier run: 4,845 pass; 17 fail; 121 ignored (absent fixtures). Current locked check stops at an inherited manifest/lock mismatch. No full-suite pass.],
)

== Execution remains pending
The user approved ongoing same-branch checkpoint uploads on 30 October.
The environment still has no configured upload transport; the normal gate
must pass before an experiment. The rejected pasted-key process was not retried.
The draft plan now follows the repository's EKS Spot default. A matching
target-binary image and custom-command route need qualification; the standard
GitHub workflow builds ecbench and is not a substitute.

After access and image qualification: prospective preflight, freeze/commit,
nine-point calibration, immutable matrix checkpoint, then all 32 primary and
successor comparisons. No refit, point replacement or omitted failure.
Inversion, signs, subgroup/Frobenius checks, eight addition/scalar relations,
Profile B and a global section identity remain open. No speedup or
vulnerability is established.

#text(size: 7.5pt)[Sources/evidence: REPORT.md, raw focused logs, validation.json and merge-preservation.json beside this PDF; Rust Iterator::zip primary documentation; Tian arXiv:2012.07173v1 §3.2; Khuri-Makdisi math/0105182 Lemmas 2.1–2.3. Main integration 1733dc59e preserves quotient records and main problem rows. Historical reports and canonical cost graphs are unchanged.]
