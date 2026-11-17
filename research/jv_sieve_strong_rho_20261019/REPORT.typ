#set page(paper: "a4", margin: (x: 1.6cm, y: 1.8cm), numbering: "1")
#set text(size: 9pt)
#set table(inset: 3pt, stroke: 0.4pt + luma(160))
#show table: set text(size: 6.5pt)
#align(center)[
  #text(14pt, weight: "bold")[The Joux–Vitse sieve on E(F#sub[p⁶]) against a strong rho] \
  #text(9pt)[Weak class only (never prime order); the cover's Jacobian has genus 3. Study `research/jv_sieve_strong_rho_20261019`, run 2026-10-19/20, protocol frozen before data. Every number is copied from `summary.json`, computed by `examples/jv_sieve_study.rs`.]
]
#strong[Scope: a weak class only.] Curves `y² = h(x)(x−α)(x−σα)` over
`F_{p⁶}` with full rational 2-torsion whose isogeny class contains a
weak curve (about 55–63 % of such curves, ledger §18.5). The class never
has prime order. The cover\'s Jacobian has #strong[genus 3]. Nothing
here is a statement about a prime-order or generic curve.

#strong[Class:] measurement of an existing algorithm (the sieve of
`src/cryptanalysis/jv_sieve.rs`) against a stronger reference than the
ledger had. Not a new algorithm. Protocol frozen before any data:
#link("PROTOCOL.md")[`PROTOCOL.md`] (deviations log appended at its
end). Numbers: #link("TABLES.md")[`TABLES.md`],
#link("summary.json")[`summary.json`]; raw runs: #link("raw/")[`raw/`];
report: #link("REPORT.pdf")[`REPORT.pdf`].

== What was measured
Instances `generate_spec(p, seed)`, seeds 1–16,
`p ∈ {251, 503, 1009, 1511, 1823}` (`ℓ ≈ 2^45.8 … 2^63.0`; 1823 is the
last `p` with `ℓ < 2^63`). The sieve and every rho arm ran on the same
instances and host. Every run was checked against the planted logarithm:
all sieve runs (80 per `JV_LIN` arm) and all 268 rho runs recovered it.

- #strong[Sieve:] `jv_cover --exp sieve --stop 64`, dense and sparse
  FGLM descent, paired.
- #strong[Rho:] `src/cryptanalysis/jv_rho_strong.rs`, a 2×2 factorial:
  R0 plain, R1 batched inversion (k \= 16), R2 negation map, R3 negation
  + batched (#strong[the strong rho]), all with 256 multipliers and
  distinguished points. Costs are #emph[counted by operation class] and
  priced twice: in the #strong[harness unit] (`F_{q³}` product 55,
  square 55, inversion 166 `F_p` multiplications, measured from the
  harness\'s own `Fq3`; one affine addition is 331) and in a
  #strong[tuned unit] (Karatsuba, Chung–Hasan squaring, adjugate
  inversion; 24/18/62) that credits rho alone.
- `S = F_p multiplications / (331·√ℓ)`. `S⁺` also charges every `F_p`
  addition, lookup and native operation one multiplication, on both
  sides.

== Results
#strong[1. The sieve still beats a strong rho, from about p \= 1009 up,
and the gap widens.] Paired sieve/rho ratios, 95 % bootstrap (n \= 16;
below 1 is the sieve cheaper):

#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,right,right,right,right,).at(col),
  inset: 6pt,
  [p], [S, harness], [S, tuned], [S⁺, harness], [S⁺, tuned],
  [251],
  [18.5 \[5.4, 47\]],
  [44 \[13, 109\]],
  [37 \[12, 89\]],
  [73 \[23, 176\]],
  [503],
  [0.48 \[0.35, 0.64\]],
  [1.15 \[0.84, 1.53\]],
  [1.34 \[0.94, 1.82\]],
  [2.65 \[1.85, 3.62\]],
  [1009],
  [0.024 \[0.019, 0.031\]],
  [0.058 \[0.044, 0.074\]],
  [0.067 \[0.053, 0.084\]],
  [0.132 \[0.105, 0.165\]],
  [1511],
  [0.0090 \[0.0070, 0.012\]],
  [0.022 \[0.017, 0.029\]],
  [0.028 \[0.022, 0.036\]],
  [0.055 \[0.044, 0.071\]],
  [1823],
  [0.0042 \[0.0032, 0.0058\]],
  [0.010 \[0.0078, 0.014\]],
  [0.015 \[0.0125, 0.020\]],
  [0.030 \[0.025, 0.038\]],
)
]

