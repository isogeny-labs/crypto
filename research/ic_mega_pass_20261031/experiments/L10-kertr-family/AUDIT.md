# L10-kertr-family -- AUDIT (independent, 2026-11-02)

Toy-scale measurement audit. No security claims. Scope: PROTOCOL.md, README.md, RESULTS.json, src/, analysis/, raw/.
I wrote nothing outside this directory. Scratch files and re-runs were in the session scratchpad (not in the repo).
No `conductor check` was run, following BRIEF 0.6 (T-5736 already holds the tree) and the instruction to write only AUDIT.md here.

## Verdict: VALID, with minor corrections

The headline (d=4 failure rates per cell) is supported by the raw data and reproduced independently. The d=5 outcome
(inconclusive/blocked) is honestly stated, but its stated cause is partly wrong (item 2). There are no changes to any headline number.

## What I checked, and how

| # | Check | Result |
|---|---|---|
| 1 | `src/gf2close.c` and `src/gf2close5.c` are byte-identical to N1 and L7 (shasum) | PASS (5882dae2.., ab620409..) |
| 2 | Binary vs source: compiled a fresh `gf2close` from `gf2close.c` (`gcc -O3`) and used it for all re-runs | PASS, same outputs as the shipped binary's logged outputs |
| 3 | Read the closure engine line by line. Queue-based closure: stored pivot rows (leading-monomial pivot) are multiplied by each variable with x^2=x folding (`col_of(mono|bv)`), truncated at degree d. `low` = pivots with column < nlow = dim(W_d cap B_{<=d-1}). `one` = pivot at column 0. | PASS. Matches the PROTOCOL definition. |
| 4 | Read `bsols` line by line. It checks bilinearity, enumerates all 2^K values of a, and does GF(2) elimination in b with full enumeration of free variables. It is exhaustive. | PASS |
| 5 | Independent UNSAT cross-check of `nsol`. Python `s3sys.count_solutions` (treatise code, pure field arithmetic) gave the same nsol as raw on 6/6 T29 draws (s=0..5; nsol 0,2,0,4,4,2). The C fast-exhaustive-search mode (`sols`, different algorithm) gave nsol=0 on T31 s=15 and T33 s=3. | PASS |
| 6 | Builder equality beyond the 7 named instances. `lib10.draw` equations equal `s3_descent` equation-for-equation on random draws T29 s=0..5, T33 s=3,13, T35 s=8,9. Also checked: V in ker Tr, z on the curve, Tr(a6/z^2)=0 after rejection, and the logged V/z match the regenerated draw. | PASS |
| 7 | Generator vs treatise `gen2.py` (basis=tr0, zmode=curve). Same loop: a6, a2, V from trace-0 elements, z from the curve, and skip if V in ker Tr and Tr(a6/z^2)=1. `lib10` does the same by resampling everything. a2 does not appear in the S3 equations, so it only affects z selection. | PASS |
| 8 | `span_rank` logic (echelon with a highest-degree pivot; low part = pivots at degree <=1). Re-derived by hand. P1 data: T rank n-1 and low 0 on 237/237 draws; A35 rank n and low 1 on 25/25. | PASS |
| 9 | Re-ran `analysis/analyze.py` (patched output path, written to scratch) on raw/. `families`, `imported_A` and `named_validation` are identical to the shipped RESULTS.json. `d5` and `predictions` are NOT reproduced (see item 3). | PASS except item 3 |
| 10 | Raw-data integrity. Seeds are contiguous from 0 with no gaps or duplicates (so no cherry-picking), and each scan stops exactly at the U-th UNSAT draw. `.log` and `.jsonl` are identical. Closure was run only on nsol=0 draws. The `one` histograms match the table: T29 8x1, T31 8x1, T33 16x0, T35 8x0, A35 8x0. low4/dim4 are constant within each cell. | PASS |
| 11 | Re-ran closures (fresh binary, 1 thread; about 2 CPU-min total): T29 s=0 gives (low 4526, dim 31931, one 1); T33 s=3 gives (2968, 48068, 0); A35 s=2 gives (3885, 61425, 0). | PASS. Identical to raw/. |
| 12 | Clopper-Pearson intervals: 0/8 gives 0.3694, 0/9 gives 0.3363, 0/10 gives 0.3085, 8/8 gives 0.6306, 16/16 gives 0.7941. | PASS |
| 13 | Imported A baselines. N1 raw: 10 UNSAT (n=29) and 9 UNSAT (n=31), all one=1. L8 RESULTS: 8 UNSAT at n=33, all one in W4, low4 6580. | PASS |
| 14 | Named-instance validation. `named.jsonl` has 7 unique records, all with `match`, `builder_equal` and `mod_eq_N1` true. | PASS |
| 15 | PROTOCOL timing and predictions. PROTOCOL.md (17:34) predates every raw/ file (17:35 onward). P1-P5 are as README states, and all hold. P6 is correctly reported as untested. | PASS |

