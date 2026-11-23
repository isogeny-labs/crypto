# Structure assessment for prime-field Semaev summation polynomials

Date: 2026-11-19 UTC

The rendered run report and quantitative/structural visuals are in
[`reports/run-20261119T0515Z/`](reports/run-20261119T0515Z/). This imported
evidence remains repository-unverified `[pcc:unverified]`; see
[`IMPORT_NOTES.md`](IMPORT_NOTES.md) for provenance and admission limits.

## Outcome

The campaign retained **900,018 specialized univariate Semaev polynomials** with
**19,794,230 coefficients** over three prime fields, three nonsingular curves per
field, and arities 3 through 8. It also implemented and locally self-tested a
structure-aware root extractor through arity 20.

For point-origin inputs, the useful structure is not coefficient sparsity. It is
an explicit parametrization of every root by a signed elliptic-curve point sum.
A balanced meet-in-the-middle implementation batches all independent final
divisions into one field inversion. On the tested degree-128 workload, this path
was 479x to 2444x faster `[pcc:unverified]` than the included coefficient-only Cantor-Zassenhaus
baseline. This is a measured **root-extraction-stage** ratio under different but
explicit input representations; it is not an end-to-end index-calculus or ECDLP
speedup.

## Requirement-to-evidence status

| Requested outcome | Status | Evidence |
| --- | --- | --- |
| Generate a large amount | Producer-recorded for specialized univariates `[pcc:unverified]` | 900,018 records in the source campaign; raw payloads are omitted pending artifact-store recovery, with checksums in `results/SHA256SUMS` |
| Assess and locate structure | Locally self-tested within the stated curve/field/arity scope `[pcc:unverified]` | 900 algebraic control trials; aggregate collision, degree-drop, and coefficient statistics |
| Build a highly optimized root path | Implemented and verified prototype `[pcc:unverified]` | `roots` CLI; naive, Gray, and batched MITM implementations; 4,200 benchmark cases validated |
| Establish globally optimal or end-to-end performance | Not established | No external library comparison, assembly tuning, parallel implementation, or complete index-calculus pipeline measurement |

“Summation polynomials” is interpreted here as specialized univariates
`S_m(x_1,...,x_{m-1},X)`. Fully symbolic multivariate `S_m` expressions were not
materialized; their size grows too quickly and they are not needed by the root
path evaluated here.

## Algebraic structure

Let

```text
E/F_p: y^2 = g(x) = x^3 + A*x + B,
p > 3 prime, 4*A^3 + 27*B^2 != 0 mod p.
```

The direct third summation polynomial, viewed as a quadratic in `X`, is

```text
S3(x1,x2,X) = (x1-x2)^2 X^2
 - 2*((x1+x2)*(x1*x2+A) + 2*B) X
 + (x1*x2-A)^2 - 4*B*(x1+x2).
```

Its quadratic discriminant factors as

```text
disc_X(S3) = 16*g(x1)*g(x2).
```

Thus, when `x_i` comes with `y_i^2=g(x_i)`, the square root needed by a generic
quadratic solver is already `±4*y1*y2`. This is the degree-2 instance of the more
general signed-sum structure.

For points `P_i=(x_i,y_i)`, fix the sign of `P_1`. Away from projective degree
drops, the monic specialization is

```text
product over epsilon_2,...,epsilon_(m-1) in {+1,-1} of
  (X - x(P_1 + epsilon_2*P_2 + ... + epsilon_(m-1)*P_(m-1))).
```

Its nominal degree is `d=2^(m-2)`. Changing any selected square-root sign or
permuting the inputs only permutes the root multiset. A signed sum equal to the
identity is a root at infinity and lowers the affine degree. Two signed sums with
equal x-coordinate create a repeated root. These cases are preserved rather than
discarded.

The implementation uses this identity as follows:

