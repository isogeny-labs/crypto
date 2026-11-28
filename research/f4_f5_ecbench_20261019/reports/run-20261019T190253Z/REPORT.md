# Complete cold DLP runtime through ecbench

Run 20261019-190253-boolean-adaptive-ecbench-cbeb18, source 9bed104b0a. Complete one-target cold DLP recovery on dedicated c7i.2xlarge; native ecbench L2 requirement, one worker,8 independent public targets per curve,5 measured rounds plus1 warmup,11 interleaved arms. Sources/specs and complete audit certificates remain adjacent/raw. Missing, failed and below-level rows are retained; a launch is not a result.

All methods within each session share the same public targets and standard polynomial subspace: dimension5 at n17 and6 at n23. These bases differ from the component experiment's Frobenius-invariant bases. The curve identities are icv1-f2m17-tm101-00378d4e and icv1-f2m23-tm5197-1f85e9e1. This two-size panel is not a scaling claim.

The pipeline solve clock includes factor-base construction, oracle prepare, relation collection, linear algebra, target descent/recovery and internal verification. Whole child process additionally includes adapter construction, report metadata, common workload construction and startup. Both clocks are recorded; the native harness's comparison gate uses the pipeline solve clock. Independent parent verification/replay, compilation and warmup are outside these clocks.

F4/F5 baseline uses the prior recursive policy with qualified GF2 kernels. Sources0acfc55b7f/71c4c99d85 use candidate matrix_runtime=quadratic-tail; corrected source9bed104b0a uses adaptive-quadratic-tail, with a different method identity. F6 names kernel_runtime=native-v1. Generalized typed roots and both Semaev table backends participate. Environment-override refusal remains intact; no admission threshold is relaxed.

Wall ratios below are B/A from ecbench's own paired bootstrap and A/A gate. Admission, completeness and outside_noise must all support a gain; partial/failed sessions do not become successful comparisons. Process medians are supplementary observed clocks, not a replacement gate.

Counted S, generic floor and matched strong rho are retained in table-n*.txt and ecbench-summary.json. Gray assignments are explicitly counted unpriced; reference native unpriced counters are retained too. Sensitivity bands at0/0.1/1 GAE per recorded operation are diagnostic and deduplicate generalized solver-count aliases. An unquantified counter stays unquantified. No ratio of lower bounds is a full-cost ranking.

Exact original-output and replay checks, query-count comparisons, source/binary hashes, failures and job resources remain in the evidence. All-L2 coverage is separate from execution/verification completeness; a fully verified session can have below-level timing rows. Low-sample clocks are descriptive; no threshold is relaxed. New matrix policies stay opt-in; no external-fastest, other-hardware or rho-beating claim is made.

n17: 528 total records, 88 warmups; replay audit OK True; all requested L2 timing coverage False.

n17 f4: candidate/baseline pipeline wall 0.5623, CI95 [0.5599368512706958, 0.5651490815056114], gate admitted, outside A/A noise True. Ratios above1 are regressions, below1 are reductions in time; admission/completeness still govern interpretation.

n17 f5: candidate/baseline pipeline wall 0.5822, CI95 [0.5795548493574879, 0.5839443622722156], gate admitted, outside A/A noise True. Ratios above1 are regressions, below1 are reductions in time; admission/completeness still govern interpretation.

n23: 528 total records, 88 warmups; replay audit OK True; all requested L2 timing coverage False.

n23 f4: candidate/baseline pipeline wall 0.6309, CI95 [0.6299009971212228, 0.6315614249975599], gate admitted, outside A/A noise True. Ratios above1 are regressions, below1 are reductions in time; admission/completeness still govern interpretation.

n23 f5: candidate/baseline pipeline wall 0.6532, CI95 [0.6526133456284446, 0.6539370178846236], gate admitted, outside A/A noise True. Ratios above1 are regressions, below1 are reductions in time; admission/completeness still govern interpretation.

![ecbench-cold-recovery.svg](../../ecbench-cold-recovery.svg)

## Recorded clocks and admission

