# Flat-presentation exponent study at m = 3 and m = 4, with wall time as a co-primary unit (run 2026-11-15 to 2026-11-22)

**Class.** An engine measurement in two units: counted F_p multiplications of the F4 row reduction
(`F4Report.field_ops`, the parent's T) and per-target wall time (W), from `curve25519_family f4sub`
with the flat subgroup presentation (`subflat`: one variable y per summand with y^D = 1). It measures
the cost T(E, m, L) of one symmetrised decomposition system against the base size |V| = D under the
parent's frozen estimator and lever rules
([`../curve25519_exponent_study_20261108/`](../curve25519_exponent_study_20261108/README.md), section 5).
It is not an end-to-end solve, not an ecbench session, and not a leaderboard or `benchmarks/` row. No
pcc claim was registered; every measured number below is an unverified assertion. Rho remains ahead on
every track [pcc:unverified].

**Question (owner, 2026-11-15: "Ok do it").** The exponent studies left the m = 3 and m = 4 verdicts
"undetermined" because the chain presentation censored the series before the estimator could apply.
Wave-6 lead L3 showed the flat presentation 10 to 360x cheaper in counted operations at m = 3. Does the
flat presentation let the preregistered estimator apply at m = 3 and m = 4, what exponent does it give
in each unit, and do the two units agree? Protocol frozen before data: [`PROTOCOL.md`](PROTOCOL.md)
(commit a297c73259).

**Answer.** The m = 3 verdict moves from "undetermined" to **no lever**: the preregistered estimator
applies for the first time at m = 3 (13 complete cells, D = 9 to 57, span 6.3x) and gives
s_T = 4.60 [4.40, 4.79] in the counted unit, far above the lever threshold s < 0, and
s_W = 4.98 [4.39, 5.56] in wall time [pcc:unverified]. The m = 4 verdict stays "undetermined": the flat
presentation completes D = 12 and 14 (the chain did not) but censors at D = 18 on memory, so the D >= 9
cells span only 1.6x; the two-point slopes among them are 11.0 and 14.1 and rising [pcc:unverified].
Five of the six predictions held. P2 failed: the wall-time slope exceeds the counted slope by 0.38, not
by the predicted 0.5 or more; the wall-time excess is not a power law but a bend at the top of the range
(per-operation time is flat at 6 to 8 ns through D = 42 and rises to 32 ns at D = 54) [pcc:unverified].

Distance remaining. This measurement changes no leaderboard row and no cost against rho. On this route
the counted cost of one system grows faster than |V|^4 at m = 3, so the parent's derived balance
2/(m + 1 - s) has no admissible value (s > m + 1): enlarging the base never pays at m = 3 under the
relation-count model, and the algebraic track stays where wave 6 left it, 18x rho-neg at w = 0 at best
[pcc:unverified].

## Cells (k = 20, p = 1,048,573, A = 926; N = 6 targets per cell; binary 5abc70ba)

T = mean `field_ops` per target; W = mean wall ms per target (engine `ms`); SD = highest productive
pair degree; wall s = whole-cell wall time (6 targets, including setup); chain T = the recorded chain
cell at the same (m, D) from the exponent study and its extension (frozen binary dc2c30a4, same seeds).
Full table with loads and container: [`analysis.md`](analysis.md); machine-readable:
[`analysis.json`](analysis.json); figure: [`flat_exponent.svg`](flat_exponent.svg).

| m | D | T | W ms | ns per op | SD | wall s | chain T (recorded) | chain / flat T |
|---|---|---|---|---|---|---|---|---|
| 3 | 9 | 7.589e6 | 74.6 | 9.8 | 10 | 0 | 1.476e8 | 19.5 |
| 3 | 12 | 2.726e7 | 198 | 7.3 | 13 | 1 | 7.748e8 | 28.4 |
| 3 | 14 | 4.686e7 | 347 | 7.4 | 15 | 2 | 7.766e8 | 16.6 |
| 3 | 18 | 1.424e8 | 950 | 6.7 | 19 | 6 | 2.428e10 | 170.5 |
| 3 | 19 | 2.204e8 | 1,331 | 6.0 | 20 | 8 | 2.204e8 | 1.0 |
| 3 | 21 | 2.649e8 | 1,795 | 6.8 | 22 | 11 | 2.788e9 | 10.5 |
| 3 | 27 | 7.266e8 | 5,864 | 8.1 | 28 | 36 | 2.635e11 | 362.6 |
| 3 | 28 | 1.003e9 | 6,966 | 7.0 | 29 | 42 | 2.016e11 | 201.0 |
| 3 | 36 | 4.133e9 | 29,780 | 7.2 | 37 | 180 | censored | |
| 3 | 38 | 6.720e9 | 44,990 | 6.7 | 39 | 272 | not run | |
| 3 | 42 | 8.231e9 | 92,510 | 11.2 | 43 | 558 | not run | |
| 3 | 54 | 2.506e10 | 804,100 | 32.1 | 55 | 4,832 | not run | |
| 3 | 57 | 3.168e10 | 1,393,000 (load-flagged, excluded from the wall fit) | 44.0 | 58 | 8,367 | censored | |
| 3 | 63 | censored: allocation failure (exit -6) after 13,904 s | | | | 13,904 | not run | |
| 4 | 3 | 4.233e5 | 9.15 | 21.6 | 8 | 2 | 4.233e5 | 1.0 |
| 4 | 7 | 2.498e7 | 369 | 14.8 | 8 | 4 | 2.498e7 | 1.0 |
| 4 | 9 | 1.333e8 | 1,432 | 10.8 | 10 | 11 | 1.421e11 | 1,065.8 |
| 4 | 12 | 3.133e9 | 17,830 | 5.7 | 13 | 110 | censored | |
| 4 | 14 | 2.738e10 | 124,100 | 4.5 | 15 | 750 | not run | |
| 4 | 18 | censored: allocation failure (exit -6) after 1,287 s | | | | 1,287 | not run | |

All 6 targets completed in every complete cell, and every system was inconsistent (no decomposition of
these targets), as in the parent: T is the cost of deciding a system, not of finding a relation. At the
prime D = 19 and at the m = 4 boundary cells D = 3, 7 the chain and flat presentations coincide and the
recorded chain T is reproduced exactly (ratio 1.0), which ties this binary and these seeds to the parent's
[pcc:unverified]. The largest complete system (m = 3, D = 57) reached a 6,920,436 x 6,957,419 sparse
matrix.

## Estimator (parent section 5; OLS of ln unit on ln D over complete cells with D >= 9; 95 % t interval)

| fit | n | D range | slope | 95 % interval | 2/(m + 1 - s), derived |
|---|---|---|---|---|---|
| m = 3, T | 13 | 9..57 | 4.598 | [4.402, 4.794] | no admissible value (s > 4) |
| m = 3, W (load-flagged D = 57 excluded) | 12 | 9..54 | 4.977 | [4.391, 5.563] | no admissible value |
| m = 3, T, first container only (D <= 54) | 12 | 9..54 | 4.597 | [4.366, 4.827] | |
| m = 4, T and W | 3 | 9, 12, 14 | not applicable (span 1.6x) | | |

Two-point local slopes. m = 3, T: 4.45, 3.51, 4.42, 8.09, 1.84, 4.01, 8.87, 5.63, 8.99, 2.03, 4.43, 4.33
(D = 9 to 57 in order); the scatter follows the arithmetic of D (the 18 -> 19 and 27 -> 28 steps are
steep, 19 -> 21 and 38 -> 42 flat), as in the parent. m = 3, W: 3.39, 3.64, 4.01, 6.24, 2.99, 4.71, 4.74,
5.78, 7.63, 7.20, 8.60, 10.16: the wall slopes rise monotonically above D = 28 while the counted slopes do
not. m = 4, T: 4.81 (3 -> 7), 6.66, 10.97, 14.06 (12 -> 14); W: 4.36, 5.39, 8.77, 12.58 [pcc:unverified].

Fit residuals at m = 3 (T over fitted T) range from 0.72 (D = 27) to 1.38 (D = 38).

## Solving degree

SD = D + 1 in every complete flat cell with D >= 9, at both m (and 8 at the m = 4 boundary cells D = 3,
7). The chain presentation's SD tracked the largest prime factor of D (wave-5 law); the flat presentation
removes that dependence and pays D + 1 regardless of the factorisation of D, which is why the chain / flat
ratio is 1.0 at prime D and 170 to 363x at the smooth D = 18, 27, 28 [pcc:unverified].

## Controls on the faithful k = 24 member (p = 16,777,141, A = 7,842; m = 3; seeds by the parent rule)

| D | T | SD | W ms | k = 20 fitted T | ratio to fit |
|---|---|---|---|---|---|
| 10 | 1.490e7 | 11 | 111 | 1.053e7 | 1.415 |
| 12 | 2.726e7 | 13 | 215 | 2.435e7 | 1.119 |
| 15 | 7.043e7 | 16 | 477 | 6.795e7 | 1.037 |
| 20 | 2.022e8 | 21 | 1,568 | 2.551e8 | 0.793 |
| 30 | 1.917e9 | 31 | 11,510 | 1.646e9 | 1.165 |

At the shared D = 12 the k = 24 cell gives T = 2.726398e7 against the k = 20 cell's 2.726394e7
(relative difference 1.6e-6; the four-digit test of P5 holds). Every control lies within 1.42x of the
k = 20 law, the same scatter as the k = 20 residuals themselves (0.72 to 1.38x), and the SD values are
D + 1 throughout: the cost depends on (m, D) and not on the prime, as in the parent and L3
[pcc:unverified]. Per-operation wall time on k = 24 (6 to 8 ns) matches k = 20.

## Predictions (written before data; a failed prediction stays failed)

- **P1 held.** The m = 3 estimator applies; s_T = 4.60 [4.40, 4.79] lies in [4.0, 4.8] with lower
  bound above 3. Verdict: no lever at m = 3 [pcc:unverified].
- **P2 failed.** s_W = 4.98 [4.39, 5.56] lies in [4.5, 7], but s_W - s_T = 0.38 < 0.5. The wall-time
  excess is concentrated at the top of the range rather than spread as a power law: ns per counted
  operation is 6.0 to 9.8 for D <= 38, 11.2 at D = 42, 32.1 at D = 54 and 44.0 at D = 57 (second
  container, load-flagged). A log-log line understates a bend of this shape; the honest statement is that
  the two units agree to within 1.2x per operation through D = 42 and diverge by 4 to 6x per operation
  at D >= 54, where the matrix exceeds 5 million rows [pcc:unverified].
- **P3 held.** The m = 3 chain reached D = 57 and censored at D = 63 (<= 84), on memory, not time.
- **P4 held.** m = 4: D = 3, 7, 9 complete; D = 12 completes under 12 GB (the chain cell did not), and
  D = 14 too; the chain censors at D = 18 (<= 28); the estimator is not applicable; both local slopes
  with D >= 9 (10.97, 14.06) exceed 2.
- **P5 held.** Four-digit agreement at D = 12; maximum deviation from the k = 20 law 1.415x (<= 1.5).
- **P6 held.** sha256 5abc70ba... in both launch lines of `run.log` and in all 25 cell records.

## Wall-time regime disclosure

The container restarted between 2026-11-15 07:34Z (D = 54 written) and 2026-11-21 19:23Z (session
resumed); the D = 57 attempt in flight was lost and rerun from scratch with the same seed in a fresh
container (kernel 6.18.44-fc-v726; note in `run.log`). Cells D <= 54 (m = 3), all m = 4 cells and no
control ran in the first container; D = 57, 63 and the k = 24 controls ran in the second. The relaunch
coincided with the D = 57 start, so its 1-minute load before the cell was 1.88 (> 1.5): under the
protocol its wall time is excluded from the wall fit and shown struck in the table. Its counted T is
unaffected (T is deterministic given the seed; the k = 24 D = 12 rerun reproduced the first-container
k = 20 D = 12 value to 1.6e-6). The counted fit with the first container only is 4.597 [4.366, 4.827],
indistinguishable from the full fit. The load averages of the other cells were 0.79 to 1.09, i.e. this
process alone on a shared host whose other tenants are not observable from inside the container.

## Censoring

Both censored cells died on an allocation failure inside `f4_with_arithmetic` (exit -6, `ulimit -v`
12,000,000 kB), not on the 7,200 s per-target cap: m = 4, D = 18 after 1,287 s and m = 3, D = 63 after
13,904 s. The engine writes its record at the end of a cell, so how many of the 6 targets had finished is
not recorded; the D = 63 process ran longer than one cap because the cap is per target. The binding limit
on this route is memory, as in the extension study.

## What changed and what did not

- The m = 3 verdict is settled under the preregistered rule: no lever (s_T >> 0, both units). It was
  "undetermined" since 2026-11-08.
- The m = 4 verdict is not settled; the flat presentation moves the memory wall from D = 12 to D = 18
  and shows accelerating local slopes, consistent with no lever but not an estimator verdict.
- No component speed-up is claimed here [pcc:unverified]: the chain / flat ratios reproduce L3 (which already reported
  them, with a validator report) and the same binary; nothing in this study is new engine work.
- The counted unit and wall time agree per operation through D = 42 and then separate; a wall-time
  co-primary unit is informative only where the matrices fit in cache-friendly sizes.

## Disclosures

No Conductor task, EKS run, S3 checkpoint, benchrec record, VALIDATION.json or pcc claim was reachable
from this runtime (as in every study of this series). The research knowledge base (MCP and CLI) and the
lead-atlas checkout were unreachable in this container; sources checked are the repository files below.
Single host, single process; the host is shared and its other tenants are not observable. The binary is
the wave-6 build 5abc70ba (validated against the frozen dc2c30a4 on control cells in
`curve25519_wave6_runs_20261115`), not the frozen binary itself. Six-day gap in the run.

Sources checked: `research/curve25519_exponent_study_20261108/` (PROTOCOL section 5, cells),
`research/curve25519_exponent_study_ext_20261110/` (cells), `research/curve25519_wave6_runs_20261115/L3_flat/`
(analyse_l3.py, validator report), `research/curve25519_leads_wave5_20261112/README.md` (SD law),
`src/bin/curve25519_family.rs` (`f4sub`, `subflat`).

## Files

- `PROTOCOL.md`: frozen protocol. `run_cells.py`: resumable runner. `run.log`: launch lines, cell
  lines, restart note. `cells/*.json`: 25 cell records with `ext` (cap, memory limit, loads, wall, start
  time, binary sha256, exit code, command).
- `analyse.py` -> `analysis.json`, `analysis.md`. `figure.py` -> `flat_exponent.svg`, `.png`.
- `REPORT.typ`, `REPORT.pdf`. `validator-report.json`: independent validation (see Errata).

## Errata

(Filled from the validator report.)
