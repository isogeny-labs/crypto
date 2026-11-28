# C82: exact semilinear trace and shared quotient powers

Hypothesis: coefficient-twisted modular composition can replace long binary quotient-squaring chains without losing repeated-root information; sharing power columns between simultaneous actions may overcome composition overhead. The contract is exact canonical T_k=sum_(i<k)t^(2^i) modH. This component contract is not a new complete original root-set computation.

Three immutable90s experiments were registered and run with lab.runner resume/run-ready, at most3 in this cycle,4096MiB per-process address-space cap and one CPU. Sources/input hashes, seed81, field bases, lengths, output boundaries and costs were frozen. C82 and changed C82b both passed1008coefficientwise comparisons with independent convolution,336nonreduced Taylor checks,987literal-point checks and644coefficient-Frobenius map checks. No inputs or losing profiles were removed. The first profile motivated the changed shared-column variant; it was not rerun identically.

Omitting coefficient twisting is falsified over GF4=F2[alpha]/(alpha^2+alpha+1), H=X^2, t=alpha, k2: exact trace alpha+alpha^2=1, whereas untwisted composition gives0. Frobenius remains valid on nonreduced quotients; only coefficient exponents are reduced using field degree. An odd-characteristic sigma(a)=a^p proof is included, but no odd-characteristic implementation or NIST performance gain is established here.

The first kernel composes T and P separately. C82b builds powers of the same P once and applies them to two twisted coefficient vectors, preserving singular maps/nilpotents, and omits unused final P states. Both pay field-map construction anew in each profiled call. Below, field multiplication and square counts are summed for compactness, not weighted as equal runtime cost. Bit-map XOR work is additional and retained separately.

|Field|degH|k|Sequential field ops|Separate composition ops|Shared composition ops|
|---|---:|---:|---:|---:|---:|
|GF2^2|1|18|0|9|11|
|GF2^2|1|83|0|11|15|
|GF2^2|2|18|81|28|30|
|GF2^2|2|83|389|46|52|
|GF2^2|5|18|484|411|420|
|GF2^2|5|83|2337|756|779|
|GF2^2|12|18|2015|9844|6768|
|GF2^2|12|83|9564|20646|14434|
|GF2^3|1|18|0|10|12|
|GF2^3|1|83|0|12|16|
|GF2^3|2|18|82|35|38|
|GF2^3|2|83|396|57|61|
|GF2^3|5|18|457|788|492|
|GF2^3|5|83|2189|1508|935|
|GF2^3|12|18|2156|13206|8751|
|GF2^3|12|83|10444|27624|19165|
|GF2^4|1|18|17|12|17|
|GF2^4|1|83|82|14|25|
|GF2^4|2|18|85|32|35|
|GF2^4|2|83|410|52|59|
|GF2^4|5|18|463|918|656|
|GF2^4|5|83|2252|1122|844|
|GF2^4|12|18|2482|13805|9084|
|GF2^4|12|83|12089|19223|12587|
|GF2^83|1|18|17|91|97|
|GF2^83|1|83|82|93|105|
|GF2^83|2|18|85|116|120|
|GF2^83|2|83|410|138|146|
|GF2^83|5|18|493|1072|773|
|GF2^83|5|83|2378|2042|1537|
|GF2^83|12|18|2635|15416|10187|
|GF2^83|12|83|12710|33018|22753|

At GF83/degree5/k83, sequential uses1968multiplies+410squares=2378; separate composition uses2042field operations; shared uses1437multiplies+100squares=1537, plus18397map XOR steps. Shared reduces this exposed field-operation count35.37% versus sequential and24.73% versus separate; this is not a count of all machine work. Shared diagnostic time was0.308976908s sequential and0.248081541s candidate under tracemalloc: single instrumented in-worker calls without qualified isolation, A/A or cold repetitions. They establish no end-to-end solver ratio. Case generation/process startup are outside component stopwatches; field-map/table preparation and quotient work are inside. Whole-worker costs are reported separately.

Tracked Python peak allocation rises1960->30716bytes in that comparison. Largest shared-panel tracked peak is37912bytes. Tracemalloc excludes some native/total-process memory. Per-worker process/ru_maxrss observations are preserved separately, with process/import/pre-exec measurement scope caveats. The cap is per-process address space, not simultaneous descendant-tree RSS.

Short traces and degree12 remain losses; no universal promotion. At GF83/degree5/k18, field ops493->773, instrumented time0.061617712->0.146107492s. At degree12/k83,12710->22753ops. All32profiles per variant remain. Existing finite-subspace support takes18stages; accelerating an83stage rationality trace does not itself beat support or reduce the73free affine norm parameters.

Primary-source review: NTL GF2EXFactoring.cpp at be43be3554f366f3710d4121323ba67a5a256c96 already contains sequential TraceMap, coefficient-twisted ComposeFrobeniusMap and cost-dependent dispatch, and notes composition-based trace as an option. PRIMARY_SOURCE.json links/quotes it. Basic Frobenius composition is prior art; no novel general-rootfinding or literature-clearance claim.

Decision: retain the exact shared kernel as experimental. Do not make it default, multiply diagnostic effects into other microbenchmark gains, claim hardness, or retry failed A/A calibration. Original all-roots search, matched genuine F4/F5/F6,20hard instances and50x stretch goal remain active/unpassed. NEXT.md specifies a changed complete-pipeline experiment.
