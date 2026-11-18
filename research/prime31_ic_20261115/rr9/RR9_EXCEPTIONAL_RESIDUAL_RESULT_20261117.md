# Prime31 RR9 exceptional residual x-fibre: signed tangent branch
2026-11-17 UTC · PR #4546 stacked on #4545/#4541/#4539 · [pcc:unverified]

## Scope and reusable implementation
This continuation closes the explicit *x_i=x(R)* exclusion in the B8 squarefree signed-fibre experiment. Source: the **existing** native Rust original-curve RR9 evaluator at `research/prime31_ic_20261115/native/examples/prime31_rr9_evaluation.rs`; no new Python solver or weakened curve.

The fixed test curve is `p=2^31-1=2147483647`, `E:y^2=x^3+847514846 x+2087444724`, order `n=2147446831` (prime, cofactor 1), `G=(625037715,2100627251)`. Values above originate from the repo's existing curve certificate, not this experiment.

## Exact algebraic identity and necessity of the confluent jet
Let `R=(r,s)`, `s!=0`; let `F=(x-r)A(x)+(y+s)V(x)`, `deg A,V<=3`, normalized by `V_3=1`. The degree-eight monic residual factor is
`M(x)=D_R(x)V(x)^2-(x-r)A(x)^2-2sA(x)V(x)`, `D_R=x^2+rx+r^2+a`.

At `x=r`, the previous `-(x-r)M=L_+L_-` identity is `0=0`, independently of whether `M(r)` vanishes. Evaluating the quotient itself instead proves
```
M(r) = V(r) * ( [3r^2+a] V(r) - 2s A(r) ).
```
**Branch 1 (signed leaf R):** `F(R)=2sV(r)=0`; since `s!=0`, require `V(r)=0`.
**Branch 2 (signed leaf -R):** `F(-R)=0` holds automatically; the second copy of the zero at `-R` requires the true first tangent jet, with local parameter `t=x-r`:
```
F'(-R) = A(r) - ((3r^2+a)/(2s))*V(r) = 0.
```
Equivalently `(3r^2+a)V(r)-2sA(r)=0`. The exceptional x root is thus a union of **two linear factors in the seven normalized coefficients**, not an arbitrary tautology. The sign of the chosen leaf determines which factor must vanish. An unsaturated use of the old product at x=r admits spurious solutions.

The even-order rational `s=0` case is excluded by this already-certified prime-order curve; the point at infinity and singular/noncanonical inputs remain explicitly excluded. **Normalization is lossless on any valid nine-zero affine relation:** a nonzero function in `L(9O)` that vanishes with total multiplicity nine must have a pole of exact order nine at `O` (otherwise its principal divisor has more zeros than poles). The only basis monomial of pole order nine is `x^3y`, so `V_3 != 0` and rescaling to one is legitimate. This is standard elliptic divisor degree algebra, not a novel cryptanalytic result.

## Registered Rust test and producer preflight
Prospective protocol: [RR9_EXCEPTIONAL_RESIDUAL_PROTOCOL_20261117.md](RR9_EXCEPTIONAL_RESIDUAL_PROTOCOL_20261117.md), source-committed **before** modifications. The implementation adds:
- The correct signed x=r rows to the existing 8x7 affine-linear solver, preserving the old x!=r formula.
- A direct `M(r)` product-identity check rather than accepting the old `0=0`.
- Two test-only original-curve fixture families: seven distinct-x leaves summing to `O` plus `R`, and seven summing to `2R` plus `-R`. All eight signed x are distinct, and each group sum is independently checked equal to public R.
- Exhaustive 256-sign group-sum oracle and 256 linear-system replay per positive fixture, plus a negative public residual lying in an exceptional x fibre but outside all 256 group sums, with another 256 systems rejected. Every returned coefficient tuple is required to reproduce `M=H_B`, `F(-R)=0`, all signed point zeros and the original group sum. A `-R` positive is also checked via the independent tangent evaluation row. There are 128 arbitrary-coefficient exceptional-factor controls per fixture.
- New Rust test `rr9_b8_exceptional_x_jet_fibre_equivalence` and an additional JSON field in the existing original-curve report. Existing B8 general-fibre test remains unchanged.

