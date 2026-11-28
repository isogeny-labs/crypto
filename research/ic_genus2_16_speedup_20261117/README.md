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

## Status (2026-11-20)

| Phase | State | Where |
|---|---|---|
| P0 lab instrument recovered | done; exact count identity at g=4 | `P0/` |
| P1 genus 6-10 instrument, arms D (degree-2 base) and E (degree-2 large primes) | done. **Validated-with-errata** by an independent validator: tables re-derived, 32/32 group orders checked, and 236/236 discrete-log certificates checked with independent code | `P1/RESULTS.md`, `P1/ERRATA.md`, `VALIDATION_P1P2.md` |
| P2 lever F (partial degree-2 base) | pilot, negative: no interior optimum (solves flag-only) | `P2/RESULTS_F.md` |
| P2 crossover p*(g) | about 270 at g=6; about 830 at g=7 (extrapolated); at g=8 not pinned, at least about 2300 under slope -1. The earlier g!-scaling is withdrawn | `P2/RESULTS_crossover.md` (erratum section) |
| g=8 decay cells, p=73 and p=101 | done: gain 24x and 22.6x over the lab arm | `P4_followup/g8decay/` |
| P3 non-hyperelliptic, e=3 plane models (L08 v2) | done: relation phase plus LA bracket, not a solve | `P3_nonhyp/RESULTS.md` |

Headline, in counted cost against expected matched rho (all `[pcc:unverified]`):
- **g = 7-8.** A degree-2 base lets index calculus solve where the lab's degree-1 2LP base cannot.
  From p = 31 it puts IC well below rho (0.0075-0.21 of rho), with a gain over the lab arm of
  12x to more than 1000x.
- **g = 10, p = 11.** The degree-2 base sits **at** rho (1.03, all 3 curves; erratum E3b), and the
  lab arm cannot solve at all.
- **g = 6.** The gain decays as about 1/p, from 9.7x at p=31 to a tie at p=211 under the §5a band
  (E2).
- **Non-hyperelliptic plane models.** The two-node quintic model of a genus-4 curve sits 30-300x
  below matched rho at p=8209 across the whole LA bracket, with a slope of about 1.21 against
  rho's 2. This is a relation-phase bracket, not a solve.
