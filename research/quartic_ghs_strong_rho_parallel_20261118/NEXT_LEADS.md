# Next GHS strong-rho optimization leads

Status: prospective engineering hypotheses recorded after the reviewed
six-lane composition. They are not measured speedups or frontier rows
`[pcc:unverified]`.

The implementation-level source changes, counters, test matrix, measurement
campaign, publication gates, and PR sequence are frozen in
[`IMPLEMENTATION_SPEC.md`](IMPLEMENTATION_SPEC.md).

## Evidence basis

The combined public q=223 `strong-b8` control totals 110,435 base-field
multiplications and 782 base-field inversions. At the illustrative conversion
factor `r_inv=18`, that is 124,511 multiplication-equivalents: 52.75% setup,
45.94% walk, and 1.31% verification `[pcc:unverified]`. This one deterministic
cell prioritizes work; it does not rank wall time or select a batch width.

## Prioritized leads

1. **Five-product general cubic multiplication.** Evaluate degree-two
   operands at `0, 1, -1, 2, infinity`, interpolate the five convolution
   coefficients, and reduce modulo `z^3-c`. The algebraic ceiling is one fewer
   `fq_mul` for each of the combined cell's 13,551 `f3_mul` calls, before
   pricing the extra additions and exact divisions by two and three
   `[pcc:unverified]`. Acceptance requires exhaustive small-modulus and random
   equality, an exact counter identity, and an unchanged rho trajectory.

2. **Reuse the joint setup table for initial walkers.** The corrected setup
   retains 511 cubic inversions, of which only 33 are batch inversions. Each
   `new_walker` still evaluates `[a]G` and `[b]Q` separately. A sequential
   Shamir/window-two path can reuse the already charged table while preserving
   RNG and retry order `[pcc:unverified]`. Any speculative all-walker batch must
   charge discarded arithmetic on rare infinity retries.

3. **Lazy-sign negation quotient.** Retain the current representative, store
   signed jumps and coefficients, and make cycle state quotient/x-based. This
   should preserve the x-coordinate walk while screening whether eager field
   and scalar negations can be removed `[pcc:unverified]`. The gate is exact
   trajectory, collision-scalar, cycle-escape, and operation-count identity.

4. **Division-free base-field inversion.** Compare the existing binary
   extended-Euclid helper in `src/cryptanalysis/prime_fast.rs` with the current
   repeated-division inverse, restricted to the documented base-field range
   below `2^31`. This is a wall-only lead until a source-bound profile and
   isolated A/A/ABAB run pass.

5. **Fully counter-free production mode.** The current `fast-b*` specialization
   removes field-counter hooks but retains `NativeCounts` mutations. Apply the
   same compile-time elimination to native diagnostics and require zero
   counter SSA markers, identical mathematical output, and continued
   fail-closed exclusion from modeled-cost analysis.

6. **Retune batch width and partition count on the composed kernel.** Freeze a
   multi-seed, multi-q panel before comparing widths and independently test
   `r=16,32,64,128`. Report both multiplication-equivalents and
   `steps/sqrt(ell)`; do not promote b16 or any partition count from the single
   q=223 sensitivity row.

7. **SIMD/structure-of-arrays field kernel.** The admitted family uses
   `q<2^31`, so screen u32 limbs and lane-wise products after profiling the
   counter-free binary. Counted work and source-bound answers must stay exact.

8. **Parallel distinguished-point rho.** Use independent batch workers with a
   shared, infrequently locked distinguished-point table. Report elapsed time,
   total CPU, total work, and overshoot together. Parallelism can reduce
   latency but does not lower expected rho work.

9. **Projective or x-only redesign.** Screen formulas before implementation.
   Ordinary mixed Jacobian addition appears more expensive than the current
   batch-affine kernel once per-step normalization for partitioning is priced
   `[pcc:unverified]`. An x-only/Kummer candidate needs a functional-state and
   collision proof plus a statistical rho-constant panel.

## Deferred leads

Specialized distinguished-point packing is low priority because the q=223
control retains only 76 distinguished points and 153 table operations.
Frobenius or order-three symmetry is also deferred: field Frobenius is not, by
itself, a demonstrated same-curve subgroup automorphism. It needs an explicit
map and eigenvalue certificate before an algorithmic claim
`[pcc:unverified]`.

## Measurement gate

No wall-ranking experiment starts until checkpoint push/gate, isolation,
source binding, and the frozen A/A/ABAB or matched-grid protocol pass. A toy
counter reduction remains a candidate until those measurements and the
independent validation/PCC chain are complete.
