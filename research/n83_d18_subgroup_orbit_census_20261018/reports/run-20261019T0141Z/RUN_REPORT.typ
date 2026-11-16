#set page(paper: "a4", margin: 14mm)
#set text(font: "New Computer Modern", size: 9.5pt)
= N83 direct-subgroup x census, dimension 18

*Protocol commit:* `b28bf2904`. *Native source commit:*
`9e483ab2ea4ce4d18b6b4e3bc5070a8db547b8c2`. The public primary
curve is `icv1-f2m83-tm6151469093347-debefd74`; source x spans all
262,144 polynomial words with bits 0–17. The exact binary SHA-256 is
`571f993f4bddc32e2e189d0778ce5b62f883460dff1dcf65a65788cc4f368bee`.
The complete native direct run was 2026-10-19 01:42:39–01:42:58 UTC on
shared arm64 macOS. It exited 0. No target entered the census.

#image("counts.svg", width: 100%)

== Measured and derived counts

#table(
  columns: (2.7fr, 1fr), inset: 3pt,
  [*Population or phase*], [*Exact count*],
  [Source x words], [262,144],
  [Rational x words], [130,724],
  [Direct-subgroup x words], [32,710],
  [Distinct full signed-Frobenius classes], [32,643],
  [Selected columns / frozen threshold], [16,627],
  [Derived selected closure points], [2,760,082],
)

Every subgroup x had orbit length 83. Class source multiplicities were
32,577 once, 65 twice and one three times. The direct run's internally
measured process wall was 18,865.461 ms at L0 while a separate release
suite ran. The first attempt wrote byte-identical class and selected CSVs
but `/usr/bin/time -l` exited 1 when its post-run macOS `sysctl` was denied;
its native internal wall was 24,721.084 ms. Both attempts and the failed
wrapper are preserved in `raw-files.sha256`. Seventeen selected points were
rechecked through the general curve, subgroup and Frobenius APIs. A fully
independent implementation has not replayed all 262,144 inputs.

The selected closure's point count is a consequence of 16,627 disjoint
83-phase, two-sign classes. The point archive and index have not been built;
natural relation yield, useful rank and complete cost are open. `RUN_REPORT.md`
and `RESULT.md` link the raw CSVs, audit and next export obligation.
