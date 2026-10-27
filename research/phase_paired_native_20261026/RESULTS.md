# Native paired fixed-budget comparison: Frobenius phases as solver unknowns, n = 13

**Scope.** A stage diagnostic on a toy field (`y^2 + xy = x^3 + 1` over `F_(2^13)`, 12 frozen targets, 5
orbit representatives): it sets the cost of finding one more independent relation with Frobenius phases
as solver unknowns against representatives only, inside one native solver, and it fixes no `S`, no ratio
to rho and no ECDLP speedup at any size.

Protocol: [`PROTOCOL.md`](PROTOCOL.md) (committed before any run on the frozen manifest; addendum 1
dated after the run). Independent replay: `examples/phase_paired_replay.rs`. Producer:
`examples/phase_paired_fixed_budget.rs`. The legacy Python study (crypto PR #2583, CryptoMiniSat) is
**historical**; none of its seconds are comparable with the seconds here.

## Verdict, in order of how firmly it is established

1. **The legacy structure reproduces exactly (gate passed before any timing was read).** Exhaustive
   native pair matching: arm A decomposes 4 of 12 targets (`b0..b3`), arm B 12 of 12, and the legacy
   sealed brute force agrees. The solver, repetition 1: arm A verified the four `b` targets and proved
   the other eight UNSAT, arm B verified 12 of 12, independent rank 4 and 5 of 5 columns, no budget or
   deadline, no contradiction with the exhaustive check. Every answer in all three repetitions was
   re-verified by the replay (0 defects over 72 receipts).
2. **H1 is not supported by the preregistered rule: the verdict is mixed.** Per newly independent
   verified relation, arm B costs 0.28 to 0.41 times arm A's solver conflicts in every repetition, but
   0.80, 1.07 and 0.63 times arm A's CPU seconds in repetitions 1, 2 and 3 (mean 0.83). The two clocks
   disagree because arm B's conflicts cost 2.5 times as much CPU as arm A's. The rule required both
   clocks below 1 in every repetition; repetition 2 breaks it, and in repetition 2 the arm B cost is
   itself a lower bound (two attempts did not finish).
3. **Native does not agree with the legacy direction on the headline metric, and does agree where the
   arms can be compared like for like.** Legacy (CryptoMiniSat via Python, one trial): arm B 1.61 times
   arm A's CPU per new relation. Native: 0.83 on average (0.63 to 1.07). On the four targets both arms
   decompose (`b0..b3`), arm B costs 6.5 to 12.2 times arm A's CPU and 1.3 to 2.2 times its conflicts,
   which is the legacy direction (legacy pairs: 2.2 to 98.7 times).
4. **Why the headline moved is a measured fact plus one untested explanation.** Arm A's cost is 96 to
   98 per cent proofs that a target is not decomposable (UNSAT): 24 proofs of 0.61 to 1.81 million
   conflicts and 38 to 234 s each, against 8 to 52 s with CryptoMiniSat. Arm B's searches cost 4.6 to 331 s (the two unfinished ones 395 s).
   That the crate's solver lacks the preprocessing that makes CryptoMiniSat refute arm A cheaply is a
   hypothesis; this study does not test it.

![Cost per newly independent verified relation](results/chart_n13.svg)

## What ran

- Host: Intel Xeon @ 2.10 GHz, 4 cores shared, Linux 6.18.44-fc-v153, rustc 1.97.0. Load average at the
  end of every attempt: at most 1.25 (never above 1.3); wall time within 2.6 per cent of CPU time.
- Solver: `crypto_lib::cryptanalysis::sat::Solver`, one thread, native XOR rows, deterministic, **not
  CryptoMiniSat**. Phase scrambling by `scramble_saved_phases(phase_seed(rep))` (the seed is in every
  receipt). Branch priority on the core unknowns (coordinates and phase one-hots).
- Budget: 3,000,000 conflicts per attempt (primary, deterministic); 400 s wall backstop; CPU clock
  `CLOCK_PROCESS_CPUTIME_ID`. One preregistered formulation, `subgroup_xor`, 3 repetitions, ABBA
  pairs, 72 attempts (36 per arm).
- Producer commit `ce20ebf30` (sha256 of `examples/phase_paired_fixed_budget.rs` at that commit, recorded
  in every receipt header: `8fb84627...ac7d`). Later commits changed only the `report` and `freeze`
  subcommands and comments of that file (`git diff ce20ebf -- examples/phase_paired_fixed_budget.rs`
  touches no line of `build_model`, `run_attempt` or `run_cmd`); a freshly built producer reproduces a
  committed attempt's conflict, decision, propagation and restart counts exactly
  (`determinism_check.sh`).
- Commands, from the repository root, release profile, `CARGO_TARGET_DIR=/home/user/cargo-target`:

```sh
d=research/phase_paired_native_20261026
phase_paired_fixed_budget run --out $d/results/receipts_n13.jsonl --forms subgroup_xor --reps 1 \
  --conflict-budget 3000000 --deadline-secs 400 --priority core --cpu-cap 6600
phase_paired_fixed_budget gate --sealed $d/fixtures/witness_sealed.json --receipts $d/results/receipts_n13.jsonl --rep 1
phase_paired_fixed_budget run --resume --out $d/results/receipts_n13.jsonl --forms subgroup_xor --reps 3 \
  --conflict-budget 3000000 --deadline-secs 400 --priority core --cpu-cap 6600   # stopped at the cap
phase_paired_fixed_budget run --resume ... --cpu-cap 8000                          # the last four attempts
phase_paired_replay $d/results/receipts_n13.jsonl $d/results/replay_n13.json --sealed $d/fixtures/witness_sealed.json \
  --tables $d/results/tables_n13.md --svg $d/results/chart_n13.svg
```

## Gate (read before any timing)

[`results/gate_rep1.txt`](results/gate_rep1.txt). Exhaustive native pair matching (no solver; discrete
logarithms in the replay, the group law in the producer): the `b` targets have exactly 1 decomposing
x-quadruple in arm A and 5,300 to 5,500 in arm B (the other targets: none in arm A, 5,300 to 5,500 in
arm B). Solver, repetition 1, arm A: 12 attempts, 4 verified, 8 UNSAT, rank 4 (legacy 4); arm B: 12
attempts, 12 verified, rank 5 (legacy 5); 0 rejected tuples, 0 failed checks, 0 contradictions: **GATE PASS**.

## Results (every attempt charged; full tables in [`results/tables_n13.md`](results/tables_n13.md))

| rep | arm | attempts | verified | UNSAT | unresolved | rank | conflicts | CPU s | conflicts / new relation | CPU s / new relation |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | A | 12 | 4 | 8 | 0 | 4/5 | 10,304,768 | 1091.5 | 2,576,192 | 272.9 |
| 1 | B | 12 | 12 | 0 | 0 | 5/5 | 3,942,027 | 1091.0 | 788,405 | 218.2 |
| 2 | A | 12 | 4 | 8 | 0 | 4/5 | 10,803,619 | 1141.4 | 2,700,905 | 285.3 |
| 2 | B | 12 | 10 | 0 | **2** | 5/5 | 5,488,419 | 1530.1 | 1,097,684 | 306.0 |
| 3 | A | 12 | 4 | 8 | 0 | 4/5 | 11,814,228 | 1319.0 | 2,953,557 | 329.7 |
| 3 | B | 12 | 12 | 0 | 0 | 5/5 | 4,066,825 | 1042.1 | 813,365 | 208.4 |

| rep | B/A conflicts per new relation | B/A CPU per new relation | decisions B/A (exploratory) |
|---|---|---|---|
| 1 | 0.306 | 0.800 | 0.319 |
| 2 | 0.406 | **1.072** (lower bound) | 0.421 |
| 3 | 0.275 | 0.632 | 0.288 |
| mean | 0.329 | 0.835 | |
| pooled over reps | 0.328 | 0.825 | |

Decision rule (PROTOCOL.md): B below A on both clocks in 2 of 3 repetitions, at or above on both in 0:
**mixed, counts as not supported.** Every relation passed every check and no tuple was rejected.

- Where the cost goes: arm A spends 1069.7, 1113.7 and 1273.4 CPU s on its eight UNSAT proofs in
  repetitions 1, 2, 3 (96.5 to 98.0 per cent of its total), 21.8 to 45.6 s on its four relations. Arm B's
  CPU is spread over its 10 to 12 searches (4.6 to 331 s each). Per conflict, arm A costs 107.9 microseconds
  and arm B 271.4 (pooled), which is why a 3.0 times conflict advantage becomes a 1.2 times CPU advantage.
- Like for like, `b0..b3` (sums over the four targets; B/A conflicts, B/A CPU): repetition 1 2.16 and
  12.24; repetition 2 2.05 and 10.87; repetition 3 1.34 and 6.52. Per-target medians of B/A CPU: `b0`
  1.72, `b1` 12.84, `b2` 23.18, `b3` 14.35 (`b0` is the only target where arm B needed fewer conflicts,
  0.27 of arm A's).
- Rank is capped at 5 and arm A can only reach 4 (it decomposes only the `b` targets), so "per new
  relation" divides arm A's cost by 4 and arm B's by 5; arm B's extra relation comes from targets arm A
  can never decompose. Arm B reaches rank 5 after 5 to 7 attempts; its remaining verified relations are
  dependent (7, 5 and 7 dependent relations in repetitions 1, 2, 3; see the rank-trajectory table).

## Native versus legacy

| | legacy (CryptoMiniSat via Python, 1 trial, historical) | native (`sat::Solver`, 3 repetitions) |
|---|---|---|
| arm A: verified / UNSAT, rank | 4 / 8, rank 4 | 4 / 8, rank 4 (all repetitions) |
| arm B: verified, rank | 12, rank 5 | 12, 10 and 12 verified (2 unresolved in rep 2), rank 5 |
| arm A CPU, 12 attempts | 204.9 s | 1091 to 1319 s |
| arm B CPU, 12 attempts | 412.4 s | 1042 to 1530 s |
| B / A CPU per new relation | 1.61 | 0.63 to 1.07 (mean 0.83) |
| B / A CPU on `b0..b3` only | 2.2 to 98.7 per pair | 6.5 to 12.2 per repetition |

The structure agrees exactly. The headline direction does not agree: legacy found B costlier than A on
CPU in both formulations; native finds B cheaper on conflicts always and on CPU in two of three
repetitions. The like-for-like comparison agrees with legacy. The disagreement is carried by arm A's
refutation cost, which is 5.8 times larger natively (1184 against 205 s per repetition) while arm B's
is 3.0 times larger (1221 against 412 s). Whether that is the solver's missing preprocessing, its
branching, or something in the encoding is not established here.

## Deviations and limits, stated plainly

- **A stop condition fired.** In repetition 2 arm B's `a2` and `a3` ended at the 400 s wall backstop
  (1,224,382 and 1,344,608 conflicts, below the 3,000,000 budget) with status `deadline`. PROTOCOL.md
  says any `deadline` stops the analysis; addendum 1 records that the analysis continued as a flagged
  deviation. Those two attempts are charged what they consumed, so repetition 2's arm B cost is a lower
  bound: its conflicts per new relation cannot exceed 1,783,886 (ratio 0.660 against arm A's 2,700,905,
  still below 1); its CPU ratio of 1.072 is a floor. The "not supported" verdict cannot be changed by
  this, because understating arm B's cost can only favour H1.
- **The cap stopped the run at 6,751 s**, before the last four attempts (`c2`, `c3`, repetition 3). Addendum
  1 (written after the first 68 attempts were read, so not blind to them) had them run with the cap
  raised to 8,000 s; the four `skipped` receipts stay in the file as superseded (the replay checks
  this). Total attempt CPU on the frozen manifest: 7,214.9 s (120.2 minutes).
- **Compute exceeded the 90-minute figure.** Attempts 120.2 minutes, calibration on non-frozen targets
  about 33 minutes, builds about 10 (an estimate, not measured): roughly 160 CPU minutes for the session,
  about 1.8 times the nominal allowance. It was accepted in the protocol (arm A's UNSAT proofs are what cost); a budget low enough to
  fit would have left every arm A UNSAT case unresolved.
- **One formulation, three repetitions, one phase seed per repetition, 12 targets.** `line_subgroup`
  is implemented and covered by the same tests but was not run (compute). Run times of a CDCL solver
  are heavy tailed in the seed (arm B on `c1` cost 18 thousand conflicts in repetition 1 and 715 thousand
  in repetition 3), so three repetitions is a small sample; the spread across repetitions is
  reported, not averaged away. All three repetitions use the same 12 targets.
- **Targets favour arm B by construction.** Arm A can decompose only 4 of 12 targets; the metric
  therefore compares a rank-4 arm with a rank-5 arm and charges arm A for eight proofs of impossibility.
  The like-for-like table is the unbiased view.
- **The encoder differs from the legacy one in one respect** (equal linear forms share a variable) and the
  solver is a different solver; branch priority on the core unknowns was chosen on a rationale, not by
  calibration (calibration did not separate the settings).
- **Calibration was done on non-frozen targets** (target seed 32) with earlier builds; its raw receipts
  are in [`calibration/`](calibration/). A first calibration target set (seed 31) was discarded because
  its first planted target equalled the frozen `a0`.
- **No PDF.** No non-Python PDF renderer exists here (no `typst`, LaTeX or `rsvg-convert`), so the
  report is this Markdown plus [`results/chart_n13.svg`](results/chart_n13.svg). The SVG is well-formed
  XML and was not rasterised here (no renderer); its geometry was checked by arithmetic only.
- **Resume checkpoint not pushed.** `.resume/RESUME.md` was filled in locally (the directory is
  gitignored); `scripts/checkpoint.sh push` was denied by the session's permission classifier and no upload
  token exists, so the S3 checkpoint the repository asks for before a run does not exist.
- **Graphs checked.** `docs/index-calculus-scoreboard.html`, `docs/ic/leaderboard.json` and
  `benchmarks/` are untouched: this is a stage diagnostic with no whole-pipeline `S`, no ratio to a
  boundary and no matched rho, so it adds no point under `AGENTS.md` section 7a.

## Not done, and the single next experiment

- Not done: `line_subgroup` at scale; the n = 19 block. A new n = 19 manifest was frozen natively and
  checked solver-free but **no solver was run on it**: [`fixtures/manifest_n19.json`](fixtures/manifest_n19.json),
  SHA-256 `5cbf0fd14cfd69f852ee7f4673d92aee4b9d03f5ca5d4da82d6cde6ede51469e` (`y^2 + xy = x^3 + 1` over
  `F_(2^19)`, modulus `0x80027`, #E = 4 x 130,873, 17 representatives, 323 admissible x, 6 targets), made with

```sh
phase_paired_fixed_budget freeze --out research/phase_paired_native_20261026/fixtures/manifest_n19.json \
  --n 19 --modulus 0x80027 --s 6 --factor-seed 20260930 --target-seed 20261026 --per-set 2
```

  Running it needs `gate` generalised (its legacy constants are n = 13 specific) and, at the measured
  speed, a conflict budget well beyond this study's; arm A's refutations will not fit.
- **Single next experiment:** hold the harness, targets, budget accounting and verification fixed and
  change only the solver. Export each arm's instance with `sat::to_dimacs_xor` and run it under a
  compiled CryptoMiniSat (an allowed native backend) with the same conflict and CPU accounting. If arm A's
  refutations fall to the legacy 8 to 52 s, the headline flip here is the solver's refutation cost, and
  the legacy direction (B costlier than A) is a property of the arm A/B circuits that survives outside
  Python; if they stay slow, it is the encoding.

## Reproduction

```sh
d=research/phase_paired_native_20261026
( cd $d && sha256sum --check SHA256SUMS )
cargo build --release --locked --example phase_paired_fixed_budget --example phase_paired_replay
cargo test  --release --locked --example phase_paired_fixed_budget --example phase_paired_replay   # 13 tests
target/release/examples/phase_paired_fixed_budget gate --sealed $d/fixtures/witness_sealed.json   # solver-free
target/release/examples/phase_paired_replay $d/results/receipts_n13.jsonl /tmp/replay.json --sealed $d/fixtures/witness_sealed.json
cmp /tmp/replay.json $d/results/replay_n13.json
bash $d/determinism_check.sh                                                                         # re-runs one attempt
```

The workflow `.github/workflows/phase-paired-native.yml` does the same (path filters limited to this
study's own files; concurrency group with cancel-in-progress).

## Files

| file | role |
|---|---|
| `PROTOCOL.md` | hypotheses, decision rule, budget, calibration disclosure, addendum 1 |
| `fixtures/manifest.json`, `fixtures/witness_sealed.json` | the frozen legacy manifest (8cb7f798...) and the sealed witnesses (read only by the checkers) |
| `fixtures/manifest_n19.json` | new native n = 19 manifest, frozen, not run |
| `results/receipts_n13.jsonl` | one header, 72 attempt receipts, 4 superseded `skipped` receipts, `resume` and `end` lines |
| `results/replay_n13.json`, `tables_n13.md`, `chart_n13.svg` | independent replay output (no host values), tables, chart |
| `results/gate_rep1.txt`, `run_rep1.log`, `run_rep23.log`, `run_last4.log` | gate output and run logs (statuses only) |
| `calibration/` | raw non-frozen calibration receipts |
| `determinism_check.sh`, `SHA256SUMS` | determinism check, hashes of the above |
