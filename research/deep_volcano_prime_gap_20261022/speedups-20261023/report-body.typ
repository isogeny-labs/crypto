= Speed-ups to the deep-volcano prime-gap study (2026-10-23)
<speed-ups-to-the-deep-volcano-prime-gap-study-2026-10-23>
The request was \"make it faster\", for four parts:

+ the cross-gap N-isogeny;
+ the measurement pipeline;
+ the rho solve;
+ CI turnaround.

Each change below keeps the computed output identical, or, for rho,
keeps the walk law. Each is measured before and after on this host. The
study\'s result (#link("../README.md")[`../README.md`]) does not change;
nothing here re-measures or re-reads it.

All numbers below are #strong[measured], except where marked
#emph[derived]. The source of each is `summary.json`, written by
`summarize.sh` (jq) from the raw files listed under \"Raw data\".

#box(image("figures/pipeline-stages.svg"))

== Summary
<summary>
#figure(
  align(center)[#table(
    columns: 5,
    align: (left,right,right,right,left,),
    table.header([stage], [before], [after], [ratio], [output],),
    table.hline(),
    [`ecbench plan`, main spec (4560 runs), 1 CPU], [96.6 s], [0.308
    s], [314×], [plan text byte-identical],
    [N-isogeny kernel and Vélu set-up, 1 CPU], [0.154 s], [0.047
    s], [3.28×], [isogeny identical (28 runs)],
    [N-isogeny map, one point, 1 CPU], [0.0186 s], [0.0188 s], [0.99×
    (none)], [identical],
    [N-isogeny map, one point, 4 CPUs], [0.0186 s], [0.0063 s], [2.95×
    (threads)], [identical],
    [`volcano-gap construct`, whole, 4 CPUs], [0.885 s], [0.282
    s], [3.13×], [identical],
    [measurement sessions, 6840 runs, 4 vCPU], [2120 s], [734
    s], [2.89×], [all 6840 records identical],
    [rho solve, median per solve, same session], [362 ms], [155
    ms], [wall ratio 0.419 \[0.417, 0.421\]], [same walk law; counted
    cost +2.5% \[2.1, 2.9\]%],
  )]
  , kind: table
  )

The first five rows come from `timing.sh`. Its runs are interleaved
before/after and pinned (`taskset`). They report the median of 3 rounds
(plan) or 7 rounds (construct). The before binaries are built from the
parent commits.

The sessions row is the recorded span from the first session start to
the last session end:

- before: `../run/sessions`, 2026-10-22, three sessions;
- after: `rerun-sharded/sessions`, 2026-10-23, five sessions.

Both ran on the same container class, but on different days and as
different session sets, so that row is descriptive.

The rho row is from one paired ecbench session (`ECBS1h45f937d2f95a`),
with both arms interleaved. Its interval is `ecbench compare`\'s.

== 1. The N-isogeny (`volcano-gap`, commit `bec9df9e8`)
<1-the-n-isogeny-volcano-gap-commit-bec9df9e8>
- #strong[Kernel.] The kernel abscissae `x(iK)`, `i = 1 … (N−1)/2`, are
  accumulated in Jacobian coordinates by mixed additions, with one batch
  inversion at the end. Before, they took one affine addition and
  inversion each. The counted field multiplications of the set-up fall
  from 43,882,856 to 8,524,264 (`run/pair.json` against
  `rerun-sharded/pair.json`), and the dual\'s from 75,454,536 to
  36,942,784.
- #strong[Map.] The map `φ(P)` runs in Montgomery form, split into one
  chunk per thread, with one batch inversion per chunk. On one CPU it is
  no faster (0.99×). The 2.95× on four CPUs comes from the threads, not
  from less work. The study needs only two map evaluations per instance,
  so this matters little.
- #strong[Test.] `fast_isogeny_matches_reference` checks both against
  the affine code they replaced (kept under `cfg(test)`), on the
  study\'s own class.

== 2. The measurement pipeline (commit `0bf3db1a3`)
<2-the-measurement-pipeline-commit-0bf3db1a3>
- #strong[Planning.] `ecbench plan` re-parsed the 4.7 MB embedded curve
  registry (`docs/curves/registry.json`) for every workload curve\'s
  slug lookup. The registry is now parsed once into a `LazyLock`, with a
  slug index: the first entry per slug wins, as the linear scan found
  it. The plan output is byte-identical (one sha256 over all six timed
  runs). Planning was most of the old 2 m 40 s `run_pipeline.sh build`,
  and the build now takes about 3 s.
