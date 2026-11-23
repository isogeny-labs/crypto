# Degree-1024 auxiliary kernel correctness fixtures

This directory contains an isolated, Rust-standard-library-only correctness fixture generator derived from the previously supplied auxiliary-curve verifier. It is not a new relation solver or a benchmark framework. Its output is intended for integration into the existing Prime31 experiment lane.

## Fixed data

All arithmetic is in `F_p`, `p = 2147483647`.

- Ordinary target `E`: `y^2 = x^3 + 847514846 x + 2087444724`.
- Degree-three isogenous target `E1`: `y^2 = x^3 + 323891735 x + 1985248359`.
- Auxiliary `Aminus`: `y^2 = x^3 - x`, `S = (4,639874715)` of exact order `2^30`.
- `K = [2^20]S = (1724938470,923814573)` of order 1024.
- `U = (0,0)`, independent of the order-two point `[512]K = (1,0)`.
- `H0 = <K>`, `H1 = <K+U>`, `H2 = <2K,U>`. All three have order 1024.
- The three quotient chains share their first nine degree-two maps, with kernel `<2K>`. Their final degree-two kernels are the images of `K`, `K+U`, and `U`, respectively.
- Baseline `Aplus`: `y^2 = x^3+x`, `S = (3,615668188)` of exact order `2^31`, and `H = <[2^21]S>`.

Each fixture uses precisely the coset `S+H`, preserving the previously specified source point.

## Executed focused checks

```sh
rustc --edition=2021 -O compare_kernels.rs -o compare_kernels
./compare_kernels --unit-fixtures
```

The command checks these fixed arithmetic fixtures:

1. Every kernel has 1024 distinct points, each mapped to infinity by its chain.
2. The full-kernel Vélu codomain agrees with the ten-step codomain.
3. The source-point image is on the codomain and is not two-torsion.
4. Every point of `S+H` maps to that same image and all 1024 x coordinates are distinct.
5. The expanded numerator has degree 1024, and the denominator has degree 1023.
6. The membership polynomial equals the exact product over the 1024 coset x coordinates, coefficient by coefficient; denominators do not vanish on those roots.
7. Affine degree-two, projective degree-two, projective degree-four, and dense degree-1024 evaluation agree on every designated x root, off-coset probes, and all kernel x coordinates, including poles.
8. Lift counts refer to x coordinates for which the specified target right-hand side is a nonzero square. One signed representative is a column; including both signs doubles the point count but does not create two independent log variables.

| Auxiliary kernel | E accepted x | E1 accepted x | Coset x count |
|---|---:|---:|---:|
| Aminus H0 | 508 | 508 | 1024 |
| Aminus H1 | 512 | 519 | 1024 |
| Aminus H2 | 515 | 533 | 1024 |
| Aplus cyclic | 518 | 506 | 1024 |

These are fixed correctness-fixture cardinalities. No timing, relation-finding yield, matrix rank, full DLP recovery, statistical advantage, or superiority of one branch was measured here.

## Files

- `compare_kernels.rs`: reproducible arithmetic source.
- `kernel_correctness_fixtures.json`: complete maps, root sets, exact expanded polynomials, and check metadata.
- `kernel_correctness_fixtures.csv`: the four fixed cardinality rows.
- `unit_fixture_output.txt`: captured verification output.

The source additionally prepares `--census`, using sixteen fixed odd source scalars shared between branches. **That mode has not been run.** It is an experiment and must pass the repository checkpoint, publication, budget, and execution-environment gates before submission. It writes separate `kernel_census.json` and `kernel_census.csv` files and does not overwrite the fixed correctness fixtures.

## Map-evaluation comparison precautions

The degree-two evaluator carries projective x coordinates through all ten maps and performs one final inversion. The degree-four evaluator expands pairs into degree-four rational maps, evaluates them projectively using stack-resident monomials, and also performs one final inversion. Comparing against a degree-two implementation that performs ten inversions would give an unfair blocking comparison.

Even an admitted map-evaluation timing would measure only that component. The reason to test degree-four equations is the tradeoff in the polynomial solver: five degree-four constraints and four intermediate variables, versus ten degree-two constraints and nine variables. The arithmetic fixture checks do not evaluate that tradeoff.
