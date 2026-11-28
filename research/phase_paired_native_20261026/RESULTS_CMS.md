# The native instances under a pinned CryptoMiniSat: is arm A's refutation cost the solver's or the encoding's?

**Scope.** A stage diagnostic on a toy field (`y^2 + xy = x^3 + 1` over `F_(2^13)`, the 12 frozen targets, 5 orbit
representatives), one solver build, three repetitions. It changes only the solver of the native study
([`RESULTS.md`](RESULTS.md)): the same instances, exported with `sat::to_dimacs_xor`, are solved by a
pinned CryptoMiniSat run as a child process, with the same conflict budget, the same CPU accounting and the same
group check on every relation. It fixes no `S`, no ratio to rho and no ECDLP speedup at any size, and it says nothing
about real curves.

Protocol: [`PROTOCOL_ADDENDUM_2.md`](PROTOCOL_ADDENDUM_2.md), committed (6589481ab3) before the producer code
(439cdaf3be) and before any run on the frozen manifest. Pins and environment: [`ENVIRONMENT_CMS.txt`](ENVIRONMENT_CMS.txt).
Independent replay: `examples/phase_paired_replay.rs` (new mode). Producer: `examples/phase_paired_fixed_budget.rs`
(`cms-selftest`, `cms-check`, `cms-run`). Nothing from RESULTS.md, PROTOCOL.md, addendum 1, the native receipts or the
native replay output was edited; the native replay output still reproduces byte for byte.

## Verdict, in order of how firmly it is established

1. **Every relation and every control checks out.** 72 attempts, 24 UNSAT (all arm A) and 48 verified (12 per
   repetition and arm B, 4 per repetition in arm A), 0 rejected tuples, 0 unfinished attempts (neither the
   3,000,000-conflict budget nor the 120 s CPU backstop ever ended an attempt), 0 instrument errors. The independent
   replay re-verified all 48 relations on the curve, recomputed rank (arm A 4 of 5, arm B 5 of 5 in every
   repetition, as in the native study), found every UNSAT consistent with the exhaustive pair matching, and
   re-derived the stage rule and the CPU cap from the receipts: **0 defects over 72 receipts**. Before the run: 104
   self-tests (100 random CNF + XOR instances against brute force and the native solver, 4 hand-written parity
   files) all agreed; the exported DIMACS of `b3` and `a0` in both arms re-exports to itself and makes the native
   solver take an identical search; the positive control (`b3`, both arms) returned verified models and a same-seed
   rerun reproduced its conflicts and decisions exactly.
2. **Preregistered verdict: SOLVER.** In each of the three repetitions the median CPU of the eight arm A
   refutations is **36.1, 20.8 and 24.9 s**, all eight UNSAT every time: class S (at or below 45 s) in all three.
   The native study's repetition medians were 113.1, 149.9 and 172.0 s; the legacy trial's median was 23.8 s. So on
   the identical exported instances, a solver with CryptoMiniSat's preprocessing and XOR handling refutes arm A
   **3.1 to 7.2 times faster** than the crate's `sat::Solver` (pooled median 23.2 s against 149.9 s, 6.5 times) and at
   the speed the legacy Python harness saw. The slow refutations in the native study are the crate solver's cost, not
   a property of the exported instances.
3. **The legacy direction of the arm B / arm A contrast returns.** Under CryptoMiniSat arm B costs **1.18, 1.49 and 1.49
   times arm A's CPU** per newly independent relation (mean 1.38; legacy 1.61 on one trial; native 0.63 to 1.07).
   B/A conflicts per new relation are 0.94, 1.18 and 1.17 (native 0.28 to 0.41). Exploratory, outside the
   preregistered rule: applied to this solver's data the native study's H1 rule reads "mixed, not supported" (B below A on
   both clocks in 0 of 3 repetitions, at or above on both in 2). The native study's "arm B needs a third of arm A's conflicts"
   does not survive a change of solver.
4. **What is not established is which part of CryptoMiniSat does it.** Arm A's refutations take about as many
   conflicts under both solvers (CryptoMiniSat 445 thousand to 1.34 million per refutation, native 0.61 to 1.81 million;
   conflict counts are not comparable between solvers) but cost 35.4 microseconds per conflict against the native
   107.9 for arm A, and 44.6 against 271.4 for arm B (pooled over the three repetitions). This study ran CryptoMiniSat with its defaults; it did not switch
   preprocessing, XOR recovery or Gauss-Jordan elimination off one at a time, so it does not say which one the crate's
   solver lacks, or whether it is the speed of each step and not a feature.

