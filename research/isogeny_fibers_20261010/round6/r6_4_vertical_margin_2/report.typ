#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 9.5pt)
#set par(justify: true)
#show heading: set text(size: 11pt)

= R6.4: the vertical margin, phase two
Round 6, 2026-10-19. Code `examples/r6_4_vertical_margin_2.rs` (native Rust). Raw records: `ladder_auto*.json`, `project.json`, `psi.json`, `modpoly.json`; fits: `analysis.json` / `analysis.txt`. Full write-up: `RESULTS.md`. Toy, synthetic instances only ($j = -3375$, $D = -7$, 61-bit $p$); no sub-rho or deployed-curve claim.

== Question and hypothesis (preregistered)
What do $psi_ell$ factoring, $Phi_ell(j, Y)$ root finding and ascending edges cost, and does NTT move the generic exponent? Hypothesis: generic finding stays $>= ell^2$, the §17.3 crossover moves < 1 bit; ascending = descending. Falsifier: exponent CI below 2, or disjoint asc/desc intervals.

== Method
Arms: (a) $FF_(p^k)$ ladder, `PolyMul::Auto`, $ell = 37..863$, both directions; (a′) per-step counts measured exactly to $ell = 2^16$ × exact step count (agrees with (a) to ≤ 0.44%); (b) $psi_ell$ + Cantor–Zassenhaus (crater) / gcd with $x^(p^k) - x$ (floor), $ell <= 131$; (c) library $Phi_ell mod p$ (uncounted, wall only) + counted evaluation and roots, $ell <= 103$. Unit Meq = M + W + 91 I, W stated at 1 M. 20/20 descending ladder rows reproduce R5.3.

== Results (measured counts; (a′) projected)
#table(columns: 3,
  [series], [range], [exponent [95% CI]],
  [(a) ladder desc], [37..863], [2.582 [2.544, 2.620]],
  [(a) ladder asc], [37..863], [2.587 [2.547, 2.628]],
  [(a′) projected Karatsuba], [$>= 2^10$], [2.579 [2.553, 2.605]],
  [(a′) projected Auto], [$>= 2^10$], [2.088 [2.020, 2.155]],
  [(a′) projected Auto], [$>= 2^13$], [1.919 [1.724, 2.115]],
  [(b) $psi$ + CZ desc], [37..131], [3.565 [3.302, 3.828]],
  [(b) $psi$ + gcd asc], [37..131], [3.439 [3.087, 3.791]],
  [(c) $Phi$ eval+roots desc], [11..103], [1.884 [1.804, 1.964]],
  [(c) $Phi_ell$ mod p, wall s], [20..103], [4.600 [4.515, 4.684]],
)
Paired asc/desc: ladder 1.001 [1.001, 1.002]; $psi$ 0.501 [0.457, 0.548]; $Phi$ 0.360 [0.347, 0.373].

#figure(image("fig1_generic_find.svg", width: 88%), caption: [Generic kernel finding, all arms and both directions (measured; (a′) projected).])
#figure(image("fig2_local_exponent.svg", width: 78%), caption: [Local exponents of the projected ladder over 2-octave windows.])
#figure(image("fig3_asc_desc_ratio.svg", width: 78%), caption: [Ascending / descending counted cost at equal $ell$.])
#figure(image("fig4_crossover.svg", width: 78%), caption: [Derived (extrapolated) crossovers with rho.])

== Derived crossovers ($g = 6$, log2 $ell^*$)
#table(columns: 4,
  [model], [$r=2^64$], [$r=2^128$], [$r=2^256$],
  [R5.3 / (a) measured fit], [9.61], [21.74], [46.41],
  [(a′) projected Auto, $>= 2^10$], [9.38], [24.23], [54.41],
  [(a′) projected Auto, $>= 2^13$], [8.92], [25.07], [57.89],
  [(b) $psi$ + CZ], [6.69], [15.38], [33.05],
  [(c) $Phi$, lower bound], [13.52], [29.98], [63.42],
)

== Verdict
Exponent $>= 2$: *not falsified* (no CI below 2), but NTT moves it from 2.58 toward ≈ 2. Crossover moves < 1 bit: *refuted in projection* ($2^21.7 -> 2^24.2..2^25.1$ at $r = 2^128$; more margin, not less). Asc = desc: *holds for the ladder*, *falsified for (b) and (c)* (floor factorisation structure); the cheapest route (a′) is direction-neutral. Whether a Sutherland-style $Phi$ evaluation beats (a′) is open.

== Limits
W = 1 M stated, not measured. (a′) is a projection beyond $ell = 863$. Crossovers are extrapolations. Time caps: $psi$ at 131, $Phi$ at 103; $Phi_ell$ generation uncounted; one instance per $ell$.
