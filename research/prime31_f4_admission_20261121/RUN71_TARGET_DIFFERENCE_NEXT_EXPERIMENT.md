# Next highest-information Prime31 experiment: target-certificate difference rows

Date: 2026-11-22 UTC. Research proposal only; **not implemented, measured, or evidence of a non-generic advantage**. Existing draft PR #4901, with extraction owned by #4692/#4812/#4822.

## Algebraic result (unconditional, elementary)

Let \(E(\mathbb F_p)=\langle G\rangle\) have prime order \(n\). Fix public factor-base points \(P_i\), write \(L_i=\log_G P_i\), and fix any public target \(Q_t\), with unknown scalar \(x_t\).

For every independently verified signed decomposition
\[
Q_t+[j_{t,r}]G=\sum_i c_{t,r,i}P_i\quad(c_{t,r,i}\in\mathbb Z)
\]
we obtain a **genuinely right-hand-side-labelled linear equation**
\[
\sum_i c_{t,r,i}L_i - x_t = j_{t,r}\pmod n.
\]
Subtracting two certificates for the *same* unknown target eliminates \(x_t\):
\[
\sum_i(c_{t,r,i}-c_{t,0,i})L_i=j_{t,r}-j_{t,0}\pmod n.
\]
The RHS is known from the public shift counters, not planted from an unknown logarithm. This holds for arbitrary arity, including signed 3-sums and the 8-slot RR9 candidate, and is independently checkable in the original elliptic-curve group law. It generalizes run69's use of one certificate per target: additional target certificates are not merely alternative readouts, **they also supply new inhomogeneous relations**.

With \(T\) targets and \(K\) certificates per target, at most \(T(K-1)\) difference rows are offered. **This is a count, not a rank guarantee**: signs, repeated points, support gaps, dependencies, and nonrandom degree distributions may make their true incremental rank much smaller. It is possible for the target-specific quotient to be determined while base logarithms remain undetermined. Use exact modular rank, not a row-count surrogate.

## Reuse and no-duplication plan

1. Keep original Rust \`Arithmetic, Base, PRIME31\`, B677, 88 verified homogeneous rows, and true-RHS signed DSU from runs 68/69. Do *not* rebuild an EC engine or silently change the ordinary cofactor-one \(p=2^{31}-1\) curve.
2. Run70 separately measures the **readout-only** effect of first 1/4/16 target certificates, holding the authentic relation stream fixed. Its source-bound gate must pass before interpreting the numbers.
3. For Run71, use a new sparse equation structure: each verified \`(public_q,shift,coefficients)\` certificate adds an equation over \(677+T\) unknowns, with \(-1\) in its target-log column, or add difference rows over the 677 base coordinates. Deduplicate exact rows and perform mod-\(n\) sparse elimination. Record *incremental ranks* of free rows, target-difference rows and true \([k]G\) relation rows separately. Preserve run69's exact target-specific row-space stop rule; do not artificially force full factor-base rank.
4. Preregister paired K=1/4/16/32 and T=1/32/128 with the same deterministic public target batches. For the reference oracle only, allow run69's explicitly GENERIC 916,469-key signed-pair table; record every dictionary build, attempted leaf, group proof, failed public shift, rank operation, and memory. Test both \`kG\` relation budget zero and bounded positive budgets. Verify all logs via original Rust \`[x]G=Q\`, post-solve harness scalar and independent BSGS.
5. Gate exact inequalities: adding correctly verified certificate rows cannot reduce the row span. However wall time can worsen from extraction and linear algebra; report the first actual break-even K, including negative results. Never project a faster *complete* solver from rank or relation count alone.
6. **Qualification step is owned by #4692/#4822/#4812:** replace BOTH generic pair dictionary and generic triple target enumeration by the existing public-only unknown-index RR9/Horner/F3 root extractors; registered 46 positive/18 negative cases, failures/timeouts/memory, 1/32/128/1024 cold and amortized real logs, strongest existing index-calculus frontier and optimized BSGS/negation rho on the same runner. Stop as censored if the source-specific blind extractor gate fails.

## Why this lead outranks further shift-correlation or micro-optimizations

Run65 found negligible eight-shift coverage differences. Run69 already validated full real-RHS target-specific rank and obtained 32/32 actual logs with 34.95% fewer relation shifts, but stayed ~10.2x slower than its same-process reference BSGS due a generic pair dictionary. Multiple decompositions may increase **actual rank per expensive oracle call**, not simply reduce nanoseconds on a generic EC probe. Crucially, the new difference equation is available with *no known target scalar*, making it a meaningful input to a genuinely algebraic extractor if the RR9/F3 lane begins yielding multiple public roots.

## Prior-art and provenance

Exact preceding run69 source/result: \`research/prime31_ic_20261115/native/examples/prime31_run69_genuine_partial_rank.rs\`, \`RUN69_RESULTS.json\`; runs 59/67/68 in #4901. Extraction remains separately owned by #4692 (registered RR9/F4), #4822 (sparse four-variable Krylov), #4812 (Horner/F3), and #4809 (quadratic S3). The general approach of computing relations and recovering targets by linear algebra is classical index calculus, and Amadori–Pintore–Sala (2018, DOI 10.1016/j.ffa.2018.01.009) already explores a one-relation prime-field strategy. **No novelty or sub-generic complexity is asserted here.** Research-KB MCP and local Rust compiler were unavailable; direct primary GitHub branch files and published bibliographic records are the checked fallback.

**Success metric:** actual new independent labelled rank and verified complete public target logarithms per fully charged public algebraic oracle millisecond, not raw decomposition coverage, planted consistency checks or stand-alone field-operation time.
