#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9pt)
#set table(inset: 3pt)
= Genus-3 GHS covers of binary curves over F\_{2^{3l}}: census and
subgroup shape
<genus-3-ghs-covers-of-binary-curves-over-f_23l-census-and-subgroup-shape>
Study `ghs_n3_genus3_census_20261024`, run 2026-10-24. Protocol:
#link("PROTOCOL.md")[`PROTOCOL.md`] (frozen before data; sha256
`7401a1de746a6b516785a5f370aa286c29bf5d4d8f5195d8124f093d2b87b642`,
unchanged after the run). Tool:
#link("tool/census.rs")[`tool/census.rs`]. Raw output: `data/`. Every
number is labelled #strong[derived] (from the algebra in PROTOCOL.md
section 1) or #strong[measured] (computed by the tool). Toy fields only
(N \= 3l \<\= 21); no claim about any deployed curve, no DLP cost or
speed claim.

#figure([#image("graph.svg")],
  caption: [
    class fractions and large-prime rate
  ]
)

#emph[Figure. Left: fraction of b in K] by GHS class vs l (measured,
exhaustive; the derived closed forms coincide with every point). Right:
P(largest prime factor of \#E \> sqrt \#E) for a \= 0, class B vs
generic b vs null integers (measured; Wilson 95%). Source data:
`data/q1_census.csv`, `data/q3_shaped.csv`; run log `data/run.log`.\*

== Reproduction
<reproduction>
```sh
cd /home/user/crypto   # target/release built beforehand (library + ghs_screen)
rustc --edition 2021 -O research/ghs_n3_genus3_census_20261024/tool/census.rs \
  -o /tmp/census --extern crypto_lib=target/release/deps/libcrypto_lib-2ea5067956474c6e.rlib \
  --extern num_bigint=target/release/deps/libnum_bigint-72cf526ce00d5673.rlib \
  -L dependency=target/release/deps
/tmp/census research/ghs_n3_genus3_census_20261024/data target/release/ghs_screen
```

Single thread, 2 min 26 s wall on a shared 4-core host. No file in
`src/`, `examples/` or `Cargo.toml` was touched. The library calls are
`ghs_genus_with_type`, `magic_number`, `mmt_condition_8`,
`relative_trace`, `absolute_trace`. The tool reimplements only exp/log
tables, the point count, factoring, Wilson intervals and the SVG.

== Q1. Closed form for the (m, type, genus) census (derived, verified)
<q1.-closed-form-for-the-m-type-genus-census-derived-verified>
For n \= 3 the Frobenius sigma \= 2^l-power has order 3, and x^3 - 1 \=
(x+1)(x^2+x+1) splits K \= k (+) ker Tr\_{K/k} as F\_2\[sigma\]-modules
(dimensions l and 2l). The annihilator of sqrt(b) is the product of
(x+1) if Tr\_{K/k}(b) !\= 0 and (x^2+x+1) if sqrt(b) has a nonzero
trace-zero part. So only m in {1, 3} occurs (never m \= 2 or m \>\= 4):

#align(center)[#table(
  columns: 6,
  align: (col, row) => (auto,auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [class], [condition on b], [m], [type], [genus], [count (derived)],
  [A],
  [b in F\_{2^l}\*],
  [1],
  [II],
  [1],
  [2^l - 1],
  [#strong[B]],
  [Tr\_{K/k}(b) \= 0, b !\= 0],
  [3],
  [I],
  [#strong[3]],
  [2^{2l} - 1],
  [C],
  [otherwise],
  [3],
  [II],
  [4],
  [(2^l - 1)(2^{2l} - 1)],
)
]

Measured (exhaustive over all 2^N - 1 values of b, library
classification):

#align(center)[#table(
  columns: 7,
  align: (col, row) => (auto,auto,auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [l], [N], [A obs/derived], [B obs/derived], [C obs/derived], [B
  fraction], [per-b deviations],
  [2],
  [6],
  [3 / 3],
  [15 / 15],
  [45 / 45],
  [23.8%],
  [0],
  [3],
  [9],
  [7 / 7],
  [63 / 63],
  [441 / 441],
  [12.3%],
  [0],
  [4],
  [12],
  [15 / 15],
  [255 / 255],
  [3825 / 3825],
  [6.23%],
  [0],
  [5],
  [15],
  [31 / 31],
  [1023 / 1023],
  [31713 / 31713],
  [3.12%],
  [0],
  [6],
  [18],
  [63 / 63],
  [4095 / 4095],
  [257985 / 257985],
  [1.56%],
  [0],
  [7],
  [21],
  [127 / 127],
  [16383 / 16383],
  [2080641 / 2080641],
  [0.78%],
  [0],
)
]

#strong[P1 held]: zero deviations for every b; the library’s class
equals the predicate (b in k) / (Tr\_{K/k} b \= 0) for each of the 2^N -
1 values at every l. The l \= 4 census (15 / 255 / 3825) is reproduced.
The 6% therefore falls like 2^{-l}: by l \= 7 only 0.78% of b are genus
3.

== Q2. MMT condition (8) (derived; measured to agree)
<q2.-mmt-condition-8-derived-measured-to-agree>
Condition (8) is (n odd) or Tr(a)\=0 or Tr\_{K/k}(b) !\= 0; n \= 3 is
odd. #strong[P2 held]: 100% of genus-3 b satisfy it for a \= 0 and for
Tr\_{K/F2}(a) \= 1, at every l \= 2..7 (`data/q2_cond8.csv`; also 100%
in classes A and C). For n \= 3 the descent condition is vacuous, so
this is not a sieve.

Detector positive control (n \= 2, derived expectation (2^{2l} - 2^l) of
2^{2l} - 1 for Tr(a)\=1, 100% for a\=0): measured l\=3: 56/63, l\=4:
240/255, l\=5: 992/1023, a\=0: all true. All three equal the derived
counts, so the tally can see a failing condition when one exists (P2c
held).

== Q3. Do the class-B curves have a cryptographically shaped subgroup?
<q3.-do-the-class-b-curves-have-a-cryptographically-shaped-subgroup>
#strong[Deviation from the brief (see PROTOCOL.md section 3).] "r does
not divide \#E(F\_{2^l})" is undefined for class B: b is not in k, so E
has no model over k. Evaluated instead (measured): S1 \= largest prime
r\_max of \#E(K) exceeds sqrt(\#E); S2 \= cofactor \#E/r\_max \<\= 4.
The literal criterion is evaluated only for b in k (arm D, where E is
defined over k): there it coincides with S1 in every row
(`data/run.log`, "Arm D"), because \#E(k) \<\= q + 1 + 2 sqrt q \<
q^{3/2} \< sqrt \#E(K) (derived), so no prime above sqrt \#E(K) can
divide \#E(k). The trace constraint is never what removes a large prime;
the question is only the shape of \#E(K) itself.

Point counts: exhaustive over x (trace formula, PROTOCOL.md section 3),
checked by a brute-force affine-point count at l \= 2, 3 (every b, both
a: 0 mismatches), and for every class-B curve at every l by Hasse bound
(0 violations), \#E even, \#E mod 4 equal to 0 (a\=0) / 2 (Tr a\=1) (0
violations), twist sum 2\*2^N+2 (0), and Frobenius-orbit count equality
(0).

