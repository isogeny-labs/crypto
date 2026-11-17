#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= R6.7: the ζ-fold of the Gaudry base over $FF_(p^2)$, phase two

Round 6, 2026-10-18 (run start 20261018T2056Z UTC), base commit abee07ee73. Example `examples/r6_7_zeta_fold_phase2.rs`; commands in `run.sh`; raw records `*.json` in this directory; full tables in `RESULTS.md`. Shared cloud container, seven runs concurrent, no isolation; wall times contended and unused. Toy synthetic curves only; no claim about rho or deployed curves.

== Arms
At $m = 3$ on the torsion family: *A* plain $F = {x in FF_p^*}$ folded by $⟨-1, zeta⟩$; *B* T-saturated $F_c = phi^(-1)(F'')$ on $E$ (18 points a column), run through a wrapper around the repository `LineOracle`; *C* the quotient $F''$ on $E'$. Cost unit: $FF_p$ multiplications, inversions counted and priced at the Fermat cost $I_p$ (12 to 25). The counting copy of the solver matched the repository solver on every call; B lift failures: 0.

#figure(image("ratios.svg", width: 100%), caption: [Measured cost ratios, one point per curve with bootstrap 95% CI over targets (1000 reps), in three pricings; red = Prop. 15.2 law. $m=3$: 9 torsion curves, $p approx 2^9$ to $2^(11)$, 100 to 200 targets. $m=2$: 18 torsion and 12 prime curves, 1000 targets. Source: `m3_*.json`, `m2_*.json`.])

#table(columns: (auto, auto, auto, auto),
  table.header([*ratio*], [*law*], [*measured (range over curves)*], [*CIs containing law*]),
  [B/A size-matched, $m=3$], [3], [2.976 to 3.277 (priced as run 2.969 to 3.267)], [8 of 9],
  [C/A size-matched, $m=3$], [1], [0.980 to 1.075], [8 of 9],
  [B/C, $m=3$], [3], [3.037 to 3.055 (CI width < 0.001)], [0 of 9],
  [(b)/(a), $m=2$], [1/3], [0.3333 on 30 of 30], [all, every pricing],
  [(c)/(b) size-matched, $m=2$], [1], [0.542 to 1.127], [12 of 18],
)

Inversions are 38 to 44 % of the priced cost in every arm, so pricing moves no ratio by more than 0.011 at $m = 3$ or 0.024 at $m = 2$. B/C exceeds 3 by about 1.4 % on every curve: this is the wrapper's own charged work ($phi$ and the lift), which the law does not count.

== Relation streams
Prime family (9 curves): full rank and log recovered on all 9, (a)/(b) relations to full rank 2.79 to 4.97. Torsion family (9 curves): base exhausted before (a) reaches full rank on all 9, (b) reaches full rank on 3 of 9. The round-4 stall is reproduced with the repository oracle.

== Limits
$p <= 2^(11)$ at $m=3$, $<= 2^(16)$ at $m=2$; 3 seeds per size; Fermat inversion price, not a hardware I/M.
