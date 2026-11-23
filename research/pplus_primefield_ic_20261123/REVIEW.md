# Same-team review of the p+1 prime-field research report

Date: 23 November 2026  
Task: Isogeny Labs research handoff, PR #5049  
Reviewer: a separate model agent within the producing team  
Standing: **same-team model review; not signed PCC validation**. [pcc:unverified]

## Verdict and scope

**No blocking mathematical or claim-scope issue remains in the reviewed report.** The conclusion is appropriately limited to a norm-one/Dickson factor-base construction that fits the PKM prime-field framework, a bounded native qualification of that selector, and a full-domain line-FFT component. The report identifies efficient elliptic relation extraction as unfinished. It claims neither an ECDLP speedup nor a complete public-input logarithm recovery.

This review is pinned to `REPORT.md` with SHA-256:

```text
b7e870980ff1be173d7db6c98924aa0729cde00a1fbdad9a906e62eedd6935fa
```

It covers the report's mathematics, count definitions, cost accounting, attribution boundaries, and consistency with the native evidence inspected during this task. Relevant inspected artifacts include the selector's compact accepted output; `fft/line_fft.rs`, `fft/production.rs`, `fft/PROTOCOL.md`, and `fft/evidence/attempt-01/native-output.jsonl`; and the identity-record README and issuer/checker sources. Earlier in the task, the reviewer inspected the relevant PKM, Granger–Vercauteren, Cheon, Cheon–Kim, and subfield-curve primary literature. The final bounded pass did not repeat a broad literature search or execute the research programs.

The arithmetic references are separately implemented checks, but the evidence and this review remain within the same producing team. This is not an external peer review, independent execution replay, Lean proof, or signed proof of execution. Renderer output and final document layout are outside this mathematical review.

## Findings resolved before this snapshot

| Finding | Resolution in the reviewed report |
| --- | --- |
| The statement that `D_N−2` has repeated roots omitted low-degree exceptions. | Section 2 now restricts the assertion to the two-power cases `N>=4` and explicitly exempts `N=1,2`. |
| A nonzero target coefficient need not be invertible for composite subgroup order. | Section 4 now requires `gcd(beta,r)=1` and states the prime-order simplification separately. |
| Filtering solely for increasing factor-base rank could discard the dependency that recovers the target. | Sections 4 and 9 now use the augmented system including the target logarithm and explicitly preserve/process dependencies with invertible target coefficient. |
| The relation-collection formula could suggest an exact, constant-rank-yield theorem. | Section 9 now uses approximate accounting, `B+O(1)` useful rows, and a mean yield measured or bounded across the full stream. |
| Identity “certification” could be read as proving general identities or the Prime31 FFT. | Section 11 now identifies deterministic finite evaluations over `F_(2^61−1)`, null formal-proof fields, and the exact excluded conclusions. |
| The rejected-attempt custody pointer did not match the portable package. | Section 11 now points to the root-level `ATTEMPT01_CUSTODY.md`. |

No edit to `REPORT.md` was made by this reviewer. The author applied the corrections.

## Mathematical checks

### Torus and selector

For odd prime `p`, the norm-one subgroup of `F_(p^2)^*` has order `p+1`, and `z^p=z^(-1)` puts `z+z^(-1)` in the prime field. The Dickson trace identity and composition law support the selector construction. If an order-`N` torus coset and its inverse are distinct, trace is injective on that coset; the guard `c != ±2` is the correct regularity condition for the stated construction. Existence of a suitable coset is conditional, rather than asserted for every divisor without qualification.

For `p=2^31−1` and `N=2^k`, `0<=k<=29`, the report's zero-fibre argument is valid. The equation `z^(2N)=−1` has `2N` roots of exact order `4N`; `4N` divides `p+1`. Inversion has no fixed point on those roots, leaving `N` distinct prime-field traces. Degree `N` then proves exhaustion and squarefreeness. The production API's rejection of depth zero is correctly distinguished from the mathematical fibre `{0}`. The largest permitted mathematical depth does not imply a feasible base of that size.

