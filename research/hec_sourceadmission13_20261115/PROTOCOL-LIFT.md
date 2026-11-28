# Full Weil-polynomial recovery for every admitted source

Sourceadmission38958f652b07/checkpoint20261116T010615Z executedonce, admitted5of16fixedcases. Include ALL5certificates/publicinputs in run/, sortedbyfilename, no adaptivechoice. Largerfields1039/2011/4003are source-only admissions, not mapped DLPs.

For each admitted source, use unchanged generic recurrence/residue/lift code from hec_coldfiber13_20261115/weil_recovery.rs, with actualf,p,ell. Independently recompute the entire first3pcoefficient prefix of f^((p−1)/2)by repeatedFp-polynomial multiplication and require exact equality to p-adic recurrence. Then recover L by the existing exact Weilresidue/coarse/endpoint/rootbracket gate. All previous unsupported/ambiguity assertions remain; do not silently discard a candidate without an exact reason. Require L(1)=certifiedJacobianN and p+1+a1=independentlycountedN1. No independently measured N2orN3claim.

Newsource/lock/binaryhash and all5inputs committed, freshS3checkpoint/gate before execution. Separate tmp/target-liftpreservesoriginaladmissionbinary; write-once lift-run/,percase stdout/stderr/status, controllerLIFT.txt/stderr/exit. Stop firstunexpectedfailure;retainalloutputs,no scientificrerun withoutamendment. Independentcoefficientreferenceisquadratic-workcontrol,not the recurrence's claimed implementation complexity; charge it in any later timedstudy.

No kernel/alpha/quotient/mapconstruction, baselineDLP, timing, 10x, genus2/4orabsolute-simplicityclaim. This establishes new sourceWeildata for later actualtransfer work, not a different successcriterion for the originalobjective.
