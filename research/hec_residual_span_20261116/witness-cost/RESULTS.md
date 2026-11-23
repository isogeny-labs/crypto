# Matched-prefix Krylov cost: slower on every panel

Pushed freezeccc5d0e1fc6, single invocation exits0; frozen hashes pass.
18panels,72alternatingcalls,144modulusrecords; allcalls complete and result/
work/countvectors repeat exactly. Target scalars and diagnostic original/
Schur/nativeFast/witness/operator checks pass. No bounded failure here.

Both arms get first originalSparse-completing prefix, selected counterdisabled,
an optimistic stopping oracle for Krylov. Krylov pays graph rebuild,pivot
normalization,RHScorrection,allprojections/operator passes/witness verification
and final target multiplication. GeneralSparse pays original matrix setup/
elimination and same finaltargetverification. Discovery/preprocessing excluded
from BOTH; this is a known-field solver component, not wholeICbenchmark.

Every18panel regresses in subgroupREDC: Sparse/Krylov0.015780-0.270930,
equivalently Krylov uses3.691-63.372times the work in that counter. Exact
integers in SUMMARY.tsv. Examplep601source1,68seedlayout:4748versus300888.
Basefield full vectors are identical between arms for all18panels because
both verify the same scalar with the same group multiplication. No arbitrary
weights combine fields, and no CPU/wall or universal crossover is claimed.

Completing prefixes contain4-66admitted rows (excluding common source row),
so loss is not caused solely by processing all2004retained rows. Dense
recurrence/projection passes and graph traversal overhead remain material
relative to efficient early incremental elimination at these dimensions.
No memory result was measured; callback-only storage guarantee is separate.

Decision: do not install this backend as default on these18systems or infer
10xfrom smaller seed dimension. Preserve it as a qualified bounded low-storage
option; a larger-size crossover needs an explicit cost case before campaigning.
Scaled producer/unknown-action integration remains unqualified. Original
genus2+10xgoal unresolved; genus3/4 not covered by these field-system panels.

Next concrete audit: geometric primitive-root search in existing setup is
linear candidate testing. A power-map construction and canonical primitive
orbit enumeration may replace it while preserving chosen root exactly. This
is a standard group/engineering optimization, not yet qualified or priced;
any comparison must optimize control equally and retain previous records.
