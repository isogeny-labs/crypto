# H116 pointwise rational reconstruction v2 experiment

Corrected protocol `H116-POINTWISE-RATIONAL-RECONSTRUCTION-002` passed all
eight frozen cells. Both canonical joint-interpolation windows had full rank,
the independent verifier reproduced every exact field with zero mismatches,
and the recovered numerator/denominator coefficients were identical across
all extension degrees of each of the three base curves. [pcc:unverified]

## Receipts

```text
2db6297b935f4d5edf4c5976205e7aa3eed1252d26f4602cf14a9a08b1854153  POINTWISE_RATIONAL_RECONSTRUCTION_V2_PROTOCOL.json
194064065ae955ea53c5b2b60ae95c8837cbc8079bf271881596a6a92e7f6767  h116_pointwise_rational_reconstruction_v2.rs
354ab9975e8fe8bf2cea2e3704e2aade762c2824465e0fd7f6ec11d365da43ec  h116_pointwise_rational_reconstruction_v2_verify.py
d1c26c6f0a29a933fb82b64fafbc5c20531fe59122c924613de4bc46f00cb9bc  POINTWISE_RATIONAL_RECONSTRUCTION_V2_RESULT.tsv
7b1759e0c34936ba78dbba0930a6504af3a8f7b68592c34d97f59ba7dfddddc6  POINTWISE_RATIONAL_RECONSTRUCTION_V2_VERIFY.json
```

V1 remains failed and byte-retained at result SHA-256
`d7ff21da47cd866f21fdb21422d0dbf48cd661e65e5963c32d6a2cc114faead9`.
V2 does not overwrite it.

## Reconstruction result

| Cell | Regular x | Pole x | Predicted/observed visible kernel | Ranks | Degrees | Separation margin |
|---|---:|---:|---:|---:|---:|---:|
| fp5-a2-b1-d2 | 17 | 0 | 1 / 1 | 14 / 14 | 7 / 6 | 4 |
| fp5-a2-b1-d3 | 57 | 0 | 1 / 1 | 14 / 14 | 7 / 6 | 44 |
| fp5-a2-b1-d4 | 297 | 0 | 1 / 1 | 14 / 14 | 7 / 6 | 284 |
| fp7-a1-b1-d2 | 27 | 0 | 1 / 1 | 10 / 10 | 5 / 4 | 18 |
| fp7-a1-b1-d3 | 191 | 0 | 1 / 1 | 10 / 10 | 5 / 4 | 182 |
| fp7-a1-b1-d4 | 1,235 | 2 | 5 / 5 | 10 / 10 | 5 / 4 | 1,226 |
| fp11-a1-b1-d2 | 70 | 1 | 2 / 2 | 28 / 28 | 14 / 13 | 43 |
| fp11-a1-b1-d3 | 630 | 7 | 14 / 14 | 28 / 28 | 14 / 13 | 603 |

The first and last `2N` windows recovered the same map in every cell. All
coefficients lie in the embedded base field, and the three stable maps are:

```text
p=5:  N(x)=[2,1,0,3,1,0,4,4]
      D(x)=[4,4,4,0,2,1,1]
p=7:  N(x)=[0,2,2,2,5,2]
      D(x)=[1,6,4,6,1]
p=11: N(x)=[8,4,8,10,2,6,6,2,3,6,0,4,3,8,5]
      D(x)=[5,1,2,6,6,4,9,1,2,4,5,10,6,1]
```

Coefficients are ascending and each denominator is monic. The recovered maps
had zero mismatches on all 2,524 regular x-values, zero regular denominator
zeros, zero missed observed poles, and zero pole cancellations.

## Kernel eigenspace

The retained v1 failure suggested the corrected relation. Lean now proves:

```text
V(P)=P and V+pi=[t]  =>  pi(P)=[t-1]P
pi^d(P)=[(t-1)^d]P
```

and proves equivalence of the Frobenius fixed-locus condition with that scalar
condition on `ker(V-1)`. The current source SHA-256 is
`fe907f1887b4a963dcb1dafbcf3f93eac8037355c656189eacc5a227dcc2d3ca`;
the exact audit reports 128 theorems, zero failures, and zero trusted native
decisions. Cyclicity then predicts visible kernel size
`gcd(N,(t-1)^d-1)`, which matched all eight cells exactly. The cyclicity and
cardinality step itself remains an empirical/model premise, not a new Lean
theorem.

Lean also proves the exact ring factorization
`(V-1)(pi-1)=p+1-t`. This closes the algebraic norm calculation behind the
degree prediction; converting it to a morphism-degree and kernel-cardinality
result remains the explicit geometric obligation.

## Controls and discrepancy

The numerator `+1` mutation rejected all 2,524 regular samples. The
denominator `+1` mutation destroyed all 10 observed poles. A sample of exactly
the cross degree fails the strict Lean cardinality premise in every cell.

The v2 Markdown anticipated a minimum separation margin of one based on the
old split-pole estimate. The observed minimum was four because no nonidentity
kernel point is visible in the `p=5` extensions. This conservative prediction
discrepancy is retained; it is not an acceptance criterion and does not alter
the frozen cells or samples.

Summed per-cell producer time was 58,665,190 ns; independent verification took
1,193,622,862 ns. These are single-run diagnostics.

## Boundary and next proof obligation

This is the first H116 panel in this branch to recover full base-field rational
x-map candidates directly from the pointwise formula with a strict finite
separation margin. It is still finite evidence. A generic result needs proofs
that `V-1` is a separable degree-`N` elliptic morphism with the modeled cyclic
kernel, followed by a scheme/rational-map identification and transport to the
completed local parameter. No subexponential ECDLP runtime follows from this
reconstruction alone. [pcc:unverified]

