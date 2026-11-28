#set page(paper: "us-letter", margin: (x: 0.48in, y: 0.42in), numbering: "1 / 1")
#set text(font: "DejaVu Sans", size: 7.9pt, fill: rgb("#111827"))
#set par(leading: 0.43em, justify: true)
#show heading.where(level: 1): set text(size: 11.4pt, weight: "bold")
#show heading.where(level: 2): set text(size: 9.2pt, weight: "bold", fill: rgb("#1e3a8a"))

= Z-Exact Signed-Matroid Partial-Relation Graphs & Multi-Pattern Descent for Extension-Field ECDLP

*Protocol:* `EXT-IC-GRAPH-ANCHORED-MP-20261126` (`protocol.json`, sealed in `pre-call.sha256` prior to execution) #h(1fr) *Standing:* `[pcc:unverified]` \
*Scope:* Joux--Vitse degree-2 genus-3 Weil-restriction cover over $FF_(p^6) = FF_(p^2)[theta]/(theta^3 - c)$ (`foreign_hook_calls = 0`, all artifacts $< 64$ KiB).

== 1. Executive Summary & Architectural Motivation

On extension-field elliptic curves $E(FF_(p^6))$ with prime subgroup order $ell$ and cofactor $h = |E(FF_(p^6))| / ell > 1$, the partial-relation graph serves *two* coupled roles: (1) combining 1-LP and 2-LP sieve relations into full homogeneous factor-base rows for sparse linear algebra modulo $ell$, and (2) exporting *exact integer ($ZZ$) witnesses* $[s] D(frak(q)) = sum_k c_k B_k$ ($s in {1, 2}$) on the genus-3 Jacobian $J_C(FF_(p^2))$ so Phase-3 individual descent can eliminate irreducible $FF_p$-quadratic factors without torsion corruption. Standard modular graph filters (`large_prime_graph::witness` Fonlupt--Lempel, `large_prime_filter` Markowitz 2-core, and `SharedKey` Prim MST) invert coefficients modulo $ell$, destroying $h$-torsion and yielding *zero* valid $ZZ$-anchors (`[pcc:unverified]`).

This study designs, implements, and qualifies four native Rust components under complete `bitop/v1` accounting (`[pcc:unverified]`):
1. *Complete `bitop/v1` & Karatsuba Tower Closure (`zero_dim_la.rs`, `f4_fp.rs`, `jv_rho_strong.rs`):* Operation-level `LinOps` hooks reconcile identically across all `action_variable in 0..n`; `tracked_rref_contributors` prices pivot scans, inversions, and bitset ops without altering `F4Report::field_ops`; `KaratsubaArith` cuts $FF_(p^6)$ `(mul, sq, inv)` from `(55, 55, 80)` to `(32, 28, 65)` $FF_p$ multiplications (`[pcc:unverified]`).
2. *Online $ZZ$-Exact `SignedMatroidGraph` (`jv_signed_matroid_graph.rs`):* Replaces `BTreeMap` per-edge BFS with $O(1)$ `HashMap` dense vertex interning, union-by-size DSU, smaller-tree re-rooting ($O(V log V)$ edge flips), LCA shared-suffix cancellation, complete Zaslavsky balanced/frustrated cycle harvesting over $ZZ$ (`0` modular inversions), dynamic shortest-witness root relaxation, multi-source Dijkstra `export_anchor_table()`, and watched-root $k=3$ (`3-LP`) hypergraph cascade (`[pcc:unverified]`).
3. *Hit-Bucket Sieving & Deterministic Frobenius-Norm Quartic Splitting (`jv_sieve.rs`, `jv_extension_anchored_dlp.rs`):* `sieve_line_buckets` records hitting factor-base abscissae directly during Montgomery batch inversion, while `split_quartic_frob_resolvent` factors $(2^2)$ quartics deterministically (`0` RNG draws) via conjugate-pair norms $n_alpha(x) = (x+alpha)(x^p+alpha) mod Q$ ($alpha in {0, 1}$) and $O(1)$ table square roots (`[pcc:unverified]`).
4. *Graph-Anchored Multi-Pattern Descent + Cold F4 Trace Replay (`jv_anchored_descent.rs`):* Accepts squarefree sextics of types $(1^6), (1^4, 2), (1^2, 2^2), (2^3)$ whose quadratic factors lie in `AnchorTable`, checking anchors *before* splitting $gcd(S, x^p - x)$ and replaying a cold-recorded F4 elimination trace (`[pcc:unverified]`).

