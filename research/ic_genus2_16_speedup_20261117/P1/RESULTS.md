# P1 results: degree-2 places against the lab baseline, genus 6-10

Run 2026-11-17 under `PROTOCOL.md` (frozen, `frozen.sha256`). Binary g16d (sha256
prefix `6cf78101b1370e2c`) = `fffa1b1243` + `instrument.patch`. All numbers are
`[pcc:unverified]`: one container, one producer, no independent replay.

## Table (geometric means over (curve, seed) pairs, `analyze.py main/*.jsonl supp/*.jsonl`)

Ratios are counted cost (FPM_1 = F_p multiplications + 10 x inversions + native
relation work + LA products) over expected-step matched negation-map rho on the
same instance. Below 1 means cheaper than rho. `>=` marks a lower bound: the lab
arm `c` exhausted its 5x10^6 trial cap without solving.

| g | p | pairs | lab `c` / rho | D / rho | E / rho | D / rho (LA at g^2) | speed-up of best arm over `c` |
|---|---|---|---|---|---|---|---|
| 6 | 31 | 6 | 4.14 | 0.448 | **0.425** | 1.33 | 9.7x (E) |
| 6 | 53 | 4 | 0.586 | 0.186 | **0.133** | 0.899 | 4.4x (E) |
| 6 | 101 | 6 | 0.171 | 0.154 | **0.0716** | 1.62 | 2.4x (E) |
| 6 | 211 | 4 | 0.0309 | 0.138 | **0.0240** | 2.94 | 1.29x (E) |
| 7 | 17 | 6 | >=1190 | **1.19** | 3.43 | 1.55 | >=1000x (D) |
| 7 | 31 | 6 | 23.1 | **0.214** | 0.384 | 0.384 | 108x (D) |
| 7 | 53 | 6 | 0.754 | **0.0641** | 0.0785 | 0.213 | 11.8x (D) |
| 8 | 11 | 6 | >=710 | **1.03** | 9.85 | 1.11 | >=691x (D) |
| 8 | 17 | 6 | >=218 | **0.444** | 2.99 | 0.529 | >=490x (D) |
| 8 | 31 | 2 | 6.26 | **0.0664** | 0.151 | 0.107 | 94x (D) |
| 10 | 11 | 4 | >=146 | **0.725** | 37.2 | 0.744 | >=201x (D) |

Every solve in the table is certified (`[t]G = T` on a planted log).

## Predictions

- **P-a (D and E cheaper than `c` everywhere): mixed.**
  - E is cheaper than `c` on every pair at g = 6, 7 and 8 (smallest pair ratio
    1.08, at g=6 p=211).
  - D is cheaper everywhere except g = 6 at p >= 101. There it loses on some pairs
    at p=101 (min 0.56) and on all pairs at p=211 (0.18-0.27), because its base is
    about p^2/4 columns.
  - At g=10 p=11, E is worse than D by 50x and loses to `c`'s lower bound on one
    pair (0.88).
- **P-b (at g=6 the gain falls as p grows): holds.** The best arm over `c`
  goes 9.7x -> 4.4x -> 2.4x -> 1.29x for p = 31, 53, 101, 211, roughly as p^-1 (a two-point endpoint slope of -1.05, from p=31 to p=211).
  The extrapolated crossover for E is near p ~ 270 at g = 6. Past it, the lab's
  degree-1 base is the better choice at genus 6.
- **P-c (g=6: E cheaper than D with LA at g^2, p >= 53): holds**, by 6x at
  p=53 and 117x at p=211.
- **P-d (g >= 8: D cheaper than E): holds** at every g >= 8 cell, by 2.3-51x.

## Mechanism (`analyze` side computation, per arm)

- At g=6 the lab arm needs about 900-1700 walk steps per relation, about 900 at p=211 (the 1/g!
  split probability plus the falling-factorial penalty at small p). D needs about
  10 and E about 13.
- E pays in relation count instead. Its degree-2 large primes live in a graph of
  about p^2/4 vertices, so it needs 10-30x more relations than `c` before enough
  cycles close. That is why E's edge decays with p, and why E collapses when
  almost every place is a degree-2 large prime (g=10 p=11).
- D's cost moves into linear algebra as p grows. Its LA share is 6% (p=31), 11%,
  27% (p=101), and dominates at p=211 once an LA product is priced as a g^2
  multi-word product.

