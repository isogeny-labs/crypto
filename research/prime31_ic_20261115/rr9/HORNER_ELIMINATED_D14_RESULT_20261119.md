# Prime31: exact rational-doubling elimination of the 21-variable x-only Horner chart

Date: 2026-11-19. Same ordinary p=2^31−1 elliptic curve, not a specially weak or smooth-order curve. **Scope: a polynomial-system alternative for an existing Rust F4 extraction lane; not a completed public-only index-calculus ECDLP solve.** [pcc:unverified]

## Mathematical advance

Continue the *same* signed, powers-of-two weighted eight-leaf Horner system from PR #4812:

    T7=P7 ; Ti=Pi+[2]Ti+1 ; Q=T0
    u_i=x(P_i), t_i=x(T_i), t7=u7, t0=x(Q).

The 21-variable original x-only system has seven auxiliary doubled x variables `d_i`, seven doubling equations and seven F3 equations, plus eight leaf-factor-base membership equations.

For `z=t_{i+1}`, define

    N(z)=z^4-2az^2-8bz+a^2
    D(z)=4(z^3+az+b)
    A(u,r)=(r-u)^2
    B(u,r)=-2((r+u)(ru+a)+2b)
    C(u,r)=(ru-a)^2-4b(r+u)
    E(u,z,r)=A*N(z)^2+B*N(z)*D(z)+C*D(z)^2.

Then `E(u,z,r)=D(z)^2*F3(u,N(z)/D(z),r)` when D(z)!=0. Because the unchanged `E(F_p)` has odd prime order `2147446831`, the curve has no rational point of order two. Thus `z^3+az+b` has **no Fp root** (otherwise (z,0) is rational two-torsion), and D(z) is invertible for **every** z in Fp. Therefore the eliminated system is *exactly rational-point equivalent* to the original doubled-x chart over Fp; no artificial rational-root branch is introduced by clearing these denominators.

After substituting public x(Q), variables shrink **21 → 14**, equations **22 → 15**: eight public leaf x values, six internal suffix x values, eight `H_i(u_i)=0` equations, and seven `E(u_i,t_{i+1},t_i)=0` equations. This is a **one-third reduction in variables**, but the step polynomial total degree increases from at most four to **ten**. Direct exact sparse expansion of E on Prime31 has **67 monomials** with individual degrees (2,8,2) in (u,z,r), making it useful to test sparse block elimination, *not evidence of a runtime gain*.

**Boundary:** The rational-point equivalence does not assert saturated ideal equality or absence of algebraic-closure components; a Gröbner basis may encounter roots of D over extension fields. As in the original chart, suffix-infinity events need a separate branch, and sign lifting must be checked after any root is found.

## Source-specific Rust validation (completed 2026-11-19)