The pre-registered kill rules did not fire: K1 (S⁺ ratio interval
reaching 1 at any `p ≥ 1009`, harness price) and K2 (the same in the
tuned price or inside the §5a band) both hold at 1009, 1511 and 1823.
Against the cheapest rho in the tuned price (R2, post hoc) the 1009
ratios are 0.062 \[0.045, 0.082\] (S) and 0.137 \[0.104, 0.183\] (S⁺).
The distance remaining is therefore a factor of about 65 (S⁺, harness)
to 33 (S⁺, tuned) at `p = 1823`.

#strong[2. The crossover moves up against a strong rho.] The ledger\'s
crossover against the weak rho was `p ≈ 333–430`. Against the strong rho
it is between `p = 251` and `p ≈ 503` in S (harness) and unresolved at
503 in S⁺ or the tuned price, resolved by 1009.

#strong[3. §5a band.] Native work priced at 0, 0.1 and 1 GAE per
operation on every arm, rho included, two layers (A: all
non-multiplication work including `F_p` additions; B: without them), the
sieve side carrying an upper bound for the F4/linear-algebra additions
it does not emit (#link("TABLES.md")[T2]). The sieve ranks below rho at
every weight and in all four columns at `p = 1009, 1511, 1823` (layer A,
`w = 1`, tuned: 0.37 \[0.29, 0.46\], 0.16 \[0.13, 0.21\], 0.088 \[0.072,
0.11\]). At `p = 251` rho ranks below; at `p = 503` the order changes
inside the band, so the pair is #strong[unresolved] there.

#strong[4. The ℓ^{1/3} claim is not supported over the measured range
(K3 fires).] Fitted exponent of total `F_p` multiplications in ℓ,
251–1823: sieve −0.122 \[−0.165, −0.084\] against the theory\'s 1/3;
strong rho 0.508 \[0.480, 0.538\] against 1/2 (the instrument check
passes). Post hoc, over `p ≥ 1009` (constant `m = 9`): sieve 0.052
\[−0.015, 0.118\], excluding 1/3. The sieve\'s cost falls from `p = 251`
to `1009` (the sieve runs at `m = 10` at 251–503 and `m = 9` from 1009,
and relations per line grow with `p`) and is then about flat, between
`2^{31.6}` and `2^{31.9}` multiplications. It beats rho because rho
grows as `ℓ^{1/2}` while the sieve does not grow over this range,
#strong[not] because it follows `ℓ^{1/3}`. Whether the cost starts to
rise like `ℓ^{1/3}` beyond `p = 1823` is untested here (`ℓ` exceeds 64
bits next), so extrapolating on `ℓ^{1/3}` is an extrapolation with no
measured exponent behind it.

#strong[5. What the rho accelerations buy in this tower] (R0 → R2 → R3,
#link("TABLES.md")[T3]): #emph[Negation] is worth about 1.2–1.6× (theory
√2). #emph[Batched inversion buys nothing]: an inversion costs about 3
products here, so Montgomery\'s trick (3 products) is neutral in the
harness price and slightly worse in the tuned price (R1 0.542 against R0
0.499 at `p = 1009`). The brief\'s \"2–3× cheaper per step\" comes from
cheaper field arithmetic (tuned price: 2.4×), not from batching.
Positive control: the plain arm gives S \= 1.14–1.45, the ledger\'s
reference ≈ 1.3. A/A at `p = 1009` (new walk randomness): R0 1.25
against 1.29, R3 0.823 against 0.808, inside their intervals; the
sieve\'s counts repeat identically on all 16 instances.

#strong[6. Sparse FGLM in the sieve\'s descent] (#link("TABLES.md")[T4],
PR \#2244\'s linear stage, `JV_LIN`): the descent rows are bit-identical
in 80/80 dense/sparse pairs (same digest and outcome). The linear stage
is 2.08–2.20× cheaper, descent multiplications 1.28–1.29×, and the
end-to-end `S` 1.014× (251) to 1.126× (1009, 1511) cheaper, 1.096× at
1823 (intervals in T4). The sparse path fell back to the dense one
0.4–7.5 times per run; that cost is counted. This is an engineering gain
on a route that already beats rho; it moves no ranking.