![Arm A refutation CPU seconds per target](results/chart_cms_n13.svg)

## What ran

- Instrument: CryptoMiniSat 5.16.0, tag `release/v5.16.0`, commit `2bf89cfb3e126e559d472388486327813f735f7e`, built Release
  with CaDiCaL `818c9562f114b315a9246ced943b66b60b38e8fb` and CaDiBaCk `445ddc2f2b5e54421af64e0652c2eb8444175322` (pinned
  commits of their floating `master` and `main`), `libgmp-dev 2:6.3.0+dfsg-2ubuntu6.1`, g++ 13.3.0, CMake 3.28.3; statically
  linked binary, SHA-256 `8e1e539976b1fa8bff2b1e30df5c200c9a03ee343c634eadc0152e1b6cecd8f7`, hash-checked by the producer
  and by the replay. Everything lives outside the repositories and nothing of it is committed.
- Options: `--threads 1 --maxconfl <budget left> --maxtime <CPU left> --random <seed> --verb 1`, all else default (native
  Gauss-Jordan on). Budget 3,000,000 conflicts (CryptoMiniSat's own count), CPU backstop `--maxtime 120` (the legacy
  study's per-attempt budget). Seeds `--random 27686829, 688479205, 812055403` for repetitions 1 to 3, shared by the arms;
  this replaces the native study's saved-phase scrambling, which CryptoMiniSat cannot take.
- CPU: child user + system CPU from `wait4` (1,946.9 s over the 72 attempts) plus the producer's own build, export,
  parse and verification CPU (7.4 s); wall time within 1.4 per cent of CPU time; load average at most 1.08; child
  resident set at most 104 MB. `nice -n 10`, one attempt at a time, producer source SHA-256
  `1fe3aa1b9d55b617629e55b3ddad61130672af9070c177d5c8a588229d30cd8a` (in every receipt).
- **Budget used: 2,196 s (36.6 CPU minutes) of the 3,600 s cap**: attempts 1,954.4 s, controls 62.0 s, pilots and unit-test
  runs 180 s (a conservative ledger, [`calibration_cms/README.md`](calibration_cms/README.md)). The stage rule ran all three
  repetitions in full (remaining allowance 3,358, 2,627 and 2,082 s against 1.1 times the dearest completed repetition,
  804 s); the cap stop never fired.
- Disk: the build tree (23 MB) was deleted; `/home/user/cms-src` (7 MB), `/home/user/cms-src-deps` (6 MB) and the 6 MB binary
  remain outside the repository; free space stayed above 2.7 GB.

## Results (every attempt charged; full tables in [`results/tables_cms_n13.md`](results/tables_cms_n13.md))

Arm A refutations, CPU seconds per attempt (all UNSAT; native and CryptoMiniSat on the same exported instances):

| target | legacy (CMS via Python) | native rep 1 | rep 2 | rep 3 | CMS rep 1 | rep 2 | rep 3 |
|---|---|---|---|---|---|---|---|
| a0 | 8.3 | 122.5 | 202.3 | 197.4 | 48.6 | 26.7 | 27.3 |
| a1 | 23.4 | 98.2 | 175.9 | 215.6 | 19.9 | 16.0 | 29.8 |
| a2 | 20.6 | 176.2 | 149.8 | 150.4 | 50.5 | 22.6 | 21.8 |
| a3 | 15.0 | 80.9 | 57.9 | 38.2 | 27.6 | 23.5 | 21.2 |
| c0 | 44.0 | 79.3 | 215.5 | 193.6 | 22.5 | 15.5 | 35.2 |
| c1 | 29.3 | 180.5 | 83.1 | 144.1 | 31.6 | 16.1 | 22.1 |
| c2 | 28.0 | 103.7 | 79.1 | 100.2 | 44.7 | 21.8 | 23.0 |
| c3 | 24.2 | 228.4 | 150.0 | 233.7 | 40.7 | 19.9 | 26.7 |
| **median** | **23.8** | **113.1** | **149.9** | **172.0** | **36.1** | **20.8** | **24.8** |
| class under the rule | S | E | E | E | S | S | S |

All 24 CryptoMiniSat refutations: 15.5 to 50.5 s, median 23.2 s, mean 27.3 s. The largest (50.5 s) is above the largest
legacy `subgroup_xor` value (44.0 s) and inside the "8 to 52 s" envelope of both legacy formulations.

Paired comparison, per repetition (arm B over arm A, per newly independent verified relation, every attempt charged):

| rep | CMS arm A CPU s | CMS arm B CPU s | CMS conflicts B/A | **CMS CPU B/A** | native conflicts B/A | native CPU B/A |
|---|---|---|---|---|---|---|
| 1 | 295.6 | 435.0 | 0.938 | **1.177** | 0.306 | 0.800 |
| 2 | 190.6 | 354.6 | 1.182 | **1.489** | 0.406 | 1.072 (lower bound) |
| 3 | 237.3 | 441.3 | 1.167 | **1.488** | 0.275 | 0.632 |
| mean | | | 1.096 | 1.384 | 0.329 | 0.835 |

Legacy (one trial): CPU B/A 1.61, arm A 204.9 s and arm B 412.4 s for 12 attempts. CryptoMiniSat here: arm A 190.6 to
295.6 s, arm B 354.6 to 441.3 s; native: arm A 1,091 to 1,319 s, arm B 1,042 to 1,530 s. Like for like on `b0..b3`
(sums over the four targets) arm B costs 14.9, 4.0 and 4.0 times arm A's CPU under CryptoMiniSat (native 12.2, 10.9 and
6.5; legacy 2.2 to 98.7 per pair) and 9.5, 3.0 and 3.0 times its conflicts (native 2.2, 2.1 and 1.3).

