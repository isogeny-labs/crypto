# M4GB persistent server vs one-shot process, 2026-11-11

Stage diagnostic (one decomposition call of the algebraic oracle), not a
leaderboard row: no table row in `benchmarks/README.md`, no VALIDATION.json.
Study, protocol, data and the pre-stated expectation:
[`research/m4gb_persistent_server_20261111/`](../../research/m4gb_persistent_server_20261111/).
Record `benchmarks/records/20261111T1946Z-m4gb-persistent-server.json`
(measured; raw `results.tar.gz` and `results.json`).

`ic descent --solver inherited-f4 --solver m4gb-f2 --solver m4gb-f2-server`
on Koblitz (K) and random binary (R) curves, 12 to 24 unknowns (m = 2) and
12 to 15 (m = 3), 6 seeded targets × 3 repeats, 60 s per call, one thread,
local container `[pcc:unverified]`:

- The persistent `--serve` process (patch 0003) saves a near-constant 4 to
  8 ms per call on the m = 2 cells: ratio server/one-shot 0.36 to 0.50 at 12
  to 14 unknowns, 0.69 to 0.85 at 16 to 20, 0.95 to 0.99 at 24; the server's
  median wall is below the one-shot's on 55 of the 60 targets whose one-shot
  call takes under 50 ms.
- On the cubic m = 3 cells (about 6 s per call) the server is 3 to 5 % slower
  (1.03× and 1.05× at 12 unknowns), so the mode pays only for short calls.
- 0 disagreements with `inherited-f4`, which stays 6 to 28× below both M4GB
  adapters on every m = 2 cell.

Files: `REPORT.pdf`, `results.svg`, `results.png` (copies of the study's).
