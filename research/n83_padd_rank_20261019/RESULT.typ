#set page(margin: 1.7cm)
#set text(size: 9pt)
#show table: set text(size: 8pt)
#set table(stroke: 0.4pt + luma(170), inset: 4pt)
= N83 K1120: the padd rank stage lowers cold time by 12.49 s
<n83-k1120-the-padd-rank-stage-lowers-cold-time-by-12.49-s>
Porting the rank-stage change of `aburan28/cryptanalysis` PR \#970 to
crypto’s K1120/root11 producer (`KIC_RANK_SOLVE=padd`) lowers the mean
cold one-target process time from #strong[119.899 s to 107.412 s], a
paired saving of #strong[12.487 s, 95% bootstrap interval \[12.248,
12.723\] s] (ratio 1.1163 \[1.1139, 1.1186\]), over 20 points on one
GitHub `ubuntu-24.04` runner. The rank stage speeds up #strong[1.2194x
\[1.2155, 1.2233\]], above the pre-registered 1.2 kill threshold by
0.019, so the idea transfers, with a much smaller rank gain than \#970’s
1.377x. Relations, rank, scalars and the factor base are identical to
the current producer on all 20 points. Matched strong rho is still
faster: cold IC/rho is #strong[52.8 \[45.0, 63.2\]] with padd and
#strong[58.9 \[50.3, 70.3\]] without (60 rho draws), IC wall over rho
wall, IC on 4 threads and rho on 1.

#figure([#image("result.png")],
  caption: [
    Paired panel. Source: #link("plot.py"), data: #link("points.json"),
    #link("analysis.json"). Vector: #link("result.svg").
  ]
)

== Result (measured; frozen protocol in #link("PROTOCOL.md"))
<result-measured-frozen-protocol-in-protocol.md>
#align(center)[#table(
  columns: 5,
  align: (col, row) => (auto,right,right,auto,auto,).at(col),
  inset: 6pt,
  [Quantity (mean over 20 points)], [current (A/A mean)], [padd],
  [paired saving, s, 95% bootstrap], [ratio current/padd, 95%],
  [cold child wall],
  [119.899],
  [107.412],
  [12.487 \[12.248, 12.723\]],
  [1.1163 \[1.1139, 1.1186\]],
  [rank stage (in process)],
  [69.621],
  [57.096],
  [12.525 \[12.330, 12.728\]],
  [1.2194 \[1.2155, 1.2233\]],
  [setup (index build etc.)],
  [49.685],
  [49.688],
  [-0.003 \[-0.091, 0.081\]],
  [0.9999 \[0.9982, 1.0017\]],
  [one-target online, ms],
  [176.9],
  [177.2],
  [-0.2 \[-1.4, 0.7\]],
  [0.9988 \[0.9919, 1.0037\]],
)
]

Student-t interval (df 19) for the cold saving: \[12.229, 12.745\] s.
All 20 per-point cold savings are positive, 11.26 to 13.52 s. The saving
is the rank stage’s: the rank-stage saving (12.53 s) and the cold saving
(12.49 s) agree, and setup and the target scan (which keeps the S3
solve) do not move.

#strong[Controls (all passed).] A/A (`cur_a` over `cur_b`): cold 0.9983
\[0.9954, 1.0012\], rank 0.9968 \[0.9933, 0.9999\], inside the frozen
\[0.97, 1.03\]. The rank A/A upper bound sits just under 1, a 0.3% slot
effect that is 40 times smaller than the effect measured. Gates: scalar
equals the sealed answer for every IC arm and rho draw; per point the
target relation, probes, rank attempts, relations, failures, rows
without gain, rank, mean rank probes and the SHA-256 of the dumped
factor base are identical across the three IC arms; every padd arm
records `rank_solve = padd`, zero fallback states, zero irregular inputs
and one resolved hit per rank relation. `cargo test` of the example (24
tests, including `padd_rank_tests`) passed on the runner. Local
bit-identity of the new binary against a binary built from `main`,
sealed point 1 at K1120, also passed
(#link("identity_check.py")[identity\_check.py]: 1120 resolved hits, 551
swapped pairs, 0 fallbacks); that check covers one point.

#strong[Kill condition.] The rank-stage speedup point estimate is
1.2194, not below 1.2, so the condition did not fire. The margin is
small; a different host or K could fall under it.

== Cold IC / matched rho (measured)
<cold-ic-matched-rho-measured>
Rho: `n83_rho_optimization_eval`, config `fold/64/DP6`, 3 draws per
point, 60 draws, mean online #strong[2.0354 s], sd 1.0548 s (sd/mean
0.52, as the earlier 5-draw figure suggested). Interval: resample points
and, inside each draw, rho draws.

#align(center)[#table(
  columns: 4,
  align: (col, row) => (auto,auto,auto,right,).at(col),
  inset: 6pt,
  [IC arm], [cold IC / rho online, 95%], [vs rho child wall, 95%], [5
  sealed points only],
  [current],
  [58.9 \[50.3, 70.3\]],
  [47.5 \[41.7, 54.7\]],
  [56.0],
  [padd],
  [52.8 \[45.0, 63.2\]],
  [42.5 \[37.3, 49.2\]],
  [49.9],
)
]

The earlier figure (cold IC/rho about 55, five rho draws, no interval,
`research/n83_e2e_holdout_20261016`) is inside the current arm’s
interval. The distance to rho remaining is 52.8x in wall time for the
best IC arm. Accounting caveat: the IC process runs 4 rank threads, the
rho process runs one thread (no threading in its source), so the ratio
is wall over wall, not per core. Per-core accounting was not measured.

