# Errata to RESULTS.md (from the independent validation)

`RESULTS.md` is pinned. The validator's report is `validation/validator-report.md`, disposition
**validated-with-errata** (findings F1–F10). These corrections take precedence over `RESULTS.md`.
Every number is `[pcc:unverified]`.

- **E1 (F1, major): x5\* is full-factor-base IC, not double large prime.**
  - At cc 2.8 every degree-1 place is in the factor base at both genus-5 primes, and no large prime
    is used (`rels_by_lp` = [n, 0, 0]).
  - A replay at cc 4.0 gives identical rows. So "the optimum may lie beyond the grid; x5\* is an
    upper bound" is wrong, and the genus-5 theory exponent quoted in RESULTS (double large prime)
    does not describe x5\*.
  - C2 survives on the genuine double-large-prime arm x5: x5 / x\* = 0.499 [0.487, 0.508] at
    N ≈ 2^56 and 0.351 [0.347, 0.354] at N ≈ 2^60.
- **E2 (F2): the genus-4 fit is curved.** The middle-point residual for x\* is −0.0114
  [−0.0137, −0.0091], which excludes 0, and the local exponent rises from 1.542 to 1.608. The
  interval [1.564, 1.573] covers sampling error only. Strike "the middle points sit on the line".
  "Consistent with 1.5" rests only on the frozen [1.4, 1.6] window.
- **E3 (F3): genus-5 slopes are two-point fits.** They imply cost growing as p^1.18 (x5\*) and
  p^1.30 (x5), against the double-large-prime theory of p^1.6. Do not extrapolate them. P4's
  "holds" rests on two point estimates.
- **E4 (F5): "every native operation charged" overstates.** The per-step hash and the hash-table and
  union-find bookkeeping are uncounted; charging the hash moves the ratios by at most 0.3%. The
  0.003 GAE calibration was measured for genus 4 only.
- **E5 (F6): "genus 5 ahead" is measured against each Jacobian's own rho**, in its own group
  operations. In a shared field-multiplication unit genus 5 is further ahead: 0.237 and 0.156 at
  w = 1.
- **E6 (F4, F7, F8): process.**
  - The raw rows and the search driver (`drive_search.sh`) are now committed. They were previously
    excluded by `.gitignore`.
  - Rows carry only `ok` and `certified`. The validator recovered and certified 16 of 216 logarithms
    independently (log = planted t, and [log]G = T). Future rows must emit `recovered`.
  - Operational events not listed in `DEVIATIONS.md`, none of which affected coverage:
    - one worker killed by the out-of-memory killer;
    - two 2-hour background stops;
    - one container restart.
- **E7 (F9): the G3 rho sanity check is thin.** Its two runs read 0.42 and 1.22 of `sqrt(pi r/2)`.
- **E8 (F10): snapshot copies are not authoritative.** Automated snapshot commits 3b2b980d617 and
  f0ac380b21c caught `validation/` mid-run. The handback copies are authoritative.
