#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
= One-point (F \= 1) walk-table oracle vs the F \= 2 incumbent,
single-target ecbench (2026-11-07)
<one-point-f-1-walk-table-oracle-vs-the-f-2-incumbent-single-target-ecbench-2026-11-07>
#strong[Class.] Measured, single-target ecbench sessions under a
protocol frozen before data (#link("PROTOCOL.md"); hashes in
`data/frozen.sha256`). Counted units only (ecbench GAE, S \=
cost/sqrt(r)), every comparison at w \= 0, 0.1, 1. Public synthetic
Curve25519-family miniatures; no security claim. No pcc claim has been
filed for these numbers. Each one is an unverified assertion until it is
reviewed \[pcc:unverified\].

#strong[Question.] The lab’s generic incumbent `wwalk`
(`x=0.9,g=1,grow=2,seed=run` on a 2-abscissa base) has F \= 2 unknown
base logarithms. Does a one-seed walk table (F \= 1) cost less, end to
end, for a single target? \[pcc:unverified\]

#strong[Answer.] Yes. At k \= 20 the primary F \= 1 arm (`f1-x07`) costs
0.809x the F \= 2 incumbent at w \= 0, with 95 % interval \[0.776,
0.842\]. Its interval stays below 1 at w \= 0.1 and at w \= 1
\[pcc:unverified\]. At k \= 24 the F \= 1 arm costs 0.855x \[0.814,
0.898\] at w \= 0 \[pcc:unverified\]. Rho-neg remains ahead. At k \= 20
the remaining distance is 1.50x at w \= 0, 1.43x at w \= 0.1 and 1.08x
at w \= 1, and every interval excludes parity \[pcc:unverified\]. At k
\= 24 it is 1.57x, 1.49x and 1.10x \[pcc:unverified\]. The gain is
measured against the generic `wwalk` incumbent and moves the method
toward rho; it is not a decomposition-based index-calculus improvement
(section 7) \[pcc:unverified\].

#figure([#image("figures/ratio_to_rho.png")],
  caption: [
    Pooled ratio to rho-neg
  ]
)

== 1. What changed in the code
<what-changed-in-the-code>
`WalkTable::build_with` (`src/cryptanalysis/ic_boundary.rs`) now accepts
a factor base of exactly one abscissa when `g=1`. In that mode the jumps
are `[a_j]P1 + [c_j]G` and the entries are `u*P1 + z*G`
\[pcc:unverified\]. A verified key collision gives the one-column affine
row `du*s1 = -dz`, which pins `log P1` \[pcc:unverified\]. A target
probe hit gives the target’s row. `grow=1|2` keeps walking until the
verified rows reach rank 1, capped at 2T or 3T. A one-seed base without
`g=1` is refused, because its row would be empty. The 2-seed path keeps
its random draws, walk and rows unchanged. The base is specified as
`prime-projected-abscissa:size=1,scan_cap=32`. The ecbench help text and
the oracle documentation say so.

Tests (`data/cargo-test-wwalk.txt`: 7 passed, 0 failed;
`data/cargo-test-plan-refuses.txt`: 6 passed) include the existing wwalk
tests, unchanged, and the new
`wwalk_one_seed_rows_are_affine_and_grow_reaches_rank_one`. That test
checks the charging ledgers, `v = 0`, one-column rows that hold under
brute-force logs, probe answers that rebuild the target, growth stopping
at rank 1, and the refusals.

== 2. Design (as frozen)
<design-as-frozen>
- Binary: release build of this tree in `/home/user/ctarget-f1`
  (`data/binary.sha256`). The existing `/home/user/ctarget` binary is
  untouched.
- 8 sessions per curve size, each with `targets_per_curve = 1` (one
  public target per session) and its own target and measurement seeds
  (`scripts/make_specs.py`, `specs/`). Each arm ran 64 measured rounds
  plus 1 warm-up (warm-up records are excluded from every pooled figure,
  see section 7), in alternate order.
- Arms: `rho-neg` (reference), `rho-aa` (A/A), and `f2-inc` (F \= 2
  incumbent: size\=2 base, `wwalk:x=0.9,g=1,grow=2,seed=run`). The
  candidates `f1-x05`, `f1-x07` (primary) and `f1-x09` use a size\=1
  base and `wwalk:x=X,g=1,grow=2,seed=run`. All pipeline arms use
  `targets=stride-generator`.
- Audit: `ecbench verify --replay 1` for every session
  (`data/verify-k*-s*.txt`).