|n|Arm|Solve ms|Process ms|Verified|L2|Requested|All L2|
|---|---|---|---|---|---|---|---|
|17|rho-strong|2.6819|25.1376|40|40|40|True|
|17|f4-baseline|91.3372|119.0197|40|40|40|True|
|17|f4-aa|91.2443|117.9840|40|40|40|True|
|17|f4-tail|51.3415|79.3053|40|40|40|True|
|17|f5-baseline|95.9779|124.2733|40|40|40|True|
|17|f5-aa|95.8601|129.1508|40|40|40|True|
|17|f5-tail|55.9904|84.5148|40|39|40|False|
|17|f6-native|113.0123|141.3677|40|40|40|True|
|17|generalized-typed|91.9735|118.9833|40|40|40|True|
|17|semaev-batched|0.8323|27.9561|40|33|40|False|
|17|semaev-full-table|0.8427|26.4699|40|34|40|False|
|23|rho-strong|2.7805|26.6863|40|39|40|False|
|23|f4-baseline|2592.6712|2619.9191|40|40|40|True|
|23|f4-aa|2593.9296|2623.7207|40|40|40|True|
|23|f4-tail|1636.2769|1666.0858|40|40|40|True|
|23|f5-baseline|2764.2312|2793.9766|40|40|40|True|
|23|f5-aa|2766.4568|2796.0575|40|40|40|True|
|23|f5-tail|1806.6459|1836.5007|40|40|40|True|
|23|f6-native|3952.1317|3981.7135|40|40|40|True|
|23|generalized-typed|5946.1759|5972.4103|40|40|40|True|
|23|semaev-batched|20.5768|47.8789|40|39|40|False|
|23|semaev-full-table|19.9445|46.3640|40|39|40|False|

## Native harness comparisons; B/A

|n|Pair|Wall gate|Median ratio|CI95|Outside noise|Below L2|
|---|---|---|---|---|---|---|
|17|f4|admitted|0.5623|0.5599..0.5651|True|0|
|17|f5|admitted|0.5822|0.5796..0.5839|True|1|
|17|rho-f4-tail|admitted|19.1779|17.2474..22.8799|True|0|
|17|rho-f5-tail|admitted|20.9193|19.1648..25.4592|True|1|
|17|rho-f6-native|admitted|41.3187|36.7152..48.1579|True|0|
|17|rho-generalized-typed|admitted|34.2003|30.2126..39.1385|True|0|
|17|rho-semaev-batched|admitted|0.3056|0.2803..0.3438|True|7|
|17|rho-semaev-full-table|admitted|0.3111|0.2959..0.3514|True|6|
|23|f4|admitted|0.6309|0.6299..0.6316|True|0|
|23|f5|admitted|0.6532|0.6526..0.6539|True|0|
|23|rho-f4-tail|admitted|586.4033|419.1506..692.5298|True|1|
|23|rho-f5-tail|admitted|647.2587|462.0887..762.5880|True|1|
|23|rho-f6-native|admitted|1416.4945|1014.6719..1680.5328|True|1|
|23|rho-generalized-typed|admitted|2137.0665|1525.7469..2506.3016|True|1|
|23|rho-semaev-batched|admitted|7.3576|5.6803..8.4332|True|2|
|23|rho-semaev-full-table|admitted|7.2811|5.5185..8.3262|True|2|

## Partly priced S sensitivity; all entries diagnostic

|n|Arm|0 GAE/op|0.1 GAE/op|1 GAE/op|Lower bound|Unknown entries|
|---|---|---|---|---|---|---|
|17|rho-strong|4.0416|4.185|5.4753|True|0|
|17|f4-baseline|43.799|43.799|43.799|False|0|
|17|f4-aa|43.799|43.799|43.799|False|0|
|17|f4-tail|33.227|33.288|33.842|True|0|
|17|f5-baseline|43.731|43.731|43.731|False|0|
|17|f5-aa|43.731|43.731|43.731|False|0|
|17|f5-tail|33.164|33.225|33.779|True|0|
|17|f6-native|33.082|33.082|33.082|False|0|
|17|generalized-typed|16.421|1080.1|10653|True|0|
|17|semaev-batched|17.317|17.317|17.317|False|0|
|17|semaev-full-table|18.083|18.083|18.083|False|0|
|23|rho-strong|0.88552|0.95031|1.5335|True|0|
|23|f4-baseline|72.251|72.251|72.251|False|0|
|23|f4-aa|72.251|72.251|72.251|False|0|
|23|f4-tail|65.998|66.034|66.354|True|0|
|23|f5-baseline|72.228|72.228|72.228|False|0|
|23|f5-aa|72.228|72.228|72.228|False|0|
|23|f5-tail|65.97|66.006|66.326|True|0|
|23|f6-native|65.541|65.541|65.541|False|0|
|23|generalized-typed|43.007|16897|1.6859e+05|True|0|
|23|semaev-batched|43.709|43.709|43.709|False|0|
|23|semaev-full-table|44.039|44.039|44.039|False|0|

## Explicit unpriced operation ledgers; different units are not equated

