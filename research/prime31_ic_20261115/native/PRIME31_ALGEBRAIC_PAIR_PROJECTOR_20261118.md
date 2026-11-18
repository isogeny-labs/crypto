# Prime31 original-curve signed-pair algebraic projector: exact mechanism and bounded controls

2026-11-18 UTC · PR [#4720](https://github.com/isogeny-labs/crypto/pull/4720) · \`[pcc:unverified]\`

## New precise result

For the **unchanged** ordinary cofactor-one source curve

$$
E: y^2=x^3+847514846x+2087444724 \quad \text{over }\mathbb F_{2147483647},
$$

with prime group order $n=2147446831$, the public-target two-summand signed-membership problem has an exact **homogeneous-polynomial** screen requiring no point-sum table and no per-candidate field inversion.

For the first rational-x factor base $B\in\{4,8,31\}$, let $H_B(T)=\prod_{j<B}(T-x_j)=\sum_{k=0}^{B} h_kT^k$. Given a public point $Q=(u,v)$ and positive base point $P_i=(x_i,y_i)$, define

$$
D_i=(u-x_i)^2,\quad
N_i=(v+y_i)^2-(u+x_i)D_i,
$$

$$
\mathcal P_i(Q)=\sum_{k=0}^{B}h_kN_i^kD_i^{B-k}.
$$

When $u\ne x_i$, $\mathcal P_i(Q)=0$ **if and only if** there exists $P_j$ in the base such that $Q=P_i\pm P_j$. This follows from the addition law $x(Q-P_i)=N_i/D_i$ and the exact root set of $H_B$. Upon a positive screen, replay the signed witness by group subtraction, checking the correct sign. The homogeneous polynomial costs $3B+O(1)$ base-field multiplications per candidate $i$, but no field inversions or EC additions on negative decisions.

### Proven exceptional branch

Simply clearing denominators is **incorrect** when $u=x_i$. At $Q=-P_i$, both $D_i$ and $N_i$ vanish, so the unguarded polynomial reports a spurious root even if $x(Q-P_i)=x(-2P_i)$ is outside the factor base.

The correct branches are:

- $Q=P_i$: then $Q-P_i=O$ and has no finite $x$-coordinate; skip candidate $i$.
- $Q=-P_i$: use the tangent $D_t=4y_i^2\ne0$ and $N_t=(3x_i^2+a)^2-2x_iD_t$, evaluating $H_B(N_t/D_t)$ by the homogeneous polynomial.
- $Q=O$: immediately return the signed witness $O=P_0-P_0$.

Since the group has odd prime order, the chosen base points have no $y_i=0$. The controls explicitly reject the unguarded false positive at $-P_0$.

## Fixed-panel independent arithmetic replay (not a Rust benchmark)

An independently written Python BigInt implementation of the exact original curve, factor-base selection, homogeneous rational-coordinate identity, full signed group law and deterministic public targets was executed in this run. It checked every deterministic case in the [frozen protocol](PRIME31_ALGEBRAIC_PAIR_PROJECTOR_PROTOCOL_20261118.md) before describing the outcomes:

| Base | Public target cases | True signed membership | Cases with target x matching a base x |
|---|---:|---:|---:|
| B4 | 169 | 33 | 8 |
| B8 | 273 | 129 | 16 |
| B31 | 273 | 129 | 16 |

Counts include **deliberately planted** $P_i\pm P_j$ cases, duplicates, identity and vertical exceptions, and 128 additional deterministic scalar-generated public targets per base. No natural random-target hit occurred in the 128-target segment; such absence is expected under the following bound, not evidence of solver failure.

The source-specific Rust example [`prime31_signed_pair_projector.rs`](examples/prime31_signed_pair_projector.rs) reuses the original `Arithmetic,Point,check_curve,PRIME31` code, generates the same public targets, compares each screened answer against an independent **O(B) group-subtraction + x-index lookup** control (not a generic O(B^2) pair table), checks every individual screen against point subtraction, and outputs counts plus diagnostic times. The separate Python verifier [`verify_prime31_signed_pair_projector.py`](verify_prime31_signed_pair_projector.py) reconstructs the public cases and replays the signed point and exceptional branches using independent modular-integer arithmetic.

**Rust source-specific CI status:** [PASS, exact-source run #44326205507](https://github.com/isogeny-labs/crypto/actions/runs/44326205507), commit `03cb38541b17e0fe134f35bab1777c6da912d9b0` (40 pinned original Rust library tests, 2 new focused tests, full 715-case JSON diagnostic and separately authored Python BigInt verifier PASSED). The earlier E0308 tuple/Option mismatch at run #44325258716 was fixed before this successful run. The workflow [`prime31-algebraic-pair-projector.yml`](../../../.github/workflows/prime31-algebraic-pair-projector.yml) pins source revision `1df714645aa683cf975bc76fcacb9842cb4780fd` and Rust 1.100.0.

## Source-specific counted costs (diagnostic, not solver speed)

The passing GitHub runner produced the following same-process original-curve component counts, including all planted cases and the random negative-target segment:

| B | algebraic field multiplications | algebraic screen EC additions / field inversions | group-subtraction control EC additions / field inversions | recorded algebraic / control time |
|---|---:|---:|---:|---:|
| 4 | 9,016 | 28 / 28 | 604 / 600 | 0.038 / 0.193 ms |
| 8 | 43,232 | 120 / 120 | 1,608 / 1,600 | 0.154 / 0.465 ms |
| 31 | 471,584 | 120 / 120 | 4,920 / 4,912 | 1.220 / 1.443 ms |

The algebraic counts exclude the EC verification/replay costs, which appear separately; these times are **unbatched diagnostic controls**, not matched optimally batch-inverted EC arithmetic, not a fully charged solver comparison and not an end-to-end speedup claim. [Raw source-bound JSON artifact](https://github.com/isogeny-labs/crypto/actions/runs/44326205507/artifacts/14436523992), sha256 `8ff5fecd3877f7e2eea0efd6455af33f1c56218087f0668a94c635d7b2f3ee6c`.

## New exact yield ceiling and its consequence

For a signed two-summand base of $B$ distinct positive points, at most $2B^2$ group elements are representable as $P_i\pm P_j$. Consequently, on this prime-order group, a uniform public residual has probability at most:

$
\Pr[R\in S_{2,B}^{\pm}]\le\frac{2B^2}{2147446831}.
$

At B31 this is **8.9501633859 × 10^{-7}**, less than one in 1.1 million. A 128-query uniformly random panel has an expected hit count at most **0.0001145621**. This is an exact uniform-support upper bound, not an independence/Poisson approximation and not a target-conditioned F4 cost.

This **rules out using the signed two-point screen alone** as a viable whole-solve target extractor on this fixed curve. Its proper use is a *finishing leaf*: first recover a public-residual-conditioned four-summand algebraic partial decomposition, then test its final two signed leaves by this exact screen. Absent efficient four-summand extraction, the component does not yield a useful full solver.

## Prior lead audit and next discriminating experiments

The literature review checked Amadori–Pintore–Sala (2018, DOI 10.1016/j.ffa.2018.01.009) and the October 2026 Bochum subexponential-ECDLP preprint. Both motivate algebraic decomposition, but neither makes this low-degree identity novel. Research-KB MCP is not available, and the repository checkout needed by its CLI is absent; this audit used GitHub source files and papers/primary records as a bounded fallback.

- [#4714](https://github.com/isogeny-labs/crypto/pull/4714) is the useful exact complete M6 Rust negative benchmark: 1,024/1,024 logs and 31/31 rank, but generic 3+3 relation enumeration loses **20.85–23.32x** to same-runner shared BSGS on the earlier parity variant; later any-witness M6 improves 1.44x against its own parity control, not against same-runner BSGS.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692) remains the separate draft RR9 public-residual algebraic extraction effort. Avoid duplicating its shared-coefficient tower work; test the new pair-screen as its *bounded final-leaf predicate* only after it yields candidate four-summand partial decompositions without generic tables.
- [#4717](https://github.com/isogeny-labs/crypto/pull/4717) supplies low-power (H_B^k) tests for a **known** degree-eight polynomial; it does not extract an unknown polynomial from Q.
- [#4698](https://github.com/isogeny-labs/crypto/pull/4698) and [Bochum 2026](https://github.com/bochum-crypto/subexponential-ecdl) offer asymptotic Boolean-network leads, not measured crossovers at 31 bits.
- [#4701](https://github.com/isogeny-labs/crypto/pull/4701) accelerates a degree-three isogeny inverse, not a relation extractor.

**Priority 1:** Complete B4/B8 RR9 public-Q signed four-leaf extraction (positive, negative, repeated-root, vertical cases) and apply this B2 finishing projector; insist on zero 3+3/4+4 point-table construction.

**Priority 2:** Promote to B31 and measure naturally discovered independent signed relations, full-rank modular solve, and 1,024 independently blinded source-curve logs in the reused Rust recovery pipeline; record all failed fibres and setup, do not pre-seed witness scalars.

**Priority 3:** Same-host A/B/B/A vs current mixed 4+4 index calculus, M6 any-witness, cold/cached shared BSGS and optimized rho. Frontiers from #4714 supply a **screening budget**, not a measured projected runtime: its best shared BSGS was 144.049 ms for the whole 1,024-target batch, so any candidate's *entire* batch must beat that, not merely the algebraic predicate.

No cheaper **complete non-generic** prime-field ECDLP solver is claimed by this bounded component. Independent signed PCC remains pending.
