#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false)
= Isogeny-class rho: review correction
2026-10-19 · PR #2187 · historical run-20261018T0857Z

This is a source and claim audit. No new timed experiment or corrected
historical statistical result is claimed. The frozen protocol and raw rows
remain unchanged.

#image("figures/review-boundary-20261019.svg", width: 100%)

== What changed
The manuscript and README now enumerate the ancillary flags: one B1
generator, two P1 shift cells, one P0 shift cell, and B1 generator
Kruskal–Wallis p=0.039. Primary comparisons inside their reported bands do
not justify “no test fired.” The historical 1.7–2.2× wall-time ratio is an
unisolated shared-VM observation, not an admitted performance result.

The native KS counterexample binds companion source a277c5f: identical
samples [1,1] and [1,1] produce D=1 instead of 0. Rank-test ties and
shared-seed covariance also require review. Historical p-values and detector
resolution remain provisional until corrected native analysis is bound to
the actual producer source and frozen rows. No preregistered test was
silently replaced.

== Proof and model scope
The pinned Lean project checks scalar transport and answer reflection under
explicit homomorphism and injectivity hypotheses, and proves the mod-8
number-theoretic implication from explicit discriminant/divisibility
hypotheses using pinned Mathlib. PROOF-MAP.md states the checked scope and remaining
bridges. Concrete isogeny/group-law/native-code refinement, coupled oracle
costs, characteristic-two map certification and experiment admission are
separate obligations. The mod-8 restriction alone does not identify the
Koblitz degrees: the recorded factorization 703889=409·1721 and volcano
hypotheses are additionally required.

== Evidence impact
The new diagram is an audit workflow, not a quantitative result. Existing
historical figure values, parent-study report/PDF and benchmark mirrors are
preserved. A corrected native reanalysis must refresh all affected canonical
graphs and copies together. No new timing, speedup or leaderboard row is
introduced. Full findings, exact pins and validation receipts are in
REVIEW-20261019.md and PROOF-MAP.md beside the manuscript.