- Pooling: `scripts/pooled_compare.py`, copied from the T12 study and
  changed once, after validation, to drop `warmup: true` records
  (section 7). It takes the ratio of per-session sums with a linearised
  t interval and df \= 7 (one session \= one target \= one cluster).
  `scripts/analyse.py` writes `data/pooled_measured.json` (measured
  rounds only).
- Runtime: about 52 s of sessions at k \= 20 and about 60 s at k \= 24.
  Both sizes ran in full, well inside the 90-minute cap.

== 3. Results (measured)
<results-measured>
Every session completed. Each wrote 390 of 390 records, all verified,
and its audit reproduced 1 of 1 replays with 0 problems
(`data/verify-*.txt`) \[pcc:unverified\]. Pooled figures use the 384
measured records per session (6 arms x 64 rounds); the 6 warm-up records
per session are excluded \[pcc:unverified\].

Mean S per arm (3,072 measured records per size; source
`data/pooled_measured.json`, which pools
`sessions/k*-s*/records.jsonl`):

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
  [2.871],
  [2.916],
  [3.330],
  [20],
  [f1-x05],
  [2.296],
  [2.365],
  [2.979],
  [20],
  [#strong[f1-x07]],
  [2.321],
  [2.379],
  [2.894],
  [20],
  [f1-x09],
  [2.331],
  [2.384],
  [2.861],
  [24],
  [rho-neg],
  [1.164],
  [1.269],
  [2.212],
  [24],
  [f2-inc (F\=2)],
  [2.138],
  [2.178],
  [2.540],
  [24],
  [f1-x05],
  [1.812],
  [1.880],
  [2.495],
  [24],
  [#strong[f1-x07]],
  [1.828],
  [1.888],
  [2.429],
  [24],
  [f1-x09],
  [1.858],
  [1.912],
  [2.402],
)
]

The F \= 2 incumbent at k \= 24 (2.138) agrees with the
rebaseline-seedrun figure of 2.105 for the same arm on the same curve
\[pcc:unverified\].

Pooled ratios with 95 % intervals over 8 single-target sessions
(`data/pooled_measured.json`):

#align(center)[#table(
  columns: 5,
  align: (col, row) => (right,left,left,left,left,).at(col),
  inset: 6pt,
  [k], [ratio], [w \= 0], [w \= 0.1], [w \= 1],
  [20],
  [f1-x07 / f2-inc],
  [0.809 \[0.776, 0.842\]],
  [0.816 \[0.782, 0.851\]],
  [0.869 \[0.820, 0.921\]],
  [20],
  [f1-x05 / f2-inc],
  [0.800 \[0.780, 0.821\]],
  [0.811 \[0.790, 0.832\]],
  [0.894 \[0.871, 0.919\]],
  [20],
  [f1-x09 / f2-inc],
  [0.812 \[0.776, 0.849\]],
  [0.817 \[0.780, 0.857\]],
  [0.859 \[0.804, 0.918\]],
  [20],
  [f1-x07 / rho-neg],
  [1.496 \[1.454, 1.538\]],
  [1.429 \[1.388, 1.472\]],
  [1.083 \[1.046, 1.122\]],
  [20],
  [f2-inc / rho-neg],
  [1.849 \[1.774, 1.927\]],
  [1.752 \[1.679, 1.829\]],
  [1.246 \[1.183, 1.313\]],
  [20],
  [rho-aa / rho-neg],
  [1.000 \[1.000, 1.000\]],
  [1.000],
  [1.000],
  [24],
  [f1-x07 / f2-inc],
  [0.855 \[0.814, 0.898\]],
  [0.867 \[0.823, 0.913\]],
  [0.956 \[0.892, 1.026\]],
  [24],
  [f1-x05 / f2-inc],
  [0.847 \[0.816, 0.880\]],
  [0.863 \[0.829, 0.899\]],
  [0.982 \[0.927, 1.042\]],
  [24],
  [f1-x09 / f2-inc],
  [0.869 \[0.842, 0.897\]],
  [0.878 \[0.849, 0.907\]],
  [0.946 \[0.901, 0.992\]],
  [24],
  [f1-x07 / rho-neg],
  [1.570 \[1.465, 1.684\]],
  [1.488 \[1.386, 1.597\]],
  [1.098 \[1.013, 1.191\]],
  [24],
  [f2-inc / rho-neg],
  [1.836 \[1.763, 1.913\]],
  [1.716 \[1.646, 1.790\]],
  [1.148 \[1.091, 1.208\]],
)
]

