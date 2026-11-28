# Prime31 RR9: distinct weighted-slot resultant criterion and original-curve Rust control

**2026-11-18 UTC** · [PR #4728](https://github.com/isogeny-labs/crypto/pull/4728) · [registered protocol](RR9_WEIGHTED_SLOT_RESULTANT_PROTOCOL_20261118.md) · [native Rust](../native/examples/prime31_rr9_weighted_resultants.rs) · [independent Python](../native/verify_prime31_rr9_weighted_resultants.py).

**Status:** a new exact algebraic **certificate/constraint encoding**, not an implementation of a public-target coefficient extractor and not a completed non-generic ECDLP solver. The precommit independent Python fixture computation succeeded. Exact-source GitHub Actions native compilation and producer replay are a separate evidence gate. Signed independent PCC is pending.

## Unchanged original curve and factor base

- \(\mathbb F_p\) with \(p=2^{31}-1=2147483647\).
- \(E:y^2=x^3+847514846x+2087444724\).
- Prime group order \(n=2147446831\), cofactor one; \(G=(625037715,2100627251)\).
- 31 canonical liftable rational-x base points from the existing Rust research crate.
- Eight slot weights \(w=(1,3,5,7,11,13,17,19)\); slot \(j\) offers \(\pm[w_j]P_i\), \(0\le i<31\).

The independent Python BigInt preflight enumerated the actual 248 weighted abscissae and found **248 distinct values**. This is a **frozen-curve fact**, not a general guarantee for other curves, other bases or arbitrary weights. The old shared-base positive-slot support cap \(\binom{31+7}{8}/n=2.2773\%\) does not bound this disjoint-image design. Eight ordered weighted slots have \(31^8=852891037441\) formal signed-index words, or about 397.17 words per ambient group element *before collisions*. That ratio is **not a measured coverage probability**, nor evidence of cheap extraction.

## New exact algebraic observation

For any public nonidentity \(R=(r,s)\), write a normalized RR9 function

\[
F(X,Y)=(X-r)A(X)+(Y+s)V(X),\qquad
\deg A\le3,\quad V=X^3+v_2X^2+v_1X+v_0.
\]

The signed ninth zero \(F(-R)=0\) is automatic. Let

\[
D_R=X^2+rX+(r^2+a),\qquad
M=D_RV^2-(X-r)A^2-2sAV.
\]

Then \(M\) is monic of degree eight, and \((X-r)M\) is the signed x-norm factor (up to sign). This uses the already established [seven-coordinate chart in #4539](https://github.com/isogeny-labs/crypto/pull/4539); it is not a new Riemann–Roch identity.

Define slot-specific \(H_j(X)=\prod_{i=0}^{30}(X-x([w_j]P_i))\) and the **eight factored resultants**

\[
g_j(A,V;R)=\operatorname{Res}_X(H_j,M)
          =\prod_{i=0}^{30}M(x([w_j]P_i)).
\]

Each \(M(x)\) is a quadratic polynomial in the seven chart coefficients; \(g_j\) is a product of 31 such quadratics of total degree at most 62. It can be *evaluated* by 31 polynomial point evaluations without expanding its millions of potential monomials. This does not imply F4 can solve the constraints at comparable cost.

**Theorem (supplied-chart gate, conditional only on pairwise disjoint slot x sets):** \(g_j=0\) for all \(j=0,\ldots,7\) if and only if the eight zeros of \(M\) consist of precisely one distinct rational abscissa from each slot. Indeed, each zero resultant forces one root of monic \(M\) in the corresponding set. Disjointness gives eight distinct roots of degree-eight \(M\); there can be no other roots or multiplicities. Conversely the eight roots make every resultant vanish. Each root has exactly one sign on \(F=0\), because both signs would force a double norm root. These eight signed base points sum to \(R\) by the degree-nine function divisor relation. This is a **constructive exact group-replay certificate** once the chart is known, not an algorithm to discover it.

**Important obstruction:** without the disjoint-x precondition, eight zero resultants need not enforce one root per slot. Take slot 7 identical to slot 0. Choose one root in slot 0, two in slot 1, and one in each of slots 2–6. The resulting monic degree-eight polynomial makes all eight resultants zero, yet slots 0 and 7 have only one available root between them; Hall's matching condition fails. The implementation checks this exact counterexample. Any future system allowing overlapping images must add matching/Hall constraints or use a slot-labelled incidence formulation.

## Original-curve preflight (independent Python computation; not a solver benchmark)

With the 16 frozen deterministic eight-slot signed fixtures in the protocol:

| Check | Result |
|---|---:|
| Distinct x images, eight \(\times\) 31 points | 248/248 |
| Signed RR9 charts found from planted test-only leaves | 16/16 |
| Slot-resultant root sets, all eight slots per fixture | 128/128 singleton hits |
| Eight signed recovered points replay sum = public \(R\) | 16/16 |
| Same-\(R\) altered \((a_0+1)\) charts rejected | 16/16 |
| Duplicated-slot false-positive resultant counterexample | Algebraically checked |

The producer Rust must independently reproduce these frozen controls with the existing \`prime31-algebraic-ic\` arithmetic, and a separately written Python BigInt script checks its emitted JSON. In this experiment the **test harness itself** knows the planted leaves; the public certificate verifier receives only chart, target and publicly derived slot sets. Consequently no hidden public chart is recovered and no unknown ECDLP is solved.

## Prior leads checked and negative results not repeated

| Lead | Frozen standing and decision |
|---|---|
| [#4539](https://github.com/isogeny-labs/crypto/pull/4539) | Original signed RR9 chart and \(M\) identity: reuse unchanged |
| [#4692](https://github.com/isogeny-labs/crypto/pull/4692) | Draft RR9/F4 unknown-coefficient extraction; remain under test; avoid duplicate solver lane |
| [#4717](https://github.com/isogeny-labs/crypto/pull/4717) | Shared-base \(M\mid H_B^k\) multiplicity strata; distinct-image formulation avoids that specific shared-support structure |
| [#4720](https://github.com/isogeny-labs/crypto/pull/4720) | Signed two-leaf projector; useful as independent finishing control |
| [#4714](https://github.com/isogeny-labs/crypto/pull/4714) | Complete weighted M6 Rust 1,024-Q solver, generic 3+3 tables, about 21× slower than matched shared BSGS |
| [#4722](https://github.com/isogeny-labs/crypto/pull/4722) | Formal collision/yield ceiling for six shared weighted words; does not certify index-calculus advantage |
| [#4698](https://github.com/isogeny-labs/crypto/pull/4698) | Unverified source-level subexponential ECDLP argument, not an implemented practical 31-bit solver |
| [2018 APS prime-field IC](https://doi.org/10.1016/j.ffa.2018.01.009) | Published Semaev variant; source of prior art, not evidence that this 7-variable resultant solver is efficient |
| [Bochum 2026 subexponential proposal](https://github.com/bochum-crypto/subexponential-ecdl) | Theoretical binary tensor-network approach; different from this polynomial extractor, practical costs unmeasured here |

The repo's research-KB MCP/wrapper could not be reached through the available connector/tool set. This is a bounded direct GitHub and primary-literature check; it is **not** an exhaustive novelty claim.

## Prioritized decisive experiments

1. **Unknown-chart extractor gate (highest priority).** On public residuals without planted witness inputs, solve the seven-coefficient system \(g_j=0\). First use B4 and B8 with truly disjoint weighted images and independent exhaustive **grader-only** controls, preserve multiple-root and no-solution cases; compare a factored/saturated F4 encoding to the alternative 15-variable incidence system \(H_j(x_j)=M(x_j)=0\) for \(j=0..7\). Use pinned native Rust F4, quotient, rootfinding and arithmetic, not a new ECDLP solver. Track F4 dense/sparse columns, field ops, memory, all failed/censored queries, and witness completeness.
2. **Native complete B31 recoveries.** Only if gate 1 passes on unseen public R, feed natural signed relations into the unchanged rank-31 Rust learning and 1,024 blinded logarithm recovery. Absolutely no generic point-sum table, known scalar in the extractor, or rho fallback. Preserve the original curve and subgroup.
3. **Matched strongest-frontier test.** Fully charge setup, slot construction, every unsuccessful F4/constraint solve, relation rank, target recovery, memory and final scalar verification. Compare cold one-target and 1,024-target workloads to native mixed 4+4 IC, any-witness M6, shared cached/cold BSGS and optimized rho **on identical runner and targets**. Historical 1,024-Q BSGS 144.049 ms and mixed IC 1.9 s are producer-side reference values from other runs, not new measurements. No advantage can be claimed until this whole-solve panel is positive and independently validated.

**Research judgment:** The exact slot-root equivalence is a concrete way to combine the previously successful *weighted relation yield* with the RR9 target-conditioned *algebraic membership* architecture without generic tables. The hard part, efficiently solving the high-degree **factored** seven-variable system, is still open and is the sole gateway to a real solver claim.
