# Prime31 signed Horner/F3 discriminant-splitting gate — protocol, 2026-11-19

**Owner:** existing #4812 x-only Horner branch. Extension of existing Rust example on the **unchanged** ordinary cofactor-one curve over p=2^31-1. This is a mathematical exactness/cost-risk check, not a public-only unknown-index ECDLP extractor or performance claim. [pcc:unverified]

## Hypothesis (stated before native Rust change)

Write f(x)=x^3+a x+b, N(z)=z^4-2 a z²-8 b z+a², D(z)=4f(z), and
M(z)=z^6+5 a z^4+20 b z³-5 a² z²-4 a b z-8 b²-a³.
For E(u,z,r)=D(z)² F3(u,N(z)/D(z),r), interpreted polynomially without division,
verify the exact symbolic identity in Fp[u,z]:

    disc_r(E) = 64 f(u) f(z) M(z)^2,

and the duplication identity

    N(z)^3 + a N(z) D(z)^2 + b D(z)^3 = M(z)^2.

Also check that the quadratic leading coefficient in r equals (N(z)-u D(z))², and that E's coefficient reconstruction matches direct E(u,z,r) evaluation.

**Geometric objective:** Establish that the *full triangular, factor-base-specialized* 14-variable affine chart has only Fp-rational geometric roots and no extension-only 2-torsion artifacts, despite the cleared-denominator risks of an isolated E step. This relies on all eight leaf polynomials splitting into distinct ordinary-curve x-values and the rational suffix x beginning at the last leaf, and does **not** imply F4 speedup.

## Fixed tests / failure policy

1. Reuse unmodified PRIME31 and Arithmetic in `research/prime31_ic_20261115/native/lib.rs`. Extend the existing `prime31_horner_xonly_sign_lift.rs`; do not copy the arithmetic or add a new engine.
2. For 8,192 deterministic uniform field pairs (u,z), compare discriminant, duplication polynomial identity, and coefficient reconstruction at r=0,1,p-1. Any nonzero residual fails.
3. For all 31 × 31 pairs of existing canonical base x values, ensure u,z are ordinary and that roots predicted by the ordinary elliptic group sums U +/- 2Z satisfy E=0 when affine. Explicitly count possible exceptional infinities; do not suppress them.
4. Independently replay the identity with separately coded Python big integers. A 10,000-pair exploratory Python preflight **already happened before protocol publication**; it is *not* blinded or preregistered evidence. The native Rust test remains source-bound by a GitHub workflow; do not claim pass until run completes.
5. Record the mathematical proof, test counts, checksum/source refs, and any failures in a separate report and session log. Mark signed independent PCC unverified.

## Solver frontier and prior art

- [#4812](https://github.com/isogeny-labs/crypto/pull/4812): 14-variable eliminated chart; prior report notes unassessed extension-field/saturation risk.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692), [#4822](https://github.com/isogeny-labs/crypto/pull/4822), [#4816](https://github.com/isogeny-labs/crypto/pull/4816): public-only RR9/F4/Wiedemann extraction owners. Do not duplicate.
- [#4809](https://github.com/isogeny-labs/crypto/pull/4809): F3 rootfinding owner; [#4826](https://github.com/isogeny-labs/crypto/pull/4826): domain-root visitor owner.
- [#4714](https://github.com/isogeny-labs/crypto/pull/4714), [#4793](https://github.com/isogeny-labs/crypto/pull/4793), merged [#4532](https://github.com/isogeny-labs/crypto/pull/4532): fully charged frontier, not beaten.
- Petit–Kosters–Messeng (PKC 2016), DOI 10.1007/978-3-662-49387-8_1, preexisting prime-field summation-polynomial theory. No novelty over classical division-polynomial or F3-discriminant identities is asserted.

Next decisive measurement remains B8 public-only nonenumerative index recovery through the **existing** Rust F4 engine on 46 positive/18 negative cases, then complete rank-31/1024-Q same-host cost accounting.