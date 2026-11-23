#set page(paper:"a4",margin:(x:15mm,y:14mm),footer:context align(right,text(size:8pt,fill:rgb("64748b"))[N83 / m3 continuation · #counter(page).display()]))
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.5em)
#text(size:21pt,weight:"bold")[Retained n83 m3 closure continuation]

#text(fill:rgb("087e8b"))[12/12 verified · 12 complete V3 spaces · 0 exact LFD values]

Run `n83-degree3-continuation-20261029-r1`. Producer observations;
independent scientific review pending. Full V3 alone is not a global LFD upper
certificate. All original parent data are preserved unchanged.

#table(columns:(2.9fr,.8fr,.8fr,.8fr,.8fr),inset:4pt,stroke:.4pt+rgb("d3dae4"),
[Retained case], [New rows], [Total], [V3 full], [LFD upper],
[original-koblitz-planted], [3391], [36159], [yes], [unknown],
[original-koblitz-natural], [3391], [36159], [yes], [unknown],
[original-binary-planted], [3391], [36159], [yes], [unknown],
[original-binary-natural], [3391], [36159], [yes], [unknown],
[baseline-koblitz-planted], [3391], [36159], [yes], [unknown],
[baseline-koblitz-natural], [3391], [36159], [yes], [unknown],
[baseline-binary-planted], [3391], [36159], [yes], [unknown],
[baseline-binary-natural], [3391], [36159], [yes], [unknown],
[spike40-koblitz-planted], [3405], [36173], [yes], [unknown],
[spike40-koblitz-natural], [3405], [36173], [yes], [unknown],
[spike40-binary-planted], [3405], [36173], [yes], [unknown],
[spike40-binary-natural], [3405], [36173], [yes], [unknown],
)

#image("continuation-ranks.svg",width:100%)

*What changed.* 40748 new independent rows and
61224 new queue operations, summed from verified
native records. Each parent has 32768 rows. 12
reported strict-fall witnesses are inherited; 0
first occur in the suffix. A complete V3 space can support later nonmembership
checks, but does not by itself establish LFD stabilization. No rank is called a
regularity value or base-quality improvement.

#pagebreak()
#text(size:18pt,weight:"bold")[Continuation, verification and preservation]

*Twelve retained identities.* Original, baseline and spike40; Koblitz/general
binary; planted/natural. Each m3 system has 164 Boolean variables and the fixed
2^27-candidate base reused across m2..9. No point, target, equation or random
sample changed. Original and paired planted targets differ; repeated natural
inputs are not independent draws. Candidate x-coordinate decompositions still
require curve-lift/sign verification.

*Exact continuation.* Replay complete V2 and the full parent prefix. Check
its source, verdict, four file hashes, row/operation counts and pending queue.
Only new rows and operations are written. IDs remain global. Fresh replay
recomputes the exact reduction sequence and checks every suffix row byte and
the first stopping boundary. The suffix header has 44 bytes; each production
row has 11489 little-endian words, or 91912 bytes. The format map defines all
735295 columns, offsets, references and null conventions.

*Limits and controls.* Stop at queue exhaustion, 65536 cumulative rows or
262144 cumulative operations. One process, 8 GiB, 1200 seconds per production
or replay plus 5 seconds grace, 30000 seconds batch; no retries. Ten native
tests pass, including byte-identical concatenation with uninterrupted toy runs,
both cap types and parent/offset/operation/trailing-byte/completion tampering.
Runtime controls pass. A quadratic outside complete V2 gives lower3; a global
unit or affine-completion argument is separately required for upper3.

*Preservation.* 12 S3-restored replays passed.
All 270 suffix-run manifest entries and
230 parent entries match before
and after replay. New raw bytes: 3906117684. Every row, operation,
partial/cap state, original source, control and immutable progress snapshot
remains. Restore parent and suffix into separate directories and check both
manifests around replay. No earlier run is replaced.

*Execution and claim limits.* EKS storage probe exit1; log retrieval failed,
so its application cause is unconfirmed. Existing-host fallback, no IAM or
capacity change. No timing comparison; CPU and qualified wall times unmeasured.
Batch `2026-10-29T06:51:31Z` to `2026-10-29T07:11:55Z`. This is application-level
trace continuation, not CRIU recovery. Independent scientific review remains
pending. Earlier regularity results are unchanged; exact higher-m LFD/regularity,
prime LFD and held-out elite-base selection remain open.

