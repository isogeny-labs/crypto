#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
#show table: set text(size: 8pt)

= Certicom 131-bit index-calculus program: gates and two falsifiers

*Study* `research/certicom131_ic_program_20261020/`. *Program text* `docs/ic/CERTICOM131_IC_PROGRAM.md`.
*Class:* measurement / accounting. *Date:* 2026-10-20. Public synthetic instances only; the Certicom
challenge targets are named to fix parameters and the rho reference, and nothing is run against them.
No sub-rho claim, no key-recovery claim, no statement about a deployed curve.

== Summary (measured / derived / extrapolated, labelled)

- *Derived (cost model, `gates.md`).* Matched rho: ECC2-131 $2^(64.83)$, ECCp-131 $2^(64.92)$ GAE (negation map).
  Binary FPPR with $m <= 3$ cannot reach rho at $n = 131$ even with a free decomposition oracle (floor
  $2^(65.2)$ to $2^(67.2)$). The first usable arity is $m = 4$ with a solving-degree gate $D^* <= 3$ (generous price)
  or $<= 1$ (plain); $m = 5, 6$ allow $D^*$ up to 4 to 5 only at the generous price and favourable width.
  PKM tower on ECCp-131: $D^* <= 3$ at $m = 4, 5$. Small-abscissa base: needs $delta >= 0.169$ at $m = 4$ against a
  lattice reach of $0.0125$ (13.5x shortfall).
- *Measured, Falsifier A.* Plain-Macaulay resolving degree at $m = 3$ on random binary curves is
  $D = 4, 5, 6$ for $ell = 1, 2, 3$ (60 draws, zero spread), equal to the semi-regular reference in every cell,
  and equal to the Koblitz $b = 1$ value on the same $(V, x_R)$ in 52 of 52 paired draws. First fall degree stays
  2 to 3. No scaling deviation from a random $b$.
- *Measured, Falsifier B.* The combined-system count of structured prime-field bases (Kummer $mu_(2^t)$, coset,
  subgroup, interval) is within 3% of the random-oracle expectation and of a random subset: null.
- *Measured, ecbench.* New 30-bit prime-kummer row: IC is 159x matched rho at $w = 0$ (121.6 to 221.4), 175x at
  $w = 0.1$, 247x at $w = 1$, up from 140.6x at 28 bits. Audit: 160 records, 160 verified, 120 replays reproduced.
- *Extrapolated, not measured.* $D$ at $n = 131$, $m = 3$ between about 6 and 7 (plateau) and 39 (semi-regular);
  the PKM gap near $log_2 r = 130$ of order $10^7$ if the $r^(1\/6)$ law holds.

#figure(image("figures/fig_gates.svg", width: 92%),
  caption: [Gates from `certicom131_ic_gates` (`gates.json`): largest solving degree $D$ at which the family's
  optimised cost reaches rho, at the plain (filled) and generous (open) price points.])

== Falsifier A: resolving degree on random binary curves

#figure(image("figures/fig_dreg_vs_ell.svg", width: 92%),
  caption: [Run `certicom131_fppr_dreg`, seed 20261020, 60 unsatisfiable draws (each verified unsatisfiable by exact
  count). Records `runs/c*.jsonl`.])

#table(columns: 9,
  [*n*], [*ℓ*], [*S*], [*unknowns*], [*draws*], [*random-b D*], [*b=1 D (same V, x_R)*], [*FFD*], [*semi-reg.*],
  [5], [1], [2], [8], [24], [4 ×24], [4 ×20 (4 sat)], [2–3], [4],
  [7], [2], [1], [13], [24], [5 ×24], [5 ×23 (1 sat)], [2–3], [5],
  [11], [2], [5], [17], [8], [5 ×8], [5 ×8], [3], [5],
  [11], [3], [2], [20], [4], [6 ×4], [6 ×1 (3 unpaired)], [3], [6],
)

*Not run:* cells (13,3), (13,4), (17,5), (19,6) were launched and produced no draw in about 3 h of wall time at host
load 90 to 170. The $n <= 63$ cap of the instrument is reachable for the exact count, not for the Macaulay
measurement (a draw costs about $10^3$ x more from $ell = 2$ to $ell = 3$). Wall times are a practicality note only.

== Falsifier B: combined-system solution count

#figure(image("figures/fig_combined_count.svg", width: 92%),
  caption: [Run `certicom131_prime_combined_count`, seed 20261020, four curves of the PKM ladder (20 to 28 bit),
  eight on-curve targets, $m = 3$, $B = 2^6$ to $2^11$; cells with expected count $>= 20$ per target pooled.
  Records `runs/fb_*.jsonl`.])

#table(columns: 6,
  [*base*], [*cells*], [*Σ N₃*], [*Σ E₃*], [*N/E*], [*95% interval*],
  [coset $g mu$], [8], [11,324], [11,322.7], [1.000], [0.968, 1.032],
  [subgroup, non-2-power order], [7], [16,348], [16,470.8], [0.993], [0.966, 1.019],
  [interval [1,B]], [8], [11,591], [11,901.4], [0.974], [0.943, 1.005],
  [random subset (control)], [7], [10,445], [10,726.1], [0.974], [0.941, 1.006],
  [Kummer $mu_(2^t)$], [7], [11,107], [11,461.6], [0.969], [0.938, 1.000],
)

The note's dead criterion $N >= B^(m-1)$ is never met: $N_3 <= 560$ per target against $B^2 >= 2.5 dot 10^5$.

== ecbench: prime-kummer past 28 bits

Session `ECBS1hfb8f08d66f01`, spec `ECS1h5e999ae301b7`, curve `icv1-fp30-t49927-d70f5de2`
($p = 1000574977$, order 1000525051 (prime), $log_2 r = 29.90$), Kummer $t = 12$ (2,092 columns), 8 targets x 3 rounds,
L0 (operation counts only, host loaded). Mean $S$ at $w$ = 0, 0.1, 1 (in that order): rho-neg 0.921 / 1.012 / 1.830; ic-rootfind
146.4 / 177.0 / 452.2; ic-mitm 144.7 / 175.6 / 454.2; ic-abscissa control 146.6 / 179.7 / 477.5. Tower base
equals plain base within 1.2%. The 32-bit row was not run (curve generator's exhaustive point count unfinished).

== Limits

Toy sizes ($n <= 11$, $ell <= 3$; $p <= 2^30$); extrapolation to 131 bits is a factor 6 to 10 outside the measured
$ell$ range and is labelled as such. The model's constants are assumptions with two bracketing price points.
Host: shared macOS machine at load 90 to 170, no isolation (L0); wall time is not a metric here.
