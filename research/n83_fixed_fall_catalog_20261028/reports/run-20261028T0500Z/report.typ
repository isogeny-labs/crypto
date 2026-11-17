#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
#let rows=json("../../evidence/summary.json").rows
#let protocol=json("../../protocol.json")
#let terminal=json("../../terminal.json")
#let exact=rows.filter(r => r.last_fall_degree != none)
#let binary=rows.filter(r => r.family != "prime")
= Fixed-base n83 FFD / LFD certificates
#text(fill:rgb("287d9c"))[ell = 27 · m = 2–9 · 28 October 2026]

Observations only. Exact input-specific certificates from the producer and its fresh-process replay; independent scientific review remains pending. No elite factor-base or regularity-improvement verdict is made.

The batch retains all #rows.len() systems from the fixed-base degree sweep. Every curve uses the same saved factor base for m=2–9. It adds first-fall witnesses/bounds, a structural source catalog, and degree-two closure for the #binary.len() binary/Koblitz systems.

#image("closure-grid.svg",width:100%)

== Measured values and scope
Binary and Koblitz: each first-fall certificate records degree #rows.find(r=>r.family=="binary").first_fall.first_fall_degree. Prime: every certificate proves no first fall through degree four, hence a lower bound of five; the exact FFD and LFD remain unresolved.

The number of exact LFD2 certificates is #exact.len(). For planted m=2, 53 independent linear constraints leave one free Boolean variable, and both assignments satisfy all original equations. For natural m=2, closure contains one. Larger m cases have degree-two closures but no accepted global upper certificate. A stable closure alone is not an LFD measurement.

These numbers concern the stated polynomial presentations. The previous regularity sweep remains separate: it obtained no completed finite top-form regularity value. FFD, LFD, final basis degree and degree of regularity are different quantities.

== Dataset and file structure
FORMAT.json explains every file type and binary field. catalog/index.json names the 48 inputs, source hashes, source sizes and variable blocks. Per-equation JSONL records give source byte offsets and Boolean term/degree/support data; prime expressions remain symbolic. Original sources and point data remain in the earlier content-addressed archives.

Each closure stores all row operations and zero reductions, immutable bit-packed rows, and its verdict. Every checked affine assignment is preserved, including counterexamples. First-fall files retain explicit polynomial witnesses or coefficient minors. Status and replay files retain all exit codes and diagnostics.

#pagebreak()
== Execution, controls and provenance
The native closure/catalog instrument is pinned to commit #text(size:8pt)[#protocol.arms.at(0).instrument.commit]. The unchanged first-fall executable is separately pinned. The protocol was committed before affected data; hash:
#text(size:8pt)[#protocol.freeze.protocol_sha256]

Started #terminal.started_at; finished #terminal.finished_at. Existing-host fallback after the recorded EKS shared-bucket authorization failure. Sequential native processes; 8 GiB virtual memory, 60 seconds per closure/replay, 600 seconds for each catalog/first-fall batch, 5400-second outer budget. No retries. Times are diagnostic only; no timing or speed comparison.

Four native controls pass: quadratic unit contradiction; affine-line exhaustive certificate; a nonlinear plateau that must not certify LFD; and tampering rejected by replay. The inherited first-fall selfcheck also passes. Root library validation from the preceding unchanged-library stage: 4966 passed, zero failures, 122 ignored.

#table(columns:(1fr,auto,auto),inset:5pt,
 [Computation],[Planned],[Completed],
 [First-fall certificate],[48],[48],
 [First-fall fresh replay],[48],[48],
 [Source structural catalog],[48],[48],
 [Boolean V2 closure],[32],[32],
 [Closure fresh replay],[32],[32],
)
No timeout or failed replay was recorded in this batch. Machine-readable export and raw status records accompany this report. Replay checks the saved derivations; it is not independent research review.

== Interpretation and next evidence
The restricted LFD upper certificate is documented in DESIGN.md. All ordinary Boolean field equations are included; products are admitted by ordinary polynomial degree, rather than arbitrary Boolean reductions. Either the unit ideal or an exhaustively checked affine solution space supplies the global bound.

The broader goal remains open: select matched-size candidate bases, freeze held-out target comparisons, and directly measure PDP regularity. No base was changed in this batch. Sparsity or a low first fall cannot substitute for a regularity result; finite input certificates cannot settle an unspecified universal conjecture.

== Access and retention
Git contains the source, protocol, summary, catalog index, export, report and content-addressed archive pointer. S3 holds all raw certificates, operations, binaries and catalog rows. The earlier incomplete compression stream is retained separately with complete=false.

#text("s3://setec-astronomy-artifacts/runs/n83-fixed-fall-catalog-20261028-r1/local/",size:8pt)

Primary references: Caminata–Gorla, arxiv.org/html/2112.05579; Kosters–Yeo, arxiv.org/html/1503.08001. Original fixed-base regularity report: ../n83_fixed_degree_sweep_20261028/reports/run-20261028T0325Z/.
