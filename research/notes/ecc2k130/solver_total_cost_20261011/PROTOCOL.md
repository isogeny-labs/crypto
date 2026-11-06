# Total verified-relation cost of FES / SAT / F4 / F5 solvers (Lead 4) -- protocol

Directory tag `20261011` is the assignment date. This protocol was committed on
2026-10-26, before any frozen measurement. Base commit `bbd330a9d` (origin/main as
of 2026-10-11); the local clone could not be refreshed afterwards (GitHub access
unavailable), so nothing here has been rebased on later `main`.

## Question and scope

The merged m=83 solver matrix (`../m83_solver_matrix_20260927/`) ranked engines by
**solver-stage** wall seconds only (FES 0.000335, CryptoMiniSat 0.0281, F5B 0.1497,
F4-style 0.1710, Z3 0.2121 median s). Those numbers exclude factor-base setup,
equation build, lifting, verification and rank work. The Oct 9 retrospective says
solver throughput must be priced as **independent verified relations per unit of
total computation**. This protocol measures that, per engine, on the frozen inputs.

Null hypothesis H0: engine choice changes total verified-relation cost by about
the same factor as it changes solver-stage time. Alternative H1: the non-solver
stages are large enough that total-cost ratios between engines are much smaller
than solver-stage ratios.

This is a **stage-composition diagnostic** under `AGENTS.md` section 2. It does not
measure a discrete logarithm, a matched rho reference or `S`, and it supports no
ECDLP speedup claim; those fields stay null.

Disclosed rule conflict: `AGENTS.md` ("Implementation language: no Python") bars new
Python for cryptographic research. The frozen workload, the F4-style/F5B prototypes
and the vendored curve arithmetic are legacy Python, and this assignment requires
reusing them byte for byte. The new code is thin Python instrumentation around those
frozen sources. All stage fractions below are therefore properties of pure-Python
setup/build/lift code and must be re-measured on a native pipeline before they
inform any performance claim.

## Workloads (inputs frozen; nothing is chosen by solver outcome)

**W1a, fixed-phase quotient s=2.** Reuse `run.py` of the m83 matrix unchanged:
`GF(2^83)` modulo `z^83+z^45+z^2+z+1`, `E0: y^2+xy=x^3+1`, order
`4*2417851639230796216685689`, seeds `260938` and `260939`, phases `(0,1,2)`, six
Boolean payload variables, Semaev S4. Per seed, inputs 0-1 are the first two proper
planted triples (positive controls, never natural yield) and inputs 2-5 are four
natural targets from `Random(seed*1000+832)`. 12 inputs x 5 engines.

**W1b, normal-subspace payload sweep s=2,3,4.** Reuse `PlainSetup` of
`run_extended.py` and `PROTOCOL_PLAIN_SUBSPACE.md`: for each s in {2,3,4} and each
seed, planted input 0 and natural input 2, target draws
`Random(seed*1000+830+s)`. 3 sizes x 2 seeds x 2 inputs x 5 engines. A cell with no
planted triple is `setup_failed`, charged for the setup work done, and not replaced.
These s=2 plain-subspace inputs are not the W1a quotient inputs.

**Engines (identical equations, identical roots required):** `fes` (64/512/4096
assignments), `sat-cms-xor` (pycryptosat, native XOR, 1 thread, seed not
configurable, see m83 `AMENDMENT.md`), `sat-z3` (Boolean And/Xor, seed 13),
`f4-boolean` (batched Macaulay prototype) and `f5b-boolean` (pinned prototype). The
same code paths as `run.py`/`run_extended.py` are used, only with stage timers added.
Unavailable and not substituted: msolve, WDSat, Sage, the `cryptominisat5` CLI
(`preflight_engines.py` records availability on this host).

Limits per child, as in the m83 protocol: 1 GiB address space, 70 s wall kill,
25 s equation-build deadline, 5 s aggregate solver deadline. `PYTHONHASHSEED=0`.
One benchmark process at a time. **W1 is run 3 times** (R1-R3, same order) because
the host is shared; the committed table uses per-cell medians over repetitions and
shows the between-repetition spread. Receipts of all three repetitions are kept.

