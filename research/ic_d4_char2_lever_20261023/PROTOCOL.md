# Protocol: `frob_roots`, a lowering of the counted cost of the char-2 D4 index calculus

Frozen 2026-10-23, committed before any session data. Branch
`claude/ic-d4-char2-lever-20261023`. Baseline study:
`research/gaudry_quartic_d4_char2_20261015` (IC/rho 6.06 theory, 4.27
measured at m = 14, one target, S = 5.37).

## Question and class

Does replacing the generic polynomial product in the oracle's `F_q`-root
finding with the characteristic-2 semilinear squaring lower the end-to-end
counted `S` of `ic.pipeline` + `bin-t-line` + `gaudry-d4-bin` on the same
instances and targets, with every answer verified?

Expected class if it holds: **engineering** (S falls, ratio to the floor
flat in kind, the algorithm's trial count unchanged). The result is
reported as a distance to rho, not as a pass/fail against rho.

## What was investigated first (before choosing)

Per-trial cost at m = 10, 300 random targets (`examples/d4_char2_profile`):
charpoly 42 %, echelon 26 %, root finding 19 %, normal forms 11 %, rest 2 %.
92 % of trials have a rational solution; 0.5-0.7 % give a relation.

- Lever (a), first-dependency stop: not available. `incremental-gauss`
  already stops when the target column is pinned (4,033 relations for 4,122
  columns in the m = 14 session) and linear algebra is 1 % of S. The
  odd-characteristic `eager_la` has nothing to remove here.
- Lever (b), cheap exact rejection: not found. A trial fails because the
  rational solution's quartic does not split or an Artin-Schreier condition
  fails, which is known only after the solve; the solve's cost does not
  depend on the outcome. No pre-test is claimed.
- Lever (c), chosen: `X^q - X mod p` for the degree-64 `det A(lambda)` takes
  `m - 6` dense squarings; each was a generic product (about 64^2
  multiplications) plus reduction. In characteristic 2,
  `(sum a_i X^i)^2 = sum a_i^2 X^(2i)`, so the product needs 64 squarings and
  the reduction needs only the `X^(2i)`, `i >= 32`, tabulated once per
  modulus. Same polynomial, same roots.

## Arms (same instances, same targets, same seeds)

| arm | role | oracle |
|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` |
| `ic-base` | baseline | `gaudry-d4-bin:macaulay=11,frobenius=1,sparsest=1,replay=1,charge_unsolved=1` |
| `ic-lever` | candidate | same plus `frob_roots=1` |
| `ic-base-aa` | control (A/A) | identical to `ic-base` |

Instances: `f2m4_torsion(m, seed 1, max_cofactor 16)`; m = 12 with 3 targets
and m = 14 with 1 target (the 2026-10-15 target seed 202610150001, so the
first targets coincide with the baseline sessions). Specs:
`specs/SPEC-lever-m{12,14}.json`. Two sessions, one per core, 2 cores total.
`RAYON_NUM_THREADS=1`, `--cpus none --allow-busy`, `isolation_required: L0`:
shared VM, counted operations only, **no wall-clock claim**. Wall time of the
oracle in-process (`d4_char2_profile`, interleaved) is descriptive.

## Statistic

Paired `Sum S_lever / Sum S_base` per curve from `ecbench compare --a ic-base
--b ic-lever`, and `ic-base` against `ic-base-aa` as the control, each at
three prices for the unpriced native work (AGENTS.md 5a): 0, 0.1 and 1 GAE
per native operation, applied alike to every arm. IC arms report their
native counters; rho's unpriced counters are listed from the records.

## Predictions and stop conditions

- A/A ratio is exactly 1.000 (counts are deterministic given the seed). Any
  other value voids the session.
- Trials, relations and recovered scalars are identical between `ic-base`
  and `ic-lever` (the lever cannot change the relation set).
- Predicted `S` ratio at m = 12 in [0.85, 0.97] and at m = 14 in [0.82, 0.95]
  (roots are 19-28 % of the trial and about 3/4 of them are removed).
- Success: ratio's interval excludes 1 at all three weights, every run
  verified, `ecbench verify --replay-all` identical for every session.
  Otherwise the result is recorded as null.

## Why this is not relabelling or accounting

1. The counter increments once per executed table multiplication (`g.mul`
   call site); no counted operation was re-priced, and `charpoly`, `echelon`,
   `nf` counts are asserted equal between arms in the unit test.
2. The product `r*r` is replaced by an algorithm that performs fewer
   multiplications and returns the same polynomial (unit test: roots equal
   the generic path for degrees 2-64, m = 3-12, and solutions and
   decompositions equal over 12 solves).
3. The in-process wall time of the oracle with `frob_roots` is measured against
   the baseline on the same targets; it must fall in the same direction
   (descriptive only).
4. The trial count and relation set are unchanged, so no work moved to a
   phase the headline does not look at.
