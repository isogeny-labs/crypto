# EXP-PRIME31-KUMMER16 — preregistered rational-fibre gate (2026-11-19)

## Question and source
On the **unchanged** ordinary, cofactor-one Prime31 curve from `research/prime31_ic_20261115/native/lib.rs` at pinned original commit `1df714645aa683cf975bc76fcacb9842cb4780fd`, does the x-only doubling map `D(x)=x([2]P)` separate curve/twist rational x-classes and induce a permutation of the entire prime field? If yes, the four-link [16] RR9 join is geometrically degree 256 but has **exactly one F_p-rational predecessor** at each x and no new twist branches.

Constants must remain: p=2147483647, a=847514846, b=2087444724, full prime E order n=2147446831, G=(625037715,2100627251). Twist order is computed from the standard trace identity `m=2*p+2-n=2147520465`. Select the least tested quadratic nonsquare d=3 and `E^d: y^2 = x^3+(a*d^2)x+b*d^3` with x'=d*x for twist x-values. Never substitute another curve, use scalar logs or replace a complete solver.

## Mathematical predictions frozen before execution
1. Both n and m are odd, hence f(x)=x^3+ax+b has no root in F_p. The x-only doubling rational map
   `D(x)=(x^4-2*a*x^2-8*b*x+a^2)/(4*f(x))`
   is defined at every x in F_p.
2. For every x in F_p, `chi(f(D(x)))=chi(f(x))` and the value is +1 for the E-rational x-class and -1 for the twist class. This follows from the rational formula `y([2]P)/y(P)\in F_p^*`, with no zero or pole on either rational class.
3. Multiplication by 2 and by 16 is bijective on the points of E(F_p) and its quadratic twist E^d(F_p), since both orders are odd. Both maps commute with negation, so D and D^4 are permutations of the two x-classes and therefore of all F_p x-values. In particular, `D^4(x)=t` has exactly **one F_p rational root for every t** (not 256 roots).
4. Let `k_E=16^{-1} mod n=134215427` and `k_T=16^{-1} mod m=2013300436`. For every public x of the corresponding class, the x-coordinate recovered after [16] then [k] is exactly the original x. For twist x, first use x'=d*x on E^d then divide the resulting x' by d.

## Controlled source-specific experiment
- Use the **original native Rust `Arithmetic`, `Curve`, `Point`, `PRIME31`** APIs and pinned Rust toolchain 1.100.0, original library 40 tests; only overlay the new study example into the pinned original crate.
- Deterministic SplitMix64 seed 0x4b554d4d45523136. Choose exactly 512 random x with chi(f(x))=+1 and 512 with chi(f(x))=-1; no hidden target scalar or known log. In addition verify the first 31 rational-x factor-base x-values, including [0,1,2,3,5,6,8,9].
- Check each x-only doubling step against full original- or twist-curve arithmetic; verify full [16], inverse [k], character invariance, nonzero denominators and on-curve membership. Verify an algebraic square identity for f(D(x))/f(x) on every sampled x.
- Freeze 64 input/output certificates (32 per class), independent Python BigInt arithmetic replays their x-only D^4 values, Legendre class and complete twisted/full EC scalar recoveries. Compare the aggregate digest and total 1024+31 public samples.
- Charge original arithmetic field mul/inversion/group additions; wall times are diagnostics and **not** a measured ECDLP-solving improvement. No generic group-sum table is constructed. All source/protocol/CI hashes recorded.

## Success and falsification
Pass only if both class populations reach 512 and all arithmetic assertions and independent certificates pass. A single inconsistent x value, twist order, root, inverse or character is a FAIL. Do not silently drop failures.

This test is a narrow correctness and F_p-root-branching theorem for the k=4 x-only chain, **not** an efficient nonenumerative unknown-index extractor, a full-index-calculus pipeline, a speedup against existing complete index calculus or a proof of subexponential ECDLP. Next handoff is to RR9/F4 owner [#4692](https://github.com/isogeny-labs/crypto/pull/4692) to avoid enumerating imaginary 256 rational branches when solving the public B8 positive/negative corpus; final complete-solver benchmark must remain same-source/same-runner and fully charged versus [#4532](https://github.com/isogeny-labs/crypto/pull/4532). Prior related controlled generic-grading [#4808](https://github.com/isogeny-labs/crypto/pull/4808) and algebraic [#4809](https://github.com/isogeny-labs/crypto/pull/4809) lanes are not altered.

## Known limitations
An explicit rational inverse on point coordinates can require a scalar multiplication of ~31 bits; this does not supply a *low-degree polynomial inverse* usable in symbolic F4 or reduce full RR9 elimination automatically. The x-only permutation result needs the prime-field rational points; over an algebraic closure D has degree four. Research-KB/Conductor/ECDB servers may not be available in this runtime; record any verification gap rather than asserting absence of prior art.
