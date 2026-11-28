# Prime31 exact Frobenius-inverse cubic: source-bound Rust and two-order component comparison

2026-11-18 UTC. **[pcc:unverified]** Exact source-specific Rust and producer-independent Python arithmetic replay passed. This is a public-point algebraic inverse *component*, not a new index-calculus relation finder or an end-to-end ECDLP speedup. Do not promote it as a non-generic ECDLP solver.

## Original ordinary prime-field target, unchanged

- `p=2147483647=2^31-1`; `n=#E(Fp)=2147446831` prime; cofactor 1.
- `E: y²=x³+847514846x+2087444724`; `G=(625037715,2100627251)`.
- `E': y²=x³+323891735x+1985248359`; `G'=(1767020261,634125456)`, from merged #4671. Normalized degree-three Vélu kernel abscissa `u=533591203`; `tau=1555600864, v=259447350`. The geometric kernel has no nontrivial Fp-point, so Frobenius swaps its two nonzero points.
- The same original Rust arithmetic and group-order certificates are pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd`. Only new example and independently written verifier are overlaid by the CI workflow; no new curve chosen.

## Mathematical derivation and boundary

For a public `Q=(X,Y)` on E', with Q != O, the degree-three inverse-coordinate polynomial in the source x is

`F_X(T)=T(T-u)^2 + 2*tau*(T-u) + 4*v - X*(T-u)^2.`

The rational-degree-three isogeny is injective on E(Fp), because its geometric order-three kernel has no nonzero rational points. Because both rational groups have the same n, this restricted homomorphism is bijective; Q has a unique rational preimage P. Over an algebraic closure the three preimages are `P,P+K,P-K`. With `K^p=-K`, Frobenius fixes P and swaps P+K/P-K. Their x-coordinates are distinct for Q != O: equality of two elliptic x-coordinates forces equality of points up to sign; for the swapped pair this would require `2K=O` or `2P=O`. Neither holds (the first kernel point has order 3 and P lies in the nonidentity rational odd-prime-order group). Equality between P and a shifted preimage would likewise force a nontrivial K rational. Consequently the inverse cubic factors as one **Fp-linear factor and one irreducible quadratic** for every nonidentity rational target.

The native implementation gets x deterministically as the unique root of

`gcd(F_X(T), T^p-T) = T-x`,

and reconstructs the signed ordinate *without any square-root branch* using `h=x-u` and

`y=Y*h^3/(h^3-2*tau*h-8*v)`.

This is a complete scalar-blind inverse on the rational points and does not enumerate original source points. The vanishing denominator is excluded for Q != O by separability and absence of rational 2-torsion. Identity is handled explicitly, malformed codomain points rejected; every output is checked on E and against the forward map in the validation panel. The argument uses standard isogeny/Frobenius facts; it is not Lean-kernel formalized and is not a new asymptotic ECDLP algorithm.

## Exact-source Rust / independent verification

- [Successful first original-candidate CI](https://github.com/isogeny-labs/crypto/actions/runs/44259095918): **40/40** original Rust library tests, **2/2** new example tests. Independent Python BigInt replay checked **256/256** public inverses, 256 forward Vélu equalities, 512 source/target scalar equalities, 256 exact cubic identities and nonsquare quadratic discriminants, 256 signed-y inversions. [Raw 256-row evidence artifact](https://github.com/isogeny-labs/crypto/actions/runs/44259095918/artifacts/14406923229) SHA256 ZIP `60636d239a218f89b7e1d6344f983a70d191850b8a087d3eeec405fa04d0ffc8` (GitHub-reported digest, not an independently downloaded archive checksum).
- [Reversed-order AB/BA exact-source CI](https://github.com/isogeny-labs/crypto/actions/runs/44259575247): same **40/40 + 2/2** Rust test gates and independent Python 256-row replay. Dual→direct repeated output and source known-answer checks match the first panel; forward round-trip, sign/identity/malformed-point controls all pass. No arbitrary point with a known scalar is supplied to the inverse function itself. All tests on fixed public `k=1..256` development controls, **not fresh blinded held-out logarithm recovery**.
- Source: `research/prime31_ic_20261115/native/examples/prime31_frobenius_inverse.rs`; independent checker `verify_prime31_frobenius_inverse.py`; workflow `.github/workflows/prime31-frobenius-inverse.yml`; protocol `PRIME31_FROBENIUS_CUBIC_INVERSE_PROTOCOL_20261118.md`. CI retains producer JSON and independently checked summary. This does not constitute a signed PCC verdict.

## Measured stage costs: polynomial Frobenius inverse vs normalized dual + scalar 1/3

The *same 256 public E' targets* are inverted by A = cubic Fp root (polynomial modular powering and Euclidean gcd) and B = normalized dual Vélu inverse followed by original Rust EC scalar multiplication by `3^{-1} mod n=1431631221`. Both include their stage work; B includes native elliptic point additions/inversions. Target generation and validation occur outside both measured stages. Sequential hosted-runner nanosecond timing is L0 diagnostic only, not isolated portable ecbench performance.

| Run / stage order | A: direct Frobenius cubic | B: dual+[1/3] | B/A |
| --- | ---: | ---: | ---: |
| First CI A→B | 1.918110 ms | 3.997873 ms | 2.084x |
| Follow-up CI A→B | 1.643902 ms | 2.978611 ms | 1.812x |
| Follow-up CI B→A | 1.652165 ms | 2.990670 ms | 1.810x |

Source-native arithmetic counts (the A/B and B/A repeats have identical counts):

| Count for 256 public inverses | A: Frobenius cubic | B: dual+[1/3] |
| --- | ---: | ---: |
| Fp multiplications (including inversion exponentiation) | **356,864** | 812,288 = 17,920 normalized dual + 794,368 native EC scalar |
| Fp inversions | **1,024** | 12,544 = 256 normalized dual + 12,288 native EC scalar |
| Native source curve EC additions | none in inverse stage | 12,544 |

The A-stage count is **2.276x fewer** field multiplications and **12.25x fewer** field inversions than the B stage under this exact implementation/cost model. Its dual reference has a separate 2-inversion setup for fixed 9 and 27 scalings, not included in the 256-query table. Direct cubic inversion itself is an existing mathematical primitive; no asymptotic ECDLP exponent improvement follows from this small component comparison. The first and follow-up CI jobs share comparable GitHub-hosted images, not guaranteed the same physical CPU; the AB/BA order panel is within one runner. No independent performance sign-off.

## Prior-lead / originality checks before selection

| Prior work | Consequence |
| --- | --- |
| [crypto #4671](https://github.com/isogeny-labs/crypto/pull/4671), [#4660](https://github.com/isogeny-labs/crypto/pull/4660) | Already mapped E→E' by degree-three Vélu, audited normalized dual and transported 1,024 signed witnesses; transporting the same base preserves exact relation matrices and rank. This report **does not** claim new transport or a rank/coverage gain. |
| `research/isogeny_torsion_fibers_20261012/portable_order_trend_20261015/inverse_velu/RESULTS.md` and `PROTOCOL_V2.md` | Already implemented inverse Vélu cubics for a toy F_(1237²) instance; its split schedule needed a correction and complete toy solving remained slower than rho. The *polynomial-inversion strategy* is prior art; this is a deterministic Prime31-specific realization. |
| [crypto #4692](https://github.com/isogeny-labs/crypto/pull/4692), [#4678](https://github.com/isogeny-labs/crypto/pull/4678), [#4682](https://github.com/isogeny-labs/crypto/pull/4682) | Active RR9 shared coefficient, S4/S3 and cold rho-parity lanes. **Do not duplicate** their algebraic relation collectors or cost protocol; hand this inverse component to those owners. |
| [crypto #4604](https://github.com/isogeny-labs/crypto/pull/4604), [#4623](https://github.com/isogeny-labs/crypto/pull/4623), [#4635](https://github.com/isogeny-labs/crypto/pull/4635), [#4647](https://github.com/isogeny-labs/crypto/pull/4647), [#4652](https://github.com/isogeny-labs/crypto/pull/4652), [#4596](https://github.com/isogeny-labs/crypto/pull/4596) | RR9 negative D9 cap, correct but generic 4A/4A rank31 solver, factor-base optimizations/nulls, and separately owned matched BSGS/IC frontier. A faster Vélu primitive does not replace a whole ECDLP comparison. |
| [cryptanalysis #1768](https://github.com/isogeny-labs/cryptanalysis/pull/1768), [crypto-autoresearcher #3309](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3309) | Other ongoing algebraic witness/recovery experiments; neither has established a comparable full Prime31 solver advantage. |
| [Petit–Kosters–Messeng (2016)](https://doi.org/10.1007/978-3-662-49387-8_1), [Amadori–Pintore–Sala (2018)](https://doi.org/10.1016/j.ffa.2018.01.009) | Prime-field algebraic relation extraction has existing literature and known scaling difficulties. No claim of cryptanalytic novelty from cubic root finding. |

Shared research-KB MCP/CLI unavailable in this runtime; fallback inspected relevant primary GitHub source/results in crypto, cryptanalysis and crypto-autoresearcher. The bibliography is not a comprehensive novelty clearance.

## Next falsifiable experiments, in priority order

1. **Nonenumerative extraction #4692 / original Rust F4.** Link the scalar-blind E' inverse as an optional public-point preprocessor, no source scalar. Validate the exact B4/B8 membership fibre (positive, negative, sign, exceptional) before B31; **zero** generic four-sum operations, otherwise classify as a generic-equivalent control. Require independent signed witnesses from fresh naturally generated public Q, not only an archived or planted relation.
2. **Destination-native B31 vs transported B31.** The isogeny transports relation coefficients exactly and cannot improve rank for unchanged base. Compare a *native* first-x E' factor base with the transported-source base, same 1,024 fixed Q, and count useful algebraic relation rank per all fully charged attempts; preserve source/destination x-base discrepancy but do not infer yield from mere x-set overlap. A correctly recovered negative call must be counted.
3. **Paired-S3 preprocess / RR9 remainder-tower.** Audit #4604's capped 15,000-reducer natural B8 failure, #4692's frozen B4/B8 obligations and F4/rootfinding code. Only allow deeper B31 after a zero-enumeration exact control; stop on resource cap and publish censored outcomes, no fabricated witness.
4. **Full Prime31 1,024-target solve.** Hook any *real algebraic* relations into the existing native rank31/target-recovery Rust pipeline (#4623). Same-source, same-host matched cold one-target and 1,024-target amortized arms versus the strongest available optimized IC, shared BSGS and negation rho. Charge isogeny/inverse/setup, unsuccessful relation searches, linear algebra, target recovery, memory, fallback and verification. Freeze protocol and blind targets; independent signed PCC validation before promoting an advantage.
5. **Scaling.** Only after a complete-advantage cell passes, test nonexceptional prime fields across at least four larger sizes with no deliberately weakened curve or advantageous preselected secret; report error bars and model assumptions.

**Result boundary:** demonstrated an exact 256-target stage gate and 1.81x B/A diagnostic improvement for an arithmetic inverse versus dual+scalar on this fixed Prime31 curve. **Zero new Semaev/RR9 natural relations and zero new complete index-calculus logarithms** were recovered in this experiment. The genuinely non-generic cheaper *complete* prime-field ECDLP solver remains an open next-phase objective.
