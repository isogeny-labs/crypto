# Protocol addendum 2: the same instances under a pinned CryptoMiniSat

Registered 2026-10-26. Written and committed **before any measurement on the frozen manifest with the external
solver** (the pilot runs it discloses used non-frozen targets and tiny instances only). `PROTOCOL.md`,
addendum 1, `RESULTS.md`, the receipts and the replay output of the native study are unchanged and are not
edited by this follow-up. Stage diagnostic on a toy field: no `S`, no ratio to rho, no ECDLP speedup at any
size is claimed or sought.

## Question

`RESULTS.md` found that arm A's cost in the native study is 96 to 98 per cent proofs of non-decomposability
(24 UNSAT proofs of 38 to 234 s each) and that this is what moves the headline B/A CPU ratio from the legacy
direction (1.61, CryptoMiniSat through Python) to a mixed result (0.63 to 1.07). It named one experiment:
hold the harness, targets, budget accounting and verification fixed, change only the solver, and ask whether
arm A's refutations stay slow.

> Are arm A's slow refutations in the crate's `sat::Solver` the **solver's refutation cost**, or do the
> exported instances themselves (the arm A and arm B circuits as this study's encoder writes them) stay slow
> under a different solver?

## Claim classes

Each sentence of `RESULTS_CMS.md` falls in one class (`controlled-experiment` skill, section 1).

| class | sentences | what establishes it |
|---|---|---|
| exact computation | every relation found is a signed sum of four points equal to the target; the instance files hash to the recorded values; the exhaustive decomposability of each target in each arm | the group check in the producer and, independently, in the replay; the exhaustive pair matching |
| comparative / statistical | "CryptoMiniSat refutes arm A faster or slower than the crate solver"; "B costs more or less than A under CryptoMiniSat" | this protocol: same instruments, A/A, positive control, the decision rule below |
| heuristic | none is relied on | |
| out of scope | anything about real curves, larger `n`, other solvers or solver versions | "not established" |

## What changes and what is held fixed (variable ledger)

| factor | treatment |
|---|---|
| instances: curve, manifest (`8cb7f798...1231`), 12 targets, factor space, formulation `subgroup_xor`, arms A and B | **fixed**: the unchanged `build_model` of `examples/phase_paired_fixed_budget.rs`, exported by `sat::to_dimacs_xor` |
| group check, rank tracker, exhaustive check, receipt fields | **fixed**: the same functions; the replay is the independent replay |
| conflict budget per attempt | **fixed**: 3,000,000, the native primary budget (see "Budget" for the unit) |
| order and repetitions | **fixed**: 12 targets in manifest order, ABBA pairs `(i + rep - 1)` even gives `A,B`, 3 repetitions, 72 attempts planned |
| **solver** | **the variable**: native `sat::Solver` becomes the pinned CryptoMiniSat 5.16.0 binary, one thread |
| nuisance seed | **randomised, crossed with repetition**: `--random` (below); the native study's saved-phase scrambling cannot be passed to CryptoMiniSat |
| branch priority | native: core unknowns named to the solver; **not available** to CryptoMiniSat, which chooses its own branching. This is part of the solver contrast, not a controlled factor |
| host | measured: same Xeon 2.10 GHz class, 4 shared cores, Linux 6.18.44; load average recorded per attempt; `nice -n 10`; one process at a time |
| analyst | arm labels are not masked (a status table has no judgement to bias); the decision rule below is fixed in advance |

## The pinned instrument

Recorded in `ENVIRONMENT_CMS.txt` and in every receipt header; the producer refuses any other binary.

| item | value |
|---|---|
| CryptoMiniSat tag | `release/v5.16.0` (the legacy study's `pycryptosat 5.16.0` core version; `release/v5.17.0` also exists and was not used) |
| CryptoMiniSat commit | `2bf89cfb3e126e559d472388486327813f735f7e` (lightweight tag; `git rev-parse` of the checkout) |
| CaDiCaL (fetched by its CMake) | `https://github.com/meelgroup/cadical` at `818c9562f114b315a9246ced943b66b60b38e8fb`, the commit `master` pointed to on 2026-10-26 |
| CaDiBaCk (fetched by its CMake) | `https://github.com/meelgroup/cadiback` at `445ddc2f2b5e54421af64e0652c2eb8444175322`, the commit `main` pointed to on 2026-10-26 |
| system package | `libgmp-dev` `2:6.3.0+dfsg-2ubuntu6.1` (with `libgmpxx4ldbl` of the same version); `apt-get update` was needed first because the package lists were empty |
| compiler, CMake | g++ 13.3.0 (Ubuntu 13.3.0-6ubuntu2~24.04.1), CMake 3.28.3 |
| configure | `cmake -S /home/user/cms-src -B /home/user/cms-build -DCMAKE_BUILD_TYPE=Release -DENABLE_TESTING=OFF -DBUILD_SHARED_LIBS=OFF -DFETCHCONTENT_FULLY_DISCONNECTED=ON -DFETCHCONTENT_SOURCE_DIR_CADICAL=/home/user/cms-src-deps/cadical -DFETCHCONTENT_SOURCE_DIR_CADIBACK=/home/user/cms-src-deps/cadiback` |
| build | `nice -n 10 cmake --build . --target cryptominisat5-bin -j2`; Release means `-O2`, `NDEBUG`, assertions off; statically linked; no Python, no tests, no documentation, no statistics, no BreakID, no MPI; zlib on (system) |
| binary | `/home/user/cms-bin/cryptominisat5`, 6,078,344 bytes, SHA-256 `8e1e539976b1fa8bff2b1e30df5c200c9a03ee343c634eadc0152e1b6cecd8f7`; its `--version` banner names all three commits |
| where the code lives | outside every repository checkout (`/home/user/cms-src`, `/home/user/cms-src-deps`, `/home/user/cms-bin`); **nothing from CryptoMiniSat, CaDiCaL, CaDiBaCk or the binary is committed**; the build tree was deleted |

The user authorized the clone, the build outside the repositories and the run as a child process, and then,
when the build turned out to need them, the two dependency clones at pinned commits and `apt-get install
libgmp-dev`. The decision rule of this addendum was written knowing none of the new solver's timings on the
frozen targets.

Native Gauss-Jordan elimination is on (the default; the binary prints `[g n]` matrix lines when it uses it, and
`--autodisablegauss` stays at its default `true`). Every other option is the default except
`--threads 1`, `--maxconfl`, `--maxtime`, `--random` and `--verb 1`.

## Instances and their faithfulness

`build_model(...)`, then `Model::load(Priority::Core)`, then `sat::to_dimacs_xor`: a CNF part (Tseitin AND gates,
the one-hot clauses, distinctness clauses) and one `x` line per native XOR row. Sizes: arm A 2,058 variables,
5,583 clauses, 200 XOR rows (about 112 KB); arm B 3,432, 7,927, 846 (about 175 KB). The files are regenerated
by the producer for each attempt, hashed (`dimacs_sha256` in the receipt) and deleted; they are not committed.

Faithfulness is checked three ways before the main run (controls below): the exported text re-exports to itself
after `parse_dimacs_xor`; the native solver, given the original model and given the round trip (same branch
priority, same phase scramble, same budget), takes **identical** searches (status, conflicts, decisions,
propagations, restarts); and every model CryptoMiniSat returns must satisfy the exported file under the
producer's own parser of the `x` convention (bare literals mean parity 1, each negated literal flips it).

## Seeds: the saved-phase substitute

The native study scrambled the saved decision phases with `phase_seed(rep)`. CryptoMiniSat has no such option.
Its `--random <seed>` seeds the generator used by rephasing and random decisions. Repetition `rep` therefore
passes `--random (phase_seed(rep) & 0x7fffffff)`: **27686829, 688479205, 812055403** for repetitions 1 to 3,
the same value for both arms. The pilot saw it matter (four seeds on one calibration instance gave 235,123,
278,401, 74,042 and 96,951 conflicts), and the same seed twice gave identical conflicts and decisions. The
repetition-to-repetition spread under this nuisance is the A/A noise floor below.

## Budget and clocks

- **Primary, deterministic:** `--maxconfl` = 3,000,000 conflicts per attempt minus conflicts already spent, the
  native study's budget. The count is **CryptoMiniSat's own**, read from its `c conflicts` statistics line
  (the last such line; labels match exactly). A conflict of one solver is not a conflict of the other, so the
  budget is identical as a number and not as an amount of search; the CPU clock below is the common unit.
  CryptoMiniSat tests the limit at restart granularity (5,002 conflicts under a limit of 5,000 in the pilot):
  the replay allows an overshoot of one part in a thousand, and `budget` status requires the limit reached.
- **Backstop, recorded as an event:** `--maxtime 120`, CPU seconds, which is the **legacy study's per-attempt
  budget** (its `time_limit` was 120 CPU s). An attempt ending that way with fewer than 3,000,000 conflicts has
  status `deadline` and is unresolved, never read as a failure to decompose, exactly as in the native study. The
  native study's 400 s was a wall-clock backstop; it is not the figure matched here. A second guard kills a
  child whose wall time passes `1.5 x 120 + 30` s and records the attempt as an instrument `error`, which stops
  the run.
- **CPU accounting:** the child's user plus system CPU from `wait4`'s resource usage, **plus** the producer's
  own `CLOCK_PROCESS_CPUTIME_ID` delta for model build, export, output parsing, the group check and receipt
  assembly. That is the native study's "everything inside an attempt, build and verification included"; the
  parent is idle while the child runs. Wall time is recorded beside it.
- **One thread**, deterministic given the seed, fresh process per attempt, attempts strictly sequential.
- **No proof is produced.** CryptoMiniSat 5.16.0 can write an XLRUP proof through a second positional file
  argument, but no checker (`cake_xlrup`) is installed here and the option changes timing, so it is not used.
  **An UNSAT answer is the solver's word, unverified by proof**, and the results say so. What the replay checks
  is that each UNSAT agrees with the independent exhaustive pair matching (no decomposition exists in that arm's
  pool): "consistent with exhaustive ground truth", not "verified". A SAT answer is different: its model is
  decoded to four abscissae and passes the same group check as the native relations, in the producer and again
  in the replay.
- A model that fails the group check (a rejected tuple) is blocked by a clause and the child re-run within the
  remaining budget, as in the native attempts. The pilot and native study saw none.

## Controls (all before the main run; a failure stops the study)

1. **Tiny known instances** (`cms-selftest`): 100 random CNF + XOR instances (10 to 14 variables, both answers)
   whose truth is found by brute force; the pinned binary, the native solver on the exported text and the
   brute-force truth must agree, and every CryptoMiniSat model must satisfy the file; four hand-written files
   check the parity convention (parity-one XOR, odd XOR cycle, a negation that flips the right-hand side, a
   pigeonhole). This is the check of the one thing that could silently corrupt the whole study: a difference in
   how CryptoMiniSat and the crate read an `x` line.
2. **Export fidelity on frozen targets** (`cms-check`): `b3` and `a0`, both arms, as described above, 20,000
   conflicts.
3. **Positive control:** `b3` in arm A and in arm B (known decomposable by the exhaustive check) run through the
   full child-process path must return `verified`, and rerunning the first with the same seed must reproduce
   its conflicts and decisions (determinism).
4. **A/A noise floor:** arm A on the same instances with an independent nuisance draw is exactly what repetition
   2 and 3 are (a different `--random`, the same instance and arm). The reported noise floor is the spread of
   per-repetition medians of the eight arm A refutations. The determinism rerun above shows that the spread
   comes from the seed and nothing else.
5. **Negative controls:** the eight targets `a0 a1 a2 a3 c0 c1 c2 c3` are decomposable in no arm A pool by the
   exhaustive check, so every `verified` there would be a defect and every `unsat` is checked against that
   ground truth. There is no population-level null here: the study makes no claim about a population of curves.

## Order, cap and stages

Order: controls 1 to 3, then `cms-run` once with `--reps 3`. Total solver CPU for the whole follow-up is
**capped at 3,600 s** (60 minutes, one core, `nice`). The cap includes the pilots (**180 s**, the ledger in
`calibration_cms/README.md`, entered as `--cpu-prior 180`) and the controls (their file is passed with
`--controls`), so the main run has what is left, about 3,380 s. At the native study's speed (about 1,200 s per
arm per repetition) one repetition would not fit; at the legacy speed (about 600 s per repetition) three
would. The run is therefore staged by a rule the producer executes and the replay re-derives from the receipts:

- repetition 1 always starts, in full (both arms, 24 attempts);
- before repetition `k` > 1, let `R` be the remaining allowance (cap minus everything charged so far),
  `F` the dearest completed full repetition (both arms) and `A` the dearest completed arm A (12 attempts) of an
  earlier repetition; with a guard of 1.1: if `R >= 1.1 F` repetition `k` runs in full; else if `R >= 1.1 A` it
  runs **arm A only** (this keeps the A/A on the primary quantity when arm B is too dear to repeat); else
  it does not run. With no completed repetition to extrapolate from it does not run;
- at the start of each pair a hard guard: if spent + (attempts in the pair) x (1.05 x 120 + 3 = 129 s) would
  exceed the cap, the run stops there. Every attempt not run is written as a `skipped` receipt with its
  reason, never dropped, and the results say what was cut.

If the cap is hit, partial results are reported as partial and the cap is not extended.

## Decision rule (fixed now)

Anchors, per arm A refutation, in CPU seconds on this host class: **legacy** (CryptoMiniSat 5.16.0 through
Python, one trial, `subgroup_xor`): 8.3, 15.0, 20.6, 23.4, 24.2, 28.0, 29.3, 44.0, median 23.8 (the "8 to 52 s"
quoted in `RESULTS.md` also covers the legacy `line_subgroup` formulation, whose largest is 51.9);
**native** (this crate's solver, 24 refutations): 38.2 to 233.7, median 149.9, per-repetition medians 113.1,
149.9 and 172.0.

For each repetition in which all eight arm A attempts on `a0..a3, c0..c3` were run ("complete"), let `t` be the
**median** of their eight total CPU seconds (any status; an attempt that did not end `unsat` is charged what it
used, a lower bound), and `u` the number of the eight that did not end `unsat`. Class the repetition:

- **S (fast)** if `t <= 45` s and `u = 0`;
- **E (slow)** if `t >= 80` s (whatever `u`);
- **M (between)** otherwise, including `t <= 45` with `u >= 1`.

The thresholds are placed by the anchors: 45 s is above all eight legacy refutations (the largest is 44.0 s) and above only 1 of the 24
native ones (38.2 s); 80 s is above only 4 of the 24 native refutations (79.3 s and below) and above every
legacy one. (Applied to the anchors, the legacy median 23.8 is class S and each native repetition median is class E.)

Verdict from the complete repetitions:

| complete repetitions | verdict |
|---|---|
| all class S, two or more | **SOLVER**: the refutation cost is the crate solver's; the instances are easy for a solver with preprocessing |
| all class E, two or more | **ENCODING**: the exported instances stay slow under CryptoMiniSat; the slowness is not specific to the crate solver |
| both S and E occur | **INCONCLUSIVE**: repetitions fall on both sides of the band, noise dominates |
| every other pattern with two or more (all M, or M with S or E) | **INTERMEDIATE**: neither anchor; the design cannot separate solver from encoding |
| exactly one | provisional: **SOLVER** only if `t <= 30` s and class S, **ENCODING** only if `t >= 120` s, otherwise **INCONCLUSIVE** (no noise floor) |
| none | no verdict |

**Budget-exhausted and deadline attempts** are unknown, never "slow because hard" and never "SAT": they are
charged their CPU as a lower bound, they make a repetition ineligible for class S, and a repetition with `t >=
80` s stays class E with them.

**What each verdict means.** SOLVER: with the same instances, a solver with CryptoMiniSat's preprocessing and
Gauss-Jordan reasoning refutes arm A at about the legacy speed, so the native study's slow refutations are the
crate solver's cost, and the legacy direction of the arm B / arm A contrast is a property of the circuits that
survives outside Python only if the CryptoMiniSat B/A CPU ratio per new relation is above 1 (reported, below).
ENCODING: the exported instances are as slow under CryptoMiniSat as under the crate solver; since the legacy
study's CryptoMiniSat refuted the legacy Python encoding quickly, the difference lies in the instance as this
encoder writes it (the shared linear-form variables and the rest of the Rust re-implementation) and in
nothing this study can split further. INTERMEDIATE and INCONCLUSIVE say that the design does not decide.

**Secondary, exploratory, outside the rule:** the B/A ratio of conflicts and of CPU per newly independent
relation under CryptoMiniSat for each repetition (arm A only repetitions have none), next to the native and
legacy figures (native 0.63 to 1.07, legacy 1.61); the like-for-like `b0..b3` ratios; decisions and
propagations (CryptoMiniSat prints propagations rounded to K, M or G).

## What this cannot show

One solver version and one build; one toy field (`n = 13`), five representatives, 12 targets; three
repetitions at most and eight arm A refutations each; CryptoMiniSat's UNSAT answers unverified by proof;
conflict counts that are not comparable between solvers; a contrast of two solvers that also differ in
branching, preprocessing, restarts and Gauss-Jordan handling, none of which is isolated here; a legacy anchor
from one Python trial on another encoder. No claim about real curves, larger fields, ECDLP cost, `S` or a
ratio to rho follows from any outcome.

## Stop conditions

The study stops, and the defect is reported, if: any control fails; the binary's hash or banner differs; an
attempt ends in `error` (unexpected exit code, status line, missing model or conflict count, a killed child,
a model that violates the exported file); a `verified` model fails the group check in the replay; an UNSAT or
SAT contradicts the exhaustive check; the cap is reached (then partial, as above).

## Pilot disclosure

Before this addendum the producer's new subcommands were exercised on the non-frozen calibration targets and
tiny instances (`calibration_cms/`, 180 s ledger): the output format and exit codes (SAT 10, UNSAT 20,
INDETERMINATE 15), the seed effect, the speed (calibration `a0` arm A: UNSAT in 666,390 conflicts and 25.4 s;
calibration `b0` arm A: SAT at 36,625 to 278,401 conflicts depending on the seed, 0.9 to 9.7 s; calibration
`b0` arm B: SAT at 433,837 conflicts and 20.9 s), the staged-run and cap paths, the replay. Nothing from them
enters a result table. They did suggest that arm A refutations will land between the legacy and native
anchors, so the thresholds above were chosen from the anchors, not from those numbers.

Pre-flight estimate (written before the run): arm A at 8 refutations of roughly 15 to 45 s plus four short
relations is 120 to 360 s per repetition; arm B at 12 searches of 2 to 120 s is 100 to 900 s per repetition;
three repetitions then take 700 to 3,800 s against the 3,380 s available.

## Commands (producer and replay are the examples named below; Rust only)

```sh
d=research/phase_paired_native_20261026
b=/home/user/cargo-target/release/examples/phase_paired_fixed_budget
cms=/home/user/cms-bin/cryptominisat5
$b cms-selftest --cms $cms --scratch $scr --out $d/results/controls_cms.jsonl --count 100
$b cms-check    --cms $cms --scratch $scr --out $d/results/controls_cms.jsonl \
                --fidelity b3:A,b3:B,a0:A,a0:B --positive b3:A,b3:B
$b cms-run      --cms $cms --scratch $scr --out $d/results/receipts_cms_n13.jsonl \
                --controls $d/results/controls_cms.jsonl --reps 3 --conflict-budget 3000000 \
                --maxtime 120 --cpu-cap 3600 --cpu-prior 180
phase_paired_replay $d/results/receipts_cms_n13.jsonl $d/results/replay_cms_n13.json \
    --sealed $d/fixtures/witness_sealed.json --controls $d/results/controls_cms.jsonl \
    --native $d/results/receipts_n13.jsonl --tables $d/results/tables_cms_n13.md --svg $d/results/chart_cms_n13.svg
```

The producer's source hash is in every receipt header; the commit that adds the code follows this one, and the
main run starts only after both are committed.
