# Explicit public prime-subgroup transport passed

V2 source9ef29556c435/checkpoint20261115T163443Z preceded one release execution.
Exit0, empty RUN_V2.stderr. All six nonzero public inputs mapped separately;
all images nonzero and annihilated by ell1028893. Identity maps to zero.
Destination addition, doubling, negation and withheld1729scalar control passed.
The mapper received public divisors and logging labels only, with no label branch,
expected output or input discrete logarithm. Lambda10 was fixed from sourcejet13,
not fitted to these images. Full source and target representations retained.

| Public input | Trials to first admitted zero | Scalar-regular | Section zeros | Scalar-chart exclusions | Nonsplit exclusions |
|---|---:|---:|---:|---:|---:|
|D1|853|841|5|12|4|
|B=2*(3,21)|38|37|1|1|0|
|D1+B|134|132|1|2|0|
|2B|19|19|1|0|0|
|-B|38|37|1|1|0|
|1729D1 control|103|102|1|1|0|

These are counted trial indices, not runtime measurements or a performance
comparison. Inputs/arms are algebraically related, not statistical replicates.
Some images use the same selected difference point (B and2B), so doubling alone
would be a weaker control; the separate sum and scalar-control scans use different
points. The scalar1729 is known only to the outer checker after mapping inputs;
it is not an algorithm argument or reconstruction input.

PROOF.md gives the extension: for decoded Q=epsilon*psi([n]D-T), set
M(D)=[(2n)^(-1) mod ell]([2]Q)=epsilon*psi(D). The same global sign is used for
both DLP public inputs, so it preserves scalar equations. Kernel degree27 is
coprime toell, hence this restriction is injective. This is a bounded evaluator
on the six executed inputs plus an algebraic identity for every successful
input; no universal success bound or independent implementation is asserted.

## Retained V1 failure and V2 amendment

V1 sourceb20b27da82a9/checkpoint163337 exited101 before mapping: its chosen
B=2*(0,44) was exactlyD1. RUN.txt/RUN.stderr andV1-RESULTS.md remain unchanged.
PROTOCOL-V2.md changed only the input to twice factor_base[1], here(3,21), and
kept the inequality assertion. New main_v2.rs/map_v2.rs and new raw filenames
make the two executions distinct. No failed cohort was silently rerun or replaced.

## Next complete-cost work

Transport is now concrete for this same-field genus3 quotient. Next integrate
the plane-quartic relation engine, verify the scalar on the source and compare
complete costs against the same-target hyperelliptic engine. Charge kernel/setup,
orientation calibration, all scan trials and exclusions, source jets, destination
arithmetic, linear algebra and verification. The density-order1/q scan heuristic
is not a measured scaling result. Genus2/4 mechanisms and the10x objective remain
open. No old record, default, canonical plot or leaderboard changed. New graph
TRIALS.svg plots raw trial counts; no performance record is created for this gate.
