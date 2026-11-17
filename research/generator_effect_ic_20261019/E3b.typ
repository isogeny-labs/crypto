#set page(margin: 1.8cm)
#set text(size: 8.5pt)
= E3b: E3 at larger B, and F4 run to full solves
<e3b-e3-at-larger-b-and-f4-run-to-full-solves>
#strong[Extension of #link("E3.md")[`E3.md`], run under
#link("E3b-PROTOCOL.md")[`E3b-PROTOCOL.md`] (commit `f9cf6ab116`, frozen
before any measured run).] No security claim. The curves are a
restricted prime family (`2^t | p − 1`). Nothing here concerns P-192 or
P-224.

== What E3 left open, and the answer
<what-e3-left-open-and-the-answer>
E3 (PR \#2323) compared the Semaev/F4 oracle on the `prime-kummer` base
with `known-log:scalars=random` + MITM at equal B, but stopped at B = 19
and priced its F4 arms from the grid arm's records. E3b closes both
limits as far as this host allows.

+ #strong[Larger B (Part L).] The two arms that run end to end, the `V³`
  grid reference on the Kummer base and known-log MITM (`m = 3`), at
  tower heights `t = 4…9` (B = 6 to 276) on three new 24-bit prime-order
  curves, 8 targets each, 312 runs, all verified. This passes the
  known-log arm's own optimum.
  - #strong[Known-log MITM is cheapest at `t = 7` (B = 61--69)] on every
    curve, at 3.2--4.4× `rho-neg` (w = 0).
  - #strong[The order of the grid and known-log MITM flips inside the
    band at every t], and no crossover in B exists. At w = 0 the grid's
    point estimate is below known-log at every t (ratio 0.36--0.75 at
    `t = 4`, B = 6--10, falling to 0.056--0.061 at `t = 9`, B ≈ 260);
    the 90% interval excludes 1 in 15 of the 18 cells. At w = 0.1 and w
    \= 1 it is 239× to 7.3·10⁵× dearer, and the gap widens with B.
  - #strong[The break-even weight w\* stops falling.] It falls to about
    10⁻⁶ by `t = 7` and then stays at 1.2--1.7·10⁻⁶ for `t = 7, 8, 9`
    (narrow intervals). E3's prediction that it keeps falling was for F4
    and is not tested at `t ≥ 6`, because F4 was not run there.
  - #strong[Against rho the order also flips inside the band.] At w =
    0.1 and 1 the grid is 1.9·10⁴ to 1.6·10⁶× `rho-neg` (w = 0.1) and
    1.1·10⁵ to 9.6·10⁶× (w = 1). At w = 0 it is 0.78--1.70× at `t ≥ 7`
    and #emph[below] rho at `t = 8` on e3-c5 and e3-c6 (the 90% interval
    excludes 1 only on e3-c6), where w = 0 prices all of the grid's
    solving work at nothing. Under AGENTS.md §5a that is unresolved, and
    it is not an improvement.
+ #strong[F4 run to full solves (Part F): closed at `t = 2` only.] At
  `t = 2` (both presentations, targets 0 and 1) the measured cost equals
  E3's substitution to five decimals at every weight. This is the weak
  case for the check: only 3 of 434 258 calls were consistent systems.
  #strong[`t = 3` was not completed.] Its runs died twice with the host
  session (deviations D1, D2), and the work they need is about 46--220 h
  for the chain arm on this host, above the spec's 72 h timeout (see
  "Why `t = 3` was not completed"). So E3's substitution is now checked
  against full F4 solves on a 24-bit curve at `t = 2` and on a 12-bit
  control, and not at any `t ≥ 3`.

#page(flipped: true, margin: 1.2cm)[
#figure(image("e3b/e3b_ladder.svg", width: 100%, alt: "E3b ladder"),
  caption: [
    E3b ladder
  ]
)

#emph[Figure 1. Top: S = GAE/√r of a full solve against B at w = 0, 0.1
and 1; solid = E3b (curves e3-c4/c5/c6, markers o/s/^), faint = E3
(c1/c2/c3, `t = 2…5`); blue = grid reference, red = known-log MITM,
dashed purple = F4 chain, #strong[substituted, not run] (from E3),
dotted grey = `rho-neg` per curve. Bottom: grid / known-log paired by
target. 8 targets per point, 90% bootstrap intervals. Sessions
ECBS1h33f9cde203a7, ECBS1h12ea835a948e, ECBS1h478cd952402d.]
]

