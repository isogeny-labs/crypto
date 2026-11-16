# R5.3 — √élu with a three-prime NTT: does the vertical-margin crossover move?

Toy and synthetic instances only (j = −3375 vertical instances of round-4 lead 16.3, 61-bit p). No sub-rho claim; no claim about any deployed curve.

## Question, hypothesis, falsifier
- Question: with asymptotically fast polynomial multiplication, how far does the measured √élu exponent (round 4: 0.82 [0.805, 0.838], Karatsuba) fall, and do the §17.3 crossovers move?
- Hypothesis: the exponent drops toward 0.5–0.6; the rational-kernel conclusion stays (gets stronger); the generic-kernel crossover moves only if kernel finding speeds up.
- Falsifier: an NTT-arm exponent CI on 2^16 ≤ ℓ ≤ 2^24 lying inside [0.5, 0.6] would confirm; a CI overlapping 0.82 refutes the "drops toward 0.5–0.6" part at the measured range.

## Library work (`src/cryptanalysis/isogeny_eval/`)
- New `ntt` module: `mul_ntt` (three 62-bit NTT primes, Montgomery word products, CRT; overflow bound proved in the module comment for every p < 2^63), `ntt_mul_cost(la, lb, square) -> (W, M)`, `NTT_WORD_COST_IN_M = 1`.
- `PolyMul {Karatsuba, Ntt, Auto}` on `PrimeField` (default `Auto`); `OpTally` gains a `word` field; `mul_equivalents` = M + W + c·I.
- `poly::mul` under `Auto` takes the NTT only when the shorter operand has ≥ `NTT_THRESHOLD` (= 256) coefficients **and** `ntt_mul_cost` ≤ `karatsuba_cost` for that exact shape (new `pub fn karatsuba_cost`). `mul_karatsuba` stays available; Karatsuba-arm counts are the round-4 counts.
- Counting rule (stated): one NTT word multiplication mod a 62-bit prime = 1 M; additions not counted; I = 91 M as round 4. Measured contended wall ratio (threshold.txt): this NTT is slower per counted unit than lazily reduced schoolbook; wall crossover ≈ 3072–4096 coefficients versus count crossover 512 (balanced).
- Tests: NTT = schoolbook/Karatsuba on random inputs at 20/50/62/63-bit p over many lengths incl. edge lengths; √élu outputs identical across arms; `karatsuba_cost` equals the counted cost; Auto-choice test.

## Commands
```
B=target/release/examples/r5_3_ntt_sqrt_velu; D=research/isogeny_fibers_20261010/round5/r5_3_ntt_sqrt_velu
$B threshold $D/threshold.json 2> $D/threshold.txt
$B rational 10 24 3 20 $D/rational.json 2> $D/rational.txt
$B generic 31 2000 0.25 600 karatsuba $D/generic_karatsuba.json 2> $D/generic_karatsuba.txt   # stopped by cap before ℓ=1031
$B generic 31 4000 0.25 480 auto $D/generic_ntt.json 2> $D/generic_ntt.txt                    # stopped by cap before ℓ=1031
$B analyze $D research/isogeny_fibers_20261010/round4/16_3_vertical_margin $D/analysis.json > $D/analysis.txt
venv/bin/python $D/plot.py
```
`rational_hi.txt` is the log of an extra run (ℓ to 2^26.5, one seed) a predecessor agent started; it was stopped before writing JSON, so it is **not** used in any fit (shown for information only: NTT/Karatsuba count ratio 0.68 at ℓ ≈ 2^26.5).