```text
1. Lift each supplied x_i to one point P_i (or reuse a known y_i).
2. Split the sign-variable points into balanced left and right halves.
3. Enumerate each half-table in Gray order; one sign changes per step.
4. Form all left/right pairs.
5. Batch-invert the nondegenerate addition denominators once.
6. Compute all finite x-coordinates; separate infinity; sort/deduplicate if the
   caller wants the ordinary finite-root set.
```

Compared with recomputing each signed sum, Gray order removes a factor close to
the number of inputs. Compared with serial Gray enumeration, balanced MITM makes
the `d` final additions independent and replaces roughly `d` field inversions by
one batch inversion plus linear-many multiplications. Half-table construction
still has about `O(sqrt(d))` serial point operations. Sorting distinct roots costs
`O(d log d)` comparisons.

Primary sources:

- Igor Semaev, “Summation polynomials and the discrete logarithm problem on
  elliptic curves,” Cryptology ePrint 2004/031:
  https://eprint.iacr.org/2004/031
- David Cantor and Hans Zassenhaus, “A New Algorithm for Factoring Polynomials
  Over Finite Fields,” *Mathematics of Computation* 36 (1981):
  https://doi.org/10.1090/S0025-5718-1981-0595053-0

## Experimental scope and validation

Corpus parameters:

| Parameter | Value |
| --- | --- |
| Fields | `p = 1009, 1000003, 2147483647` |
| Curves per field | `(A,B) = (2,3), (0,7), (5,11)` |
| Arities | 3 through 8; nominal degrees 2 through 64 |
| Samples | 16,667 per arity/field/curve; 900,018 total |
| Seed | `6000285944033488934` |
| Retained data | field, curve, seed, arity, sample, input x values, full coefficient vector, infinity count, distinct-root count |
| Compiler | g++ 14.2.0, `-O3 -march=native -std=c++20` |
| Machine | 5 vCPU AMD EPYC 9V74, Linux x86-64 |

The self-test covered all nine field/curve combinations and passed:

- 900 direct `S3` discriminant identities;
- 899 direct-`S3` coefficient/root comparisons;
- 899 `S4` comparisons against an independently constructed 4x4 Sylvester
  resultant;
- 900 agreements among naive, Gray, and batched MITM root multisets;
- orientation and permutation invariance; and
- coefficient-only generic factor recovery.

One `S3` and one `S4` direct comparison were skipped by their explicit
nondegenerate/full-degree guards; they were not counted as passes. The small
field is a deliberate failure control for square-free and full-degree
assumptions.

## Corpus observations

Aggregated over the three curves in each field:

| Field | Polynomials | Nominal roots | Roots at infinity | Duplicate finite-root occurrences | Zero nonleading coefficients |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1009 | 300,006 | 6,300,126 | 6,163 | 559,330 | 14,024 |
| 1,000,003 | 300,006 | 6,300,126 | 3 | 534 | 10 |
| 2,147,483,647 | 300,006 | 6,300,126 | 0 | 0 | 0 |

At degree 64 specifically:

| Field | Samples | Samples with repeated roots | Samples with affine degree drop |
| ---: | ---: | ---: | ---: |
| 1009 | 50,001 | 42,331 (84.6603%) | 2,909 (5.8179%) |
| 1,000,003 | 50,001 | 104 (0.2080%) | 2 (0.0040%) |
| 2,147,483,647 | 50,001 | 0 observed | 0 observed |

The absence of an event in the largest-field sample is not proof that it cannot
occur. Exact curve orders for the two scan-feasible fields are retained in
`results/point_counts.csv`; these support interpreting small-field infinity and
collision events as genuine group-size effects.

No coefficient pattern survived cross-field testing as a universal shortcut:

- no even-only or palindromic sample occurred at arity 4 or above;
- only 10 zero nonleading coefficients appeared among 6,300,123 such
  coefficients at `p=1000003`; and
- none appeared at `p=2147483647`.

