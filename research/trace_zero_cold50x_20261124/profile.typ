#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
#let p=json("profiles/baseline-cold/hotspots.json")
#let aa=json("profiles/baseline-aa-cold/hotspots.json")
= ECC2K-130: another 50x target, current cost profile

Complete fixed degree131 d12 uniform query; one fresh process per arm. This is a descriptive simulated-instruction profile, not a native timing result.

#table(columns:(2fr,1fr),inset:6pt,
[*Quantity*],[*Observed Ir*],
[Baseline complete cold],[#p.total_ir],
[Identical A/A complete cold],[#aa.total_ir],
[Proposed 50x total ceiling],[427207],
)

== Largest self costs
#table(columns:(3fr,1fr),inset:5pt,
[*Function / attribution*],[*Ir*],
[Relation decomposition],[#p.hotspots.at(0).ir],
[Fused multiplication],[#p.hotspots.at(1).ir],
[Hex decoding],[#p.hotspots.at(3).ir],
[Trace table construction],[#p.hotspots.at(4).ir],
[Compact squaring],[#p.hotspots.at(5).ir],
)

Rows above are disjoint self attribution entries, not whole-call costs. Inlining also attributes related work to other source files. Full raw annotations and both profiles are retained.

== Consequence for the requested target
The current hex-decoder attribution alone exceeds the proposed total ceiling. A solver-only change retaining that observed ingestion cost cannot meet 50x. This is conditional on the same measured ingestion path, not a general impossibility result.

== Reproduction and limits
Binary SHA256: #raw("fe0d7ea116db22549af4453b4afb8bdfa36dc7c902176253c95fc23956a61a7d"). Callgrind3.24.0, one CPU0, two GiB address-space cap, no native clock claim. Protocol TZ50X-20261124-current-profile was frozen and published before data. Canonical export has zero findings. PCC-20261124-f46d25; unsigned producer evidence. [pcc:unverified]