- #strong[Sharding.] `volcano-gap spec --shards K` writes
  `spec-shard{k}.json` (every K-th curve), and `run_pipeline.sh` runs
  the shards alongside the A/A and BSGS sessions. ecbench\'s algorithm
  seed is `derive_u64(measurement seed, workload, round)`, which does
  not depend on execution order. So the sharded run must record exactly
  what the original did.
- #strong[Sharded re-run.] All 6840 records are identical by arm,
  workload, target, round, warmup, counted cost, recovered value and
  every counter: `rerun-sharded/compare.sh`, digests in
  `compare.sha256`. Every session passes `ecbench verify --replay 8`
  (`rerun-sharded/sessions/*.audit.json`).

== 3. The rho solve: `rho.negation_lanes` (commit `d81da47d5`)
<3-the-rho-solve-rhonegation_lanes-commit-d81da47d5>
#box(image("figures/rho-speed.svg"))

#strong[Where the time went.] `rho.negation` spent most of each step in
one modular inversion of the affine addition. A microbenchmark of
`invmod` on this host (not committed; descriptive) took about 200 ns, of
the measured 294 ns per group operation.

#strong[What changed.] `rho_walk_lanes` advances 32 tuned walks in
lockstep. It hands each round\'s additions to the prime curve\'s native
lane adder (`PrimeCurve::native_lane_walk_adder`: Montgomery form, one
batch inversion per round).

#strong[What stays the same.] Each walk follows `TunedWalker::walk`\'s
rules from the same start:

- the same set-up and seed draws, the same jumps;
- the same Wiener--Zuccherato look-ahead and cycle escape by doubling;
- the same distinguished points and cap;
- starts are handed out in order as lanes finish.

What changes is the order in which distinguished points reach the table,
so a seed\'s collision, and its count, differ. At one lane it reproduces
`rho_walk_with` exactly: the same steps, collision and counters on 24
seeds (`lane_rho_recovers_replays_and_batches_its_inversions`).

#figure(
  align(center)[#table(
    columns: 6,
    align: (left,right,right,right,right,right,),
    table.header([arm], [runs], [verified], [median ns / GAE], [median
      solve], [mean S],),
    table.hline(),
    [`rho.negation` (serial)], [576], [576], [293.9], [362.1
    ms], [0.9303],
    [`rho.negation_lanes` (32 lanes)], [576], [576], [119.4], [154.6
    ms], [0.9538],
  )]
  , kind: table
  )

- #strong[Setting.] 48 curve models (every 16th of the main spec: both
  sides, every level), 4 targets each, 3 rounds, warmup 1. Isolation
  L1/L2, `--cpus auto`.
- #strong[Counted cost.] `ecbench compare` gives lanes/serial =
  #strong[1.0252 \[1.0213, 1.0295\]] over 576 pairs. The lanes cost 2.5%
  more operations, mostly the walks still in flight when the collision
  is found (32 walks of about 2^10 steps each).
- #strong[Wall time.] The median wall ratio is #strong[0.4193 \[0.4173,
  0.4214\]], over 306 admitted pairs.
- #strong[Audit.] The session passes `ecbench verify --replay 24`
  (`rho-lanes/session.audit.json`). The §5a band is in
  `rho-lanes/band.json`. Both arms run the same walk, so the band does
  not rank them.

=== Against the fastest external rho at hand (AGENTS.md §15)
<against-the-fastest-external-rho-at-hand-agentsmd-15>
#strong[What ran.] `ca solve --alg rho --threads 1`, from
`aburan28/cryptanalysis` 1aa48b5 (`ca` 0.1.0), pinned to CPU 2 and timed
by its own internal timer. It ran on the same 192 workload targets
(`external_ca.inputs.tsv`, taken from the session\'s records), with 3
seeds each. All 576 solves recovered the planted scalar.

#figure(
  align(center)[#table(
    columns: 4,
    align: (left,right,right,right,),
    table.header([implementation], [median ns / op], [median
      solve], [mean ops / √r],),
    table.hline(),
    [ecbench `rho.negation_lanes`], [119.4], [154.6 ms], [0.954 (GAE)],
    [`ca solve --alg rho`], [45.4], [58.4 ms], [0.976 (its own count)],
  )]
  , kind: table
  )

