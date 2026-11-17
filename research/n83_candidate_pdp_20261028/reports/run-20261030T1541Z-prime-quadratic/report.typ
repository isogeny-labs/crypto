#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Serif",size:9pt)
= Bounded prime quadratic coefficient maps
*Observations only; independent scientific review and formalization pending.* Run n83-prime-quadratic-20261030-r1. Seven templates and all42 source cases passed production, fresh replay and S3-restored replay. Zero failed, capped or timed-out templates.
#table(columns:4,inset:4pt,[m],[Source cases],[Rank / columns],[FFD interval],[3],[6],[464 / 464],[7–17],
[4],[6],[1311 / 1311],[7–17],
[5],[6],[2824 / 2824],[7–17],
[6],[6],[5195 / 5195],[7–17],
[7],[6],[8616 / 8616],[7–17],
[8],[6],[13279 / 13279],[7–17],
[9],[6],[19376 / 19376],[7–17],)
#image("ranks.svg",width:100%)
*Meaning.* The recorded degree-six maps are injective. The pair-covering and selector-square regular-sequence argument in PRIME_QUADRATIC_DESIGN.md gives conditional highest-form FFD ≥7. Matching exact source and factor-base hashes to the earlier upper17 dataset gives7–17 for all42 cases. Seven shared maps are not42 independent rank experiments. No exactFFD,newLFD,solving-degree,degree-of-regularity or elite-base improvement is established.

*Inputs.* Original,baseline,spike40 arms; planted and natural targets;m3–9. Each arm retains its immutable factor base and source system. The full three-family m2–9 objective remains open; this continuation measures prime cases only.

*Execution.* One template process at a time;8GiB address space;300s producer and300s replay,5s termination grace;120s assembly passes;4800s outer limit;zero automatic retries. Caps:100000columns,5000000coefficients,50000000reduction updates,2000000basis entries. Exact commands and identities are in the frozen protocol and archived runner. Existing-host fallback; fresh EKS source-access probe timed out with no retained Pod/log and gives no access verdict. Interval 2026-10-30T15:41:50Z–2026-10-30T15:43:04Z is diagnostic only,with no isolation or speed claim.

*Validation and retention.*26 native test invocations comprise10 new tests and16 duplicated donor invocations;two runner controls retain failure/timeout journals. All198 raw payloads matched their hashes before and after S3 replay. Every coefficient column and elimination row is retained. Archive86,270,953bytes; format and replay instructions:PRIME_QUADRATIC_FORMAT.json and evidence/prime-quadratic/README.md. Metadata publication required a directory-path correction; no experiment rerun.

#text(size:7pt)[Data commit:42ea5dfd9ff1eac38f52333e137a17bad93f1b24 #linebreak() Raw SHA256:1db8c8bec12f40ac3ef2a239eba5654541c7b01097b4befd80d12676d7275da9]
Primary definitions:#link("https://arxiv.org/html/2112.05579")[Caminata–Gorla,Definition1.3 and Section4].