|n|Arm|Unit|Samples|Median count|Min|Max|
|---|---|---|---|---|---|---|
|17|rho-strong|canonicalisations_uncharged|40|192.0|64|356|
|17|rho-strong|partition_hashes_uncharged|40|148.5|30|306|
|17|rho-strong|table_inserts_uncharged|40|9.0|2|20|
|17|rho-strong|table_queries_uncharged|40|10.0|3|22|
|17|f4-tail|quadratic_tail_assignments_uncharged|40|132.0|4|426|
|17|f5-tail|quadratic_tail_assignments_uncharged|40|132.0|4|426|
|17|generalized-typed|generalized_assignments_uncharged|40|2613760.0|1051648|5709824|
|23|rho-strong|canonicalisations_uncharged|40|672.0|160|1217|
|23|rho-strong|partition_hashes_uncharged|40|594.5|119|1105|
|23|rho-strong|table_inserts_uncharged|40|39.5|12|79|
|23|rho-strong|table_queries_uncharged|40|40.5|13|80|
|23|f4-tail|quadratic_tail_assignments_uncharged|40|599.0|12|2000|
|23|f5-tail|quadratic_tail_assignments_uncharged|40|599.0|12|2000|
|23|generalized-typed|generalized_assignments_uncharged|40|355270656.0|33546240|580587520|

## Admitted phase clocks; recorded ms