- #strong[The gap.] #emph[Derived] from these medians:
  `rho.negation_lanes` is still about #strong[2.6×] slower than `ca` per
  solve (154.6/58.4) and per operation (119.4/45.4). That is the
  distance remaining.
- #strong[The operation counts.] Their ops/√r are close, so the gap is
  per-operation cost, not walk length. `ca` counts its own operations.
  We have not established that its count matches ecbench\'s GAE line for
  line, so the per-operation ratio is an estimate. The per-solve ratio
  does not depend on it.
- #strong[Where the rest of the time likely goes.] In ecbench, each step
  still converts points between canonical and Montgomery form, hashes
  the point twice, and runs the cycle window on a `VecDeque`. These are
  the obvious next targets.

== 4. CI turnaround (commit `6d7390d78`)
<4-ci-turnaround-commit-6d7390d78>
`tools/lean/proof-gate.sh` passes the auditor\'s flags after `--`, and
`AuditAxioms.lean` drops a `--` itself. Lean 4.19 needs the separator
and 4.34 passes it through to the script. The
`research/sea_point_count_20261016/lean` job, red on `main` before this
change, is the check: its result on this PR\'s CI is the evidence, and
the self-test passed locally on 4.34.1.

== What is not claimed
<what-is-not-claimed>
- No change to the study\'s result or its counted measurements. The rho
  method the study used (`rho.negation`) is unchanged. The re-run
  sessions, whose records equal the committed ones, replay exactly with
  the new binary.
- `rho.negation_lanes` is not a new reference rho for the IC
  leaderboard. It costs 2.5% more counted operations than `rho.negation`
  and runs 2.4× faster in wall time on this host. An IC row priced in
  GAE against matched rho is unaffected either way.
- The wall times are from one shared 4-vCPU cloud VM (Intel Xeon \@ 2.10
  GHz). Expect ±5--10% residual noise (AGENTS.md §10); no A/A run was
  made for the timing script.

== Raw data
<raw-data>
- `rho-lanes/`:
  - `spec.json`, `spec.plan.txt`\;
  - `session/` (`records.jsonl.gz`, `plan.json.gz`, `session.json`,
    `host.json`), `session.log`, `session.audit.json`\;
  - `session/comparisons/rho-neg__rho-neg-lanes.json`, `band.json`\;
  - `external_ca.sh`, `external_ca.inputs.tsv`, `external_ca.jsonl`.
- `rerun-sharded/`:
  - `specs/` (the shards, A/A and BSGS), `sessions/` (records gzipped),
    `*.audit.json`\;
  - `compare.sh`, `compare.sha256`, `pair.json`\;
  - the build logs.
- `timing.sh`, `timing-plan.jsonl`, `timing-construct.jsonl`.
- `MANIFEST.sha256` pins the uncompressed records and plans.
  `summarize.sh` → `summary.json`, and `plot.py` → `figures/`.
- Validation: #link("validation/")[`validation/`].

== Reproduce
<reproduce>
```
cargo build --release --bin ecbench --bin volcano-gap
D=research/deep_volcano_prime_gap_20261022
$D/run_pipeline.sh build $D/speedups-20261023/rerun-sharded 52 6 16384 20261022 2 6 4 2 3
$D/run_pipeline.sh run   $D/speedups-20261023/rerun-sharded
$D/speedups-20261023/rerun-sharded/compare.sh
target/release/ecbench run --spec $D/speedups-20261023/rho-lanes/spec.json --out <dir> --cpus auto --allow-busy
target/release/ecbench compare --dir <dir> --a rho-neg --b rho-neg-lanes
$D/speedups-20261023/rho-lanes/external_ca.sh      # needs aburan28/cryptanalysis built at 1aa48b5
$D/speedups-20261023/timing.sh OLD_ECBENCH NEW_ECBENCH OLD_VOLCANO_GAP NEW_VOLCANO_GAP 3
```
