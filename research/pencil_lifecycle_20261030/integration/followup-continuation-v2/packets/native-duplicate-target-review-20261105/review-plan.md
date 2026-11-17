# Prospective independent source review: native signed duplicates

T-1618. Written before reading the producer patch. Root has read the existing canonical collector duplicate branch and the producer's proposed design. This review is independent of patch authorship, not blind to the design. No compiler, solver, experiment or timing run is authorized here. Outcome remains source-only/inconclusive pending machine qualification. [pcc:unverified]

## Claim and prior

The opt-in collector retains the first canonical signed label pair and tests subsequent equal signed divisors for a target equation. It returns a duplicate-derived scalar only after multiplication on the original curve matches the target. Prior: algebra is plausible; sign orientation, scalar-ring hypotheses, cost counters and early-return paths require source inspection. Preserve the original default algorithm, full cohort, controls and complete matched costs.

## Independent derivation before patch reading

Let Q=kG and D_i=(a_i+b_i*k)B in the Jacobian. If canonical(D_i)=epsilon_i*D_i with epsilon_i in{+1,-1}, equality of canonical representatives gives ((epsilon_1*a_1-epsilon_2*a_2)+(epsilon_1*b_1-epsilon_2*b_2)*k)B=0. On a nonzero prime-order-r image subgroup, coefficients yield delta_a+delta_b*k=0 mod r. If delta_b!=0, k=-delta_a/delta_b. Both labels must change sign together. The actual source check kG=Q supplies candidate acceptance even where the image-order premise has not been separately established; it does not establish the collision distribution or cost advantage. If delta_b=0, retain without inversion; delta_a=0 is redundant, delta_a!=0 contradicts the stated subgroup/label premises and must not be called a solve.

## Joints and worked attacks

1. Sign/RHS: choose D2=-D1, (a2,b2)=(-a1,-b1); after normalization both deltas must vanish. Flipping a alone gives a false equation.
2. Distinct labels: enforce epsilon1*a1+epsilon1*b1*k = epsilon2*a2+epsilon2*b2*k; check subtraction direction and minus sign in recovery.
3. Field premises: delta_b=0 must never invoke inversion; composite r makes nonzero insufficient and requires explicit prime-ring contract.
4. Source acceptance: inspect kG=Q on actual original G/Q, never fixture secret k or transferred points alone. Mutation/observer failure must not bypass the check.
5. First-label state: no replacement with a later record; labels and first attempt remain aligned; canonical key and orientation use the same computed negation.
6. Default/cost scope: false opt-in retains previous branch decisions and no extra allocations/arithmetic; enabled mode charges normalization, map comparison/storage, candidate scalar arithmetic and original-curve verification in correct phase/counter ownership. No unmeasured cost is asserted zero.
7. Collector coupling: recovered return is consistent with outcome flags/counts, observers, cleanup and full verification paths. No new graph edge is inferred solely from duplicate detection.

## Acceptance boundary

Bind exact committed/published candidate and base hashes before final source review. Record findings and source corrections separately. No passing runtime verdict until compilation, actual native positive/negative controls, input invariant binding, full source suite and prospective matched cost cohort have their own evidence.
