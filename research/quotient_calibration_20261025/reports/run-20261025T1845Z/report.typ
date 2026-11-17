#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set par(leading: 0.5em)
#set heading(numbering: none)
= Quotient calibration: native source replay
*aburan28/crypto · run-20261025T1845Z · October 25, 2026*

The protocol and source preflight have been relocated from ghs-trapdoor PR 10.
The original input hashes, kernel, training frame and 32 challenges are preserved.
*Target calibration and inversion remain unexecuted.*

#figure(image("flow.svg", width: 100%), caption: [Solid edges: source checks executed. Dashed edges: target gates pending. Counts describe frozen input sets, not target successes.])

== Recorded checks
#table(columns: (1.1fr, 2.9fr), inset: 5pt, stroke: 0.4pt + luma(190),
  [*Check*], [*Outcome*],
  [Source], [F343 = F7[z]/(z³+z²+1), y² = x³+248x+42; G=(0,176), r=373, kernel 153.],
  [Pilot], [32 points × 64 sections; rank 8; selected indices 0–7.],
  [Frame], [G,2G,…,8G,11G; every eight of nine vectors have rank eight. All vectors and frame weights match.],
  [Challenge freeze], [32 inputs and P+G partners reproduced exactly; 99 sampling counters; no target images or challenge scalar labels.],
  [Normalization], [16 level-2 and 81 level-3 normal Weil-cocycle checks. Modulus irreducibility and source generator order checked.],
  [Controls], [Four Rust tests pass, including altered-manifest and singular-frame rejection. Strict Clippy passes.],
  [Provenance], [Four complete upstream input reports hash-verified; kept outside the evaluator.],
)

Full normalized section-matrix SHA-256 matches the historical source:
#text(size: 8pt)[`52f1938e51d9baca3e9d1753a90516611468ed1472c08203decbdd0a41ab87fe`]

== Execution and evidence
The command-line replay exited 0. Source freeze:
#text(size: 8pt)[`a0cd24e1c7f4381e9ce59cfce760eb5eb8cea3c5`]

Rust 1.99.1, Cargo 1.99.1; Linux 6.18.55 x86_64; five available logical CPUs.
Ordinary shared worker, no timing/isolation admission. No speed or memory-cost
comparison is reported. Exact command, failed setup/lint attempts, provenance,
root validation status and full interpretation are retained in REPORT.md and
VALIDATION.md. The native-preflight.json receipt carries the measured counts.

The complete raw validation-log archive remains local and unpublished;
automatic upload approval and AWS access are pending. Its hash and contents
are recorded in ARCHIVE_PUBLICATION.json.

== What this establishes
This is a separate native implementation check against historical Python/FLINT
source outputs on one worker. The certified seed is fixed; its search is not
repeated. No Python research process was executed. This finite source sample
does not certify a global map identity, a target transfer or an attack gain.

The next step is to evaluate the nine target frame images, freeze M, then test
all 32 target images exactly before inversion. Nine fitted projective frames
alone supply no validation. Profile A retains the inherited anchor; Profile B
still needs normalized target Weil functions and a global section certificate.
The F7¹⁸ coefficients also require compatible fields; the old F7¹² field cannot
be reused. The complete acceptance gates remain in PROTOCOL.md.

#text(size: 8pt)[Source: github.com/aburan28/ghs-trapdoor, commit b314cf87940cfb7199cd98261ba6d3910f275c90. Evidence: research/quotient_calibration_20261025/reports/run-20261025T1845Z/. No canonical cost graph or benchmark index changes.]