== 2. Seven-Engine Partial-Relation Graph Head-to-Head (`p = 1009, seed = 389`)

All seven repository partial-relation algorithms were executed on the *identical* collected stream (`100,088` sources at `b_count = 155,648`: `215` full `0-LP`, `8,013` `1-LP`, `82,961` `2-LP`, `8,899` `3-LP`; $|cal(B)| = 509$, target rank $508$) (`[pcc:unverified]`).

#figure(
  image("figures/graph_engine_comparison.svg", width: 92%),
  caption: [Head-to-head comparison of 7 partial-relation graph/hypergraph engines on the identical Run50 stream (`[pcc:unverified]`).],
)

#table(
  columns: (1.95fr, 0.52fr, 0.62fr, 0.58fr, 0.72fr, 0.78fr, 0.85fr, 0.88fr, 0.72fr),
  inset: 3.4pt,
  align: (left, center, center, center, right, right, right, right, right),
  stroke: 0.4pt + rgb("#d1d5db"),
  [*Graph Engine*], [*Arity*], [*Online*], [*$ZZ$-Exact*], [*FB Rank*], [*$FF_ell$ Inv*], [*Mean Wt*], [*$ZZ$-Anchors*], [*Graph Bitops*],
  [`1lp-singleton-table`], [$<= 1$], [Yes], [Yes], [`315 / 508`], [`0`], [`10.56`], [`7,913`], [`3.09e6`],
  [`2lp-forest-witness-fl`], [$<= 2$], [Yes], [No], [`508 / 508`], [`91,352`], [`16.33`], [`0`], [`8,515.36e6`],
  [`klp-sharedkey-prim-mst`], [$<= 3$], [No], [No], [`508 / 508`], [`1,562`], [`16.28`], [`0`], [`1,949.82e6`],
  [`klp-markowitz-2core`], [$<= 3$], [No], [No], [`508 / 508`], [`793`], [`16.33`], [`0`], [`1,039.91e6`],
  [`2lp-deferred-bfs`], [$<= 2$], [Yes], [Yes], [`508 / 508`], [`0`], [`16.33`], [`20,667`], [`392.85e6`],
  [*`2lp-signed-matroid-dsu`*], [*$<= 2$*], [*Yes*], [*Yes*], [*`508 / 508`*], [*`0`*], [*`16.26`*], [*`20,667`*], [*`254.61e6`*],
  [*`3lp-signed-matroid-hyper`*], [*$<= 3$*], [*Yes*], [*Yes*], [*`508 / 508`*], [*`0`*], [*`16.37`*], [*`21,080`*], [*`291.04e6`*],
)

*Key findings (`[pcc:unverified]`):* `1lp-singleton-table` stalls at rank `315/508` (`62%`) on `155,648` planes. Among full-rank engines, `2lp-signed-matroid-dsu` cuts pure graph + projection `bitop/v1` cost by *`1.54x`* vs `2lp-deferred-bfs` (`2.29x` fewer hash probes), *`4.08x`* vs `klp-markowitz-2core`, *`7.66x`* vs `klp-sharedkey-prim-mst`, and *`33.44x`* vs `2lp-forest-witness-fl`, while achieving the *lowest* mean row weight (`16.26`, `8,438` nonzeros) and exporting `20,667` $ZZ$-exact anchors. Extending to `3lp-signed-matroid-hyper` cascades resolved $(1^3, 2^3)$ triples into `+413` extra unit anchors (`21,080` total) at `291.04e6` graph bitops (`3.57x` below `klp-markowitz-2core`).

#pagebreak()

== 3. Panel A: 13-Arm Run50 End-to-End Factorial & Matroid Ablation (`p = 1009, seed = 389`)