== Section 5a (pricing)
<section-5a-pricing>
IC native work per run (mean over 20 points, identical in both IC arms):
566,791,207 rank probes plus 560,285 target probes, each probe one
canonicalisation and one table query, one root solve per two probes. Rho
(mean of 60): 8,248,365 group additions, 8,248,364 canonicalisations,
8,117,869 partition hashes, 128,894 table queries, 128,893 inserts.

#align(center)[#table(
  columns: 3,
  align: (col, row) => (right,right,right,).at(col),
  inset: 6pt,
  [weight, GAE per native op], [rho cost, GAE], [IC unpriced native
  work, GAE (both arms)],
  [0],
  [8.25e6],
  [0],
  [0.1],
  [9.91e6],
  [1.42e8],
  [1],
  [2.49e7],
  [1.42e9],
)
]

The IC producer does not emit its group-addition count (the lifts and
target descent), so its total cost in GAE cannot be formed: a named
counter without an emitted count cannot be ranked, and this is recorded
as an implementation gap. IC and rho are therefore #strong[unresolved at
the 0 weight boundary] and no IC-versus-rho GAE ranking is claimed. The
two IC arms have identical native-operation counts, so in counted S the
change is a tie; the gain is an implementation (wall-clock) gain at
equal counted operations. The field work inside a root solve falls (a
point addition per candidate in place of the S3 solve), which the
counters above do not price.

== One canonical N83 baseline row
<one-canonical-n83-baseline-row>
Two N83 baselines disagreed because they measure different pipelines on
different runner configurations, not because either is wrong:

#align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [], [crypto frontier (this study)], [cryptanalysis
  `docs/ic/BASELINES.md` (\#970)],
  [pipeline],
  [K1120/root11, Bloom, sparse canonical, 4 rank threads],
  [K600 stacked producer, one pinned CPU],
  [cold time],
  [119.90 s current, #strong[107.41 s with padd]],
  [261.0 s, 195.7 s with rank cut],
  [rank-stage gain],
  [1.2194x],
  [1.377x],
  [matched rho],
  [strong rho, mean 2.035 s (60 draws)],
  [tuned rT770, about 2.98 s implied],
  [cold IC/rho],
  [#strong[52.8 \[45.0, 63.2\]] (padd)],
  [65.7 \[55.4, 80.2\]],
)
]

The canonical N83 row is this study’s K1120/root11 + padd row on the a
\= 1 curve (`EC1N83Ckb1a1haacb69ee9f37`, 53-bit subgroup, 4-vCPU
runner): cold 107.412 s, rho 2.035 s, ratio 52.8 \[45.0, 63.2\]. Its
predecessor (current producer, 119.899 s, 58.9 \[50.3, 70.3\]) replaces
the five-draw figure of about 55 with an interval. The cryptanalysis row
is a different, slower configuration (K600, one CPU); the two intervals
overlap but the rows are not interchangeable and neither is a
rho-matched comparison of the other. `aburan28/cryptanalysis` could not
be edited from this session (read-only); its `BASELINES.md` should cite
this row, with the pipeline and thread difference above.

== Limits
<limits>
- #strong[No extrapolation to ECC2K-130.] Every complete N83 result,
  this one included, is on the a \= 1 curve with a 53-bit prime
  subgroup. The a \= 0 curve (81-bit subgroup) is the ECC2K-130 analogue
  and has no complete IC result. Nothing here is evidence about it or
  about an exponent.
- One runner allocation. The intervals are over points; between-runner
  variance was not sampled. Earlier panels of the same pipeline on other
  runner allocations gave cold 119.83 s (5 points) and 119.70 s (16
  points), consistent with this panel’s 119.90 s.
- The child wall time is quantized to 0.5 s by the runner’s reaping loop
  (fractional parts cluster at .1 and .6), so per-point cold savings are
  coarse; the in-process rank timer is not quantized and agrees.
- Fifteen new points are disjoint from every N83 point file in the
  repository and the 1000 derived wide3 points; earlier fixtures held
  only in an external archive could not be replayed (collision
  probability about 1e-12, not checked).
- Wall-clock is a practicality measurement, not the headline metric
  (AGENTS.md section 6).
- Hypothesis, not tested: \#970’s baseline loop pipelines table lookups
  with prefetch, so root arithmetic was a larger share of its per-probe
  cost (its profile puts the S3 solve at about 110-130 of 256 ns per
  probe); crypto’s loop looks each key up as it is computed, so the same
  arithmetic saving buys less. A pipelined crypto loop is a separate
  candidate.

== Evidence
<evidence>
- Run:
  #link("https://github.com/aburan28/crypto/actions/runs/39296352594")[Actions 39296352594],
  commit `4b2b3efa6210ce3724f1350c3a9b1cc2df1309fd` (the later commit
  `ceeb652a1` only reformats the added code). Artifact `12223498464`,
  ZIP SHA-256
  `41e1a7f4c7ee33c71902b39d21c52189abb0b5f2e7f9a15fbdf910b082bd9f19`
  (866 MB unpacked, 265 MB zip; not committed).
- Committed here: #link("analysis.json") (also `analysis.log`),
  #link("points.json"), `host.json`, `source.json`, `run.log`,
  #link("PROTOCOL.md"), #link("run.py"), #link("analyze.py"),
  #link("generate.py"), #link("extract_points.py")[extract\_points.py],
  #link("identity_check.py")[identity\_check.py].
- Source: `examples/koblitz_orbit_dlp_fast.rs`, `scan_state128_padd`,
  `probe_candidate128`, `extract128_with`, tests `padd_rank_tests`.
- Host: four AMD EPYC vCPUs, 16,377,684 KiB, peak IC RSS 1,955.6 MiB;
  receipts in `host.json`.
