# Source-pinned explicit-curve 14-bit successor

The historical `explicit14.jsonl` store is immutable because its frozen
executable is unavailable. Use commit `a5503f4bb66ac3aa4eac3125fba01534d2574096`
and retained `ic` SHA-256
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`.
Keep the historical sixteen explicit prime-order EFD curves and target seeds
from `PROTOCOL.md`. Re-run the 16-repeat subtraction preflights for both
`prime-abscissa:below=32` and `prime-isogeny:t=5,seed=5,aux=ordinary`.
Record every command and result in a new append-only JSONL store. Select the
first three curves, in roster order, whose instance/order match and whose
factor bases have equal abscissa counts. Freeze their indices before timing.

For each selected curve, run control/candidate/candidate/control with m=2
Semaev F4 chain degree 8, 1,000 trials, five-second solver-call cap, one rho
context run and a 900-second outer cap. Run one invocation at a time, only
under quiet CPU load. Retain all failures and censoring. Require same target
scalars, verified recovery, and equal base sizes. Report each mirrored
candidate/control ratio for charged GAE, external wall and child CPU, plus
relation counts, trials, F4 work, and median `s_over_rho`. Do not infer a
complete cold-rho crossover from this component panel.

## Load-race amendment, before the first qualifying timed call

At 13:25:08 PST the first curve-6 control invocation began after an empty
load snapshot, but a separate Cargo/Rust build started about 22 seconds later.
The `ic` child and runner were terminated; no timed JSON was emitted. The
append-only store records this as `aborted-load-contamination:6:1`, with a
process snapshot. Future calls check load before **every** invocation and
poll for competing Cargo/Rust/`ic` work during the call. A contaminated call
is terminated, saved as censored, and excluded from qualifying ratios. The
curve-6 mirror restarts from its first timed arm under quiet load.
