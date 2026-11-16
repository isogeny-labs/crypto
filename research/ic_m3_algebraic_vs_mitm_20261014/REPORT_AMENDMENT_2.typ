#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show table: set text(size: 8.5pt)

= The narrow Gröbner and SAT engines past 64 unknowns, $m = 4$

*Stage diagnostic* · study `research/ic_m3_algebraic_vs_mitm_20261014/` · frozen in `AMENDMENT_2.md` before the grid · build `cargo build --release --features wide-vars --example ic_algebraic_floor` · data `runs/m4-engines/` · analysis `amendment-2/ANALYSIS.json` (2,000 bootstrap replicates, seed 20261014).

== What changed in the code

`pq_groebner_f2::Mask` is `u64` by default and `u128` with the Cargo feature `wide-vars`. F4, inherited F4, F5/F6-IC, the Koblitz Gröbner driver, the SAT encoder and the solution plumbing run on it, and `koblitz_groebner::MAX_VARS` is 128 in that build. The default build is unchanged: its lib suite passes (4438 passed, 0 failed). The wide build passes the same suite after three assertions that test the 64-unknown cap itself were limited to the default build. New tests exercise more than 64 unknowns: a 100-unknown triangular system with its planted root, key and sort and sum checks across bit 64, and an end-to-end $m = 4$, $n = 23$, $d = 6$ (70 unknowns) decomposition through both F4 frontends, checked in the group.

== Result

In the original run these arms were `unsupported` at $n = 23, 29, 31$. They now run on all seven $m = 4$ cells.

- *F4 and F6-IC finish $n = 23$ and $n = 29$* (70 and 90 unknowns), all 12 targets each, no censoring. At $n = 31$ (94 unknowns) all 6 non-decomposable targets are censored at 120 s, so that cell is a lower bound. My pre-data prediction that $n = 29$ would also be censored was wrong for these two arms.
- *Both grow faster than the floor.* Slope on the six measured cells ($n = 11$–$29$): 0.644 (F4) and 0.645 (F6-IC) against 0.298 for the floor on the same cells. Δ = arm − floor, 95% interval [0.313, 0.372] and [0.316, 0.371], entirely above 0.
- *SAT*: undetermined (fewer than 3 usable cells). It censors from $n = 13$ on and the cells above $n = 19$ were not run, because the early-stop rule ended the arm at $n = 19$.
- *Exactness:* every finished verdict agreed with the floor. The floor is exhaustive only for $n <= 23$; at $n = 29, 31$ the check is against the pair-table floor, not exhaustive enumeration.

#figure(image("amendment-2/figures/cost_per_relation_m4.svg", width: 92%),
  caption: [$m = 4$, wide build: $log_2$ wall ns per relation against $n$, 95% bootstrap bars. Open markers with arrows are lower bounds. Faded markers: the original narrow-build run. Source: `amendment-2/cells.csv`.])

#table(columns: 6, align: right,
  [*n*], [*unknowns*], [*floor*], [*f4-chain*], [*f6-ic*], [*sat*],
  [11], [34], [18.75], [29.09], [28.87], [32.03],
  [13], [42], [15.63], [26.11], [26.33], [≥ 35.65],
  [17], [54], [17.26], [30.62], [30.82], [≥ 39.19],
  [19], [58], [19.05], [32.52], [32.57], [≥ 40.24],
  [23], [70], [21.42], [36.56], [36.59], [not run],
  [29], [90], [22.00], [38.13], [38.14], [not run],
  [31], [94], [24.45], [≥ 40.48], [≥ 40.48], [not run],
)

== Caveats

- At $n <= 19$ the wide build returns the same verdicts on the same targets as the narrow build. Its counted word XORs per target are slightly higher (for example 2.02e6 against 1.97e6 on one $n = 13$ target), so counted units are not comparable across the two builds. Wall-clock costs differ by up to a factor of 1.3 at $n <= 19$, and the floor was re-timed in this run.
- The slope verdict covers $n = 11$–$29$ only; $n = 31$ is a lower bound and SAT is undetermined. No claim is made beyond that range.
- None of this is an improvement over the index-calculus baseline; the floor remains $10^3$–$10^5$ times cheaper per relation on these cells.
