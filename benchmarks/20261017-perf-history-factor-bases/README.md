# Perf history: regression surfacing, factor bases, charts (2026-10-17)

Renders of `scripts/perf/perfhistory.py render` over the 75 snapshots on the
`perf-history` branch plus one snapshot recorded locally at `ca00c151` (the
last commit that compiles; `main` at `c6d64436` does not) with the eleven new
`ic_factor_base` children. That local snapshot is shown as a synthetic
`e2e0e2e0` commit; it was taken on a different host from the CI runner, so its
step against the CI history is not a measurement (`field_ec/gf3m_mul_m97`
reads 4.3× more `Ir` here than on the runner for the same commit, everything
else within ±5 % apart from `bool_gb/fes_moebius_random_n22` at +12 %), which
is why the panels ring `field_ec`.

* `factor-bases.svg` / `.png`: solve `Ir` of each frozen index-calculus run by
  factor base, coloured by symmetry class, with the matched rho reference.
* `history-panels.svg` / `.png`: one panel per area, every kernel.
* `history.svg` / `.png`: overall and area indices.
* `report.pdf`: the three PNGs.

Method and rules: `docs/perf/PERFORMANCE_INDEX.md` ("Progress over time").
Host: managed Linux x86_64 container, valgrind callgrind, four cores.