## Reproduction of round 4 (measured)
87/87 rational rows: same p, codomain j and kernel-finding counts. Karatsuba √élu counts differ from round 4 by a constant +2 M +1 I per row (the #1699 library port's normalisation step), which does not affect exponents (0.822 [0.805, 0.838], identical to round 4). 16/16 generic Karatsuba rows: same p, j and ladder counts.

## Results (measured counts, Meq = M + W + 91 I)
| series | range | exponent [95% CI] |
|---|---|---|
| Vélu, codomain + 1 image | ℓ ≥ 2^10 | 0.995 [0.993, 0.997] |
| √élu Karatsuba | 2^16..2^24 | 0.822 [0.805, 0.838] |
| √élu NTT (Auto) | 2^16..2^24 | 0.798 [0.775, 0.820] |
| √élu NTT (Auto) | 2^20..2^24 | 0.745 [0.686, 0.804] |
| √élu NTT, wall (contended) | 2^16..2^24 | 0.749 [0.715, 0.784] |
| generic find, Karatsuba (this run) | 31..863 | 2.587 [2.550, 2.625] |
| generic find, NTT | 31..863 | 2.582 [2.544, 2.620] |
| generic find, NTT | 256..863 | 2.554 [2.352, 2.756] |

At ℓ = 2^24 the NTT arm counts 2.21e7 Meq vs Karatsuba 2.45e7 (ratio 1.11). Local 4-octave exponents fall from ≈ 0.84 (2^16–2^20) to 0.745 (2^20–2^24) for NTT while Karatsuba stays ≈ 0.80–0.82. Fitting c·√ℓ·(log ℓ)^k gives k ≈ 4.1 [3.86, 4.34]: at these sizes the log factors dominate, so the theoretical ½ is not visible. Generic kernel finding does not speed up: the F_{p^k} products (k ≤ 431) sit mostly below the per-shape NTT crossover, so the NTT arm is within 3% of Karatsuba.

## Derived crossover table (extrapolated; log2 ℓ*, g = 6 M per group op; full table incl. g = 12 in analysis.txt)
| model | r = 2^64 | r = 2^128 | r = 2^256 |
|---|---|---|---|
| ℓ_max = 2√p | 33 | 65 | 129 |
| rational, √élu Karatsuba fit | 35.9 | 74.8 | 152.7 |
| rational, √élu NTT fit ≥2^16 | 36.4 | 76.5 | 156.8 |
| rational, √élu NTT fit ≥2^20 | 37.4 | 80.4 | 166.3 |
| rational, √ℓ log²ℓ model | 40.9 | 99.8 | 223.2 |
| generic, round-4 fit | 9.6 | 21.5 | 45.7 |
| generic, NTT fit (central / CI-low) | 9.6 / 9.6 | 21.7 / 22.8 | 46.4 / 49.6 |

(The derivation uses the round-4 rule: cost of find + evaluate vs g·√(πr/4); a lower √élu cost moves the rational crossover *up*, since the crossover is where the cost reaches rho.)

## Verdict
- Exponent drops toward 0.5–0.6: **refuted at the measured range** (2^10–2^24). NTT gives 0.798 [0.775, 0.820] on 2^16–2^24, overlapping 0.82 only marginally; the 2^20–2^24 window 0.745 [0.686, 0.804] excludes 0.6. The trend is downward, consistent with √ℓ·polylog asymptotics, but not reached.
- Rational-kernel conclusion: **confirmed, stronger** — every rational-kernel crossover stays beyond ℓ_max = 2√p for r = 2^64/2^128/2^256 (except the plain-Vélu row, as in round 4).
- Generic-kernel crossover: **unchanged** (2^21.5–2^25 at r = 2^128, in the round-4 band 2^22–2^25), because kernel finding did not speed up (task 3 measured, null result).

## Limits
Counted unit treats an NTT word product as 1 M (stated, not measured; contended wall says the NTT is costlier per unit). Single machine, contended wall times. Generic arm stopped at ℓ = 863 by the time cap. Crossover rows are extrapolations by many octaves.

## Next step
Push √élu to ℓ ≥ 2^28 (needs memory-lean product trees) to see the ½ regime; make the F_{p^k} product use NTT with k-adapted shapes (Kronecker substitution) before re-fitting generic finding.

Figures: fig1_sqrtvelu_cost.svg, fig2_local_exponent.svg, fig3_generic.svg, fig4_threshold.svg, fig5_crossover.svg; report.pdf.