S1 and S2, measured (a \= 0 and Tr(a)\=1). B is over all class-B b; "B
reps" is over one b per Frobenius orbit (the independent sample, Wilson
95%); G is 20000 uniformly random b; I is 20000 integers in the Hasse
interval with the same \#E mod 4.

#align(center)[#table(
  columns: 9,
  align: (col, row) => (auto,auto,auto,auto,auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [l], [a], [S1 B (n)], [S1 B reps \[95%\]], [S1 G], [S1 I], [S2 B], [S2
  G], [S2 I],
  [4],
  [a\=0],
  [0.518 (255)],
  [0.478 \[0.292,0.670\]],
  [0.516],
  [0.581],
  [0.000],
  [0.151],
  [0.157],
  [4],
  [Tr\=1],
  [0.647 (255)],
  [0.652 \[0.449,0.812\]],
  [0.695],
  [0.727],
  [0.377],
  [0.260],
  [0.301],
  [5],
  [a\=0],
  [0.396 (1023)],
  [0.391 \[0.285,0.509\]],
  [0.537],
  [0.595],
  [0.000],
  [0.070],
  [0.104],
  [5],
  [Tr\=1],
  [0.575 (1023)],
  [0.580 \[0.462,0.689\]],
  [0.678],
  [0.741],
  [0.147],
  [0.130],
  [0.185],
  [6],
  [a\=0],
  [0.532 (4095)],
  [0.524 \[0.460,0.587\]],
  [0.593],
  [0.611],
  [0.000],
  [0.087],
  [0.095],
  [6],
  [Tr\=1],
  [0.756 (4095)],
  [0.758 \[0.698,0.808\]],
  [0.712],
  [0.738],
  [0.149],
  [0.128],
  [0.159],
  [7],
  [a\=0],
  [0.535 (16383)],
  [0.535 \[0.500,0.570\]],
  [0.594],
  [0.633],
  [0.000],
  [0.049],
  [0.075],
  [7],
  [Tr\=1],
  [0.705 (16383)],
  [0.706 \[0.673,0.736\]],
  [0.697],
  [0.727],
  [0.094],
  [0.103],
  [0.148],
)
]

(l \= 2, 3 are in `data/q3_shaped.csv`; their samples are 15 and 63
curves.)

