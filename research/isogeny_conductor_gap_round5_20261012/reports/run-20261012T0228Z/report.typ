#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
= Run report: E6, yield of a pushed-forward factor base
_2026-10-12, 02:27–02:30 UTC. research/isogeny_conductor_gap_round5_20261012._

*Runner* examples/isogeny_gap_pushforward_yield.rs (commit cb33477a1), 20,000 targets per curve; cloud container, no wall times reported. *Records* e6/records.jsonl: 112 arm records on 15 curves (crater, floor, floor model of five 22-bit round-1 pairs with h = 12, 18, 50, 72, 200), bases of the 32 and 64 smallest abscissae outside E[h]; arms direct and quotient by C2, C4, E[2] (where rational) and E(F#sub[p])[h]. Every decomposition lifted back to E and checked: 0 failures.

#table(columns: 5, align: right,
  [subgroup], [|H|], [cases], [quotient ÷ direct], [95 % interval],
  [C2], [2], [30], [1.81], [1.64 to 1.99], [C4], [4], [16], [4.08], [3.52 to 4.75],
  [E[2]], [4], [6], [3.57], [2.76 to 4.60], [E[h]], [12], [6], [8.43], [6.00 to 11.86],
  [E[h]], [18], [6], [21.87], [17.70 to 27.01], [E[h]], [50], [6], [44.07], [39.24 to 49.50],
  [E[h]], [72], [6], [79.57], [67.54 to 93.73], [E[h]], [200], [6], [139.39], [103.62 to 187.50])

Derived: weighted least squares of ln(ratio) on ln|H|, exponent 1.001 ± 0.036. F4 (one random target per arm): equal cost at matched columns on the 106 unsolvable targets (136,476–145,630 multiplications at c = 32, 581,788–600,510 at c = 64); 483,561 and 2,991,006–6,313,667 on the 6 solvable ones.

#figure(image("e6_yield_ratio.svg", width: 92%), caption: [Decompositions on E/H ÷ on E over the same targets and columns, per curve and pooled (95 % Poisson intervals), against |H|. Records e6/records.jsonl.])
