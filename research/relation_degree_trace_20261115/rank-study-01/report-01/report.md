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
