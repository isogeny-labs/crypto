# C89: generic finite-kernel lifting and full rational3quotient

Three frozen experiments executed,all passed,4096MiB per worker,one pinned core,90s per experiment;source/input/command hashes frozen before evaluation. The old C88failed and dependent pending plans remain unchanged. This is a research checkpoint;the50xstretch goal and frozen2xrelease gate remain active/unpassed.

Hypothesis1: backward finite-kernel state pruning handles arbitrary kernels,partial fibers and original infinity-prefix guards with exact coverage. Proved label-path contract and implemented reusable group-table engine. Independent Cartesian-product oracles passed144cases overC2,C3,C3xC3. Independent literal S3enumeration passed32curve/target/arity cases overF3,F5,F7,GF4,GF8(two binaryAvalues),GF16,GF9,GF27,including rational2torsion,C2xC2,C3xC3,repeats,partial fibers,empty outputs and finite intermediates. Across those32cases,371distinct full original polynomial roots were compared;454point-lift paths map onto these sets. The distinction matters:torsion/signs can duplicate x-vectors. Point-level tests do not establish unrestricted nonrational-source coverage.

On the unchanged C88positive P224cases,trellis matches all81roots but gives no clear latency improvement over direct cyclic3lifting. Cold medians kernel/trellis:6points .031915/.032968s;12points .034072/.034480s;24points .052730/.052320s. Keep direct cyclic3default;generic DP is a correctness/coverage extension,not a promoted speed optimization.

Hypothesis2(changed assumption): use all9rational3torsion points on the certified P224twist. The quotient is[3],not a cyclic9kernel or new-j auxiliary curve. Fresh domains contain18,36,54signed original points,closed under fullE[3]. Actual membership,full rational kernel,andpoint-map setup charged;prepared source-domain construction and curve/order proofs are input preparation. Original prefix-domain problem is unchanged and not solved by this domain redesign.

All3methods complete and agree on6561full original source/intermediate roots for the18point plantedpositive case and0for both fixed-negative cases. Each returned root rechecked against every originalS3equation.3cold repetitions/method/case,27children,with startup,JSONinput,validation,map/fiber setup,join,all lifts,root dedup,originalverification,fullJSONandprocess exit charged.

|Case|Reference group join s|Cyclic3cached s|Fullkernel9 s|Cyclic3/fullkernel internal ratio|
|---|---:|---:|---:|---:|
|P224_full3kernel_18_positive|0.998281201|0.38576761|0.363730469|1.061|
|P224_full3kernel_36_negative|2.14113277|0.111711324|0.038132249|2.930|
|P224_full3kernel_54_negative|7.13890671|0.293361045|0.052215308|5.618|

Fullkernel/reference internal ratios are2.745x,56.150x,136.720x respectively. These are specialized group-join comparisons on exposed small orbitdomains,NOT genuine F4/F5/F6ratios. The positive6561root case shows output/verification cost:only1.061x versus currentcyclic3. Empty-instance gains alone are inadequate for a general rootfinding claim. No cases discarded or extrapolated to other curves.

Memory:trellis comparison18cold children reports wait4peak12672–12800KiB. Fullkernel comparison27children reports22132–38904KiB(21.61–37.99MiB). Both observer and child self peakRSS retained;the observer forks from a parent that retains earlier outputs,so inherited high-water memory/parent output retention can inflate later child peaks. These are whole-process diagnostic measurements,not isolated algorithm-only memory. Address-space cap4096MiB per process,not a sum across descendants. HostA/A/exclusive isolation is not qualified.

Novelty review pinned Sage primary implementation cf0ebedcef1363d6339df14cf6bb32bc2cc275b9:finite-kernel isogenies and Velu/Kohelalgorithms are established. This checkpoint does not claim a new isogeny or mathematical novelty clearance. The proposed combination of exact original guards,finite-state fibers and outerquotient search needs comparison with literature and stronger specialized methods.

No20preregistered hard matchingGBinstances,no genuine full matchingF4/F5/F6triplet,noall-family speed evidence. GF(2^83)target and NISToriginal prime-order domains remain separate open streams. A generic field backend and tinybinary/oddextension oracles are correctness evidence only.

Next:bounded finite-domain original polynomial encoder and independent all-root baseline integration;then select a fresh nonempty/empty domain ladder with all preprocessing charged. Investigate a cost-based sparse-block quotient dispatcher:fullkernel for noncyclic compression,direct last-label for smallcyclic kernels,identity quotient when originalrational domain has no nontrivial kernel. Generic group kernel assumptions do not give compression on prime-order NISTgroups. RestoreC84exact residue-cover search for actual GF(2^83),with complete table-XOR branch coverage and explicit unresolvednode/memory caps. No more experiments inC89.