All 13 arms received public input $(p=1009, ell=263,807,419,614,139,553, G, Q)$ with `Spec.d = 0` (`|Q_2(1009)| = 254,530`) and independently recovered the exact 58-bit discrete logarithm $d = 242,624,768,583,643,540$ (`[pcc:unverified]`).

#figure(
  image("figures/run50_ablation.svg", width: 92%),
  caption: [Panel A 13-arm cold end-to-end phase breakdown (`w=0`) and multi-pattern yield curve $M(f)$ (`[pcc:unverified]`).],
)

#table(
  columns: (2.05fr, 0.62fr, 0.58fr, 0.58fr, 0.68fr, 0.68fr, 0.72fr, 0.74fr, 0.60fr),
  inset: 3.2pt,
  align: (left, right, right, right, right, right, right, right, right),
  stroke: 0.4pt + rgb("#d1d5db"),
  [*Run50 Arm (`p=1009, seed=389`)*], [*Planes $B$*], [*$f$ (%)*], [*Resid.*], [*Rel ($10^(12)$)*], [*Desc ($10^(12)$)*], [*Total $w=0$*], [*Kara $w=1$*], [*Wall (s)*],
  [`sparse-table:original`], [`225,280`], [`4.44`], [`3,415`], [`1.476`], [`4.368`], [`6.172T`], [`6.176T`], [`20.46`],
  [`sparse-table:traced`], [`225,280`], [`4.44`], [`3,415`], [`1.476`], [`2.485`], [`4.289T`], [`4.292T`], [`18.70`],
  [`sparse-table:anchored`], [`225,280`], [`4.44`], [`666`], [`1.476`], [`0.852`], [`2.657T`], [`2.660T`], [`15.51`],
  [`sparse-table:traced-anchored`], [`225,280`], [`4.44`], [`666`], [`1.476`], [`0.490`], [`2.294T`], [`2.297T`], [`15.34`],
  [`sparse-table-fast:traced-anchored`], [`225,280`], [`4.44`], [`666`], [`1.473`], [`0.490`], [`2.291T`], [`2.294T`], [`15.11`],
  [`deferred-pairs:original`], [`155,648`], [`8.06`], [`3,415`], [`2.043`], [`4.368`], [`6.872T`], [`7.495T`], [`30.53`],
  [`deferred-pairs:traced`], [`155,648`], [`8.06`], [`3,415`], [`2.043`], [`2.485`], [`4.989T`], [`5.612T`], [`28.76`],
  [`deferred-pairs:anchored`], [`155,648`], [`8.06`], [`666`], [`2.043`], [`0.852`], [`3.356T`], [`3.979T`], [`24.64`],
  [`deferred-pairs:traced-anchored`], [`155,648`], [`8.06`], [`666`], [`2.043`], [`0.490`], [`2.993T`], [`3.617T`], [`24.12`],
  [`deferred-pairs-fast:traced-anchored`], [`155,648`], [`8.06`], [`666`], [`1.906`], [`0.490`], [`2.856T`], [`3.479T`], [`23.06`],
  [`deferred-pairs-fast-cycle:traced-anchored`], [`155,648`], [`8.06`], [`666`], [`1.906`], [`0.490`], [`2.856T`], [`3.480T`], [`22.99`],
  [*`signed-matroid-2lp:traced-anchored`*], [*`155,648`*], [*`8.12`*], [*`666`*], [*`1.131`*], [*`0.490`*], [*`2.065T`*], [*`2.068T`*], [*`11.89`*],
  [`signed-matroid-3lp:traced-anchored`], [`154,624`], [`8.13`], [`666`], [`1.315`], [`0.490`], [`2.248T`], [`2.262T`], [`13.43`],
)

