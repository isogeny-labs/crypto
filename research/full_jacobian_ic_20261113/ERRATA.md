# Errata for `RESULTS.md` (independent validation, 2026-11-14)


`research/validation_20261114/REPORT.md` admitted this run with caveats
(`VALIDATION.json`, `[pcc:unverified]`). It recomputed 2,057 values, replayed
all 9 primes with 0 non-timing differences and re-checked every log with
independent group-law and Cantor code. It found eight reporting errors, none of
which changes a cell, a class or the verdict. `RESULTS.md` is left as validated (its hash is pinned by the validator
report); the corrections are here.

1. F6 was scored "held"; it failed on 3 of 176 runs.
2. The smallest `w = 0` ratio is B's 17.96, not A's 18.7.
3. Three secondary fits were over all 9 primes without saying so.
4. "0 bad relations" was not a measurement (the checks were off).
5. Linear-algebra additions are not on the band; they appear only in
   `total_w1_with_additions`, as `PROTOCOL.md` says.
6. The seeds in `PROTOCOL.md` (700+s, 500+s, 1000+s) differ from the code
   (700+s+10k, 500+s+10k, 1000+s+(seed mod 997)). Both are fixed formulas.
7. The IC arms are lower bounds but carry `lower_bound: false`. Uncounted work
   (partition hash, large-prime map, RNG draws, `[h]Z`) and the unit (one
   Cantor step counted as one E1 addition, though it costs about 4.3–5.7 E1
   additions in time) both understate the IC side, so every ratio is
   conservative for "above".
8. p = 102,407 is absent from `stage2_not_run`, and the killed first stage-2
   run is not in the bench record.

The validator also could not establish the freeze ordering from git (the
protocol and data entered in one commit), and the smoke run had already shown
the operation counts at p = 211 and 419 before the predictions were frozen.

Corrected statements, replacing the ones in `RESULTS.md`:

- Line 28: no bad-relation count was measured (relation and walk checks were
  off); the logs themselves are certified (360/360 by the validator).
- Lines 29–30: group operations and linear-algebra multiplications are on the
  band; additions are only in `total_w1_with_additions`.
- Line 52: the smallest whole-J ratio at `w = 0` is 17.96, for B at p = 211
  (controls: B_S 10.9, A_S 11.9).
- Lines 75 and 79–80: the fits "0.234 against ln r" and "0.420 (A), 0.292 (B)"
  are over all 9 primes. Over the 7 stage-1 primes they are 0.238, 0.074 and
  −0.033, so they do not show a positive B trend. The reading rests on the
  cells: at least 17.96× rho at `w = 0`, 406× at `w = 0.1`.
- F6 (scorecard): **failed**. 3 of 176 whole-J runs exceed 0.6, all at
  p = 211 (0.6031, 0.6031, 0.6034); per-prime means are 0.4830–0.5398.
