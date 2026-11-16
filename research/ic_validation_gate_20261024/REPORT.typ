#set page(paper: "a4", margin: 16mm)
#set text(size: 9pt)
#set heading(numbering: none)
= IC validation gate, Tables B and C (2026-10-24, Conductor T-2)

*Question.* Can the existing rows of leaderboard Tables B and C pass the validation gate (replay certificate, validator report,
matched rho reference with the section 5a band)? Decision rule R1 to R4 frozen in PROTOCOL.md before any replay.
Nothing here is an improvement; it decides which numbers may be shown.

*Result (measured and derived, this study).*
- Table B: 13 of 13 rows pass R1 to R4. 1257 counted runs replay identically in a fresh process; 633 archive and 945 replay scalar runs verify as [d]G = Q in an independent verifier (1578 of 1578 negative controls rejected); all 52 headline numbers recompute to 1e-9. IC stays above rho at every weight of the section 5a band. The manifest is staged for docs/ic/runs (outside this task's write scope).
- Table C: 0 of 9 pass. R1 holds (27 of 27 sampled replays identical), R3 holds (36 numbers), R2 cannot be checked (no target points archived) and R4 fails: the reference is a different study's batch rho, not a same-process arm, and no unpriced counts exist for a band.
- Gap to matched rho (distance remaining): Table B 5.47x to 38.21x (4.05x to 54.76x across the band); Table C 3.62x to 14.90x.

#figure(image("figures/row_outcome.svg", width: 100%), caption: [Per-row outcome of R1 to R4 (recorded verdicts; x reference from the board).])

#pagebreak()
== Per row (S in GAE per sqrt r for B, units per sqrt r per target for C)
#table(columns: (auto, 1fr, auto, auto, auto, auto, auto), align: (left, left, left, right, right, right, right), inset: 3pt, stroke: 0.4pt,
  [table], [row], [standing], [S_IC], [S_ref], [x ref], [IC/rho over band w = 0, 0.1, 1],
  [B], [icv1-fp10-t5-192cb216], [pass], [22.85], [3.605], [6.34], [6.11 to 6.26],
  [B], [icv1-fp12-t19-d4a315df], [pass], [15.62], [2.855], [5.47], [4.66 to 5.31],
  [B], [icv1-fp14-tm43-158b6914], [pass], [13.07], [2.034], [6.43], [6.26 to 6.27],
  [B], [icv1-fp16-t351-d433f24e], [pass], [11.68], [1.74], [6.71], [6.31 to 7.43],
  [B], [icv1-fp18-t175-45770ebc], [pass], [8.414], [1.446], [5.82], [5.65 to 6.28],
  [B], [icv1-fp20-t1293-3c2999ba], [pass], [10.72], [1.292], [8.30], [8.10 to 9.50],
  [B], [icv1-fp22-tm1267-a5acaf6d], [pass], [12.77], [1.216], [10.50], [10.24 to 12.64],
  [B], [icv1-fp24-t1577-77336155], [pass], [13.97], [1.038], [13.46], [13.02 to 16.30],
  [B], [icv1-f2m15-t131-ed7256f0], [pass], [26.31], [2.013], [13.07], [10.28 to 13.01],
  [B], [icv1-f2m18-t393-d2c04152], [pass], [21.3], [2.006], [10.62], [10.51 to 12.76],
  [B], [icv1-f2m21-t2591-de54d72e], [pass], [24.39], [1.356], [17.99], [17.73 to 26.13],
  [B], [icv1-f2m24-tm3047-2d0c0e8c], [pass], [27.24], [1.183], [23.03], [22.76 to 32.03],
  [B], [icv1-f2m27-t12543-569dca8b], [pass], [36.93], [0.9665], [38.21], [37.73 to 54.76],
  [C], [icv1-f2m19-tm797-9c54981b], [fail R4], [4.873], [0.3271], [14.90], [no band],
  [C], [icv1-f2m23-tm5197-1f85e9e1], [fail R4], [1.432], [0.205], [6.99], [no band],
  [C], [icv1-f2m45-tm6236725-40939294], [fail R4], [0.8563], [0.1191], [7.19], [no band],
  [C], [icv1-f2m37-tm534059-32aad96b], [fail R4], [0.4853], [0.09224], [5.26], [no band],
  [C], [icv1-f2m43-tm998717-e2e742b0], [fail R4], [0.3248], [0.06663], [4.88], [no band],
  [C], [icv1-f2m47-t22705043-f4e44623], [fail R4], [0.2344], [0.06022], [3.89], [no band],
  [C], [icv1-f2m41-tm2308219-7f48b14a], [fail R4], [0.2408], [0.0666], [3.62], [no band],
  [C], [icv1-f2m53-tm56619371-dac20a85], [fail R4], [0.2605], [0.05095], [5.11], [no band],
  [C], [icv1-f2m61-t158598901-ab42b6c5], [fail R4], [0.4885], [0.05308], [9.20], [no band],
)

== The 96 root-finder replay mismatches
`ecbench verify --replay-all` on session ECBS1hfd5bf1885227, current binary: 72 of 168 identical, 96 differ, same scalars. By arm: ic-f4-m2 24, ic-f4-m3 24, ic-rootfind-m2 24, ic-rootfind-m3 24 differ; rho-strong, rho-strong-aa, rho-frob 24 of 24 identical each.
Field level (each flagged record re-executed and diffed): the 48 F4 rows differ in relations.native.solver\_ops (m2 300,531 to 301,961; m3 2,832,171 to 3,732,225) and are repriced (word XORs now charged). The 48 root-finder rows differ only by two new counters in oracle\_setup (pair\_table\_kind 0, pair\_table\_points 33); every existing counter, every GAE and every scalar is equal.
So the statement that the 96 are inherited-F4 rows and not root-finder rows is refuted as stated (half are root-finder rows), while the substantive point stands: no root-finder number changes. Table B and C rows read ic boundary and ic price oracle counts, which R1 re-executes identically; they are not affected.

== Limits
Binaries were built from the working tree with other agents' uncommitted edits (koblitz\_index\_calculus.rs, ic price.rs, workflow.rs); all replays agree with the archive and with sweep A's clean-build count (72 of 168). Table C scalars are not independently checked. Table A and the v4 run are not covered here. L0 shared host; no timing is used for B.
