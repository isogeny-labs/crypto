#set page(paper: "a4", margin: 13mm)
#set text(size: 8.7pt)

= Orbit-complete line-2 thinning in quartic-twist index calculus

23 October 2026. Preregistered ideal exact-oracle survey on all 21
orbit-complete line-2 subsets of the six public one-target p=73/p=97
streams. The full source-linked report, native input records and
byte-identical A/A run are in `K1_ORBIT_THINNING.md` and the two JSON
records beside it. This is a group-oracle rank and peel-count result,
not a polynomial speed comparison.

== Construction and independent checks

Every line-2 abscissa orbit has 12 physical factors and one folded log
column. The k=1 collector asks for up to 24 signed S4 peels per orbit
per residual. Keep all line-0 factors, then retain the first j complete
line-2 orbits by smallest physical index, for every j=1..4 at p=73 and
j=1..3 at p=97. The complete scalar-keyed signed set on every query
matches the earlier independently audited point-keyed census digest.
Filtering that set by line-2 orbit gives the exact subset relations.
Folded rows are added in sorted order until full rank. An independent
Python Fp4 elliptic-curve implementation checks every solved factor
representative and target by scalar multiplication. All 21 cases reach
rank; the A/A JSON is byte-identical.

#image("k1_orbit_thinning.svg", width: 88%)

== Frozen grid

#table(
  columns: (.8fr, .55fr, .65fr, .65fr, .75fr, .8fr, 1fr),
  inset: 2.7pt,
  [*p/seed*], [*j*], [*L2 size*], [*cols*], [*queries*], [*prefix hits*], [*S4 peels*],
  [73/0], [1], [12], [5], [30], [6], [720],
  [73/0], [2], [24], [6], [20], [6], [960],
  [73/0], [3], [36], [7], [15], [7], [1,080],
  [73/0], [4], [48], [8], [15], [8], [1,440],
  [73/1], [1], [12], [5], [40], [6], [960],
  [73/1], [2], [24], [6], [19], [8], [912],
  [73/1], [3], [36], [7], [19], [9], [1,368],
  [73/1], [4], [48], [8], [19], [10], [1,824],
  [73/2], [1], [12], [5], [47], [5], [1,128],
  [73/2], [2], [24], [6], [45], [6], [2,160],
  [73/2], [3], [36], [7], [43], [11], [3,096],
  [73/2], [4], [48], [8], [43], [15], [4,128],
  [97/0], [1], [12], [4], [310], [4], [7,440],
  [97/0], [2], [24], [5], [130], [8], [6,240],
  [97/0], [3], [36], [6], [310], [16], [22,320],
  [97/1], [1], [12], [4], [302], [4], [7,248],
  [97/1], [2], [24], [5], [75], [5], [3,600],
  [97/1], [3], [36], [6], [75], [6], [5,400],
  [97/2], [1], [12], [4], [182], [4], [4,368],
  [97/2], [2], [24], [5], [150], [5], [7,200],
  [97/2], [3], [36], [6], [150], [6], [10,800],
)

== Interpretation

The lowest ideal S4 peel count in each seed is 1.5 to 3.66 times below
the full-base count, but the best orbit count varies by seed. The p=73
seed-one full-base exact oracle stops at query 19, one query before the
partial polynomial collector: it uses two relations missed by a failed
native S4 system. These are exact requested-system counts, not solver
field work or runtime. No native polynomial run has yet tested the
thinned bases. A prospective all-j native grid with independent replay,
full accounting and matched rho is the next measurement; selecting a
best j after this survey cannot justify a speed claim.
