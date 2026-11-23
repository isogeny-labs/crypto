# Bounded variable-coefficient screen on the full-law CRT curves

**The seven-coefficient panel finds rational relations only at coefficient 2.** On the unchanged full-law stream of 32,256 centered CRT curves, the exact scan checks `P₁+kP₂=O` for `k=2,…,8`. Each coefficient has 42 compatible reduced point pairs among all 1,722 ordered inputs. After the 7- and 11-adic relation screens, only `k=2` has survivors. Its 56 local survivors yield the eight previously verified rational relation records. All other six coefficients have zero rational checks because their local conditions fail.

| Coefficient `k` | Compatible reduced pairs | CRT curves on those pairs | Pass at 7 | Pass at 7 and 11 | Rational relations |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 2 | 42 | 816 | 352 | 56 | 8 |
| 3 | 42 | 816 | 0 | 0 | 0 |
| 4 | 42 | 816 | 0 | 0 | 0 |
| 5 | 42 | 720 | 0 | 0 | 0 |
| 6 | 42 | 912 | 352 | 0 | 0 |
| 7 | 42 | 912 | 0 | 0 | 0 |
| 8 | 42 | 768 | 0 | 0 | 0 |

The [frozen protocol](protocol.json) and [runner](run.py) were committed before measurement in `a72557842da`. The runner reconstructs the complete old CRT stream and matches its SHA-256 digest, then performs 12,054 reduced-group coefficient checks and 5,760 local checks at 7. The 704 cases passing 7 receive a check at 11; 56 receive rational checks. The complete [result](result.json) and [positive rows](positive-rows.jsonl.gz) retain counts and coefficients. The independent [Sage replay](sage_replay.py) uses Sage elliptic-curve point operations for every reduced and local check and matches each coefficient row; its [result](sage-replay-result.json) is `PASS` with empty stderr. The checked [Sage runtime receipt](sage-runtime-info.json) records the installed build. Rational positives at `k=2` match the earlier [full-law independent arithmetic audit and Sage replay](../xedni_full_law_crt_20261120/README.md).

## Structural interpretation and proof map

For any additive good-reduction map `ρ`, a rational relation `P+kQ=O` implies `ρ(P)=-kρ(Q)`. For a fixed `G=ρ(Q)` and `k`, at most one reduced target fits. If two coefficients fit the same target, their multiples of `G` coincide. The [Lean module](formal/VariableCoefficient.lean) proves all three statements for arbitrary additive groups, an additive homomorphism, and every natural-number coefficient. Its pinned Lean `v4.35.0-rc4` / Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9` [build](formal/build-output.txt) reports only `propext` for each theorem. The mathematical bridge supplied as an assumption is that the concrete good-reduction map is a group homomorphism; the 43-point census and CRT construction are checked computationally, not formalized here. `lean4checker --fresh` is unavailable on this host.

The coefficient screen is a **conditional incidence diagnostic**: it tests reduced compatibility first. Finding the compatible coefficient by reduced-group trials already performs target-dependent group work. The panel therefore does not supply a free coefficient-selection step for an unknown target. The local gates with curve orders 4 and 6 also reject six of the seven bounded coefficients on this chosen centered-CRT stream. The experiment preserves these zeros as measured finite counts; it does not extrapolate them to other coefficient ranges, CRT representatives, or auxiliary-prime policies. A subsequent construction should specify a target-independent coefficient rule and charge its search, rational lifts, relation use, and same-target rho comparison.

Replay from this checkout:

```sh
python3 research/exotic_ecdlp_20261021/xedni_variable_coefficient_20261120/run.py > research/exotic_ecdlp_20261021/xedni_variable_coefficient_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_variable_coefficient_20261120/run.stderr
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_variable_coefficient_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_variable_coefficient_20261120/sage_replay.py > research/exotic_ecdlp_20261021/xedni_variable_coefficient_20261120/sage-replay-result.json 2> research/exotic_ecdlp_20261021/xedni_variable_coefficient_20261120/sage-replay.stderr
sh research/exotic_ecdlp_20261021/xedni_variable_coefficient_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_variable_coefficient_20261120/formal/build-output.txt
```
