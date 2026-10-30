#set page(paper:"a4",margin:(x:15mm,y:14mm),footer:context align(right,text(size:8pt,fill:rgb("64748b"))[N83 / quartic closure · #counter(page).display()]))
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.45em)
#text(size:21pt,weight:"bold")[Hybrid quartic closure]

#text(size:10pt,fill:rgb("116d8a"))[n83-degree4-quartic-20261029-r1 · retained m3 systems]

*12/12 producer/fresh pairs and 12/12 S3-restored replays passed.*
50122 new independent rows; 0 of degree at most three.
Full V4 closures: *0*. Global upper certificates: *0*.
Exact LFD certificates: *0*. Lower4 is inherited; review pending.

#text(size:8pt)[#table(columns:(2.5fr,.7fr,.8fr,1fr,.5fr),inset:4pt,stroke:.35pt+rgb("d3dae4"),
[Retained case], [New rows], [Attempts], [Pending], [LFD],
[original-koblitz-planted], [4301], [4302], [5916919], [>=4],
[original-koblitz-natural], [4302], [4303], [5916918], [>=4],
[original-binary-planted], [4302], [4303], [5916918], [>=4],
[original-binary-natural], [4301], [4302], [5916919], [>=4],
[baseline-koblitz-planted], [4304], [4305], [5916916], [>=4],
[baseline-koblitz-natural], [4302], [4303], [5916918], [>=4],
[baseline-binary-planted], [4302], [4303], [5916918], [>=4],
[baseline-binary-natural], [4301], [4302], [5916919], [>=4],
[spike40-koblitz-planted], [3927], [3928], [5919589], [>=4],
[spike40-koblitz-natural], [3927], [3928], [5919589], [>=4],
[spike40-binary-planted], [3922], [3923], [5919594], [>=4],
[spike40-binary-natural], [3931], [3932], [5919585], [>=4],
)]

#image("added-rows.svg",width:100%)
#text(size:8pt,fill:rgb("475569"))[Exact capped counts, not factor-base quality. K/B: Koblitz/general binary;
P/N: planted/natural. Repeated inputs are not independent draws.]

*Unchanged inputs.* n83, m3, ell27,164 Boolean variables. Actual point manifests
remain fixed across m2–9; targets and equations are unchanged. The 48-row
fixed-base regularity view is separate. No prime/higher-m LFD result is added.

*Stops.* basis payload cap: 12. Blocked candidates and pending work remain saved.

#pagebreak()
#text(size:18pt,weight:"bold")[Claim boundaries and reproducibility]

*The calculation.* Replay complete V2 and four immutable parent layers. Queue
products of added parent rows of degree at most3 and all complete-V3 cubics.
New degree-at-most-three consequences receive urgent variable-product tasks.
Each ordinary product has degree at most4 before Boolean field reduction.
Dense lower blocks and sorted quartic indices retain every row and reducer.
A blocked candidate remains separate and does not consume its queued source.

*Completion is separate from a global bound.* Exhausting the full product queue
would certify Boolean normal-form V4. A global upper4 still requires an admitted
unit or exhaustive affine ideal equality (at most 12 free variables). With the
inherited strict lower4, either certificate gives exact4. Caps supply no lower5
or universal FFD/LFD conclusion. Unknown upper/exact degrees remain null.

*Validation.* All 28 native tests pass: independent dense V4 equality, genuine
quartic cancellation, positive/negative controls, every cap, fill-in interruption,
queue coverage, global upper gates and altered/truncated records. Independent
scientific review and full algebraic formalization remain pending.

*Restoration.* Manifest counts verified before and after replay: raw: 278, prefix: 230, suffix: 270, witness: 370, propagation: 299.
All 12 cumulative summaries are retained.
Interrupted restored-verification attempts preserved: 1; successful
statuses were checked and skipped on resume. Production was not repeated.

*Bounds.* 16384 new rows; 2 GiB new basis payload; 131,072 attempts; 1,048,576 active
quartic nonzeros; 8 GiB virtual memory; one native process; 1800s per producer/replay
plus 5s grace. Existing-host L0 fallback; no performance or CRIU claim.
Diagnostic UTC batch boundaries: 2026-10-29T20:48:10Z to 2026-10-29T21:55:27Z.

*Reproduce.* Read DEGREE4_QUARTIC_FORMAT.json and the archive README.
Four immutable parent archives are required. The storage JSON and unified export
bind raw objects and every verification record.
#text(size:8pt)[Source: `2b9a04ffecce07bf46606e5751f28f8a80792b31` #linebreak()
Protocol: `514afddfda129310ee67ace84defbfabac5631506bff9c392b41b247a0b84700` #linebreak()
Data before report: `a5320f7f1cd20554938bd692edd52bb6c04e512d`]

*Sources.* #link("https://arxiv.org/html/1504.01175")[Semaev section4.5];
#link("https://arxiv.org/html/2112.05579")[Caminata–Gorla Definition1.5 and section2].
The full n83 three-family m2–9 FFD/LFD and held-out elite-base objective remains open.
