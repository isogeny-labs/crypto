#set page(paper: "a4", margin: (x: 16mm, y: 14mm), footer: context align(right, text(size: 8pt, fill: rgb("64748b"))[N83 / chain bounds · #counter(page).display()]))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: .5em)
#text(size: 21pt, weight: "bold")[Stronger binary chain regularity bounds]

#text(fill: rgb("087e8b"))[84 saved inputs · 84 fresh replays · 84 S3-restored replays]

All binary/Koblitz systems at m=3–9 passed. Each new interval holds for
original, baseline and spike40 bases, planted and natural targets: 12 cases per m.
These are tighter certificates on unchanged systems, not a measured base gain.

#table(columns: (.5fr,1.4fr,1.4fr,1.5fr,.9fr), inset: 5pt, stroke: .4pt + rgb("d3dae4"),
[m], [Previous original], [Previous paired], [New interval], [Blocks],
[3], [82–165], [111–165], [*111–138*], [1],
[4], [109–275], [194–275], [*194–221*], [2],
[5], [136–385], [277–385], [*277–331*], [2],
[6], [163–495], [360–495], [*360–414*], [3],
[7], [190–605], [443–605], [*443–524*], [3],
[8], [217–715], [526–715], [*526–607*], [4],
[9], [244–825], [609–825], [*609–717*], [4],
)

#image("regularity-bounds.svg",width:100%)

*No exact full-chain degree is claimed.* All intervals retain a gap. The bases
still use 2^27 candidate x-coordinates per curve and remain fixed across m=2–9.
Original planted targets differ from paired planted targets; repeated natural
sources are provenance identities, not independent samples.

*What this establishes.* Nonadjacent chain components supply stronger upper
bounds, while a checked coordinate projection strengthens the older original
lower bounds. These identical overlapping intervals do not rank baseline and
spike40. The prior exact m=2 degrees remain Koblitz 28, binary 28 and prime 42.

#pagebreak()
#text(size: 18pt, weight: "bold")[Certificate construction and preservation]

*Source binding.* Check pinned source/input/base hashes; ring and equation IDs;
every highest coefficient and full expected support; every variable-square field
equation; and the exact saved witness suffix. Lower-degree coefficients remain
unchanged by their source hash. No affine consequences are added.

*Cubic components.* Checked invertible coordinate transforms over the field of
2^83 elements give 27 disjoint cubic slots. Every coefficient of these relations
is checked for both selector domains and both cubic shapes. Each slot permits
at most two of its three variables; the component socle upper bound drops by 27.

*Final quadratic component.* Explicit original-equation combinations express
all embedded products as scalar multiples of one product h for nonzero target.
Variable squares imply every local variable annihilates h. Adding the missing
product direction changes only degree 2. The full product quotient vanishes
above degree 83, so the original quadratic component does too. Zero target is
handled separately by invertible coefficient Frobenius. Small controls cover
both branches.

*Combine disjoint blocks.* Select even-numbered S3 groups and verify that their
variable blocks do not overlap. Tensor-product socle bounds add, giving
`upper = N - 27*floor(m/2) + 1`, with `N = 27m + 83(m-2)`.
Keeping only the first 27 selectors and all intermediate variables kills every
non-field top monomial and gives `lower = 28 + 83(m-2)`.

*Validation and scope.* Five native tests pass, including 30 explicit small
Macaulay/Hilbert cases and 128 independent arithmetic checks. Field irreducibility,
matrices/inverses and all component relations are checked. Lean proves only
slot-sum arithmetic, using standard propext and Quot.sound. The full ideal,
annihilator, tensor and scalar-extension bridges are written/native evidence.
Independent scientific review remains pending; these are producer replays.

*Preserved data.* All 360 raw manifest entries matched before and after
S3-restored replay; all 84 immutable partial summaries remain. Source, inputs,
full certificates, executable, logs and prior failures are retained. Data were
committed and pushed before this report. Read `CHAIN_FORMAT.json`,
`CHAIN_DESIGN.md`, `experiment-chain.export.json` and `evidence/chain-storage.json`.

#text(size:8pt)[Archive SHA256: `5da13e39bbc9db23f61a2fbd709da896bda04b1bb8eacbe78c218011758d579d`]#linebreak()
#text(size:8pt)[Source: `8ccd4eb9729cb01767064e41c5507dcb0476b2ac`]#linebreak()
#text(size:8pt)[Protocol: `ce0a47828dbf3f0dcfe311cefd43730ab671dcf9ac1da7e0cee8ecd2d9a1fb72`]#linebreak()
#text(size:8pt)[Data commit: `19c6cc710c2acfafef5a20ad7847f48d44524428`]#linebreak()

The content-addressed archive is in `s3://setec-astronomy-artifacts/store/sha256/`
under its first two hash characters and full hash. Restore, check
`manifest.sha256`, run `instrument replay ROOT`, and recheck the manifest.

*Remaining work.* Exact larger-m regularity/LFD, prime LFD and held-out elite-base
studies. Higher-m prime top quotients are positive-dimensional in the saved
presentation. Existing-host fallback followed retained EKS bucket denial.
No timing, solving-degree, universal FFD/LFD or ECDLP attack claim.
