# Fresh comparison and relation-degree follow-up

The fixed relation additions did not lower observed F4 degree on this development grid. All18 cells completed with degree5 and independently matching complete std/slimgb/msolve bases; their affine ideals equal the corresponding star. The root quartic reduces matrix rows, while clique closure can enlarge them. These are unsigned descriptive observations, not a further15x speedup. PCC13cd4f. [pcc:unverified]

| Requested lead | Evidence and outcome |
|---|---|
| Check improvement against previous fastest Rust candidate | Three separately preregistered fresh seeds,324 matching answers, all per-case A/A gates passed; registered sustained increment across all3 isfalse. Prior-fastest work medians1.032623/1.035859/1.044978; full-paid1.002306/1.002391/1.002709. Fresh report PCCe67478/91e8cf. |
| Distinguish construction, checking and paid cost | Six separate instrumented calls retain input, construction/wire, wirewrite, root replay and unattributed parent remainder. No causal startup attribution. Identical full common fees in fresh comparison; conditional median free-candidate ceilings about1.08 apply only to that policy. |
| Test relation classes and trace degree | Fixed6classes x n3/4/6, seed7, same point/scalar sequence. All actual F4 and affine closure thresholds5. Top-form first vanishing stays null within caps8/8/6, column700. Raw degree-by-degree ranks/pivots/falls/nonzeros/provenance retained. |
| Account for certificate and matrix effects | Root projection changes maximum rows34to29,71to57,136to126 with same columns37/71/175. At n6 clique gives455rows/364columns versus star136/175. Selected quartics have80 proof terms and product degree14. |
| Integrate and preserve accurate results | Standalone Rust capture/diagnostics/raw reporter and cost formatter added; unchanged actual-root F4 and production algebra.42 capture/15 library release tests, strict lint/build and6 correctness-only calibration cells passed; corrupted certificate refused. Full raw, protocols, source bundles and visual reports attached to PR4349. |

All measurements above are producer evidence [pcc:unverified], PCCf0c771/d31431/13cd4f and fresh claims. The original15x result compares two original paid workflows on known seed7; it does not establish15x over the previous fastest implementation. Neither follow-up supplies that additional gain. All original conditional reserved seeds19001/23003/29009 remain untouched.

The next discriminating hypothesis is to select directly certified low-degree consequences by **early independent row-space contribution subject to certificate product-degree and support budgets**, rather than by output sparsity. Existing pair cubics provide a concrete starting class. Compare their contribution at degree4 against an equally sparse noncontributing control, include all construction/selection/checking costs, then observe actual F4 degree and paid runtime. This is a successor hypothesis, not a measured gain or novelty claim. The earlier two-variable c1/c0 ablation already shows equal output degree can have different closure effects; FINAL_REPORT.md also records that cubics alone did not approach tenfold. Reuse those controls instead of treating them as a new discovery. [pcc:unverified]

Sources checked (KB unavailable): study README degree definitions and original c1/c0 observations; FINAL_REPORT.md existing cubic timing limits; projection.rs full vertex certificates; cheap.rs exact direct pair identities; current raw F4/trace/Hilbert outputs. The prior primary review of Caminata–Gorla arXiv2112.05579v2 supplies the degree distinctions. No intrinsic Castelnuovo regularity claim, finite top-form threshold, or physical cause for parent remainder follows.

Flow: fresh3 planned/attempted/completed cases,0 excluded/replaced;21 main blocks per case plus per-case A/A/warmup,324 checked uninstrumented answers and6 distinct diagnostic calls. Development18 planned/attempted/completed cells,0 failures/excluded/replaced;6 oldn3 correctness cells are separately retained and excluded from development. LocalL0/CPU0/checkpoint waiver/zero cloud spend. Current canonical experiment.export validator was inaccessible/encrypted; no canonical-schema validation is claimed. Signed independent validation/CI/review remain pending; scoped checks are not a whole-root suite verdict.

\newpage

# Fixed development relation ablations

Method PCC-20261116-49c28b; measurements PCC-20261116-13cd4f. [pcc:unverified]

Single bounded calls on public known-answer seed7 fixtures. No statistical speedup admission. Exact F4 observed degree, affine closure threshold, and top-form Hilbert first vanishing are different quantities. Missing/capped values remain null. Unsigned localL0. [pcc:unverified]

| n | Class | Added | Same ideal | F4 degree | Max rows | Max columns | Affine closure | Top-form vanishing | Paid ms |
|---|---|---:|---|---:|---:|---:|---:|---:|---:|
|3|star|0|true|5|34|37|5|null|196.521594|
|3|root-projection|1|true|5|29|37|5|null|60.462200|
|3|nonroot-projections|0|true|5|34|37|5|null|56.666605|
|3|all-projections|1|true|5|29|37|5|null|59.721734|
|3|leaf-closure|0|true|5|33|43|5|null|57.946463|
|3|clique|0|true|5|33|43|5|null|69.651513|
|4|star|0|true|5|71|71|5|null|59.747252|
|4|root-projection|1|true|5|57|71|5|null|82.038833|
|4|nonroot-projections|0|true|5|71|71|5|null|73.226696|
|4|all-projections|1|true|5|57|71|5|null|71.429012|
|4|leaf-closure|0|true|5|55|77|5|null|62.705453|
|4|clique|0|true|5|100|100|5|null|126.574224|
|6|star|0|true|5|136|175|5|null|64.318535|
|6|root-projection|1|true|5|126|175|5|null|88.607345|
|6|nonroot-projections|0|true|5|136|175|5|null|97.455688|
|6|all-projections|1|true|5|126|175|5|null|77.013360|
|6|leaf-closure|0|true|5|123|181|5|null|69.535430|
|6|clique|0|true|5|455|364|5|null|116.931021|

