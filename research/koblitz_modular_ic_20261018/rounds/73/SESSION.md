# R73 local execution record

Two native attempts and three Lean attempts are terminal. Native
attempt 01 and Lean attempts 01–02 are rejected; native attempt 02 and
Lean attempt 03 are accepted. Each attempt retains its copied source,
protocol, driver, exact revision and hash, exit status and full log.
Native math used one local Singular/FLINT thread, seed 73, 1200-second
timeout and 4 GiB virtual limit. The final Lean run used pinned Lean
4.34.1/mathlib `d13f23b`, CPU0, `-j1 -M4096`, separate compile/audit
limits and no virtual-memory cap. No two math jobs in this lane ran
simultaneously. See `results.json` for exact timestamps and failures.
