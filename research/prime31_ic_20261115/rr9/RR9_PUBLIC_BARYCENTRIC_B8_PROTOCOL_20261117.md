# Preregistered public B8 barycentric RR9 chart experiment — 2026-11-17

**Scope:** actual ordinary prime-field curve `p=2147483647`, `a=847514846`, `b=2087444724`, prime group order `n=2147446831`, cofactor one. Reuse the existing native Rust algebraic IC library and original RR9 factor-polynomial checks. The candidate receives only public R=(r,s), public B8 factor-base coordinates and arithmetic. It explicitly enumerates all 256 signed choices. This is a generic support-enumeration control, not a non-generic algorithm or an ECDLP solver.

## Fixed positive/negative panel (before measurement)

- B8 abscissas [0,1,2,3,5,6,8,9], with each public quadratic-residue ordinate canonicalized to min(y,p-y).
- Positive public R=(1911579686,548820096).
- Off-support negative public R1=(625037715,2100627251), R2=(1923055775,307731458).
- Eight sign masks = 256 candidate systems per residual, with neither oracle support choice nor hidden scalar passed into the extractor.
- Success: one valid normalized chart on the positive, no chart on either negative, all coefficient identities M=H_B and independent original-curve signed group sums replay. Record wall per residual (ns), not a complete-solver time.

## Exact algebraic elimination

On nonexceptional B8, for each signed orientation epsilon_i and base point (x_i,y_i), put

    c_i(epsilon) = (s + epsilon_i*y_i)/(x_i-r),
    w_i = 1/prod_{j!=i}(x_i-x_j),
    V(X) = X^3 + v2*X^2 + v1*X + v0.

The selected signed fibre equation is A(x_i)+c_i V(x_i)=0 with deg A<=3.
For distinct eight x_i, values f_i interpolate a polynomial of degree <=3 **if and only if**

    sum_{i=1}^8 w_i*x_i^k*f_i = 0, k=0,1,2,3.

Proof: the forward direction is the top-coefficient Lagrange identity applied to X^k A(X), whose degree <=6. The four moment equations are independent (weighted Vandermonde rows of rank four), and their common kernel has dimension 4, equal to the evaluation image of cubics. Substituting f_i=-c_i V(x_i) yields a **4-by-3 affine linear system** for v0,v1,v2. Consistency is checked by exact finite-field RREF. When consistent, free v-coordinates may be set to 0, and A is uniquely interpolated from any four distinct nodes. All eight fibre equations and M(X)=H_B(X) are checked independently. For the squarefree B8, nonexceptional, monic-V chart this is a complete test over F_p; it is *not* complete over all chart normalizations or larger/repeated B31 branches.

## Performance/science guardrails

- Compare with test-oracle signed point group sum only **after** chart extraction; no supplied 7-leaf anchors, masks, planted coefficients or scalar labels in the candidate.
- The method still performs 256 candidate tests, so it does not solve the public-residual nonenumerative gate of #4548–#4550.
- Use GitHub Actions pinned Rust 1.100.0 with immutable native library commit 1df714645aa683cf975bc76fcacb9842cb4780fd. Run original lib tests, example tests and release JSON validation. This has no user-costly compute requirement.
- Keep B8 timing strictly separate from full B31 unknown-log benchmark #4532 (1024Q mixed IC 2221.474 ms; cached 2^20 BSGS 170.113 ms, producer only). No extrapolation from 256 masks to a complete 31-coordinate solver.

## Prior-art / duplication audit

- Crypto RR9 sequence: #4539 chart, #4541 radical, #4545 signed fibres, #4546 exceptional x=r, #4547 seven-anchor generic equivalence, #4548/4549 dense D5/D6 null experiments, #4550 D9 exact criterion.
- Crypto speed frontier: #4513 generic-equivalent mixed-base batch vs rho; #4532 cached shared generic BSGS.
- Crypto autoresearcher #3301 (39 lead atlas dispositions) and #3305 (Semaev alternatives/errata), cryptanalysis #1536 (matched auxiliary/random/same-base controls).
- Primary literature: Petit–Kosters–Messeng PKC 2016 DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala FFA 2018 DOI 10.1016/j.ffa.2018.01.009; Kudo–Yokota–Takahashi–Yasuda CANS 2018 DOI 10.1007/978-3-030-00434-7_19. They already motivate hybrid/exhaustive algebraic IC and limit novelty claims. The barycentric step is standard interpolation algebra.
- Research-KB MCP and repository wrapper not available in this connector-only environment. Exact existing GitHub primary records and publication repositories are the fallback; this is NOT an exhaustive corpus audit.

## Falsification and next gate

If the candidate returns the positive chart but negatives zero, it validates a cheap **enumerative** algebraic oracle. A productive next algorithm must remove the 256-sign dependence (and B31 choose-eight support search) while preserving exact positive/negative fibres. Test seven-equation quotient compression (generic Bezout upper bound 128, actual dimension unmeasured) and D9 sparse negative unit certificates **without sign input**. Any future prime31 B31 run must solve blinded unknown logarithms end to end, charge missed relations, setup, factor-base logarithms, sparse LA and memory, and beat same-host cached BSGS and optimized rho with independent replay.