#text(size:8pt)[Source: `bc39e12318d778a08bcbc8edc093db498cd75e74`]#linebreak()
#text(size:8pt)[Protocol: `a0b03b7a6355e5904f5ed8b242940fa0b5c41ad2ab79b4dc1872d42c615447b4`]#linebreak()
#text(size:8pt)[Suffix archive: `b6101a8e8a33b96ba6f6522802343eec2104b74e07b868279d4050765f059a4c`]#linebreak()
#text(size:8pt)[Data commit: `51b92f9146f0856aafcbb99136eafad67856a9f2`]#linebreak()

Read `DEGREE3_CONTINUATION_FORMAT.json`, `DEGREE3_CONTINUATION_DESIGN.md`,
`experiment-degree3-continuation.export.json` and the storage/publication records.
Definitions and plateau limitation: Caminata–Gorla, arXiv:2112.05579,
Definitions 1.3–1.5 and Example 2.4.
#pagebreak()
#text(size:18pt,weight:"bold")[Original fixed-base regularity sweep]

A consolidated view of existing source-bound certificates. These are the
original 2^27-candidate bases reused across m=2–9. Each row covers both the
saved planted and natural target. All 48 system hashes and base identities
were checked when joining the source summaries. No new algebra was performed
for this table; the new continuation is described on the preceding pages.

#table(columns:(.35fr,1.1fr,1.1fr,1.6fr),inset:6pt,stroke:.4pt+rgb("d3dae4"),
[m], [Koblitz], [General binary], [Prime],
[2], [28 (exact)], [28 (exact)], [42 (exact)],
[3], [111–138 (bounds)], [111–138 (bounds)], [No finite top vanishing degree],
[4], [194–221 (bounds)], [194–221 (bounds)], [No finite top vanishing degree],
[5], [277–331 (bounds)], [277–331 (bounds)], [No finite top vanishing degree],
[6], [360–414 (bounds)], [360–414 (bounds)], [No finite top vanishing degree],
[7], [443–524 (bounds)], [443–524 (bounds)], [No finite top vanishing degree],
[8], [526–607 (bounds)], [526–607 (bounds)], [No finite top vanishing degree],
[9], [609–717 (bounds)], [609–717 (bounds)], [No finite top vanishing degree],
)

*What the degree means.* First vanishing degree of the graded quotient by the
original highest homogeneous forms. The binary/Koblitz intervals remain
bounds; their exact degrees are unresolved. Positive-dimensional prime top
quotients have no finite first graded vanishing degree in this formulation.
This does not mean infinite Castelnuovo–Mumford regularity. FFD, LFD and solving
degree are separate invariants.

*Actual shared point bases.* From 134217728 candidate x-values per curve:
Koblitz 134220785 points, general binary 134227133 points and prime 134213656
points. Both affine signs are retained. The per-family point manifest stays
unchanged across m=2–9. The point binary layout, shard hashes and restoration
instructions are in `research/n83_fixed_factor_base_20261028/`.

*Execution coverage.* The original fixed-base solver sweep attempted all 48
systems in 96 solver runs: 4 affine completions, 63 timeouts and 29 memory
limits. Caps and the maximum degree of a final affine basis are not regularity
certificates. Subsequent certificates supply the exact/interval/structural
results above. Candidate-base differences still require held-out matched
sampling and do not establish an elite base.

*Direct data files.* `FIXED_BASE_REGULARITY.csv` has 48 rows, one per saved
original system. `FIXED_BASE_REGULARITY.json` defines fields, nulls, source
paths/hashes and the degree convention. This report retains a byte-identical
`fixed-base-regularity-view.json`. `DATA_INDEX.json` maps all older datasets;
`BASE_BINDINGS.json` records original and paired point-manifest identities.

*Scope remaining.* Higher-m exact binary regularity, prime LFD and a base
selection result remain open. The present continuation completes V3 spaces
but certifies no new exact LFD or regularity reduction.