== Limits
- Weak class only; `p ≤ 1823` (`ℓ < 2^63`). Harness unit throughout;
  wall time is an observation (sieve 4 threads, rho one core: at
  `p = 1823` the sieve\'s mean wall is 9 s against 760 s for one
  strong-rho run; not a speedup claim).
- The isogeny walk to reach the class (ledger §13, §18.4) was
  #strong[not re-measured]; end-to-end from a non-weak curve against the
  strong rho is not asserted here.
- The tuned price is a cost model, not a faster implementation; rho\'s
  native work (canonicalisation, hashing, DP checks, scalar updates,
  cycle-window comparisons) is counted and priced in S⁺ and the band.
  The sieve\'s F4 additions are not emitted by the harness
  (implementation gap): the band uses an upper bound, labelled as such.
- The post hoc fit and the R2 comparison are labelled post hoc and were
  not in the protocol.

== Figures
#image("figs/ratio_vs_l.svg")

#emph[Figure 1. Sieve over strong rho (R3), paired by instance, four
prices; 1 is parity. Study jv\_sieve\_strong\_rho\_20261019, instances
seeds 1–16 per size, 95% bootstrap intervals; source summary.json.]

#image("figs/exponent.svg")

#emph[Figure 2. Mean total F\_p multiplications against ℓ; fitted slopes
in the legend (theory 1/3 sieve, 1/2 rho). Study
jv\_sieve\_strong\_rho\_20261019, instances seeds 1–16 per size, 95%
bootstrap intervals; source summary.json.]

#image("figs/rho_factorial.svg")

#emph[Figure 3. Rho factorial at p \= 251, 503, 1009: negation map and
batched inversion, harness price. Study
jv\_sieve\_strong\_rho\_20261019, instances seeds 1–16 per size, 95%
bootstrap intervals; source summary.json.]

#image("figs/dense_vs_sparse.svg")

#emph[Figure 4. Dense over sparse FGLM in the sieve\'s descent, paired
on the same instances. Study jv\_sieve\_strong\_rho\_20261019, instances
seeds 1–16 per size, 95% bootstrap intervals; source summary.json.]

== T1. Sieve against the strong rho (R3), paired by instance
S, S⁺ in units of 331·√ℓ F\_p multiplications. Ratios are sieve/rho:
below 1 is the sieve cheaper. 95% bootstrap over instances.

#align(center)[#table(
  columns: 11,
  align: (col, row) => (right,right,right,right,right,right,right,right,right,right,right,).at(col),
  inset: 6pt,
  [p], [log₂ℓ], [n], [sieve S], [sieve S⁺], [rho S (harness / tuned)],
  [rho S⁺ (harness / tuned)], [S ratio, harness], [S ratio, tuned], [S⁺
  ratio, harness], [S⁺ ratio, tuned],
  [251],
  [45.8],
  [16],
  [15.28 \[4.63, 34.6\]],
  [35.84 \[11.9, 78.9\]],
  [0.827 / 0.344],
  [0.975 / 0.492],
  [18.47 \[5.35, 46.6\]],
  [44.41 \[12.9, 109\]],
  [36.76 \[11.7, 88.6\]],
  [72.9 \[22.9, 176\]],
  [503],
  [51.8],
  [16],
  [0.3625 \[0.264, 0.471\]],
  [1.195 \[0.839, 1.59\]],
  [0.758 / 0.315],
  [0.893 / 0.451],
  [0.4783 \[0.348, 0.64\]],
  [1.149 \[0.842, 1.53\]],
  [1.338 \[0.935, 1.82\]],
  [2.652 \[1.85, 3.62\]],
  [1009],
  [57.9],
  [16],
  [0.01954 \[0.0161, 0.0231\]],
  [0.06346 \[0.0554, 0.0711\]],
  [0.808 / 0.336],
  [0.952 / 0.480],
  [0.02419 \[0.0185, 0.0311\]],
  [0.05814 \[0.0444, 0.0741\]],
  [0.06666 \[0.0532, 0.0837\]],
  [0.1321 \[0.105, 0.165\]],
  [1511],
  [61.4],
  [16],
  [0.007043 \[0.00613, 0.00819\]],
  [0.0254 \[0.0236, 0.0273\]],
  [0.780 / 0.325],
  [0.920 / 0.464],
  [0.009024 \[0.00703, 0.012\]],
  [0.02169 \[0.0169, 0.0289\]],
  [0.02762 \[0.0221, 0.036\]],
  [0.05473 \[0.0439, 0.0713\]],
  [1823],
  [63.0],
  [16],
  [0.003761 \[0.00323, 0.00434\]],
  [0.01605 \[0.0148, 0.0174\]],
  [0.887 / 0.369],
  [1.045 / 0.527],
  [0.004241 \[0.00324, 0.00575\]],
  [0.01019 \[0.00779, 0.0138\]],
  [0.01535 \[0.0125, 0.0196\]],
  [0.03043 \[0.0249, 0.0384\]],
)
]

