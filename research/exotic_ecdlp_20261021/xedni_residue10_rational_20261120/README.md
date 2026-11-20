# Rational check of every locally selected residue-10 curve

**None of the 1,488 frozen CRT curves carrying local coefficient residue 10 has a rational relation `P+10Q=O`.** The [protocol](protocol.json) and [Sage runner](run_sage.py) were pushed before measurement in `f3a979ea671`. Curve selection reads only the prior local-residue candidate rows; it never reads a reduced discrete logarithm or reduced relation flag.

| Stage | Exact count |
| --- | ---: |
| Full-law ordered point pairs | 1,722 |
| Full-law centered CRT candidates | 32,256 |
| Candidates carrying local residue 10 | 1,488 |
| Rational `P+10Q` checks | 1,488 |
| Rational relations | 0 |
| Reduced-compatible selected candidates, post-run analysis | 32 |
| Reduced-incompatible selected candidates, post-run analysis | 1,456 |

The exact [Sage result](result.json), [runtime receipt](sage-runtime-info.json), and complete [rational-check rows](rational-checks.jsonl.gz) preserve source-pair index, local witness indices, centered CRT shifts, rational coefficients, and every zero status. The independent [exact arithmetic audit](audit.py) replays all 1,488 rational group sums using a separate Python law, verifies the raw-row digest, and checks reduced compatibility only after selection. Its [result](audit-result.json) passes: 32 selected curves are compatible with `T+10G=O` at 47, while 1,456 are excluded by that necessary reduction condition. The main run deliberately paid for their rational checks to measure the local selector without a reduced-log oracle. Both executions have empty stderr.

The [variable-coefficient Lean theorem](../xedni_variable_coefficient_20261120/formal/VariableCoefficient.lean) proves that any rational `P+kQ=O` relation must reduce to `T=-kG`, so the post-run reduced check is a necessary condition. The theorem does not imply a rational relation for the 32 compatible curves. The [local-selector census](../xedni_local_selector_20261120/README.md) supplies the full 32,256-candidate selection and independent Sage audit. This result is exact for coefficient 10 and the frozen centered CRT representatives. The preexisting 2,249,856 local coefficient-residue examinations, 32,256 CRT constructions, and 177,408 local relation tests remain part of any total-cost assessment using this selector.

Replay:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_residue10_rational_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_residue10_rational_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_residue10_rational_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_residue10_rational_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_residue10_rational_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_residue10_rational_20261120/audit-result.json
```
