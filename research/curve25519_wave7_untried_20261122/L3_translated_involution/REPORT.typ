#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set table(inset: 4pt, stroke: 0.4pt + gray)
#show table: set text(size: 7.5pt)

= Wave 7, lead L3: translated involutions and the E\' models
(2026-11-22)
<wave-7-lead-l3-translated-involutions-and-the-e-models-2026-11-22>
#strong[Class.] This is a mechanism measurement: exact coverage counts
on the algebraic (summation-polynomial) track for the family miniatures
k \= 16, 18 and 20. It is not a leaderboard row, it is not a solve, and
it does not change any order against the generic baseline or Pollard
rho. The protocol (#link("PROTOCOL.md")) was frozen before any data
(`FREEZE.sha256`, hash-checked at launch, commit 0e3c0fbda0). The desk
derivations are in #link("DESK.md"), and the run was committed at
e783d12a5b \[pcc:unverified\]. The PDF is #link("REPORT.pdf").

== Question
<question>
The frame theorem covers degree-2 functions invariant under negation.
Every other degree-2 function on E is invariant under some translated
involution P ↦ T − P with T ≠ O, and so is outside the theorem. The Huff
and Jacobi-intersection coordinates of the 2-isogenous curve E\' \= E/
were proposed as such functions.

Do these functions give more exact order-r coverage than x-frames at
equal t?

== Answer
<answer>
#strong[No. No function in this family beats the x-frames, and the only
gain found is a small one on E\' (item 5).] \[pcc:unverified\]

+ #strong[Functions whose T has a rational half are exactly x-frames,
  translated by that half.]
  - Their mixed-sign relations are the x-frame relations.
  - Their same-sign relations are x-frame relations for the shifted
    targets R ∓ T.
  - This was checked frame by frame on 89,966 of 90,000 frames. The
    remaining 34 frames had too few points to test, and none violated
    it. \[pcc:unverified\]
+ #strong[Functions whose T has no rational half are a genuinely
  different F\_p base, but a worse one.]
  - These are half of all T on E and three quarters on E\', and they
    include both Huff coordinates.
  - Within a fibre {P, T − P} no difference lands in the order-r
    subgroup, so these bases lose the relations x-frames get from P −
    (−P).
  - Mean coverage is 0.79–0.81× the x-frame null at t \= 4 on E and
    0.64–0.67× on E\', rising toward 1 as t grows. This matches the
    pair-count model in every cell. \[pcc:unverified\]
  - The best frame chosen by the log-free selector is not better either:
    the geometric means are 0.82–0.97× the selected null.
    \[pcc:unverified\]
+ #strong[The Jacobi-intersection coordinates have degree 4, not 2.]
  - They are x-frames on the other 2-isogenous curves, pulled back to
    E\'.
  - They give 0.48–0.70× the coverage of E\' x-frames at equal t, as
    predicted, before counting their higher polynomial degree.
    \[pcc:unverified\]
+ #strong[The extra signed relations of a translated involution are
  bookkeeping, not a gain.]
  - These are the same-sign sums D ± T, which give about 3× the
    coverage.
  - An x-frame with the same T-shifted targets gets 1.00–1.17× of that
    count.
  - Either way, the decomposition degree in the target is 3× larger.
    \[pcc:unverified\]
+ #strong[The one predicted positive held.]
  - Plain x-frames on E\' cover more than plain x-frames on E:
    1.21–1.24× at t \= 4, 1.10–1.12× at t \= 5 and 1.01–1.02× at t \= 8,
    with confidence intervals excluding 1 at every t. \[pcc:unverified\]
  - The selected E\' frame is 1.12× the selected E frame (geometric mean
    over 9 cells). \[pcc:unverified\]
  - This is a component effect on the algebraic track that fades as 1/N.
    It needs an engine check before anyone relies on it (see \"The one
    positive\").

#image("l3_coverage.png")

The left panel shows k \= 20: mean exact coverage relative to the
matched null, with 95% bootstrap intervals, and the pair-count model
dashed. The right panel shows the coverage of the frame chosen by the
log-free selector, divided by the selected null, for each (k, t ∈ {5, 6,
7}); the shaded band is P6\'s acceptance range.

== Desk results (DESK.md)
<desk-results-deskmd>
`scripts/desk_checks.py` passed 30/30 checks at each of k \= 16, 18 and
20 (`data/desk_k*.json`). \[pcc:unverified\]

- #strong[Curve facts, checked exactly.]

  - p and r are prime, \#E \= 8r by a character sum, E(F\_p) is cyclic,
    and A² − 4 is a non-square.
  - E\' was built with Vélu\'s formulas for kernel : y² \= (x + A)(x −
    2)(x + 2), with φ(x, y) \= (x + 1/x, y − y/x²).
  - \#E\' \= \#E by a character sum, and E\'(F\_p) ≅ Z/2 × Z/4r.
  - φ is a homomorphism mapping E\[r\] injectively into E\'\[r\],
    checked on 2000 samples.
  - So targets transported by φ are covered exactly when their E\'\[r\]
    images are.

