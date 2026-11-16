# Lead B: dual-rail gates on the repository's ECC2K-130 S'4 SAT instance (2026-11-15)

Local pilot, one core of a shared 4-core container. Every number here is an unverified pilot
assertion, tagged `[pcc:unverified]`: no isolab host, no replay certificate, no validator report,
no matched rho arm, no benchmark record, no leaderboard row. Protocol written before the measured
runs: [`PROTOCOL.md`](PROTOCOL.md). Code: `dr.py` (rail post-pass, linear-leaf system, solver
wrapper, independent arithmetic), `b_run.py` (runner), `analyze.py`. Raw rows: `raw/d*.jsonl`,
merged in `results.json` (312 rows). All tables: [`tables.md`](tables.md). Figure:
`conflicts_vs_d.svg` / `conflicts_vs_d.png`.

**Decision by the pre-registered rule: INCONCLUSIVE.** The rule needs a reduction of at least 5x
against arm (a) at d = 7 and at d = 8. At d = 8 all three rail arms pass both legs (50x, 31x,
591x; 0.59, 0.94, 0.050 times the pair count). At d = 7 none passes the 5x leg (3.8x, 3.5x, 4.2x),
because the repository encoding itself needs only about 1,000 conflicts there. [pcc:unverified]

## The repository generator ran

`research/nagao_relations/solver_16/weil.py` (`buildS4`, `System.cmsClauses`,
`uniformTarget`, `plantedTarget`, `verifyProjected`) was imported and run unchanged at n = 131:
K_0: y^2 + xy = x^3 + 1 over F_2[t]/(t^131 + t^8 + t^3 + t^2 + 1), V = {deg x < d} \ {0}, m = 3.
This is the instance of `RESEARCH_ECC2K130_RR_SOLVER_PANEL.md` section 11. No fallback
re-implementation was needed. Differences from the panel: pycryptosat 5.18.0 here (panel: 5.15);
conflicts are the solver's own per-call count from its verbose line (panel: smallest deciding
`confl_limit` by bisection); seeds 2026111500 + d, 8 uniform targets (panel: 4). Arm (a) at d = 8
measures 29.5x the pair count here against the panel's 36.7x.

## Arms

| arm | instance | solve |
|---|---|---|
| a | repository S'4 CNF+XOR as is (d = 8: 1,510 variables, 4,835 clauses, 176 XORs) | one free solve |
| b | a + `dr.railify`: for every AND gate t = a & rest and every input a, a rail r = (not a) & rest and the XOR t + r + rest = 0 (d = 8: 4,904 variables, 15,017 clauses, 3,570 XORs) | one free solve |
| c | instance of b | outer loop over x1 = 1 .. 2^d - 1 as assumptions, one incremental solver |
| a-scan (control) | instance of a | same outer loop |
| L-scan | chained system S3(x1, xR, u) = 0, S3(x2, x3, u) = 0 with s = x2 + x3, p = x2 x3 (second equation affine in (s, p) once u is known), built with the repository's `Descent`/`System`; rails with selectors x1 and u (d = 8: 9,412 variables, 27,702 clauses, 5,394 XORs) | same outer loop |
| L-plain-scan (control) | chained system, no rails | outer loop, 8 sampled leaves per target, 5,000 conflicts per leaf |
| L-free, L-rail-free | chained system, without / with all rails | free solve, 2 uniform targets per d, 20 s |

## Result: non-decomposable targets, mean conflicts per target as a multiple of the pair count [pcc:unverified]

Reference named throughout: pair enumeration, C(A, 2) unordered abscissa pairs (the panel's null
object). 8 uniform targets per cell for d = 5..8; 2 for d = 9. ">=": censored, lower bound.

| d | A | pairs | scan count 2^d-1 | (a) repository | (b) + rails, free | (c) + rails, x1 loop | a-scan control | L-scan |
|---|---|---|---|---|---|---|---|---|
| 5 | 17 | 136 | 31 | 0 conflicts | 0 | 0 | 0 | 79 = 0.58x |
| 6 | 34 | 561 | 63 | 0 conflicts | 0 | 0 | 0 | 124 = 0.22x |
| 7 | 74 | 2,701 | 127 | 993 = 0.37x | 259 = 0.096x | 285 = 0.11x | 635 = 0.24x | 235 = 0.087x |
| 8 | 131 | 8,515 | 255 | 251,039 = 29.5x | 5,025 = 0.59x | 7,988 = 0.94x | >= 255,132 = >= 30x (22 of 2,040 leaves at the 20,000 cap) | 425 = 0.050x |
| 9 (exploratory, 2 targets) | 252 | 31,626 | 511 | >= 1,266,012 = >= 40x (0 of 2 decided in 45 s) | >= 36,864 = >= 1.2x (0 of 2 decided in 45 s) | >= 120,858 = >= 3.8x (301 and 319 of 511 leaves in 240 s) | not run | 818 = 0.026x |

Conflicts per leaf in the scan arms (x1 fully assumed):