Noise (A/A). Repetitions differ only by `--random`. The median of the eight refutations moves by a factor of 1.74
(36.1 to 20.8 s) and a single target by up to 2.3 (`a2`: 50.5, 22.6, 21.8 s), the same heavy tail the native study saw.
A same-seed rerun is identical (124,479 conflicts and the same decisions on `b3` arm A). Repetition 1 is the slow one;
its median sits at 80 per cent of the 45 s cut, so the verdict has a margin of 1.25 on the weakest repetition and
2.2 on the E side. It is robust to the noise seen here, not to an arbitrarily bad fourth seed.

## What each number is worth

The verdict compares one solver's cost on instances exported from this study's encoder with two other things: the same
crate solver on the same instances (the native receipts) and the legacy trial (CryptoMiniSat on the legacy Python
encoding). That CryptoMiniSat's median here (23.2 s over 24, 20.8 to 36.1 per repetition) matches the legacy median
(23.8 s over 8) says that, for CryptoMiniSat, the native encoder's shared linear-form variables and the rest of the Rust
re-implementation cost nothing visible; it is two separate measurements, not a controlled equivalence. The design cannot
split the "circuits" from the "encoder" any further, and the ENCODING outcome it could have returned (slow under both
solvers) did not occur.

## Not measured, and limits

- One solver version and one build (5.16.0, the legacy version, default options). No ablation of its features; no other
  solver; no recent crate-solver change tested. The crate solver was **not** re-run: the native numbers are those of
  `results/receipts_n13.jsonl` (same host class, a different session and load).
- CryptoMiniSat's **UNSAT answers carry no proof**: 5.16.0 can write an XLRUP proof through a second file argument, but no
  checker is installed here and it would change timing, so none was written. Each UNSAT agrees with the independent exhaustive
  pair matching (consistent with ground truth), which is evidence, not verification. All 48 SAT answers are verified on
  the curve by the producer and again by the replay.
- Seeds are not the native study's phase scrambling and branch priority on the core unknowns does not exist for
  CryptoMiniSat. The two solvers therefore differ in branching and seeding as well as in preprocessing and XOR handling.
  The B/A conflicts finding (about a third in the native study, about 1.1 under CryptoMiniSat) is consistent with that
  branch priority helping the native arm B searches, but nothing here tests it.
- Conflict counts are not comparable between solvers; CPU is the common unit. CPU is one shared 4-core VM, no isolation
  level beyond the recorded load; the same instance varies by up to 2.3 times across seeds.
