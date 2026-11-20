# Order-5/order-11 auxiliary selector on the full point-pair law

**A coefficient is unique on each local CRT curve, but the curve family returns eleven coefficients per eligible point pair.** Replacing the auxiliary curve-order gates `(4,6)` by `(5,11)` gives coefficient period `lcm(5,11)=55`, larger than the exact reduced coefficient domain `1,…,41`. The frozen full-law scan constructs 215,040 centered CRT curves. Each of the 1,344 geometry-eligible source pairs has 160 curves and the same union of eleven selected coefficients: `{2,3,7,8,13,17,18,27,28,37,38}`. The actual reduced coefficient is in that union for 372 of the 1,722 ordered inputs.

| Stage or outcome | Exact count |
| --- | ---: |
| Ordered source pairs | 1,722 |
| Same rational x / x collision at 7 / x collision at 11 | 42 / 224 / 112 |
| Geometry-eligible pairs with both local gate families | 1,344 |
| Local coefficient residues examined at 7 / 11 | 460,992 / 1,788,864 |
| Order-5 / order-11 local curve witnesses | 5,376 / 53,760 |
| Local relation tests / generator steps | 618,240 / 559,104 |
| Centered rational CRT curves constructed | 215,040 |
| Curves with a selected coefficient in `1,…,41` | 147,840 |
| Selected coefficient incidences across point pairs | 14,784 |
| Point pairs whose true coefficient is selected | 372 |
| Analysis-only reduced relation tests / generator steps | 74,046 / 72,324 |

The [protocol](protocol.json) and [runner](run.py) were pushed before measurement in `2ddbcd706e0`. Curve selection never reads the true reduced coefficient. It enumerates all local coefficients on each selected order-5 and order-11 curve, combines them by CRT to one residue modulo 55, and retains a coefficient only if it lies in `1,…,41`. Only after completing that selector does it enumerate the true reduced coefficient for coverage analysis. Every status and selected coefficient is in the compressed [pair rows](pair-rows.jsonl.gz) and [candidate rows](candidate-rows.jsonl.gz), with raw SHA-256 digests in the [result](result.json).

The independent [checked-Sage audit](sage_audit.py) uses a separate nonsingularity test and character-sum point count for the local gate enumeration, Sage elliptic-curve group arithmetic for the local and reduced coefficients, and reconstructs every candidate row. Its [result](sage-audit-result.json) matches both complete raw digests and all 215,040 candidate rows. The [runtime receipt](sage-runtime-info.json) records the accepted Sage build; stderr is empty. Exact operation counts describe this finite algorithm, not calibrated wall-time performance.

## Proof map and interpretation

The pinned [Lean theorem](formal/WidePeriod.lean) `coefficient_unique` proves that two numbers in `1,…,41` with equal residues modulo 5 and 11 are equal; `representative_unique` proves the equivalent modulo-55 form. [Build output](formal/build-output.txt) reports only the standard `propext` and `Quot.sound` axioms. Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9` are pinned. The bridge from local point relations to residues uses the fact that the selected local groups have prime orders 5 and 11 and the affine generator reductions are nonidentity; those concrete group orders and point relations are checked by the arithmetic census and Sage audit. The rational CRT curves and finite count are not Lean-formalized. `lean4checker --fresh` is unavailable on this host.

The coefficient period solves ambiguity **within one curve**, while the 160 curves per eligible pair yield eleven different coefficients after union. Compared with the earlier order-4/order-6 selector's 32,256 CRT curves, this policy constructs 215,040. Its covered true-coefficient inputs rise from 72 to 372, but the selected union rises from seven on 372 pairs to eleven on all 1,344 eligible pairs. These are paired finite-census counts on the same source law; they do not establish a rational relation yield or an ECDLP speedup. The next construction test is to check all selected coefficient-2 curves for rational relations and charge the enlarged candidate stream.

Replay:

```sh
python3 research/exotic_ecdlp_20261021/xedni_wide_period_selector_20261120/run.py > research/exotic_ecdlp_20261021/xedni_wide_period_selector_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_wide_period_selector_20261120/run.stderr
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_wide_period_selector_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_wide_period_selector_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_wide_period_selector_20261120/sage-audit-result.json 2> research/exotic_ecdlp_20261021/xedni_wide_period_selector_20261120/sage-audit.stderr
sh research/exotic_ecdlp_20261021/xedni_wide_period_selector_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_wide_period_selector_20261120/formal/build-output.txt
```
