# PKM Kummer tower m = 3: H0 lower-bound evidence from specialised msolve runs, with controls

**Status: frozen** by the commit that adds this file, before any specialised system has been run.
Workstream E of `ic-regime-review-20261024/PLAN.md` (REVIEW.md §1 item 3, §3 item 2, §5.1 F3).
Labels: *recorded* (a committed row), *derived* (calculation), *measured* (this study's runs),
*predicted* (stated here, before the runs).

## 1. Question

The unspecialised `m3_t7_x221026520` (N = 21) ran out of pairs at degree 7 and then died at
99.6 GiB inside the first degree-8 round (*recorded*: `research/pkm_m3_n21_msolve_20261018/RESULTS.md`,
A1 addendum). Does fixing one or two tower variables bring the msolve F4 trace to a productive degree-8
step, or to completion, within a 12-16 GiB cap? What H0 lower-bound evidence (solving degree grows with N)
does that give, read through controls? (H1, bounded degree, cannot be supported by this design.)

## 2. What a specialised run can and cannot show (read before the table)

Specialisation `x := c` is a ring map `K[x,...] -> K[...]`. A degree-<=D certificate of `1` in the original
ideal `I` (a combination of generators times monomials of degree <= D) maps to a degree-<=D certificate for the
specialised ideal `I'`. So the specialised solving degree is **at most** the original's (*derived*). The ideals
differ, and `I'` is generally strictly easier. Consequences, fixed now:

1. **Only a result of >= 8 counts.** A specialised N = 21 run that still **needs degree >= 8** (a printed round
   with `deg >= 8` and `new > 0`, or a completed run whose certificate of `1` needs degree >= 8) is an **H0 lower
   bound** on the original: `D(I) >= 8`. This holds **only if `I` itself is the unit ideal** (no solution);
   if `I` has solutions the specialised system can be unsatisfiable while `I` is not, and the bound says nothing
   about `I`. The unit-ideal status of the two N = 21 targets is **not established** (target 0: degree 7
   exhausted with `1` not in the basis, run 20261018T2208Z-mt; unit-ideal confirmation absent). This study
   reports it for the targets only if an unspecialised run completes, and otherwise labels every N = 21 reading
   "conditional on `I` being the unit ideal".
2. **A specialised run that finishes at degree <= 7 says nothing about H1** (nor about H0): it is the expected
   result of an easier ideal.
3. Existing H0 evidence, cited, not produced here: on target 0, the A1 trace shows degree 7 exhausted without `1`
   in the basis (`pkm_m3_n21_msolve_20261018/reports/run-20261018T2208Z-mt/README.md`), i.e. `D > 7` if `I` is the
   unit ideal. It lacks the unit-ideal confirmation and any finished degree-8 round.
4. **Controls calibrate how much each specialisation lowers the degree.** Same arms applied to (a) `m3_t5_x1289903598`
   (N = 15, recorded `D = 7`, unit ideal, `.out` `[-1]`), and (b) the m = 4, N = 20 cell
   `m4_t5_x362838726` (recorded `D >= 8`, `.../03-pkm-degree/README.md` lines 11-17). The tower level fixed (head
   `k = 0` or mid) and the value (`P = 1` or `3`) are varied. N = 21 is read only through the control drops.
5. The dreg-closure specialisation study (`research/dreg_closure_specialise_20261017`) is GF(2) Weil-descent
   MutantXL on different systems and its null control was invalid. It is cited only as the reason these controls
   are required, not as precedent for the trick.
6. Fixed values are roots of unity (finite set); no claim about other roots.

## 3. Systems and how the arms are built

The committed unspecialised systems are in
`research/ic_next10_20261014/03-pkm-degree/systems/pkm_kummer_p2013265921_m3_t{5,7,8}_*.ms`
(*recorded*: written by that card's generator in `.../03-pkm-degree/gen/`, the t8 systems `..._m3_t8_x1227422598.ms` and `..._x1272776806.ms` included; `p1 = 2013265921`, three
Kummer chains `y_i_0 ... y_i_{t-1}` with `y_i_{k+1} = y_i_k^2`, `y_i_{t-1}^2 = 1`, and one target variable
`u0`; two target equations, the first coupling `y0_0, y1_0, u0`, the second `y2_0, u0`). A chain variable
`y_i_k` is forced to a root of unity of order dividing `2^(t-k)`; so the fixed value is a **primitive**
`2^(t-k)`-th root of unity, `(31^((p-1)/2^(t-k)))^P` with `P` odd (31 is a primitive root of `p`).

`specialise.py` substitutes the constants into every polynomial, drops the fixed variable and writes a
new `.ms`; nothing else changes (the resulting linear chain equations are left for msolve).

| arm | fixed variables | `P` | meaning |
|---|---|---|---|
| F0 | none | - | the unspecialised system (control identity) |
| H1, H1b | `y0_0` (chain head) | 1, 3 | removes chain 0 entirely |
| H2 | `y0_0`, `y1_0` | 1, 3 | removes chains 0 and 1; first target equation becomes univariate in `u0` |
| M1, M1b | `y0_k`, `k = floor(t/2)` | 1, 3 | chain 0 keeps `k` free squarings below the fixed value |
| M2 | `y0_k`, `y1_k` | 1, 3 | both coupled chains keep `k` free levels; target coupling stays genuine |

*Derived structural remark*: H2 is expected to collapse the problem (a univariate quadratic in `u0` fixes
`u0` to at most 2 values), so H2 is a floor on how far specialisation can lower the degree, not a test. M-arms
keep the coupled structure and are the informative arms. `k` = 2 (N = 15), 3 (N = 21), 4 (N = 24).

## 4. Runs (frozen list)

| id | system | arms |
|---|---|---|
| control (a) | `m3_t5_x1289903598` (N = 15, D = 7) | F0, H1, H1b, H2, M1, M1b, M2 |
| control (b) | `m4_t5_x362838726` (m = 4, N = 20, D >= 8) | H1, M1, M2 (F0 is already recorded; not rerun) |
| first (cheapest informative) | `m3_t7_x704078485` (target 1, N = 21) | F0: unspecialised, to the end of degree 7 (earlier stop: 7,728 s wall in the 4th degree-7 round, 8.9 GB) |
| target | `m3_t7_x221026520` (N = 21) | H1, H2, M1, M2 |
| stretch | `m3_t8_x1227422598` (N = 24) | M2, then H2 (only if time permits) |

Queue B (starts first): target 1 F0, then target 0 arms H2, H1, M1, M2. Queue A: control (a), control (b), then stretch.
At most two msolve processes at once.

## 5. Instrument

- `/opt/homebrew/bin/msolve` **0.9.5** (this host). The earlier study used 0.6.5. The control arm F0 must give the
  recorded answer (one-element basis, no solution) and the recorded max degree 7
  (`runs-mt/*m3_t5*` of the earlier study); if its degree trace differs, N = 21 numbers are still reported but
  not compared against the 0.6.5 unspecialised record, and the control arms alone calibrate.
- `msolve -t 4 -v 2`, `OMP_NUM_THREADS=4`. Host load is 60-110 so **wall caps are not used**: `run_one.py` stops a
  run at **21,600 CPU-seconds** (user+sys of the process from `ps cputime`, about 1.5 h at four busy threads) with a
  12 h absolute wall backstop, and at the RSS cap below. Wall time is recorded, not read.
- **RSS cap.** The host has **48 GB** RAM shared with other agents (`system_profiler`; `vm_stat` showed almost no
  free pages). Cap per process: **12 GiB**, except the first run (target 1 F0, whose sibling reached 15.3 GB in a
  degree-7 round) at **16 GiB**; at most two processes at once, so at most 28 GiB in use. *Derived*: degree-8
  monomials number 4.29M (N = 21), 3.11M (N = 20), 2.22M (N = 19), and the earlier study derived 40-120 GB for an
  unspecialised degree-8 round (`pkm_m3_n21_msolve_20261018/PROTOCOL.md` A1.5). A memory stop in a degree-8 round is
  therefore the expected outcome of unspecialised and mildly specialised runs; it is recorded as "degree 8
  reached, not productive" and is not evidence either way. macOS ignores `ulimit -v` (checked), so `run_one.py`
  polls `ps` every 0.5 s and kills on RSS; it records the polled peak and `ru_maxrss`.
- `run_all.sh` runs a queue; `make_systems.sh` builds `systems/`; `runs/NAME.{log,out,mem}`; logs over 2 MB
  are trimmed to head and tail (first 500 and last 500 lines) and the original byte count is noted.
- Degree trace parsed from the `-v 2` log. msolve 0.9.5 prints one line per F4 round with columns
  `deg sel pairs (rows x cols) density (N new) (Z zero) time`, for example
  `7  20637  20689  91252 x 82416  1.32%  2352 new  18329 zero ...` (*measured* in a 60 s smoke run, discarded,
  and in the earlier study's logs). The `deg` column is **not monotone** (rounds at degree 6, 5 and 2 follow a
  degree-7 round). Definitions: `max_deg` = largest `deg` among all printed rounds; a round is **productive** if
  its `new` count is > 0; `d_A(N)` = largest `deg` among productive rounds of a completed run; **degree 8
  reached** = some round with `deg >= 8` is printed (its pairs selected), **degree 8 productive** = such a round
  prints `new > 0`. A round that is selected but killed before printing is "reached, not productive".
- Memory metric: peak RSS (polled `ps` and `ru_maxrss`). Under host memory pressure macOS RSS excludes
  compressed pages, so it can understate the footprint; the RSS guard acts on RSS.
- If `ps` fails or returns nothing, `run_one.py` kills msolve and records `stop=guard_unavailable` (never runs
  unguarded).

## 6. Outcome rules (fixed before the runs)

Per run: **completed** (exit 0, answer file written; the `.out` is recorded as unit `[-1]` or not), **mem stop** (guard), **cpu stop**.
Per arm pair (N, A): `d_A(N)` for completed runs; for stopped runs, "reached degree g, round g incomplete"
is a lower bound `d >= g-1`, and `d >= g` only if a degree-g round reported new elements.

| observation (specialised N = 21 arm) | reading |
|---|---|
| productive round with `deg >= 8`, or completes needing degree >= 8 | **H0 lower bound** `D(I) >= 8`, conditional on `I` being the unit ideal; interpreted against the control drop for that arm |
| completes with `d_A(21) <= 7` | **no reading** (neither H0 nor H1); only the drop `d_A(15) - d_A(21)` is reported |
| completes with `d_A(21) > d_A(15)` but <= 7 | growth in the specialised family only; recorded, no inference to `I` |
| cpu or mem stop with no productive degree-8 round | inconclusive; degree reached recorded |

The unspecialised target-1 run: finishing degree 7 without `1` in the basis duplicates the target-0 evidence for a
second target (`D > 7` conditional on unit); a completed run with `.out` `[-1]` settles the unit-ideal question
for that target.

This study does not decide H0 against H1 in any outcome. Its deliverable is H0 lower-bound evidence with controls; H1 cannot be supported by it.

## 7. Predictions (stated before running)

- *Predicted*: F0 (N = 15) reproduces `D = 7` (msolve 0.9.5 may differ from 0.6.5; the trace is compared). H2 completes in degree <= 4 at all N. H1 completes with
  `d` clearly below 7 at N = 15 and not rising with N. M1/M2 at N = 15 give `d` between H-arm and 7.
- *Predicted*: M2 at N = 21 is the arm most likely to exceed the cap; I do not predict whether it reaches
  degree 8.

## 8. Not done

No native `f4_fp_tower` run; no other prime; no claim about any root of unity other than the ones listed.
