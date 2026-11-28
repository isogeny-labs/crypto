# Conditional moment search contract

For five sources in V, g_q=sum(x_i^q)+e1^q for q=3,5,9 lies in the joint F2 image of pair generators v_i^(2^j)v_k+v_i v_k^(2^j), j=1,2,3. This follows by expanding the characteristic-two power of the sum; repeated sources are included. Gaussian elimination on this image yields necessary and sufficient linear tests for image membership. These tests are necessary for supported quintics, not sufficient for support.

The circuit uses the exact tested Newton expressions, with Boolean AND and affine XOR definitions. Every primary assignment has a unique gate extension. The small oracle separately uses the general Newton recurrence and exhaustive 16-point truth tables, then enumerates native projected SAT models and compares their entire primary sets. Zero quintic X^5 supplies a repeated-root positive control.

The actual 69-bit fixed-h/k chart is inherited from the pinned affine certificate; it is not all charts of the original problem. Preliminary models are never blocked before adding g9. Full-stage models are independently checked and blocked in all 69 primary coordinates, preserving norm aliases. A capped, unknown or partial enumeration is inconclusive, never empty/all-roots output. Any observed conditional_moment_unsat refers only to the residual conditional necessary-model search, not the original system or supported-root count. Every reported model is only a necessary-moment candidate.

Budgets: one core, 4096 MiB, 90 s supervisor, 84 s final output, 2/18 s native stage budgets, maximum 8 models/stage, 200000 gates, 300000 variables, 50M XOR literals, raw 128KiB/gzip32KiB/stdout8KiB. Setup, verification and serialization charged. Novelty and qualified performance are unclaimed.