The corrected discussion of `D_N−2` is accurate. In this two-power family, its distinct-root count for `N>=2` is `N/2+1`; repeated roots occur for `N>=4`. `D_1−2=X−2` and `D_2−2=X^2−4` are squarefree.

### Curve-order obstruction

The recorded curve trace is `p+1−2147487637=−3989`. The fixture order is explicitly attributed to the repository rather than newly recertified. Since the recorded order is odd and `p+1=2^31`, coprimality proves that every homomorphism from a group of that order into the norm-one torus is trivial. This statement does not require primality of the recorded order.

The more general argument is also valid: a common divisor `r` of `#E(Fp)` and `p+1` divides the trace. If `r>2 sqrt(p)`, Hasse's bound forces trace zero; for prime `p>3` this is supersingular. The report correctly treats this as an obstruction to a direct group transfer, while retaining coordinate selectors on ordinary curves. It does not confuse a necessary divisibility condition with an efficiently computable reduction.

### Elliptic relations and recovery

The retained `S_3` constraints are candidate decomposition equations. The report preserves rational point lifting, sign reconciliation, identity branches, actual elliptic verification, and final `[s]G=Q`. Its degree statement is correct: standard `S_3` has degree two in each variable and total degree four. A dependency gives scalar recovery only when its target coefficient is invertible modulo the subgroup order. The augmented-system clarification closes the potential rank-filtering error.

## Counts and native component evidence

The factor-base cardinality `B` consistently counts rational negation-orbit columns, rather than the full selector domain or signed affine points. The accepted selector output agrees with every displayed row: the values of `B` for depths 5 through 12 are `15,34,72,116,263,516,1001,2062`. No selected point has `y=0`, so signed-point counts are exactly `2B`. The total `8190` production roots is `2+4+...+4096`; the additional depth-zero control gives `8191` checked roots. These are direct finite counts, not a half-density estimate or relation-yield measurement.

The forward/inverse line FFT is correct at the stated algebraic level. With

```text
b_i(X) = product_j D_(2^j)(X,1)^(bit_j(i)),
```

the identities `b_(2i)(X)=b_i(X^2−2)` and `b_(2i+1)(X)=X b_i(X^2−2)` justify the parity split and the butterfly. The domain ordering supplies sibling roots `x,−x` above the parent `x^2−2`; the nonzero twiddle condition makes the inverse legal. The recurrence gives `O(N log N)` transform arithmetic for hierarchical-basis input and precomputed domains/twiddles. The report charges setup and conversion separately.

The reported evidence totals agree with the inspected output: four vectors at each depth 1 through 8 yield 32 forward comparisons, 32 roundtrips, 32 quotient-product comparisons, and `4(2+4+...+256)=2040` compared forward values. Independent `u128` arithmetic, monomial Horner evaluation, and schoolbook multiplication/reduction provide meaningful implementation cross-checks. Deterministic samples and deliberate corruption controls do not prove all inputs or all depths.

The multiplication claim is correctly restricted to `Fp[X]/(D_N)`. The report distinguishes `O(N)` live transform storage from the reference harness's `O(N^2)` expanded-basis storage. It makes no timing claim.

The full domain's uniform folding does not automatically survive filtering by elliptic liftability, adding selected `y` values, or passing to actual elliptic pair sums. Nor does a fast transform reduce a full `m`-leaf tensor representation's dimension `N^m`. Sections 5 and 6 preserve all these limitations.

## Cost-model cross-check

The report's collection model is appropriate as an accounting model, subject to its explicit hypotheses. For fixed arity `m`, the heuristic

```text
lambda approximately (2B)^m/(m! r)
```