- `n = 13` only, five representatives, 12 targets, `subgroup_xor` only, three repetitions, eight arm A refutations each.
  `line_subgroup` and the frozen `n = 19` manifest were not run. A stage ratio here says nothing about end-to-end cost
  at larger fields, real curves, `S` or the ratio to rho.
- The legacy anchor is one Python trial on a different encoder; its seed spread is unknown.
- No `experiment.protocol/v1` JSON and no second-reader `review-run` record: the tools for both are Python (this repository
  forbids it for harness code) and the native study took the same route; the replay is the second reader of the receipts, not
  of the design. No PDF: no non-Python renderer is installed; the SVG is well formed (checked with `xmllint`), not rasterised.
- `docs/index-calculus-scoreboard.html`, `docs/ic/leaderboard.json` and `benchmarks/` are untouched: no whole-pipeline `S`, no
  boundary and no matched rho, so no leaderboard point (`AGENTS.md` section 7a).

## Deviations and events, stated plainly

- **Authorization came in two steps**, both relayed by the coordinating agent from the user: the clone, build outside the
  repositories and child-process run of CryptoMiniSat at one pinned tag; then, after the build was found to need them, the
  clones of CaDiCaL and CaDiBaCk at commits recorded here and `apt-get install libgmp-dev`. `apt-get update` had to run first (the
  package lists were empty). Nothing else was installed or fetched.
- The PR is **not stacked**: the brief planned a stack on #2623, but #2623 had already merged (17:41 UTC) and its branch
  was deleted, so the branch is merged with `origin/main` and the PR targets `main`.
- The first pilot assumed exit code 0 for a stopped search; CryptoMiniSat uses 15 (`INDETERMINATE`). The producer failed
  closed (status `error`), and the mapping was fixed before the addendum (`calibration_cms/pilot_run1.jsonl`).
- **`origin/main` does not compile** after the merge: `src/cryptanalysis/binary_semaev_chain_sat.rs` has an unclosed delimiter
  (a bad merge, commit 4f1fa86ac1; `verify_external_assignment` lost its closing brace). This follow-up neither causes nor
  touches it. Everything in this directory was built and tested before the merge, on the tree of the native branch head
  (c22e9a4390), whose library is identical; the merged tree could not be built here.
- Conductor is not installed in this environment (the hooks warn); no task was attached. The S3 resume checkpoint was not
  pushed (the push was denied by the classifier in the previous study and was not retried); `.resume/RESUME.md` was filled in locally.

## Reproduction

```sh
d=research/phase_paired_native_20261026
( cd $d && sha256sum --check SHA256SUMS && sha256sum --check SHA256SUMS_CMS )
cargo build --release --locked --example phase_paired_fixed_budget --example phase_paired_replay
cargo test  --release --locked --example phase_paired_fixed_budget --example phase_paired_replay   # 25 tests
phase_paired_replay $d/results/receipts_cms_n13.jsonl /tmp/replay_cms.json --sealed $d/fixtures/witness_sealed.json \
    --controls $d/results/controls_cms.jsonl --native $d/results/receipts_n13.jsonl \
    --tables /tmp/tables_cms.md --svg /tmp/chart_cms.svg
cmp /tmp/replay_cms.json $d/results/replay_cms_n13.json
```

The replay needs neither the binary nor the sources of CryptoMiniSat. Re-running the measurement needs the pinned binary
(hash in the producer; build recipe in `ENVIRONMENT_CMS.txt`) and `cms-selftest`, `cms-check`, `cms-run` as in the addendum.
With `PHASE_PAIRED_CMS` set to the binary, one unit test also runs it against brute force.

## Files

| file | role |
|---|---|
| `PROTOCOL_ADDENDUM_2.md`, `ENVIRONMENT_CMS.txt` | design, decision rule, cap, pins; the environment capture |
| `calibration_cms/` | pilot receipts on non-frozen targets and tiny instances, with their CPU ledger |
| `results/controls_cms.jsonl` | header, 104 self-tests, 4 fidelity checks, 3 positive-control records |
| `results/receipts_cms_n13.jsonl`, `run_cms.log` | header, 3 stage records, 72 attempt receipts, `end`; statuses only in the log |
| `results/replay_cms_n13.json`, `tables_cms_n13.md`, `chart_cms_n13.svg` | independent replay output (no host values), tables, strip plot |
| `SHA256SUMS_CMS` | hashes of the addendum, the environment capture, the pilot receipts and the results files |
