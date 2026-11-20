#set page(paper:"a4",margin:17mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.6em)
#text(size:22pt,weight:"bold")[Kernel admission: full archived cohort]

20 existing curves · Genus 2 / 3 / 4 · 15 November 2026

*Two existing genus-3 curves pass the tractable-kernel prerequisite.*
They can advance to a rational-trigonal-map gate. No curve was replaced or
newly selected. The four ineligible genus-3 curves and all other exclusions
remain in this complete finite census.

#image("HELDOUT.svg",width:100%)

#table(columns:(1fr,1fr,1fr,1fr,1fr),align:right,
 [Genus],[Curves],[Full eligible],[Pair eligible],[Total full / pair kernels],
 [2],[8],[3],[3],[3 / 3],
 [3],[6],[2],[2],[4 / 2],
 [4],[6],[2],[1],[5 / 1])

*Positive genus-3 inputs.* p127, family1: branch cycles (1,1,2,4), three
stable full kernels, one pair-generated kernel. p1009, family1: cycles
(1,1,6), one stable full kernel and one pair-generated kernel. A stable
kernel need not have pointwise-rational torsion. Neither input has yet been
shown to admit a rational trigonal map or a suitable same-field quotient.

*Other genera.* Three of eight genus-2 curves have a stable kernel; this
alone does not improve destination geometry or complete solve cost. Two of
six genus-4 curves have full kernels, but just one has a pair kernel.
p31,family1 has two full kernels and no pair kernel. A genus-4 polarized
quotient need not be a Jacobian. The full genus2/3/4 objective is unchanged.

*Method.* Native Rust enumerates all geometric isotropic subspaces and
Frobenius-stable ones in the even-subset model of 2-torsion. Polynomial gcds
recover finite factor degrees; infinity is a separate branch cycle.
Geometric totals for genera 1–4 and all 22 genus-3 cycle types pass the
registered controls. Fixed-torsion dimensions match the factor-count formula.
Twenty fixtures produce 46 CASE outputs including controls, no error or cap.
Native jq retains all per-curve counts and abstract branch-label bases.

*Frozen custody.* Amendment 91471e18d3 preceded source 9ccc6658d373 and
checkpoint 20261115T124300Z-9ccc6658d373. All inputs were restored from the
original held-out archive, SHA256
#text(size:8pt)[7dede0f0ff49824e6dbe1fce10be0d65a82042e0bbf8e4dc2b5651c483a6603d]
(781017 bytes), matching the current manifest. No old selector, timing,
raw receipt, closed record or default changed. Copied fixture hashes,
HELDOUT-CHECKS.txt and HELDOUT-SUMMARY.json permit exact replay.

*Interpretation.* This is an admission census of a reused cohort, not a
random-population estimate, novel algorithm, explicit isogeny or speed result.
Smith’s known genus-3 route still requires a rational trigonal construction,
non-hyperelliptic destination, base-field twist check, divisor map and
same-target complete cost. The six-fixture proof/report remains unchanged.
Primary source: Smith, arXiv:0806.2995v2, Sections3/9 and Lemma1.
