# The unique minimum-height lift of the frozen target

On `E/ℚ: y²=x³−47x²+751x−3997`, the rational point **`−21Q`**, where `Q=(17,10)`, is the unique lift of `T=(10,1)∈E(F₄₇)` with minimum `H=max(|a|,b)` under `x=a/b²`, `gcd(a,b)=1`, `b>0`. Its exact height is

`H = 68617873573181386696531408604448494739990891555786277625197982824152597888532291858202306593`,

a 306-bit integer. The only other target-class multiplier with `|n|<64` is `n=22`, whose `H` has 333 bits. Every remaining target-class multiple has **`H>2^1300`** by the explicit canonical-height comparison from the [height-exclusion packet](../xedni_explicit_height_exclusion_20261121/README.md).

| Target-class multiplier | `H` bit length | Verification |
| ---: | ---: | --- |
| `−21` | 306 | Exact Python Fraction group law and checked-Sage replay |
| `22` | 333 | Exact Python Fraction group law and checked-Sage replay |
| Every other `n≡22 (mod 43)` | Greater than 1,300 | Integer residue-case theorem plus height inequality |

The [generator certificate](../xedni_rank1_generator_certificate_20261121/README.md) supplies `E(ℚ)=ℤQ` and reduction `T=22Q̄`; hence every rational lift is `nQ` with `n≡22 (mod 43)`. Elementary integer arithmetic splits that class into `−21`, `22`, or `|n|≥64`. For the tail, the exact 2,894-bit numerator of `x(65Q)` and the conservative Silverman bound `B<14` give `ĥ(Q)>(2893 log 2−14)/4225`. Canonical-height quadraticity then gives

```
h(x(nQ)) > 4096(2893·(2/3)−14)/4225−14
          = 23349974/12675 > 1820,
```

using `2/3<log 2`. Under `H≤2^1300`, the candidate law would give `h(x(nQ))≤2600 log 2<1820`, using `log 2<7/10`. Thus every tail point exceeds the cap. The finite coordinates and their exact decimal heights then establish the unique minimum.

The [protocol](protocol.json) froze the curve, source hashes, target class, finite representatives, tail cap, and claim before the computation. [`run.py`](run.py) independently rebuilds the finite multiples with exact rational arithmetic, verifies their reductions and square denominators, and audits the tail inequality. [`result.json`](result.json) records the exact heights and coordinate digests. [`check_sage.py`](check_sage.py) independently replays both coordinates with the accepted checked Sage build; [`sage-replay.json`](sage-replay.json) is `PASS`. The runtime and stderr receipts are retained.

## Proof map

| Result | Lean declaration or evidence | Status |
| --- | --- | --- |
| Target-class partition into `−21`, `22`, and `|n|≥64` | `XedniMinimumLiftHeight.target_cases` | Lean checked |
| Tail multiplier satisfies `n²≥4096` | `tail_square` | Lean checked |
| `2/3<log 2<7/10` | `log_two_bounds` | Lean checked |
| Height inputs contradict `H≤2^1300` for the tail | `no_tail_under_cap` | Lean checked conditional real inequality |
| Finite height comparisons and tail imply unique minimum | `unique_minimizer` | Lean checked conditional structural implication |
| Exact finite points, reductions, and heights | Python Fraction and checked Sage | Independently checked computationally |
| Mordell–Weil generator and model-specific height comparison | Prior packets and [Sage's Silverman-bound documentation](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_rational_field.html) | Mathematical inputs; curve-specific Lean bridge open |

The pinned [`formal/MinimumHeight.lean`](formal/MinimumHeight.lean) builds without `sorry`, custom claim axioms, or native-evaluation bypasses. Its `#print axioms` receipt lists only standard Lean foundations. The **full theorem for this specific curve remains partially formalized**: a Lean account of its Mordell–Weil generator, reduction map, and canonical-height comparison is still required to discharge the conditional hypotheses. The exact arithmetic and written proof identify those obligations rather than assuming the desired minimum.

The result fixes the smallest coordinate search height for this curve and target. A next algorithmic test is to seek this rational point from `T` and the curve equation without scalar-labelled enumeration, charging the complete target-dependent search.

Replay from the repository root:

```sh
python3 research/exotic_ecdlp_20261021/xedni_minimum_lift_height_20261121/run.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python "$(pwd)/research/exotic_ecdlp_20261021/xedni_minimum_lift_height_20261121/check_sage.py"
sh research/exotic_ecdlp_20261021/xedni_minimum_lift_height_20261121/formal/build.sh
```
