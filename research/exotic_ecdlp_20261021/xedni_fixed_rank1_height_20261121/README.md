# One rank-one curve reaches the full reduced word span at 7,241 bits

On the fixed rank-one rational curve
`E: y²=x³−47x²+751x−3997` with `Q=(17,10)`, the rational multiples
`nQ` for `1≤n≤14` and the exact words `P=−2nQ`, `P=−6nQ` produce a
rank-20 relation matrix on the order-43 reduction. This is the first prefix
to reach rank 20. Its relation endpoints cover all 42 nonidentity reduced
points; the largest affine numerator or denominator used through that prefix
has **7,241 bits**. At prefix 13, the matrix has rank 19 and endpoints cover
40 points. A separate exact-fraction and modular arithmetic replay agrees
with every Sage prefix rank and coordinate-bit count.

| Positive-multiple prefix `M` | Relation rank | Bits in `MQ` | Largest bits in words through `M` |
| ---: | ---: | ---: | ---: |
| 1 | 2 | 5 | 37 |
| 5 | 10 | 25 | 924 |
| 10 | 18 | 103 | 3,695 |
| 13 | 19 | 177 | 6,242 |
| 14 | 20 | 201 | 7,241 |
| 21 | 20 | 455 | 16,293 |

The [protocol](protocol.json) freezes the exact rational curve, its point,
the checked order-43 reduction, multipliers `2,6`, the 21-prefix law,
sign-folded columns, coordinate height metric and wall cap. The
[`run_sage.py`](run_sage.py) checks every rational group relation and
reduction, then records the coordinate digests, bit lengths, and ranks in
[`rows.jsonl`](rows.jsonl). [`summary.json`](summary.json) records the first
rank-20 prefix. The independent [`audit.py`](audit.py) implements rational
addition with Python `Fraction`, a separate F₄₇ group law and modular
elimination; its [`audit-result.json`](audit-result.json) is `PASS`.

## Lean proof map

| Result | Declaration | Scope | Status |
| --- | --- | --- | --- |
| Positive multiples `1..21` cover every nonzero residue up to sign | `signed_multiples_cover_nonzero` | `ZMod 43` | Lean checked |
| Positive multiples `1..14` alone omit residue 15 up to sign | `prefix14_misses_scalar15` | `ZMod 43` | Lean checked |
| Endpoints with multipliers `1,2,6` and signs from prefix 14 cover every nonzero residue | `prefix14_words_cover_nonzero` | `ZMod 43` | Lean checked |
| The same endpoints from prefix 13 omit residue 15 | `prefix13_words_miss_scalar15` | `ZMod 43` | Lean checked |
| Fixed-curve coordinate heights and prefix matrix ranks | — | Displayed rational curve and its F₄₇ reduction | Independently replayed by Sage and exact Python; Lean height and matrix bridges open |

The pinned [`formal/Coverage.lean`](formal/Coverage.lean) builds and records
`#print axioms`, listing only Lean's standard foundations. The selected
curve's rational relation and rank-one receipts are in the
[six-curve packet](../xedni_anchor_rank1_search_20261121/README.md).

## Cost interpretation

This fixed-curve construction reaches the same algebraic rank as the
six-curve witness by taking rational multiples of a known generator.
Each reduced point comes with a known scalar label, so matching a public
target to the generated endpoints is an enumeration method. The matrix
identities certify the multiplication relations used to generate those
points. The 7,241-bit result measures exact coordinate growth for this
prefix; it is not an asymptotic height bound or a calibrated online-time
comparison. The next experiment should freeze public targets and count
the exact rational arithmetic, reduction, lookup, matrix work, and scalar
verification under one target-dependent clock, paired with same-point rho.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
gtimeout 180 /Volumes/SSD990/cryptanalysis/sage -python "$(pwd)/research/exotic_ecdlp_20261021/xedni_fixed_rank1_height_20261121/run_sage.py"
python3 research/exotic_ecdlp_20261021/xedni_fixed_rank1_height_20261121/audit.py
sh research/exotic_ecdlp_20261021/xedni_fixed_rank1_height_20261121/formal/build.sh
```