An *independent producer preflight* using the same fixed curve but separate local scalar field/group arithmetic found expected sign masks `[0,127]` for the `R`-leaf fixture and `[0]` for the `-R`-leaf fixture; the corresponding seven-variable linear systems returned exactly these masks, and both fixtures had same-x absent targets. In the positive zero mask, the factors `V(r)` and `(3r^2+a)V(r)-2sA(r)` respectively evaluated to `(0,106725705)` and `(760576979,0)`. This is independent source preflight, **not an independently signed PCC verdict, Rust CI result, performance measurement or ECDLP solve**.

## CI and scientific status
- Source change commits `5fdd8ad4d987b6b6d48be8b71df82468950d8d59` and `2bcfb1227b91411a15f0969ddf1c09547e299631` on the stacked branch. 
- **Final source revision 2bcfb122: [exact-source Rust 1.100 CI #44064951148 SUCCESS](https://github.com/isogeny-labs/crypto/actions/runs/44064951148)**: 40/40 native library tests, **8/8 RR9 example tests** including `rr9_b8_exceptional_x_jet_fibre_equivalence`, and 64+64 previously registered original-curve determinant controls. [Raw 128-case JSON artifact #14319783227](https://github.com/isogeny-labs/crypto/actions/runs/44064951148/artifacts/14319783227) (ZIP SHA-256 `e0f6b48cbda7866d846961d08db9ef2bf85d9b9b07418d9cd5fa724a3671c8ec`). Precursor [source CI #44064526434](https://github.com/isogeny-labs/crypto/actions/runs/44064526434) also passed (40+8) on the earlier code revision without the additional raw JSON report field. These tests are exact-source **producer verification only** [pcc:unverified]; they do not certify independent PCC, ECDLP recovery or runtime improvement.
- No new complete public unknown-scalar logs, solver runtime or asymptotic advantage. Signed independent PCC is not available for this producer experiment.

## Prior leads and next discriminating experiment
Previously examined: [#4466](https://github.com/isogeny-labs/crypto/pull/4466) Hermite confluence, [#4539](https://github.com/isogeny-labs/crypto/pull/4539) signed 7-variable chart, [#4541](https://github.com/isogeny-labs/crypto/pull/4541) radical support, [#4545](https://github.com/isogeny-labs/crypto/pull/4545) B8 sign-mask fibre equivalence, [#4373](https://github.com/isogeny-labs/crypto/pull/4373) bounded m8 F4, [#4228](https://github.com/isogeny-labs/crypto/pull/4228)/[#4349](https://github.com/isogeny-labs/crypto/pull/4349) quotient/subresultants, [#4513](https://github.com/isogeny-labs/crypto/pull/4513) mixed-base complete index calculus, and [#4532](https://github.com/isogeny-labs/crypto/pull/4532) cached-BSGS comparator. Crypto-autoresearcher [#3301](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3301)/[#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) record related negative lead surveys. Primary literature: Amadori, Pintore and Sala, *Finite Fields and Their Applications* 51 (2018), DOI 10.1016/j.ffa.2018.01.009. Research-KB MCP and its wrapper are unavailable in this runtime; this is a bounded GitHub/literature check, **not exhaustive novelty confirmation**.

The next algorithmic gate is public-only *nonenumerative* RR9 coefficient recovery using the **existing Rust quotient/F4 machinery**; blinded positive and absent-fibre B8 examples (including x=r) must be recovered without supplying signs or eight leaves, then B31 with multiplicity partitions and higher jets. Only then attempt complete unknown-Q rank recovery. The #4532 one-run, producer-level equal-precomputation comparison gave 170.113ms for 1,024Q cached BSGS against 2,221.474ms mixed IC; charge every rejected extraction attempt, F4 linear algebra, setup, relation collection, memory and verification before claiming an advantage. [pcc:unverified]