Small-field zeros were more frequent and correlate with algebraic degeneracies,
but they do not support a general sparse-polynomial solver. The reusable signal is
the signed-sum factorization, plus explicit handling of multiplicity and infinity.

## Matched root-stage benchmark

The benchmark used curve `(A,B)=(2,3)`, 200 cases at every degree 2 through 128,
and all three fields. Every one of the 4,200 cases was checked for equality between
the structure-derived and generic distinct finite-root sets before timing.
Structure paths were repeated 20 times; the more expensive generic batch ran
once. Reported values are aggregate mean nanoseconds per polynomial from one
process, not confidence intervals.

Degree-128 result, including sort/dedup for all paths:

| Field | Generic factoring | Naive signed sums | Gray signed sums | Batched MITM | Generic / MITM |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1009 | 5,103.09 us | 76.27 us | 16.72 us | 10.65 us | 479.1x |
| 1,000,003 | 14,877.71 us | 116.58 us | 20.79 us | 10.12 us | 1470.6x |
| 2,147,483,647 | 34,584.88 us | 215.28 us | 36.28 us | 14.15 us | 2444.5x |

Tiny-degree setup matters. Direct enumeration was fastest at degree 2. Gray was
the robust choice at degree 4. Batched MITM won from degree 8 on the two larger
fields; at `p=1009`, Gray was 1.9% faster `[pcc:unverified]` at degree 8 and MITM won from degree 16.
The CLI's `auto` rule therefore uses direct/Gray/MITM at degrees 2/4/8+, but a
production implementation should calibrate these cutoffs on its target CPU and
field.

Accounting boundaries:

- Generic timing starts with the coefficient vector already built.
- Structure timing starts with affine curve points already lifted from the input
  x-coordinates. Point sampling, square-root recovery, and factor-base setup are
  excluded. If y-values are not already available, these costs must be added.
- Polynomial materialization is excluded from both timed paths. It is unnecessary
  for structure-aware root extraction.
- The benchmark returns distinct finite roots. The corpus and `roots` CLI can
  retain multiplicity and report infinity separately.
- This is a single-threaded in-repository baseline, not a comparison against NTL,
  FLINT, Sage, or a hardware-specialized implementation.

## Practical algorithm and next optimization targets

The implemented path is directly usable:

```bash
./build/semaev_lab roots --prime 1000003 --a 2 --b 3 \
  --x 409692 --x 926728 --x 748446 --method auto --emit-coefficients
```

For a fixed production field, the next measurable optimizations are:

1. replace generic 128-bit reduction with field-specific Montgomery or Barrett
   reduction and vectorize independent cross additions;
2. build half-tables in projective coordinates and batch-normalize them, removing
   most remaining serial inversions;
3. precompute point lifts and doubles for a reused factor base;
4. process final pairs in cache-sized batches and parallelize those batches; and
5. when coefficients are actually required at higher degree, replace the current
   quadratic root-product expansion with a subproduct tree and fast polynomial
   multiplication.

These are proposed work items, not measured gains. A claimed end-to-end gain must
also price input lifting, relation construction, multiplicity handling, memory,
and downstream linear algebra.

## Reproduction and artifacts

```bash
make test
./scripts/run_campaign.sh
sha256sum -c results/SHA256SUMS
```

- `results/corpus/*.jsonl.gz`: all 900,018 complete polynomial records
- `results/corpus/*.summary.csv`: per-curve summaries
- `results/aggregate_summary.csv`: cross-curve aggregate used above
- `results/benchmark_p*.csv`: complete degree 2–128 timings and ratios
- `results/point_counts.csv`: exact point counts where exhaustive counting was
  feasible
- `results/raw/`: self-test, generation, benchmark, and analysis logs
- `results/environment.txt`: compiler, machine, seed, and workload
- `results/SHA256SUMS`: artifact integrity manifest

The corpora occupy approximately 82 MiB compressed. Superseded duplicate copies
from the initial single-curve run were byte-compared with their final named
counterparts and removed; no unique records were deleted.
