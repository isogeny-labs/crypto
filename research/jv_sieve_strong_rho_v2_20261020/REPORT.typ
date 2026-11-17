#set page(paper: "a4", margin: (x: 1.6cm, y: 1.8cm), numbering: "1")
#set text(size: 9pt)
#set table(inset: 3pt, stroke: 0.4pt + luma(160))
#show table: set text(size: 6.5pt)
#align(center)[
  #text(14pt, weight: "bold")[Replication V2: the Joux–Vitse sieve against a strong rho] \
  #text(9pt)[Thresholds fixed before data; acceptance rule A4 failed, every decision withheld. Weak class only; the cover's Jacobian has genus 3. Study `research/jv_sieve_strong_rho_v2_20261020`, seeds 17–32, run 2026-10-20. Every number is copied from `summary.json`, computed by `examples/jv_sieve_study_v2.rs`.]
]
#strong[Outcome: the pre-registered acceptance rule A4 failed, so every
decision is withheld.] Everything else in the acceptance list passed.
The tables below are descriptive; they are not claims.

#strong[Scope: a weak class only.] Curves with full rational 2-torsion
whose isogeny class contains a weak curve, never prime order; the
cover\'s Jacobian has #strong[genus 3]. Class: measurement of an
existing algorithm against a stronger reference. Protocol with numeric
thresholds, committed before any run:
#link("PROTOCOL.md")[`PROTOCOL.md`] (one amendment, below). Numbers:
#link("TABLES.md")[`TABLES.md`], #link("summary.json")[`summary.json`];
raw runs: #link("raw/")[`raw/`]; report:
#link("REPORT.pdf")[`REPORT.pdf`].

== What was run
New instances (seeds 17–32, disjoint from the first study\'s 1–16) at
`p ∈ {251, 503, 1009, 1511, 1823}` (ℓ \= 2^45.8 … 2^63.0), on the
corrected producer of PR \#2281 (DP table keeps `y`, every `hash_x`
evaluation counted, option validation). The producer\'s commit and
binary hash were recorded before the first run (`raw/PROVENANCE.json`:
commit `d688f3c2b`, `jv_cover` SHA-256 `1e63ff73…da848`). Arms, samples,
prices and the §5a band are the first study\'s; 16 runs per size for the
strong rho (R3) and the sieve (dense and sparse), a 2×2 rho factorial, a
plain-rho control, and A/A repeats at `p = 1009`.

== The acceptance rules
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [rule], [requirement], [result],
  [A1],
  [complete seed inventories, every run correct and uncapped],
  [pass],
  [A2],
  [hash-scope completeness per run (2 hash evaluations per step plain, 4
  with negation)],
  [pass (the engine emits exactly 2.0000 and 4.0000)],
  [A3],
  [plain rho reproduces the ledger\'s S ≈ 1.3 (interval contains 1.3,
  point in \[1.0, 1.6\])],
  [pass at all five sizes (points 1.06, 1.33, 1.19, 1.21, 1.28)],
  [#strong[A4]],
  [#strong[rho A/A at p \= 1009: ratio of means in \[0.85, 1.15\] and
  interval contains 1]],
  [#strong[FAIL]],
  [A5],
  [sieve A/A at p \= 1009: identical counts and digests],
  [pass (16/16)],
  [A6],
  [dense and sparse FGLM identical descent digests and outcomes],
  [pass (80/80)],
  [A7],
  [strong-rho exponent point in \[0.45, 0.55\], interval contains 0.5],
  [pass (0.511 \[0.483, 0.540\])],
)
]

#strong[What failed.] R0 first run over repeat: 0.792 \[0.572, 1.073\];
R3: 0.729 \[0.516, 1.035\]. Both intervals contain 1; both point
estimates are outside \[0.85, 1.15\]. Under the protocol a failed
acceptance rule withholds D1–D6, so #strong[no K1/K2 decision, no
exponent classification, no crossover and no acceleration claim is made
from this study], and the first study\'s qualified status is unchanged.

