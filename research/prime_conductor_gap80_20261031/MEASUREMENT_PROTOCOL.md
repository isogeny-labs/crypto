# Requested construction, map and endpoint measurements

This protocol retains the exact 80-bit target in `parameters.json`. No
construction, complete point-map or matched endpoint timing has been obtained.
Missing times are `null`, not zero or a partial-tile estimate.

## What prevents execution now

The existing native codomain constructor
[`sqrt_velu_batched_v4.cpp`](../conductor_gap_cm_orbits_20261015/native/sqrt_velu_batched_v4.cpp)
reads `ell` into `uint64_t`. Its included
[`sqrt_velu.cpp`](../conductor_gap_cm_orbits_20261015/native/sqrt_velu.cpp)
uses the same width for scalar multiplication. Our prime degree
`729845429276318409687041` has 80 bits, so that input is rejected rather
than represented correctly. A 96-bit *field* receipt in the earlier study
does not establish an 80-bit *degree* computation.

Widening the degree alone is insufficient. The existing method retains the
whole baby root vector before bounded giant batches. Exact native metadata
checks in `measurement-readiness.json` show the array slots alone exceed
the lab's 128-GiB pool cap, excluding the large integer limbs, polynomial
coefficients, seven derivative polynomials and product trees. This is a
method-specific lower bound, not an impossibility result for all isogeny
algorithms. The archived tile method has different memory requirements but
has not demonstrated feasible total work.

Finally, the earlier constructor is expressly codomain-only; see its
[`CONSTRUCTOR.md`](../conductor_gap_cm_orbits_20261015/native/CONSTRUCTOR.md).
It does not evaluate arbitrary points or certify a dual map. The current
optimized helper is not a replacement full constructor/evaluator.

The following preflight is an eligibility audit, not an experiment or timing:

```sh
NATIVE_PREFIX=/path/to/flint-gmp-pari-prefix sh native/build.sh
native/build/measurement_preflight 729845429276318409687041 137438953472
```

## Required measurements once prerequisites pass

1. **Construction.** Freeze source/binary hashes and exact parameters.
   Record complete setup, quotient construction, verification, peak RSS,
   memory limit, timeout, CPU seconds and wall seconds separately. Retain
   every failure and censored run. Verify the resulting exact equation,
   trace/order and conductor argument independently. Do not substitute the
   field bit length, a kernel certificate or a tile result for a crossing.
2. **Point evaluation.** Use 32 fixed nonzero known scalars modulo the
   recorded prime subgroup order. Transport the existing generator G first;
   verify the image H is nonzero and has order q. For every scalar s, check
   phi(sG)=sH and destination membership, plus kernel and exceptional-point
   behavior. Measure reusable map setup separately from each complete
   evaluation; include all conversion and verification costs.
3. **Scalar multiplication.** On both endpoints, use the same 32 scalars
   and transported bases. Alternate endpoint order and use the same native
   field backend, coordinate model, build options and resource envelope.
   Record each measurement rather than replacing it with a batch average.
4. **ECDLP.** Construct matched targets Q=sG and phi(Q)=sH from the same
   known answers. Verify recovered scalars independently. Compare native
   Pollard rho and BSGS where implemented, with explicit setup/online/recovery
   costs and memory. Retain the earlier request for an index-calculus arm:
   it needs an implemented method applicable to this prime-field instance;
   report unavailable arms as unavailable, never as timings or wins.
5. **Controls and execution.** Freeze the exact scalar list and seeds before
   launch. Use native code, EKS Spot per Iso Lab, a source/image/binary-pinned
   request, checkpoint before launch, one warmup and at least five measured
   rounds, matching A/A controls and the harness's L2 admission. Record all
   job IDs, receipts and interrupted epochs; do not silently retry or relax
   isolation. Add a coordinate-isomorphism control and retain source-only
   diagnostics separately from paired endpoint results.

No EKS job for these timings was submitted: the compatible complete workload
is not implemented. The present executor also has no AWS identity/CLI or
EKS MCP; the documented authenticated GitHub submission route is a separate
possible launcher, not a substitute for the missing executable.

## Completion gate

Only a verified complete quotient, complete non-kernel evaluations and
matched endpoint ECDLP measurements satisfy the requested crossing category.
A global largest-known claim additionally needs a source-based record
comparison. The existing source/kernel and local correctness checks remain
valid components; this protocol does not promote or replace them.
