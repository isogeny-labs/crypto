# R6.4 — Vertical margin, phase two: ψ_ℓ, Φ_ℓ(j, Y), ascending edges, NTT ladder

Toy and synthetic instances only (the j = −3375, D = −7 vertical instances of round-4 lead 16.3, 61-bit p). No sub-rho claim; no claim about any deployed curve. Preregistration: `round6/PROGRAM.md` §R6.4.

## Question, hypothesis, falsifier (as preregistered)
- Question: what do ψ_ℓ factoring, Φ_ℓ(j, Y) root finding and ascending edges cost, and does NTT move the generic-kernel exponent?
- Hypothesis: generic kernel finding stays ≥ ℓ² so the §17.3 generic crossover moves by less than one bit of ℓ; ascending edges cost the same as descending ones at equal ℓ.
- Falsifier: a generic-kernel exponent below 2 with NTT, or an ascending edge measurably cheaper or dearer than a descending one at equal ℓ (95% intervals disjoint).

## Arms and unit
- (a) `ladder`: x-only ladder over F_{p^k}, k = (ℓ−1)/2, `PolyMul::Auto` (NTT where its counted cost is lower), descending from the crater and ascending from the floor curve reached; ℓ = 37 … 863 (20 primes; 3 reps for ℓ ≤ 211).
- (a′) `project`: the ladder's per-step counted cost measured exactly in F_{p^k} for ℓ up to 2^16 = 65 537, times the exact step count. Validated against (a): projection/measured = 1.0006–1.0044 on all 20 measured ℓ.
- (b) `psi`: ψ_ℓ by the standard recurrences, then Cantor–Zassenhaus equal-degree splitting (crater) or gcd(ψ_ℓ, x^{p^k} − x) (floor); ℓ = 37 … 131 (cap 3000 s reached after 131; 9 primes).
- (c) `modpoly`: the library's Φ_ℓ mod p (q-series; wall time only, *not counted*), then counted evaluation at j and root finding, library Elkies kernel certified; ℓ = 11 … 103 (23 primes; cap reached before 107). A Sutherland-style CRT evaluation was not implemented (the "if feasible" part of arm (c) is not done).
- Unit (R5.3's): M = F_p mult/square incl. coefficient products; W = word product mod a 62-bit NTT prime, stated at 1 W = 1 M; I = 91 M; Meq = M + W + 91 I. Additions not counted. Wall times contended (4 cores shared by several agents).

## Commands
```
B=target/release/examples/r6_4_vertical_margin_2; D=research/isogeny_fibers_20261010/round6/r6_4_vertical_margin_2
$B project 5 16 0.25 $D/project.json > $D/project.txt 2>&1
$B ladder 37 470 0.25 3000 auto 3 $D/ladder_auto.json > $D/ladder_auto.txt 2>&1
$B ladder 500 900 0.25 4000 auto 1 $D/ladder_auto_hi.json > $D/ladder_auto_hi.txt 2>&1
$B psi 37,41,47,59,67,79,97,109,131 3000 $D/psi.json > $D/psi.txt 2>&1
$B modpoly 131 1500 $D/modpoly.json > $D/modpoly.txt 2>&1
$B analyze $D research/isogeny_fibers_20261010/round5/r5_3_ntt_sqrt_velu research/isogeny_fibers_20261010/round4/16_3_vertical_margin $D/analysis.json > $D/analysis.txt
python $D/plot.py
```

## Reproduction
20/20 descending ladder rows reproduce R5.3 (`generic_ntt.json`): same p, same counted ladder Meq. Ascending reached the crater in 19/20 ladder rows, 8/9 ψ rows, 20/23 Φ rows; library Elkies succeeded in both directions in 23/23.

## Results (measured counts unless marked projected)
| series | range | exponent [95% CI] | n |
|---|---|---|---|
| (a) ladder find, desc, Auto | 37..863 | 2.582 [2.544, 2.620] | 20 |
| (a) ladder find, desc, Auto | 257..863 | 2.554 [2.352, 2.756] | 8 |
| (a) ladder find, asc, Auto | 37..863 | 2.587 [2.547, 2.628] | 19 |
| (a′) projected, Karatsuba | ≥ 2^10 | 2.579 [2.553, 2.605] | 25 |
| (a′) projected, Auto | ≥ 2^10 | 2.088 [2.020, 2.155] | 25 |
| (a′) projected, Auto (= NTT) | ≥ 2^13 | 1.919 [1.724, 2.115] | 13 |
| (a′) projected, Auto, local window | 2^14..2^16 | 1.748 [1.409, 2.086] | — |
| (b) ψ + CZ, desc | 37..131 | 3.565 [3.302, 3.828] | 9 |
| (b) ψ + gcd, asc | 37..131 | 3.439 [3.087, 3.791] | 8 |
| (b) ψ_ℓ recurrences alone | 37..131 | 2.817 [2.724, 2.911] | 9 |
| (c) Φ eval + roots, desc (Φ not counted) | 11..103 | 1.884 [1.804, 1.964] | 23 |
| (c) Φ eval + roots, asc (Φ not counted) | 11..103 | 1.748 [1.659, 1.836] | 20 |
| (c) library Φ_ℓ mod p, wall seconds | 20..103 | 4.600 [4.515, 4.684] | 19 |

Ascending / descending at equal ℓ (paired ratio of counted Meq, 95% CI):
- (a) ladder, expected find: 1.001 [1.001, 1.002] (n = 19) — the preregistered disjoint-level criterion did not trigger (levels overlap), but the paired interval excludes 1: a small difference (ascending ≈ 0.1% dearer) was detected. No equivalence margin was prespecified, so this is not a claim of equality.
- (b) ψ route: 0.501 [0.457, 0.548] (n = 8) — levels disjoint: the floor's kernel factor is found by one gcd with x^{p^k} − x, while the crater needs CZ splitting of ψ_ℓ.
- (c) Φ route: 0.360 [0.347, 0.373] (n = 20) — levels disjoint: on the floor Φ_ℓ(j, Y) has one rational root (the crater), on the crater ℓ+1 (rational roots 1 vs ℓ+1), so root extraction is cheaper.

## Derived (extrapolated) crossovers with rho, g = 6 M/op, log2 ℓ*
Full table, incl. g = 12, in `analysis.txt`. **Restricted cost model:** every row prices only M, W (= 1 M) and I (= 91 M); additions are not counted in any arm or in the rho reference, and row (c) further omits Φ_ℓ construction and Elkies work. The rows below are therefore stage counts under that model, not complete costs, and any ordering between them (which route is "cheapest", which crossover is largest) holds only under that model. The complete-cost ordering, with additions priced (e.g. a 0 / 0.1 / 1 M band) and Φ_ℓ construction counted, is **unresolved**: the library's counter has no addition tally, so this round adds no addition counts.

| model | r = 2^64 | r = 2^128 | r = 2^256 |
|---|---|---|---|
| R5.3 generic NTT fit (ℓ ≥ 256), = (a) measured desc and asc | 9.61 | 21.74 | 46.41 |
| (a′) projected Auto fit ℓ ≥ 2^10 | 9.38 | 24.23 | 54.41 |
| (a′) projected Auto fit ℓ ≥ 2^13 | 8.92 | 25.07 | 57.89 |
| (a′) same, exponent at CI low end | 8.11 | 26.10 | 62.64 |
| (a′) model ℓ² log ℓ calibrated at 2^16 | 9.72 | 24.55 | 55.47 |
| (b) ψ + CZ desc | 6.69 | 15.38 | 33.05 |
| (c) Φ eval + roots, LOWER BOUND (Φ_ℓ not counted) | 13.52 | 29.98 | 63.42 |

## Verdict against the preregistration
- "Generic kernel finding stays ≥ ℓ²": **not falsified** — but not confirmed either: the projected ℓ ≥ 2^13 interval 1.919 [1.724, 2.115] contains values below 2 as well as 2 itself, so this is insufficient evidence of an exponent below 2, not evidence that the exponent is ≥ 2. The NTT does move the exponent: from 2.58 (Karatsuba, and Auto inside the measured range ℓ ≤ 863) toward ≈ 2 once k exceeds the NTT threshold — consistent with the ℓ² field-degree term (log2 ℓ bits per step × k-coefficient products) dominating.
- "The §17.3 generic crossover moves by less than one bit": **refuted for the projected ladder.** At r = 2^128 the ladder crossover moves from 2^21.7 (round 4 / R5.3 fit) to 2^24.2–2^25.1 (2^26.1 at the CI low end); at r = 2^256 from 2^46.4 to 2^54–2^58. These are extrapolations of a measured-per-step projection, not end-to-end measurements beyond ℓ = 863. The direction is *more* protection from the gap, not less. Inside ℓ ≤ 863 the measured crossover is unchanged (2^21.74).
- ψ_ℓ factoring (b) has the smallest crossover of the counted routes under the restricted cost model (exponent ≈ 3.5–3.7, crossover 2^15.4 at r = 2^128); its complete-cost rank is unresolved.
- Φ_ℓ(j, Y) (c): the counted evaluation + roots alone gives an exponent ≈ 1.9, but the library's Φ_ℓ mod p costs ℓ^4.6 in wall seconds and is not counted, so (c) is only a lower bound (2^30 at r = 2^128). A Sutherland CRT evaluation (≈ ℓ^{2+o(1)} per j) was not implemented; whether (c) beats (a′) is **open**.
- "Ascending = descending": **not falsified for the ladder** by the preregistered disjoint-interval criterion, but the paired ratio 1.001 [1.001, 1.002] detects a small difference (≈ 0.1%), so equality is not claimed; **falsified for (b) and (c)** (ascending 0.50× and 0.36×, intervals disjoint). The (b)/(c) difference is the factorisation structure on the floor vs the crater, not the edge's direction per se. Whether direction changes the margin depends on which route is cheapest at complete cost, which is unresolved (see the restricted-cost-model note above); under the restricted model the ladder rows are within 0.2% in the two directions.

## §17.3 table, superseded (r = 2^128, g = 6)
Generic kernel, ladder route under the restricted cost model (M + W + 91 I, additions unpriced; complete-cost ordering of routes unresolved): ℓ* ≈ 2^21.7 (measured fit ≤ 863) → 2^24.2–2^25.1 (NTT ladder, projected to 2^16). Rational kernel: unchanged from R5.3 (no crossover within ℓ ≤ 2√p).

## Limits
W = 1 M is stated, not measured (R5.3: this NTT is slower per counted unit in wall time). (a′) multiplies a measured per-step count by an exact step count; it does not run the full find beyond ℓ = 863. Crossovers are extrapolations. ψ arm capped at ℓ = 131 and Φ arm at ℓ = 103 by time. Φ_ℓ mod p generation is uncounted. Additions are uncounted in every arm and reference, so route rankings and crossover orderings are conditional on the restricted cost model; the 0 / 0.1 / 1 M addition-weight band was not computed (no native addition counts exist for these runs). One instance per ℓ (seed 0x1630 ^ ℓ).
