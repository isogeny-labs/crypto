# Prime31 Kummer [16] rational fibres — theorem, certificate, extractor handoff (2026-11-19)

**Standing:** exact mathematical theorem; new Rust source-specific CI and independent replay are pending confirmation. This is not a new complete ECDLP solver or solving-speed advantage.

## Mathematical statement

The unchanged ordinary cofactor-one curve E over p=2147483647 is y²=f(x)=x³+847514846x+2087444724 and has odd prime order n=2147446831, generator (625037715,2100627251). Its quadratic twist E³ has order m=2p+2−n=2147520465, also odd. The element 3 is a quadratic nonsquare. The twisting model is y²=x³+9ax+27b with transformed x'=3x.

Define the rational x-only doubling map:

    D(x) = (x^4 − 2*a*x^2 − 8*b*x + a^2)/(4*f(x)).

**Theorem:** D and D composed four times are bijections on every x in F_p. Both preserve the Legendre character of f(x). The two invariant classes have cardinalities (n−1)/2=1,073,723,415 (curve) and (m−1)/2=1,073,760,232 (twist), summing to p. Every specified final x under [16] has exactly one F_p-rational predecessor, even though the geometric degree over an algebraic closure is 4^4=256.

**Proof.** Both n and m are odd. Thus f has no rational root: an x with f(x)=0 would yield rational 2-torsion on E, contradicting odd group order. Consequently D has no F_p pole, and the curve and twist x-lift sets partition F_p. The map x(P)=x(−P) identifies precisely the sign pairs in the corresponding group. Multiplication by 2 or 16 is an automorphism of both odd-order groups and commutes with negation, hence descends to a permutation on each sign quotient. The induced rational map is exactly D or its fourth iterate. Their union is F_p, proving global bijectivity and class preservation.

A direct algebraic identity provides an independent symbolic check. Put N(x)=x⁴−2ax²−8bx+a² and M(x)=(3x²+a)(4xf(x)−N(x))−8f(x)². The curve doubling formula yields y([2]P)=M(x)y(P)/(8f(x)²), so 64 f(x)³ f(D(x))=M(x)². Because there are no rational f-roots, M(x) cannot vanish; thus the quadratic character is preserved.

## Known-endpoint inverse and limits

The exact scalar inverses of 16 are 134215427 modulo n and 2013300436 modulo m. A public final x can be lifted to either sign of the corresponding curve/twist and scalar-multiplied by the appropriate inverse; the recovered x is the unique predecessor. This uses a ~31-bit full group scalar multiplication and is NOT a low-degree symbolic inverse that magically accelerates F4, since the intermediate final x in the RR9 join is not initially known.

This lemma prevents the #4692 extractor from interpreting 256 algebraic-closure inverse branches as 256 distinct prime-field rational candidates. It does not solve the unknown-index extraction, eliminate the RR9 F4 system, or demonstrate a complete prime-field ECDLP gain.

## Frozen source and validation artifacts

- [Preregistered protocol](KUMMER16_TWIST_PERMUTATION_PROTOCOL_20261119.md), committed before executable and CI workflow.
- [Original native Rust crate example](../native/examples/prime31_kummer16_twist_permutation.rs), pinned underlying arithmetic SHA 1df714645aa683cf975bc76fcacb9842cb4780fd: 512 deterministic public x in each curve/twist class; 31 original first-rational-x factor-base points; exact polynomial-square identity at each step; full group-law [16] and inverse-scalar assertions. This is not a group-sum-table oracle. Original Arithmetic counters do not price the separately written x-only rational formula; no runtime advantage may be inferred.
- [Independent BigInt affine validator](../native/verify_prime31_kummer16_twist.py) reconstructs all 1,024 public samples and replays 64 sealed per-class x, [16] and inverse witnesses. This Python code is only a verifier; the candidate implementation is Rust.
- [Source-specific CI](https://github.com/isogeny-labs/crypto/actions/workflows/prime31-kummer16-twist-permutation.yml) overlays only the example onto pinned source and performs original library and focused Rust tests, the full experiment, JSON checks, independent replay and SHA256 artifact capture. Pending final CI inspection at document creation.

## Prior leads checked

| Lead | Existing work | Decision |
|---|---|---|
| k=4 weighted W8 blockscale | [#4808](https://github.com/isogeny-labs/crypto/pull/4808) generic 4+4 table: 452/512 hits | Keep structural algebra, but not a public-only extractor |
| Low-degree two-factor roots | [#4809](https://github.com/isogeny-labs/crypto/pull/4809) Semaev F3; support <=1923 targets | Too sparse as a sole B31 relation generator |
| B8 unknown-index F4/remainder tower | [#4692](https://github.com/isogeny-labs/crypto/pull/4692), 46-positive/18-negative sealed corpus | **Owner for next integration; do not duplicate** |
| Exterior linear compression / all-sign interpolation | [#4779](https://github.com/isogeny-labs/crypto/pull/4779), [#4801](https://github.com/isogeny-labs/crypto/pull/4801) negative rank results | Do not repeat failed linear routes |
| Whole solver baselines | [#4793](https://github.com/isogeny-labs/crypto/pull/4793) and [#4532](https://github.com/isogeny-labs/crypto/pull/4532) historical 170.113 ms/1024 optimized shared BSGS | Requires same-runner, fully charged control |

Published prime-field polynomial index calculus was also checked: Petit–Kosters–Messeng (PKC 2016, DOI 10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala (2018, DOI 10.1016/j.ffa.2018.01.009). This Kummer/twist theorem is standard arithmetic and **not claimed as an original attack**. Repo code search covered Kummer doubling, twist character, RR9 F4, F3 roots and sibling negative leads; shared Research-KB/Conductor/ecdb were not available here, so absence of hits is not proof of novelty.

## Next falsifiable experiments

1. **#4692**: run public-only B8 extraction on frozen 46 positive/18 negative public targets. Log F_p rational-vs-geometric branches, twist rejections, Macaulay degree/rows, memory, failed-query tails and independently verified factor indices, with all generic half-sum table counters zero.
2. **Same extractor, paired ablation**: compare monolithic x([16]) degree-256 rational equation versus four-link doubling-chain equations with and without character pruning. Test if the mathematical invariant *actually* reduces F4 cost rather than assuming it does.
3. **Whole solver**: only after a correct public-only extractor exists, integrate into existing original Rust rank31/1024 pipeline. Report separately fully charged single-target cold vs rho, and 64/1024-target throughput vs same-runner optimized BSGS and existing IC frontier. Verify every recovered scalar and all setup.

No whole solver advantage, subexponential algorithm or new complete logarithm has been established by this theorem alone.
