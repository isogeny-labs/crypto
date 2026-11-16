# Positive control, linear algebra: results (2026-11-16)

Status: every number is `[pcc:unverified]` (one local run, no independent
validator, so no leaderboard or `benchmarks/README.md` row). Protocol frozen
before data (`frozen.sha256`, commit `caae5eca9f`); test binary is that
commit's build (`binary.sha256`). Training: `TRAINING.md`. Test data
`run.json`, log `run.log` (`all_certified=true`, 1007 s), tables `tables.md`,
figure `figure.svg`, report `REPORT.pdf`.

## Gates

| gate | result |
|---|---|
| G1: M reproduces the Lucas study's M rows (group ops, relation and LA multiplications, trials, relations) | **pass** |
| G2: N and S have M's trials, relations and relation multiplications on every run | **pass** |
| G3: every log certified (M, N, S, R; 696 runs) | **pass** |
| unit tests (coordinate solve on random systems; all LA modes certify on real instances) | pass |

## Component: the linear algebra, paired on identical relations

| p | M LA mult. | N/M [95%] | S/M [95%] | S/M incl. additions [95%] | fallbacks (N, S) |
|--:|--:|--|--|--|--|
| 4099 | 2.67e6 | 0.669 [0.647, 0.694] | 0.323 [0.307, 0.337] | 0.405 [0.388, 0.423] | 0, 0 |
| 16411 | 1.80e7 | 0.667 [0.661, 0.674] | 0.326 [0.323, 0.329] | 0.394 [0.390, 0.399] | 0, 0 |
| 32771 | 4.63e7 | 0.667 [0.662, 0.673] | 0.328 [0.325, 0.331] | 0.388 [0.383, 0.392] | 0, 0 |
| 65537 | 1.20e8 | 0.667 [0.664, 0.670] | 0.329 [0.327, 0.330] | 0.453 [0.451, 0.455] | 0, 0 |

- N (one coordinate, read from the Krylov sequence; no reconstruction) is
  0.67× of the baseline linear algebra at every prime.
- S (the same on a random square row subset: no `AᵀA`, no `D`) is
  0.30–0.33× at every prime, 0.39–0.55× counting additions too. The
  decision rule (S/M ≤ 0.6 at every p ≥ 4099) holds: **the component
  advances**.
- No coordinate answer failed the group check (0 fallbacks in 348 N and S
  runs), so the square subset was nonsingular every time.

## End to end: distance to the matched negation rho

Arm total ÷ R total, two-stage bootstrap 95%:

| p | M w=0.1 | S w=0.1 | M w=1 | S w=1 |
|--:|--|--|--|--|
| 4099 | 5.10 | 2.77 [1.35, 8.20] | 26.8 | 14.0 [6.8, 41.6] |
| 16411 | 3.69 | **1.97** [1.15, 3.96] | 19.5 | **10.1** [5.9, 20.3] |
| 32771 | 3.69 | **1.92** [1.08, 3.71] | 19.6 | **9.85** [5.55, 19.0] |
| 65537 | 3.97 | 2.57 [1.70, 4.14] | 21.3 | 13.6 [9.0, 21.9] |

- Against the original positive control's arm C (2026-11-12) at
  p = 32771, the two ports together take the distance at w = 0.1 from 7.44 to
  1.92 (3.9× closer) and at w = 1 from 40.2 to 9.85 (4.1× closer).
- **§5a verdict: still unresolved.** S is below rho at w = 0 from p = 257 on
  and above it at w = 0.1 and w = 1 at every prime. At p = 16411 and 32771
  the w = 0.1 interval's lower end is 1.15 and 1.08: close to, not below, 1.
- The linear algebra is now 0.26–0.49 of S's native multiplications; the
  relation phase is the larger share again.

## Training (TRAINING.md)

On disjoint curves, re-tuning the factor-base constant did not help: the
score (geometric mean of S/R at w = 0.1) was 4.07, 3.27, 2.88 and 2.84 for
cc = 0.7, 1.0, 1.4 and 2.0. cc* = 2.0, so S* = S and was not run separately.

## Scorecard of the frozen predictions

| id | prediction | outcome |
|---|---|---|
| P1 | S/M LA in [0.25, 0.45], N/M in [0.55, 0.80] at every p ≥ 4099 | **held** (0.32–0.33; 0.67) |
| P2 | fallbacks ≤ 5% | **held** (0%) |
| P3 | S/R at w = 0.1, p = 32771, in [1.8, 3.0] | **held** (1.92) |
| P4 | cc* < 2 | **failed** (cc* = 2.0): with cheap LA, a smaller base costs more relation work than it saves |
| P5 | verdict still unresolved | **held** |

## Where the rest of the cost is, and next levers

Two diagnostic facts (training curves only, not part of this verdict):

1. The dense column of random `T`-coefficients now costs about N
   multiplications per Krylov step. A walk that starts at `a G + T` and jumps
   by multiples of `G` only gives every relation the `T`-coefficient `−h`
   (`WalkMode::UnitT`, implemented and tested here, unit test
   `unit_t_walk_certifies_and_removes_the_dense_column`). Smoke on two
   training curves: LA 0.73× of S, total 0.92–0.94× at w = 0.1.
   Berlekamp–Massey (about 2N²) is then most of what is left in the LA.
2. At p = 521, 2053, 8209 and 65537 (where `3 | n/3`) root finding falls back
   to Cantor–Zassenhaus, and S/R at w = 0.1 is 2.6–5.6 against 1.9–2.8 at the
   neighbouring primes. A cube root for that case (Adleman–Manders–Miller)
   is the largest relation-phase lever left.

Both go into a new preregistered study, with BM (half-gcd or a smaller N via
structured elimination) after them.