Both [push Actions run #44587413473](https://github.com/isogeny-labs/crypto/actions/runs/44587413473) and [PR Actions run #44587425573](https://github.com/isogeny-labs/crypto/actions/runs/44587425573) **completed SUCCESS** on the exact code/workflow commit `2efeb7e685fc161b1156c45db5f01e7e92f98adc`, with original native arithmetic pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd`. They passed **40/40 original library tests, 2/2 original x/y tests, and 3/3 x-only tests** (including the new eliminated-rational-set test). The release protocol gate accepted **1024/1024** original-curve planted positive controls and **7168** eliminated step identities, and checked **8192** independent uniform-field triples. The separately authored Python BigInt checker within the same source-specific workflow replayed **1024/1024**, checked **16/16** stored sign certificates, all **7168** eliminated steps and all **8192** random triples, with zero chart exceptions. Logs explicitly report `pcc_signed=false`; this is not an independent signed PCC attestation.

Frozen raw evidence: [artifact #14562965497](https://github.com/isogeny-labs/crypto/actions/runs/44587413473/artifacts/14562965497) (push) and [artifact #14563335008](https://github.com/isogeny-labs/crypto/actions/runs/44587425573/artifacts/14563335008) (PR). Artifact sizes 7042 bytes each. The successful Rust run is **not** a measurement of F4 unknown-index extraction or complete-log solving runtime.

## Original curve and independent experiment

Frozen protocol: [HORNER_ELIMINATED_D14_PROTOCOL_20261119.md](HORNER_ELIMINATED_D14_PROTOCOL_20261119.md), committed before the source changes.

Original Rust arithmetic: `research/prime31_ic_20261115/native`, pinned commit `1df714645aa683cf975bc76fcacb9842cb4780fd`, unchanged library and `PRIME31`; the existing `prime31_horner_xonly_sign_lift.rs` is **extended**, not copied or replaced. CI path: `.github/workflows/prime31-horner-xy-chain.yml`.

The separately implemented Python-bigint preflight on the exact original `p,a,b,n,G` and four frozen B8 windows completed:

| Check | Independent Python result |
|---|---:|
| Signed planted Horner fixtures | 1,024 / 1,024 ordinary |
| Elimination + F3 + doubling identities | 7,168 / 7,168 |
| Nonzero doubled denominators, sampled suffixes | 7,168 / 7,168 |
| Same-witness target x+1 mutation rejections | 1,024 / 1,024 |
| Unbiased field-triple rational identity checks | 8,192 / 8,192 |
| Expanded polynomial terms | 67 |
| Maximum total polynomial degree | 10 |
| Public-only unknown-index extractions | **0** |

The independent Python implementation also recomputed `[n]G=O` and the exact field-base x sequence. This is a local independent arithmetic cross-check; original Rust CI is a separate gate, and neither is signed independent PCC. Mutated same-witness controls are *not* proofs of natural target nonmembership.

## Previous-lead survey and decisions

1. [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns the public-only shared-coefficient RR9/F4/remainder-tower attempt and its frozen 46-positive/18-negative corpus. Integrate/test there; do not create another F4 engine.
2. [#4812](https://github.com/isogeny-labs/crypto/pull/4812) produced the 72-variable x/y and 21-variable x-only Horner charts with certified planted recoveries. The 14-variable rational elimination is a new **encoding** relative to that local baseline, not an unpublished new summation-polynomial attack.
3. [#4809](https://github.com/isogeny-labs/crypto/pull/4809) investigated standard F3 two-factor roots; [#4810](https://github.com/isogeny-labs/crypto/pull/4810) established the rational Kummer doubling-permutation property on the registered odd-order curve.
4. [#4807](https://github.com/isogeny-labs/crypto/pull/4807), [#4808](https://github.com/isogeny-labs/crypto/pull/4808) observed improved powers-of-two/ternary generic *grading coverage*. Neither introduced a non-generic target extractor.
5. [#4801](https://github.com/isogeny-labs/crypto/pull/4801), [#4803](https://github.com/isogeny-labs/crypto/pull/4803) gave interpolation and unweighted support limits; avoid reopening those exhausted variants.
6. [#4728](https://github.com/isogeny-labs/crypto/pull/4728) is the native full rank31/1024-log pipeline; merged [#4532](https://github.com/isogeny-labs/crypto/pull/4532) is an optimized shared-BSGS control (170.113 ms for 1024 logs, historical separate run); [#4679](https://github.com/isogeny-labs/crypto/pull/4679) is the Faugère–Lachartre F4 optimization lane. Compare **same host, same targets and fully charged totals** when an extractor becomes available.
7. Earlier same-curve audit `research/prime31_ic_20261115/native/M8_UNIFORM_RANK_CEILING_AND_F4_PILOT.md` found that small-variable/degree arguments do not constitute measured F4 wins. The published 2020 [analysis of Semaev naive index-calculus complexity](https://doi.org/10.1515/jmc-2019-0029) warns about treating favorable F4 degree heuristics as a complexity proof. The 2016 Petit–Kosters–Messeng approach is prior art, not a novel attack. Shared research-KB MCP was unavailable in this runtime; GitHub sources and public literature are the bounded fallback, not an exhaustive novelty audit.

## Next discriminating experiment

Compute the exact 14-variable sparse E polynomials, eight H_i polynomials and the 21-variable quartic/doubling baseline in the **existing Rust F4 engine**; fix the same monomial order, degree cap, time/RSS cap and seeds. Run the **frozen 46 positive / 18 negative public-target corpus** in both arms with no supplied signed indices, no planted suffixes and no generic 3+3/4+4 point-sum tables. Measure F4 Macaulay widths, peak sparse fill, degree reached, setup amortization, exceptional handling, verified recovered relation vectors and failures. If all 46 positives aren't recovered or a negative is falsely asserted UNSAT without a proof, fail the gate.

If B8 passes, integrate into existing rank-31 Rust solver and verify 1,024 original Prime31 discrete logs against same-host optimized IC, shared BSGS and optimized rho for 1/64/1024 queries. Charge all relation misses, rootfinding, linear algebra, reusable setup, verification and peak memory. If the higher-degree 14-var system fails at F4, preserve it as a negative result and pursue another nonenumerative route.

**Scientific standing:** Exact 14-variable *candidate* and independent preflight are available; public-only rootfinding, complete new ECDLP recoveries, cost advantage and signed independent validation are not established.
