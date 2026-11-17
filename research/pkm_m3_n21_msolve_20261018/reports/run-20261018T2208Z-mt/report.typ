#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= PKM Kummer tower m = 3, N = 21: msolve -t 16 (Amendment A1, minimal P2)
Run 20261018T2208Z, pod `dapncaphacx5w3` (16 vCPU, 128 GB). System `m3_t7_x221026520` (target 0).

#image("msolve-mt-degree.png", width: 100%)

#table(columns: 5, stroke: 0.4pt,
  [*system*], [*outcome*], [*wall*], [*peak RSS*], [*rounds*],
  [control m3_t5 (N = 15)], [exit 0, answer identical], [245.9 s], [2.0 GiB], [as committed],
  [m3_t7 target 0 (N = 21)], [exit 139 in first degree-8 round], [7,088 s], [99.6 GiB / 108.8 GB cap], [27 done, 7 at degree 7])

*Recorded.* The F4 trace equals the single-thread run round for round. Degree 7 ran out of pairs without reaching the unit ideal; msolve then selected 398,409 degree-8 pairs and died inside that round before reporting `new`.

*Reading (A1.4, frozen).* Inconclusive: no productive degree-8 round, memory stop, one target only. The question needs the same system and target 1 on a host where the first degree-8 round fits (256 GB or more per process).

*Not claimed.* Not H0, not a plateau reading, not a cost row or an improvement.
