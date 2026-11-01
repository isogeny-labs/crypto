#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9pt)
#set par(justify: true)
#show heading: set text(size: 11pt)
#show raw: set text(size: 8pt)

= R7.1: complete-cost ordering of kernel-finding routes
Round 7, 2026-10-31. Code `examples/r7_1_complete_cost.rs` and the library change in `src/cryptanalysis/isogeny_eval/` (native Rust). Raw records: `ladder_grid_*.json`, `ladder_extra_a.json`, `project_*.json`, `psi_*.json`, `modpoly_*.json`, `selftest.json`; fits and tables: `analysis.json`, `analysis.txt`; full write-up: `RESULTS.md`. Toy, synthetic instances only ($j = -3375$, $D = -7$, 61-bit $p$, one instance per $ell$); counted unit, not time; no sub-rho claim and no claim about a deployed curve.

== Question, hypothesis, falsifier (from the task)
R5.3 and R6.4 priced rows as $M + W + 91 I$ only. *H*: the ordering of the routes (ladder/NTT, $psi_ell$ + factoring, $Phi_ell (j, Y)$) and the crossover with rho at $r = 2^128$ survive when additions and $Phi_ell$ construction are counted, with an addition priced at $a in {0, 0.1, 0.3, 1}$ M. *Falsifier*: the ordering of the cheapest two routes changes at some $a$ for some measured $ell$, or the exponent CI moves its lower endpoint below 1.5 or its upper above 2.3.

== Verdict
*The ordering part of the falsifier is triggered; H is not confirmed as stated. The exponent part is not triggered.*
- Ordering. Cold accounting ($Phi_ell$ construction charged), ascending edges: the second place between $psi$ and $Phi$ swaps with $a$ at $ell = 89, 109, 131$ (3 of 22; the two come within 1.6%, 0.3%, 0.4% of each other over the weights, one instance per $ell$: a near tie). The ladder is cheapest at every $ell$ and $a$ in the cold accounting. Amortised ($Phi_ell$ given), both directions, and cold descending: no change of the cheapest two or of the full order with $a$ at any measured $ell$ (order: amortised $Phi$ < ladder < $psi$; cold at $a = 0$: ladder < $Phi$ < $psi$).
- Exponent. Projected generic ladder ($ell >= 2^13$, `PolyMul::Auto`): $1.919$ at every $a$; 95% OLS interval at worst [1.717, 2.121], bootstrap at worst [1.781, 2.126]; inside [1.5, 2.3]. At $a = 0$ this is R6.4's $1.919$ [1.724, 2.115]: still insufficient to show an exponent below 2.
- Crossover (supplementary, extrapolated; $r = 2^128$, $g = 6$): ladder $log_2 ell^*$ shifts by $-0.24$ to $-0.39$ bit from $a = 0$ to $a = 1$; $psi$ $-0.37$; $Phi$ cold $-0.01$; $Phi$ amortised $-1.48$ (fit over $ell <= 103$ extrapolated to $2^30$). The rank order of the routes by $ell^*$ does not change with $a$.

