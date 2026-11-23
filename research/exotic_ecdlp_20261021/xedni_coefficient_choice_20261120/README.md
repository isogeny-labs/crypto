# Bounded coefficient choice restores strict two-prime admission on four ordered xedni pairs

A frozen 27-choice family of rational Weierstrass coefficients admits **four of the 20 ordered point pairs** through the simultaneous auxiliary gates `#E(F₇)=4` and `#E(F₁₁)=6`. Two admitted pairs satisfy `P₁+2P₂=O` on the fixed 43-point group over `F₄₇`. Their chosen rational lifts fail that relation, with a direct reduction certificate at 7: the same sum is nonidentity on the good-reduction curve over `F₇`. This demonstrates that coefficient choice changes the outcome of the prior fixed-curve second-prime screen, which admitted zero pairs under both order arms.

For each of the 20 pairs accepted by the earlier weak gate at 7, the screen takes its centered rational point lifts, chooses `a₁,a₂,a₃ = 47(k₁,k₂,k₃)` with each `kᵢ∈{−1,0,1}`, and solves the two point equations for `a₄,a₆` exactly. The resulting 540 curves all reduce to the original curve `y²=x³−x−2` at 47. The experiment checks integrality, discriminant, point-incidence rank, group order, and `P₁+2P₂` over `ℚ` and each applicable finite field. Separate strict (`#E(F₁₁)=6`) and weak (`#E(F₁₁)=7`) arms select the first passing coefficient triple in lexicographic order for each input pair.

| Frozen screen | Candidate choices | Passing candidate choices | Selected ordered pairs | Reduced relation events | Rational relation events |
| --- | ---: | ---: | ---: | ---: | ---: |
| 7-order-4 and 11-order-6 | 540 | 4 | 4 | 2 | 0 |
| 7-order-4 and 11-order-7 | 540 | 0 | 0 | 0 | 0 |

The four passing choices cover two distinct rational curves up to the recorded coefficient tuples and four ordered point pairs. The reduced-event pairs are input indices 2 and 7; their selected `p₇.relation` is false. Their coefficients are respectively `[47,47,−47,20743/31,−342222/31]` and `[−47,47,47,20743/31,−342222/31]`. The other two passing pairs are indices 14 and 17. The per-choice [rows](rows.jsonl.gz) retain all 540 statuses, including nonintegrality and bad reduction at the auxiliary primes; the [result](result.json) retains full counts and the selection map. The gzip file losslessly restores the exact raw SHA-256 in the result, as checked by [package_rows.py](package_rows.py).

The [protocol](protocol.json) fixed the source hash, coefficient box, gates, order of selection, and 540-choice cap before execution. The exact source bytes are bundled in [source-rows-v2.jsonl.gz](source-rows-v2.jsonl.gz); [source-amendment.json](source-amendment.json) records this portability addition without changing the selection or gates. The run used the repository's checked Sage launcher, with [runtime receipt](sage-runtime-info.json) outside the measurement. [Independent audit](audit.py) replays every rational coefficient, discriminant, finite-field order, relation, gate and selected row using Python exact fractions and separate group formulas; [audit-result.json](audit-result.json) records `PASS` for 540 rows. Stderr is empty. These timings are correctness-run durations, not performance comparisons.

This finite family leaves the unrestricted CRT coefficient-selection rule open. A next experiment should freeze an input-independent coefficient search over the full 1,722 ordered-pair law, retaining curve-construction effort and all failed gates; a larger-field and higher-point selection law is needed to probe the rank and uniform-height hypotheses in the Lean obstruction.

Replay from this checkout:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_coefficient_choice_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_coefficient_choice_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_coefficient_choice_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_coefficient_choice_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_coefficient_choice_20261120/package_rows.py verify
python3 research/exotic_ecdlp_20261021/xedni_coefficient_choice_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_coefficient_choice_20261120/audit-result.json
```

This packet introduces no mathematical theorem. The existing Lean result remains conditional on a uniform rank, torsion, height, and reduction envelope for the selected family.