**W2, n=13 phase-aware SAT engine comparison (only if CPU remains).** Encoder
`../../../frobenius_quotient_sat_20260930/encode.py` unchanged; field/curve
`n=13`, `s=4`, seed `20260930`, formulation `subgroup_xor` with `--phases` (the
fastest native-XOR variant there), the four **random** (natural) targets that
`run.py --n 13 --s 4 --phases` draws. Their coordinates are compared with the
recorded `meta` in `results/n13_phase_b300.jsonl` before use. Random targets are
used because at n=13 they decompose, so the cost per independent verified natural
relation is finite and measurable there. Engines: CryptoMiniSat with native XOR
(the encoder's own load path) and Z3 fed the *same* clauses and XOR constraints
(clause -> `Or`, XOR -> `Xor` tree). Aggregate solve deadline 120 s per target,
2 GiB address space, 200 s wall kill, rejected tuples blocked and re-solved inside
the deadline. Single trial per cell. This does not repeat Lead 1's paired
phase-on/phase-off comparison; it varies the engine, not the formulation.

## Cost model: stages and charging

Every stage records wall (`perf_counter`) and CPU (`process_time`) seconds, charged
to the engine even when it fails, times out, or finds no root.

| stage | what is timed |
|:--|:--|
| `fb_setup` | factor-base `Setup`, generator, Frobenius eigenvalue, factor-base lookup (orbit table) used for relation vectors |
| `eq_build` | S4 field polynomial and Boolean equations (W2: circuit `build`) |
| `engine_load` | importing the engine library / loading the pinned prototype module |
| `solver_setup` | solver object and constraint loading (FES: none; F4: none; F5B: constructor and polynomial conversion) |
| `solve` | search itself (FES scan, SAT `check`/`solve`, F4 Macaulay reduction, F5B `basis`) |
| `model_extraction` | models to payload bits, algebraic re-check, blocking clauses, F4/F5B root enumeration |
| `lifting` | `core.lift` of each algebraic root to subgroup points whose sum is the target |
| `verification` | independent checks of each lifted witness and of the relation-vector identity |
| `rank_update` | relation-vector construction and one incremental Gaussian elimination step over `Z/r` |

Not charged, reported separately: `input_gen` (planted-triple search and natural
target generation), planted-witness sanity checks, solver statistics collection, and
interpreter start-up (`unattributed` = child wall minus all measured stages).

Solver stage `T_solver = solver_setup + solve + model_extraction`, which is exactly
what the m83 report called the solver stage.

Three accountings from the same receipts:
- **A (primary): core, cold per input.** `T_core = fb_setup + eq_build + T_solver + lifting + verification + rank_update`, as measured in each child.
- **B: setup amortised.** As A with `fb_setup` divided by the number of inputs that share one factor base (`(policy, s, seed)` group).
- **C: fully loaded.** A + `engine_load` + `input_gen`.

## Relations and rank

A verified relation is an algebraic root that lifts to three subgroup points whose
sum equals the target and passes independent checks. Its relation vector is the
coefficient vector over the factor-base orbit classes (representative `i`,
Frobenius `lam^j`, sign), with the identity `sum a_i rep_i = sum witness` re-checked.
Rank is the rank over `Z/r` (`r` the 81-bit prime subgroup order; W2 uses the odd
subgroup order) of the **natural** vectors. **Planted rows are lifted, verified and
rank-updated on a scratch copy of the matrix, so their cost is charged, but they are
not natural yield and do not enter the natural matrix.** Factor-base columns are
Frobenius orbit classes; a representative already inside an earlier column's signed
orbit is skipped, so columns are not redundant.

## Metrics

Primary, per engine: `T_core` (CPU seconds; wall alongside) divided by the number of
**newly independent verified natural relations**. If that number is zero the metric
is reported as *undefined; no natural relation found*, with: cost per natural
attempt, cost per verified planted relation, and a one-sided 95% Clopper-Pearson
upper bound on per-attempt yield (so a lower bound on cost per relation). A separate
**expected-yield calculation** (clearly labelled derived, not measured) gives
`p ~ (candidate signed triples)/r` per fixed-phase attempt from the receipts' own
factor-base sizes.

Secondary: `f_s = T_solver / T_core` per engine, and the Amdahl bound
`g = T_core / (T_core - T_solver)` (the most that making the solve stage free could
save). Also the max/min spread across engines of `T_solver` versus of `T_core`, and
the Kendall rank agreement between the two orderings.

## Predictions registered before measuring (from the m83 stage medians)

1. Natural verified relations: 0 in every W1 cell (8 quotient + 6 subspace natural attempts per engine).
2. `fb_setup` (about 0.1 s per child in m83 receipts) exceeds `eq_build` (about 0.008 s) and, for FES and CryptoMiniSat, exceeds `T_solver`.
3. `f_s` below 5% for FES and below 50% for CryptoMiniSat; Z3 above 50% in accounting A.
4. Engine order by `T_core` equals engine order by `T_solver` (a common additive cost cannot reorder), while the max/min ratio shrinks sharply.
5. In W2, `f_s` exceeds 95% for both engines, so the dominant component flips with the workload.

Decision thresholds on `g` (declared now): `g < 1.25` solver stage negligible for
total cost; `1.25 <= g < 2` minor; `g >= 2` the solver stage is the majority. A
prediction that fails is reported as failed.

## Success, stop and honesty conditions

Complete means: every planned child either completes or records a bounded outcome
(timeout, oom, setup_failed); every completed algebraic root set equals the
exhaustive FES set and the m83 archived receipt for the same cell (input hash,
equation hash, roots, group verification); `verify.py` recomputes every table number
from the committed receipts. Deadlines and inputs are not changed after results are
seen; smoke runs for crash testing are discarded, not reported. Timings are
single-host, shared-CPU, single-trial per repetition; no hardware-independent claim.
Python 3.13 and a different Z3 build than the m83 run (Python 3.12, Z3 4.15.3) are
recorded in the manifest and listed as a deviation.

## Reproduction

```bash
cd research/notes/ecc2k130/solver_total_cost_20261011
python3 preflight.py
PYTHONHASHSEED=0 timeout 3000 python3 run_w1.py --out results/w1 --reps 3
PYTHONHASHSEED=0 timeout 3000 python3 run_w2.py --out results/w2        # if CPU remains
python3 summarize.py && python3 make_figures.py && python3 verify.py
python3 build_report.py            # report.pdf
```
