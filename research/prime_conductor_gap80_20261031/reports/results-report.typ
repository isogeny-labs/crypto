#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
= Native helper and 80-bit source/kernel

The native helper shares the trace functional and the modulus reciprocal.
It passed the 96-case direct-root oracle and 96 rejection controls across
three fields. The independent PARI checks verify the frozen source/kernel
arithmetic and a replayable 202-bit field-prime certificate.

#image("helper-flow.svg", width: 100%)

== Retained observations, not admitted performance rows

#image("historical-timings.svg", width: 100%)

The saved ratio of baseline/candidate medians is 1.189706 at native
polynomial degree 4,096 and 1.175257 at degree 16,384 (seven pairs each,
unpinned). On the same 80-bit partial tile it is 1.088610 (three retained
CPU-0 pairs), an arithmetic 8.139711% reduction in that interval. These
are historical diagnostics. The run report documents Python/Sage tile
provenance, all exclusions, host limits and missing uncertainty/replay.
No result is promoted into the admitted benchmark or IC/rho tables.

== Exact candidate and open construction

The prime degree is 729845429276318409687041 (80 bits). The source is
y² = x³ − 140x + 784 over the 202-bit prime in `../parameters.json`.
The ordinary CM source has j = −3375 and maximal order of discriminant −7;
the inert prime-degree kernel gives the structural conductor argument.
Native arithmetic and independent PARI verify the rational kernel and
41-bit subgroup, plus a trace-sign witness. This is not a formal proof
assistant certificate or an explicit target-curve computation.

The explicit quotient equation and complete non-kernel images remain
uncomputed. At block 4,096 the current bounded method needs
10,875,544,558,802,641 rectangular tiles plus 722,432,689,208 leftover
roots. The next construction must complete the whole map with recorded
native costs. Matched endpoint trials depend on that destination and
transported base. No 80-bit evaluated crossing or largest-known record is
established. The published 263-bit comparison does not establish one either.

== Evidence, admission and follow-up

The source-linked study README, unchanged summary, parameters, field-prime
certificate, frozen archive and checksum are one directory above. Native
C source and portable build/verification entry points are in `../native/`.
Historical Python remains archived evidence, not an active execution path.

The existing obligation is tracked in
`problems/conductor-gap-upper-floor-and-cost.md`. Documentary copies live
in `benchmarks/20261031-gap80-native-diagnostic/`, outside the admitted
table under the validation gate. Canonical leaderboard, IC/rho progress,
curve registry and browser graphs remain unchanged: no admitted comparison,
completed crossing or registered curve promotion is supplied. References
and the complete retained measurement flow are in the README/run report.