#strong[Why I think the rule was mis-specified, and why I did not change
it.] The ±0.15 window has no power calculation behind it. With a per-run
spread of about 50 % (the A/A arms have coefficients of variation
0.39–0.64) and 16 runs, the standard error of a ratio of two arm means
is about 0.18, so the window is 0.85 standard errors wide: a sound
instrument would fail it about 40 % of the time per arm, about 64 % for
two. The failure is therefore weak evidence of an instrument defect; the
intervals contain 1 and no difference is detected. That is an argument
about the rule, not a licence to relax it after seeing the data: the
protocol says a failed rule is reported as failed, and a changed
threshold is a new protocol version. The cheapest honest next step is a
V3 with a rule sized to the sample (for example, the interval of the
ratio must contain 1, with an equivalence margin of about two standard
errors, or enough A/A runs, roughly 85 per arm for ±0.15), fixed before
a further campaign.

== Amendment (before the final analysis)
A dry run of the analyzer on partial data showed that A1 only inspected
arms that had finished runs, so a scheduled arm with none would have
passed. The analyzer now fails A1 for any absent scheduled arm
(`missing_arms`, unit-tested). No threshold, window, sample or rule
changed; the producer was not rebuilt. Logged at the end of
`PROTOCOL.md`.

== Descriptive results (not decisions)
Sieve/R3 ratios, paired by instance seed, 95 % bootstrap (below 1 is the
sieve cheaper):

#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,right,right,right,right,).at(col),
  inset: 6pt,
  [p], [S, harness], [S, tuned], [S⁺, harness], [S⁺, tuned],
  [251],
  [45.8 \[5.2, 137\]],
  [110 \[13, 332\]],
  [86.4 \[11, 249\]],
  [170 \[22, 496\]],
  [503],
  [0.647 \[0.45, 0.93\]],
  [1.56 \[1.1, 2.2\]],
  [1.78 \[1.2, 2.6\]],
  [3.5 \[2.3, 5.1\]],
  [1009],
  [0.0289 \[0.023, 0.036\]],
  [0.0695 \[0.055, 0.086\]],
  [0.0759 \[0.063, 0.093\]],
  [0.149 \[0.12, 0.18\]],
  [1511],
  [0.00738 \[0.0055, 0.0099\]],
  [0.0177 \[0.013, 0.024\]],
  [0.025 \[0.019, 0.032\]],
  [0.0493 \[0.038, 0.064\]],
  [1823],
  [0.00404 \[0.0033, 0.0051\]],
  [0.00971 \[0.0079, 0.012\]],
  [0.014 \[0.011, 0.018\]],
  [0.0275 \[0.022, 0.036\]],
)
]

For orientation only: the first study (seeds 1–16, earlier producer)
read 0.066 \[0.053, 0.083\], 0.027 \[0.022, 0.036\] and 0.015 \[0.012,
0.019\] for the `S⁺` harness ratio at `p = 1009, 1511, 1823` against
0.076, 0.025 and 0.014 here; its fitted sieve exponents were −0.122 and
0.052 (post hoc, `p ≥ 1009`) against −0.152 \[−0.208, −0.107\] and 0.042
\[−0.020, 0.103\] here; its strong-rho exponent 0.508 against 0.511.
Different seeds and a different producer, so this is a description of
agreement, not a test. At `p = 503` the sieve is below the strong rho in
`S` (0.647 \[0.447, 0.932\]) and above it in `S⁺` (1.78 \[1.18, 2.61\]);
`p = 251` is far above.

In this tower batching shows no saving in the harness price (R1 against
R0, R3 against R2 in #link("TABLES.md") T3), negation lowers `S` (R2
against R0), and sparse FGLM leaves the descent rows identical (80/80)
with the linear stage 2.08–2.20× cheaper and `S` 1.006–1.135× cheaper.
These are descriptions of this sample; the decision rules D5 and D6 were
not evaluated.

