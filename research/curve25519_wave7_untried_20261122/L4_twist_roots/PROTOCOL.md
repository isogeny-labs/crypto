# L4 protocol: mixed curve/twist summation roots (frozen before any data, 2026-11-22)

Class: exact-count mechanism measurement (unit test of idea record 6 of
`research/curve25519_autolab_20261104/ideas/STRUCTURE-R4.md`, "never run"). Not a leaderboard row, not a
speed claim, not an improvement claim. Expected effect on the index-calculus baseline: ratio 1.00
(no change); the study decides whether twist x-coordinates can enter a factor base at all. Every result
sentence written from this protocol carries [pcc:unverified]. Phase 1 status: design only, no data taken;
`smoke/` is "smoke, not data".

## 1. Question

For the Montgomery curve `y^2 = x^3 + A x^2 + x` over F_p, does the summation polynomial S3(x1, x2, X)
have F_p roots for pairs with one x on E and one on the twist, and where do the F_p roots of the
(E,E) and (twist,twist) pairs lie?

## 2. Mechanism (stated before data; derivation is the thing under test)

Notation: f(x) = x^3 + A x^2 + x; class of x in F_p: **E** (f(x) a nonzero square), **T** (f(x) a
non-square, x on the twist only), **Z** (f(x) = 0: x = 0 and the roots of x^2 + A x + 1; 2-torsion,
on both curves).

- Standard S3 (roots x(P1+P2), x(P1-P2)): `a X^2 - 2 B2 X + c`, `a = (x1-x2)^2`, `c = (x1 x2 - 1)^2`,
  `B2 = (x1+x2)(x1 x2+1) + 2A x1 x2`. **Erratum to the task text:** the task text writes the coefficients
  `a` and `c` the other way round (`(x1 x2-1)^2 X^2 ... + (x1-x2)^2`). That polynomial is the reverse of
  the standard one; its roots are 1/x(P1+-P2) = x(P1+-P2+T0), T0 = (0,0). Both are run: **"std" is the
  primary instrument**, "given" is a secondary cell; their classification is identical because
  f(1/x) = f(x)/x^4. The standard form is verified by explicit point arithmetic (control C1).
- Discriminant: `B2^2 - a c = 4 f(x1) f(x2)` (checked numerically in smoke; to be re-checked by C4).
  So S3 has F_p roots iff f(x1) f(x2) is a square (or zero): same class gives two F_p roots, mixed
  class gives none, except when `a = 0` (x1 = x2, so same class) or an x is in Z.
- Mixed pair, P1 in E(F_p), P2 in the twist (pi(P2) = -P2 over F_p^2): pi(P1+P2) = P1-P2, so the
  two roots are Frobenius conjugates, rational only if x(P1+P2) = x(P1-P2), i.e. 2*P1 = 0 or 2*P2 = 0
  (x1 or x2 in Z).
- Z pairs: x2 = t in Z (T = (t,0)): P1+T = P1-T, double root x(P1+T) = phi_t(x1) =
  f(x1)/(x1-t)^2 - A - x1 - t, on the class of x1.
- Same-class pairs: roots lie on the pair's own curve or in Z, and they land in Z exactly when
  x2 = phi_t(x1) for some t in Z (P1 +- P2 = T).
- General statement (not measured here, for S_m with m > 3): a rational x-only relation sum +-P_i = O
  mixing E and twist points forces the E-part sum R in E(F_p) to equal minus the twist-part sum, so
  pi(R) = R and pi(R) = -R, hence 2R = 0: rational mixed relations exist only through E[2].

## 3. Instrument (exact commands; Python 3 stdlib + numpy, one core)

All scripts are in this directory; `run_all.sh` runs them in order into `data/`.

    python3 controls.py --k {16,20,24} --n {20000,20000,5000} --n3 1000000 --tag main --out data --predict
    python3 run_l4.py exhaustive --k 16 --tag main --out data
    python3 run_l4.py sample --k 20 --n 200000000 --tag main --out data
    python3 run_l4.py sample --k {16,20} --variant given --n 50000000 --tag main --out data
    python3 run_l4.py sample --k 24 --n 50000000 --tag main --out data
    python3 analyze_l4.py --dir data --tag main --ks 16,20,24

`l4core.roots` computes, for arrays of pairs, the exact distinct F_p roots of S3 (discriminant, Legendre
table, square-root table, inverse table, all integer arithmetic mod p, no floats) and the class of
each root by the Legendre symbol of f(X). Degenerate leading coefficient (`a = 0`) is solved as the
linear equation; the second root is X = infinity and is not counted.

## 4. Cells and seeds

Curve facts (p, A, #E = 8r, twist order 4r', primality of r and r', counts of x in each class, Z
set, class of x = +-1, whether A^2 - 4 is a square) are recomputed by `Curve.facts()` in every run
and stored in the JSON; the values in the task text are claims until then.

