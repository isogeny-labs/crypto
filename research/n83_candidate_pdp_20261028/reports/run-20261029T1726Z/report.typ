#set page(paper:"a4",margin:(x:15mm,y:14mm),footer:context align(right,text(size:8pt,fill:rgb("64748b"))[N83 / strict degree-four witnesses · #counter(page).display()]))
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.45em)
#text(size:21pt,weight:"bold")[Strict degree-four falls]

#text(size:11pt,fill:rgb("116d8a"))[Retained n83 m3 systems · original / baseline / spike40]

*12/12 producer and fresh-replay pairs passed; 12/12 S3-restored replays passed.*
2988 strict coordinate witnesses among 2988 verified candidates.
12 systems acquire *LFD >= 4*. Exact LFD and global upper bounds remain unknown.
Independent scientific review is pending.

#text(size:8pt)[#table(columns:(2.5fr,.6fr,.6fr,.8fr,.7fr),inset:4pt,stroke:.35pt+rgb("d3dae4"),
[Retained case], [Tested], [Strict], [V3 rank], [LFD],
[original-koblitz-planted], [249], [249], [36159], [>= 4],
[original-koblitz-natural], [249], [249], [36159], [>= 4],
[original-binary-planted], [249], [249], [36159], [>= 4],
[original-binary-natural], [249], [249], [36159], [>= 4],
[baseline-koblitz-planted], [249], [249], [36159], [>= 4],
[baseline-koblitz-natural], [249], [249], [36159], [>= 4],
[baseline-binary-planted], [249], [249], [36159], [>= 4],
[baseline-binary-natural], [249], [249], [36159], [>= 4],
[spike40-koblitz-planted], [249], [249], [36173], [>= 4],
[spike40-koblitz-natural], [249], [249], [36173], [>= 4],
[spike40-binary-planted], [249], [249], [36173], [>= 4],
[spike40-binary-natural], [249], [249], [36173], [>= 4],
)]

#image("strict-candidate-counts.svg",width:100%)
#text(size:8pt,fill:rgb("475569"))[K/B: Koblitz/general binary; P/N: planted/natural.
Coordinate witnesses are correlated. Counts do not rank base quality.]

*Unchanged inputs.* Every case retains n83, m3, ell27, 164 Boolean variables and
its exact saved point-base manifest. No point, target or equation changed.
Original and paired planted targets differ; natural original/baseline sources repeat.
The former lower bound 3 is inherited; the degree4 witnesses are new.

#pagebreak()
#text(size:18pt,weight:"bold")[How to check the evidence]

*1. Check the source.* Assemble all 83 coordinates of the first saved cubic into
F=S3(X0,X1,U), including every lower term. Match the complete field polynomial
to the unchanged equation source.

*2. Check membership in V4.* Multiply F by X0,X1,U in the ordinary polynomial
ring. Every coordinate has a degree<=4 derivation. Reduce repeated variables
with the original Boolean equations; retain the correction provenance and
verify that each normal form has degree<=3. Test all 249 coordinates.

*3. Check nonmembership in V3.* Replay complete V2, the immutable V3 prefix and
completed suffix. Reduce each candidate against that complete basis. The files
retain candidate bytes, ordered reducer IDs and full remainders. A nonzero
remainder proves a strict degree4 fall and LFD>=4.

*What remains unknown.* The test does not construct complete V4. It cannot
distinguish LFD=4 from LFD>4. An allzero finite family would give no global upper
bound. No degree-of-regularity reduction or elite-base advantage follows.

*Validation.* Fourteen native tests pass, including independent tiny V3/V4
closures, polynomial long-division arithmetic checks, degree guards and altered
trace/witness rejection. Runtime controls: xyz+1 gives 3 strict witnesses; xyz
gives 0. The initial fixture setup failure remains archived.

*Restored data.* All 370 new raw files,
230 prefix files and
270 suffix files match their manifests
before and after replay. All candidate, remainder and ordinary/normal polynomial
files are retained, with 12 case summaries and
36 block summaries. Both parent archives are required.

*Resources and limits.* One native process, 8 GiB, 1800 s per producer/replay plus 5 s
grace. Existing-host L0 fallback after the retained failed EKS storage probe;
its application cause remains unconfirmed. No performance clock or speed claim.
Batch boundaries: 2026-10-29T17:26:50Z to 2026-10-29T17:50:29Z.

*Reproduce.* Read `DEGREE4_WITNESS_FORMAT.json`, the native summary in
`evidence/degree4-witness/`, and `evidence/degree4-witness-storage.json`.
The unified export binds outcomes to the frozen protocol and raw hashes.
#text(size:8pt)[Source: `17f501a6e6b918525c1f0a9bee84b398ab7430b4` #linebreak()
Protocol: `605323399f307993d011309e52f91b364bd31347e1107bd31d4c911de83d516d` #linebreak()
Data before report: `38611ee034d53409c028b5c7deabfb06b104d985`]

*Literature.* #link("https://arxiv.org/html/1504.01175")[Semaev, section4.5]
supplies the multiplier identity. #link("https://arxiv.org/html/2112.05579")[Caminata--Gorla, Theorem2.8]
supplies the strict-fall criterion. The full n83 three-family, m2--9 FFD/LFD and
elite-base objective remains open.
