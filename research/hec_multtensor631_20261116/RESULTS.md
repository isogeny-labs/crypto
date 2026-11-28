# Compiled target quotient multiplication and dual colon maps

One correctness execution passes, exit0/stderrempty. Sourcefe390c895615/checkpoint20261116T034008Z precedes execution. Cache built from actual quartic in the run:66degreepairs through degree10,20132sparse entries, below the frozen1millioncap. No old compiled map is loaded.

Every compiled form product is compared coefficient-by-coefficient with the original normal-form multiplication:283914comparisons pass. Every dual-contraction colon result is compared against the original canonical nullspace:2665comparisons pass. Counters include operations invoked by reference-check work and are not claimed production operation counts or avoided-field-operation counts.

Actual direct-output reconstruction preserves both unique prime-subgroup classes byte-for-byte against the prior independent replay. All primary/adjacent/sign/corruption checks and both charts pass. The checked section shortcut executes48generic evaluations plus one degree2fixedbase fallback. No class/scalar oracle enters reconstruction; post-run byte comparisons are validation.

Mechanism: compile quotient-ring basis multiplication into sparse structure coefficients mu(i,j,k), eliminating inner normal-form HashMap lookups. For each colon call, compute dual bilinear matrices from its annihilator rows once; apply them to all supplied forms. The rearrangement is justified by linearity and finite-sum interchange. Nullspace normalization/group semantics remain unchanged.

Dual.lean checks the contraction identity over arbitrary commutative rings and finite index types. Lean4.19/mathlibc44e0c8ee63ca166450922a373c7409c5d26b00b; exit0; standard axiomspropext,Classical.choice,Quot.sound. It does not formalize Rust, the normal-form compiler, nullspace correctness or full geometric correspondence. Exact finite implementation comparisons are separate evidence.

This build always executes reference comparisons and cannot measure speedup. Precomputation and dual-map construction must be charged in a later production measurement. No novelty, generic complexity,10x, cold end-to-end or genus2/4claim. The earlier1.621xcomponent record and drift-affected0.930cohort remain unchanged. Next remove comparison-only work in a production artifact, retain exact class/base controls and use stable same-host phase attribution before timing claims.
