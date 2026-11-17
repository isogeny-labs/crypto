# Conditional doubled-order phase rescue

T-1879. Derived identities and producer checks; independent signed verdict pending. [pcc:unverified]

## Orbit recombination

Retain the predecessor's caller-certified prime scalar order r, exact-order zeta and public-point source. Assume m >= 3 odd and 8m divides r-1, so h=(r-1)/2 is divisible by 2m. Put eta=-zeta^((m+1)/2). Then eta^2=zeta and eta^m=-1, proving eta has exact order 2m. The two disjoint sets b<zeta> and -b<zeta> are precisely b<eta>.

Because h is divisible by 2m, the compensated mode weight at an offset b*w is w^e. On the negative coset, this weight is (-1)^e zeta^(je). Therefore, for e=0,1,2,

    f_e^(2m)(b) = f_e^(m)(b) + (-1)^e f_e^(m)(-b).

This exact identity holds for arbitrary signs (including zero), not only signs arising from a scalar fixture. The implementation needs only three field additions/subtractions; it does not requery or retain the two full sign vectors. For an explicit ordered DFT reference, interleave positive[j] with negative[j+(m+1)/2 mod m] to obtain the eta order. No normalization factor of two is introduced into the observation; A changes from m*b^h to 2m*b^h in the polynomial model.

The binomial coefficient sections obey

    F_e^(2m)(Y) = sum_j a_(2j) Y^j,
    F_e^(m)(V) = sum_j a_j V^j,  Y=V^2.

The low-mode combination thus supplies a complete standard order-2m input for the existing direct support and phase code. The old phase exponent n=(r-1)/(4m) becomes n/2. Initial upper F0 coefficient deliveries are n/2+1 rather than n+1 for this additional attempt.

## Conditional control flow

Run the original order-m recovery path at the first radius, including its materialization, phase, history and extra-offset exits. Save only its radius and three observations for possible reuse if execution proceeds to the next radius. If that radius is its negative and the parity/divisibility conditions hold, recombine the observations and attempt an order-2m singleton phase recovery.

If cap<2m, reject the attempt before constructing a new coefficient context. Otherwise create and validate eta, a separate order-2m coefficient context, and its direct support. Materialize under the existing cap and image cap floor(cap/(2m)). A nonsingleton support, cap miss or zero phase denominator yields no rescue. A singleton supplies its phase through the existing F1 evaluator, and the normal elliptic-curve multiplication verifies the proposed scalar before return.

On a miss, continue the original order-m update using the unchanged compressed state and source history. The rescue uses its own context and image and never replaces the original state. It is attempted at most once, at the second radius. Unsupported schedules follow the same original path. Identity, endpoint and first-radius exits are unchanged.

## Source-query property and costs

The added helper has no source argument and no group-query operation. Before a successful rescue, both executions therefore query the same sequence; after a miss, the original execution resumes with its original state. The rescue can only terminate that sequence earlier. This proves nonincrease of source calls under the same input and caller-certified conditions, assuming the exact arithmetic succeeds. It is not a proof of reduced runtime: a failed rescue adds work.

Added costs include a retained three-entry observation vector, eta exponentiation, exact-order checks, a fresh coefficient-product context, four folded arrays of n/2 entries, n/2+1 upper F0 deliveries, all C0/C2 quotient work, materialization, phase evaluation and final group verification on success. The original first-radius construction has already been paid. Counters distinguish attempts, contexts, cap misses, nonsingletons, zero denominators and phase candidates; upper-F0 deliveries exclude F1/F2 and setup and must not be reported as total coefficient work. Existing source-grid construction, order checks, source queries, copies, allocations and failures remain charged. No timed or full-operation comparison is run.

Recombination uses only the three low doubled modes, so it need not preserve the full paired-m support. It can discard information from the high doubled modes. Safety comes from final point verification and retaining the complete original fallback, not from claiming equality of the two support representations.

## Checks and sources

The separate checker verifies exact coefficient sections, the three scalar-polynomial identities for every scalar and three radii in seven configurations, and 729 arbitrary sign vectors per configuration. m=3 panels exhaust all 3^6 vectors; m=5 panels are explicitly partial. A wrong-plus-sign control detects the missing negative-coset orientation. Public recovery compares every target at fixed caps 8,10,16,20,32,64; it asserts per-target source-call nonincrease, preservation of every baseline success and first-radius exit, with unresolved cases retained.

Sources checked: feature-native-opposite-phase/{DERIVATION.md,opposite-adapter.rs,direct-adapter.rs,context.rs,streamed.rs}; NIST DLMF 26.3 binomial identities, https://dlmf.nist.gov/26.3; FFTW's transform-definition documentation, https://www.fftw.org/fftw3_doc/What-FFTW-Really-Computes.html, for the need to fix sign and normalization conventions. The finite-field formula above is derived directly and does not rely on FFTW's numerical implementation. KB MCP unavailable; scripts/research-kb status reports no executable. No novelty claim.

Next: remove the extra dense folded arrays by streaming the doubled coefficient section into the needed quotient, or reuse compatible first-radius coefficient setup. Establish an operation-level accounting before adopting the rescue as a default based on call savings alone.