== T2. §5a band: sieve/rho with native work priced at w GAE per
operation on every arm
Layer A prices every non-multiplication counted operation including F\_p
additions; layer B excludes F\_p additions. The sieve side adds a bound
for F4/linear-algebra additions it does not emit (one per multiplication
outside the sieve and enumeration). A ratio is a rank only where its
whole interval is on one side of 1 at every weight and in all four
columns.

#align(center)[#table(
  columns: 7,
  align: (col, row) => (right,right,right,right,right,right,left,).at(col),
  inset: 6pt,
  [p], [w], [A, harness], [A, tuned], [B, harness], [B, tuned], [ranked
  at every weight],
  [251],
  [0.0],
  [18.5 \[5.4, 46\]],
  [44.4 \[13, 1.1e+02\]],
  [18.5 \[5.3, 45\]],
  [44.4 \[13, 1.1e+02\]],
  [no],
  [],
  [0.1],
  [126 \[44, 2.9e+02\]],
  [138 \[48, 3.1e+02\]],
  [15.9 \[5.9, 35\]],
  [21.1 \[8, 47\]],
  [],
  [],
  [1.0],
  [142 \[50, 3.3e+02\]],
  [144 \[50, 3.3e+02\]],
  [14.4 \[6.3, 29\]],
  [15 \[6.7, 31\]],
  [],
  [503],
  [0.0],
  [0.478 \[0.35, 0.64\]],
  [1.15 \[0.84, 1.5\]],
  [0.478 \[0.35, 0.63\]],
  [1.15 \[0.84, 1.5\]],
  [no],
  [],
  [0.1],
  [5.81 \[4.1, 8\]],
  [6.35 \[4.5, 8.6\]],
  [0.686 \[0.49, 0.93\]],
  [0.907 \[0.64, 1.2\]],
  [],
  [],
  [1.0],
  [6.61 \[4.7, 9.1\]],
  [6.68 \[4.7, 9.1\]],
  [0.811 \[0.56, 1.1\]],
  [0.844 \[0.59, 1.2\]],
  [],
  [1009],
  [0.0],
  [0.0242 \[0.018, 0.031\]],
  [0.0581 \[0.044, 0.075\]],
  [0.0242 \[0.018, 0.031\]],
  [0.0581 \[0.045, 0.074\]],
  [yes],
  [],
  [0.1],
  [0.32 \[0.25, 0.4\]],
  [0.35 \[0.28, 0.44\]],
  [0.0307 \[0.025, 0.038\]],
  [0.0405 \[0.033, 0.05\]],
  [],
  [],
  [1.0],
  [0.365 \[0.29, 0.46\]],
  [0.368 \[0.29, 0.46\]],
  [0.0345 \[0.028, 0.043\]],
  [0.036 \[0.029, 0.044\]],
  [],
  [1511],
  [0.0],
  [0.00902 \[0.007, 0.012\]],
  [0.0217 \[0.017, 0.029\]],
  [0.00902 \[0.007, 0.012\]],
  [0.0217 \[0.017, 0.029\]],
  [yes],
  [],
  [0.1],
  [0.136 \[0.11, 0.18\]],
  [0.149 \[0.12, 0.19\]],
  [0.0128 \[0.01, 0.017\]],
  [0.0169 \[0.014, 0.022\]],
  [],
  [],
  [1.0],
  [0.156 \[0.12, 0.2\]],
  [0.157 \[0.13, 0.2\]],
  [0.015 \[0.012, 0.019\]],
  [0.0156 \[0.013, 0.02\]],
  [],
  [1823],
  [0.0],
  [0.00424 \[0.0032, 0.0058\]],
  [0.0102 \[0.0078, 0.014\]],
  [0.00424 \[0.0033, 0.0058\]],
  [0.0102 \[0.0078, 0.014\]],
  [yes],
  [],
  [0.1],
  [0.0761 \[0.062, 0.097\]],
  [0.0832 \[0.068, 0.11\]],
  [0.00708 \[0.0058, 0.009\]],
  [0.00936 \[0.0076, 0.012\]],
  [],
  [],
  [1.0],
  [0.0869 \[0.071, 0.11\]],
  [0.0878 \[0.071, 0.11\]],
  [0.00879 \[0.0073, 0.011\]],
  [0.00914 \[0.0076, 0.011\]],
  [],
)
]

