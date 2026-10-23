#set page(paper: "a4", margin: 15mm)
#set text(size: 9pt)
#set par(justify: true)

= Twist symmetry in mixed-component polynomial index calculus

23 October 2026. Public synthetic one-target quartic-twist workloads at
$p=73$ and $p=97$, each with $Q=[271]G$ and three frozen independent seeds.
Protocol, raw records, independent audits, per-run PDFs and source links are
in `K1_SCALING_REPORT.md`, `K1_SCALE_RUNS.md`, and `K1_CENSUS_RUNS.md`.

== Mechanism

The transported Frobenius and cubic automorphism give a signed 24-action.
Each tested nonzero abscissa orbit has size 12. A mixed base with $B_0$
points on $L_0=F_p$ and $B_2$ on $L_2=t^2 F_p$ has $(B_0+B_2)/12$
folded log columns: eight for the $48+48$ base at $p=73$, six for the
$36+36$ base at $p=97$. Restricting to three $L_0$ factors and one $L_2$
factor permits a signed $L_2$ peel followed by a symmetrised $S_4$ solve
in three $L_0$ abscissae. The 24 images of a folded relation row are
scalar multiples of that row, so symmetry does not provide 24 new
independent equations. This was checked on all 42 selected rows under
all 24 actions: 1,008 exact row/RHS equalities and 12,096 factor-image
closure checks.

#image("k1_scaling_overview.svg", width: 92%)

== Six one-target polynomial runs

#table(
  columns: (1fr, 1fr, 1fr, 1fr, 1.4fr, 1fr),
  inset: 3pt,
  [*p/seed*], [*Queries*], [*Exact/poly*], [*Rank*], [*IC Fp M*], [*IC/rho Fp M*],
  [73/0], [15], [8/8], [8/8], [888,203,272], [1,788],
  [73/1], [20], [11/9], [8/8], [1,183,617,228], [3,225],
  [73/2], [43], [15/15], [8/8], [2,542,727,892], [5,283],
  [97/0], [310], [16/16], [6/6], [13,839,508,536], [17,249],
  [97/1], [75], [6/6], [6/6], [3,349,559,070], [6,695],
  [97/2], [150], [6/6], [6/6], [6,696,848,851], [8,461],
)

Every target was independently verified as 271. Five processed-prefix
signed sets matched exactly. The $p=73$, seed-one $S_4$ collector missed
two relations at one failed system (query index 18), but still reached
anchored rank eight and recovered the target. It remains an explicit
partial relation-finder result. All first/A/A non-timing records match.

== Complete frozen-stream occupancy

There are exactly $16 binom(B_0,3) B_2$ signed distinct-index $k=1$
tuples, giving expected set size $lambda_1=16 binom(B_0,3) B_2/r$
for a uniformly random residual. This first moment does not imply
independent hits or guaranteed matrix rank. A point-keyed group census
and independent BSGS-derived scalar-keyed auditor matched all 1,856
per-query complete-set digests, including post-stop queries.

#table(
  columns: (1fr, 1fr, 1fr, 1fr, 1.2fr),
  inset: 3pt,
  [*p/seed*], [*Stream*], [*Exact k=1*], [*Zero queries*], [*Uniform total*],
  [73/0], [117], [77], [57], [54.71],
  [73/1], [120], [45], [80], [56.11],
  [73/2], [122], [45], [88], [57.05],
  [97/0], [558], [27], [535], [25.92],
  [97/1], [458], [32], [427], [21.27],
  [97/2], [481], [20], [461], [22.34],
)

At $p=97$, 1,423 of 1,497 residuals have no $k=1$ relation. The
polynomial producer agrees with the exact oracle on every processed
$p=97$ query, so low physical occupancy explains these long collection
paths. A one-count mutation in the census was rejected by the scalar
auditor at query zero.

== Claim boundary

The displayed ratios compare counted field multiplications only. Native
hash/tree and allocator CPU, requested peak bytes, calibrated group/field
bands, isolated interleaved timing, and same-input msolve/Magma results
remain missing. They are not whole-method speed ratios or an admitted
leaderboard result. The p=37 control had a counted ratio of 332.6 on a
different base. This finite three-prime panel does not establish an
asymptotic trend or a deployed-curve claim. The complete Markdown report
names the next failure repair, base-shape comparison, and pricing gates.
