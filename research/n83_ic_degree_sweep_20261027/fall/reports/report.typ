#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(leading: 0.55em)
#text(size: 23pt, weight: "bold")[N83 first-fall certificates]

#text(fill: rgb("#44566c"))[Balanced candidate domains · m = 2–9 · 27 October 2026]

#block(fill: rgb("#edf2f7"), inset: 10pt, radius: 4pt)[
*32 exact first-fall values; 16 lower bounds; 48 certificate replays passed.*
Every Koblitz and general-binary input has first fall 2. Every prime input has a certified lower bound of 5. The exact prime values remain unresolved.
]

#image("degree-grid.svg", width: 100%)

*Workload.* All 48 original synthetic inputs: three curve families, eight arities, one planted and one natural target. ell = floor(83/m): 41, 27, 20, 16, 13, 11, 10, 9. Candidate x-domain size is 2^ell; points still require lifting to the curve. No target resampling or outcome selection.

*Binary certificate.* The original equations contain no nonzero generator of degree below 2. A saved constant combination of the 83 quadratic equations gives a nonzero linear polynomial. Quadratic-part rank is 81 at m=2 and 82 at m=3–9; full rank is 83 throughout. Forward and reverse pivot orders agree. Each combination was reconstructed from the original equations in a fresh process. Thus the earliest new degree fall is 2, also a nontrivial leading-form syzygy at 2.

*Prime certificate.* Selector equations form a monic degree-two Groebner basis with no degree falls. After quotienting their top squares, a diagonal coefficient minor of the m−1 quartic S3 rows has nonzero determinant: 16 at m=2 and 64 otherwise. This excludes new falls through degree 4. It does not establish a fall at 5.

*Limits.* These values depend on the specific S3-chain encoding. Last fall and regularity remain unmeasured. The earlier regularity run's 96 resource-limited computations are preserved separately. One target per kind does not support a statistical curve-family claim. Fresh-process replay uses the same implementation and is not an independent research review. No performance or ECDLP recovery claim.

*Validation and provenance.* Seven native tests passed; positive, duplicate, homogeneous and zero-row controls passed. All 48 certificate replays and the raw file hashes passed. The unified export validates with file verification. Source, binary, input list and individual systems are pinned in `protocol.json`. EKS job `n83-first-fall-m2-m9-20261027-r3` exited 0; two CPUs, 8 GiB, 900-second cap, no retries. Wall times are diagnostic.

*Reproduce and inspect.* `evidence/results.json` holds every polynomial witness or coefficient minor; `evidence/replay.jsonl` records replays; `experiment.export.json` binds the protocol and raw outputs. The graph is generated from those records by `../src/bin/fall_report.rs`. Full definitions and execution rules are in `DESIGN.md`.

*Primary references.* #link("https://arxiv.org/html/1503.08001")[Kosters and Yeo: trace identities]; #link("https://arxiv.org/html/2112.05579")[Caminata and Gorla: first and last fall]; #link("https://arxiv.org/html/1906.05594")[Kousidis and Wiemers: Boolean filtered algebra].
