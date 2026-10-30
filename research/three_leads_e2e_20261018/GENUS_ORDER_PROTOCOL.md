# Actual prime-order subgroup cohort (frozen before execution)

The first cohort indexes the cardinality of F_(p²); its actual subgroup
orders are reported separately. To remove ambiguity in the supplied “40 bits”
claim, execute an additional complete cohort with exact subgroup bit lengths
16,20,24,28,32,36,40. Use three Scholten models at each level, sixty independent
targets per model: another 1,260 paired workloads, four arms and 5,040 solves.

For level b, scan primes p=3 mod4 upwards from ceil(sqrt(16*2^(b-1))). Choose
the first three with a Hasse-certified source order N=h*r, prime r with
2^(b-1)<=r<2^b, and h<=64. This eligibility depends only on certified curve
parameters, never solver cost or coverage. Preserve rejected candidates and
all actual cofactors. Do not replace a failed workload. Stop at p=8*2^(b/2)
if three eligible models cannot be generated and report the deficiency.

Use the same v2 collector, cold phases, full direct plain/negation references,
transferred generic Jacobian rho, and target/algorithm seed laws as the first
cohort. Caps are 100*max(p)+10000 per size level. Additional independent and
unfiltered controls use one target per model. Every scalar is checked on E.
All twelve arithmetic and three native-charge cells remain predeclared. The
field is larger than the subgroup; neither bit count substitutes for the other.