== Limits
Weak class only; `p ≤ 1823` (ℓ \< 2^63). The isogeny walk to reach the
class was not re-measured. The tuned price is a cost model that credits
rho only. The sieve\'s F4 additions are not emitted by the harness; the
band uses a labelled upper bound. Wall time is an observation only. One
instance at `p = 251` (seed 22) needed `m = 11` and took 31 minutes per
sieve arm: a heavy tail of the sieve at small `p`, deterministic and
included.

== Figures
#image("figs/ratio_vs_l.svg")

#emph[Figure 1. Sieve over strong rho (R3), paired by seed, four prices;
1 is parity. Study jv\_sieve\_strong\_rho\_v2\_20261020, seeds 17–32,
95% bootstrap intervals; source summary.json. Descriptive: acceptance
rule A4 failed, decisions withheld.]

#image("figs/exponent.svg")

#emph[Figure 2. Mean total F\_p multiplications against ℓ; fitted slopes
in the legend. Study jv\_sieve\_strong\_rho\_v2\_20261020, seeds 17–32,
95% bootstrap intervals; source summary.json. Descriptive: acceptance
rule A4 failed, decisions withheld.]

#image("figs/rho_factorial.svg")

#emph[Figure 3. Rho factorial at p \= 251, 503, 1009, harness price.
Study jv\_sieve\_strong\_rho\_v2\_20261020, seeds 17–32, 95% bootstrap
intervals; source summary.json. Descriptive: acceptance rule A4 failed,
decisions withheld.]

#image("figs/dense_vs_sparse.svg")

#emph[Figure 4. Dense over sparse FGLM in the sieve\'s descent, paired.
Study jv\_sieve\_strong\_rho\_v2\_20261020, seeds 17–32, 95% bootstrap
intervals; source summary.json. Descriptive: acceptance rule A4 failed,
decisions withheld.]

