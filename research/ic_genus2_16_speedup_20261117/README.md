# Index-calculus speed-up for genus 2-16 (hyperelliptic and non-hyperelliptic)

Started 2026-11-17. Standing of every number: `[pcc:unverified]` until validated.

Phase 1 (this commit): deep research. Four parallel research notes are written to
`research/` as they progress:
- `A_literature_hyperelliptic.md`: algorithms, exponents, constants, records for g = 2..16.
- `B_lab_baselines.md`: what the lab already measured and built, per genus.
- `C_engineering_levers.md`: arithmetic, smoothness, linear-algebra and parallel levers.
- `D_nonhyperelliptic.md`: plane curves, C_ab, Diem/Heß relation generation, isogeny routes.

Phase 2: plan (`PLAN.md`), synthesised from the four notes.
Phase 3: build and measure against the lab baseline and matched rho.

## Status (2026-11-17)

| Phase | State | Where |
|---|---|---|
| P0 lab instrument recovered | done; exact count identity at g=4 | `P0/` |
| P1 genus 6-10 instrument, arms D (degree-2 base) and E (degree-2 large primes) | done; 11 cells, all solves certified | `P1/RESULTS.md` |
| P2 lever F (partial degree-2 base) | pilot, negative: no interior optimum | `P2/RESULTS_F.md` |
| P2 crossover p*(g) at g=7 and g=8 | running | `P2/crossover/` |
| P3 non-hyperelliptic small-e models | not started | `PLAN.md` |

Headline (P1, counted cost against expected matched rho):
- **g = 7-10:** a degree-2 base makes index calculus solve where the lab's
  degree-1 2LP base cannot, and puts it below rho from p = 31. The gain over
  the lab arm is 12x to more than 1000x.
- **g = 6:** the gain decays as about 1/p, from 9.7x at p=31 to 1.29x at p=211.