== T3. Rho factorial: what negation and batched inversion each buy (S,
harness price; n per cell)
#align(center)[#table(
  columns: 8,
  align: (col, row) => (right,left,right,right,right,right,right,right,).at(col),
  inset: 6pt,
  [p], [arm], [n], [steps/√ℓ], [S harness], [S tuned], [S⁺ harness],
  [wall s (1 core)],
  [251],
  [R0 plain],
  [16],
  [1.39 \[1, 1.8\]],
  [1.4 \[1, 1.8\]],
  [0.54 \[0.41, 0.68\]],
  [1.59 \[1.2, 2\]],
  [4],
  [251],
  [R1 batched],
  [16],
  [1.24 \[0.94, 1.5\]],
  [1.24 \[0.94, 1.5\]],
  [0.517 \[0.39, 0.64\]],
  [1.41 \[1.1, 1.8\]],
  [3],
  [251],
  [R2 negation],
  [16],
  [1.01 \[0.81, 1.2\]],
  [1.02 \[0.81, 1.2\]],
  [0.394 \[0.31, 0.48\]],
  [1.2 \[0.96, 1.5\]],
  [3],
  [251],
  [R3 negation + batched],
  [16],
  [0.822 \[0.63, 1\]],
  [0.827 \[0.64, 1\]],
  [0.344 \[0.27, 0.42\]],
  [0.975 \[0.74, 1.2\]],
  [2],
  [503],
  [R0 plain],
  [16],
  [1.17 \[0.91, 1.5\]],
  [1.17 \[0.9, 1.5\]],
  [0.453 \[0.35, 0.56\]],
  [1.33 \[1, 1.7\]],
  [26],
  [503],
  [R1 batched],
  [16],
  [1.32 \[1, 1.6\]],
  [1.32 \[1, 1.6\]],
  [0.548 \[0.41, 0.68\]],
  [1.5 \[1.1, 1.9\]],
  [23],
  [503],
  [R2 negation],
  [16],
  [0.971 \[0.71, 1.2\]],
  [0.974 \[0.71, 1.2\]],
  [0.377 \[0.27, 0.48\]],
  [1.15 \[0.84, 1.5\]],
  [24],
  [503],
  [R3 negation + batched],
  [16],
  [0.757 \[0.59, 0.91\]],
  [0.758 \[0.6, 0.91\]],
  [0.315 \[0.25, 0.38\]],
  [0.893 \[0.7, 1.1\]],
  [14],
  [1009],
  [R0 plain],
  [16],
  [1.29 \[1, 1.6\]],
  [1.29 \[1, 1.6\]],
  [0.499 \[0.4, 0.6\]],
  [1.47 \[1.2, 1.8\]],
  [241],
  [1009],
  [R1 batched],
  [16],
  [1.3 \[1.1, 1.6\]],
  [1.3 \[1.1, 1.6\]],
  [0.542 \[0.44, 0.65\]],
  [1.48 \[1.2, 1.8\]],
  [186],
  [1009],
  [R2 negation],
  [16],
  [0.82 \[0.65, 0.99\]],
  [0.822 \[0.65, 1\]],
  [0.318 \[0.25, 0.39\]],
  [0.968 \[0.77, 1.2\]],
  [160],
  [1009],
  [R3 negation + batched],
  [16],
  [0.808 \[0.69, 0.94\]],
  [0.808 \[0.69, 0.94\]],
  [0.336 \[0.29, 0.39\]],
  [0.952 \[0.82, 1.1\]],
  [120],
  [1009],
  [R0 A/A repeat],
  [16],
  [1.25 \[0.96, 1.6\]],
  [1.25 \[0.95, 1.6\]],
  [0.482 \[0.37, 0.6\]],
  [1.42 \[1.1, 1.8\]],
  [229],
  [1009],
  [R3 A/A repeat],
  [16],
  [0.824 \[0.57, 1.1\]],
  [0.823 \[0.58, 1.1\]],
  [0.343 \[0.24, 0.46\]],
  [0.971 \[0.68, 1.3\]],
  [121],
  [1511],
  [R0 plain],
  [8],
  [1.14 \[0.92, 1.4\]],
  [1.14 \[0.91, 1.4\]],
  [0.44 \[0.35, 0.53\]],
  [1.29 \[1, 1.6\]],
  [722],
  [1511],
  [R3 negation + batched],
  [16],
  [0.781 \[0.6, 0.96\]],
  [0.78 \[0.6, 0.96\]],
  [0.325 \[0.25, 0.4\]],
  [0.92 \[0.71, 1.1\]],
  [373],
  [1823],
  [R0 plain],
  [4],
  [1.45 \[0.82, 2.1\]],
  [1.45 \[0.82, 2.1\]],
  [0.56 \[0.32, 0.81\]],
  [1.65 \[0.93, 2.4\]],
  [1609],
  [1823],
  [R3 negation + batched],
  [16],
  [0.887 \[0.71, 1.1\]],
  [0.887 \[0.7, 1.1\]],
  [0.369 \[0.29, 0.45\]],
  [1.05 \[0.83, 1.3\]],
  [760],
)
]