- #strong[Every degree-2 f is an ι\_T function with T ∈ E(F\_p).] Its
  four ramification points are the halves of T. This is derived.

- #strong[Theorem 1.] If S ∈ E(F\_p) with 2S \= T, then f \= μ∘x∘τ\_{−S}
  with μ ∈ PGL2(F\_p), and the base is S plus an x-frame base. For the
  targets:

  #align(center)[#table(
    columns: 2,
    align: (col, row) => (left,left,).at(col),
    inset: 6pt,
    [relation], [as an x-frame relation],
    [R \= P\_i − P\_j],
    [x-frame relation for R],
    [R \= P\_i + P\_j],
    [x-frame relation for R − T],
    [R \= −P\_i − P\_j],
    [x-frame relation for R + T],
  )
  ]

  This was checked on every frame of the three control arms: the
  identity holds pointwise and the census equals the x-frame census on
  the matched frame. \[pcc:unverified\]

- #strong[Lemma 2.] For every T, B + B \= T + (B − B), so all signed
  pair sums are D ∪ (D ± T). The mixed-sign coverage depends only on the
  class of T modulo 2E(F\_p). This is derived; P4a tests it.

- #strong[Lemma 3: which T have no rational half.]

  - On E: the T with odd log, equivalently x\_T a non-square, half of
    all points. Checked exactly on 2000 points per curve. T0 does have a
    rational half, so Montgomery y/x is a translated frame.
  - On E\': three of the four classes of E\'/2E\' ≅ (Z/2)², three
    quarters of all points. The class-to-2-torsion map was computed with
    explicit F\_{p²} halving.

- #strong[Theorem 4.] With no rational half, no within-fibre difference
  lies in E\[r\]. This is derived; P1 tests it.

- #strong[Huff (E\').]

  - a x(y² − 1) \= b y(x² − 1) via x \= b(u + a²)/v, y \= a(u + b²)/v;
    the curve equation was checked on every point.
  - Huff x is an ι\_T function for T \= Wa, and Huff y for T \= Wb. Both
    are 2-torsion points outside 2E\', in class (0,1).

- #strong[Jacobi intersection (E\').]

  - αs² + c² \= 1, βs² + d² \= 1, with α \= A + 2, β \= A − 2, checked
    on every point.
  - s, c and d have degree 4, read off the fibre histograms.
  - Their invariance groups show that c and d are x-frames pulled back
    from E\'/\<eβ\> and E\'/\<eα\>.
  - s is a translated E x-frame pulled back by the dual isogeny. That
    last identification is derived from the invariance groups, not
    checked pointwise.

== Corrections to the brief
<corrections-to-the-brief>
- #strong[The halves are defined over F\_{p⁴}, not F\_{p²}.] On E, a T
  with no rational half has no half over F\_{p²}: N(x\_T − e1) \= x\_T²
  + A x\_T + 1 is a non-square, and the Galois argument puts the halves
  over F\_{p⁴}. On E\' the brief\'s F\_{p²} is correct: every T has a
  half there.
- #strong[The Jacobi-intersection coordinates have degree 4.] They are
  therefore not ι\_T functions. The premise holds for Huff.

== Census (exact counts, 2000 random Möbius frames per cell;
`analysis/summary.json`)
<census-exact-counts-2000-random-möbius-frames-per-cell-analysissummaryjson>
Mean covered\_D, the number of distinct points R ≠ O of the order-r
subgroup with R \= P\_i − P\_j. For x-frames this equals `frame_search`
coverage. \[pcc:unverified\]