is a leading signed-tuple count under uniform-sum assumptions. Distinctness, cancellation exclusions, rank efficiency, and the actual structured distribution are separate issues. A one-row hit probability does not describe the useful yield of a solver returning many rows.

A precise exponent form confirms the four-summand discussion. Suppose

```text
B = r^(b+o(1)),
C_attempt = r^(c+o(1)),
conditional useful-row efficiency = r^(-e+o(1)),
```

with fixed `m` and the stated uniform-sums heuristic. The relation-stage exponent is

```text
b + c + max(1−m b, 0) + e.
```

Conventional sparse black-box linear algebra contributes exponent `2b`. Therefore a strictly smaller-than-one-half exponent within this model requires

```text
b < 1/4,
b + c + max(1−m b, 0) + e < 1/2,
```

as well as setup and recovery exponents below one-half. These are model-dependent requirements, not universal lower bounds on ECDLP.

At the balanced base `b=1/m`, the linear-algebra exponent is `2/m`: it equals one-half for `m=4` and falls below it for `m>=5`. That observation alone says nothing favorable about the extractor exponent. For the undersized four-summand base `b=1/5` and subpolynomial efficiency loss (`e=0`), the relation exponent is `2/5+c`. Thus the report's conditional `c<1/10` threshold is correct. Four summands are not universally excluded.

The phrase “below `r^(1/10)`” must continue to mean a strict exponent improvement when claiming a strictly lower total exponent. A constant or logarithmic reduction from `r^(1/10)` alone would not establish that conclusion. The current report expressly includes strict asymptotic conditions; an optional clearer replacement is:

> With `B=r^(1/5+o(1))`, fixed arity four, subpolynomial rank-efficiency loss, and `C_attempt=r^(c+o(1))`, the relation-stage exponent is `2/5+c`. A strictly lower-than-one-half exponent requires `c<1/10`, together with the stated setup, recovery, and linear-algebra conditions.

Duplicate filtering and incremental rank tests must be included in `C_attempt` or `C_LA` during implementation; the complete-cost definition already allows this. The mean useful-row efficiency cannot be assumed equal to an early-run estimate after the matrix approaches its required rank.

## Attribution and proof boundaries

The report appropriately calls the norm-one/Dickson construction an instantiation of the PKM selector framework, rather than a newly proved fast ECDLP algorithm. It retains the proper-extension hypothesis in the published torus and subfield-curve approaches. It correctly distinguishes the field characteristic from the group order in Cheon's auxiliary-input algorithm and states that ordinary public `G,Q` inputs do not supply the required higher unknown powers. Williams' method remains classified as integer factorization. Historical repository counts are labeled historical and are not presented as newly executed results.

A primary citation for the named MOV attribution would improve bibliographic completeness, without changing any argument in this report: A. J. Menezes, T. Okamoto, and S. A. Vanstone, “Reducing elliptic curve logarithms to logarithms in a finite field,” *IEEE Transactions on Information Theory* 39(5), 1639–1646 (1993), <https://doi.org/10.1109/18.259647>. The report's coprimality and trace arguments are direct derivations and do not depend on importing a generic transfer claim.

The identity records are now described at their exact scope: fixed finite evaluations over `F_(2^61−1)` of three stated expressions, with separate checker-process evaluations and negative controls. They do not prove polynomial identities, denominator nonvanishing, the Prime31 FFT executable, the general root theorem, or ECDLP complexity. The second checker process does not make the producing team independent. Null formal-proof fields and unsigned status are disclosed.

## Remaining work

No mandatory mathematical correction is proposed for the pinned snapshot. Optional improvements are the explicit exponent sentence above, specifying that the selector's exact-order check concerns its associated torus lifts, and adding the primary MOV citation.

Final package/render checks should preserve the reviewed mathematics and resolve the figure paths. Efficient public-target relation extraction, complete recovery runs, comparative performance, formal proofs, and signed PCC remain future work as the report states. This review does not authorize promoting any of those outstanding outcomes to completed results.
