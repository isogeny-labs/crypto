# H116 quadratic-Kummer parity experiment

Protocol: `H116-KUMMER-PARITY-001`

Source/launch gate commit: `229d6989aa`

Disposition: **all 184 quadratic x-cover rules are exactly balanced, and Lean
proves the symmetry obstruction for every negation-invariant observable on an
odd-order orbit**. No x-only quadratic-cover parity decoder can be total and
exact. [pcc:unverified]

## Literature and execution gates

The source gate records Lauter--Stange's ECDLP-equivalent EDS problems (DOI
`10.1007/978-3-642-04159-4_20`) and Shipsey--Swart `IACR 2008/444`. The tested
bounded Kummer subset family was not identified in the targeted Crossref and
arXiv searches; that is a prior-art screen, not a novelty claim.

The eight anchors, all subsets of size at most three, both polarities, exact
one-sided statistic and 4,096 family-wise shuffles were pushed before any
signature was computed. The retained run covers every nonidentity scalar on
five prime-order curves, one generator per curve.

## Result

The family contains 184 rules. There are zero total-exact rules and zero rules
exact even after their branch-point abstentions. Every rule has nonpositive
one-sided excess; deterministic tie-breaking therefore retains the first rule,
the single-anchor cover

```text
z^2 = x - x(P).
```

It covers 1,510/1,520 targets and is correct on exactly 755. Its signed excess
`2M-C` is zero. All 4,096 shuffled family maxima meet or exceed the observed
zero statistic, so the plus-one family-wise p-value is one.

| subgroup order | covered | correct |
| ---: | ---: | ---: |
| 127 | 124 | 62 |
| 197 | 194 | 97 |
| 257 | 254 | 127 |
| 397 | 394 | 197 |
| 547 | 544 | 272 |

The exact half counts on every curve are structural. For odd order `r`, the
nonzero scalar orbit pairs `k` with `r-k`. These points are negatives and have
the same x-coordinate, so every tested split/inert signature and prediction is
the same on the pair. Because `r` is odd, the two scalar parities are opposite.
Exactly one prediction in every covered pair is correct; branch points are
also negation-paired.

## Lean obstruction

The proof map precedes the source change. The 219-theorem pinned build proves:

1. `(n-k)%2 != k%2` for odd `n` and `k<=n`;
2. `[(n-k)]P = -[k]P` when `[n]P=0`;
3. for every observable satisfying `obs(-Q)=obs(Q)` and every decoder, at
   least one of the paired parity predictions fails;
4. an even-order mutation where subtraction does not reverse parity.

The exact auditor reports 219 theorems, zero failures, and zero trusted or
`native_decide` declarations. This general theorem is stronger than the finite
184-rule result, but only within the negation-invariant/x-only class. A signed
y-observable or nonsymmetric cover is outside its scope and is the next new
candidate. [pcc:unverified]

## Reproducibility

```text
fed5d58134ea0a4af3d489a6eacc2c1aae02371dae0d7e3f05248fc8e49d1d0a  KUMMER_PARITY_RESULT.json
42b403444a6de1465f0436489d76de54def3b6c6ba74bf0234b729a1ebd7c79a  lean/H116FiberTrace.lean
```

The complete experiment replay is byte-identical.