#align(center)[#table(
  columns: 6,
  align: (col, row) => (left,right,right,right,right,right,).at(col),
  inset: 6pt,
  [k \= 20 arm], [t \= 4], [t \= 5], [t \= 6], [t \= 7], [t \= 8],
  [E x-frame (null)],
  [18.9],
  [71.0],
  [269.9],
  [1043.1],
  [4082.7],
  [E translated frame, T \= 2S (control)],
  [19.2],
  [69.1],
  [266.7],
  [1041.9],
  [4098.1],
  [E Montgomery y/x, T \= T0 (control)],
  [18.9],
  [70.4],
  [266.6],
  [1051.3],
  [4085.7],
  [E odd T, no rational half],
  [15.3],
  [61.7],
  [251.2],
  [1012.2],
  [4022.6],
  [E\' x-frame (null)],
  [23.3],
  [78.2],
  [284.6],
  [1078.3],
  [4162.2],
  [E\' T ∈ 2E\' (control)],
  [22.7],
  [79.4],
  [282.8],
  [1084.7],
  [4128.7],
  [E\' class (0,1), no rational half],
  [15.6],
  [62.3],
  [253.5],
  [1013.2],
  [4044.6],
  [E\' class (1,0), no rational half],
  [15.1],
  [61.3],
  [251.2],
  [1021.3],
  [4036.1],
  [E\' class (1,1), no rational half],
  [14.9],
  [62.0],
  [252.3],
  [1008.8],
  [4018.0],
  [E\' Huff x],
  [15.1],
  [61.3],
  [251.2],
  [1010.7],
  [4034.5],
  [E\' Huff y],
  [15.0],
  [62.5],
  [250.8],
  [1016.2],
  [4018.3],
  [E\' Jacobi-intersection s (degree 4)],
  [11.5],
  [38.8],
  [139.9],
  [541.8],
  [2096.9],
  [E\' Jacobi-intersection c (degree 4)],
  [15.6],
  [47.9],
  [158.6],
  [573.0],
  [2152.4],
  [E\' Jacobi-intersection d (degree 4)],
  [11.2],
  [39.9],
  [140.1],
  [536.4],
  [2098.5],
)
]

The k \= 16 and k \= 18 tables have the same shape
(`analysis/summary.json`). \[pcc:unverified\]

Ratio to the matched null, with 95% bootstrap intervals and the model in
brackets, at k \= 20: \[pcc:unverified\]

#align(center)[#table(
  columns: 5,
  align: (col, row) => (left,left,left,left,left,).at(col),
  inset: 6pt,
  [comparison], [t \= 4], [t \= 5], [t \= 6], [t \= 8],
  [E odd T / E x-frame],
  [0.808 (0.780–0.839) \[0.790\]],
  [0.869 (0.849–0.890) \[0.886\]],
  [0.931 (0.916–0.947) \[0.940\]],
  [0.985 (0.978–0.993) \[0.985\]],
  [E\' Huff x / E\' x-frame],
  [0.646 (0.625–0.669) \[0.652\]],
  [0.784 (0.766–0.802) \[0.795\]],
  [0.883 (0.869–0.896) \[0.888\]],
  [0.969 (0.962–0.976) \[0.970\]],
  [E\' Jacobi-intersection s / E\' x-frame],
  [0.492 (0.469–0.512) \[0.500\]],
  [0.496 (0.482–0.512) \[0.500\]],
  [0.492 (0.481–0.502) \[0.500\]],
  [0.504 (0.499–0.509) \[0.504\]],
  [E\' x-frame / E x-frame],
  [1.236 (1.196–1.275) \[1.210\]],
  [1.102 (1.078–1.126) \[1.114\]],
  [1.055 (1.038–1.072) \[1.060\]],
  [1.020 (1.012–1.027) \[1.015\]],
)
]

== Predictions and controls (all decided by the frozen
`scripts/analyse.py`)
<predictions-and-controls-all-decided-by-the-frozen-scriptsanalysepy>
#strong[Calibration.]

- #strong[CAL1 held, 30/30.] The bits\_lo0 and edwards frames reproduce
  `frame_search` exactly. \[pcc:unverified\]
- #strong[CAL2 held, 15/15.] The x-frame null means are within 3 SE of
  `frame_search`\'s Möbius means. \[pcc:unverified\]
- #strong[CAL3 held, 30/30.] The model matches both nulls to within
  0.990–1.012. \[pcc:unverified\]

#strong[Controls.]

- #strong[C1 held, 89,966/90,000 frames, with 0 violations.]
  \[pcc:unverified\]
  - All 34 exceptions are code 2, all at t \= 4: frames with fewer than
    3 fibres, where the Möbius map cannot be fitted.
  - That is 0.04% of frames against the protocol\'s 1% allowance.
  - No frame gave code 0, which would have been a violation.
- #strong[C2 held, 45/45.] Control means relative to the null are
  0.973–1.025. \[pcc:unverified\]
- #strong[A/A held, 28/30.] \[pcc:unverified\]
  - The two intervals that exclude 1 are k \= 20, t \= 4, E\' x-frame
    (1.073, CI 1.026–1.121) and k \= 20, t \= 6, E x-frame (0.971, CI
    0.951–0.993).
  - Two misses in 30 is in line with the 5% nominal rate, and the
    threshold was ≥ 27.
- #strong[PC1 held, 24/24.] The planted removal of within-fibre pairs
  was detected at t \= 4–6 in every k (CI upper bound ≤ 0.992), and it
  matches the model in 15/15 cells. \[pcc:unverified\]
- #strong[PC2 held, 9/9.] The frame-search selection gain was
  reproduced: 2.62–2.75 at t \= 5, 1.98–2.18 at t \= 6 and 1.55–1.72 at
  t \= 7. \[pcc:unverified\]

#strong[Predictions.]

- #strong[P1 held, 36/36 direction and 90/90 model cells.] No-half arms
  sit below the null at t \= 4 and 5, and every cell is within tolerance
  of the model. \[pcc:unverified\]
- #strong[P2 held, 6/6 direction and 15/15 model cells.] E\' x-frames
  beat E x-frames at t \= 4 and 5, and every cell matches the model.
  \[pcc:unverified\]
- #strong[P3 held, 27/27 direction and 45/45 model cells.]
  Jacobi-intersection coverage is about half. \[pcc:unverified\]
- #strong[P4a held, 36/36.] Huff x, Huff y and class (0,1) have the same
  distribution: mean ratios within 3.7% of 1 and KS distance at most
  0.0585, against a critical value of 0.0617. \[pcc:unverified\]
- #strong[P4b held, 36/36.] The three no-half classes are
  indistinguishable: pairwise ratios 0.984–1.017. \[pcc:unverified\]
- #strong[P5 held, 60/60.] The full signed set matches the T-shifted
  x-null model. Full ÷ mixed-sign coverage is 2.0–4.1× (the low end is
  the saturated cell k \= 16, t \= 8). \[pcc:unverified\]
- #strong[P6 held, 6/6.] The selected no-half frame divided by the
  selected null frame has geometric means of 0.971 (E odd T), 0.853,
  0.836 and 0.884 (E\' classes), and 0.865 and 0.815 (Huff x, Huff y).
  The single-cell maximum is 1.13. \[pcc:unverified\]
- #strong[P7 held.] E\' selected over E selected has a geometric mean of
  1.117, with cells ranging 1.03–1.39. \[pcc:unverified\]

== Derived vs measured
<derived-vs-measured>
#strong[Derived (exact algebra, with the numerical checks listed
above):] Theorem 1, Lemmas 2–3, Theorem 4, the Huff and
Jacobi-intersection invariance groups, the field of definition of the
halves, the pair-polynomial multidegree (2,2,2) for mixed-sign
relations, and the 3× target degree of the full signed set. The last two
are sketches and are not engine-tested.

#strong[Measured:] exact integer coverage counts per frame. There are no
timings and no F4 runs.

== The one positive: E\' x-frames over E x-frames
<the-one-positive-e-x-frames-over-e-x-frames>
#strong[What was measured.] Mean coverage is up 21% at t \= 4 and 11% at
t \= 5. The best selected frame is up about 12% on geometric mean. This
is a measured component effect of coverage alone. \[pcc:unverified\]

#strong[Why.] E\' has full rational 2-torsion, so 4 of its 8 cosets are
self-paired under negation, against 2 of 8 on E. That adds about N/4
within-fibre relations.

#strong[Why it needs an engine check before anyone relies on it.]

+ Coverage is not cost. Whether F4 on E\' has the same solving degree
  and polynomial size as on E has not been measured. Only the
  multidegree argument says it should.
+ The gain fades as 1/N with N ≈ 2^{t−1}: it is 1.5% at t \= 8, and it
  has no bearing at 255 bits.
+ The isogeny transport, a few field operations per target and per base
  point, has to be charged.
+ The selected-frame ratio is noisy, ranging 1.03–1.39 across cells.

The check needed is a `pkm-semaev` run on E\' against E, with the same
seeds, in the engine.

== Distance to rho (imported, not measured here)
<distance-to-rho-imported-not-measured-here>
These figures are from
`research/curve25519_frame_search_20261103/README.md`, which imports
`curve25519_algebraic_ecbench_20261102`. \[pcc:unverified\]

- The generic baseline is 7.8× rho at k \= 20.
- The best algebraic arm against rho-neg is 18× at w \= 0, 9.6·10⁵× at w
  \= 0.1 and 5.9·10⁶× at w \= 1.

Coverage differences of the size found here, ≤ 1.24× in mean, do not
change any of those orders across the §5a band. \[pcc:unverified\]

== Data and reproduction
<data-and-reproduction>
- The binary per-cell outputs `data/k*/*.npz` exceed the 64 KB data-file
  policy and are git-ignored.
  - Their sha256 values are in `data/SHA256SUMS`.
  - Lossless JSON mirrors are in `data_json/`, with sums in
    `data_json/SHA256SUMS`, written by `scripts/export_json.py`.
  - `export_json.py --check` reloads every mirror. It reproduces all
    1470 arrays of the 45 files exactly (`data_json/CHECK.txt`).
    \[pcc:unverified\]
- Run with `cd scripts && ./run_all.sh`: about 20 minutes on one core,
  with a peak RSS of 253 MB.
- Make the figure with `python3 scripts/figure.py`.

== Disclosures
<disclosures>
- No Conductor task, no EKS route, and no S3 resume checkpoint: the
  upload token had expired, as recorded in `run.log`. No `bench.record`
  and no pcc claim were filed.
- The host was shared with four other lead agents. Every quantity is an
  exact count, so timing noise does not enter.
- The research knowledge base and the lead atlas were unreachable.
- Sources checked (repository files actually read):
  - `research/curve25519_wave7_untried_20261122/BRIEF.md`
  - `research/curve25519_frame_search_20261103/` README.md, PROTOCOL.md,
    scripts/frame\_census.py, scripts/analyse.py,
    scripts/run\_confirm\*.sh and analysis/summary.json
  - `research/curve25519_autolab_20261104/ideas/STRUCTURE-R4.md`: lines
    154–155 and record R4-I2-linear-2torsion-quotient-family

== Errata
<errata>
(none)

#pagebreak()
= Appendix: desk derivations (DESK.md)

== L3 desk part: translated involutions and the models closed on E
(2026-11-22)
<l3-desk-part-translated-involutions-and-the-models-closed-on-e-2026-11-22>
Status: derivations, each with an exact numerical check at k \= 16 in
`smoke/desk_k16.json` (`scripts/desk_checks.py`, 30/30 checks pass,
smoke, not data) \[pcc:unverified\]. The same script runs at k \= 16,
18, 20 in the full run.

== 0. Setting
<0-setting>
- E: y^2 \= x^3 + A x^2 + x over F\_p, \#E \= 8r with r prime, E(F\_p) ≅
  Z/8r (cyclic), T0 \= (0, 0). A^2 − 4 is a non-square, so T0 is the
  only rational 2-torsion point.
- E\' \= E/, built with Vélu\'s formulas (kernel point (0,0), v \= 1):
  y^2 \= x^3 + A x^2 − 4x − 4A \= (x + A)(x − 2)(x + 2), with φ(x, y) \=
  (x + 1/x, y − y/x^2). We use X \= x + A, so that Y^2 \= X(X − α)(X −
  β), with α \= A + 2 and β \= A − 2.
  - \#E\' \= \#E, by a character sum.
  - E\'(F\_p) ≅ Z/2 × Z/4r, and all of E\'\[2\] is rational.
  - φ is a homomorphism and maps E\[r\] injectively into E\'\[r\]. This
    was checked on 2000 samples (checks `phi_*`).
- G2 is the 2-part of the group: Z/8 on E, and Z/4 × Z/2 on E\'. The
  coset of P is its image in G2. The order-r subgroup is coset 0.
- An x-frame base is B\_x(V) \= {P : x(P) ∈ V}, where V \= ν(\[0, 2^t))
  for a Möbius map ν. This is `frame_search`\'s base. Its coverage is
  the number of distinct R ≠ O in E\[r\] with R \= ±P\_i ± P\_j.

== 1. Every degree-2 function is an ι\_T function, and T is rational
<1-every-degree-2-function-is-an-ι_t-function-and-t-is-rational>
Let f: E → P^1 have degree 2 and be defined over F\_p.

- All fibres of f are linearly equivalent degree-2 divisors, so the
  group sum of a fibre is a constant T. The fibre over ∞ is
  Galois-stable, so T ∈ E(F\_p).
- f is therefore invariant under ι\_T: P ↦ T − P, and F\_p(E)^{ι\_T} \=
  F\_p(f).
- By Riemann–Hurwitz f has 4 ramification points. They are the points
  with 2P \= T, the four halves of T.

T \= O gives the x-frames, where ι\_O \= negation.

Explicit generator. For T ∉ E\[2\], h\_T(P) is the slope of the line
through −T and P. For T ∈ E\[2\], h\_T(P) \= y/(x − x\_T). Since P, T −
P and −T are collinear, h\_T is ι\_T-invariant. It has degree 2, with
poles at O and T. Every ι\_T function equals μ∘h\_T with μ ∈ PGL2(F\_p).
The census builds bases of μ∘h\_T by solving the line–cubic intersection
(`curves.slope_base`).

== 2. Theorem 1: a rational half makes it a translated x-frame
<2-theorem-1-a-rational-half-makes-it-a-translated-x-frame>
Suppose S ∈ E(F\_p) with 2S \= T. Then x∘τ\_{−S}, i.e. P ↦ x(P − S), is
ι\_T-invariant, because (T − P) − S \= −(P − S). It is defined over F\_p
and has degree 2. So #strong[f \= μ∘x∘τ\_{−S}, with μ ∈ PGL2(F\_p).]

#strong[Base.] B\_f(V) \= S + B\_x(μ^{-1}V): the base of f is the
x-frame base for frame μ^{-1}∘ν, translated by S.

#strong[Relations.] Write P\_i \= S + Q\_i, where B\_x is closed under
negation.

#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [relation on the f-base], [equals], [as an x-frame relation],
  [R \= P\_i − P\_j],
  [Q\_i − Q\_j],
  [target R itself],
  [R \= P\_i + P\_j],
  [T + (Q\_i + Q\_j)],
  [target R − T],
  [R \= −P\_i − P\_j],
  [−T − (Q\_i + Q\_j)],
  [target R + T],
)
]

So the mixed-sign relations of a translated frame are exactly the
x-frame relations, with no shift. The same-sign relations are x-frame
relations for the shifted targets R ∓ T. As sets:

- D\_f :\= B\_f − B\_f \= Σ\_x :\= B\_x ± B\_x.
- All signed pair sums of B\_f \= Σ\_x ∪ (Σ\_x + T) ∪ (Σ\_x − T).

An x-frame user covers that union by decomposing R, R − T and R + T,
which is three decompositions of the same degree. Counting it as one
relation search of a \"new coordinate\" is relabelling.

Check `E_translated_frame_identity` / `Ep_translated_frame_identity`:
for 5 random T \= 2S per curve, B\_f − S is closed under negation, and
h\_T \= M∘x∘τ\_{−S} holds on every base point for a fitted Möbius M. In
the census, `control_check` repeats this for every frame of the three
control arms. There the counts must equal the x-frame census on frame
M^{-1}(V) exactly.

== 3. When there is no rational half
<3-when-there-is-no-rational-half>
#strong[Lemma 2 (holds for every T).] B\_f \= T − B\_f, so B\_f + B\_f
\= T + D\_f. All signed pair sums are therefore D\_f ∪ (D\_f + T) ∪
(D\_f − T).

D\_f is unchanged under f ↦ f∘τ\_R for R ∈ E(F\_p): the base becomes
B\_f − R, which has the same differences, and T moves to T − 2R. So
#strong[the mixed-sign coverage of an ι\_T function depends only on the
class of T in E(F\_p)/2E(F\_p) and on the Möbius frame], up to
translation.

#strong[Lemma 3: which T have a rational half.]

- #strong[E.] E(F\_p)/2E(F\_p) ≅ Z/2.
  - T ∉ 2E(F\_p) ⇔ log T is odd ⇔ x\_T is a non-square. This is half of
    all points. The reason is that 2E(F\_p) \= φ̂(E\'(F\_p)), which is
    the kernel of P ↦ x\_P mod squares.
  - T0 \= 4r·g lies in 2E(F\_p): its halves are the order-4 points with
    x \= ±1. So the Montgomery coordinate y/x \= h\_{T0} is a translated
    frame.
  - For odd T, no half lies in E(F\_{p^2}). With E\[2\] ⊂ E(F\_{p^2}),
    the halving criterion asks whether x\_T − e\_i is a square in
    F\_{p^2} for each i. For e\_1 ∉ F\_p, N(x\_T − e\_1) \= x\_T^2 + A
    x\_T + 1 \= y\_T^2/x\_T, a non-square.
  - Galois argument: e(S) \= π(S) − S takes the values {e, e, e + T0, e
    + T0} on the four halves. With no rational half, e is an irrational
    2-torsion point, and π^2(S) \= S + e + π(e) \= S + T0 ≠ S. So the
    halves are defined over F\_{p^4}.
  - #strong[Correction to the brief:] on E the translation is defined
    over F\_{p^4}, not F\_{p^2}.
  - Checks `E_half_iff_log_even_iff_xT_square`,
    `E_odd_T_no_half_over_Fp2`.
- #strong[E\'.] E\'(F\_p)/2E\'(F\_p) ≅ (Z/2)^2. Encode T \= k·g + ε·h;
  its class is (k mod 2, ε).
  - T ∈ 2E\'(F\_p) ⇔ class (0,0) ⇔ X\_T − e\_i is a square for all three
    e\_i. That covers 1/4 of the points.
  - Every T has a half S ∈ E\'(F\_{p^2}), because every element of F\_p
    is a square in F\_{p^2}.
  - The Kummer map class ↦ π(S) − S ∈ E\'\[2\] is a bijection.
  - At k \= 16, in the X coordinate: (0,1) ↦ (A+2, 0), (1,0) ↦ (A−2, 0),
    (1,1) ↦ (0,0). This is computed with explicit F\_{p^2} halving
    (`half_over_fp2`).
  - Checks `Ep_half_iff_class00_*`, `Ep_every_T_has_half_over_Fp2_*`,
    `Ep_class_to_e_injective`.

#strong[What base and relations look like then.]

- f is defined over F\_p. Its base is still a translate: B\_f \= S + B̃
  with B̃ \= {Q : x(Q) ∈ μ^{-1}V}. But S, B̃ and μ live over F\_{p^4} on E
  (F\_{p^2} on E\').
- The frame is twisted by the 2-torsion: Frobenius moves Q to Q − e, so
  μ^σ \= m\_e∘μ, where m\_e is the Möbius action of τ\_e on x.
- No F\_p x-frame has the same difference set in general.
- The relation table of §2 still holds with S over the extension: D\_f
  \= B̃ − B̃ ⊂ E(F\_p).

#strong[Theorem 4: the within-fibre term vanishes.]

- A fibre {a, T − a} gives the difference a − ι\_T a \= 2a − T. It lies
  in E\[r\] iff 2·coset(a) \= coset(T). That has a solution only when
  coset(T) ∈ 2G2, i.e. exactly when T has a rational half.
- So #strong[an ι\_T function with no rational half has no within-fibre
  relations.] An x-frame has them whenever 2·coset(a) \= 0, i.e. P −
  (−P) \= 2P ∈ E\[r\]. On E that happens for 2 of the 8 cosets. On E\'
  it happens for 4 of the 8, because of the extra rational 2-torsion.
- Cross-fibre relations have the same expected count in both cases (§4).
- Prediction: a no-half ι\_T base has strictly lower mean coverage. The
  deficit is about N/2 points on E and about N points on E\', out of
  N^2/4, where N ≈ 2^{t−1} fibres.
- E\' x-frames have about N/4 more than E x-frames.

#strong[Degree of the pair polynomial (sketch, not engine-tested).]

- For a no-half ι\_T base with the target read in x, the mixed-sign set
  gives 2 values of x(R) per pair (x(a − b) and x(a + b − T)). Given
  x(R) and f\_2, it gives 2 values of f\_1. So the relation polynomial
  Φ(f\_1, f\_2, x\_3) has multidegree (2, 2, 2), the same as S3.
- The full signed set gives 12 points per pair: 6 values of x(R), i.e.
  3× the target degree for about 3× the coverage (model ratio full/D \=
  3.0–3.5 below saturation). It is reported, not counted as a gain.

== 4. The named E\' models
<4-the-named-e-models>
#strong[Huff.]

- A Huff model exists over F\_p: a x(y^2 − 1) \= b y(x^2 − 1). It comes
  from W: v^2 \= u(u + a^2)(u + b^2), with u \= X − e0, where e0 is the
  unique 2-torsion abscissa in 2E\'.
- The map is x \= b(u + a^2)/v, y \= a(u + b^2)/v (derived here; check
  `huff_equation_all_points`).
- x has simple poles at W0 and Wb, so it is an ι\_T function with T \=
  Wa. Likewise y has T \= Wb.
- Both T are 2-torsion points outside 2E\'(F\_p), and both lie in class
  (0,1). They have halves only over F\_{p^2}. Checks `huff_degree_2`,
  `huff_*_iota_T_with_T_2torsion_not_in_2E`.
- By Lemma 2, Huff x, Huff y and any class-(0,1) ι\_T function with a
  random Möbius frame have the same D-coverage distribution. The
  brief\'s premise holds for Huff.

#strong[Jacobi intersection: the premise fails.]

- αs^2 + c^2 \= 1, βs^2 + d^2 \= 1, with s \= 2Y/(X^2 − αβ), c \= (X^2 −
  2αX + αβ)/(X^2 − αβ), d \= (X^2 − 2βX + αβ)/(X^2 − αβ).
- These were derived from the JI doubling formulas (check
  `ji_equations_all_points`).
- s, c and d all have #strong[degree 4] (fibre sizes 4 or 0; check
  `ji_degree_4`), so they are not ι\_T functions. Each is invariant
  under a Klein four group, read off the fibres:
  - c: {id, −1, τ\_{eβ}, ι\_{eβ}}. So c \=
    Möbius∘x\_{E\'/\<eβ\>}∘φ\_{eβ}, an x-frame pulled back from another
    curve of the 2-isogeny graph.
  - d: the same with eα.
  - s: {id, τ\_{(0,0)}, ι\_{eα}, ι\_{eβ}}. (0,0) \= ker φ̂, so s \= g∘φ̂
    for a degree-2 function g on E that is ι\_{T0}-invariant, since
    φ̂(eα) \= T0.
- #strong[JI s is an E translated x-frame (by an order-4 point S ∈ 2E)
  pulled back to E\' by the dual isogeny.] Its mixed-sign coverage of
  E\'\[r\] is the E x-frame coverage restricted to base points in
  2E(F\_p). This last identity is derived, not separately checked
  pointwise.
- Degree-4 consequence:
  - Each value carries an orbit of 4 points with probability 1/4.
  - Each difference value is repeated 4 times rather than 2.
  - Cross-orbit relations number about N4^2/2 with N4 ≈ 2^{t−2}, against
    N^2/4 with N ≈ 2^{t−1}.
  - Prediction: #strong[about half the coverage at equal t], on top of a
    higher-degree summation polynomial.

== 5. Pair-count model (`scripts/model.py`)
<5-pair-count-model-scriptsmodelpy>
- #strong[Degree 2.] N ~ Bin(2^t, n/2p) fibres, with cosets uniform and
  independent.
  - The distinct differences in coset z number E\[N(N−1)\]/4 +
    E\[N\]·w2(z), with w2(z) \= (\#{2c \= t\_T + z} + \#{2c \= t\_T −
    z})/8.
  - The full set adds the cosets z \= ∓t\_T (shifted by ±T).
- #strong[Degree 4.] E\[N4(N4−1)\]/2 + E\[N4\]·w4, with N4 ~ Bin(2^t,
  n/4p).
  - w4 \= 1 for s. For c and d, w4 \= 1 + \[e ∈ 2E\'\].
- Saturation: (r−1)(1 − e^{−M/(r−1)}).
- The model reproduces `frame_search`\'s Möbius-frame means to about 1%:
  19.1 / 70.2 / 265.5 / 990.4 / 3271.6 predicted at k \= 16, against
  19.2 / 70.0 / 266.3 / 988.7 / 3266.5 measured (`frame_search`
  `analysis/summary.json`, `mobius_mean`) \[pcc:unverified\].

== 6. What the desk part decides before any census
<6-what-the-desk-part-decides-before-any-census>
- #strong[ι\_T with a rational half (T ∈ 2E(F\_p)).] This includes the
  E\' translates and Montgomery y/x. They are translated x-frames over
  F\_p, so they are controls. Their mixed-sign census equals the x-frame
  census frame by frame, and the extra signed relations are x-frame
  relations for R ∓ T.
- #strong[ι\_T with no rational half.] This is half of the T on E and
  three quarters on E\', and includes the Huff coordinates. They lie
  outside the F\_p frame theorem: the translation exists only over
  F\_{p^4} (E) or F\_{p^2} (E\'). They are a genuinely different F\_p
  factor base. The model predicts they are #strong[worse] at equal F4
  degree, since they lose the within-fibre relations.
- #strong[JI coordinates.] They are degree-4 pullbacks of x-frames on
  2-isogenous curves (for s, the curve E itself), predicted at about
  0.5× coverage.
- #strong[The only predicted positive difference is E\' x-frames over E
  x-frames.] E\' has 4 self-paired cosets instead of 2. The predicted
  mean gain is 1.21 / 1.11 / 1.06 / 1.03 / 1.01 at t \= 4..8. It is a
  small component effect on the algebraic track and fades as 1/N. The
  isogeny transport of targets costs a few field operations per point.
