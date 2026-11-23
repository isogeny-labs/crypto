# Two cheap cubics lower the observed F4 degree

The fixed early-rank study lowers actual Rust F4 degree from5to4 on all three known development dimensions n3/4/6 over F2147483647,seed7. The selector retains two directly certified syzygy cubics; equal-count difference controls add no rank and remain at degree5. All18 complete Rust bases match fresh Singular std/slimgb/msolve bases, and all affine ideals match their dimension-specific star. PCCedfc6c. Producer evidence remains unsigned. [pcc:unverified]

| n | Star F4 degree | Selected F4 degree | Star max rows/columns | Selected max rows/columns | Exported degree4 rank gain |
|---|---:|---:|---|---|---:|
|3|5|4|34/37|26/30|8|
|4|5|4|71/71|26/31|10|
|6|5|4|136/175|78/73|14|

Each retained cubic has10 polynomial terms,8 original-input proof terms and certificate product degree6. Both selected rows total16 proof terms. The equally counted difference control is not equally sparse:it has16 total polynomial terms and4 proof terms versus selected20/16; exact support/certificate sizes are reported rather than assumed equal. The previously tested quartic route had80-term degree14 certificates and kept F4 degree5 on this grid. This observation supports studying early independent contribution, not sparsity alone. No reduction in discovery-inclusive maximum algebraic degree follows from degree4 F4 matrices:the original-input identities still have product degree6. [pcc:unverified]

All syzygies and both-family arms also reach F4 degree4. The budgeted two-row arm is smaller at n6 (78/73 versus all-syzygies134/95). Affine closure thresholds separately drop5to4. Top-form first vanishing differs:some full syzygy/both systems have finite values within the fixed caps, while the selected n4/n6 systems retain null. Thus the actual solver improvement does not establish an intrinsic regularity decrease or require a finite observed top-form threshold. [pcc:unverified]

Timing remains descriptive. The run contains one full checked F4 call per fixed cell, and prices generation, the bounded build/selection child, candidate construction/verification, exact rank scouting, preparation, fresh Singular identity checking, all3 reference routes and complete solver consumption. The selected n6 checked F4 call is5.616800ms versus star9.576815ms, but build/selection is10.582751ms versus star7.617232ms; all individual full-paid clocks remain in the raw report. Different external-check timings and single calls prevent a confirmed timing gain or15x admission. Earlier cubic-only timing limits in FINAL_REPORT.md remain applicable; this is a degree/matrix component result. [pcc:unverified]

The next hypothesis is to avoid per-input rank scouting with a fixed structural rule. Every selected development result uses candidate ids1and3, the first two pair syzygies. A prospectively frozen first-two rule can construct these identities directly and preserve ideal equality without running the rank selector; exact final checks and a paid fallback policy would still apply. It must be tested against the dynamic selector and stronger existing constructors with matched repeated timing before any speed claim. This is an unmeasured successor, not a fresh-family conclusion. [pcc:unverified]

Method: RANK_BUDGET_PROTOCOL_02.md (PCCd8ef6a) was frozen before data; the original tag-correction subject1d2f7b is retained. Source/config/47test/lint/build and oldn3/matrix/corrupt-rank qualification (PCCe54197/e63246) were published as commit9534b33b before the registered development run. Production algebra and actual-root F4 executable are unchanged. The reporter recomputes every rank decision and identity from raw bytes; no completed basis or witness enters selection.

Sources checked (KB unavailable): original c1/c0 trace and direct construction in README/FOLLOWUP; FINAL_REPORT earlier cubic timing results; cheap.rs and degree4 closure implementation; primary Caminata–Gorla arXiv2112.05579v2 Preliminaries71–83 for the distinction between bounded closure and actual algorithm observations. Known constructions are reused, with no novelty claim.

Flow:18 planned/attempted/completed development cells,0 failed/excluded/replaced,0 contention,unchanged pins. Six oldn3 correctness cells and4 tiny field-rank matrices are separately retained and excluded from development timings. Complete rawarchives, exact sourcebundles, certificate/rank/cost tables and SVG/PNG/PDF accompany PR4349. LocalCPU0/L0/checkpoint waiver/zero cloud spend. Fresh41011/43013/47017 and original conditional reserved19001/23003/29009 remain untouched by this study. Canonical export tooling remains inaccessible/encrypted; no canonical-schema or signed independent validation is asserted.

\newpage

# Budgeted early-rank relation study

Method PCCd8ef6a, results PCCedfc6c. [pcc:unverified]

Fixed development p2147483647,seed7. Rank gain is a proxy; actual F4, affine closure and top-form Hilbert remain separate. Single-call times descriptive, no speedup admission. [pcc:unverified]