== T0. Acceptance rules (PROTOCOL.md): every decision is withheld unless
all pass
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [rule], [requirement], [result],
  [A1],
  [complete seed inventories, every run correct and uncapped],
  [#strong[pass]],
  [A2],
  [hash-scope completeness per run (2 per step plain, 4 with negation)],
  [#strong[pass]],
  [A3],
  [plain rho reproduces the ledger\'s S ≈ 1.3 (interval contains 1.3,
  point in \[1.0, 1.6\])],
  [#strong[pass]],
  [A4],
  [rho A/A at p \= 1009 (ratio in \[0.85, 1.15\], interval contains 1)],
  [#strong[FAIL]],
  [A5],
  [sieve A/A at p \= 1009 identical counts and digests],
  [#strong[pass]],
  [A6],
  [dense and sparse FGLM identical descent digests and outcomes],
  [#strong[pass]],
  [A7],
  [strong-rho exponent point in \[0.45, 0.55\], interval contains 0.5],
  [#strong[pass]],
)
]

All pass: #strong[False]. Failed: \[\'A4: rho A/A at p\=1009\'\]

== T1. Sieve against the strong rho (R3), paired by instance seed
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
  [37.04 \[4.55, 91.9\]],
  [82.92 \[11.6, 204\]],
  [0.808 / 0.336],
  [0.960 / 0.488],
  [45.83 \[5.17, 137\]],
  [110.2 \[12.5, 332\]],
  [86.41 \[11.1, 249\]],
  [170.1 \[21.8, 496\]],
  [503],
  [51.8],
  [16],
  [0.55 \[0.411, 0.709\]],
  [1.794 \[1.29, 2.35\]],
  [0.850 / 0.354],
  [1.009 / 0.513],
  [0.6473 \[0.447, 0.932\]],
  [1.556 \[1.06, 2.22\]],
  [1.778 \[1.18, 2.61\]],
  [3.498 \[2.34, 5.08\]],
  [1009],
  [57.9],
  [16],
  [0.02038 \[0.0171, 0.024\]],
  [0.06351 \[0.0556, 0.072\]],
  [0.705 / 0.293],
  [0.837 / 0.426],
  [0.02891 \[0.023, 0.036\]],
  [0.06948 \[0.0553, 0.0864\]],
  [0.07586 \[0.0626, 0.093\]],
  [0.1492 \[0.124, 0.183\]],
  [1511],
  [61.4],
  [16],
  [0.006148 \[0.00552, 0.0069\]],
  [0.02478 \[0.0233, 0.0262\]],
  [0.833 / 0.347],
  [0.989 / 0.503],
  [0.00738 \[0.00549, 0.00991\]],
  [0.01773 \[0.0131, 0.0236\]],
  [0.02504 \[0.0195, 0.0323\]],
  [0.04926 \[0.0385, 0.0643\]],
  [1823],
  [63.0],
  [16],
  [0.004013 \[0.00341, 0.00473\]],
  [0.01648 \[0.0149, 0.0181\]],
  [0.993 / 0.413],
  [1.180 / 0.600],
  [0.00404 \[0.00327, 0.00507\]],
  [0.00971 \[0.00786, 0.0122\]],
  [0.01397 \[0.011, 0.0181\]],
  [0.02748 \[0.0218, 0.0358\]],
)
]

== T2. §5a band: sieve/rho with native work priced at w GAE per
operation on every arm
#align(center)[#table(
  columns: 6,
  align: (col, row) => (right,right,right,right,right,right,).at(col),
  inset: 6pt,
  [p], [w], [A, harness], [A, tuned], [B, harness], [B, tuned],
  [251],
  [0.0],
  [45.8 \[5.2, 133\]],
  [110 \[12, 333\]],
  [45.8 \[5.2, 140\]],
  [110 \[13, 321\]],
  [],
  [0.1],
  [272 \[41, 767\]],
  [296 \[44, 863\]],
  [30.8 \[5.1, 89\]],
  [39.4 \[6.6, 109\]],
  [],
  [1.0],
  [304 \[46, 858\]],
  [307 \[46, 895\]],
  [23.2 \[5.1, 64\]],
  [24 \[5.3, 63\]],
  [503],
  [0.0],
  [0.647 \[0.44, 0.93\]],
  [1.56 \[1.1, 2.3\]],
  [0.647 \[0.45, 0.93\]],
  [1.56 \[1.1, 2.2\]],
  [],
  [0.1],
  [7.48 \[5, 11\]],
  [8.13 \[5.5, 12\]],
  [0.804 \[0.55, 1.2\]],
  [1.03 \[0.69, 1.5\]],
  [],
  [1.0],
  [8.45 \[5.7, 12\]],
  [8.53 \[5.7, 13\]],
  [0.883 \[0.59, 1.3\]],
  [0.913 \[0.61, 1.3\]],
  [1009],
  [0.0],
  [0.0289 \[0.023, 0.036\]],
  [0.0695 \[0.056, 0.087\]],
  [0.0289 \[0.023, 0.036\]],
  [0.0695 \[0.055, 0.087\]],
  [],
  [0.1],
  [0.353 \[0.29, 0.43\]],
  [0.384 \[0.32, 0.47\]],
  [0.0314 \[0.026, 0.038\]],
  [0.0401 \[0.033, 0.049\]],
  [],
  [1.0],
  [0.4 \[0.33, 0.49\]],
  [0.403 \[0.33, 0.49\]],
  [0.0326 \[0.027, 0.04\]],
  [0.0337 \[0.028, 0.041\]],
  [1511],
  [0.0],
  [0.00738 \[0.0055, 0.0099\]],
  [0.0177 \[0.013, 0.024\]],
  [0.00738 \[0.0055, 0.0099\]],
  [0.0177 \[0.013, 0.024\]],
  [],
  [0.1],
  [0.119 \[0.093, 0.15\]],
  [0.129 \[0.1, 0.17\]],
  [0.0103 \[0.008, 0.013\]],
  [0.0132 \[0.01, 0.017\]],
  [],
  [1.0],
  [0.135 \[0.1, 0.17\]],
  [0.136 \[0.11, 0.18\]],
  [0.0118 \[0.0093, 0.015\]],
  [0.0122 \[0.0096, 0.016\]],
  [1823],
  [0.0],
  [0.00404 \[0.0033, 0.0051\]],
  [0.00971 \[0.0079, 0.012\]],
  [0.00404 \[0.0033, 0.0051\]],
  [0.00971 \[0.0079, 0.012\]],
  [],
  [0.1],
  [0.067 \[0.053, 0.086\]],
  [0.073 \[0.058, 0.095\]],
  [0.00576 \[0.0046, 0.0075\]],
  [0.00735 \[0.0059, 0.0095\]],
  [],
  [1.0],
  [0.076 \[0.06, 0.098\]],
  [0.0767 \[0.061, 0.1\]],
  [0.00662 \[0.0052, 0.0088\]],
  [0.00684 \[0.0054, 0.009\]],
)
]

