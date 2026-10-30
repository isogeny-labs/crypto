#set page(paper: "a4", margin: (x: 16mm, y: 14mm), footer: context align(right, text(size: 8pt, fill: rgb("64748b"))[N83 / original-top certificates · #counter(page).display()]))
#set text(font: "DejaVu Sans", size: 10.3pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
#text(size: 21pt, weight: "bold")[Binary m=2 regularity is exactly 28]

#text(fill: rgb("087e8b"))[12 retained inputs · 12 fresh replays · 12 S3-restored replays]

The saved Koblitz and binary systems all have original-top vanishing degree 28.
This closes their former interval 28–55. Each domain contains 2^27 candidate
x-coordinates and remains fixed across m=2..9; this certificate batch covers m=2.

#table(columns: (2.7fr, .7fr, 1fr, 1fr, 1fr), inset: 5pt, stroke: .4pt + rgb("d3dae4"),
[Dataset / family], [Targets], [Old bound], [Exact], [Q rank],
[original / koblitz], [2], [28–55], [28], [53],
[original / binary], [2], [28–55], [28], [53],
[baseline / koblitz], [2], [28–55], [28], [53],
[baseline / binary], [2], [28–55], [28], [53],
[spike40 / koblitz], [2], [28–55], [28], [67],
[spike40 / binary], [2], [28–55], [28], [67],
)

#image("regularity.svg", width: 100%)

*Meaning of the result.* Spike40 raises quadratic rank 53 to 67 but has the same
regularity on these saved targets. Previous paired last-fall degrees remain 2
for baseline and 3 for spike40. Earlier prime m=2 certificates give regularity 42.
These are distinct invariants; no elite factor-base gain is established here.

*Scope.* Original and paired planted targets differ. Repeated natural sources
are retained provenance identities, not independent samples. Full saved sources
are unchanged. The right plot shows bounds on Hilbert coefficients, not their
exact values. Larger-m results are not inferred from this m=2 proof.

#pagebreak()
#text(size: 18pt, weight: "bold")[Proof, replay and preserved data]

*1. Bind the calculation to the inputs.* Reconstruct every byte of each saved
Singular source, including ring, all S3 descent and field equations, and witness
suffix. Check source, metadata and factor-base hashes. Require every nonzero
non-field highest form to be cross-bilinear.

*2. Check the upper bound.* Over the verified field of 2^83 elements, express all
product-coordinate polynomials as XOR combinations of original quadratics.
Verify both products of the 27-by-27 coordinate matrix and inverse. Check all 729
coefficients of each of 27 pair-product relations. The changed ideal contains
all squares and 27 disjoint pair products. A surviving monomial chooses at most
one variable from each pair, so degree 28 vanishes.

*3. Check the lower bound.* Set the second selector block to zero. The quotient
on 27 remaining square-zero variables retains their degree 27 product. Scalar
extension preserves each graded dimension. The matching bounds certify28.

*Controls.* Five native tests pass. Retained controls include 128 independent
field multiplications, 512 direct Boolean S3 evaluations, 8 explicit small
Macaulay/Hilbert cases, all 83 Frobenius squarings, and corrupted/singular inverse
rejection. Negative span tests fail as expected; such failure is not a claim
that regularity differs. Lean proves the finite slot argument only, with standard
propext and Quot.sound. Full algebraic formalization and independent review
remain pending.

*Preservation.* Raw archive: 60 manifest entries plus the manifest. All entries
matched after downloading from S3 and again after replaying the downloaded
instrument. Source, protocol, inputs, complete certificates, executable, controls,
logs and host metadata are retained. Data were committed and pushed before this
report. Earlier data and failed attempts remain intact.

#text(size: 8pt)[Archive SHA256: `38c0fd69215a2cf966a2bd14049f4f5b465aa8b4302f363c77c23079aca861dc`]#linebreak()
#text(size: 8pt)[Source commit: `ac08cf2f9029145b48e799c1de5ede0564732b3d`]#linebreak()
#text(size: 8pt)[Protocol SHA256: `95cf1fbd579f98e7b766b3fceb105b833d90c372566e180defe027e13cf95ad6`]#linebreak()
#text(size: 8pt)[Data commit: `4f52357908db49c3fb9171c969603464de7362c9`]#linebreak()

*Find and replay.* Read `BINARY_PAIR_FORMAT.json`, `BINARY_PAIR_DESIGN.md`,
`experiment-binary-pair.export.json` and `evidence/binary-pair-storage.json`.
The archive lives under `s3://setec-astronomy-artifacts/store/sha256/38/` followed
by its full SHA256 above. Extract, check `manifest.sha256`, then run
`instrument replay ROOT` on the extracted directory.

*Literature.* The design records primary summation-polynomial/Weil-descent
literature (arXiv 1503.08001), Rabin irreducibility references, and Stacks 05G3
on algebra base change. The pair quotient bound is the explicit elementary
argument above; no novelty or characteristic-zero transfer is claimed.

*Remaining work.* Larger-m binary regularity/LFD, prime LFD and held-out base
comparisons. Prime m=3–9 top quotients are positive-dimensional in the saved
presentation. Existing-host execution followed retained EKS bucket denial;
17:15:52–17:16:05 UTC is diagnostic wall time only. No speed or attack claim.
