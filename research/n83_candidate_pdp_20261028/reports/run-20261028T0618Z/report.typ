#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Paired n83 factor-base fall certificates
Observations only. This panel measures input-specific first-fall certificates and degree-two closure. It does not measure regularity or intermediate solving degree and does not establish elite-base status.

== Matched experiment and outcome
The baseline uses coordinate positions0..26; spike40 uses0..25,40. Both contain exactly2^27 candidate x-values, with all affine points stored. Each curve reuses its fixed base across m2..9. Curves, selector equations, chained S3 representation and preprocessing are identical. Original natural targets are shared; planted tuples come from the common low26 support and are identical across arms. This is not uniform planted sampling over either complete base.

All96 systems completed their first-fall computation and equation catalog. All64 binary V2 closures and all160 fresh-process producer replays completed. No process failure, timeout or exclusion. Native comparison checked48 paired targets, tuples and witnesses. Natural baseline source hashes match the earlier fixed-base systems.

#table(columns:(2fr,1fr,1fr),inset:5pt,
 [Recorded quantity],[Baseline],[Spike40],
 [Binary/Koblitz FFD, all m],[2],[2],
 [Prime FFD, all m],[at least5],[at least5],
 [Binary/Koblitz m2 exact LFD],[2],[unknown],
 [Binary/Koblitz m3..9 exact LFD],[unknown],[unknown],
 [Prime exact LFD],[unknown],[unknown])

#image("linear-ranks.svg",width:100%)
Recorded ranks from n83-candidate-paired-falls-20261028-r1; four matched m2 cases with54 Boolean variables. Exact ranks, no sampling uncertainty. FFD is2 in every plotted case.

For baseline planted cases, V2 has53 independent linear constraints and both remaining Boolean assignments satisfy the full original system. Baseline natural cases contain1 in V2. These supply global LFD2 upper certificates. Candidate V2 has17 independent linear constraints and no global upper certificate. Unknown candidate LFD does not imply LFD greater than2. For m3..9, both arms have linear rank1 and equal recorded closure ranks; exact LFD remains unknown.

#pagebreak()
== Definitions and claim limits
FFD is established by a nonzero degree-fall witness or a prime lower-bound certificate. Prime degree-four coefficient minors exclude falls through degree4; exact FFD remains unknown. V2 preserves ordinary multiplication-degree limits before Boolean reduction. A closure plateau is not a global LFD upper bound. Only a unit or exhaustive affine-completion certificate supports the reported exact LFD2.

The differing m2 linear ranks are recorded alongside the same FFD. This panel supplies no regularity reduction, solver-speed comparison, universal FFD/LFD theorem or ECDLP cost result. Earlier top-quotient certificates describe a separate metric and remain in their original study. Candidate regularity and solving degree remain unmeasured.

== Execution and validation
Run n83-candidate-paired-falls-20261028-r1 completed28 October2026,06:18:11–06:25:17UTC on the existing Spot workspace. One native process at a time,8GiB virtual cap,60seconds per closure/replay,600seconds per first-fall/catalog batch,1200seconds per input generation,14400seconds outer. No retry. CPU user/system time and peak RSS were not recorded. Wall timestamps are diagnostic, not performance evidence.

Six generator tests and eight first-fall tests pass, including altered selector-weight rejection. The unchanged V2 instrument retains its previous native controls. All989 raw-manifest file hashes pass. Every96 first-fall replay status and64 closure/replay status is zero. Producer replay is not independent research review.

#text(size:8pt)[
Source commit: ca3ac5bd0d371a25883cc8e5b05af15fcb7ffdd4\
Protocol: 36a0b990d80dbe2e4660d1910eed3dfa39e23bea4cb4630b4b5eeac2ba4531ab\
Raw archive: 25f88306bde0643f435ffe3db3ab1de9c95952f9e3476242083d4ab551c23845\
Source archive: de1a23cd8ece0b5fb28c54ea62e77d428370a7469095887fd19559ee6489ab0b
]

== Data structure and durable access
../../experiment-pdp.export.json binds96 case rows to the frozen protocol. ../../evidence/pdp/paired-summary.json records both arms in48 matched rows. ../../PDP_FORMAT.json and ../../PDP_COMPONENT_FORMAT.json specify source equations and offsets, variable blocks, closure-row binary encoding, replay operations and unknown-value semantics. The full archive includes the exact executables and complete raw tree. Source, errors and previous datasets remain retained.

#raw("s3://setec-astronomy-artifacts/runs/n83-candidate-paired-falls-20261028-r1/local/",block:true)

Storage and reconstruction: ../../evidence/pdp-storage.json. All affine point files remain separately addressable through ../../data/index.json and per-family artifact manifests. See PDP_PLAN.md for the preserved full objective. A higher-degree closure on these same m2 candidate inputs is the next planned measurement; it requires a new frozen protocol and is not a result of this run.

Primary definitions and cautions: #link("https://arxiv.org/html/1906.05594")[Kousidis and Wiemers]; source-specific fall and closure conventions are fixed in the protocol and native code. No claim that these chained-system certificates prove the paper's global heuristic.