== Method
Library: `PrimeField::counted_with_adds` tallies additions in `AddTally` (opt-in; $M$, $W$, $I$, `OpTally` and all frozen outputs unchanged). One addition = one modular add/sub/neg: every `add`/`sub`/`neg` call; schoolbook accumulations $l_a l_b - (l_a + l_b - 1)$; per NTT product $3 T N log_2 N + 3n$ + data-dependent conditional subtractions modulo NTT primes, and $2n$ `u128` accumulations in $F_p$. Cost $= M + W + 91 I + a (A + W A)$ with $W = 1 M$ as before. Route (c) now includes $Phi_ell$ construction (counted port of the library's q-series builder; equal to the library's table for all 23 $ell <= 103$; fast and per-operation ports agree in table and tally) and the Elkies kernel polynomial (counted port; equal to the library's for all 28 $ell$, both directions); reported cold (construction charged per edge) and amortised ($Phi_ell$ given). Sutherland's CRT evaluation is *not implemented*. Arms (a) ladder over $F_(p^k)$, (a') per-step projection to $ell = 2^16$, (b) $psi_ell$ + Cantor-Zassenhaus / gcd, on R6.4's instances. Frozen counts are unchanged: the unmodified round-6 example rebuilt on the new library reproduces its records (9/9, 2/2, 5/5 rows), and this example's $a = 0$ columns reproduce R6.4's counts (ladder 20/20, $psi$ 9/9, $Phi$ 23/23, projection 119/119).

== Exponents per route and weight (measured; (a') projected)
$log_2 "cost" = log_2 c + e log_2 ell$; intervals 95%: OLS t-interval and pairs bootstrap (2000 resamples).
#set text(size: 7pt)
#table(columns: 5, align: (left, left, left, left, left), inset: 3pt, stroke: 0.4pt,
  [series], [$a = 0$], [$a = 0.1$], [$a = 0.3$], [$a = 1$],
  [(a') projected ladder, Auto, $ell >= 2^13$], [1.919 \ OLS [1.724, 2.115] \ boot [1.795, 2.113]], [1.919 \ OLS [1.722, 2.116] \ boot [1.781, 2.113]], [1.919 \ OLS [1.720, 2.118] \ boot [1.790, 2.126]], [1.919 \ OLS [1.717, 2.121] \ boot [1.783, 2.125]],
  [(a') projected ladder, Auto, $ell >= 2^10$], [2.088 \ OLS [2.020, 2.155] \ boot [2.023, 2.158]], [2.092 \ OLS [2.024, 2.160] \ boot [2.029, 2.163]], [2.097 \ OLS [2.028, 2.167] \ boot [2.029, 2.169]], [2.105 \ OLS [2.034, 2.177] \ boot [2.035, 2.179]],
  [(a) ladder measured, desc, $ell >= 256$], [2.554 \ OLS [2.352, 2.756] \ boot [2.376, 2.713]], [2.566 \ OLS [2.380, 2.753] \ boot [2.399, 2.710]], [2.584 \ OLS [2.420, 2.748] \ boot [2.434, 2.715]], [2.612 \ OLS [2.482, 2.742] \ boot [2.498, 2.720]],
  [(b) $psi$ + CZ, desc, 37..131], [3.565 \ OLS [3.302, 3.828] \ boot [3.316, 3.792]], [3.588 \ OLS [3.331, 3.845] \ boot [3.346, 3.819]], [3.618 \ OLS [3.370, 3.867] \ boot [3.384, 3.833]], [3.665 \ OLS [3.428, 3.903] \ boot [3.454, 3.856]],
  [(b) $psi$ + gcd, asc, 37..131], [3.439 \ OLS [3.087, 3.791] \ boot [3.187, 3.724]], [3.456 \ OLS [3.106, 3.806] \ boot [3.222, 3.738]], [3.478 \ OLS [3.130, 3.826] \ boot [3.228, 3.743]], [3.512 \ OLS [3.166, 3.859] \ boot [3.270, 3.764]],
  [(c) $Phi$ amortised, desc, 11..103], [1.860 \ OLS [1.785, 1.936] \ boot [1.779, 1.932]], [1.882 \ OLS [1.808, 1.956] \ boot [1.804, 1.959]], [1.914 \ OLS [1.842, 1.987] \ boot [1.838, 1.988]], [1.976 \ OLS [1.906, 2.045] \ boot [1.900, 2.043]],
  [(c) $Phi$ cold, desc, 11..103], [4.638 \ OLS [4.581, 4.696] \ boot [4.574, 4.751]], [4.639 \ OLS [4.582, 4.696] \ boot [4.577, 4.750]], [4.640 \ OLS [4.583, 4.697] \ boot [4.572, 4.753]], [4.642 \ OLS [4.586, 4.698] \ boot [4.576, 4.745]],
  [(c) $Phi$ construction alone, 11..103], [4.685 \ OLS [4.641, 4.729] \ boot [4.632, 4.768]], [4.685 \ OLS [4.641, 4.729] \ boot [4.635, 4.769]], [4.685 \ OLS [4.641, 4.729] \ boot [4.636, 4.771]], [4.685 \ OLS [4.641, 4.729] \ boot [4.634, 4.768]],
)
#set text(size: 9pt)

#figure(image("fig2_exponents_by_weight.svg", width: 100%), caption: [Fitted exponents by addition weight with OLS and bootstrap 95% intervals (data: `analysis.json`, `fits`; $n$ as in the table).])

== Ordering of routes
L ladder, P $psi$ + factoring, F $Phi$ amortised, C $Phi$ cold. Counts over all measured $ell$ with the three routes of the accounting measured (`analysis.json`, `ordering_summary`).
#table(columns: 5, inset: 3pt, stroke: 0.4pt,
  [direction], [accounting], [$ell$ tested], [cheapest-two changes with $a$], [smallest adjacent cost ratio],
  [descending], [amortised], [25], [0], [4.670],
  [descending], [cold], [25], [0], [1.605],
  [ascending], [amortised], [22], [0], [11.635],
  [ascending], [cold], [22], [3 (at $ell$ = 89, 109, 131)], [1.003],
)

#figure(image("fig1_costs_by_route.svg", width: 100%), caption: [Measured cost of the four route accountings, descending edges, by addition weight (`analysis.json`, `ordering`; one instance per $ell$, counted unit).])

Selected rows of the ordering table (cost in $M$-equivalents, $M + W + 91 I + a (A + W A)$; full table in `RESULTS.md` and `analysis.json`):
#set text(size: 6.5pt)
#table(columns: 9, inset: 2.5pt, stroke: 0.4pt, align: (right, left, right, right, right, right, left, left, left),
  [$ell$], [dir], [$a$], [L], [P], [F], [C], [order amortised], [order cold],
  [37], [desc], [0.0], [8.13e+06], [6.37e+08], [4.88e+05], [6.13e+07], [F < L < P], [L < C < P],
  [37], [desc], [0.1], [8.93e+06], [7.27e+08], [5.40e+05], [6.74e+07], [F < L < P], [L < C < P],
  [37], [desc], [0.3], [1.05e+07], [9.08e+08], [6.46e+05], [7.97e+07], [F < L < P], [L < C < P],
  [37], [desc], [1.0], [1.61e+07], [1.54e+09], [1.02e+06], [1.23e+08], [F < L < P], [L < C < P],
  [37], [asc], [0.0], [8.14e+06], [3.39e+08], [2.03e+05], [6.10e+07], [F < L < P], [L < C < P],
  [37], [asc], [0.1], [8.94e+06], [3.89e+08], [2.24e+05], [6.71e+07], [F < L < P], [L < C < P],
  [37], [asc], [0.3], [1.05e+07], [4.88e+08], [2.67e+05], [7.93e+07], [F < L < P], [L < C < P],
  [37], [asc], [1.0], [1.62e+07], [8.37e+08], [4.17e+05], [1.22e+08], [F < L < P], [L < C < P],
  [59], [desc], [0.0], [2.58e+07], [2.59e+09], [1.04e+06], [5.68e+08], [F < L < P], [L < C < P],
  [59], [desc], [0.1], [2.89e+07], [3.03e+09], [1.17e+06], [6.24e+08], [F < L < P], [L < C < P],
  [59], [desc], [0.3], [3.50e+07], [3.91e+09], [1.43e+06], [7.38e+08], [F < L < P], [L < C < P],
  [59], [desc], [1.0], [5.67e+07], [7.00e+09], [2.33e+06], [1.14e+09], [F < L < P], [L < C < P],
  [59], [asc], [0.0], [2.58e+07], [1.24e+09], [3.76e+05], [5.67e+08], [F < L < P], [L < C < P],
  [59], [asc], [0.1], [2.89e+07], [1.46e+09], [4.23e+05], [6.24e+08], [F < L < P], [L < C < P],
  [59], [asc], [0.3], [3.51e+07], [1.90e+09], [5.16e+05], [7.37e+08], [F < L < P], [L < C < P],
  [59], [asc], [1.0], [5.68e+07], [3.42e+09], [8.41e+05], [1.13e+09], [F < L < P], [L < C < P],
  [79], [desc], [0.0], [6.18e+07], [7.62e+09], [1.89e+06], [2.34e+09], [F < L < P], [L < C < P],
  [79], [desc], [0.1], [6.89e+07], [8.94e+09], [2.11e+06], [2.57e+09], [F < L < P], [L < C < P],
  [79], [desc], [0.3], [8.31e+07], [1.16e+10], [2.56e+06], [3.04e+09], [F < L < P], [L < C < P],
  [79], [desc], [1.0], [1.33e+08], [2.08e+10], [4.14e+06], [4.67e+09], [F < L < P], [L < C < P],
  [79], [asc], [0.0], [6.19e+07], [3.53e+09], [6.60e+05], [2.34e+09], [F < L < P], [L < C < P],
  [79], [asc], [0.1], [6.90e+07], [4.16e+09], [7.37e+05], [2.57e+09], [F < L < P], [L < C < P],
  [79], [asc], [0.3], [8.32e+07], [5.42e+09], [8.93e+05], [3.04e+09], [F < L < P], [L < C < P],
  [79], [asc], [1.0], [1.33e+08], [9.80e+09], [1.44e+06], [4.67e+09], [F < L < P], [L < C < P],
  [97], [desc], [0.0], [1.13e+08], [1.85e+10], [2.57e+06], [6.37e+09], [F < L < P], [L < C < P],
  [97], [desc], [0.1], [1.26e+08], [2.18e+10], [2.90e+06], [7.00e+09], [F < L < P], [L < C < P],
  [97], [desc], [0.3], [1.51e+08], [2.84e+10], [3.56e+06], [8.28e+09], [F < L < P], [L < C < P],
  [97], [desc], [1.0], [2.40e+08], [5.18e+10], [5.87e+06], [1.27e+10], [F < L < P], [L < C < P],
  [97], [asc], [0.0], [1.13e+08], [9.11e+09], [8.62e+05], [6.36e+09], [F < L < P], [L < C < P],
  [97], [asc], [0.1], [1.26e+08], [1.08e+10], [9.73e+05], [7.00e+09], [F < L < P], [L < C < P],
  [97], [asc], [0.3], [1.52e+08], [1.41e+10], [1.20e+06], [8.27e+09], [F < L < P], [L < C < P],
  [97], [asc], [1.0], [2.41e+08], [2.56e+10], [1.98e+06], [1.27e+10], [F < L < P], [L < C < P],
  [103], [desc], [0.0], [1.05e+08], [2.11e+10], [2.49e+06], [8.54e+09], [F < L < P], [L < C < P],
  [103], [desc], [0.1], [1.19e+08], [2.49e+10], [2.83e+06], [9.40e+09], [F < L < P], [L < C < P],
  [103], [desc], [0.3], [1.49e+08], [3.23e+10], [3.52e+06], [1.11e+10], [F < L < P], [L < C < P],
  [103], [desc], [1.0], [2.51e+08], [5.85e+10], [5.94e+06], [1.71e+10], [F < L < P], [L < C < P],
  [103], [asc], [0.0], [1.05e+08], [9.78e+09], [8.52e+05], [8.54e+09], [F < L < P], [L < C < P],
  [103], [asc], [0.1], [1.20e+08], [1.15e+10], [9.71e+05], [9.39e+09], [F < L < P], [L < C < P],
  [103], [asc], [0.3], [1.49e+08], [1.51e+10], [1.21e+06], [1.11e+10], [F < L < P], [L < C < P],
  [103], [asc], [1.0], [2.51e+08], [2.74e+10], [2.05e+06], [1.71e+10], [F < L < P], [L < C < P],
  [109], [desc], [0.0], [1.25e+08], [2.35e+10], [2.77e+06], [1.13e+10], [F < L < P], [L < C < P],
  [109], [desc], [0.1], [1.42e+08], [2.77e+10], [3.16e+06], [1.24e+10], [F < L < P], [L < C < P],
  [109], [desc], [0.3], [1.76e+08], [3.61e+10], [3.94e+06], [1.47e+10], [F < L < P], [L < C < P],
  [109], [desc], [1.0], [2.96e+08], [6.53e+10], [6.66e+06], [2.25e+10], [F < L < P], [L < C < P],
  [109], [asc], [0.0], [1.25e+08], [1.05e+10], [9.61e+05], [1.13e+10], [F < L < P], [L < P < C],
  [109], [asc], [0.1], [1.42e+08], [1.24e+10], [1.09e+06], [1.24e+10], [F < L < P], [L < P < C],
  [109], [asc], [0.3], [1.76e+08], [1.61e+10], [1.36e+06], [1.47e+10], [F < L < P], [L < C < P],
  [109], [asc], [1.0], [2.96e+08], [2.93e+10], [2.29e+06], [2.25e+10], [F < L < P], [L < C < P],
  [131], [desc], [0.0], [2.15e+08], [5.82e+10], [3.84e+06], [2.78e+10], [F < L < P], [L < C < P],
  [131], [desc], [0.1], [2.43e+08], [6.88e+10], [4.36e+06], [3.06e+10], [F < L < P], [L < C < P],
  [131], [desc], [0.3], [3.00e+08], [8.99e+10], [5.39e+06], [3.62e+10], [F < L < P], [L < C < P],
  [131], [desc], [1.0], [4.97e+08], [1.64e+11], [9.02e+06], [5.57e+10], [F < L < P], [L < C < P],
  [131], [asc], [0.0], [2.15e+08], [2.60e+10], [1.38e+06], [2.78e+10], [F < L < P], [L < P < C],
  [131], [asc], [0.1], [2.44e+08], [3.07e+10], [1.56e+06], [3.06e+10], [F < L < P], [L < C < P],
  [131], [asc], [0.3], [3.00e+08], [4.02e+10], [1.92e+06], [3.62e+10], [F < L < P], [L < C < P],
  [131], [asc], [1.0], [4.98e+08], [7.35e+10], [3.19e+06], [5.56e+10], [F < L < P], [L < C < P],
)
#set text(size: 9pt)

== Route (c): what construction costs
#figure(image("fig4_phi_breakdown.svg", width: 92%), caption: [Cost components of route (c) at $a = 0$ and $a = 1$ (`modpoly_*.json`): construction $approx ell^(4.69)$, evaluation, root finding, Elkies.])
Construction / ladder find at $a = 0$: 7.5 ($ell = 37$), 28 (67), 81 (103), 129 (131). Additions per $M$-equivalent: construction 1.00, ladder 0.99 ($ell = 37$) to 1.61 (863), $psi$ 1.4 to 1.8, projected NTT ladder 1.8.

== Crossover with rho (extrapolated), $r = 2^128$, $g = 6$, $log_2 ell^*$
rho cost $sqrt(pi r / 4) (g + g a)$ ($g$ multiplications and $g$ additions per group operation, stated assumption); find costs scaled by $log_2 r / 61$; $ell_"max" = 2^65$.
#set text(size: 7.5pt)
#table(columns: 6, inset: 3pt, stroke: 0.4pt,
  [model], [$a = 0$], [$a = 0.1$], [$a = 0.3$], [$a = 1$], [shift],
  [ladder (a), measured desc fit l$>=$256], [21.74], [21.66], [21.54], [21.35], [-0.39],
  [ladder (a), measured asc fit l$>=$256], [21.74], [21.66], [21.54], [21.35], [-0.39],
  [ladder (a'), projected auto fit l$>=$2\^10], [24.23], [24.16], [24.07], [23.92], [-0.31],
  [ladder (a'), projected auto fit l$>=$2\^13], [25.07], [25.02], [24.94], [24.81], [-0.25],
  [ladder (a'), projected auto l$>=$2\^13, exponent at OLS CI low end], [26.10], [26.05], [25.98], [25.85], [-0.24],
  [ladder (a'), projected best of karatsuba/auto fit l$>=$2\^13], [25.07], [25.02], [24.94], [24.81], [-0.25],
  [ladder (a'), model l\^2 log l calibrated at top], [24.55], [24.51], [24.44], [24.32], [-0.23],
  [psi + CZ desc (b), measured fit], [15.38], [15.30], [15.19], [15.01], [-0.37],
  [psi + gcd asc (b), measured fit], [16.02], [15.95], [15.84], [15.69], [-0.34],
  [Phi amortised desc (c), measured fit 11..103], [30.26], [29.97], [29.54], [28.77], [-1.48],
  [Phi cold desc (c), measured fit 11..103], [13.69], [13.69], [13.69], [13.68], [-0.01],
)
#set text(size: 9pt)
#figure(image("fig3_crossover_r128.svg", width: 92%), caption: [Extrapolated crossover with rho by route and weight (`analysis.json`, `crossover`).])

== Limits (what is still not counted)
Sutherland's CRT evaluation (cold route (c) is the library builder, exponent 4.69); setup of $F_(p^k)$ and group orders for the ladder, kernel certification; additions inside inversions ($I = 91 M$ fixed), reductions, memory and storage of the $Phi_ell$ table, any wall time; `PolyMul::Auto` chooses by $M + W$ (whole-ladder best-of Karatsuba/Auto checked, equal to Auto for $ell >= 2^13$); $a$ is swept, not measured on hardware; the rho step's $g$ additions are an assumption; one instance per $ell$ (the near ties are within that noise); crossovers and the projection beyond $ell = 863$ are extrapolations; the full-crate library test was not run in this session (memory limit), the `isogeny_eval` tests (18) pass in a minimal crate that includes the same sources.