Number of class-B curves with a shaped subgroup (measured, S1, all b,
not orbit-reduced; the curves come in Frobenius orbits of size up to
3l): a \= 0: 132 / 405 / 2178 / 8760 and Tr(a)\=1: 165 / 588 / 3096 /
11553 for l \= 4 / 5 / 6 / 7. As a fraction of all 2^N - 1 curves
(derived from these counts): a \= 0: 132/4095 \= 3.2%, 405/32767 \=
1.2%, 2178/262143 \= 0.83%, 8760/2097151 \= 0.42%; Tr(a)\=1: 165/4095 \=
4.0%, 588/32767 \= 1.8%, 3096/262143 \= 1.2%, 11553/2097151 \= 0.55%. Of
the genus-3 share itself, the shaped part is 0.40-0.76 (table). "The 6%
survives" therefore only in the sense that about half to three quarters
of the genus-3 curves are shaped, and the 6% itself halves with each
increment of l.

#strong[P3 (class B indistinguishable from generic and null) was partly
rejected.] Pooled two-proportion z of B reps vs G (`data/zscores.txt`,
measured): for Tr(a)\=1, |z| \<\= 1.75 at l \= 4..7 (consistent with
generic; vs the null integers l \= 5 gives z \= -3.06, the other l are
within 1.4). For a \= 0, B is lower than generic at l \= 5, 6, 7 (z \=
-2.43, -2.13, -3.27) and than the null integers (z \= -3.45, -2.71,
-5.58); l \= 4 is not significant (n \= 23 orbits). Unplanned (post hoc,
measured, no derivation yet) observation that accompanies it: for a \= 0
every class-B curve has 8 | \#E(K) (minimum v2(\#E) \= 3 at every l \=
3..7, 63 to 16383 curves), where a generic a \= 0 curve only has 4 | \#E
(v2 \>\= 2). That forced extra 2-power also explains why S2 (cofactor
\<\= 4) is exactly 0 for a \= 0 class-B curves. For Tr(a)\=1, v2(\#E) \=
1 on every curve, as for a generic curve. Why trace-zero b adds a factor
2 for a \= 0 is not established here; flagged for follow-up, not a
result. Divisibility by 3 in class B ran at 27/63, 126/255, 495/1023,
1269/4095, 8127/16383 for a \= 0 (l \= 3..7), not compared with a
generic arm.

Arm D (b in k\*, defined over k; counts of 2^l-1 curves): r\_max \> sqrt
\#E(K) for 49 and 42 of 127 curves at l \= 7 (a \= 0 / Tr(a)\=1), 18/30
of 63 at l \= 6, and so on (`data/run.log`); small, shown only as the
contrast.

== Controls and cross-checks
<controls-and-cross-checks>
#align(center)[#table(
  columns: 2,
  align: (col, row) => (auto,auto,).at(col),
  inset: 6pt,
  [control], [result],
  [positive: b in k gives m \= 1],
  [all 2^l - 1 such b have m\=1, genus 1, every l\=2..7],
  [positive: toy census l \= 4 reproduced],
  [15 / 255 / 3825],
  [positive: cond (8) detector, n \= 2],
  [equals derived counts at l \= 3, 4, 5],
  [point-count formula vs brute force],
  [0 mismatches (l \= 2, 3, all b, both a)],
  [Hasse, parity, mod 4, twist, Frobenius invariance],
  [0 violations, all class-B curves, l\=2..7],
  [cross-check vs `target/release/ghs_screen`],
  [60 rows (l \= 3..7, 12 each; classes A, B and random; a \= 0 and
  Tr(a)\=1): magic number, genus, GHS type, condition (8) all equal, 0
  mismatches (`data/crosscheck_ghs_screen.csv`)],
  [negative: generic b and null integers],
  [see Q3 table],
)
]

== Failures and limits
<failures-and-limits>
- No run failed. The brief’s \#E(F\_{2^l}) criterion is ill-posed for
  genus-3 b (above); this was declared in the protocol before data.
- l \= 2, 3 samples are tiny; Wilson intervals at l \= 4 on orbit
  representatives (23 orbits) are wide.
- The primitive polynomial used per N is the smallest found (listed in
  `data/q1_census.csv`); counts do not depend on it.
- P3 numbers use one random sample (fixed seed) per l; G and I intervals
  ignore seed variation. The "8 | \#E" observation was seen in the order
  list `data/classB_orders.csv`, post hoc; no hypothesis test was run on
  it.
- Whether the genus-3 cover actually descends to a smooth model over
  F\_{2^l} and the cost of index calculus on it were not examined
  (condition (8) is necessary, not sufficient; see
  `docs/GHS_SCREEN.md`).
- PDF: built from this README with Pandoc (to Typst) and the Typst wheel
  (`typst 0.15.0`, installed in a scratch virtualenv outside the
  repository); no system PDF engine was present.

== Files
<files>
`PROTOCOL.md`, `README.md`, `README.pdf`, `graph.svg`, `graph.png`,
`tool/census.rs`,
`data/{q1_census,q2_cond8,q3_shaped,classB_orders,crosscheck_ghs_screen}.csv`,
`data/run.log`, `data/zscores.txt`, `data/graph.svg` (tool output;
`graph.svg` is a copy).

`report.typ` is the Pandoc-generated Typst source of the PDF.