*Factorial & Sensitivity Analysis (`[pcc:unverified]`):*
- *Orthogonal Descent Speedups:* Cold F4 trace replay (`1` recorded, `657` replayed, `8` cold fallbacks) yields `1.758x` alone; anchored multi-pattern acceptance (`1` $(1^6)$ + `1` $(1^4, 2)$ witness, cutting residuals `3,415 -> 666`) yields `5.124x` alone; combined `TracedAnchored` achieves *`8.920x`* (`4.368T -> 0.490T`, `-88.79%`), matching multiplicative independence $1.758 times 5.124 = 9.008 times$ within `0.97%`.
- *Phase-1 Relation Acceleration & Collapse of the $w$-Band:* `signed-matroid-2lp` cuts Phase-1 `relations_bitops` by *`1.807x`* (`2.043T -> 1.131T`; `2.089x` wall speedup `22.78s -> 10.90s`), making 2-LP *faster than 1-LP* (`2.065T` vs `2.291T` total `w=0`; `11.89s` vs `15.11s` wall) while eliminating live deferred-edge overhead (`live_deferred_edges = 0`, so $w=0 -> w=1$ Karatsuba varies by only `0.16%`: `2.0645T -> 2.0678T`, vs `+9.1%` on `deferred-pairs:original`).
- *`2-LP` vs `3-LP` Trade-off:* `signed-matroid-3lp` reaches full rank in `1,024` fewer planes (`154,624`) and harvests `20,704` anchors, but cubic $(2^3)$ splitting adds `0.184T` relation bitops, confirming $S_6$ cycle-type theory (`1/8` vs `1/48` yield) that `signed-matroid-2lp` is end-to-end optimal at $p <= 1009$. Against matched `p=1009` Pollard rho (`130.78T` analytical / `74.52T` empirical Karatsuba `B=16`, `60.70s` wall), `signed-matroid-2lp:traced-anchored` is *`63.3x`* / *`36.1x`* lower in bitops (`5.10x` faster in wall time).

#pagebreak()

== 4. Panel B: Multi-Prime Scaling, Finite-$B$ Transition & Pollard Rho (`60` IC Runs)

Across $p in {307, 383, 503, 661, 1009}$ and three fresh seeds `{101, 211, 307}` per prime (`60` IC bundles + `63` executed negation-rho `B=16` walks), every solved bundle recovered the planted scalar with `foreign_hook_calls = 0` (`[pcc:unverified]`).

#figure(
  image("figures/multiprime_scaling.svg", width: 90%),
  caption: [Multi-prime total `bitop/v1` scaling vs Karatsuba/schoolbook Pollard rho and anchor coverage $f(p)$ (`[pcc:unverified]`).],
)

#table(
  columns: (0.48fr, 0.72fr, 0.65fr, 0.72fr, 0.62fr, 0.72fr, 0.78fr, 0.80fr, 0.85fr, 0.88fr),
  inset: 3.2pt,
  align: (right, right, center, right, right, right, right, right, right, right),
  stroke: 0.4pt + rgb("#d1d5db"),
  [*$p$*], [*Mean $ell$*], [*1LP / M2*], [*$|cal(Q)_2|$*], [*$f$ (%)*], [*$p dot f$*], [*Rel Spd*], [*Desc Spd*], [*Matroid $w=0$*], [*IC / Kara-Rho*],
  [`307`], [`2.09e14`], [`1/3` | `2/3`], [`23,473`], [`20.73`], [`63.6`], [`1.28x`], [`11.67x`], [`0.302T`], [`0.0988` (`10.1x`)],
  [`383`], [`7.89e14`], [`1/3` | *`3/3`*], [`36,559`], [`17.83`], [`68.3`], [`1.31x`], [`6.91x`], [`0.655T`], [`0.1102` (`9.1x`)],
  [`503`], [`4.05e15`], [`3/3` | `3/3`], [`63,009`], [`14.58`], [`73.3`], [`1.49x`], [`3.60x`], [`0.777T`], [`0.0577` (`17.3x`)],
  [`661`], [`2.09e16`], [`3/3` | `3/3`], [`109,024`], [`11.09`], [`73.3`], [`1.56x`], [`4.79x`], [`1.067T`], [`0.0290` (`34.5x`)],
  [`1009`], [`2.64e17`], [`3/3` | `3/3`], [`254,353`], [`7.60`], [`76.7`], [`1.79x`], [`2.97x`], [`1.879T`], [*`0.0144` (`69.6x`)*],
)

