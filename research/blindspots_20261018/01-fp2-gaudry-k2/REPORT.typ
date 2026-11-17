#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: none)
#show table: set text(size: 8pt)

= Item 01: Gaudry k = 2 index calculus on E over $F_(p^2)$ against matched rho

Assessed 2026-10-18 from `raw/summary.json` (blindspot01.v1, 945 rows from 32 files, card 11) under the frozen rule in `PROTOCOL.md`. Instrument `examples/blindspots_01_fp2_gaudry.rs` (sha256 0629a73c...), cards CARD-BLINDSPOT-01-01 to 11, host L0 (shared 4-vCPU container, two lanes at once).

== Verdict

*REGRESSES at the constant level; INCONCLUSIVE at the exponent level.* No arm is cheaper than its matched rho at any size from log2 \#E = 24 to 40, at 0, 0.1 or 1 GAE per unpriced native operation; the order holds across the whole band in every cell. At 40 bits the best arms (ic-b, ic-c2 on the 2-torsion family) cost 50.0 (95% CI 42.0 to 60.3) and 51.6 (43.3 to 62.0) times matched rho-neg at w = 0, and 22.2 (19.7 to 25.5) and 14.1 (12.0 to 16.9) times at w = 1. The exponent is decided only for plain torsion ic-a (REGRESSES); for every other arm the all-size fit is above rho's but the three-largest-size fit does not reproduce it. Extrapolation to FourQ (log2 r = 246) gives a log2 gap of 16.7 to 17.4 (w = 0) and 6.9 to 11.6 (w = 1) from the all-size fit, 4.7 to 7.0 from the top-three fit with intervals reaching parity: distance remaining, not a forecast. Toy scale (log2 r 21 to 40, 3 to 6 curves a size). No improvement is claimed.

#figure(image("gap_vs_n.svg", width: 100%), caption: [Left: mean S = cost / sqrt(r) in GAE (w = 0), measured, per arm. Right: gap to matched rho per size, w = 0 (solid, 95% cluster-bootstrap band over curves) and w = 1 (dotted). Source: raw/summary.json, 945 rows; plot_gap.py.])

== Gap to matched rho at 40 bits (measured; ratio of means, 95% CI)

#table(columns: 4, align: (left, left, left, left),
[Arm / matched rho], [w = 0], [w = 1], [Exponent label (frozen rule)],
[torsion ic-a / rho-neg], [384 (350 to 430)], [116 (106 to 130)], [REGRESSES],
[torsion ic-b / rho-neg], [50.0 (42.0 to 60.3)], [22.2 (19.7 to 25.5)], [INCONCLUSIVE],
[torsion ic-c1 / rho-neg], [141 (120 to 168)], [38.1 (32.8 to 45.1)], [INCONCLUSIVE (boundary)],
[torsion ic-c2 / rho-neg], [51.6 (43.3 to 62.0)], [14.1 (12.0 to 16.9)], [INCONCLUSIVE],
[prime ic-a / rho-neg], [187 (155 to 232)], [57.1 (46.8 to 70.5)], [INCONCLUSIVE],
[GLS ic-gls-neg / rho-neg], [196 (145 to 406)], [58.3 (43.2 to 121)], [INCONCLUSIVE],
[GLS ic-gls-psi / rho-neg], [104 (77.8 to 213)], [32.9 (23.9 to 68.3)], [INCONCLUSIVE],
[GLS ic-gls-psi / rho-psi], [167 (119 to 486)], [42.2 (29.3 to 125)], [INCONCLUSIVE],
)

== Fitted exponents (all-size, 95% CI; top-three in brackets)

#table(columns: 4, align: (left, left, left, left),
[Arm], [alpha, w = 0], [alpha, w = 1], [matched rho alpha, w = 0 / w = 1],
[torsion ic-a], [0.536 (0.531 to 0.540) [0.536]], [0.526 [0.530]], [0.456 (0.444 to 0.466) [0.469] / 0.473 [0.473]],
[torsion ic-b], [0.507 (0.494 to 0.521) [0.473]], [0.484 [0.476]], [same],
[torsion ic-c1], [0.556 (0.543 to 0.569) [0.529]], [0.546 [0.527]], [same],
[torsion ic-c2], [0.510 (0.497 to 0.524) [0.474]], [0.509 [0.476]], [same],
[prime ic-a], [0.528 (0.522 to 0.534) [0.537]], [0.506 [0.529]], [0.468 [0.477] / 0.479 [0.482]],
[GLS ic-gls-neg, ic-gls-psi], [0.572, 0.567 [0.495, 0.488]], [0.556, 0.552], [rho-neg 0.488 [0.506]; rho-psi 0.457 [0.445]],
)

== Controls

Positive: 330 of 330 index-calculus rows and 936 of 936 solve rows recovered the planted d. Oracle cross-check: 101,415 decisions, 0 disagreements. Negative (shuffled base): full solves at 24 and 28 bits, rate within 0.03 of ic-a in 6 of 6 cells and S\_control / S\_a from 11.0 to 52.7 (at least 10 required); rate-only at 32, 36, 40 bits within 0.08 in 8 of 9 cells, the ninth (40 bits, curve 2) at 0.084, about 2.5 binomial standard errors, a strict miss of the frozen criterion, recorded as such.

== Deviations and caveats

- Cards 3, 6 and 8 did not finish in their first runs (executor foreground timeout, no receipts); the unreceipted partial outputs were moved unchanged to `raw-aborted/` and the three cards were re-run unmodified at the user's request (same card and instrument sha256), then card 11 (its first 887-row summary was premature) was re-run: 945 rows, 32 files. The re-run reproduced all 122 overlapping aborted rows exactly in every counted field.
- `check-relay` cannot confirm relays after the fact (it re-lints the card, which fails once outputs exist, for every card). Receipts were checked by hand: all `completed`, quoted receipt hashes match; relay files for cards 1, 2, 4, 5, 7, 9, 10 hold only the header lines rebuilt from the receipts.
- The instrument is a std-only port (the library does not compile at the branch base); the library filter's pass count is a lower bound. Extrapolation to 246 bits is a straight-line extension over a six-fold range. The within-instrument 7.7x (w = 0) lowering of ic-a's S by the 2-torsion fold is a lever comparison, not an improvement over the repository baseline.
- Not done: independent runner replay, isolab, msolve or library comparison arms.
