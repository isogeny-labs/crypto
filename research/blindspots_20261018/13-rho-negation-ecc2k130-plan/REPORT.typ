#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9pt)
#set heading(numbering: none)
= Item 13: negation-map shortfall and ECC2K-130 plan
_Run report and result. Protocol `PROTOCOL.md` (frozen 2026-10-18 before any measured run); cards 01 to 15; instrument `examples/blindspots_13_rho_negation.rs`; read-out `raw/summary.md`. Host: shared 4-vCPU container, isolation L0, one thread per job. All numbers below are measured or derived from the counted operations; derivations are in the README and the protocol._

== Verdict
*TRADES* (level: constant factor) for the negation walk against the un-quotiented Teske walk; *DELIVERED* for the part B plan. Frozen first prediction: INCONCLUSIVE (11, 14, 12 collision-preserving cells reach 1.35 at 2^40, 2^44, 2^48 under free native work, none robustly). Second prediction: a sign change exists in the tested range (12 settings are clear net losses at all three sizes). Per-setting: 0 ADVANCES, 30 TRADES, 2 MATCHES, 12 REGRESSES. Every gate passed: baseline against the ideal-map control (0.948 [0.882, 1.019], 1.023 [0.943, 1.104], 1.008 [0.932, 1.087]), ideal-map control sees sqrt 2 (1.490, 1.412, 1.409), no-escape absorbed at r = 16, A/A covers 1 in 12 of 12 cells.

== Gain over the plain walk
#figure(image("graph.svg", width: 100%), caption: [Fully charged gain (Jacobian model, native work 0), 64 paired targets per cell, 95% paired percentile bootstrap (4,000 resamples). Hollow markers: at least one target unsolved at the cap, gain is an upper bound. Small triangles: worst of the nine pricing points. Source: `raw/summary.json`, drawn by `plot_graph.py`. Item 13 sweeps, seed offsets 7000000 and 9000011.])

*Distance remaining.* Gain 1.0 is rho (the plain walk); the target is sqrt 2 = 1.414. Best collision-preserving cell per size, native work free: 1.557 [1.325, 1.825] (2^40, `laper16`, r = 128), 1.488 [1.254, 1.779] (2^44, same), 1.552 [1.277, 1.895] (2^48, `laper16`, r = 32), i.e. 1.10, 1.05, 1.10 of sqrt 2 with intervals that include 1.35 and sqrt 2. At the harshest pricing point (native operations at one group addition each, Jacobian) every collision-preserving cell is below 1.0 (nine-point minimum 0.21 to 0.81). The order is unresolved across the section 5a band, which is what TRADES records.

== Every collision-preserving setting
Point gain and 95% interval, fully charged JAC30, native work 0; `+` marks a censored cell (upper bound). The nine-point range is the minimum and maximum point gain over JAC30, AFF3, AFF30 times native work 0, 0.1, 1 over the three sizes. `best/sqrt2` is the largest of the three point gains divided by 1.414.
#set text(size: 7pt)
#include "tables.typ"
#set text(size: 9pt)

== Controls, extension, caveats
Selftest (card 01): two-implementation agreement at 2^20, positive controls, no wrong logarithm. Fruitless 2-cycle entry rate per draw against 1/(2r): 48 settings, ratio 0.948 to 1.039 (HEUR-RHO-1 not refuted). Size dependence: 4 of 44 collision-preserving pairs have disjoint intervals at 2^40 and 2^48 (`la`, `min2`, `per2` at r = 128; `per4` at r = 16). Theta extension at 2^44, r = 128 (outside the frozen rule): `la` 1.344 / 0.810 / 0.443 at DP bits 8 / 11 / 14, `laper16` 1.474 / 1.488 / 1.179, so the size dependence is a DP-length dependence. Caveats: toy scale 2^40 to 2^48 in a counted `Z/N` model (curve agreement only at 2^20); not an ECC2K-130 or NIST measurement; the 1 GAE point is a harsh bound; no exponent is touched. Relay check: 9 of 15 executor reports verbatim, 6 deviated in format (card 12 alters one hex digit of the card sha256); receipts all completed and 33 output digests match disk (`raw/relay-check.txt`).

== Part B: costed ECC2K-130 plan
Measured: current `ecc2k130/` client, `--curve 131 --bench --threads 1`, five runs 14.702, 15.189, 14.532, 15.206, 15.605 M iterations/s (median 15.189, min 14.532, max 15.605; load average 0.83 before, 1.45 after; L0). Cited, not re-measured: GPU rates 15.884369 B/s (RTX PRO 6000 sigma-table) and 12.499203 B/s (RTX PRO 4500 x-only bridge, work factor 1.006696), 2^60.9 iterations, DP intervals 2^28.41 and 2^25.27, 2026-09 prices.
#table(columns: 2, stroke: 0.4pt + gray, inset: 3pt,
  [*quantity*], [*value*],
  [expected work], [2.1514e18 iterations],
  [CPU alone], [3.935e7 core-hours = 4,488 core-years],
  [DPs and storage at 32 B (w32 / w34)], [6.03e9, 193 GB / 5.32e10, 1.70 TB],
  [RTX PRO 6000 sigma-table], [37,623 GPU-h; \$45,629 at \$1.2128/h; \$155,760 at \$4.14/h on-demand],
  [RTX PRO 4500 x-only (other iteration function)], [48,133 GPU-h; \$27,460 at \$0.5705/h],
  [50% / 90% / 99% (Rayleigh model)], [35,345 / 64,419 / 91,103 GPU-h],
  [wall clock for 100 / 1,000 / 10,000 GPUs], [15.7 / 1.6 / 0.2 days],
  [break-even CPU core-hour price], [\$0.00116],
)
No live price, no GPU rate measured here, no campaign progress credit, no network or orchestration cost. Sources: `raw/plan.md`.