== T3. Rho factorial and controls (S, harness price)
#align(center)[#table(
  columns: 8,
  align: (col, row) => (right,left,right,right,right,right,right,right,).at(col),
  inset: 6pt,
  [p], [arm], [n], [steps/√ℓ], [S harness], [S tuned], [S⁺ harness],
  [wall s (1 core)],
  [251],
  [R0 plain],
  [16],
  [1.05 \[0.84, 1.3\]],
  [1.06 \[0.84, 1.3\]],
  [0.408 \[0.33, 0.51\]],
  [1.21 \[0.97, 1.5\]],
  [3],
  [251],
  [R1 batched],
  [16],
  [1.01 \[0.75, 1.3\]],
  [1.01 \[0.74, 1.3\]],
  [0.42 \[0.31, 0.54\]],
  [1.15 \[0.85, 1.5\]],
  [2],
  [251],
  [R2 negation],
  [16],
  [0.648 \[0.46, 0.86\]],
  [0.655 \[0.46, 0.86\]],
  [0.253 \[0.18, 0.33\]],
  [0.777 \[0.55, 1\]],
  [2],
  [251],
  [R3 negation + batched],
  [16],
  [0.803 \[0.59, 1\]],
  [0.808 \[0.6, 1\]],
  [0.336 \[0.25, 0.42\]],
  [0.96 \[0.71, 1.2\]],
  [2],
  [503],
  [R0 plain],
  [16],
  [1.33 \[1, 1.7\]],
  [1.33 \[1, 1.7\]],
  [0.516 \[0.39, 0.65\]],
  [1.52 \[1.2, 1.9\]],
  [33],
  [503],
  [R1 batched],
  [16],
  [1.65 \[1.4, 1.9\]],
  [1.64 \[1.4, 1.9\]],
  [0.684 \[0.56, 0.81\]],
  [1.88 \[1.6, 2.2\]],
  [31],
  [503],
  [R2 negation],
  [16],
  [1.06 \[0.83, 1.3\]],
  [1.06 \[0.83, 1.3\]],
  [0.412 \[0.32, 0.51\]],
  [1.26 \[0.98, 1.5\]],
  [28],
  [503],
  [R3 negation + batched],
  [16],
  [0.849 \[0.7, 1\]],
  [0.85 \[0.69, 1\]],
  [0.354 \[0.29, 0.42\]],
  [1.01 \[0.83, 1.2\]],
  [17],
  [1009],
  [R0 plain],
  [16],
  [1.19 \[0.84, 1.6\]],
  [1.19 \[0.84, 1.6\]],
  [0.46 \[0.32, 0.61\]],
  [1.36 \[0.96, 1.8\]],
  [223],
  [1009],
  [R1 batched],
  [16],
  [1.44 \[1.2, 1.7\]],
  [1.43 \[1.2, 1.7\]],
  [0.596 \[0.49, 0.7\]],
  [1.64 \[1.3, 1.9\]],
  [220],
  [1009],
  [R2 negation],
  [16],
  [0.769 \[0.59, 0.95\]],
  [0.771 \[0.6, 0.95\]],
  [0.298 \[0.23, 0.37\]],
  [0.915 \[0.71, 1.1\]],
  [167],
  [1009],
  [R3 negation + batched],
  [16],
  [0.705 \[0.58, 0.83\]],
  [0.705 \[0.58, 0.83\]],
  [0.293 \[0.24, 0.35\]],
  [0.837 \[0.68, 0.99\]],
  [121],
  [1009],
  [R0 A/A repeat],
  [16],
  [1.5 \[1.1, 1.9\]],
  [1.5 \[1.1, 1.9\]],
  [0.58 \[0.43, 0.74\]],
  [1.71 \[1.3, 2.2\]],
  [301],
  [1009],
  [R3 A/A repeat],
  [16],
  [0.966 \[0.72, 1.2\]],
  [0.966 \[0.72, 1.2\]],
  [0.402 \[0.3, 0.52\]],
  [1.15 \[0.86, 1.5\]],
  [159],
  [1511],
  [R0 plain],
  [8],
  [1.21 \[0.9, 1.5\]],
  [1.21 \[0.9, 1.5\]],
  [0.468 \[0.35, 0.58\]],
  [1.38 \[1, 1.7\]],
  [804],
  [1511],
  [R3 negation + batched],
  [16],
  [0.833 \[0.65, 1.1\]],
  [0.833 \[0.65, 1.1\]],
  [0.347 \[0.27, 0.44\]],
  [0.989 \[0.77, 1.3\]],
  [466],
  [1823],
  [R0 plain],
  [4],
  [1.28 \[0.53, 2.1\]],
  [1.28 \[0.53, 2.1\]],
  [0.495 \[0.2, 0.81\]],
  [1.46 \[0.61, 2.4\]],
  [1476],
  [1823],
  [R3 negation + batched],
  [16],
  [0.993 \[0.74, 1.3\]],
  [0.993 \[0.75, 1.3\]],
  [0.413 \[0.31, 0.53\]],
  [1.18 \[0.88, 1.5\]],
  [957],
)
]