|n|Arm|Phase|Median ms|Min ms|Max ms|Samples|
|---|---|---|---|---|---|---|
|17|f4-baseline|factor_base|0.1368|0.1314|0.1656|40|
|17|f4-baseline|linear_algebra|0.0111|0.0040|0.0211|40|
|17|f4-baseline|oracle_setup|0.0489|0.0457|0.0659|40|
|17|f4-baseline|relations|91.0934|39.1054|205.1595|40|
|17|f4-baseline|verify|0.0021|0.0016|0.0031|40|
|17|f4-aa|factor_base|0.1370|0.1307|0.1603|40|
|17|f4-aa|linear_algebra|0.0099|0.0054|0.0204|40|
|17|f4-aa|oracle_setup|0.0487|0.0444|0.1227|40|
|17|f4-aa|relations|90.9996|39.0758|204.9668|40|
|17|f4-aa|verify|0.0020|0.0016|0.0027|40|
|17|f4-tail|factor_base|0.1362|0.1303|0.1634|40|
|17|f4-tail|linear_algebra|0.0107|0.0022|0.0158|40|
|17|f4-tail|oracle_setup|0.0490|0.0436|0.0565|40|
|17|f4-tail|relations|51.0986|21.8000|114.7090|40|
|17|f4-tail|verify|0.0020|0.0016|0.0025|40|
|17|f5-baseline|factor_base|0.1356|0.1290|0.1634|40|
|17|f5-baseline|linear_algebra|0.0094|0.0023|0.0187|40|
|17|f5-baseline|oracle_setup|0.0472|0.0438|0.0582|40|
|17|f5-baseline|relations|95.7273|41.3605|215.7969|40|
|17|f5-baseline|verify|0.0020|0.0016|0.0026|40|
|17|f5-aa|factor_base|0.1356|0.1310|0.1606|40|
|17|f5-aa|linear_algebra|0.0103|0.0022|0.0169|40|
|17|f5-aa|oracle_setup|0.0473|0.0438|0.0579|40|
|17|f5-aa|relations|95.6181|41.3161|215.2174|40|
|17|f5-aa|verify|0.0019|0.0016|0.0026|40|
|17|f5-tail|factor_base|0.1371|0.1286|0.1576|39|
|17|f5-tail|linear_algebra|0.0091|0.0024|0.0172|39|
|17|f5-tail|oracle_setup|0.0492|0.0437|0.0581|39|
|17|f5-tail|relations|55.7400|23.7127|125.0042|39|
|17|f5-tail|verify|0.0020|0.0016|0.0027|39|
|17|f6-native|factor_base|0.1349|0.1291|0.1594|40|
|17|f6-native|linear_algebra|0.0100|0.0031|0.0198|40|
|17|f6-native|oracle_setup|0.0472|0.0434|0.0647|40|
|17|f6-native|relations|112.7684|45.7423|241.6172|40|
|17|f6-native|verify|0.0020|0.0016|0.0026|40|
|17|generalized-typed|factor_base|0.1356|0.1286|0.1576|40|
|17|generalized-typed|linear_algebra|0.0094|0.0026|0.0154|40|
|17|generalized-typed|oracle_setup|0.0467|0.0433|0.0565|40|
|17|generalized-typed|relations|91.7102|37.1233|197.4759|40|
|17|generalized-typed|verify|0.0020|0.0016|0.0027|40|
|17|semaev-batched|factor_base|0.1437|0.1393|0.1686|33|
|17|semaev-batched|linear_algebra|0.0071|0.0035|0.0104|33|
|17|semaev-batched|oracle_setup|0.0183|0.0156|0.0257|33|
|17|semaev-batched|relations|0.6191|0.2713|1.2083|33|
|17|semaev-batched|verify|0.0019|0.0016|0.0026|33|
|17|semaev-full-table|factor_base|0.1361|0.1309|0.1666|34|
|17|semaev-full-table|linear_algebra|0.0071|0.0026|0.0091|34|
|17|semaev-full-table|oracle_setup|0.0569|0.0552|0.0744|34|
|17|semaev-full-table|relations|0.5934|0.2899|1.2396|34|
|17|semaev-full-table|verify|0.0020|0.0016|0.0023|34|
|23|f4-baseline|factor_base|0.1538|0.0582|0.1826|40|
|23|f4-baseline|linear_algebra|0.0261|0.0048|0.0438|40|
|23|f4-baseline|oracle_setup|0.0767|0.0729|0.0878|40|
|23|f4-baseline|relations|2592.4235|246.9147|4210.3284|40|
|23|f4-baseline|verify|0.0036|0.0032|0.0050|40|
|23|f4-aa|factor_base|0.1557|0.0607|0.1963|40|
|23|f4-aa|linear_algebra|0.0276|0.0045|0.0415|40|
|23|f4-aa|oracle_setup|0.0761|0.0728|0.0996|40|
|23|f4-aa|relations|2593.6937|247.0205|4206.4476|40|
|23|f4-aa|verify|0.0037|0.0032|0.0049|40|
|23|f4-tail|factor_base|0.1561|0.0620|0.2481|40|
|23|f4-tail|linear_algebra|0.0232|0.0053|0.0417|40|
|23|f4-tail|oracle_setup|0.0771|0.0738|0.1517|40|
|23|f4-tail|relations|1636.0355|154.8664|2650.6260|40|
|23|f4-tail|verify|0.0037|0.0033|0.0049|40|
|23|f5-baseline|factor_base|0.1564|0.0594|0.1927|40|
|23|f5-baseline|linear_algebra|0.0255|0.0033|0.0438|40|
|23|f5-baseline|oracle_setup|0.0766|0.0731|0.0980|40|
|23|f5-baseline|relations|2763.9939|263.3492|4492.0319|40|
|23|f5-baseline|verify|0.0036|0.0032|0.0046|40|
|23|f5-aa|factor_base|0.1536|0.0593|0.1904|40|
|23|f5-aa|linear_algebra|0.0267|0.0056|0.0505|40|
|23|f5-aa|oracle_setup|0.0775|0.0726|0.0996|40|
|23|f5-aa|relations|2766.2142|263.4290|4493.2206|40|
|23|f5-aa|verify|0.0036|0.0032|0.0052|40|
|23|f5-tail|factor_base|0.1545|0.0618|0.1813|40|
|23|f5-tail|linear_algebra|0.0228|0.0028|0.0429|40|
|23|f5-tail|oracle_setup|0.0775|0.0722|0.0991|40|
|23|f5-tail|relations|1806.3969|170.4640|2928.3321|40|
|23|f5-tail|verify|0.0037|0.0032|0.0045|40|
|23|f6-native|factor_base|0.1563|0.0602|0.2034|40|
|23|f6-native|linear_algebra|0.0308|0.0041|0.0526|40|
|23|f6-native|oracle_setup|0.0757|0.0734|0.0953|40|
|23|f6-native|relations|3951.8285|374.9495|6444.7310|40|
|23|f6-native|verify|0.0037|0.0032|0.0048|40|
|23|generalized-typed|factor_base|0.1500|0.0567|0.1898|40|
|23|generalized-typed|linear_algebra|0.0264|0.0038|0.0478|40|
|23|generalized-typed|oracle_setup|0.0742|0.0706|0.0944|40|
|23|generalized-typed|relations|5945.8935|560.9768|9731.2949|40|
|23|generalized-typed|verify|0.0037|0.0033|0.0043|40|
|23|semaev-batched|factor_base|0.1623|0.1538|0.2572|39|
|23|semaev-batched|linear_algebra|0.0143|0.0023|0.0239|39|
|23|semaev-batched|oracle_setup|0.0315|0.0288|0.0379|39|
|23|semaev-batched|relations|20.3236|1.9198|33.8123|39|
|23|semaev-batched|verify|0.0037|0.0032|0.0044|39|
|23|semaev-full-table|factor_base|0.1497|0.0568|0.1804|39|
|23|semaev-full-table|linear_algebra|0.0150|0.0025|0.0228|39|
|23|semaev-full-table|oracle_setup|0.1197|0.1162|0.1407|39|
|23|semaev-full-table|relations|19.6966|1.9538|33.6351|39|
|23|semaev-full-table|verify|0.0036|0.0032|0.0047|39|
