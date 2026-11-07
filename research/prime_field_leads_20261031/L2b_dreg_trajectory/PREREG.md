# L2b criteria, written before the ladder was run

(Written after a single p = 1009, N = 10 smoke run used only to check the
generator and the msolve log format. No ladder cell had been run.)

Metrics per run, parsed from `msolve -v 2` (F4 rounds table): `maxdeg` (largest
`deg` column), `lastdeg` (`deg` of the final round), `rounds`, `maxrows x maxcols`
(largest matrix), `lastmat` (matrix of the final round), `sumrows` (total rows over
rounds, a counted work proxy), `basis` (size of reduced basis). msolve exposes no
first-fall-degree field; as a stated proxy only, `zdeg` is the smallest `deg` of any
round reporting a nonzero count of zero reductions (or `NA`). It is not a first fall degree.

Per-p summary: median over the runs of an arm that completed.
Trajectory ratio of an arm: median `maxdeg` at the largest p where both arms
completed every run, divided by the same at the smallest p (p = 1021).

- reopens: real-arm ratio < 1.5 while control-arm ratio >= 2.
- null_confirmed: the two ratios agree within 25% relative.
- anything else: reported as observed, decision `writeup`.

Note fixed by construction: both arms contain three univariate membership polynomials of
degree N = round(p^(1/3)), so `maxdeg >= N` in both arms regardless of S_4. A flat
real-arm trajectory is therefore not expected from the degree profile alone.
Stop rule: per-run cap 30 min and 16 GiB (monitored by a polling watchdog because
macOS refuses `ulimit -v`); a shorter cap, if used, is stated in the report.
