# P2: where does the degree-2 base stop beating the lab's degree-1 base?

Run 2026-11-17. Binary g16d, the same as P1. All numbers are `[pcc:unverified]`.

## New cells

These extend P1 with the same arms, units and analysis (`../P1/analyze.py`).

| g | p | pairs | lab `c` / rho | D / rho | E / rho | best over `c` | certified |
|---|---|---|---|---|---|---|---|
| 7 | 101 | 4 (2 curves) | 0.195 | **0.0237** | 0.0268 | 8.2x (D) | all |
| 8 | 53 | 2 (1 curve) | >=7.82 (exhausted at 2x10^7) | **0.0402** | 0.231 | >=195x (D) | all |

The 2-curve g=8 p=53 cell admitted one curve in 40 minutes. It was replaced by
a single-curve run on that curve (`crossover_supp/`), and the cell was stopped.

## Decay of the gain with p

The table gives the best degree-2 arm over the lab arm, as a geometric mean,
and the log-log slope between adjacent primes.

| g | p | gain | slope to next p |
|---|---|---|---|
| 6 | 31, 53, 101, 211 | 9.7, 4.4, 2.4, 1.29 | -1.48, -0.95, -0.84 |
| 7 | 31, 53, 101 | 108, 11.8, 8.2 | -4.13, -0.56 |
| 8 | 31, 53 | 94, >=195 | no decay yet (lower bound) |

At the smallest p, the steep slopes come from the falling-factorial penalty on
degree-1 bases, which shows up when p/2 is not much larger than g. Once p is
well above g the slope settles near -1, the value that holds at g = 6 over a
7x range of p.

## Crossover estimate p*(g)

p*(g) is where the best degree-2 arm stops beating the lab arm. Extrapolated at
slope -1 from the largest measured p:

| g | p* (slope -1) | g! |
|---|---|---|
| 6 | ~270 (measured trend reaches 1.29x at 211) | 720 |
| 7 | ~830 | 5040 |
| 8 | >~10^4 | 40320 |

The estimates grow faster than exponentially in g, roughly tracking g!/3 to
g!/6. That is the expected scaling: the degree-1 split probability is about
1/g!, while the degree-2 arms' cost grows polynomially in p.

If the trend continues, the following would hold (an extrapolation, not a
measurement):
- for g >= 10, p* ~ 10^6 or more;
- for g >= 12, p* ~ 10^8 or more.

On that reading, at genus 12-16 a degree-2 (or higher) base beats a degree-1
base at every prime where these Jacobians have practical size.

## Caveats

- **Few points.** The extrapolations rest on 2-4 points per genus, a handful of
  curves, and a slope assumed constant. At g=8 there is no measured decay at all.
- **Genus 6 has a ceiling.** At g=6 and large p, E's gain over `c` shrinks
  because E needs many more relations for its large-prime cycles. D's gain
  shrinks because its p^4 linear algebra grows. At cryptographic p, genus 6
  stays with the degree-1 base.
- **Lower bounds.** Where `c` exhausts, the speed-up is a lower bound.
- **What is new.** The structure (an optimal smoothness degree that grows with g)
  is classical (Enge-Gaudry). The measured crossover points in the lab
  instrument are the new part.