Flow: planned 18, attempted 18, cells with recorded failures 0. All rows retained; no statistical pooling. [pcc:unverified]

Paid costs include generation, preparation, full projection construction even for subset arms, exact identity checking, fresh3CAS references and the complete checked F4 call. Reference/oracle construction and separate diagnostic costs are retained in audit.json; diagnostic time is not production solver speed. F4 matrix nonzeros/fill are unavailable when the unchanged worker does not emit them; affine diagnostic nonzeros are not substitutes. Geometric additions may change the ideal; equal checked complete canonical bases determine the label. Capped closure/Hilbert is not an intrinsic regularity or impossibility result. Star leaf projections may be absent; duplicate/effectively identical arms remain. [pcc:unverified]

![Observed degrees](report-01/degrees.pdf){width=90%}

\newpage

# Matrix, certificate and paid-cost detail

Read-only Rust formatting of the audited18 fixed development observations. All times are single calls in milliseconds; no timing confidence or speed claim. F4 matrix nonzeros/fill are absent from this worker. The terminal affine diagnostic nonzeros and row reductions below belong to the separate closure algorithm. [pcc:unverified]

| n | Class | Terminal degree | Diagnostic new row nnz | Diagnostic reductions | Projection terms | Projection product degree |
|---|---|---:|---:|---:|---:|---:|
|3|star|5|2975|4647|null|null|
|3|root-projection|5|2646|3912|80|14|
|3|nonroot-projections|5|2975|4647|null|null|
|3|all-projections|5|2646|3912|80|14|
|3|leaf-closure|5|2737|4455|null|null|
|3|clique|5|2737|4455|null|null|
|4|star|5|10998|27141|null|null|
|4|root-projection|5|9600|21900|80|14|
|4|nonroot-projections|5|10998|27141|null|null|
|4|all-projections|5|9600|21900|80|14|
|4|leaf-closure|5|10407|25502|null|null|
|4|clique|5|10441|27531|null|null|
|6|star|5|71359|270221|null|null|
|6|root-projection|5|61241|214965|80|14|
|6|nonroot-projections|5|71359|270221|null|null|
|6|all-projections|5|61241|214965|80|14|
|6|leaf-closure|5|67326|243645|null|null|
|6|clique|5|72754|334471|null|null|

| n | Class | Generation | Full construction | Preparation | Identity check | All3 refs | Checked F4 | Fastest external | External ms | Separate diagnostic |
|---|---|---:|---:|---:|---:|---:|---:|---|---:|---:|
|3|star|0.021202|0.016515|0.175683|115.598078|71.122827|9.587289|"msolve"|14.894869|41.385840|
|3|root-projection|0.035013|0.916065|0.353080|14.555313|38.596311|6.006418|"msolve"|11.077903|23.711662|
|3|nonroot-projections|0.030866|1.038529|0.413411|13.511006|35.712104|5.960689|"msolve"|7.725589|40.601475|
|3|all-projections|0.032008|0.912730|0.978949|15.801756|36.403629|5.592662|"msolve"|9.278442|21.350191|
|3|leaf-closure|0.119890|0.136735|0.707553|13.718707|37.122898|6.140680|"msolve"|8.154463|28.416556|
|3|clique|0.039610|0.017266|0.315123|15.907123|45.772350|7.600041|"msolve"|14.953437|34.371528|
|4|star|0.038508|0.020421|0.187271|18.781779|35.467077|5.252196|"msolve"|7.595034|291.808108|
|4|root-projection|0.042394|1.164829|0.830387|25.392501|49.565790|5.042932|"msolve"|9.953804|144.221229|
|4|nonroot-projections|0.041432|1.600223|0.209454|22.403262|42.259695|6.712630|"msolve"|8.383196|274.920181|
|4|all-projections|0.041823|1.079491|0.285749|16.325632|47.163738|6.532579|"msolve"|12.889658|136.702395|
|4|leaf-closure|0.046009|0.023716|0.210626|15.749677|40.675357|6.000068|"msolve"|9.207064|278.368064|
|4|clique|0.058798|0.348504|0.187020|53.509056|61.844490|10.626356|"msolve"|13.791757|191.400917|
|6|star|0.054241|0.176996|0.303416|14.326610|41.608888|7.848384|"msolve"|9.403199|4762.740608|
|6|root-projection|0.060201|2.497069|0.758188|33.942058|44.115326|7.234503|"msolve"|10.543676|2061.423511|
|6|nonroot-projections|0.145829|1.830839|0.561212|30.041666|55.484862|9.391280|"msolve"|12.815703|4555.716391|
|6|all-projections|0.061803|1.847814|0.514092|27.668732|40.535512|6.385407|"msolve"|8.777469|1914.422171|
|6|leaf-closure|0.095773|0.019940|0.238248|15.110201|42.153760|11.917508|"msolve"|8.829240|5360.243476|
|6|clique|0.124967|0.168943|1.015805|29.993431|72.460792|13.167083|"msolve"|15.467332|4255.594863|

Reference clocks include process launch, basis parsing and route checking. The checked F4 clock includes process launch, the unchanged native solver/checks and full equality consumption. These whole-route clocks are descriptive and are not a controlled timing comparison against the fastest external route. Full paid costs also include all three reference routes. Oracle construction is the separately recorded reference fee already included in paid F4 cost. Diagnostic child time is recorded outside that paid cost and is never counted as an F4 speed ratio. Projection product degree measures original-input identity products, not solving degree. [pcc:unverified]

![All matrix observations](details-01/matrices.pdf){width=90%}
