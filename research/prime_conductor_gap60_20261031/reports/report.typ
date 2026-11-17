#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false)
= Historical 60-bit prime conductor-gap archive

This import preserves a verified source-and-kernel component, a failed explicit
constructor, and checked partial polynomial sums. It adds no new experiment or
native execution path. The frozen producer revision is
#text(size: 8pt)[b6666f5058260f602070c06d1f56d03979295dec].

#image("status.svg", width: 100%)

== What the historical records support

The archived prime degree is 576460755499654231 (60 bits). Exact field,
source-model coefficients, subgroup and kernel coordinates are in
`../parameters.json`; the unchanged summary is `../historical-summary.json`.
The historical Sage replay passed its source, field, kernel-order and
conductor-argument checks. These are executable checks and a CM/volcano
argument, not a complete formal Lean proof.

The standard constructor failed with MemoryError. The alternative completed
bounded pieces of the polynomial sums and matched complete small examples;
its partial pieces are not a full map evaluation. The explicit quotient,
complete non-kernel point images and matched endpoint ECDLP trials remain
open. No quantitative comparison is plotted because the failed constructor
and the partial tiles measure different work.

== Provenance and retrieval

`../source-and-evidence.tar.xz.sha256` pins the immutable archive. Retrieve it
with the repository artifact tool if migrated, verify with `sha256sum -c`,
then extract it. The extracted `historical/evidence/input_manifest.json`
checks original paths and bytes. Original Python/Sage source, native C
helper, clocks, failures and receipts are historical evidence; this import
does not install a Python research harness in crypto.

The source archive SHA-256 is:
#text(size: 8pt)[e23bac98d4d3c663597fccaed07d19402f93ebeb0809f916b97b58309f6cbabd].

== Related work and remaining obligation

The existing problem record is
`problems/conductor-gap-upper-floor-and-cost.md`. The archive's
`historical/research/route_assessment.json` distinguishes initial map-image
requirements, missing endpoint inputs and the kernel's trivial Frobenius
extension orbit. It links the primary sources:

- #link("https://eprint.iacr.org/2022/1068")[Robert: Evaluating isogenies in polylogarithmic time].
- #link("https://antsmath.org/ANTSXVI/papers/Galbraith.pdf")[Galbraith: Climbing and descending tall volcanos].
- #link("https://arxiv.org/abs/2306.16072")[Banegas et al.: Fast and Frobenius].

The next technical obligation is an explicit quotient and non-kernel images
for the unchanged minimum 60-bit target, with a feasible native execution
path and fresh receipts. Archive publication does not discharge that
obligation. No largest-known status, backend ranking or leaderboard row is
promoted. Existing canonical graphs remain unchanged.