== 4. Predictions (PROTOCOL.md section 5)
<predictions-protocol.md-section-5>
#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,left,).at(col),
  inset: 6pt,
  [id], [prediction], [outcome], [status],
  [P1],
  [k\=20 f1-x07/f2-inc upper end \< 1 at w \= 0, 0.1, 1],
  [0.842, 0.851, 0.921],
  [#strong[held] \[pcc:unverified\]],
  [P1b],
  [k\=20 f1-x07/f2-inc point in \[0.80, 0.95\] at w \= 0],
  [0.809],
  [#strong[held] (near the lower edge) \[pcc:unverified\]],
  [P2],
  [k\=20 f1-x07/rho-neg point in \[1.45, 1.85\] at w \= 0],
  [1.496],
  [#strong[held] \[pcc:unverified\]],
  [P3],
  [k\=20 f1-x07/rho-neg lower end \> 1 at all w (no parity)],
  [1.454, 1.388, 1.046],
  [#strong[held] \[pcc:unverified\]],
  [P4],
  [rho-aa/rho-neg interval contains 1],
  [1.000 exactly],
  [#strong[not informative] (degenerate; not scored as held)
  \[pcc:unverified\]],
  [P5],
  [k\=20 f1-x05 and f1-x09 / f2-inc upper end \< 1 at w \= 0],
  [0.821, 0.849],
  [#strong[held] \[pcc:unverified\]],
  [P6],
  [k\=24 f1-x07/f2-inc upper end \< 1 at w \= 0],
  [0.898],
  [#strong[held] \[pcc:unverified\]],
)
]

P4 is not informative: the check is degenerate. The rho-aa and rho-neg
records are identical, run for run, because ecbench derives the run seed
from the workload and round, not from the arm name. The A/A arm
therefore gives no noise floor. The real noise is the between-session
spread, and the t intervals carry it \[pcc:unverified\]. Not scored, but
reported: at k \= 24 and w \= 1, two of the three F \= 1 / F \= 2
intervals include 1.00, so the k \= 24 ordering holds at w \= 0 and w \=
0.1 but is unresolved at w \= 1 for x \= 0.5 and 0.7 \[pcc:unverified\].

== 5. Distance to matched rho (rho-neg, same sessions)
<distance-to-matched-rho-rho-neg-same-sessions>
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
  [1.75x],
  [1.25x],
  [24, F \= 1 (f1-x07)],
  [1.57x],
  [1.49x],
  [1.10x],
  [24, F \= 2 (f2-inc)],
  [1.84x],
  [1.72x],
  [1.15x],
)
]

At w \= 0 the F \= 1 arm closes 42 % of the F \= 2 incumbent’s gap to
rho at k \= 20 ((1.849 - 1.496) / 0.849, derived) and 32 % at k \= 24
((1.836 - 1.570) / 0.836, derived) \[pcc:unverified\]. At w \= 0.1 the
figures are 43 % (k \= 20) and 32 % (k \= 24); at w \= 1 they are 66 %
and 34 % (derived) \[pcc:unverified\]. The gap narrows at w \= 1 because
rho-neg’s unpriced native work, about one uncharged negation
canonicalisation per walk step (`canonicalisations_uncharged`), is
larger per GAE than the walk-table arms’ lookups and row operations
\[pcc:unverified\].

== 6. Reading and limits
<reading-and-limits>
- #strong[What the gain is.] F \= 1 needs two collisions: one walk
  self-collision to pin `log P1`, then one table hit for the target. F
  \= 2 needs three. The measured gain is the drop from three collisions
  to two. The frozen model (PROTOCOL.md section 4) predicted a ratio of
  0.876 (derived). The measured ratio is 0.809 at k \= 20 and 0.855 at k
  \= 24 \[pcc:unverified\].
- #strong[Scope of the claim.] The F \= 1 arm does no decomposition
  work. It has one base point of unknown logarithm, pinned by a walk
  self-collision, and finds the target by a generic table hit. With P1
  :\= G (known logarithm) it becomes F \= 0, a rho/BSGS-style walk
  table, which costs less. The measured gain (three collisions down to
  two) is therefore a gain over the generic `wwalk` incumbent that moves
  toward rho. It is not a decomposition-based index-calculus improvement
  and is not comparable with the decomposition-based rows of the IC
  leaderboard, which are a different baseline. No leaderboard claim is
  made, and none without an owner ruling and the validation gate
  \[pcc:unverified\].
- #strong[The 1.5x floor does not bind.] The k \= 20 w \= 0 distance
  (1.496x) is numerically close to the derived F \= 1 collision floor of
  1.5x, and the agreement is a coincidence. The floor is relative to the
  model rho (S \= 0.886), whereas the measured k \= 20 rho-neg has S \=
  1.55 (1.75x the model) because of small-r set-up overhead. One sampled
  k \= 20 rho-neg record has 54 set-up additions and 76 set-up doublings
  beside 123 walk operations. The ratio lands near 1.5 because of this
  overhead cancelling, not because the floor binds. At k \= 24 the
  distance is 1.57x \[pcc:unverified\]. No inference about a lower bound
  for generic F \= 1 variants is drawn from it \[pcc:unverified\].
- #strong[Model gap.] The asymmetric walk-table model predicted 1.75x
  rho-model for F \= 1 and 1.99x for F \= 2 (derived). The measured
  ratios to measured rho-neg are lower because rho-neg’s measured
  overhead factor is larger than the walk-table arms’
  \[pcc:unverified\].
- #strong[Not done.] The study has no VALIDATION.json and no replay
  certificate (the independent validator report is
  `validator-report.json`, status pass\_with\_errata), and no
  leaderboard or `benchmarks/README.md` row. Those are for the landing
  session under the validation gate. The raw `records.jsonl` files are
  in `data/sessions-k{20,24}-records.tar.gz`, which is git-ignored per
  session, and their hashes are in the benchrec records.

== 7. Validation
<validation>
An independent validator (`validator-report.json`, status
pass\_with\_errata, not edited here) found the following, and this
README applies its errata \[pcc:unverified\].

- #strong[Warm-up records.] The frozen design is 64 measured rounds plus
  1 warm-up per arm and session, but `scripts/pooled_compare.py`
  (`load`) pooled the warm-up round as well (3,120 records per size
  instead of 3,072). The loader now skips records with `"warmup": true`;
  `scripts/analyse.py`, `data/pooled_measured.json` (new file) and
  `figures/ratio_to_rho.*` were regenerated and every number above was
  updated \[pcc:unverified\]. Pooled means and ratios move by at most
  0.011 (ratios) and 0.007 (S); the largest change is the k \= 24 f1-x07
  / f2-inc ratio at w \= 0, 0.859 to 0.855 \[pcc:unverified\]. No
  prediction verdict changed; P1, P1b, P2, P3, P5 and P6 held on the
  same criteria \[pcc:unverified\].
- #strong[P4 is not informative.] rho-aa and rho-neg are identical run
  for run (65 of 65 runs in every session) because ecbench seeds by
  workload and round. The A/A arm measures nothing, so P4 is not scored
  as held \[pcc:unverified\].
- #strong[The 1.5x floor.] The agreement of 1.496x with the floor is a
  coincidence of rho-neg’s small-r overhead (rho-neg S \= 1.55 against a
  model 0.886; k \= 24 gives 1.57x), not evidence that the floor binds
  \[pcc:unverified\].
- #strong[Gap closed.] The earlier "about 40 %" is replaced by the exact
  figures, w \= 0 only: 42 % at k \= 20 and 32 % at k \= 24; at w \=
  0.1, 43 % and 32 %; at w \= 1, 66 % and 34 % \[pcc:unverified\].
- #strong[Scope.] F \= 1 is a generic walk-table variant that moves
  toward rho (section 6), not a decomposition-based index-calculus
  improvement \[pcc:unverified\].
- #strong[Binary and source.] The measuring binary
  (`data/binary.sha256`) was built before the last edit of
  `src/cryptanalysis/ic_boundary.rs`, so byte identity between the
  binary and the source is not proven. Binary check (main session,
  2026-11-07): the measuring binary (sha256 prefix 44442a4ba260bbfb) was
  built before the last edit of `ic_boundary.rs`. A rebuild from the
  committed source has a different hash (1d1a57c7aadc5779). Under the
  rebuilt binary, `ecbench verify --replay-all` on a copy of session
  k20-s0 reproduced 384 of 384 replays identically, with 0 problems, so
  the late edit did not change any counted result in that session
  \[pcc:unverified\].

== Files
<files>
`PROTOCOL.md`, `specs/`,
`scripts/{make_specs.py,run_sessions.sh,pooled_compare.py,analyse.py,make_figure.py}`,
`sessions/k*-s*/{session,spec,plan,host}.json`, `data/` (plans, run
logs, bands, verify reports, `pooled.json` (the first analysis, warm-up
included, kept unchanged because the closed benchmark records pin its
hash), `pooled_measured.json`, `binary.sha256`, `frozen.sha256`,
`cargo-test-wwalk.txt`, record tarballs),
`figures/ratio_to_rho.{svg,png}`, `REPORT.pdf`. Benchmark records:
`benchmarks/records/20261107T1945Z-curve25519-f1-walk-k20.json` and
`benchmarks/records/20261107T1947Z-curve25519-f1-walk-k24.json`.