== What ran
<what-ran>
#figure(
  align(center)[#table(
    columns: (50%, 50%),
    align: (left,left,),
    table.header([item], [value],),
    table.hline(),
    [curves], [e3-c4 `p = 10398721`, e3-c5 `p = 15052801`, e3-c6
    `p = 15161857`: 24-bit, `a = −3`, prime order, 2⁹ divides p − 1
    (table in the protocol)],
    [ladder arms], [per curve and `t = 4…9`: `sem-grid-t{t}`
    (`prime-kummer:t={t}` + `pkm-semaev:m=3,engine=exhaustive`),
    `kl-mitm3-t{t}` (`known-log:size=B,scalars=random` +
    `mitm:m=3,negation_folded=1`), plus `rho-neg`\; 8 `planted_shared`
    targets per curve, target seed 20261019, 1 round, no warm-up],
    [ladder sessions], [e3-c4 `ECBS1h33f9cde203a7`, e3-c5
    `ECBS1h12ea835a948e`, e3-c6 `ECBS1h478cd952402d`: 312 of 312 runs,
    all `verified`],
    [F4 full-solve arms], [planned: `sem-chain-t2`, `sem-full-t2`
    (targets 0, 1), `sem-chain-t3` (target 0), `sem-full-t3` (targets 0,
    1), E3's oracle strings with `crosscheck=1` and no `max_trials`, on
    E3's e3-c1 and E3's targets. #strong[Completed: the `t = 2` arms
    only]],
    [F4 sessions], [`t = 2`: chain `ECBS1h305ebf1e98d2`, full
    `ECBS1hbe41d11e8904` (4 of 4 runs each, all `verified`). `t = 3`:
    none completed (D1, D2)],
    [binary], [the E3 build, `ecbench` sha256 `b72458fbfa802e88…`, the
    binary of every E3 record],
    [host], [macOS arm64, shared, heavily loaded, no CPU pinning
    (`--cpus none`), isolation L0. Operation counts only; wall time is a
    practicality note],
    [aggregation], [`examples/generator_effect_e3_summary.rs` (extended
    additively: E3's `summary.json` is reproduced exactly) →
    #link("e3b/summary.json")[`e3b/summary.json`]\;
    `examples/generator_effect_e3b_f4check.rs` →
    #link("e3b/f4check.json")[`e3b/f4check.json`]\; 10 000 resamples,
    seed 20261019 (E3b's added statistics, seed 20261020); pricing
    through `ecbench::band`],
    [records], [#link("e3b/sessions/")[`e3b/sessions/`], specs in
    #link("e3b/specs/")[`e3b/specs/`], sizes in
    #link("e3b/sizes.jsonl")[`e3b/sizes.jsonl`]],
    [graphs], [#link("e3b/e3b_ladder.svg")[`e3b/e3b_ladder.svg`],
    #link("e3b/e3b_f4.svg")[`e3b/e3b_f4.svg`] (PNG beside them), from
    #link("e3b/plot_e3b.py")[`e3b/plot_e3b.py`], which plots the two
    JSON files only],
  )]
  , kind: table
  )

`S(w) = (G − C + w·(U + n_C)) / √r` as in E3, with `C = n_C = 0` (fresh
curves, no pinned calibration).

== Results, Part L
<results-part-l>
=== 1. Yield at equal B (L1 fails by one cell)
<yield-at-equal-b-l1-fails-by-one-cell>
`N₃` of the two bases agree within 0.3% in every cell (ratio known-log /
Kummer 0.9985--1.0030). The Wilson 90% interval of the measured hit rate
covers `h = N₃/r` in #strong[33 of 36] (curve, t, arm) cells. The three
misses are e3-c4 `t = 5` for both arms (grid 3.61·10⁻⁴ \[3.08,
4.23\]·10⁻⁴ and known-log 5.8·10⁻⁴ \[3.3, 10.4\]·10⁻⁴ against
`h = 2.85·10⁻⁴`) and e3-c4 `t = 9` for the grid (0.892 \[0.880, 0.902\]
against 0.903). L1 required at least 34, so it #strong[fails]. 90%
intervals are expected to miss about 3.6 of 36 cells by chance, which is
what happened, so this is read as a sampling effect and not a defect in
the oracle. The known-log arm stops at its first relation, so its trials
are geometric waiting times and a fixed-n interval is only approximate
for them.

Hit rates and exact counts (Wilson 90% intervals):

