# Pack interval requests across recurrence ratios

T-1856, 2026-11-07. Producer derivation and deterministic checks; unsigned evidence [pcc:unverified].

## Construction

The previous interval helper finished an evaluation batch at every request boundary. CoeffStream called it separately for each numerator and denominator, leaving partially filled trees even when the next request used the same cached block polynomial. The new helper accepts a bounded list of requests and keeps filling its point buffer across those boundaries.

For coefficient cap B, the request list has at most2Bentries (the stored machine limit saturates at usize::MAX to avoid integer wrap). CoeffStream makes two requests per transition, plus a two-request initializer. Before any allocation, polynomial construction or evaluation, the helper checks the request count and every start/length. An empty list returns an empty vector. A single-product wrapper preserves the old interface and supplies the separate-request reference.

Each buffered point has a destination request index. At width w, or at the end of the entire list, the helper builds one product/remainder tree, evaluates the cached P_w, and multiplies each value into its designated output. Tails and short direct products remain attached to their original requests. The buffer is cleared after each flush. No deduplication or assumption about distinct points is made.

CoeffStream first generates the2s numerator/denominator intervals for s<=B transitions, calls the shared helper once, and consumes consecutive result pairs. Prefix/backward batch inversion, sign correction and coefficient updates remain unchanged. Public recovery and source code are unchanged.

## Exact routing invariant

For each request i, the current accumulator equals its direct tail contribution times the block factors already flushed for that request. Pending (point,owner) pairs identify exactly its unconsumed full blocks. Each full block is enqueued once and each flush consumes each pending pair once, multiplying the correct accumulator by P_w(point). Thus at termination every request equals its original complete interval product.

The identity holds with repeated points, repeated requests, zero factors and modular wrap. Tree evaluation preserves leaf order; the owner vector is zipped with that output. Repeated points are allowed because this is remainder-tree evaluation with monic divisors, not interpolation requiring distinct-point inverses. All recurrence denominators remain nonzero by the previous interval proof. Validation errors are caught before helper work; any internal arithmetic failure returns an error rather than exposing partial products.

## Exact number of evaluation trees

For each blocked request i let t_i=floor(l_i/w); set t_i=0 for direct-route requests. Put K=sum_i t_i. The previous separate-request schedule builds

    T_separate = sum_i ceil(t_i/w).

The packed schedule builds exactly

    T_packed = ceil(K/w) <= T_separate,

with both zero when K=0. It fills every tree except the final one to width w. The number of evaluated block points K, value-combination multiplications, tails and the cached block polynomial remain unchanged. Packing removes request-boundary flushes; it does not discard arithmetic factors.

For a real coefficient chunk at r=1009,m=56,B=7,e=0, seven transitions request fourteen length56 products. Width7 gives8block points per request: separate scheduling needs14*ceil(8/7)=28trees, while packing needsceil(112/7)=16. The native checker asserts both formulas and all emitted coefficients against the compiled reference.

Under the existing conservative O(M(w)logw) bound per tree, the evaluation upper bound is O(T_packed M(w)logw). Exact kernel work depends on the sizes and shapes of the individual trees; fewer trees alone does not prove lower field-operation totals or wall time. No timing or universal arithmetic dominance claim is made. This construction does not change the previous cap-dependent asymptotic bound.

## Storage and work limits

The mode object remains parameter-only. A coefficient call retains at most2Brequests and2Bresults, plus the existing bounded numerator/denominator/prefix arrays. Evaluation buffers hold at mostw live points andw owner indices; Vec allocation capacity can exceed its logical length by the allocator growth policy, while remaining O(w). One cached polynomial has w+1 coefficients; temporary tree coefficients are O(wlogw), kernel workspace separate. No list of all K block points is built. Source-grid, mode-order validation, first-C0 and inverse costs remain charged.

## Native checks

`validation01/receipt.json` pins rustc,47transitive source inputs and stdout, with overflow checks enabled. `validation01/check.stdout` records:

- 375 request-list panels and3,195 exact request products, compared both with individual helper calls and independent direct multiplication. Primes97,257,65537; target lengths32,56,64,80,128,512 where valid; caps1,2,7,16,64; mixed zero-length, short and long requests; repeated starts0,1,r-1,(r-1)/2.
- Both exact tree-count formulas hold in every panel:123have fewer trees and252equal; no panel has more. These are deterministic coverage/assertion counts, not sampled workload performance estimates.
- Over-cap and invalid-late-request preflight controls leave the polynomial absent and evaluation count zero; empty input returns no outputs.
- The actual r1009/m56 coefficient chunk has16trees versus28separate and equal coefficients.
- Inherited4,820interval comparisons including zero/wrap controls,476coefficient sequences,4,176residues,1,392scalar evaluations and42,324sequential-state comparisons pass.
- Public recovery panels remain exact:281targets at m56 with14,421source calls and617targets with5,929calls; zero unresolved in both. Results/routes/statistics agree with the reference. Earlier63-case controls also pass.

No native compile/assertion failure. Known scalars and root searches are checker-only; runtime uses public points. Original4,563-case cohort, source trace/protocol and older artifacts unchanged. No cloud or timing run, no fullCargo rerun; the historical unrelated missing-fixture/deadline limitation remains.

Replay into a fresh output directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-packed-products/validate-recovery.py /tmp/packed-products-replay-unique
```

## Sources and next step

KB unavailable: MCP absent, scripts/research-kb status reported no executable. Inspected exact predecessor interval/stream/polynomial sources and the standard multipoint-evaluation discussion on pages3-4 of Bostan-Gaudry-Schost, https://mathexp.eu/bostan/publications/BoGaSc04.pdf. Packing is a bounded scheduling adaptation of that standard arithmetic, not a new interpolation method or the paper's specialized progression-shift algorithm. No novelty claim. Historical and canonical graphs remain unchanged; the new SVG/PDF depicts this update only.

Next examine structured progression evaluation or reuse of repeated block values, including their exact preconditions and storage costs, to reduce kernel work beyond the removed boundary flushes.