| cell | curve | pairs | design |
|:--|:--|:--|:--|
| M16 | k = 16: p = 65437, A = 1058, #E = 65672 = 8 * 8209 | all 2,141,033,203 unordered pairs x1 <= x2 (x in F_p) | exhaustive, no sampling |
| M20 | k = 20: p = 1048573, A = 926, #E = 8 * 131203 | 2e8 pairs each of EE, ET, TT (ordered, x drawn uniformly from the class list, with replacement); plus every (z, x) with z in Z, x in F_p exhaustively | seeded sample |
| M24 (supplementary) | k = 24 faithful member: p = 16777141, A = 7842, #E = 16784072 = 8 * 2098009 (verified in Phase 1: 2098009 prime; twist 4 * 4192553) | 5e7 pairs per stratum, Z strata exhaustive | seeded sample |
| G16, G20 (secondary) | same curves, "given" (task-text) polynomial | 5e7 per stratum | seeded sample |

Seed rule: `numpy.random.default_rng(int(sha256("L4|k|tag|stratum|chunk")[:8]))`, chunk of 2e6 pairs;
controls use `"L4ctl|k|control|stratum|tag"`. Tag = `main` for the confirmatory run. No other seeds,
no re-draws.

Counted unit: pairs (x1, x2) classified by (pair class, number of distinct F_p roots 0/1/2, class
of each root, `a = 0` flag). Counts are exact; for M16 they are the complete population, for M20/M24
samples the estimator is the sample fraction, and a zero count is bounded by the 95% upper limit 3/N
(rule of three; Clopper-Pearson upper limit 1 - 0.05^(1/N), reported).

## 5. Predictions (numeric, written before data; a failed prediction stays failed)

Population sizes: M16 has |E| = 32835, |T| = 32601, |Z| = 1 (x = 0 only; A^2 - 4 is a non-square so
there is no further rational 2-torsion), so N_ET = 1,070,453,835 pairs. M20: |E| = 524811, |T| = 523761,
|Z| = 1 (all from the Phase 1 table builder, to be re-computed).

- **P1 (the claim).** (E,T) pairs with any F_p root: **0 of 1,070,453,835** at M16; **0 of 2e8** at M20
  (95% upper limit 1.5e-8 per pair); 0 at M24, G16, G20. Exceptional-pair list: empty. The exceptional
  set outside Z is predicted empty: x = +-1 are not special (x = 1 is class E and x = -1 is class T at
  k = 16, and neither is 2-torsion), x1 x2 = 1 gives the same class (f(1/x) = f(x)/x^4), and x1 = x2 is
  same-class.
- **P2 (Z pairs).** Every (Z,E) pair with x1 != x2 (x2 = 0) has exactly 1 distinct root, on E; every
  (Z,T) pair exactly 1 root, on the twist. 100% (exhaustive in M16 and in the Z strata of M20, M24).
- **P3 (non-degenerate same-class pairs).** Every (E,E) and (T,T) pair with x1 != x2 and no Z has exactly
  2 distinct F_p roots: 100%, i.e. zero pairs with 0 or 1 roots (counted exactly at M16).
- **P4 (root curve).** Roots of (E,E) pairs on the twist: 0. Roots of (T,T) pairs on E: 0. (All roots are
  on the pair's own curve or in Z.)
- **P5 (the Z-rooted same-class pairs are exactly the 2-torsion translates).** The number of non-diagonal
  (E,E) pairs with a root in Z is 16417 and of (T,T) pairs 16300 at M16 (predictor `predict_Z_pairs`,
  output of the Phase 1 smoke run, to be recomputed), 0 for (E,T). At M20 (sampled) the observed counts
  are within 4 standard deviations of the Poisson expectation from the predicted 262405 (EE) and 261880
  (TT) unordered pairs.
- **P6 (diagonal).** x1 = x2: exactly one finite root x(2P), on the point's own curve (E for EE, T for TT,
  or Z), the other root being infinity; 100%.
- **P7a (positive control C1).** For 20000 sampled (E,E) and 20000 (T,T) pairs (x1 != x2) the F_p root set
  of S3 equals {x(P1+P2), x(P1-P2)} computed by the explicit affine chord formula (B = 1 for E, B = the
  smallest non-residue for the twist): 100% match, 0 mismatches.
- **P7b (positive control C2, mechanism).** For 20000 sampled (E,T) pairs computed over F_p^2 (P2 with
  y2 = w s, s^2 = non-residue): both x(P1+P2), x(P1-P2) satisfy S3, are conjugate and are outside F_p: 100%.
- **P8 (negative and sensitivity control C3).** The same root counter with a wrong constant A' = A + 1
  on 1e6 sampled pairs per class: F_p-root fraction in [0.47, 0.53] for each of ET, EE, TT (random
  quadratics are solvable about half the time); with the true A the fractions are 0 (ET) and 1 (EE, TT)
  exactly. This shows the counter returns mixed-class F_p roots when the structure is absent, at a rate
  (0.5) far above any rate that would matter (a factor-base use needs a mixed rate of the order of the
  same-class rate, 1), and that the sample bound 1.5e-8 is far below the 0.5 it can see.
