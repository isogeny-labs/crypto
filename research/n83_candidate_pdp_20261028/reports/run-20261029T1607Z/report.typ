#set page(paper: "a4", margin: (x: 16mm, y: 14mm), footer: context align(right, text(size: 8pt, fill: rgb("64748b"))[N83 / exact chain regularity · #counter(page).display()]))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: .5em)
#text(size: 21pt, weight: "bold")[Exact original-top binary chain degrees]

#text(fill: rgb("087e8b"))[84 saved systems · 84 fresh replays · 84 S3-restored replays]

Matching lower and upper certificates determine the original-top degrees for all
retained binary/Koblitz systems at m=3–9. Native checks verify the hypotheses of
the written overlap proof. *Full algebraic Lean formalization and independent
scientific review remain pending.* These are not measured solver degrees.

#table(columns: (.5fr,1.7fr,1.7fr,.8fr), inset: 5pt, stroke: .4pt + rgb("d3dae4"),
[m], [Earlier interval], [Exact original-top degree], [Cases],
[3], [111–138], [*111*], [12],
[4], [194–221], [*194*], [12],
[5], [277–331], [*277*], [12],
[6], [360–414], [*360*], [12],
[7], [443–524], [*443*], [12],
[8], [526–607], [*526*], [12],
[9], [609–717], [*609*], [12],
)

#image("regularity.svg",width:100%)
#text(size:8pt, fill:rgb("64748b"))[Run: n83-chain-exact-20261029-r1. Recorded degree certificates, twelve retained cases per m.]

Each base uses 2^27 candidate x-values per curve. Each row covers original,
baseline and spike40, both binary families and both
target kinds. Bases remain fixed across m=2–9; no points, targets or systems
changed. Natural repeats are provenance identities, not independent samples.

*Context.* The prior m=2 degrees remain Koblitz 28, binary 28 and prime 42. Prime
m=3–9 original top quotients remain positive-dimensional, with no finite first
vanishing degree. The48-row fixed-base view now has34 exact and14 prime entries.
All earlier intervals and views are preserved. This is improved knowledge of
unchanged systems; it is not an elite-base gain or an FFD/LFD conclusion.

#pagebreak()
#text(size: 18pt, weight: "bold")[Proof structure, controls and preserved evidence]

*1. Verify the original presentation.* Bind every input and actual point-base
manifest by hash. Reconstruct all original highest coefficients and complete
supports; check ring, field equations and exact witness suffix. No affine
consequence is substituted before taking highest forms.

*2. Account for overlapping components.* Full coefficient Frobenius diagonalizes
all field-product cubics. All 83 cyclic27-row selector windows have verified
inverse pairs. Counting window intersections gives a rank lower bound for every
embedding subset. Decomposition by intermediate-variable monomial support then
bounds the augmented product quotient by socle 83(m−2)+27.

*3. Preserve the terminal defect.* For nonzero target, the terminal quadratic
leaves a possible product direction h in degree 2. All terminal local variables
annihilate h. A truncated-chain quotient bounds its shifted module by
83(m−3)+56, which is below the augmented bound. The original-source projection
retains 83(m−2)+27 variables, giving equality and first vanishing degree one higher.
Zero target is checked separately through invertible coefficient Frobenius.

*Controls.* Five native tests pass, including 44 complete small mixed-degree
Hilbert calculations. All 36 cases inside the hypotheses match. All 8 negative
cases give degree 7 rather than formula 6 and are refused by the guard. Saved
augmented/truncated Hilbert arrays, kernel differences and exhaustive support
checks are independent of the large-case bound. Two Lean lemmas prove only the
arithmetic implications, using standard propext and Quot.sound. Full polynomial,
rank-decomposition and graded-module bridges remain written, not formalized.

*Preservation and replay.* All 360 raw manifest entries matched before and
after downloaded-data replay. All 84 immutable cumulative progress snapshots,
original inputs, executable, protocol, controls, certificates and errors remain.
Data were committed and pushed before this report. Read CHAIN_EXACT_FORMAT.json,
CHAIN_EXACT_DESIGN.md, the unified export and evidence/chain-exact-storage.json.
Producer replay does not substitute for independent scientific review.

#text(size:8pt)[Archive SHA256: `a0f135c6a765d04f9169f875cd90a73b56c97db42f4c99bb9595949a0925ced3`]#linebreak()
#text(size:8pt)[Source: `15b67074130e90f92852b261f46e253b74eab338`]#linebreak()
#text(size:8pt)[Protocol: `35d448ff9521388239b6aaca3cb6efaeca9a354b0d32d7d97a7ccf44fffd5c29`]#linebreak()
#text(size:8pt)[Data commit: `7ea0bcc977e315edadb1af6c5d2b384fdec5dc48`]#linebreak()

The archive is in setec-astronomy-artifacts under store/sha256, first two hash
characters and full hash. Extract fresh, check manifest, replay, recheck manifest.
Existing-host fallback followed a failed EKS storage probe with unconfirmed
cause; refreshed logs were empty. No timing, speed or attack claim.

*Remaining work.* FFD/LFD, prime-case completion, independent review and held-out
elite-base evidence. The tested linear selector bases have equal original-top
degrees. A basis change alone in this theorem's class cannot improve this metric;
other invariants or presentations require their own explicit protocols.
