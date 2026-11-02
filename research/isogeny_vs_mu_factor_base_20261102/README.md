# Isogeny-kernel vs μ_k factor bases for m = 2 point decomposition (2026-11-02)

This study tests a user's toy sympy experiment. It compares the
Petit-style μ_k factor base (a chain of squarings) with an isogeny-kernel
factor base: V = x(⟨G⟩) ⊂ E′(F_p), presented as a chain of Vélu 2-isogeny
x-maps. It then searches equations, point representations and kernels for
a cheaper presentation.

**Scope.** Component level, m = 2, toy primes (p ≤ 40961). There is no
ECDLP cost claim and no rho row. The distance remaining is stated against
μ_k. Nothing here reaches the IC leaderboard or `benchmarks/`, and no
validation gate applies.

- Protocols, frozen before data:
  - [`PROTOCOL.md`](PROTOCOL.md): Q1 yield, Q2 cost of presentation,
    Q3 terminal, S1 density.
  - [`PROTOCOL_Q4_SEARCH.md`](PROTOCOL_Q4_SEARCH.md): the representation,
    equation and kernel search.
- Code (native Rust, no dependencies):
  - [`harness/`](harness/) for Q1–Q3;
  - [`search/`](search/) for Q4, which copies the harness and adds the
    search;
  - launchers [`run.sh`](run.sh) and [`run_q4.sh`](run_q4.sh).
- Runs: task cards and director assessments are in [`cards/`](cards/),
  including the relay re-check. Raw records are in
  [`results/`](results/).
- Report:
  [`reports/run-20261102T0139Z/REPORT.md`](reports/run-20261102T0139Z/REPORT.md),
  with [PDF](reports/run-20261102T0139Z/REPORT.pdf) and
  [figures](reports/run-20261102T0139Z/figures/). Figures are laid out by
  [`plot.py`](plot.py) from recorded values.
