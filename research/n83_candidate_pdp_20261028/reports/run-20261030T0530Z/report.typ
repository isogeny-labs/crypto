#set page(paper:"a4",margin:(x:17mm,y:16mm),footer:context align(right,text(size:8pt,fill:rgb("667085"))[N83 / prime first-fall · #counter(page).display()]))
#set text(font:"DejaVu Serif",size:9pt)
#set par(justify:false,leading:0.45em)
#show heading:set text(fill:rgb("173c50"))
= Prime m2 first-fall certificates
#text(fill:rgb("116d8a"))[n83-prime-first-fall-20261030-r1 · six retained n83 systems]

*6/6 producers, 6/6 fresh replays and 6/6 S3-restored replays passed.* Independent scientific review and full algebraic formalization remain pending.

#table(columns:(2.4fr,0.8fr,1fr,0.9fr,1fr),inset:4pt,stroke:0.3pt+rgb("d8e0e8"),
 [Retained input],[FFD],[Strict fall],[LFD lower],[Upper/exact],
 [original-planted], [17], [17], [≥17], [unknown],
[original-natural], [17], [17], [≥17], [unknown],
[baseline-planted], [17], [17], [≥17], [unknown],
[baseline-natural], [17], [17], [≥17], [unknown],
[spike40-planted], [17], [17], [≥17], [unknown],
[spike40-natural], [17], [17], [≥17], [unknown],)
#image("degrees.svg",width:100%)

*Witnesses.* Each has a derivation of degree at most17 and a degree16 normal form outside V16 under the written filtered-space argument. All six production functionals are nonzero. Global LFD upper and exact values remain unknown.

*Unchanged data.* n83,m2,ell27,54 selectors. Actual point manifests,targets and original equations remain fixed; each point base is reused across m2–9. Original-top regularity remains42. Original/baseline natural inputs repeat. No elite-base or performance gain is established.

#pagebreak()
= Algebraic certificates and reproducibility

*Highest-form FFD.* Normalize with `z_i=w_i*v_i`. Selector squares give the square-zero associated graded algebra. The original quartic top is `X^2Y^2`. Multiplication ranks are injective through multiplier degree12. The degree13 product `H=product(z_2j-z_(2j+1))` is nonzero and annihilates `X^2`. The written Koszul/annihilator argument gives first-fall degree17; rank arithmetic and the sparse witness are checked natively.

*Strict affine fall.* Store `g=NF(H*f)` exactly as three tensor products of six complete sparse factors. The degree16 coefficient functional,computed directly from complementary masks,equals `2*(-2)^13*(sum(weights)-2*target_x)`. Nonzero implies a strict degree17 fall using the written characterization of V16. V16 is characterized algebraically;no dense V16 basis or global ideal upper certificate is claimed. A zero functional is inconclusive for this test.

*Validation.* 12 native tests; 36 expanded-polynomial comparisons; 12 small dense-closure comparisons; 12 exceptional-target controls. Inherited rank controls include small-characteristic deficiencies. Source checks require the entire equation template and exact planted-witness suffix. All 71 raw payloads match before and after S3 replay.

*Bounds and execution.* 2GiB virtual memory;60s controls;300s producer and300s fresh replay;+5s termination grace;720s outer;no retries. Batch UTC:2026-10-30T05:30:23Z to 2026-10-30T05:30:36Z. These are diagnostic boundaries,not a performance measurement. EKS CLI role assumption was denied and MCP inventory timed out;existing-host fallback,no new EKS job or IAM/capacity change.

*Reproduce.* Hydrate/extract the raw archive,verify its manifest,run `instrument replay ROOT ROOT/results`,then verify the manifest again. PRIME_FIRST_FALL_FORMAT.json defines every factor byte and tensor reconstruction. The raw archive is sufficient for replay;source and base identities remain bound to earlier datasets.

*Scope.* Global prime LFD upper/exact,higher-m work,the binary quartic frontier and held-out elite-base selection remain open. Native replay is distinct from independent scientific review and a full algebraic formal proof.

*Sources.* #link("https://arxiv.org/html/2112.05579")[Caminata–Gorla Definitions1.3/1.5 and Section4]; #link("https://arxiv.org/pdf/2211.13548")[Phuong–Tran Theorem2.7]. Dataset-specific proof:PRIME_FIRST_FALL_DESIGN.md.
#text(size:8pt)[Source:`30cad456257df5250058717f0b5ae8f385cf30fd` #linebreak()
Protocol:`5c10dc03ea5f1447ed50430d297c691bbc008316dff21a9c6d7c3240c13f291f` #linebreak()
Data before report:`8e6dc2ac89862f0f4ea5ad1e0317f27cb4b4c5a0`]