#figure(
  align(center)[#table(
    columns: (10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%),
    align: (left,right,right,right,right,right,left,left,right,right,),
    table.header([curve], [t], [B], [N3 Kummer], [N3 known-log], [h =
      N3/r], [grid hit rate \[Wilson 90%\]], [known-log hit rate
      \[Wilson 90%\]], [grid trials], [kl trials],),
    table.hline(),
    [e3-c4], [4], [6], [304], [304], [2.922e-05], [2.480e-05 \[1.97e-05,
    3.12e-05\]], [3.793e-05 \[2.14e-05,
    6.73e-05\]], [2056689], [210898],
    [e3-c4], [5], [13], [2964], [2964], [2.849e-04], [3.607e-04
    \[3.08e-04, 4.23e-04\]], [5.832e-04 \[3.29e-04,
    1.03e-03\]], [296685], [13717],
    [e3-c4], [6], [29], [32518], [32534], [3.126e-03], [2.814e-03
    \[2.51e-03, 3.15e-03\]], [2.500e-03 \[1.41e-03,
    4.43e-03\]], [73558], [3200],
    [e3-c4], [7], [68], [411836], [411584], [3.959e-02], [3.822e-02
    \[3.56e-02, 4.10e-02\]], [3.463e-02 \[1.96e-02,
    6.04e-02\]], [13422], [231],
    [e3-c4], [8], [128], [2457910], [2460400], [2.363e-01], [2.422e-01
    \[2.31e-01, 2.54e-01\]], [2.353e-01 \[1.38e-01,
    3.72e-01\]], [3943], [34],
    [e3-c4], [9], [263], [9390715], [9396807], [9.027e-01], [8.917e-01
    \[8.80e-01, 9.02e-01\]], [1.000e+00 \[7.47e-01,
    1.00e+00\]], [2114], [8],
    [e3-c5], [4], [9], [996], [996], [6.614e-05], [6.321e-05 \[5.25e-05,
    7.61e-05\]], [4.391e-05 \[2.47e-05,
    7.79e-05\]], [1233909], [182201],
    [e3-c5], [5], [19], [9196], [9196], [6.107e-04], [6.322e-04
    \[5.52e-04, 7.25e-04\]], [3.649e-04 \[2.06e-04,
    6.47e-04\]], [229354], [21925],
    [e3-c5], [6], [35], [57166], [57216], [3.796e-03], [4.110e-03
    \[3.72e-03, 4.54e-03\]], [2.176e-03 \[1.23e-03,
    3.86e-03\]], [64967], [3676],
    [e3-c5], [7], [69], [430302], [431594], [2.857e-02], [2.809e-02
    \[2.61e-02, 3.02e-02\]], [3.077e-02 \[1.74e-02,
    5.38e-02\]], [18086], [260],
    [e3-c5], [8], [134], [2889312], [2892498], [1.919e-01], [1.990e-01
    \[1.90e-01, 2.08e-01\]], [1.739e-01 \[1.01e-01,
    2.83e-01\]], [4996], [46],
    [e3-c5], [9], [276], [12716067], [12719539], [8.444e-01], [8.518e-01
    \[8.39e-01, 8.64e-01\]], [7.273e-01 \[4.80e-01,
    8.85e-01\]], [2314], [11],
    [e3-c6], [4], [10], [1360], [1360], [8.969e-05], [8.348e-05
    \[6.97e-05, 1.00e-04\]], [6.838e-05 \[3.85e-05,
    1.21e-04\]], [994242], [116993],
    [e3-c6], [5], [18], [7824], [7824], [5.160e-04], [4.788e-04
    \[4.16e-04, 5.51e-04\]], [5.346e-04 \[3.01e-04,
    9.49e-04\]], [286160], [14964],
    [e3-c6], [6], [30], [35990], [36002], [2.373e-03], [2.356e-03
    \[2.11e-03, 2.63e-03\]], [3.279e-03 \[1.85e-03,
    5.81e-03\]], [96794], [2440],
    [e3-c6], [7], [61], [298996], [299670], [1.972e-02], [2.016e-02
    \[1.87e-02, 2.18e-02\]], [1.677e-02 \[9.48e-03,
    2.95e-02\]], [22417], [477],
    [e3-c6], [8], [129], [2611328], [2607494], [1.722e-01], [1.785e-01
    \[1.70e-01, 1.87e-01\]], [1.212e-01 \[6.97e-02,
    2.03e-01\]], [5463], [66],
    [e3-c6], [9], [253], [11521250], [11514803], [7.598e-01], [7.580e-01
    \[7.43e-01, 7.72e-01\]], [8.000e-01 \[5.41e-01,
    9.31e-01\]], [2413], [10],
  )]
  , kind: table
  )

