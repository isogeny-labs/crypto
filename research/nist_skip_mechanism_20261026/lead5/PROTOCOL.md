# Lead 5 (L5-elkies-failure): protocol, frozen before any Elkies count was computed on the cohort

Frozen 2026-10-26 (UTC time recorded in `out/FROZEN_AT.txt` at freeze). Provenance question
only. No ECDLP claim. All numbers below are labelled when they appear: measured (GP output),
derived (arithmetic on measured numbers) or documentary (read from the sibling studies).

## What exists before this protocol (documentary, not recomputed here)
- Sibling studies, read-only, in `/home/user/crypto-autoresearcher/research/`:
  `nist_seed_1997_acceptance_20261024` (69 seed-level positions with certified prime order that
  pass every numerical 1997-draft check: 10 published, 11 "omissions", 48 others) and
  `nist_seed_1997_nearprime_20261024` (99 = those 69 plus 30 positions with a 255-smooth cofactor;
  per-position field, order and subgroup-point records in `runs/points-20261024T022754Z/points.jsonl`).
- `inputs/cohort.tsv` (sha256 `4acc4f95baa8b8da65e6ee6f6fa79aaf620c86c78300d36b50c16c2af9a361b9`)
  is a parse (Python, no arithmetic) of those two files: stream, run, seed index, role, class,
  p, group order u = h*n, cofactor h. Roles: published (10), skipped (the 11 omissions), extra69
  (48), extra99 (30, cofactor > 1). t is computed in GP as t = p + 1 - u.
- In every stream the published position is the last eligible position (documentary, from the
  index lists), so no "visited window" cut is needed: all eligible positions precede or equal it.
- In-sample signal (documentary): on the 21 = 10 published + 11 skipped, published curves have a larger
  Elkies bound L_E (fewer small Elkies primes); reverse tail 0.00112 over 4,480 within-run
  relabellings. These 21 were used to find the direction. They are excluded from the primary test.

## Model M (one primary model)
Elkies prime: odd prime l with kronecker(t^2 - 4p, l) != -1 (same convention as `mechanism.gp`).
E_L(c) = number of odd Elkies primes l <= L for curve c. **L = 100 is fixed now, not fitted** (24 odd
primes 3..97; chosen before looking at any extra curve; not tuned on the 21).
A 1997 SEA implementation that fails at each Elkies prime with probability q gives a curve
survival probability (1-q)^{E_L}. For a run with s eligible positions of which k were published
(count preserved), the model is: P(published set A) = prod_{i in A} w_i / e_k(w_1..w_s),
w_i = (1-q)^{E_L(i)}, e_k the k-th elementary symmetric polynomial. q = 0 is the null (uniform
sets). Failure confined to specific small primes is NOT tested (it would add a fitted set; the
earlier B2 covering test of that form already had no power).

