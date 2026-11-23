# Iteration 6 frozen correctness scope

Date: 2026-11-22 America/Los_Angeles. Existing draft PR #4928. Starting source checkpoint e1d1519aa7f2003658df56b83af75ab4eb93e824.

## Objective

Specify a full-Dickson-fibre rational auxiliary S3 representation and validate its signed witnesses on the unchanged Prime31 curve. This is a bounded development correctness component, not a performance experiment. Current new derivations have unverified PCC standing until independently certified.

## Fixed scope before execution

1. Reuse original crypto_lib finite-field, quadratic-extension and curve arithmetic plus research/prime31_ic_20261115/native/lib.rs unchanged; preserve p=2147483647, a=847514846, b=2087444724 and n=2147446831.
2. Use the existing depth-six split Dickson fibre. Construct a nonliftable w and liftable r; check the collapsed S4(w,r,w,r) ghost by its identical quadratic factors, nonsquare S3 discriminant, and explicit Fp2 group witness.
3. Verify all affine rational S3-chain certificates by oriented top-down recovery and original group replay; cover repeated leaves, signs and negative/corrupted certificates.
4. Check backward peeling of nonzero signed tuples whose sum is nonzero, including P,-P,Q with a failing fixed order and a successful permutation. Exhaust the fixed m=3 and m=4 panels over the first two canonical rational base points and their negatives.
5. Check the equivalent quadratic node equations and Dickson ladders where applicable. State explicitly that all auxiliaries range over Fp; a polynomial system with Fp coefficients alone does not enforce that domain.

## Execution and publication

Freeze Rust source hash and exact commands in a separate receipt before execution. The root crypto_lib example will use the repository manifest and lock in hosted correctness CI. Local unavailable dependency/build failures must be retained; no dependency substitution is an original-lock replay. No timing panel, paid compute, new curve, test-selected output filter or speedup claim is authorized by this correctness scope. Performance remains subject to the existing admission and matched-control protocol.

## Mathematical deliverables

Give a written soundness proof; completeness for unordered signed decompositions with m<=n by free leaf permutation; quadratic variable/equation counts; explicit Frobenius-domain guards; comparison with Semaev 2015 rational chains and the repository's existing oriented leaf-pair example. Charge solving degree, rational extraction, misses, setup, rank and growing-family requirements. Preserve the concurrent iteration-five result and all earlier evidence.
