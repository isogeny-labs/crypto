# RESULT: LEAD-ICP-icperf-binary-pdp-solver

## Outcome
- Status: blocked (kind == blocked, no run.py, empty run_cmd). Nothing executed.
- Wall seconds: 0. Exit code: n/a.
- See BLOCKED.md for what is missing and the smallest unblocking action.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): "R < 1 in a majority of cells, and d_solv parseable in all cells that fit in RSS." Falsified if R >= 1 in a majority of cells or d_solv is unparseable/OOM in most cells. Inconclusive if the order flips inside the 0/0.1/1 GAE band.
- Measured: none. R, d_solv and peak RSS were not computed.
- Verdict: not_run.
- Controls: null (random Boolean systems), positive (planted key), A/A (two hosts) were not run. Controls passed: no (not applicable, none run).

## Reading
- The toy does not exist, so the lead's claim was not tested. PROTOCOL.md itself says a stdlib toy would be unfaithful to a solver-engineering comparison at the boundary.
- No discrepancy between a verdict field and numbers, since there is no run.py output.
- The lead needs BRiAl, WDSat, and a large-memory host (ANF beyond 2^31 monomials).

## Bearing on the sub-sqrt(p) question
none. Nothing was measured. The lead is binary-field only. Even a win would only rank solvers and says nothing about beating rho.

## Standing
[pcc:unverified], no run, no independent replay.
