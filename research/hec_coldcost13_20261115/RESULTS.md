# Complete cold-cost gate failed on the registered genus3 instance

All20cells succeeded and verified k183049 for the same public source pair.
The numerical10xgate failed. Frozen sourceedf84d571f5b and pre-run checkpoint
20261115T185949Z preceded the single write-once cohort. No rerun or arm dropped.
Record: benchmarks/records/20261115T1859Z-hec-coldcost13.json.

| Arm | n | Median core ms | Observed min–max ms | Median process wall ms |
|---|---:|---:|---:|---:|
|Cold quotient+mapped IC|5|1780.974708|1706.933666–1938.462791|1784.938625|
|Native Compact|5|0.797166|0.434917–1.196125|5.186084|
|Native Fast|5|1.452792|1.267083–2.768875|5.570083|
|Native Hybrid|5|1.188166|1.148458–2.023333|5.494542|

Primary derived ratio best-native/cold=0.00044760096615589, threshold10.
Compact is the fastest native median under the preregistered selection rule.
Equivalently cold/best=2234.132800445579. This fixed implementation/case would
need a22341.328004455783factor reduction in its measured core cost to meet10x.
These observations are local ARM L0/unisolated, producer unvalidated. Five
repeated timings of one input pair are not five independent curve experiments.
Observed extrema are not confidence intervals. No broad scaling/novelty claim.

Core includes public input validation, source pointcounts/Weil recovery,
field/kernel/normalization/quotient reconstruction, orientation calibration,
actual maps, relation solve/descent and final source/destination verification.
It excludes final certificate/measurement emission and process startup;
secondary process wall includes both. Algorithm log output within the core is
charged. Every arm uses one binary and identical public-input validation.
The unrelated sharedresume caused commit_dirty=true; measured scientific
source was committed and binary/input fingerprints are retained.

## What consumes the candidate cost

Median phases in milliseconds (not additive medians): sourceWeil4.234041,
field/kernels55.060125, Weil/normalization17.316625, geometry113.973750,
points/Jacobian0.721125, relations2.085917, mapGenerator1382.098958,
mapTarget152.435708, descent25.244334, verification23.568708; input0.145417.
The generator scan is dominant, but it is not the only gap.

Post-run descriptive analysis in analyze.rs computes each cell's total minus
its two maps before taking the median:240.507208ms, extrema230.574375–403.950541.
BestNative/residual=0.00331452020348596. The recorded relations+descent+verification
alone have median52.211917ms, extrema47.902166–76.139958, ratio0.01526789372625410.
Thus optimizing maps alone cannot meet10x if the measured remainder is unchanged.
This is a counterfactual bound on these recorded costs, not a universal theorem
about a different algorithm. It was not a preregistered success criterion.

## Decision and custody

Reject10x for this cold implementation at this instance. Do not promote the
314relation/47column count improvement into an end-to-end gain or continue
small map changes as though they alone could close the measured gap. A next
mechanism needs a cost case that changes construction, transfer and/or destination
arithmetic together, or a rigorously justified different admitted regime.
Genus2/4 and the full mathematical10xobjective remain open.

All20stdout/stderr files, cells.tsv, input-onlyPUBLIC and summary.txt retained
in run/ and archived as raw-coldcost.tar.gz for content-addressedS3 storage.
Rust collector/reducer produced the statistics/plots; initial standalone analyzer
compile used the default old edition and failed, then compiled with --edition2021;
no measurement was repeated. PDFs/graphs/benchmark entry are producer evidence,
not independent validation. Prior closed records/defaults/leaderboard untouched.