=== 2. Known-log MITM's optimum (L2 holds)
<known-log-mitms-optimum-l2-holds>
Mean S at w = 0, B at its minimum: e3-c4 3.67 (B = 68), e3-c5 3.26 (B =
69), e3-c6 3.65 (B = 61), all at `t = 7`, and above `rho-neg` at every t
on every curve (3.2--4.4× at the minimum; intervals in the table). Its
cost is higher on both sides of this t in every curve (the ladder has
one point per t).

=== 3. S at the band weights (L3 holds)
<s-at-the-band-weights-l3-holds>
Full table at w = 0, with the break-even weight (S in GAE/√r, mean of 8;
intervals are 90% bootstrap, ratios paired by target):

#figure(
  align(center)[#table(
    columns: (10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%, 10%),
    align: (left,right,right,right,right,right,left,left,right,right,),
    table.header([curve], [t], [B], [S grid], [S known-log], [S
      rho], [grid / kl \[90%\]], [grid / rho \[90%\]], [kl /
      rho], [w\*],),
    table.hline(),
    [e3-c4], [4], [6], [80.41], [106.79], [0.83], [0.753 \[0.549,
    1.037\]], [96.73 \[65.56, 143.44\]], [128.47], [4.5e-05],
    [e3-c4], [5], [13], [12.23], [14.88], [0.83], [0.822 \[0.576,
    1.196\]], [14.71 \[11.34, 20.45\]], [17.90], [4.5e-06],
    [e3-c4], [6], [29], [3.41], [8.20], [0.83], [0.415 \[0.277,
    0.634\]], [4.10 \[3.54, 4.85\]], [9.87], [4.4e-06],
    [e3-c4], [7], [68], [1.15], [3.67], [0.83], [0.313 \[0.278,
    0.357\]], [1.38 \[1.16, 1.74\]], [4.41], [1.7e-06],
    [e3-c4], [8], [128], [1.02], [7.01], [0.83], [0.145 \[0.139,
    0.150\]], [1.22 \[1.01, 1.56\]], [8.44], [1.7e-06],
    [e3-c4], [9], [263], [1.41], [24.48], [0.83], [0.058 \[0.057,
    0.058\]], [1.70 \[1.40, 2.19\]], [29.45], [1.6e-06],
    [e3-c5], [4], [9], [40.35], [111.94], [1.01], [0.361 \[0.247,
    0.658\]], [39.97 \[31.12, 54.54\]], [110.88], [2.4e-04],
    [e3-c5], [5], [19], [8.02], [28.09], [1.01], [0.286 \[0.184,
    0.526\]], [7.95 \[6.93, 9.55\]], [27.83], [5.3e-05],
    [e3-c5], [6], [35], [2.55], [9.31], [1.01], [0.273 \[0.169,
    0.565\]], [2.52 \[1.97, 3.40\]], [9.22], [8.5e-06],
    [e3-c5], [7], [69], [1.10], [3.26], [1.01], [0.338 \[0.304,
    0.374\]], [1.09 \[0.85, 1.46\]], [3.23], [1.3e-06],
    [e3-c5], [8], [134], [0.89], [6.43], [1.01], [0.139 \[0.135,
    0.143\]], [0.89 \[0.71, 1.17\]], [6.37], [1.5e-06],
    [e3-c5], [9], [276], [1.25], [22.42], [1.01], [0.056 \[0.055,
    0.056\]], [1.23 \[0.98, 1.62\]], [22.21], [1.6e-06],
    [e3-c6], [4], [10], [32.52], [79.31], [1.14], [0.410 \[0.287,
    0.688\]], [28.55 \[22.52, 34.98\]], [69.63], [2.0e-04],
    [e3-c6], [5], [18], [9.80], [18.31], [1.14], [0.535 \[0.342,
    0.947\]], [8.61 \[7.14, 10.48\]], [16.07], [1.8e-05],
    [e3-c6], [6], [30], [3.56], [5.56], [1.14], [0.641 \[0.450,
    1.021\]], [3.13 \[2.77, 3.65\]], [4.88], [1.7e-06],
    [e3-c6], [7], [61], [1.22], [3.65], [1.14], [0.335 \[0.265,
    0.447\]], [1.07 \[0.90, 1.26\]], [3.21], [1.2e-06],
    [e3-c6], [8], [129], [0.89], [6.19], [1.14], [0.144 \[0.141,
    0.148\]], [0.78 \[0.66, 0.92\]], [5.43], [1.3e-06],
    [e3-c6], [9], [253], [1.17], [19.00], [1.14], [0.061 \[0.061,
    0.062\]], [1.02 \[0.87, 1.21\]], [16.68], [1.3e-06],
  )]
  , kind: table
  )

