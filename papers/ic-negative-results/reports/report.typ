#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= BLINDSPOT-19: negative-results paper and Lean statements
*Verdict: DELIVERED* (paper item). Level: none (no exponent, constant or primitive change). Not a speed or cost result; no cross-method row, so no gap to matched rho and no GAE pricing applies.

Run: `examples/blindspots_19_checks.rs` (sha256 751277a8...8ec5), output `evidence/checks.txt`, re-run twice (second pass and this assessment, both under prlimit --as=4294967296) with identical output (ALL CHECKS PASS, about 0.3 s, exact integer arithmetic, no sampling, so no intervals). Host: shared 4-vCPU container, L0 isolation.

#figure(image("lemma4_census.svg", width: 100%), caption: [Recorded values from evidence/checks.txt: census of 7875 pairs and the three witnesses.])

== Statement labels (frozen decision rule in PROTOCOL.md)
#table(columns: (auto, 1fr, auto), 
[S1], [compact-orbit IC on or above S T^2 = r/|Aut|, 14 rows], [RECORDED],
[S2], [isogeny oracles add nothing in generic model; Lean core written], [PROVED; Lean unchecked],
[S3], [yield conserved: every total = C(B+m-1,m); Z/11..13 checked], [PROVED, EXACT],
[S4], [monodromy (Z/2)^(m-2)], [DERIVED, UNREVIEWED],
[S5], [Lemma 4 false over F_2: witnesses A,B,C; census 6138 outside, 4830 inside violations], [EXACT],
[S6], [Assumption 1 off diagonal], [NOT ESTABLISHED])

Controls passed: positive (X, X+1: d\_F = d'\_F = 1), negative (Z/8, Z/9 show more than one agreement; field needed), yield mirror (AP base: same total, lower coverage 7 vs 9).

== Campaign items that bear on the statements (second pass, PROTOCOL.md Addendum B)
Quoted from each item's own verdict; none is promoted and no label changed.
#table(columns: (auto, 1fr, auto),
[01], [REGRESSES (constant), INCONCLUSIVE (exponent): distance to matched rho], [S1 context],
[02], [ADVANCES (derived, model level), conditional], [S6],
[04], [ADVANCES (primitive, stage diagnostic); incumbent closure degree D = 4], [S6 context],
[05], [ADVANCES (constant, wdeg); D\_mat never below 4 on four countable cells], [S6 context],
[07], [BLOCKED at (15,5,4), the deciding cell], [S6],
[10], [INCONCLUSIVE; conservation check not run], [S3],
[14], [REGRESSES (primitive), first measured m = 4 cells], [S1 context],
[17], [TRADES (constant); solving-degree leg BLOCKED; cost, not yield], [S1, S3 context])
Items 03, 06, 08, 09, 11, 12, 13, 15, 16 do not bear; no number cited. Item 10 README digest is the version read at 20:0x on 2026-10-18 and may still change.

== Caveats
Lean modules are written, not kernel-checked (no toolchain on host); the CI proof gate is the first check. S6 demoted from the design. S1 S T line holds only at the rank-stage mean. Relabelling of existing negatives is not an improvement under the success criterion. Authors and emails owed from a human; nothing submitted.
