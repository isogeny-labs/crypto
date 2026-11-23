# Batched and projective signed-sum expansion

## Scope and status

| Requirement | Status | Evidence |
| --- | --- | --- |
| Level-by-level all-roots expansion | Implemented | `roots_tree_batch` and `roots_tree_projective` |
| Preserve multiplicity and roots at infinity | Recorded local self-test pass in implemented fixtures `[pcc:unverified]` | 900 randomized trials plus 31 targeted degeneracy cases |
| Compare with existing methods | Provisional measurement | `results/optimization_projective_p*.csv` |
| Establish final integrated timing | Open | Must be rerun serially after all parallel branches are integrated |

Both methods enumerate the same `2^(m-2)` signed sums as the baseline. They do
not change the output-size lower bound and do not apply to arbitrary coefficient
vectors without the originating curve points.

## Implemented methods

### Affine level batching (`--method tree`)

Starting with `[P1]`, each level maps every current sum `S` to `S+Pi` and
`S-Pi`. The two additions share the denominator `x(Pi)-x(S)`. All nonzero
denominators at one level are inverted with one Montgomery batch inversion.
Exceptional cases are explicit:

- an infinite `S` produces `Pi` and `-Pi`;
- `S=Pi` produces `2S` and infinity;
- `S=-Pi` produces infinity and `2S`; and
- 2-torsion doubles to infinity.

This uses `m-2` inversions rather than the serial half-table inversions in the
baseline MITM method. It still performs one batch prefix/reverse pass per level.

### Jacobian expansion (`--method projective`)

The same tree stays in Jacobian coordinates. Paired `S+Pi` and `S-Pi` additions
share `Z^2`, the lifted affine x-coordinate, `H`, `H^2`, and other terms. A single
batch inversion normalizes all final Z coordinates. General-a Jacobian doubling
handles equality cases.

This minimizes inversions but uses more multiplications than the affine tree. It
can therefore win at smaller degrees or more expensive inversions and lose once
the linear multiplication term dominates.

## Correctness evidence

Command:

```bash
make test
```

Observed receipt:

```text
PASS self-test s3=899 s4=899 root_methods=900 targeted_degeneracies=31 discriminants=900
```

The nine field/curve fixtures cover the three campaign fields and `(A,B)` values.
Each randomized trial compares the complete sorted multiset and infinity count
against naive, Gray, and MITM enumeration. Targeted tests force `P+(-P)=O`,
doubling, and available rational 2-torsion cases. Every benchmark case is also
checked against MITM and generic finite-root recovery before timing.

The same self-test passed an AddressSanitizer plus UndefinedBehaviorSanitizer
build; its receipt is retained in
`results/raw/optimization_projective_sanitizer.log`.

## Provisional performance evidence

Exact command pattern:

```bash
./build/semaev_lab benchmark --prime P --a 2 --b 3 \
  --seed 6000285944033488934 --min-arity 3 --max-arity 9 \
  --bench-cases 200 --bench-repeats 20 \
  --output results/optimization_projective_pP.csv
```

At degree 128, including sorting/deduplication, the provisional per-polynomial
means were:

| p | MITM | affine tree | projective tree | MITM / affine tree |
| ---: | ---: | ---: | ---: | ---: |
| 1009 | 11.269 us | 10.067 us | 12.278 us | 1.119x |
| 1,000,003 | 10.403 us | 8.390 us | 14.106 us | 1.240x |
| 2,147,483,647 | 15.479 us | 9.428 us | 14.134 us | 1.642x |

These jobs ran while other parallel optimization agents could contend for the
five CPUs. They are retained as diagnostic evidence, not final matched timings.
The final comparison must rerun every candidate serially in the integrated tree.

The projective path beat affine batching at selected lower degrees `[pcc:unverified]`, including
degree 8 for the two larger fields, but affine batching was faster at degree 128
in all three provisional runs. This supports retaining both methods until the
serial crossover is measured.

## Accounting and remaining work

- Inputs start as already-lifted affine points; square-root recovery is excluded.
- Timed outputs are sorted and deduplicated finite roots. Infinity is validated
  but not included in the returned finite-root set.
- Allocation, tree construction, batch inversion, normalization, and sorting are
  included.
- No SIMD, threading, field-specialized reduction, or persistent scratch arena is
  used in this branch.
- `auto` does not select these experimental methods yet.
- Measurements are single-process aggregate means without confidence intervals.
