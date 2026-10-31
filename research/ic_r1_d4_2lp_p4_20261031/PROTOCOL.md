# Protocol R1 (frozen 2026-10-31, before any measured run)

Question: on E(F_{p^4}) (Gaudry k = 4 decomposition), does stacking the D4 symmetry fold with
double large primes (2LP) change the fitted exponent in p of total counted cost, against
plain, D4 only and 2LP only, and against rho on the same group?

Producer: `ic.k4_gaudry` (src/cryptanalysis/ecbench/fp4.rs), the only registered producer that
exposes both D4 (`base=yline-d4`) and 2LP (`large_primes=2`). `ic.pipeline` with
`fghr-y-line` + `gaudry-d4` (fp4_torsion curves) has no large-prime option, so it cannot carry
the stack. Linear algebra in `ic.k4_gaudry` is the repo's CADO-style filter to a square core
followed by Wiedemann (the `sparse-cado` = `filter+wiedemann` pair), accepted only when [d]G = Q.
That is the full-rank completion rule: first attempt at small+1 full relations, retry +5 percent.

Arms (spec SPEC-r1.json, sha256 in RESUME): rho-neg (reference), plain (xline, c4 1.213e12),
d4 (yline-d4, c4 2.54e10), 2lp (xline, large_primes 2, lp_exponent 0.95, c4 1.213e12),
d4-2lp (yline-d4-lp, large_primes 2, lp_exponent 0.95, c4 2.54e10). 2LP parameters are the
values frozen by item ic_improve_20261014/01 tuning; not retuned here. No other symmetry.

Workloads: fp4_translation p in {101,181,269,397,521,769,1033}, curve seeds 1..3,
2 targets per curve (target_seed 20261031), 1 round. Hypotheses fixed in advance:
H0 the slope of log2 S_whole vs log2 p is equal across arms (difference CI contains 0);
H1 d4-2lp slope differs from d4 and from 2lp.

Statistic: for each arm and p, mean over targets of S_whole (and per phase) per curve; OLS of
log2 S on log2 p over curves; 95 percent CI by bootstrap over curves (2000 resamples, fixed
seed), and paired bootstrap for slope differences between arms. Report also the LA-phase and
relation-phase slopes, the LA share, and a reprice at c4 / 340 (the #1792 D4 measurement
noted in the R10 ledger) computed offline from counters, labelled as a reprice.

Stop rule: a (p, arm) cell whose session runs longer than 1 h machine time is skipped and
reported. Counted metrics only (host load is high). No wall-time claim. Oracle work is
uncharged (`oracle_*_uncharged`), so every S is a lower bound.
Out of scope: twist/GLS symmetry, other 2LP tunings, ic.pipeline Macaulay oracle at p > 29.
