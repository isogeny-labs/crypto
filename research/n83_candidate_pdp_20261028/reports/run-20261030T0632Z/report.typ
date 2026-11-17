#set page(paper:"a4",margin:(x:17mm,y:16mm),footer:context align(right,text(size:8pt,fill:rgb("667085"))[N83 / prime terminal · #counter(page).display()]))
#set text(font:"DejaVu Serif",size:9pt)
#set par(justify:false,leading:0.45em)
#show heading:set text(fill:rgb("173c50"))
= Higher-m prime terminal certificates
#text(fill:rgb("116d8a"))[n83-prime-terminal-20261030-r3 · all 42 retained prime m3–9 systems]

Observations only. *42/42 producer cases,42/42 fresh replays and42/42 S3-restored replays passed.* Independent scientific review and full algebraic formalization remain pending.

#table(columns:(0.5fr,0.8fr,1.2fr,1.4fr,1.5fr),inset:4pt,stroke:0.3pt+rgb("d8e0e8"),[m],[Cases],[FFD upper],[Affine degree],[Exact FFD / new LFD],[3],[6],[≤17],[16],[unknown],
[4],[6],[≤17],[16],[unknown],
[5],[6],[≤17],[16],[unknown],
[6],[6],[≤17],[16],[unknown],
[7],[6],[≤17],[16],[unknown],
[8],[6],[≤17],[16],[unknown],
[9],[6],[≤17],[16],[unknown],)
#image("degrees.svg",width:100%)

*Boundaries.* Each group contains original/baseline/spike40 × planted/natural. Every stored affine consequence has derivation degree at most17 and actual degree16. Other chain equations may derive it earlier. Nonmembership inV16 is not tested;no new LFD bound follows. Previous prime m2 FFD17 andLFD≥17 are separate results.

*Unchanged workload.* n83,ell27,actual immutable point bases reused across m2–9. Original targets and equations remain fixed. These are correlated retained cases,not independent samples or a factor-base ranking. No elite-base or performance gain is established.

#pagebreak()
= Certificate argument and reproducibility

*Highest-form upper.* The terminal top is `F=U^2*X^2`. The degree13 product `H=product(z_2j-z_(2j+1))` has8192 distinct terms and annihilates `X^2` modulo selector squares. Projection onto the last selector block kills every original top generator while preserving H. Thus H is outside the other-generator ideal and cannot be a terminal Koszul coordinate. The written projection argument gives FFD≤17;earlier-syzygy exclusion is not supplied.

*Exact affine data.* `A0=H,A1=NF(HX),A2=NF(HX^2)`. Store all126 sparse A factors plus explicit degree≤2 B(U) coefficients. The circuit `g=sum Ai*Bi(U)` reconstructs every coefficient. U remains an unbounded intermediate;no field equation or preprocessing is added. PRIME_TERMINAL_FORMAT.json defines every byte and variable mapping. Membership at derivation degree17 alone is not a strict filtered-fall certificate.

*Validation.* 18 native tests;18 independent expanded-polynomial comparisons;18 negative annihilator controls;21 synthetic full-source layouts. Source mutations,wrong base,invalid planted coordinates,wrong projection andcorrupt/truncated factors are rejected. All269 raw payloads match before/after S3 replay. Export verification:zero findings. Data index:19datasets/70hashes. Original48regularityrows andbasebindings unchanged.

*Retained infrastructure failures.* R1 EKS failed exit2 before native computation becauseUID1000 could not read worker.py. R2 fixed permissions but failed exit1 on source HeadObject403. Both versions,protocols,images andlogs remain preserved. R3 used the established host fallback without IAM/capacity changes.

*Execution limits.* One process,2GiBvirtual,60scontrols,300sproducer,300sfreshreplay,+5sgrace,720souter,no retries. Diagnostic batch:2026-10-30T06:32:02Z to 2026-10-30T06:32:46Z. No timing or CRIU claim.

*Reproduce.* Hydrate/extract raw archive;verify manifest;run `instrument replay ROOT ROOT/results`;verify manifest again. Raw archive is sufficient for replay. All failed/development attempts and later delivery receipts are retained separately.

*Remaining scope.* Exact higher-m FFD,strict LFD andglobalupper,binaryquarticfrontier andheld-out elite-base selection remain open. Native replay does not substitute for independent scientific review or full algebraic proof formalization.

*Source.* #link("https://arxiv.org/html/2112.05579")[Caminata–Gorla Definition1.3 and Section4];dataset argument:PRIME_TERMINAL_DESIGN.md.
#text(size:8pt)[Source:`1ec2f68e9a2956dfc55c4e35354e9ce7c0fcaeab` #linebreak() Protocol:`e197dbafacbee40a41a695ffcbf472c35951686d36af58899f58f7acb9b7d143` #linebreak() Data before report:`71f819f5647c26d3ab0095e664d9c7aeeb5acd63`]
