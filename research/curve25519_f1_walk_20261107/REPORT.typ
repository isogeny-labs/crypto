#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set table(inset: 4pt, stroke: 0.4pt + gray)
#show table: set text(size: 7.5pt)
= One-point (F \= 1) walk-table oracle vs the F \= 2 incumbent,
single-target ecbench (2026-11-07)
<one-point-f--1-walk-table-oracle-vs-the-f--2-incumbent-single-target-ecbench-2026-11-07>
#strong[Class.] Measured, single-target ecbench sessions under a
protocol frozen before data (#link("PROTOCOL.md"); hashes in
`data/frozen.sha256`). Counted units only (ecbench GAE, S \=
cost/sqrt(r)), every comparison at w \= 0, 0.1, 1. Public synthetic
Curve25519-family miniatures; no security claim. No pcc claim has been
filed for these numbers. Each one is an unverified assertion until it is
reviewed \[pcc:unverified\].

#strong[Question.] The lab\'s generic incumbent `wwalk`
(`x=0.9,g=1,grow=2,seed=run` on a 2-abscissa base) has F \= 2 unknown
base logarithms. Does a one-seed walk table (F \= 1) cost less, end to
end, for a single target? \[pcc:unverified\]

#strong[Answer.] Yes. At k \= 20 the primary F \= 1 arm (`f1-x07`) costs
0.807x the F \= 2 incumbent at w \= 0, with 95 % interval \[0.777,
0.839\]. Its interval stays below 1 at w \= 0.1 and at w \= 1
\[pcc:unverified\]. At k \= 24 the F \= 1 arm costs 0.859x \[0.819,
0.901\] at w \= 0 \[pcc:unverified\]. Rho-neg remains ahead. At k \= 20
the remaining distance is 1.50x at w \= 0, 1.43x at w \= 0.1 and 1.08x
at w \= 1, and every interval excludes parity \[pcc:unverified\]. The w
\= 0 distance equals the derived F \= 1 collision floor of 1.5x
\[pcc:unverified\].

#image("figures/ratio_to_rho.svg")

== 1. What changed in the code
<1-what-changed-in-the-code>
`WalkTable::build_with` (`src/cryptanalysis/ic_boundary.rs`) now accepts
a factor base of exactly one abscissa when `g=1`. In that mode the jumps
are `[a_j]P1 + [c_j]G` and the entries are `u*P1 + z*G`. A verified key
collision gives the one-column affine row `du*s1 = -dz`, which pins
`log P1`. A target probe hit gives the target\'s row. `grow=1|2` keeps
walking until the verified rows reach rank 1, capped at 2T or 3T. A
one-seed base without `g=1` is refused, because its row would be empty.
The 2-seed path keeps its random draws, walk and rows unchanged. The
base is specified as `prime-projected-abscissa:size=1,scan_cap=32`. The
ecbench help text and the oracle documentation say so.

Tests (`data/cargo-test-wwalk.txt`: 7 passed, 0 failed;
`data/cargo-test-plan-refuses.txt`: 6 passed) include the existing wwalk
tests, unchanged, and the new
`wwalk_one_seed_rows_are_affine_and_grow_reaches_rank_one`. That test
checks the charging ledgers, `v = 0`, one-column rows that hold under
brute-force logs, probe answers that rebuild the target, growth stopping
at rank 1, and the refusals.

== 2. Design (as frozen)
<2-design-as-frozen>
- Binary: release build of this tree in `/home/user/ctarget-f1`
  (`data/binary.sha256`). The existing `/home/user/ctarget` binary is
  untouched.
- 8 sessions per curve size, each with `targets_per_curve = 1` (one
  public target per session) and its own target and measurement seeds
  (`scripts/make_specs.py`, `specs/`). Each arm ran 64 measured rounds
  plus 1 warm-up, in alternate order.
- Arms: `rho-neg` (reference), `rho-aa` (A/A), and `f2-inc` (F \= 2
  incumbent: size\=2 base, `wwalk:x=0.9,g=1,grow=2,seed=run`). The
  candidates `f1-x05`, `f1-x07` (primary) and `f1-x09` use a size\=1
  base and `wwalk:x=X,g=1,grow=2,seed=run`. All pipeline arms use
  `targets=stride-generator`.
- Audit: `ecbench verify --replay 1` for every session
  (`data/verify-k*-s*.txt`).
- Pooling: `scripts/pooled_compare.py`, copied unchanged from the T12
  study. It takes the ratio of per-session sums with a linearised t
  interval and df \= 7 (one session \= one target \= one cluster).
  `scripts/analyse.py` writes `data/pooled.json`.
- Runtime: about 52 s of sessions at k \= 20 and about 60 s at k \= 24.
  Both sizes ran in full, well inside the 90-minute cap.

== 3. Results (measured)
<3-results-measured>
Every session completed. Each wrote 390 of 390 records, all verified,
and its audit reproduced 1 of 1 replays with 0 problems
(`data/verify-*.txt`) \[pcc:unverified\].

Mean S per arm (3,120 records per size; source `data/pooled.json`, which
pools `sessions/k*-s*/records.jsonl`):

