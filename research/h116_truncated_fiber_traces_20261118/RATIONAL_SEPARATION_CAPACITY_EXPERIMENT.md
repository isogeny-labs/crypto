# H116 rational-map separation capacity experiment

Protocol `H116-RATIONAL-SEPARATION-CAPACITY-001` completed with all eight
frozen cells accepted by the sealed Rust producer and independently reconstructed
Python verifier.  There were zero exact-field mismatches. [pcc:unverified]

## Formal and data receipts

The Lean source SHA-256 is
`63ba08391dcf0de78d97df5b173d400c85f212cbd4f615f74d25573c6bde5588`.
The pinned build and exact axiom audit report 121 theorems, zero failures, and
zero trusted native decisions.  The formal result proves that regular
evaluations on `S` separate supplied rational functions when both cross-product
degrees are strictly below `|S|`; it also proves the strictness control over
`ZMod 3`.

```text
d58dd2215ca4922dbfc00480dabc218a59ec78e9afcd63bed5257bd04a9ce4fa  RATIONAL_SEPARATION_CAPACITY_PROTOCOL.json
efaafe9bf3107a0b7d643367eaca8543de473ee3e18b0ca5236309422b14eda6  h116_rational_separation_capacity.rs
70e4edd6b81426edf95f722172352d1a5bb6c6c9dea0965d91564c1a4cc9b75d  h116_rational_separation_capacity_verify.py
eb4c9f7912f151a503c364732d0795cc3c0730515054bd42bd6a372d33a099e6  RATIONAL_SEPARATION_CAPACITY_RESULT.tsv
215b6095c19f6dbde7b214d910489db0a3a042e4f5f108479b239399fdd9bca7  RATIONAL_SEPARATION_CAPACITY_VERIFY.json
```

The parent pointwise result SHA-256 is
`5c8f506a81280c650470dac6d2b32deca78c9e63a45282b1a579a6ace66368eb`.

## Results

| Cell | Affine points | Singleton fibers | Paired fibers | Distinct x | Base-field x | Raw strict capacity |
|---|---:|---:|---:|---:|---:|---:|
| fp5-a2-b1-d2 | 34 | 0 | 17 | 17 | 3 | 16 |
| fp5-a2-b1-d3 | 111 | 3 | 54 | 57 | 3 | 56 |
| fp5-a2-b1-d4 | 594 | 0 | 297 | 297 | 3 | 296 |
| fp7-a1-b1-d2 | 54 | 0 | 27 | 27 | 2 | 26 |
| fp7-a1-b1-d3 | 379 | 3 | 188 | 191 | 2 | 190 |
| fp7-a1-b1-d4 | 2,474 | 0 | 1,237 | 1,237 | 2 | 1,236 |
| fp11-a1-b1-d2 | 139 | 3 | 68 | 71 | 7 | 70 |
| fp11-a1-b1-d3 | 1,273 | 1 | 636 | 637 | 7 | 636 |
| **Total** | **5,058** | **10** | **2,524** | **2,534** | **29** | **2,526** |

Every parent group order equaled the affine count plus infinity.  Every fiber
was exactly a `y=0` singleton or a `{y,-y}` pair, and both frozen counting
identities held.  Extension sample growth was positive in all eight cells;
the distinct-x count ranged from 17 to 1,237, hence the raw strict degree
capacity ranged from 16 to 1,236.

For every cell, the retained polynomial
`M_S(T)=prod_{x in S}(T-x)` was monic, nonzero, degree `|S|`, and had zero
evaluation failures on the complete sample.  Thus degree `|S|` is empirically
indistinguishable from zero on `S`, matching the formal strict-bound control.
Adding one to the constant coefficient produced exactly 2,534 rejections over
2,534 samples.

## Diagnostic cost

The producer recorded 1,036,235 extension multiplications and 2,072,470
extension additions to construct the eight vanishing polynomials.  Evaluating
them on their complete samples used 2,072,470 multiplications and 2,072,470
additions; the constant-mutation arm used the same multiplications and
2,072,478 additions.  Summed per-cell producer time was 507,283,660 ns.  The
independent verifier took 11,291,312,020 ns.  These are one-run diagnostics for
quadratic polynomial construction/evaluation, not algorithm benchmarks.

## Requirement-to-evidence result

| Requirement | Outcome |
|---|---|
| Strict finite-evaluation separation | proved in Lean |
| Strictness of the degree boundary | proved over `ZMod 3`; reproduced in 8/8 finite samples |
| Raw distinct-x capacity | measured and independently replayed, 8/8 |
| Actual H116 numerator/denominator construction | open |
| Denominator pole exclusion | open |
| Actual cross-product degree below capacity | open |

The useful next certificate is therefore concrete and falsifiable: supply the
candidate H116 numerator and denominator, derive both cross-product degrees,
enumerate their poles, and test whether the remaining regular distinct-x set
still strictly exceeds the degree bound.  Until that certificate exists, this
result neither identifies the H116 rational/scheme morphism nor changes the
ECDLP runtime or asymptotic status. [pcc:unverified]

