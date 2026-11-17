#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 10pt)
= Retained structured N83 archive import check

*Date:* 2026-10-17 UTC. *Outcome:* PASS. This is a native import and
integrity check. Solver search, natural relation yield, useful rank, and
complete cold cost were not executed.

The worker authenticated the retained compressed object and manifest,
checked the curve, source projection, subgroup and Frobenius identities,
replayed every point/phase/sign label, and rebuilt the generic factor-base
map. The output was 332,166 points, 166,083 distinct x-coordinates and
2,001 signed-Frobenius columns.

#image("import-diagram.svg", width: 100%)

== Provenance

Source commit: `1b00cf8a3450540aacd7778b6b2115276f46a81c`.
Executable SHA-256:
`a24a7b28418e0c44ec47ab5ab03234ab9f419d9ea2e730c420ab467cc2483b32`.
Object SHA-256:
`c5de0b26e84ad80bfb7be6a214b89c9682e356effb6079c7abf28cd1151ffb03`.
Manifest SHA-256:
`bd4202ed599f2659f6279dd07dbdc6b5948594df79db969e66530c19e3cf5dca`.
The macOS release `verify` command had a 300-second outer cap and exited 0.
The machine was shared (L0); no valid wall or memory measurement was recorded.

== Limits

The receipt is `verify.json`. A missing sparse-checkout input and an initial
attempt to launch a test harness as the executable are preserved in separate
failure logs. The subsequent bounded m = 4 and m = 5 results are recorded
in `RUN_REPORT.md`; a matched explicit-orbit comparison remains open.
