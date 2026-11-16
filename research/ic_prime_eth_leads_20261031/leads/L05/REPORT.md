# L05: PDP-aware SAT-policy synthesis on the prime-field hybrid encoding (harness + pilot)

Workdir: /tmp/claude-501/L05 (= $TMPDIR/L05; $TMPDIR in this sandbox is /tmp/claude-501). No LLM calls, no Modal. Sweep repo untouched.

## What was wired (measured: builds and runs)
- Copy of the sweeper: /tmp/claude-501/L05/sweeper (patch script patch1.py, applied once). Added:
  - `CONFLICTS=N` env -> `kissat --conflicts=N` (deterministic cap); `KISSAT_SEED`; statistics parse. `ticks` is NOT captured (field is 0 in every row; kissat's statistic key did not match). Work proxy = conflicts (decisions, propagations also logged).
  - `--rename none|hi|lo|rand`: variable renaming applied at DIMACS write time; model is un-permuted before decoding, so every SAT is still decoded and verified by exact curve arithmetic and every UNSAT checked against the exhaustive oracle (0 FAIL verdicts in 350 rows). hi/lo = window bits (bits of x1,x2,x3) get highest/lowest indices; rand = seeded uniform permutation of all vars.
  - Symmetry on/off via the existing `--sym 1|0`.
- This is a proxy for a branching policy: kissat's VSIDS/VMTF tie-breaking by index. It is NOT a branching hook. The CaDiCaL external-propagator dir (crypto/research/prime_sat_prefix_search_20261030) is absent, so no engine with per-decision hooks exists on disk.

## Commands
- Driver: /tmp/claude-501/L05/run_pilot2.sh (15 background processes: 5 variants x seeds 11,12,13). Core call:
  `CONFLICTS=150000 ./sweep2 sweep --m 3 --bits 8 --seed S --two-adicity 1 --families small-x --encodings bits-grouplaw --ls 2,3,4 --planted 5 --uniform 5 --timeout 400 --sym {1|0} --rename {none|hi|lo|rand} --out res_b8/VAR/seedS`
- Analysis: `python3 analyze.py res_b8 150000` -> analysis_b8.txt (bootstrap 2000 draws, seed 0, resampling instances within l).
- Data: res_b8/{ref,nosym,hi,lo,rand}/seed{11,12,13}/runs.jsonl (350 rows total, 70 per variant).

## Instances
- m = 3, b = 8 bits (p = 197..~250 per seed), small-x factor base, bits-grouplaw encoding (the encoding that terminates in the existing sweep), kissat 4.0.4 default options, cap 150000 conflicts, 3 curves (seeds 11,12,13), 5 planted + 5 uniform targets per (curve, l) when the base allows. Targets and curves are identical across variants (paired).
- Split SAT/UNSAT by the exhaustive oracle (`oracle_sat`), not by target kind.
- Realised cells per variant: l=2: 15 instances (all UNSAT; fb has 1-2 abscissae so planted targets cannot exist, T_enum 0 or 4); l=3: 25 (12 SAT, 13 UNSAT; T_enum 12-24); l=4: 30 (28 SAT, 2 UNSAT; T_enum 60-144). So ~30 instances in the top cell, 15-25 below, and the UNSAT side at l=4 is only n=2.
- Deviation: the planned b=12 ladder was abandoned. Measured (aborted pilot, /tmp/claude-501/L05/results, 125 rows): 104 of 125 runs hit the 1e5 conflict cap at b=12 for l=2..4, so the window effect is invisible. That agrees with the existing m=3 sweep (all 14/16-bit cells censored). Bits 9 and 10 calibration (cal/*.log): 4e4-1.3e5 conflicts at l=3, too slow for 30 x 5 runs in budget. So the single feasible rung is b = 8, window s = l = 2,3,4 (NOT b = 3s; p is fixed). This is a ladder in the window at fixed field, not the ladder-rule diagonal.

## Results (measured; analysis_b8.txt). Mean log2(conflicts), 95% bootstrap CI; cens = hit cap
| variant | l=2 UNSAT | l=3 SAT / UNSAT | l=4 SAT (cens) |
|---|---|---|---|
| ref (sym on, native order) | 12.00 | 14.92 / 14.66 | 16.33 (0/28) |
| nosym | 14.13 | 15.56 / 15.59 | 16.70 (4/28 cens) |
| hi (window bits highest) | 11.97 | 14.93 / 14.71 | 16.31 (1/28) |
| lo (window bits lowest) | 12.01 | 15.00 / 14.64 | 16.32 (0/28) |
| rand (control) | 12.26 | 14.80 / 14.66 | 16.34 (1/28) |
Median conflicts, ref: l=2 3.9e3, l=3 3.2e4, l=4 8.6e4. Conflicts / T_enum at l=4: ~600-1000 (so the oracle sits ~3 orders above enumeration, same as the 12-16 bit sweep).

Slope e = d log2(conflicts)/d l, all instances, bootstrap CI (censored runs counted at the cap, so nosym is a lower bound):
- ref: 2.17 [1.93, 2.38] (l=2..4); 1.55 [1.35,1.76] (l=3..4)
- hi 2.17 [1.96,2.39]; lo 2.16 [1.91,2.39]; rand 2.04 [1.86,2.22]; nosym 1.30 [1.10,1.49]
- SAT only (l=3,4): ref 1.41 [1.22,1.60]; hi 1.39; lo 1.32 [1.08,1.54]; rand 1.54; nosym 1.14 [0.80,1.62]
- UNSAT only (l=2..4, n=2 at l=4): ref 2.23 [2.00,2.45]
Paired log2 ratios vs ref: hi, lo, rand are all within noise of 0 in every cell (e.g. hi l=4 SAT -0.02 [-0.19,+0.16]); nosym is worse everywhere: +2.13 [1.76,2.44] (l=2), +0.93 (l=3 UNSAT), +0.64 (l=3 SAT), +0.37 [0.19,0.55] (l=4 SAT). That is 1.3x to 4.4x more conflicts without lex symmetry breaking, reproducing the binary program's finding that symmetry breaking is the dominant lever.

## Censoring
Ref 0/70 censored (150k cap); nosym 6/70 (all at l=4); hi 1, rand 1, lo 0. At b=8 the cap is harmless except for nosym. At b=12 (aborted) 104/125 censored.

## Decision rule applied
- Constant-factor wins: none. The hand variants (renaming) change nothing; the only measurable lever is symmetry breaking, already in the reference.
- Exponent: no policy has e below the reference with CI excluding 0 on 3 sizes. The one nominally lower slope (nosym, 1.30 vs 2.17) is an artifact of the lever it removes: the symmetry saving shrinks from 4.4x (l=2) to 1.3x (l=4) while nosym stays strictly worse at every size; l=2 cells are degenerate (1-2 abscissae). It is not a win.
- Reference e is above 1 on every cut (lowest 1.14-1.41, SAT only, l=3..4). Nothing is below e = 1 at m = 3. The ladder rule needs e < 0.5 at m = 3 for rho; the reference is 3-4x too high.

## Verdict
closed (pilot-level) for the prime SAT lane at this encoding family: reference slope e = 1.4-2.2, hand policies equal it, no variant below 1. Strength is limited: one field size, three curves, window 2..4 at fixed p, kissat only, variable-index proxy rather than branching hooks, tiny and partly degenerate factor bases, UNSAT side thin at l=4. It is consistent with the existing 12/14/16-bit sweep and with the e ~ m-1 = 2 enumeration-like cost. I do not claim an asymptotic statement.

## Skeptic re-run (one command, ~25-40 min on a loaded host)
`cd /tmp/claude-501/L05 && ./run_pilot2.sh && python3 analyze.py res_b8 150000`
(move res_b8 aside first; the sweeper refuses to overwrite). Cheaper: `CONFLICTS=150000 ./sweep2 sweep --m 3 --bits 8 --seed 12 --two-adicity 1 --families small-x --encodings bits-grouplaw --ls 3 --planted 2 --uniform 2 --rename hi --out /tmp/x`.

## Owner decisions needed before an LLM search (plan, not done)
1. Engine with branching hooks: a CaDiCaL external-propagator (decision callback) build or kissat fork; none exists on disk. The proposer's action space must be per-decision variable/phase choice, not renaming (renaming is shown inert here).
2. Work counter: kissat `ticks` parse is broken in this harness (0); either fix it or define an instruction-count metric as in pdp-sat-synth. Conflicts are a relabelling-prone proxy (protocol warns node count can mislead).
3. Budget and ladder: the feasible rung is b=8..10, l=2..4 only; confirm whether the search should run at b=8 (cheap, ~1e4-1e5 conflicts) knowing there is only 1 size with non-degenerate factor bases, or fund a larger-field ladder (b=12+ censors at 1e5).
4. Promotion set: PROTOCOL.md uses 999 fresh instances; at ~30 s per l=4 run that is ~8 CPU-hours per candidate-policy arm; confirm.
5. Whether to proceed at all: the pilot gives no sign of exponent headroom, so I recommend not funding the search unless an engine with structure-aware hooks (L1/L2) is built first.
