#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9.5pt)
= Native screen: primary and secondary
Run group 20261018T0437Z · 2026-10-18 04:37:30–04:37:45 UTC

Source b3ed888964100f1bce4780a4c0500a399c7aee50; executable
dd9c1e582effe751777081fc4d7fc6ba1acd90e1696797ef210d20da58c3c3f0.
Protocol commit 846cc9433 precedes execution. Plans primary.json (5 cases)
and secondary.json (43 cases), seed 2026101801, one thread, degree 8,
2048 MiB and2 million observed matrix cells. Primary30 s/candidate,
secondary20 s/candidate. This shared Linux x86-64 host has no timing qualification.

== Primary: five completed candidates
#image("../../primary.svg",width:100%,height:115mm,fit:"contain")
Every primary control passes under the original whole-curve scope; no random
train/holdout probe decomposes. Exact independent support is shown, including
zero-support cases. Actual r = 1,439,393, cofactor 1,492. Coordinate/reversed/
triangular bases have the same signed points; their ordered hashes differ.
Positive-control targets can differ, so their costs are not compared.
Native verification checks 5 seals and 5 point witnesses.

Raw records: runs/primary, primary.stdout.json, primary.stderr.log and
primary.verify.json inside ../../runs.tar.xz. New subgroup-aware behavior is
validated separately; these old records are not rewritten.

#pagebreak()
= Secondary: 42 complete, one failed
#image("../../secondary.svg",width:100%,height:228mm,fit:"contain")
#text(size:8pt)[Secondary native verification checks 43 seals and 596 point
witnesses. The failed dimension 3 trace-zero/Frobenius seed 29 has zero points,
not a completed zero-yield run. Full data: observations.json and raw
runs/secondary. Base size, duplicate coordinate sets and subgroup model differ;
counts are descriptive. Exact support is an audit, not a speed claim.]
