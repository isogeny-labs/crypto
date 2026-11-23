# Chained-x post-main diagnostic

Status: matched one-curve L0 diagnostic; not an ECBench receipt and not a
leaderboard result.

This panel followed the `chained-x` registration merged into `ic bench` on
2026-11-01. It held the Koblitz `a = 0`, `n = 13` instance, dimension-6 binary
subspace factor base, walk target source, seed, trial cap, four repeats, and
four matched rho runs fixed while varying `m = 2, 3, 4` and the three native
engines.

Inherited F4 is the only competitive chained-x engine in this toy panel, but
it remains 24.71x, 144.09x, and 794.37x matched rho at `m = 2, 3, 4`.
Matrix F5 reduces matrix F4's mean complete `S` by 37.2% at `m = 3`, yet is
still 16,296.76x rho. Both `m = 4` matrix engines failed to produce a report
under a five-minute external watchdog even after reducing `node_budget` from
4,096 to 256.

The CLI's `--solver-budget-seconds` is wired to `descent-algebraic`, not to
`chained-x`; the first default matrix-F4 attempt was therefore manually
censored after 18 minutes. This is a sweep-safety bottleneck. Raw JSON reports
remain local and ignored in `raw/`; [`SUMMARY.json`](SUMMARY.json) binds their
hashes and retains every completed and censored cell.
