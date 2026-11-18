# Prime31 fraction-free Frobenius cubic batch-inverse gate (2026-11-18)

Status: preregistered *proposal*, before execution; [pcc:unverified].

## Source and mathematical control
Extend the existing scalar-blind degree-3 Vélu inverse in `examples/prime31_frobenius_inverse.rs` (PR #4701) on the **same** ordinary cofactor-one curve E/F_(2^31-1), n=2147446831 and the same E' and 256 public points, without an ECDLP scalar as an input. Retain the original `invert_public` as an unchanged reference.

For the cubic `F_X(T)`, compute `Z=T^p-T mod F_X` using the existing monic-modulus multiplication. Instead of ordinary Euclid, compute
`prem(A,B)=lc(B)*A - lc(A)*T^(deg(A)-deg(B))*B` iteratively until degree drops below `deg(B)`. Repeated `prem` steps preserve gcd *up to nonzero units* because the ground ring is a field and `lc(B) != 0`. Thus `g=gcd_prem(F_X,Z)` must be degree one, although its leading coefficient need not be 1. Recover the **unique rational root** as `x=-g_0/g_1`.

Keep the original ordinate formula `y=Y*h^3/(h^3-2*tau*h-8*v)`. Batch invert all nonzero `g_1` in one Montgomery inversion, then all nonzero ordinate denominators in a second. No enumeration of source points, group sums, hidden scalars or curve points is allowed in the candidate. Target validation and post hoc scalar/group replay remain mandatory.

This uses classical pseudo-division and Montgomery's trick, *not* a novel cryptanalytic algorithm.

## Frozen checks
- Reuse Rust crate, generator, coefficients, targets, and scalar-blind API. Identity returns identity; reject off-curve, zero leading coefficient, unexpected gcd degree, pole or zero ordinate denominator.
- Compare every 256 public-source preimage with old `invert_public`, normalized dual+scalar inverse and original curve scalar post-verification. Check negation, identity and invalid point.
- Assert batch full inverse uses exactly **two Fp inversions for 256 nonidentity points**, versus 1,024 in the retained pointwise reference; record all field multiplications and actual same-host AB/BA times. No timing claim is registered before an exact-source execution.
- Existing GitHub source-specific Rust 1.100.0 test job: 40 library tests, focused example tests, independent Python BigInt 256-row replay, report source SHA + exact CI status. Retain both candidate and original outputs.
- No alteration to rank31 relation solver, no full ECDLP or index-calculus speedup attribution for this component.

## Earlier work and boundaries
- Repo #4671 merged exact 3-isogeny, #4701 current single-point Frobenius-gcd inverse, #4623/#4635/#4647 original B31 complete generic-equivalent controls, #4692 active RR9 nonenumerative extractor, #4711 recent k-sum controls, #4698 theoretical subexp claim (unverified).
- Repo includes previous native batched inversion / Jacobian trials and earlier F_(p^2) inverse Vélu; novelty is restricted to this *specific* fraction-free batched Prime31 implementation/control.
- Literature: Montgomery simultaneous inversion (FLINT docs, https://flintlib.org/doc/algorithms/arithmetic/batch-inversion.html); Amadori–Pintore–Sala 2018 prime-field Semaev alternative https://doi.org/10.1016/j.ffa.2018.01.009. A search of the reachable GitHub primary-source files did not find this exact combined Prime31 pseudo-gcd batch variant. Shared research-KB tools are unavailable in this runtime, so this is **not exhaustive prior-art clearance**.

## Gate
The experiment only improves transfer preprocessing if all correctness controls pass and total multiplication/inversion costs and AB/BA measurements improve. Even success cannot establish a cheaper complete *non-generic* ECDLP solver. The independent RR9 extractor must first produce fresh, natural exact signed relations from public residuals with a no-enumeration witness and solve 1,024 original blinded logs against charged IC/BSGS/rho baselines.