Numbers in README match RESULTS.json for every table cell, including the 237 T draws (33+44+118+42).

## Issues found (none change a headline number)

1. **README caveat has a wrong number.** It says T-family UNSAT draws are "about 7-20% of all draws". RESULTS.json gives T
   UNSAT fractions of 8/33 = 24.2%, 8/44 = 18.2%, 16/118 = 13.6% and 8/42 = 19.0% (A35: 8/25 = 32%). The correct
   statement is "about 14-24% (T), 32% (A35)". The field `sat_frac` is the fraction of draws WITH solutions, which is
   confusing next to the text.
2. **Wrong stated cause for the empty d=5 log.** RESULTS.json `d5.cause` says the log was "empty despite PROGRESS=1".
   `src/d5.py` calls `lib10.call`, which uses `subprocess.run(capture_output=True)`. `gf2close5` writes progress to stderr
   (`PROGRESS` branch, line 268). On a clean run that stderr is discarded, and it surfaces only on a nonzero exit. So
   progress could never have appeared in the log, and an empty log carries no information about whether progress was
   made. The conclusion "inconclusive, no W_5 dimension and no `one`" stands. The 4500 CPU-s, 9100 s wall, 6.7 GB peak
   RSS and "swap thrash" figures are unverifiable from the repo (no log, no jsonl). The 6.7 GB figure is consistent with
   the ~6.9 GB triangular storage estimate (331,212 columns for n=33, d=5).
3. **RESULTS.json is not fully regenerable.** `d5` (an agent-written entry) and `predictions` were added by hand after
   `analyze.py` ran. Re-running `analyze.py` gives `d5: []` and drops `predictions`, which contradicts README's "RESULTS.json,
   which analysis/analyze.py builds from raw/*.jsonl". The affected content is only the blocked d=5 record.
4. **"Failure rate is 0 or 1" is an observed-rate statement, not a population statement.** The README's population answer
   should read "observed 0/N or N/N in every cell", with the CP intervals as the claim (e.g. T33 >= 0.794, T29/T31 <= 0.369).
   Likewise "onset is n=33 for T and n=35 for A" is only over odd n in {29,31,33,35}. n=34 and n>=36 were not run, and
   monotonicity is not shown.
5. **Omission.** PROTOCOL listed an R-family control ("only if time allows"). It was not run, and README does not say so.
   This is harmless, since A35 reproduces the R35a tuple 8/8, but it should be stated. The T-vs-A contrast uses different
   seed streams and V in ker Tr is not the only difference (T also uses the Tr(a6/z^2) rejection and UNSAT conditioning); the
   rejection rule affects z, so "V in ker Tr" is the main but not the only family difference, as PROTOCOL itself notes.
6. **Independence caveat that README already states, worth keeping.** "Fails d=4" means 1 not in W_4 for this closure,
   with no independent certificate for the new draws. My re-run uses the same engine, so it verifies reproducibility and
   builder/UNSAT correctness, not an independent proof of non-membership. No theorem or proof is given in this
   experiment, so there was no proof to check line by line (the Thm reform(3) rejection rule is cited from the treatise).
7. **Disclosed deviations, all fine.** More than 3 threads briefly (4 processes for about 2 min, per README). The d=5 cap
   of 10800 CPU-s exceeds the BRIEF ~20 CPU-min guidance, but PROTOCOL pre-declared it. The T33 extension to 16 UNSAT was
   a stated option.

## Overclaiming check

No security claim. "Novelty: none on onset values" is appropriate. "A35 onset closed as fail" is supported (8/8, same
tuple as R35a). The plateau-equals-treatise statement (T33 2968/48068, R35a 3885/61425) is supported by RESULTS.json.
The README honestly records that predictions were treatise-informed and not independent. No overclaim beyond items 4 and 1 above.

## Suggested corrections (text only)

- Replace "about 7-20%" with "about 14-24% (T), 32% (A35)".
- In RESULTS.json `d5.cause` and README Addendum, replace "log empty despite PROGRESS=1" with "d5.py captured stderr, so PROGRESS output was
  never logged".
- Add to README: "R-family control not run. `d5` and `predictions` in RESULTS.json were added by hand."
- Soften "failure rate is 0 or 1" to "observed 0/N or N/N".

## Reproduce

Cheap checks (about 3 CPU-min): the fresh-compiled `gf2close` on `lib10.draw('T',29,0)`, `('T',33,3)` and `N1.system(35,18,2)['A']`; `sols` mode on T31 s=15 and T33 s=3; `analysis/analyze.py` with the output redirected away from RESULTS.json.