- #strong[w = 0.] The grid's point estimate is below known-log at all 18
  cells; the interval excludes 1 in 15 and contains it in 3 (e3-c4
  `t = 4, 5`\; e3-c6 `t = 6`). At w = 0 the grid pays only target draws
  and lifts, about one group addition a trial, while known-log MITM pays
  for a pair table of about B² points.
- #strong[w = 0.1 and w = 1.] The grid is above known-log in every cell
  (row intervals in the table below), by at least 239× (w = 0.1) and
  1.3·10³ (w = 1) at `t = 4`, at least 7.8·10³ at `t = 6`, rising to
  6.0--7.4·10⁴ (w = 0.1) and 6.0--7.3·10⁵ (w = 1) at `t = 9`:

#figure(
  align(center)[#table(
    columns: (16.67%, 16.67%, 16.67%, 16.67%, 16.67%, 16.67%),
    align: (left,right,left,left,right,right,),
    table.header([curve], [t], [grid / kl at w = 0.1], [grid / kl at w =
      1], [grid / rho at w = 0.1], [kl / rho at w = 0.1],),
    table.hline(),
    [e3-c4], [4], [499 \[366, 687\]], [2.75e+03 \[2.02e+03,
    3.79e+03\]], [6.49e+04], [130],
    [e3-c4], [5], [3.61e+03 \[2.49e+03, 5.23e+03\]], [2.01e+04
    \[1.39e+04, 2.98e+04\]], [6.51e+04], [18.1],
    [e3-c4], [6], [1.21e+04 \[7.89e+03, 1.88e+04\]], [6.96e+04
    \[4.53e+04, 1.11e+05\]], [1.2e+05], [9.89],
    [e3-c4], [7], [4e+04 \[3.48e+04, 4.71e+04\]], [3.11e+05 \[2.61e+05,
    3.87e+05\]], [1.68e+05], [4.2],
    [e3-c4], [8], [4.96e+04 \[4.63e+04, 5.34e+04\]], [4.79e+05
    \[4.4e+05, 5.21e+05\]], [3.86e+05], [7.8],
    [e3-c4], [9], [6.05e+04 \[5.87e+04, 6.22e+04\]], [6.05e+05
    \[5.87e+05, 6.21e+05\]], [1.64e+06], [27.1],
    [e3-c5], [4], [239 \[164, 430\]], [1.32e+03 \[898,
    2.36e+03\]], [2.68e+04], [112],
    [e3-c5], [5], [1.23e+03 \[783, 2.23e+03\]], [6.8e+03 \[4.3e+03,
    1.24e+04\]], [3.44e+04], [28],
    [e3-c5], [6], [7.81e+03 \[4.75e+03, 1.63e+04\]], [4.48e+04
    \[2.67e+04, 9.81e+04\]], [7.2e+04], [9.22],
    [e3-c5], [7], [5.02e+04 \[4.4e+04, 5.68e+04\]], [3.86e+05
    \[3.29e+05, 4.52e+05\]], [1.54e+05], [3.07],
    [e3-c5], [8], [5.68e+04 \[5.32e+04, 6.04e+04\]], [5.42e+05
    \[4.97e+05, 5.87e+05\]], [3.34e+05], [5.88],
    [e3-c5], [9], [6.01e+04 \[5.94e+04, 6.07e+04\]], [5.99e+05
    \[5.92e+05, 6.05e+05\]], [1.22e+06], [20.4],
    [e3-c6], [4], [272 \[189, 456\]], [1.5e+03 \[1.04e+03,
    2.53e+03\]], [1.9e+04], [70.1],
    [e3-c6], [5], [2.34e+03 \[1.49e+03, 4.22e+03\]], [1.3e+04
    \[8.23e+03, 2.35e+04\]], [3.78e+04], [16.2],
    [e3-c6], [6], [1.95e+04 \[1.37e+04, 3.08e+04\]], [1.14e+05
    \[7.81e+04, 1.88e+05\]], [9.47e+04], [4.86],
    [e3-c6], [7], [5.46e+04 \[4.27e+04, 7.4e+04\]], [3.8e+05 \[2.82e+05,
    5.59e+05\]], [1.69e+05], [3.09],
    [e3-c6], [8], [6.42e+04 \[6.1e+04, 6.75e+04\]], [5.99e+05
    \[5.66e+05, 6.33e+05\]], [3.22e+05], [5.02],
    [e3-c6], [9], [7.37e+04 \[7.24e+04, 7.49e+04\]], [7.35e+05
    \[7.22e+05, 7.47e+05\]], [1.13e+06], [15.3],
  )]
  , kind: table
  )

