#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
= Red team: the binary-field $k = 4$ index-calculus crossover

2026-10-26 · audits the study `research/gaudry_k4_char2_crossover_20261015` (branch `claude/k4-char2-crossover-dev`, claim binary built at `e403626cb`) · Haiku/Sonnet workflow, 9 agents · full write-up in `REPORT.md`, working files in the audit folders.

== Verdict: not established (unsupported, not refuted)

The claim puts index calculus below rho near $m = 17$ (subgroup $approx 2^(67)$). No clean IC run exists at $m >= 16$. The only $m = 17$ IC record comes from a run killed by a container restart and is not evidence. At $m = 15$, the last size with clean data on both sides (2 targets each), IC costs about $6.3 times$ rho as recorded and about $5.2 times$ after the corrections that favour IC.

#table(columns: 3, [check], [verdict], [effect on IC ÷ rho],
  [rho reference], [holds: generic curves ($j$ in no proper subfield), $"Aut" = {plus.minus 1}$; IC's `frobenius=1` is a solver-side reduction, not a group automorphism rho could use], [1.0],
  [unit price], [unresolved: binary IC converts by count (159 / 161 $F_q$ multiplications per addition at $m = 15$ / 17); host measurements give 0.3–3.1× depending on method], [0.64 central at $m = 17$; sign undetermined],
  [unpriced work], [rho's canonicalisations are uncharged (every `rho.negation` row is a lower bound); IC fully counted; no full-pricing methods exist on main], [about 0.93 (estimate)],
  [replication], [fails: $m = 16$ has no runs; $m = 17$ has 2 rho and 0 clean IC targets; $n = 2$ gives no interval], [—])

*Where the crossover lands.* Fitted exponents of cost in $q = 2^m$ agree with the prime-field ladder: IC relation collection 1.07 [1.01, 1.13] (prime 1.00), IC linear algebra 2.03 (prime 1.94), rho 2.00 [1.71, 2.29] (prime 1.86). Anchored on $m = 15$, the projected crossover is $m^* approx 17.6$, range 16.3–19.2 depending on pricing: an extrapolation, at or beyond the edge of the measured range.

#figure(image("figures/s-by-m.svg", width: 100%), caption: [Every recorded target of the crossover study: recorded $S = "GAE"\/sqrt(r)$ for rho with the negation map and the two IC arms, by $m$. The hollow circle at $m = 17$ is the killed run, not evidence. Data: `replication/inventory.json`.])

== Next step (frozen plan in `REPORT.md` §3)

8 fresh targets per side at $m = 14$–17, with a new fully priced `rho-strong` (batched inversion, canonicalisations charged), reported in counted field multiplications and in isolated wall time, with decision rules fixed before data. About 30 h of wall time, after the full pricing and `rho-strong` are built; a full replay of the claim records on this host (about 50 min) comes first.