*Scaling Laws (`log cost / log p`, `[pcc:unverified]`):* Anchor coverage obeys $p dot f(p) approx 64..77$ (log-slope `-0.854` over all 5 primes, `-0.894` on `661..1009`). End-to-end `signed-matroid-2lp:traced-anchored` `w=0` bitops scale with slope *`1.393`* (`1.337` on `661..1009`; relations `1.308`, descent `0.770`), whereas Karatsuba Pollard rho scales as $O(p^3)$ with slope *`3.000`* (`661..1009`). At `p = 383`, 1-LP starves on 2 of 3 seeds (`1/3`), whereas 2-LP solves `3/3`; at tiny `p = 307` (`B_max = 46,971`), seed `307` hits the finite plane cap across all arms (`2/3` solved, cleanly recorded with `verified = false`).

== 5. Hypothesis Verdicts, Controls, Independent Qualification & Limitations

#table(
  columns: (0.48fr, 1.05fr, 3.45fr),
  inset: 3.3pt,
  align: (left, left, left),
  stroke: 0.4pt + rgb("#d1d5db"),
  [*ID*], [*Verdict*], [*Observed Evidence vs Frozen Acceptance Criteria (`[pcc:unverified]`)*],
  [`H1`], [*Confirmed*], [`LinOps` reconciles identically across all `action_variable`; `foreign_hook_calls = 0` on all 73 bundles; `KaratsubaArith` verified at `(32, 28, 65)` vs `(55, 55, 80)` $FF_p$ muls.],
  [`H2`], [*Refined (6.7%)*], [Byte-identical sources/anchors/kernels/scalars confirmed (`11,822` / `90,877` sources), but `HitBuckets` alone cut Phase-1 bitops by `6.7%` (`< 25%` target) because Berlekamp + Cantor replay dominated legacy `DeferredPairs`; this diagnosis directly drove `H7`.],
  [`H3`], [*Confirmed*], [$M_"pred"(0.0812) = 2.523 >= 1.25$ at `p=1009`; residuals dropped `3,415 -> 666` (`5.13x`) with `100%` Cantor replay `[scale]R == sum d_k B_k`.],
  [`H4`], [*Confirmed*], [`TracedAnchored` cut Phase-3 `descent_bitops` by `88.79%` (`8.920x`, `4.368T -> 0.490T`), exceeding the `>= 45%` threshold.],
  [`H5`], [*Null Confirmed*], [Synthetic frustrated odd cycles pass `[2]R == sum d_k B_k`, while empirical `p=1009` rank-stop streams have `0` frustrated cycles ($Delta f = 0 < 0.02$), confirming the sparse-subgraph null.],
  [`H6`], [*Confirmed (14/15)*], [2-LP overcomes `p=383` 1-LP starvation (`3/3` vs `1/3`) and solves `14/15` instances (`1/3` finite-plane cap at `p=307, seed=307`); IC slope `1.34` vs rho `3.00`.],
  [`H7`], [*Graph Confirmed; Rel 1.81x*], [`SignedMatroidGraph` wins all 7-engine metrics among full-rank $ZZ$-exact engines (`1.54x..33.4x` lower graph bitops, `0` $FF_ell$ invs) and `2-LP` beats `3-LP` (`2.065T` vs `2.248T`). Phase-1 bitops fell `1.807x` (`2.09x` wall; `< 2.5x` bitop target due to irreducible Montgomery batch-sieve floor).],
)

*Overflow & Independent Qualification Controls (`[pcc:unverified]`):* Standard release (`overflow_checks_compiled = false`) vs `-C overflow-checks=on` (`true`) across `(307,101), (383,211), (503,307)` produced bit-identical `sources_sha256`, `projections_sha256`, `recovered_scalar`, `residuals_tried`, and `observed_bitops`. Pre-sealed `/tmp/immutable_jv_graph_ic_anchored_qualify` (`sha256 = 3eb64afa...`) checked all `73` bundles (`qualification_panel_a.json`, `qualification_panel_b_part1.json`, `qualification_panel_b_part2.json`: `73/73` `all_passed = true`, `failed_bundles = 0`, plus 4 negative sabotage unit tests). Results apply strictly to the degree-2 genus-3 Weil-cover family over $FF_(p^6)$ and carry no general prime-field ECDLP claim (`[pcc:unverified]`).
