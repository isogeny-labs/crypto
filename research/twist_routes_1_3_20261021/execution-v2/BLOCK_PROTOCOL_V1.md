# E26: four equivalent encodings, bounded native and independent CAS checks

Freeze this source and protocol in Git before executing the new panel. This
is an E26 construction/solver pilot. E19 full transfer costs, E27 full cold
accounting and E28 final selection retain their original requirements.
No final E28 label 32--63 is generated. No funding or lease is requested.

## Frozen task and cases

Use all ten original ordinary odd-prime-order fixtures, B=8,12,16 distinct
abscissae and arities m=2,3,4: 90 workloads. The base is the first B
nonzero-y rational abscissae in increasing field order; both signs are
allowed. This explicitly specifies the previously unspecified base family;
it does not replace the older coordinate-box panels. Use original solver
target label 0 only, as a training pilot; preserve the wider training and
heldout requirements as open. Record exact bases, targets and input hashes.

The task is every ordered base-abscissa tuple admitting signed points whose
sum is the target. Reports deduplicate the first-pair permutation, consistently
in every arm. The direct Semaev addition chain represents affine prefix sums.
Retain every identity-prefix path separately and charge its enumeration;
never silently discard it or count it as a polynomial-system solution.
Every prefix is a sum of rational points and hence rational. The prime odd
source order excludes nonidentity rational 2-torsion.

Construct and retain all four candidates for every workload:

1. **direct-chain:** existing product-membership Semaev addition chain.
   Also retain the existing quadratic factored-membership chain and Boolean
   index encoding as direct rational controls. The addition equations have
   degree at most four.
2. **frobenius-block:** expand every addition intermediate t as u+alpha*v,
   alpha^2=d, take both coefficients, and impose v=0, justified by rational
   prefix sums. Group anti-rational coordinates first, then real addition
   intermediates, inputs and other coordinates. This is a variable-block
   grevlex order, not a claim that the native engine implements a distinct
   block monomial order. Retain the full uneliminated system. No field
   equations are silently added. The linear v=0 equations make this ideal
   isomorphic to the original affine ideal and give a unique auxiliary lift.
3. **hybrid-eight:** introduce the helper h assigning the i-th base abscissa
   to i mod 8. Enumerating h=0,...,7 replaces only the first membership
   polynomial by the product over that disjoint bucket. This is a generic
   rational helper control, not intrinsically a twist construction. The
   union is the original finite task; all eight solves share one cell budget.
4. **first-pair-invariant:** reuse u=x1+x2, v=x1*x2, the two remainder
   coefficients of the squarefree base polynomial modulo X^2-u*X+v, and
   F3=(u^2-4v)z^2-2[u(v+a)+2b]z+(v-a)^2-4bu. Squarefreeness ensures that
   divisibility describes distinct unordered base pairs. Supply the
   x1=x2 branch separately by substituting into the direct system. Charge
   both branches to the same cell and lift pair roots/signs explicitly.

## Correctness and controls

For every workload enumerate base tuples and x-coordinate prefix paths.
Verify every affine assignment in each encoding and each branch. Independently
enumerate signed group sums on all tuples for p=59,211,251; on the other seven
fixtures retain signed witnesses for every admitted tuple. Exact source
group verification, signs, diagonal paths and infinity prefixes are included.
Record the complete tuple sets and SHA-256 digests, not just booleans/counts.
The independent PARI input reconstructs base lifting, the symmetric membership
remainder and the F3 identity, replays retained signed relation witnesses and
rejects wrong-sign/F3-constant controls. A zero PARI exit without the marker
and clean-error gate is not acceptance.

Per-branch expected affine quotient dimensions are obtained by the native
complete prefix-path census. For the invariant main branch count the two
first-pair orders only once. The invariant diagonal branch and all helper
buckets have their own counts. Zero-dimensional complete bases must match
these counts; this is vdim, not Krull dimension. Exhaustively evaluate every
native solution on every input equation before admitting its lift.

Use native F4 with grevlex, degree bound 32, no early staircase or Hilbert
stop, one worker, a 30-second shared native budget per candidate cell, and
4 GiB address space. A cooperative child budget and an outer hard timeout
apply. Retain timeout, degree truncation, signal, allocation and parser
failures; they are incomplete results. Capture field multiplications in row
reduction, Macaulay row/column maxima and step logs. These counts exclude
construction, symbolic preprocessing, roots, verification and exception
enumeration; they cannot establish complete cost or a speedup.

For each completed native branch, run stock msolve 0.7.5 q0 on the exact same
system, then independently check the native and any parsed msolve basis with
Singular: compute G=std(I),
check both ideal containments and equality of the leading ideals, and compare
vdim with the complete assignment census. This supplies a complete-basis
certificate, not only ideal equality. Save the actual script and raw output.
The external-reference/checker phase has a separate 30-second budget per
candidate shared across branches. Each branch gives msolve at most half of
the remaining budget and Singular the remainder. Record external failures
separately from native certificate success. No signature mode is used.
Magma/FGb/OpenF4 are not available. Native counts cannot rank the fastest
external implementation; no such claim is admitted.

Keep a same-input native A/A control for the p=59, B=8, m=2 direct case.
Keep factored-membership and Boolean direct chains for all workloads as
ordinary controls. No candidate is selected on this training panel and no
heldout labels are opened. Incomplete cells cannot be counted as victories.

## Execution envelope and report

Emit all 90 workloads and all four candidate systems plus ordinary controls.
The native solver phase has a global 1,800-second cap; the CAS-certificate
phase has a separate 1,800-second cap, with checkpoints after every cell.
Cells not started before a cap remain explicitly not attempted. These are
bounded pilot limits, not removal of any full E26 requirement. At most one
solver or checker runs at a time. The worst-case two-phase pilot budget is
one CPU-hour, with a 1 GiB checkpoint stop for raw output and an additional
65,536-shell-block output-file bound per child. Record measured resource use;
do not infer admitted wall performance from this L0 shared host.

The construction/census has a separate 3,600-second cap and 4 GiB memory
bound; independent PARI replay has a 600-second cap. All generated files are
fresh and all failures survive. Build and tests precede a local source freeze.
GitHub currently returns 403; local source commits remain valid pins, while
remote delivery stays explicitly blocked until actual transport succeeds.

Report verified/completed/incomplete/not-started separately for every workload
and candidate, source/binary/input/protocol hashes, all native and CAS costs,
relations recovered, exceptional work and remaining costs. Ship editable
figures, a report and inspected PDF. Preserve the original E26--E28 proposals.