## Primary statistic and test (one)
Pool P58 = the 10 published + the 48 extra69 positions (the 11 skipped are excluded as in-sample).
Within each run rank E_100 over the run's pool positions (midranks for ties). W = sum of those
ranks over the 10 published. Small W means published curves have fewer Elkies primes (the
direction found in-sample, fixed one-sided). Reference distribution: N = 1,000,000 random
count-preserving within-run relabellings of the pool (each run keeps its published count: runs
863966734:3, 863711343:3, 891607927:1, 871696354:1, 872952608:1, 871002724:1), `setrand(20261026)`.
p = (1 + #{W* <= W_obs}) / (N + 1). Decision rule fixed now: p <= 0.05 -> the direction is
supported against fresh comparison curves; otherwise it is reported as not supported. No other
number decides.
Independence caveat, stated in advance: the comparison group (48 extras) is new data, but the 10
published curves are the discovery set. The test is therefore partly out of sample, not fully.
The null (published set is a uniform draw from the pool of its run) is exact for it. The 48 extras
include positions rejected for other reasons (opposite root, order width), which dilutes any
Elkies effect and lowers power but does not bias the null.

## Controls (all must pass or the primary result is void)
- C0 integrity: every one of the 21 rows matches `inputs/historical.tsv` on (p, n); every
  prime-order row has h = 1, isprime(u) = 1 and Hasse |t| <= 2 sqrt(p); counts 10/11/48/30.
- PC1 deterministic planted labels: published = the lowest-E_100 positions in each run (ties by
  index). Must give p < 0.001 against the same reference.
- PC2 planted model draws: for q in {0, 0.05, 0.10, 0.20, 0.35}, 1000 datasets each (seed
  20261027), published sets drawn from model M on the P58 pool (exact conditional-Bernoulli
  sampling), each tested with the primary rule against the same reference distribution. q = 0 is
  the false-positive control: its rejection rate must lie in [0.036, 0.064] (binomial 95%
  around 0.05; ties make the test conservative, so only the upper bound is binding). Power is
  the rejection rate at q > 0. The smallest grid q with power >= 0.8 is the stated detectable q.
  A non-significant primary result is informative only for q at or above that value.

## Secondary results (exploratory, not decision-bearing, labelled as such)
S1 same test on the 99 pool (10 published + 78 extras; cofactor-smooth extras have l | order, which
   forces Elkies by construction, a known upward bias in E for them).
S2 same test on all 69 (adds the 11 skipped; in-sample, expected significant).
S3 primary test with L = 50 and L = 200 (sensitivity to the fixed L).
S4 q-hat: grid q in {0, 0.01, ..., 0.60}, maximizing the model-M likelihood on the 21 in-sample
   curves, with E_100; and power at q-hat from PC2 sampling on the P58 pool (1000 datasets).
S5 descriptive means of E_100 by role.

## Run
`gp -q elkies_fail.gp` from this directory; writes `out/*.tsv` and `out/summary.txt`. If any hook
or gate refuses the command, the computation stops and is reported blocked with the exact message.

## Amendment 0 (still before any computation, 2026-10-26)
Secondary tests S1, S2, S3 use N = 200,000 relabellings each (seed 20261026 + test number), because
the protocol left N open for them. The primary test keeps N = 1,000,000.

## Result (recorded after computing; full numbers in `out/summary.txt`, `out/power.tsv`, `out/cohort_E.tsv`)
Run: `gp -q elkies_fail.gp` (script sha256 in `out/elkies_fail.sha256`; two syntax/loop bugs in the
script were fixed before the final run, one `for(i=s,1,..)` that never iterated; the primary
statistic code was unchanged and gave the same value in the aborted first pass).
No hook or gate refused any command.
- C0 integrity: PASS (21 of 21 historical rows matched on p, n, role; counts 10/11/48/30).
- Primary (measured): W_obs = 80 (doubled midranks; null mean 130.0), count(W* <= W_obs) = 27,260 of
  1,000,000, p = 0.0273 -> by the frozen rule "supported at 0.05". Published mean E_100 = 9.70 vs
  extras (48) 11.65, skipped (11) 12.64, extras99 (30) 12.37 (24 odd primes <= 100).
- PC1 deterministic planted: p = 1.0e-6 (minimum attainable), PASS.
- PC2 (measured, 1000 datasets each): rejection rate q=0: 0.045 (inside [0.036, 0.064], PASS);
  q=0.05: 0.089; q=0.10: 0.150; q=0.20: 0.355; q=0.35: 0.734. No grid q reached power 0.8, so no
  q is stated as "detectable"; the test is weak for q <= 0.2.
- S1 (C99 pool, 88): p = 0.0154. S2 (all 69, in-sample): p = 0.0142.
- S3 sensitivity (measured): L=50: p = 0.474 (no effect); L=100: 0.0273; L=200: 0.0616. The result
  depends on the fixed L.
- S4: in-sample q-hat = 0.33 (LR vs q=0: 3.86); power of the primary test at q-hat: 0.711.
Interpretation limits: one marginal p-value, the published curves are the discovery set (test is
partly out of sample), result is not stable across L, and q-hat = 0.33 per Elkies prime would make
nearly every real SEA run fail, which is implausible for a working program. Not a claim about the
historical rule.
