# Local m = 4 smoke test (owner decision 17): NOT a registered row

The probe is `m = 4`, `t = 4`, `c = 7`, draw 0, on the session machine (4 vCPU, 15 GB), using the frozen `pkm_tower_pilot`.
It differs from the registered command in two settings: entry cap 1.2e9 instead of 13.96e9, and budget 7,200 s instead of
172,800 s. It enters no reading, gate or median.

Started 2026-11-15 05:34:46Z, and in progress when this was committed. `host.txt` gets the exit code, wall time and peak RSS
when the run ends.