- #strong[§5a verdict.] The order of the grid and known-log MITM changes
  between w = 0 and w = 0.1 at every (curve, t), so the order does not
  hold across the band. #strong[Unresolved. Not a win for either.]

=== 4. Crossover in B (L4 holds, with a correction to E3's trend)
<crossover-in-b-l4-holds-with-a-correction-to-e3s-trend>
- #strong[No `B*` exists at any weight.] The ratio grid / known-log is
  below 1 at every t for w = 0 and above 1 at every t for w = 0.1 and 1.
- #strong[`w*` is below 10⁻³ at every `t ≥ 6`] (largest 8.5·10⁻⁶, e3-c5
  `t = 6`).
- #strong[`w*` does not keep falling.] From `t = 7` it is flat at
  1.2--1.7·10⁻⁶ on all three curves (90% intervals up to about ±0.4·10⁻⁶
  at `t = 7`, ±0.1·10⁻⁶ or narrower at `t = 8, 9`). The grid's
  multiplications per call grow 6.9--7.9× per level (7 440 at `t = 4`,
  180 797 952 at `t = 9`) while the exact `h` grows 3.8--13× per level
  (slower past `t = 7`, where it saturates toward 1). Why `w*` levels
  off is not derived here; the plateau is the measured fact.
- #strong[What this does and does not say about F4.] E3's F4 `w*` fell
  49--880× per level over `t = 2…5`. F4 was not run at `t ≥ 6` (one
  `t = 6` system took 44 minutes and 14 GB in the PKM note §16).
  Extending E3's measured growth of F4's cost (67--406× per level) puts
  F4 far above the grid at every `t ≥ 6`, and so above known-log at
  every w \> 0 there. #strong[That is an extrapolation, not a
  measurement.] No F4 number at `t ≥ 6` appears in any table or figure
  of this report.

=== 5. Against rho (L5 fails)
<against-rho-l5-fails>
The prediction was that the grid stays above `rho-neg` at w = 0 at every
t, and is closest at `t = 9`. Measured, grid / rho-neg at w = 0, mean of
8 with 90% interval:

#figure(
  align(center)[#table(
    columns: (25%, 25%, 25%, 25%),
    align: (left,left,left,left,),
    table.header([curve], [t = 7], [t = 8], [t = 9],),
    table.hline(),
    [e3-c4], [1.38 \[1.16, 1.74\]], [1.22 \[1.01, 1.56\]], [1.70 \[1.40,
    2.19\]],
    [e3-c5], [1.09 \[0.85, 1.46\]], [#strong[0.89 \[0.71,
    1.17\]]], [1.23 \[0.98, 1.62\]],
    [e3-c6], [1.07 \[0.90, 1.26\]], [#strong[0.78 \[0.66,
    0.92\]]], [1.02 \[0.87, 1.21\]],
  )]
  , kind: table
  )

- At `t = 8` the grid is below rho on e3-c5 and e3-c6; the interval
  excludes 1 only on e3-c6. The closest approach to rho is at `t = 8` on
  every curve, not at `t = 9`. #strong[L5 fails on both counts.]
- The part of L5 that holds: at w = 0.1 the grid is 1.9·10⁴ to 1.6·10⁶
  times rho and at w = 1 it is 1.1·10⁵ to 9.6·10⁶ times (smallest at
  `t = 4`--`5`, largest at `t = 9`).
- #strong[What w = 0 means.] It prices the grid's 2.3·10⁷ field
  multiplications per call at `t = 8` (1.8·10⁸ at `t = 9`) at zero. On
  e3-c6 at `t = 8` the grid's S rises by about 4.0·10⁶ per unit w
  (derived from its S at w = 0.1), so a price above about 6·10⁻⁸ GAE per
  multiplication puts it back above rho, and at w = 0.1 it is 3.2·10⁵×
  rho. Rho is ahead across the rest of the band. Against rho too the
  order flips inside the band, so it is unresolved under §5a. This is a
  statement about the pricing rule, not a faster algorithm.
- Known-log MITM is above rho at every t and w on every curve (interval
  excludes 1 in all 54 cells): 3.1--130× at w = 0.1 and 2.3--138× at w =
  \1.

== Results, Part F
<results-part-f>
=== F4 against E3's substitution at `t = 2`
<f4-against-e3s-substitution-at-t-2>
Four full F4 solves on e3-c1 (chain and full presentation, E3's targets
0 and 1), each run to a verified recovery:

