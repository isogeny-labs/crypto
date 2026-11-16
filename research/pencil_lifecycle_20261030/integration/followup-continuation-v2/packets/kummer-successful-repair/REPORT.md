# Successful geometric repair and explicit evaluation masks

The fixed software checks cover 84 valid geometric repair cases across21 retained models. All84 cleared mixed additions initially return zero, then torsion-translation repair succeeds and agrees with direct signed Cantor addition. All84 selected eager/pruned pairs and evaluation/selection-mask checks agree. Signed recovery equals the expected signed divisor in80 cases; four p739 cases return general_conversion_inapplicable. The prospective requirement was all84: the overall check exits101, preserved without dropping failures.

## Construction and scope
Take each pinned exceptional identity witness T with two zero Kummer coordinates. T is a valid rational2-torsion divisor. From retained Q form the adjacent pair(Q,Q+T). The mixed result is2Q+T. Verify2T=0, validity, rG=rQ=0, and the explicit word bounds before checks. Recover a resulting point R using the original non2-torsion signed G and the pair(kappa(R),kappa(R+G)). This external anchor is essential: the CCS paper excludes2-torsion anchors. These fixtures qualify repaired arithmetic and signed-recovery composition, not a collector with a torsion sign anchor.
Sources: Chung-Costello-Smith, sections2 and6.4, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf; pinned historical N19 main_labelled.rs and walk_impl_uncached.rs. Existing84 ordinary/forced-fallback checks are separate and unchanged.

## Telemetry candidate
The additive candidate carries evaluated[3] and selected[3] alongside repair roots. Eager evaluation is[true,true,true]. Pruned evaluation equals selection: [true,true,false] for branch0 and[false,true,true] for branch1. Unexecuted now differs from evaluated-without-recorded-repair. Option output still distinguishes failure from success. Fixed cases cover52 branch0 and32 branch1 decisions. Native per-case walk counter reductions are24M,36A,4I, not a timing or complete collector cost claim.

## Retained recovery failure
All four failures yield the same degree1 divisor on p739: u=[362,1,0],v=[303,0], for exceptional indices8,12,16,20. Both versions fail identically. Generic-coordinate conversion rejects a zero first homogeneous coordinate, although the forward map explicitly represents degree1 as(0:1:x:x^2). This provides a concrete next construction: decode that boundary chart, enumerate its at most two curve lifts and use signed anchor addition to choose uniquely. Identity, zero vector, nonsquare and ambiguous anchors must remain explicit.

## Reproduction
PLAN.md freezes selection and assertions before execution. Source/fixture hashes are in pre-execution-manifest.json; receipt.json retains the bounded locked/offline command and exit101. No new scientific solves, cloud operations, runtime updates or publication occurred. Candidate patch remains private and requires independent review.