| d | (c) S'4 + rails | a-scan, S'4 plain | L-scan | L-plain-scan |
|---|---|---|---|---|
| 5 | 0 | 0 | 2.56 | >= 5,002 (all 64 sampled leaves capped) |
| 6 | 0 | 0 | 1.97 | >= 5,002 (all capped) |
| 7 | 2.25 | 5.0 | 1.85 | >= 5,002 (all capped) |
| 8 | 31.3 (largest leaf 261) | >= 1,001 | 1.67 (largest leaf 79) | >= 5,002 (all capped) |
| 9 | 390 over the 620 leaves reached (largest 2,222) | not run | 1.60 | not run |

Reduction against arm (a), ratio of mean conflicts, baseline = arm (a) on the same targets:

| d | a / b | a / c | a / L-scan | a / a-scan |
|---|---|---|---|---|
| 7 | 3.83 | 3.48 | 4.23 | 1.56 |
| 8 | 50.0 | 31.4 | 590.7 | <= 0.98 |
| 9 | not defined (both censored) | not defined (both censored) | >= 1,549 | - |

Per-target conflicts at d = 8: (a) 183,160 to 320,914; (b) 2,722 to 14,068; (c) 7,402 to 8,785;
L-scan 404 to 464. Every (b), (c), L-scan target is below every (a) target.

Free solves of the chained system do not finish: L-free is censored at 20 s on all 8 runs
(>= 248,513 to 415,749 conflicts) and L-rail-free on all 8 (>= 767 to 3,503 conflicts in the same
time; its Gauss matrix has 6,000 to 9,000 columns and each conflict is slow).

## Decomposable (planted) targets, first model, 4 per cell [pcc:unverified]

| d | pairs | (a) | (b) | (c), loop stops at first model | a-scan | L-scan |
|---|---|---|---|---|---|---|
| 5 | 136 | 2 = 0.015x | 0 | 5.5 = 0.040x | 6.3 = 0.046x | 46 = 0.34x |
| 6 | 561 | 2.5 = 0.0045x | 0 | 8.3 = 0.015x | 8.5 = 0.015x | 65 = 0.12x |
| 7 | 2,701 | 1,091 = 0.40x | 164 = 0.061x | 81 = 0.030x | 290 = 0.11x | 112 = 0.042x |
| 8 | 8,515 | 82,970 = 9.7x | 2,605 = 0.31x | 447 = 0.053x | 12,939 = 1.5x | 109 = 0.013x |

The scan arms stop at the smallest planted abscissa, so their matching reference is the scan with
early stop, not the full pair count. Four targets per cell; (a) at d = 8 ranges 8,619 to 225,993.

## Readings

- **d = 8 is the first size where the repository encoding searches hard, and there the rails
  change the count by a factor 31 to 591.** Arm (b), a pure CNF+XOR post-pass with no outer loop,
  lands at 0.59 times the pair count where the repository encoding is at 29.5 times.
- **Attribution (pre-registered leg): rails, not the outer loop.** Arm (c) is 0.031 times the
  a-scan control at d = 8 (threshold 0.5). The outer loop alone, on plain gates, costs at least
  as much as the free solve (>= 255,132 against 251,039). The protocol's prediction that most of
  the reduction would come from the outer loop was wrong.
- **The rails do not keep S'4 near the pair count at d = 9.** Arm (b) passes 36,000 conflicts
  (1.2 times the pairs) undecided at 45 s, and arm (c) rises from 31 to 390 conflicts per leaf.
  From d = 8 to d = 9 the pair count grows 3.7x; arm (c) grows more than 15x. Two targets,
  censored: a direction, not a slope.
- **Only L-scan stays flat per leaf**: 2.56, 1.97, 1.85, 1.67, 1.60 conflicts per leaf for
  d = 5..9, the same level as the toy pilot's 1.6. Its total is 1.6 to 2.6 times the scan count
  2^d - 1 and 0.026 to 0.58 times the pair count. This arm is the 2+1 scan carried out by an outer
  loop with the solver doing the linear algebra of each leaf; it is the scan by construction, not
  a solver that finds the scan. Without rails the same leaves do not finish in 5,000 conflicts
  each.
- **Conflicts are not work.** Mean wall time per uniform target at d = 8 on this loaded machine:
  (a) 8.4 s, (b) 7.0 s, (c) 14.0 s, L-scan 18.7 s. At d = 7: 0.06, 0.40, 1.0, 11.1 s. Per
  conflict the railed instances are 40 to 1,300 times slower (three times the variables, 20 to 30
  times the XOR rows). Pair enumeration in plain Python on the same targets took 1 to 3 s at d = 8.
  No arm is shown to cost less than pair enumeration in time or in field operations.