#figure(
  align(center)[#table(
    columns: (12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%, 12.5%),
    align: (left,right,right,right,right,right,right,right,),
    table.header([arm], [target], [trials], [relations], [F4 calls], [F4
      mult-adds per call], [E3 capped mean], [measured / substituted U],),
    table.hline(),
    [`sem-chain-t2`], [0], [444 341], [3], [434
    258], [3.9079·10⁵], [3.9079·10⁵], [1.00000],
    [`sem-chain-t2`], [1], [841 257], [3], [805
    655], [3.9079·10⁵], [3.9079·10⁵], [1.00000],
    [`sem-full-t2`], [0], [444 341], [3], [434
    258], [2.2429·10⁵], [2.2429·10⁵], [1.00000],
    [`sem-full-t2`], [1], [841 257], [3], [805
    655], [2.2429·10⁵], [2.2429·10⁵], [1.00000],
  )]
  , kind: table
  )

- #strong[F1 holds] for these runs: trials, relations and group GAE
  equal those of E3's `sem-grid-t2` record of the same target exactly,
  `same_stream` is true in every row, and `pkm_crosscheck_disagreements`
  is 0 with 0 budget overruns.
- #strong[F2 holds] for these runs: measured and substituted S agree to
  five decimals at w = 0, 0.1 and 1.
- The agreement is nearly forced at `t = 2`: only 3 of the 434 258 calls
  were consistent systems, so the refutation mean carries the sum, and a
  consistent system costs 0.5--1.5× a refutation. The check is exact on
  the three things it tests (same stream, per-call count by shape,
  accounting), and weak on the consistent-system cost at this t.
- Solver wall time (host shared and heavily loaded): chain 3.7 h and 1.5
  h, full 3.4 h and 1.3 h for targets 0 and 1.

#figure(image("e3b/e3b_f4.svg", width: 100%, alt: "F4 against E3’s substitution at t = 2"),
  caption: [
    F4 against E3's substitution at t = 2
  ]
)

#emph[Figure 2. e3-c1, `t = 2`, F4 run on every trial. Left: S measured
\/ S substituted (E3's formula) at w = 0, 0.1, 1 for the four runs;
shaded band is F2's \[0.99, 1.01\]. Right: F4 multiply-adds per call,
measured (bars) against E3's capped mean (crosses). Sessions
ECBS1h305ebf1e98d2, ECBS1hbe41d11e8904; all 4 runs verified.]

=== F4 against E3's substitution at `t = 3`: not completed
<f4-against-e3s-substitution-at-t-3-not-completed>
No `t = 3` F4 record exists in this report. The runs were started twice
(the launch and one rerun) and killed with the host session each time
(D1, D2); the first interrupted `sem-full-t3` target 0 is not used (D1).

==== Why `t = 3` was not completed
<why-t-3-was-not-completed>
Derived from recorded values, not measured at `t = 3`:

- The `t = 2` solves took longer per F4 call than E3's capped runs:
  chain 30.6 ms (target 0) and 6.5 ms (target 1) per call against E3's
  1.64 ms (655 ms over 400 calls), i.e.~4.0× to 18.7×. Full: 28.2 and
  5.9 ms against 1.41 ms, 4.2× to 20×. The spread is the load on a
  shared host.
- E3's capped `t = 3` runs cost 108 ms (chain) and 55 ms (full) per
  call. A `t = 3` solve of target 0 makes about 381 500 F4 calls (the
  interrupted `sem-full-t3` run recorded 381 501, the same stream as the
  grid). That is 11.5 h (chain) and 5.8 h (full) at E3's speed, and with
  the same 4--20× load factor about 46--220 h (chain) and 24--117 h
  (full) per target. The spec timeout is 72 h.
- Two sessions did not live that long (D1, D2), and a third in-session
  attempt is very unlikely to finish.

What is still needed: a detached launch that outlives the session, the
E3 binary (sha256 `b72458fb…`), a timeout of several days, and ideally
an idle host. `t = 3` is a follow-up, not a result.

