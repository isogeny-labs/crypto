# A certified rank-one generator fixes the rational lift class

On `E/ℚ: y²=x³−47x²+751x−3997`, checked Sage reports rational rank one,
trivial torsion, and a certain free generator `(17,−10)`. The point used in
the fixed-curve experiments, `Q=(17,10)`, is its negative. Hence every
rational point on this curve is `nQ` for an integer `n`. Since `Q` reduces
to a point of order 43, a rational lift of the frozen public target
`T=(10,1)=22Q̄` has **`n≡22 (mod 43)`**. The smallest possible `|n|` is 21,
attained at `n=−21`. Its exact affine coordinate height metric is **455
bits**; its 128-bit numerical Néron–Tate height is about **209.134055**.

| Lift multiplier `n` | `n mod 43` | Max affine coordinate bits | Numerical `ĥ(nQ)/ĥ(Q)` at 128-bit precision |
| ---: | ---: | ---: | ---: |
| −64 | 22 | 4,203 | 4,096 |
| **−21** | **22** | **455** | **441** |
| 22 | 22 | 496 | 484 |
| 65 | 22 | 4,338 | 4,225 |

The [generator protocol](protocol.json) froze proof-enabled mwrank
generation, `gens_certain`, torsion and a 90-second cap before running.
[`result.json`](result.json) records rank one, `gens_certain=true`, torsion
order one, and `Q=−generator`. The separately frozen
[lift-class protocol](lift-class-protocol.json) fixes the target, residue
class and four representatives. [`lift-class-result.json`](lift-class-result.json)
records exact coordinate bit lengths and 128-bit numerical heights; every
height ratio differs from `n²` by less than `2⁻⁶⁰`. The independent
[`audit.py`](audit.py) rebuilds all four rational points using Python exact
fractions, checks their equations and reductions, exact bit lengths,
source hashes and integer congruences. [`audit-result.json`](audit-result.json)
is `PASS`. The checked Sage runtime and stderr receipts are retained.

## Proof map

| Result | Lean declaration or evidence | Exact scope | Status |
| --- | --- | --- | --- |
| A group homomorphism carries integer multiples to integer multiples | `XedniRankOneLiftClass.reduction_preserves_multiple` | Every pair of additive commutative groups and homomorphism | Lean checked |
| `n mod 43=22` requires `|n|≥21` and `n²≥441` | `target22_min_abs`, `target22_min_square` | Every integer `n` in the stated class | Lean checked |
| `−21` belongs to that class | `minus21_is_target_class` | Exact integer congruence | Lean checked |
| Canonical-height lower-bound arithmetic | `target22_height_lower_bound` | Any nonnegative base height, conditional on the quadratic height law | Lean checked conditional inequality |
| This curve has rank one, trivial torsion, and `Q` primitive | Proof-enabled Sage mwrank, PARI rank, `gens_certain`, torsion check | Displayed curve over `ℚ` | Computational certificate; Lean bridge open |
| Reduction and exact affine coordinates of four lifts | Sage and independent Fraction replay | Displayed curve and F₄₇ reduction | Checked computationally; Lean curve bridge open |

The pinned [`formal/LiftClass.lean`](formal/LiftClass.lean) builds without
admitted claims or research-specific axioms; retained `#print axioms`
output lists only standard Lean foundations. A full Lean theorem for this
particular elliptic curve still needs formal certificates for its
Mordell–Weil group, reduction map, and canonical-height quadraticity. The
128-bit height values are numerical checks; the exact minimum-
multiplier statement is integer arithmetic plus the certified generator
and reduction order.

This gives a concrete fixed-curve lift-search target: finding a rational
preimage of `T` entails finding a multiple in the residue class `22 mod 43`.
The smallest canonical-height representative is `−21Q` because canonical
height scales as `n²` on a non-torsion point. It does not supply an
efficient way to find that representative from the public reduced point.
The next algorithmic test is a public-coordinate-only lift search with
its height/operation cap and target-dependent time recorded separately
from scalar-labelled enumeration and same-point rho.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
gtimeout 90 /Volumes/SSD990/cryptanalysis/sage -python "$(pwd)/research/exotic_ecdlp_20261021/xedni_rank1_generator_certificate_20261121/run_sage.py"
gtimeout 60 /Volumes/SSD990/cryptanalysis/sage -python "$(pwd)/research/exotic_ecdlp_20261021/xedni_rank1_generator_certificate_20261121/lift_class_sage.py"
sh research/exotic_ecdlp_20261021/xedni_rank1_generator_certificate_20261121/formal/build.sh
python3 research/exotic_ecdlp_20261021/xedni_rank1_generator_certificate_20261121/audit.py
```