- Below d = 7 the comparison is empty: arms a, b, c and a-scan all refute with 0 conflicts (the
  linear certificate of the panel's section 9), and L-scan pays 80 to 124 conflicts for a
  certificate the symmetrised system gives for nothing.

## Correctness

- `dr.FastField` agrees with the repository's `field.Pb` on 100 random products per run.
- Ground truth by pair enumeration in own arithmetic for all 50 targets (d = 5..8: 8 uniform + 4
  planted; d = 9: 2 uniform): every planted target decomposes with exactly its planted triple
  among the triples found, no uniform target decomposes.
- 235 decided answers (80 SAT, 155 UNSAT; arms a, b, c, L-scan, and a-scan where no leaf was capped) all equal the
  ground truth, so the arms agree with each other on every target each decided. All 80 SAT models
  lift, in own arithmetic, to points with signs summing exactly to R, and pass the repository's
  `verifyProjected` (relation, not degenerate); each is a triple found by pair enumeration.
- Not decided, hence not checked: all L-plain-scan, L-free, L-rail-free runs; a-scan at d = 8
  uniform (7 of 8 targets have capped leaves); arms a, b, c at d = 9.
- `railify` only adds definitions and XORs implied by the gates; SAT answers of arms b, c
  verifying against the curve is the evidence that it does not cut solutions. A direct
  equisatisfiability proof was not written.

## Deviations, errors and limits

- **Rule evaluated as written.** The d = 7 leg fails, so the verdict is inconclusive although
  d = 8 passes with a wide margin. The rule was not changed after the data.
- **d = 9 differs from the protocol.** Planned: arms c, a-scan, L-scan on 8 + 4 targets. Run: arms
  a, b, c, L-scan on 2 uniform targets, for time. Arms a and b at d = 9 were added after seeing
  d = 8 and are post hoc; a-scan at d = 9 and all planted d = 9 targets were not run.
- d = 5 is an extra full cell (not in the protocol's list); its first 3 targets had been used once
  as a smoke test of the runner before the measured launch, with identical results.
- First launch stopped during d = 6: the runner asserted the repository's `sum_is_R` flag, which is
  false when the first matching sign pattern gives -R. The assertion was corrected to the
  repository's own criterion (relation, not degenerate); the exact-sum check stays in own
  arithmetic. Rows of that launch are in `raw/aborted_first_launch/` and are not used; d = 5 was
  rerun with identical conflict counts on the arms that are not capped. Second launch vanished after the first d = 8 row with no
  traceback (cause unknown; other agents were starting and stopping processes); its one row is in
  `raw/killed_second_launch/` and was reproduced exactly (248,702 conflicts). The runner was then
  renamed `run.py` -> `b_run.py`.
- **Collateral damage to another pilot.** Stopping my first launch with `pkill -f "run.py raw/"`
  also matched and terminated two runs of the sibling pilot `C-block-refutations`
  (`run.py raw/sweep_l8...` mid-run and `raw/sweep_l9...` at start; its `raw/log.txt` shows two
  "Terminated" lines at 07:39:50 UTC). Those two files of pilot C are truncated or empty and need a
  rerun by its owner. I did not touch that directory.
- Free-solve caps overran: pycryptosat's `time_limit` returned after 46 to 49 s (cap 45 s) and
  27 to 39 s (cap 20 s). Wall times come from a machine with load 4 to 6 and are not comparable
  across runs.
- Protocol-before-data rests on file times only (`raw/started.txt`); there is no Git history.
  Development on seed 999 (listed in the protocol) fixed the caps and the selector choice for
  L-scan; it also showed arm b below arm a at d = 7 before the protocol was written.
- Not covered: decisions and propagations (not exposed by pycryptosat; verbose level 1 prints
  conflicts only); solver options other than the defaults (the toy pilot used
  `autodisablegauss=0, sls=0`); a fresh solver per leaf; rails on a subset of the S'4 gates (which
  gates carry the effect); the 3-input gates re-emitted as chains; arm b decided at d = 9 or
  beyond; a-scan at d = 9; d > 9; m = 4; the RR encoding; the ONB/weight factor base of
  `ecc2k130/codegen/indexcalc.py` (a different, non-subspace instance, no scan coordinate); WDSat;
  a second curve or field; any cost in field operations; any proof-size statement.
- One conflict is not one pair test. The multiples above compare a solver counter with a count of
  group operations, as the panel does; they are not a cost ranking.
- Not done because this pilot was confined to its directory and to no Git changes:
  `docs/session-logs/` entry, Conductor task, research-kb receipt, lessons brief, resume
  checkpoint, PDF report, benchmark record. This README is the handoff.

## Every arm run

a, b, c, a-scan, L-scan, L-plain-scan at d = 5, 6, 7, 8 on 8 uniform + 4 planted targets; L-free
and L-rail-free at d = 5, 6, 7, 8 on 2 uniform targets; a, b, c, L-scan at d = 9 on 2 uniform
targets. Nothing else was measured after the protocol. Development runs are listed in the protocol.

## Rerun

Smallest cell (d = 5, the five decided arms, about 30 s):

`python3 b_run.py /tmp/b_d5.jsonl 5 a,b,c,a-scan,L-scan && python3 analyze.py /tmp/b_d5.jsonl`

(`analyze.py` with a file argument rewrites `tables.md`, `results.json` and the figure from that
file; `python3 analyze.py` without arguments restores them from `raw/d*.jsonl`.) Full cell:
`python3 b_run.py raw/dN.jsonl N` for N = 5..8 (about 4, 4, 6, 22 minutes on the loaded machine);
d = 9 as run: `python3 b_run.py raw/d9.jsonl 9 a,b,c,L-scan 2 0`. Run one process at a time.