#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,left,right,right,right,).at(col),
  inset: 6pt,
  [k], [arm], [S w\=0], [S w\=0.1], [S w\=1],
  [20],
  [rho-neg],
  [1.552],
  [1.664],
  [2.672],
  [20],
  [f2-inc (F\=2)],
  [2.875],
  [2.921],
  [3.337],
  [20],
  [f1-x05],
  [2.293],
  [2.361],
  [2.973],
  [20],
  [#strong[f1-x07]],
  [2.321],
  [2.378],
  [2.891],
  [20],
  [f1-x09],
  [2.328],
  [2.381],
  [2.856],
  [24],
  [rho-neg],
  [1.161],
  [1.265],
  [2.205],
  [24],
  [f2-inc (F\=2)],
  [2.134],
  [2.174],
  [2.532],
  [24],
  [f1-x05],
  [1.819],
  [1.888],
  [2.513],
  [24],
  [#strong[f1-x07]],
  [1.834],
  [1.894],
  [2.438],
  [24],
  [f1-x09],
  [1.856],
  [1.910],
  [2.398],
)
]

The F \= 2 incumbent at k \= 24 (2.134) agrees with the
rebaseline-seedrun figure of 2.105 for the same arm on the same curve
\[pcc:unverified\].

Pooled ratios with 95 % intervals over 8 single-target sessions
(`data/pooled.json`):

#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,left,left,left,left,).at(col),
  inset: 6pt,
  [k], [ratio], [w \= 0], [w \= 0.1], [w \= 1],
  [20],
  [f1-x07 / f2-inc],
  [0.807 \[0.777, 0.839\]],
  [0.814 \[0.782, 0.847\]],
  [0.866 \[0.820, 0.915\]],
  [20],
  [f1-x05 / f2-inc],
  [0.798 \[0.775, 0.821\]],
  [0.808 \[0.786, 0.831\]],
  [0.891 \[0.865, 0.918\]],
  [20],
  [f1-x09 / f2-inc],
  [0.810 \[0.774, 0.848\]],
  [0.815 \[0.777, 0.855\]],
  [0.856 \[0.801, 0.915\]],
  [20],
  [f1-x07 / rho-neg],
  [1.495 \[1.454, 1.538\]],
  [1.429 \[1.388, 1.471\]],
  [1.082 \[1.044, 1.120\]],
  [20],
  [f2-inc / rho-neg],
  [1.852 \[1.781, 1.925\]],
  [1.755 \[1.686, 1.826\]],
  [1.249 \[1.189, 1.311\]],
  [20],
  [rho-aa / rho-neg],
  [1.000 \[1.000, 1.000\]],
  [1.000],
  [1.000],
  [24],
  [f1-x07 / f2-inc],
  [0.859 \[0.819, 0.901\]],
  [0.871 \[0.829, 0.916\]],
  [0.963 \[0.899, 1.032\]],
  [24],
  [f1-x05 / f2-inc],
  [0.852 \[0.820, 0.886\]],
  [0.869 \[0.833, 0.906\]],
  [0.992 \[0.934, 1.054\]],
  [24],
  [f1-x09 / f2-inc],
  [0.870 \[0.844, 0.896\]],
  [0.879 \[0.851, 0.907\]],
  [0.947 \[0.903, 0.993\]],
  [24],
  [f1-x07 / rho-neg],
  [1.580 \[1.472, 1.696\]],
  [1.497 \[1.393, 1.610\]],
  [1.106 \[1.018, 1.202\]],
  [24],
  [f2-inc / rho-neg],
  [1.839 \[1.767, 1.914\]],
  [1.718 \[1.649, 1.790\]],
  [1.148 \[1.093, 1.207\]],
)
]