== T4. Dense against sparse FGLM in the sieve\'s descent
#align(center)[#table(
  columns: 7,
  align: (col, row) => (right,right,right,right,right,right,right,).at(col),
  inset: 6pt,
  [p], [pairs], [identical digest], [identical outcome], [S
  dense/sparse], [linear stage dense/sparse], [descent muls
  dense/sparse],
  [251],
  [16],
  [16],
  [16],
  [1.006 \[1, 1.05\]],
  [2.203 \[2.18, 2.24\]],
  [1.286 \[1.28, 1.29\]],
  [503],
  [16],
  [16],
  [16],
  [1.063 \[1.05, 1.08\]],
  [2.151 \[2.14, 2.16\]],
  [1.285 \[1.28, 1.29\]],
  [1009],
  [16],
  [16],
  [16],
  [1.135 \[1.12, 1.15\]],
  [2.099 \[2.09, 2.11\]],
  [1.285 \[1.28, 1.29\]],
  [1511],
  [16],
  [16],
  [16],
  [1.101 \[1.08, 1.12\]],
  [2.077 \[2.07, 2.09\]],
  [1.285 \[1.28, 1.29\]],
  [1823],
  [16],
  [16],
  [16],
  [1.105 \[1.08, 1.13\]],
  [2.11 \[2.1, 2.12\]],
  [1.285 \[1.28, 1.29\]],
)
]

== T5. Exponent of total F\_p multiplications in ℓ
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,right,left,).at(col),
  inset: 6pt,
  [quantity], [slope \[95%\]], [sizes],
  [sieve F1],
  [-0.152 \[-0.208, -0.107\]],
  [251–1823],
  [sieve F2],
  [0.042 \[-0.020, 0.103\]],
  [1009–1823],
  [strong rho, harness],
  [0.511 \[0.483, 0.540\]],
  [251–1823],
  [strong rho, tuned],
  [0.511 \[0.482, 0.540\]],
  [251–1823],
)
]

== T6. Decisions (PROTOCOL.md D1–D6)
#strong[Withheld:] acceptance rules A1-A7 did not all pass. \[\'A4: rho
A/A at p\=1009\'\]