|n|Class|Rows added|Early rank gain|F4 degree|Max rows|Max columns|Affine threshold|Top-form vanishing|Build ms|Paid ms|
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|3|star|0|null|5|34|37|5|null|11.050469|122.360839|
|3|differences|2|null|5|34|37|5|null|5.646810|66.895106|
|3|syzygies|2|null|4|26|30|4|6|5.764846|62.648598|
|3|both|4|null|4|26|30|4|5|4.798841|81.987298|
|3|rank-budget|2|8|4|26|30|4|6|6.299971|90.424604|
|3|count-control|2|0|5|34|37|5|null|7.696192|110.039427|
|4|star|0|null|5|71|71|5|null|6.373160|66.277133|
|4|differences|3|null|5|71|71|5|null|9.025537|84.649690|
|4|syzygies|3|null|4|64|62|4|8|6.746190|59.720022|
|4|both|6|null|4|64|62|4|7|5.878287|93.220912|
|4|rank-budget|2|10|4|26|31|4|null|10.042435|76.203303|
|4|count-control|2|0|5|71|71|5|null|16.809325|98.915466|
|6|star|0|null|5|136|175|5|null|7.617232|121.499685|
|6|differences|5|null|5|136|175|5|null|10.794047|84.695637|
|6|syzygies|5|null|4|134|95|4|null|15.589385|140.332420|
|6|both|10|null|4|134|95|4|null|7.464177|70.073126|
|6|rank-budget|2|14|4|78|73|4|null|10.582751|76.615594|
|6|count-control|2|0|5|136|175|5|null|10.019115|67.576052|

Flow:planned/attempted 18, cells with recorded failures 0. All retained. [pcc:unverified]

|n|Class|Generation ms|Build/selection ms|Prepare ms|Identity check ms|All3 refs ms|Checked F4 ms|Separate diagnostic ms|
|---|---|---:|---:|---:|---:|---:|---:|---:|
|3|star|0.043245|11.050469|0.122484|35.417403|69.084989|6.642249|46.198928|
|3|differences|0.029424|5.646810|0.243816|16.331785|37.310010|7.333261|38.611158|
|3|syzygies|0.033981|5.764846|0.283916|14.107985|37.494673|4.963197|31.164542|
|3|both|0.029675|4.798841|0.391568|16.686462|54.742226|5.338526|24.008517|
|3|rank-budget|0.092068|6.299971|0.285829|22.601043|54.587876|6.557817|32.705952|
|3|count-control|0.036475|7.696192|0.415033|16.851704|76.443775|8.596248|34.845890|
|4|star|0.043435|6.373160|0.134803|16.078724|38.250807|5.396204|362.743997|
|4|differences|0.042995|9.025537|0.995055|19.643682|47.905475|7.036946|268.947866|
|4|syzygies|0.041752|6.746190|0.741764|14.291430|30.879359|7.019527|129.616258|
|4|both|0.038728|5.878287|0.530727|28.267763|54.151869|4.353538|106.423246|
|4|rank-budget|0.037737|10.042435|0.638358|19.330437|40.446956|5.707380|226.943988|
|4|count-control|0.042825|16.809325|0.407281|19.894499|55.094350|6.667186|329.442306|
|6|star|0.067221|7.617232|0.179079|26.122116|77.937222|9.576815|5055.035415|
|6|differences|0.066059|10.794047|0.485109|26.045716|39.777765|7.526941|4358.019241|
|6|syzygies|0.083275|15.589385|1.365881|30.832224|81.979955|10.481700|4775.470279|
|6|both|0.059519|7.464177|0.706401|13.404655|40.623611|7.814763|3296.367832|
|6|rank-budget|0.172239|10.582751|0.832420|16.095610|43.315774|5.616800|2574.348105|
|6|count-control|0.058508|10.019115|0.798209|14.143087|36.130492|6.426641|3969.344406|

Early rank gain is for the actually exported rows. The count-control pays the identical selector but exports differences; hypothetical winning gains remain separately archived. Full discovery/selection/serialization/process/native/external checks and all3 reference fees are paid. Separate affine/Hilbert diagnostics are outside the paid F4 clock; reference/oracle fees remain in it. F4 nonzeros/fill absent, not zero. No intrinsic regularity or15x claim. [pcc:unverified]

\newpage

# Exact exported certificate costs

Values below describe exported rows; unused candidate discovery and rank selection are still fully paid. Product degree belongs to original-input membership identities, not F4 solving degree. All18 cells remain; single-call timing gains are unconfirmed. [pcc:unverified]

|n|Class|Selected ids|Total polynomial terms|Total proof terms|Max certificate product degree|
|---|---|---|---:|---:|---:|
|3|"star"|[]|0|0|null|
|3|"differences"|[0,2]|16|4|4|
|3|"syzygies"|[1,3]|20|16|6|
|3|"both"|[0,1,2,3]|36|20|6|
|3|"rank-budget"|[1,3]|20|16|6|
|3|"count-control"|[0,2]|16|4|4|
|4|"star"|[]|0|0|null|
|4|"differences"|[0,2,4]|24|6|4|
|4|"syzygies"|[1,3,5]|30|24|6|
|4|"both"|[0,1,2,3,4,5]|54|30|6|
|4|"rank-budget"|[1,3]|20|16|6|
|4|"count-control"|[0,2]|16|4|4|
|6|"star"|[]|0|0|null|
|6|"differences"|[0,2,4,6,8]|40|10|4|
|6|"syzygies"|[1,3,5,7,9]|50|40|6|
|6|"both"|[0,1,2,3,4,5,6,7,8,9]|90|50|6|
|6|"rank-budget"|[1,3]|20|16|6|
|6|"count-control"|[0,2]|16|4|4|

![Actual F4 degree](visual-01/degree.pdf){width=90%}

![All matrix dimensions](visual-01/matrix.pdf){width=90%}