== 4. Predictions (PROTOCOL.md section 5)
<4-predictions-protocolmd-section-5>
#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,left,).at(col),
  inset: 6pt,
  [id], [prediction], [outcome], [status],
  [P1],
  [k\=20 f1-x07/f2-inc upper end \< 1 at w \= 0, 0.1, 1],
  [0.839, 0.847, 0.915],
  [#strong[held] \[pcc:unverified\]],
  [P1b],
  [k\=20 f1-x07/f2-inc point in \[0.80, 0.95\] at w \= 0],
  [0.807],
  [#strong[held] (near the lower edge) \[pcc:unverified\]],
  [P2],
  [k\=20 f1-x07/rho-neg point in \[1.45, 1.85\] at w \= 0],
  [1.495],
  [#strong[held] \[pcc:unverified\]],
  [P3],
  [k\=20 f1-x07/rho-neg lower end \> 1 at all w (no parity)],
  [1.454, 1.388, 1.044],
  [#strong[held] \[pcc:unverified\]],
  [P4],
  [rho-aa/rho-neg interval contains 1],
  [1.000 exactly],
  [#strong[held, but degenerate] \[pcc:unverified\]],
  [P5],
  [k\=20 f1-x05 and f1-x09 / f2-inc upper end \< 1 at w \= 0],
  [0.821, 0.848],
  [#strong[held] \[pcc:unverified\]],
  [P6],
  [k\=24 f1-x07/f2-inc upper end \< 1 at w \= 0],
  [0.901],
  [#strong[held] \[pcc:unverified\]],
)
]

P4 is degenerate. The rho-aa and rho-neg records are identical, run for
run, because ecbench derives the run seed from the workload and round,
not from the arm name. The A/A arm therefore gives no noise floor. The
real noise is the between-session spread, and the t intervals carry it
\[pcc:unverified\]. Not scored, but reported: at k \= 24 and w \= 1, two
of the three F \= 1 / F \= 2 intervals include 1.00, so the k \= 24
ordering holds at w \= 0 and w \= 0.1 but is unresolved at w \= 1 for x
\= 0.5 and 0.7 \[pcc:unverified\].

== 5. Distance to matched rho (rho-neg, same sessions)
<5-distance-to-matched-rho-rho-neg-same-sessions>
#align(center)[#table(
  columns: 4,
  align: (col, row) => (right,right,right,right,).at(col),
  inset: 6pt,
  [k], [w \= 0], [w \= 0.1], [w \= 1],
  [20, F \= 1 (f1-x07)],
  [1.50x],
  [1.43x],
  [1.08x],
  [20, F \= 2 (f2-inc)],
  [1.85x],
  [1.76x],
  [1.25x],
  [24, F \= 1 (f1-x07)],
  [1.58x],
  [1.50x],
  [1.11x],
  [24, F \= 2 (f2-inc)],
  [1.84x],
  [1.72x],
  [1.15x],
)
]

The F \= 1 arm closes about 40 % of the F \= 2 incumbent\'s w \= 0 gap
to rho at k \= 20 ((1.852 - 1.495) / 0.852, derived) and about 31 % at k
\= 24 ((1.839 - 1.580) / 0.839, derived) \[pcc:unverified\]. The gap
narrows at w \= 1 because rho-neg\'s unpriced native work, about one
uncharged negation canonicalisation per walk step
(`canonicalisations_uncharged`), is larger per GAE than the walk-table
arms\' lookups and row operations \[pcc:unverified\].

== 6. Reading and limits
<6-reading-and-limits>
- #strong[What the gain is.] F \= 1 needs two collisions: one walk
  self-collision to pin `log P1`, then one table hit for the target. F
  \= 2 needs three. The measured gain is the drop from three collisions
  to two. The frozen model (PROTOCOL.md section 4) predicted a ratio of
  0.876 (derived). The measured ratio is 0.807 at k \= 20 and 0.859 at k
  \= 24 \[pcc:unverified\].
- #strong[How close this is to rho.] F \= 1 is the last index-calculus
  step before F \= 0. F \= 0 (a base of known logarithms) is rho or BSGS
  relabelled and is excluded by the success rule. The F \= 1 arm still
  has a base element of unknown logarithm, solved by its own collision.
  It is not rho relabelled, but the factor base does no decomposition
  work: the arm is a two-collision generic algorithm. Whether the lab
  counts it as an index-calculus improvement or as a step toward
  relabelling is a call for the reviewer. This README makes no
  leaderboard claim \[pcc:unverified\].
- #strong[The floor still binds.] The w \= 0 distance of 1.495x sits on
  the derived F \= 1 collision floor of 1.5x. Under the generic model,
  no generic F \= 1 variant can go much lower. The k \= 20 rho-neg S of
  1.55 is far above the model\'s 0.886, because small-r overheads
  inflate both arms. One sampled k \= 20 rho-neg record has 54 set-up
  additions and 76 set-up doublings beside 123 walk operations. At k \=
  20 the ratio therefore matches the floor partly by cancellation of
  these overheads \[pcc:unverified\].
- #strong[Model gap.] The asymmetric walk-table model predicted 1.75x
  rho-model for F \= 1 and 1.99x for F \= 2 (derived). The measured
  ratios to measured rho-neg are lower because rho-neg\'s measured
  overhead factor is larger than the walk-table arms\'
  \[pcc:unverified\].
- #strong[Not done.] The study has no VALIDATION.json, no replay
  certificate or validator report, and no leaderboard or
  `benchmarks/README.md` row. Those are for the landing session under
  the validation gate. The raw `records.jsonl` files are in
  `data/sessions-k{20,24}-records.tar.gz`, which is git-ignored per
  session, and their hashes are in the benchrec records.

== Files
<files>
`PROTOCOL.md`, `specs/`,
`scripts/{make_specs.py,run_sessions.sh,pooled_compare.py,analyse.py,make_figure.py}`,
`sessions/k*-s*/{session,spec,plan,host}.json`, `data/` (plans, run
logs, bands, verify reports, `pooled.json`, `binary.sha256`,
`frozen.sha256`, `cargo-test-wwalk.txt`, record tarballs),
`figures/ratio_to_rho.{svg,png}`, `REPORT.pdf`. Benchmark records:
`benchmarks/records/20261107T1945Z-curve25519-f1-walk-k20.json` and
`benchmarks/records/20261107T1947Z-curve25519-f1-walk-k24.json`.