- **P9 (identities, C4).** disc/4 = 4 f(x1) f(x2) for 4e5 random pairs (exact); roots of the "given"
  polynomial are the inverses of the roots of the standard one (20000 pairs, 100%).
- **P10 (curve facts).** #E = 8r and twist order 4r' with r, r' prime for k = 16, 20, 24 as listed;
  #E + #twist = 2p + 2. Already satisfied in Phase 1 for k = 16 (65672 = 8 * 8209, 65204 = 4 * 16301),
  k = 20 and k = 24; recomputed in the run.

## 6. Failure and what happens

- A prediction fails if its observed count differs from the stated one at all (P1-P4, P6-P7, P9, P10 are
  exact statements) or leaves the stated band (P5 by 4 sigma, P8 by [0.47, 0.53]). The verdict is written
  as FAIL in `results_main.md` with the exceptional pairs, and stays.
- If P1 fails at M16 the exceptional pairs are all listed (cap 20000, counts exact) and classified by
  tag: x = 0, Z, x = +-1, x1 x2 = 1, discriminant 0, other. An "other" pair would contradict the
  derivation of Section 2 and is escalated, not explained away.
- If any control fails (P7-P9) every cell that depends on that instrument is invalid.
- A pass of P1 is a statement about S3 on these curves, not about hyperelliptic or higher-degree summation
  polynomials (the general statement in Section 2 is argued, not measured).

## 7. What the outcome implies for a factor base (stated before data)

If P1-P4 hold: the F_p x-line splits into two classes that never mix in S3 relations (apart from Z).
A relation S3(x_Q, x1, x2) = 0 with the target x_Q in class E can use only E-class factor-base points
(for an (E,T) pair the roots are non-rational, and for a (T,T) pair the roots lie on the twist or in
Z). Twist points therefore (a) add no relations for a target on E, (b) are only usable for the twist DLP,
a group of order 4r' with r' != r, which carries no information about E, and (c) the only mixed
rational relations run through 2-torsion translation (P5), which leaves <G> as in the torsion leads. A
factor base built from twist x-coordinates would lower the density of usable columns, not raise it; so
the lever "twist" has ratio 1.00 for the baseline, matching the entry in STRUCTURE-R4.md section 3. This
is a derivation plus count, not an improvement; it is also not a distance to rho, which is imported
from earlier studies and not stated here.

## 8. Cost estimate (Phase 1, measured on this host)

Throughput 6.5e6 pairs/s single core (smoke: 400 rows of k = 16, 2.6e7 pairs, 4.1 s).

| step | pairs | CPU est. |
|:--|--:|--:|
| M16 exhaustive | 2.14e9 | about 330 s |
| M20 sample | 6.0e8 + 3 * 1e6 | about 100 s |
| G16, G20 | 3 * 5e7 each | about 50 s |
| M24 (tables 12 s) | 1.5e8 | about 40 s |
| controls (k = 16, 20, 24; the k = 24 Z predictor loops 1.7e7 x) | | about 5 min |
| **total** | | **about 15 min, one core** |

Memory: under 1 GB (largest: M24 tables 0.4 GB plus a 2e6-pair chunk about 0.4 GB; M16 and M20 are
smaller). Output files: under 1 MB each (exception lists capped at 20000 records).

## 9. Sources checked (repository files actually read; the knowledge base and lead atlas are unreachable)

- `research/curve25519_wave7_untried_20261122/BRIEF.md` (task rules).
- `research/curve25519_autolab_20261104/ideas/STRUCTURE-R4.md`, lines 14, 112-150, 296-310 (the table
  of curve facts, the "quadratic twist (shared x-line)" row of the lever table, the idea list).
- `research/curve25519_frame_search_20261103/README.md` (head, 40 lines), `PROTOCOL.md` (grep for the
  family rule) and `scripts/run_confirm.sh` (k = 16: p = 65437, A = 1058).
- `curve25519_family-5abc70ba family` output (k = 20, 24, 28, 32 family curves; k = 24 there is
  p = 16777213, A = 318, a different member than the brief's p = 16777141, A = 7842; M24 uses the
  brief's).
- Not read: the other studies listed in the brief; no claim of novelty is made against them.

## 10. Files

`PROTOCOL.md`, `l4core.py` (instrument), `run_l4.py` (exhaustive and sample runs), `controls.py` (C1-C5),
`analyze_l4.py` (P1-P9 verdicts, results tables), `run_all.sh` (the Phase 2 launcher, not executed),
`smoke/` (smoke, not data). Phase 2 additionally needs the repository's launcher rules (conductor task,
resume checkpoint) from the main session before `run_all.sh`.