== Predictions against outcome
<predictions-against-outcome>
#figure(
  align(center)[#table(
    columns: (33.33%, 33.33%, 33.33%),
    align: (left,left,left,),
    table.header([], [prediction], [outcome],),
    table.hline(),
    [L1], [hit rates cover `N₃/r` in ≥ 34 of 36 cells; `N₃` within
    1%], [#strong[fails by one cell]: 33 of 36 (3 misses, all on e3-c4);
    `N₃` within 0.3%: holds],
    [L2], [known-log lowest at `t = 7` on every curve; 1.5--10× rho
    there], [holds: B = 68, 69, 61; 4.4×, 3.2×, 3.2×],
    [L3], [w = 0 grid below known-log at every t; ≥ 10³ above at
    `t ≥ 6`, w ≥ 0.1; unresolved], [holds in point estimates (interval
    excludes 1 in 15 of 18); smallest `t ≥ 6` factor 7.8·10³],
    [L4], [no `B*`\; `w*` \< 10⁻³ at `t ≥ 6`\; "w\* falls"], [no `B*`
    and `w*` \< 10⁻³: hold; #strong[w\* is flat from `t = 7`], not
    falling],
    [L5], [grid above rho at w = 0 at every t, closest at `t = 9`,
    within 3×; ≥ 10³ above at w ≥ 0.1], [#strong[fails]: below rho at
    `t = 8` on e3-c5 and e3-c6, closest at `t = 8`\; the w ≥ 0.1 part
    holds],
    [L6], [F4 at `t ≥ 6` only "extrapolated, not run"], [holds],
    [F1], [same stream as E3's grid record; verified; 0
    disagreements], [`t = 2` holds (4 of 4). `t = 3`: not tested,
    interrupted twice (D1, D2)],
    [F2], [measured / substituted within \[0.99, 1.01\]], [`t = 2` holds
    (all within 3.3·10⁻⁶ of 1). `t = 3`: not tested (D1, D2)],
  )]
  , kind: table
  )

== Scope and limits
<scope-and-limits>
- #strong[Population:] a restricted prime family (`2^t | p − 1`), three
  curves per part, `m = 3`, 24-bit group orders, 8 targets per curve. B
  up to 276 reaches the known-log arm's optimum, but for `m = 3` only,
  and not E1's `m = 2` optimum of about 220--250, where the pair table
  is the whole cost.
- #strong[Pricing:] w = 0 prices the grid's field multiplications at
  zero. Every statement about the grid at w = 0 depends on that choice.
- #strong[F4:] run to full solves only at `t = 2` (this report) and on
  E3's 12-bit control, and as capped runs at `t ≤ 5` in E3. At `t ≥ 3`
  E3's F4 full-solve cost remains a substitution. At `t ≥ 6` F4 is not
  run and the report makes no F4 claim there beyond the labelled
  extrapolation.
- #strong[The grid is a reference, not a proposed algorithm.] It decides
  the same systems as F4 with fewer operations; it is not a security
  claim.
- #strong[No security statement.] These are generic precomputation
  shapes and no exponent moves. This is not an improvement over the IC
  baseline: the grid is above known-log at every w \> 0, and ahead of or
  behind rho depending on the price of unpriced work.

== Interrupted runs (protocol deviations D1 and D2)
<interrupted-runs-protocol-deviations-d1-and-d2>
The host session ended twice while the `t = 3` F4 runs were in progress.
The three ladder sessions and the `t = 2` F4 sessions had completed both
times.

- #strong[D1.] The first pair (`ECBS1hf02ca1ecbf63` chain,
  `ECBS1hdfef6661171e` full) were left `running` with partial records
  (full: target 0 of `sem-full-t3` verified, 389 357 trials, 381 501 F4
  calls; chain: no F4 record).
- #strong[D2.] The rerun pair (`ECBS1h345b571362e6` chain,
  `ECBS1hb680db5cb1a8` full) was started at about 18:08 PDT on
  2026-10-25 and ended with the session before any F4 call finished.
  Each wrote only its `rho-neg` record (chain: target 0; full: target 0)
  and no F4 record.

All four partial directories are kept outside the repository (they are
marked `running` with no sealed record hash and would not pass the audit
of a sealed session) and no number of theirs enters any table. The D1
`sem-full-t3` target-0 record is not used either: choosing it now, after
the rerun failed, would be a post-hoc choice. Part F at `t = 3` is
therefore not reported.

== Files
<files>
- `E3b-PROTOCOL.md`: the frozen protocol, with deviations D1 and D2.
- `reports/run-20261021T0512Z-e3b/`: the run report (observations only),
  with its PDF.
- `e3b/specs/`, `e3b/sessions/`, `e3b/sizes.jsonl`: specs, records,
  sizes.
- `e3b/summary.json`, `e3b/f4check.json`: aggregates.
- `e3b/plot_e3b.py`, `e3b/e3b_ladder.{svg,png}`, `e3b/e3b_f4.{svg,png}`.
- `examples/generator_effect_e3_summary.rs`: grid `w*` and crossover
  added.
- `examples/generator_effect_e3b_f4check.rs`: Part F aggregation.
- `E3b.pdf` (this report), built by `build_e3b_pdf.sh`.
