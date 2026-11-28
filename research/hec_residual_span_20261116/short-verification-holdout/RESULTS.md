# All-genus complete substitution passes correctness; count gains are not uniform

[pcc:unverified]. Freezec021e49f65c; input hashes pass, sole120CPU-bounded Rust invocation exit0. All24original holdout publicsource/target/known-scalar receipts match oldstderr exactly.192complete fresh solver calls across four modes and two repeats recover native-verified scalars. Every base-field16vector equals old complete minus measured scalarverification plus measured polynomialverification; all subgroup vectors unchanged. Repeats/outcomes exact. Source R/S operator and wrong scalar/source/target controls pass. No model, target, collector, anchor, seed or budget changed.

## All96per-arm/workload results

Ratios are old baseREDC/new baseREDC; below1is a regression. SUMMARY.tsv retains every arm/workload, BEST.tsv retains strongest complete arm on each side, with no target pooling.

|genus|complete-count ratio range|improved arm/workloads|verification-kernel ratio range|
|---|---|---|---|
|2|0.972937-1.084701|24/32|0.626481-2.105769|
|3|1.090295-1.813890|32/32|2.211232-9.978479|
|4|0.835746-1.281463|16/32|0.617501-2.179041|

All eight genus-three public source/target workloads improve in all four arms. Both genus-two fields have one source/target regression each, repeated across four arms. Genus-four p181improves on all four workloads; p271regresses on all four, in all arms. Worst complete regression0.835746means about19.65percent more baseREDC, not a gain. No complete case reaches10x. Kernel ratios and arithmetic vector changes are not walltimes; integer reduction/allocations not priced by counters.

Strongest complete-arm ratios per field across four workloads:

|g,p|old strongest/new strongest ratios|
|---|---|
|2,251|1.084701,1.061888,0.973465,1.047674|
|2,1021|1.013568,1.039257,1.003572,0.972937|
|3,211|1.369122,1.486152,1.383137,1.559264|
|3,421|1.590863,1.813890,1.499333,1.714984|
|4,181|1.281463,1.123100,1.208820,1.174661|
|4,271|0.995527,0.835746,0.933425,0.928155|

These six-field inputs differ from original prime-order genus-two source-phases fixtures; the earlier1.239-2.471complete ratio range remains valid ONLY on those earlier12workloads. This holdout extension therefore refutes extrapolating uniform benefit from that range. The dense polynomial evaluator can lose on small-subgroup/high-degree-source combinations; no universal explanation or exact threshold is proved from six fields.

## Contract and decisions

The conditional binomial Frobenius-operator premise is the same admitted action premise used in preparation. Source/operator controls and external nativeverification are diagnostic outside candidate counts, not an independently qualified verification pipeline. Genus-four models remain retained point-generated subgroups, not generic simple4dimensional Jacobians or wholeJacobian orders. Shared arithmetic and independent-review gaps retained.

Retain BOTH scalar and short verifier versions as comparators. Do not replace scalar unconditionally or hide regressions by a post-hoc field/case fallback. A prospective public-input cost rule would need a separate protocol; choosing the minimum measured count per workload is a descriptive comparator envelope, not an implemented selection algorithm. Future producer claims must beat available stronger verifier choices while respecting same input and validation contracts. This is a useful integrated improvement in part of the panel, especially genus3, but is classical endomorphism scalar decomposition and not the10xmathematical breakthrough.

Next work must address relation-generation/preparation cost with the stronger comparator set, or construct a materially different geometric mechanism. Merely tuning the verifier to one field/component does not satisfy fullgenus2+objective. Current raw/certificates/full vectors/hashes remain immutable.
