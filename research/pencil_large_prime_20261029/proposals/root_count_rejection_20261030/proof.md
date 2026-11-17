# Root-count rejection reorder: static proposal, not applied

Status: mathematical output equivalence under valid bounded-arithmetic inputs; NOT full observational equivalence. No source edited, patch applied, solver, test, replay or benchmark executed. This is a proposed new instrument, not a repair to frozen evidence.

## Exact source boundary

research/pencil_large_prime_20261029/src/native_solver.rs:
- 804 complement constructs w=v+lambda*u, w²-h, divides by monic u, asserts empty remainder; it also allocates and increments thread-local arithmetic/collection counters.
- 850 point_sieve_candidates calls complement(d,0,h,p) at858 before constructing candidates. Thus every candidate loop entry has already passed monicity/divisibility checks for the same d and h.
- 862-980 ordinary and exceptional candidate paths all feed the same finalized bucket array and candidate set.
- 1311-1327 skips degree!=2 before the sieve; every candidate-verification invocation here has degree2.
- 1330 candidate verification currently calls complement before roots.len()!=3.
- 580-621 measured brackets the whole closure and captures arithmetic, semantic, allocation and clock deltas; the proposed guard stays inside this boundary.
- 1361 graph.relation and subsequent graph.failure cap checks occur only after a Some(points) result.
- src/cryptanalysis/three_leads.rs1139 asserts the Scholten polynomial has length6.

## Mathematical argument

The prior sieve call at lambda0 establishes u monic and u | (v²-h), for this unchanged d,h. For every field lambda,

(v+lambda*u)²-h = (v²-h) + u*(2*lambda*v+lambda²*u).

Therefore u divides the new numerator; monicity is unchanged. None of d,h,p,lambda or the completed buckets is mutated by the roots-length check. On a bucket of length3 the same complement and all polynomial/curve/Jacobian assertions run, with the same points and order, followed by the same graph relation. On any other length both versions return None, increment rejected_candidates once and never touch graph state. Candidate order, walk labels, accepted relations, solution_key and algebraic recovery result are unchanged under these assumptions.

The exceptional branches do not create a bypass: exceptional denominators add candidate lambdas; all-lambda exceptional points may append to every bucket; all of this is finished before candidate verification. The guard examines the final bucket exactly as before. It need not assume every candidate was an ordinary three-point collision, distinct roots or a successful complement geometry check.

## Limits of the equivalence

This is not a proof of identical panics/resource outcomes for arbitrary Rust inputs. The removed complement call contains allocation, monic/remainder assertions and modular helpers using u64 addition/subtraction. For normalized residues with p<=2^63, their a+b and a+p-b are within u64; u128 multiplication handles products. The solve_one interface does not itself impose p<=2^63, so a blanket full-u64 behavioral equivalence is not established. On malformed/overflowing inputs the old rejected-candidate call may panic or diverge from field algebra while the new guard returns None. Prior lambda0 validation alone does not prove absence of lambda-dependent overflow at arbitrary p. Allocation failure, deadline interruption, counter overflow and peak memory behavior may also change; those are intended observable resource changes, not identities.

Even on valid field inputs counters and clocks differ, so this patch is explicitly non-equivalent as a complete counted experiment. Accepted candidates have the same event totals but a changed order (one bucket visit and comparison occurs before complement); rejected candidates lose exactly that complement's events/allocations. The guard's own collection(1,0,0,1) remains once per candidate. No matrix cap is moved or bypassed: both paths already skip graph insertion for a rejected root count. Runtime deadline/resource behavior can still differ because less work is attempted.

## Exact ledger delta on rejected candidates

For this degree2/quintic path, let D be the number of divide_monic loop iterations in the removed complement (1..4); L be trimmed w² vector length; T be the total trim predicate evaluations over pmul trim, numerator trim, D dividend trims and final quotient trim; V be the number of those evaluations on nonempty vectors. All are defined by the original complement's actual execution, not predicted averages.

The removed arithmetic counts are base_muls=12+3D and base_adds=18+3D. There are no inversions and no engine group operations in complement. Breakdown: w construction3M+3A; fixed3x3 pmul9M+9A; numerator6A; monic division3D M+3D A.

Removed semantic primitives: initialized elements18 (w3, product5, numerator6, quotient4), copied elements D (leading dividend coefficients copied to quotient), entry visits33+L+7D+V, key/predicate comparisons6+D+T. The event definitions are those actually instrumented by meter::collection; no extra API-call charge should be added. Counts of allocator calls/bytes, time and high-water marks must be newly observed, not inferred from these primitives.

Phase candidate_verification and candidate/global totals decrease by this exact observed delta; setup, sieve and elimination counts remain unchanged if the same logical run completes. The existing phase validation must stay enabled. Native pricing must consume new instrument counters and newly reviewed source bindings; do not subtract an estimated saving from old records.

## Minimal implementation proposal and next validation obligation

The adjacent unapplied.patch moves one call below the existing guard, nothing else. It has NOT been applied or compiled. Before any instrument use, independently review the scoped arithmetic assumptions, and add software equivalence fixtures for ordinary, exceptional-denominator, all-lambda, length0/1/2/3/>3 cases, plus exact rejected-complement counter deltas and unchanged graph cap behavior. Those tests are future work, not results reported here. A later benchmark requires a prospective known-data protocol and resource authorization; incomplete upper-gate evidence does not authorize a rerun.