## Distance to rho (the success criterion's reading)

- At **g=6**, the lab arm is already below rho from p=53 up (0.59x, 0.17x,
  0.031x). The best degree-2 arm moves that to 0.13x, 0.072x and 0.024x.
- At **g=7 and g=8**, the lab arm is above rho at every p measured except g=7
  p=53 (0.75x), and is often unable to solve at all. The degree-2 base puts index
  calculus below rho from p=31 (g=7: 0.21x, 0.064x; g=8: 0.44x at p=17, 0.066x at
  p=31). At the smallest p it is about at rho (1.0-1.2x).
- At **g=10, p=11**, D solves at 0.73x of rho where the lab arm cannot solve.

## What this does not show

- **Cryptographic p.** Order verification needs #J < 2^63, so p is small at
  large g. The g=6 trend shows the gain over a degree-1 base disappears as p
  grows. At g=7 and 8 the gain is large on every p measured, but the same decay
  must be expected; it is not measured here.
- **Wall time.** Counted operations only. D's dense column table and E's
  large-prime bookkeeping are charged as table set-up multiplications only. The DSU operations
  (counted in `la_add`) and all additions (`rel_add`, `la_add`) are recorded in the raw data but
  NOT included in FPM_1 (erratum E1, validator 2026-11-20).
  Memory is not.
- **Standing.** No independent replay, no pcc claim, no benchmark record. The
  lab's `tools/benchrec.py` and `scripts/checkpoint.sh` are not on this branch's
  base. The method (degree-2 places in a hyperelliptic base) is classical
  (Enge-Gaudry style smoothness bases), so what is new here is the measurement
  in the lab instrument against the lab baseline and matched rho, not the idea.

## Deviations from the protocol

1. **g=6 p=53:** 2 curves were admitted within 120 seeds, not 3, giving 4 pairs.
2. **g=6 p=211, g=8 p=31, g=10 p=11:** the 3-curve cell hit the 1700 s limit.
   - Rerun as single-curve invocations (`run_curve.sh`, `supp/`) on the curve seeds
     the timed-out cell had admitted, with a 3600 s limit.
   - The protocol's g=10 clause allowed "not run"; running was better.
   - g=8 p=31 has 1 curve (2 pairs). g=10 p=13 admitted no curve in 1700 s: not run.
   - g=10 p=11: the 3-curve cell admitted curves 7, 12 and 27. Only 7 and 12 were rerun, so
     the row has 4 pairs. Seed 27 was never run (erratum E3).
3. **Pilot binaries** (g16, g16b, g16c) differ from g16d only in curve-admission
   logic. Pilot data is kept in `pilot*/` and not used in the table.

## Validation (2026-11-20)

An independent validator subagent, using its own code, checked this file against the committed
raw data (`../VALIDATION_P1P2.md`; scripts in `../validation/`). Verdict: **validated-with-errata**.

What it checked:
- every table value re-derived to within 0.38%;
- all 32 group orders checked against PARI `hyperellcharpoly` and an independent implementation;
- a determinism replay of 3 cells (2457 fields identical);
- a Monte Carlo split probability matching the trials-per-relation ratios.

What it could not check: the discrete-log certificates. The run records carry only
`certified`/`ok` flags, not G, T or the recovered t. Until they do, no row here is admissible to
the leaderboard or a `VALIDATION.json`.

Errata, folded in above or noted here:
- **E1:** additions and DSU work are not in FPM_1.
- **E2, the §5a band.** The validator priced the unpriced operations on every arm, rho included,
  at 0, 0.1 and 1 per operation. Every arm-versus-rho ordering holds across the band. One ranking
  does not: at g=6 p=211 the best arm's gain over `c` is 1.29x, 1.24x and 0.995x. That cell is
  therefore a tie with the lab arm, not a gain.
- **E3:** g=10 p=11 seed 27 was not run.
- **E4:** see `DESIGN.md`. In 14 of 32 instances `order` is an exponent multiple; r is prime and
  exactly divides #J in all 32.
- **E5:** the range of walk steps per relation now includes p=211.
- **E6:** the slopes are two-point endpoint slopes.
- **E7:** the P2 crossover cells used a 2e7 trial cap against 5e6 here. This is immaterial at
  g=7 p=101.