== T4. Dense against sparse FGLM in the sieve\'s descent (paired, same
instances)
#align(center)[#table(
  columns: 8,
  align: (col, row) => (right,right,right,right,right,right,right,right,).at(col),
  inset: 6pt,
  [p], [pairs], [identical descent digest], [identical outcome], [S
  dense/sparse], [linear stage dense/sparse], [descent muls
  dense/sparse], [sparse→dense fallbacks per run],
  [251],
  [16],
  [16],
  [16],
  [1.014 \[1, 1.05\]],
  [2.202 \[2.19, 2.22\]],
  [1.286 \[1.28, 1.29\]],
  [7.50],
  [503],
  [16],
  [16],
  [16],
  [1.059 \[1.04, 1.08\]],
  [2.149 \[2.14, 2.16\]],
  [1.286 \[1.28, 1.29\]],
  [3.75],
  [1009],
  [16],
  [16],
  [16],
  [1.126 \[1.1, 1.15\]],
  [2.091 \[2.08, 2.1\]],
  [1.283 \[1.28, 1.29\]],
  [1.75],
  [1511],
  [16],
  [16],
  [16],
  [1.126 \[1.1, 1.15\]],
  [2.078 \[2.07, 2.08\]],
  [1.285 \[1.28, 1.29\]],
  [1.19],
  [1823],
  [16],
  [16],
  [16],
  [1.096 \[1.07, 1.12\]],
  [2.116 \[2.11, 2.12\]],
  [1.286 \[1.28, 1.29\]],
  [0.44],
)
]

== T5. Exponent of total F\_p multiplications in ℓ
#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,right,left,left,).at(col),
  inset: 6pt,
  [quantity], [slope \[95%\]], [sizes], [status],
  [sieve],
  [-0.122 \[-0.165, -0.084\]],
  [251–1823],
  [pre-registered; theory 1/3],
  [strong rho (harness price)],
  [0.508 \[0.480, 0.538\]],
  [251–1823],
  [pre-registered; theory 1/2 (instrument check)],
  [sieve],
  [0.052 \[-0.015, 0.118\]],
  [1009–1823],
  [#strong[post hoc] (m \= 9 regime), not in the protocol],
  [strong rho],
  [0.498 \[0.420, 0.571\]],
  [1009–1823],
  [#strong[post hoc]],
)
]

Local sieve exponents between adjacent mean sizes: -0.397, -0.199,
0.079, -0.057.

IC A/A (sparse, p \= 1009, run twice): 16/16 instances with identical
counts and digests.

Pre-registered kill rules: K1 fired at p \= none; K2 narrowed at p \=
none.
