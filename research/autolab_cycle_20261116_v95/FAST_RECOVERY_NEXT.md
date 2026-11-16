# Next changed hypothesis after target-cache gains

The sum scan lost against the O(M) source scan on the larger fields. C95's response was target-batch cache reuse, measured with construction cost included. Its remaining O(MT) quadratic coefficient work is now a separate bottleneck.

For x!=u, put fx=f(x), fu=f(u), and choose h with h^2=fx*fu. Direct expansion gives

    (x+u)(xu+A)+2B = fx+fu-(x+u)(x-u)^2.

Therefore every third coordinate is

    y = (fx+fu +/- 2h)/(x-u)^2 - x - u.

Cache fx,fu as well as normalized square roots. Equal square classes give h=rx*ru; equal nonsquare classes give h=delta*rx*ru; a zero gives h=0; mixed nonzero classes supply no roots. The formula avoids constructing the quadratic's unused constant coefficient and much repeated coefficient arithmetic. Batch all nonzero (x-u)^2 denominators over the target panel. With x=u and fx!=0, use the tangent value (3x^2+A)^2/(4fx)-2x; with fx=0 smoothness excludes a finite root. Thus repeats, torsion and twist polynomial branches stay covered. This is standard chord/tangent algebra, not a novelty claim.

Prediction: cached direct recovery has lower total cold cost than the current cached coefficient pipeline, especially when increasing target count makes O(MT) coefficient work dominate. It remains O(MT) and output-sensitive. An optimization of this component alone cannot certify the global search or an F4/F5/F6 ratio.

The next strong comparator must also specialize square-root arithmetic: for p=3 mod4 compute r=a^((p+1)/4) and check r^2=a rather than use redundant generic Tonelli-Shanks powers. Cache field-dependent Tonelli-Shanks constants on BOTH arms for P-224, and consider independently verified Cipolla only in a separate variant. Otherwise generic reference overhead can exaggerate ratios. Enforce canonical M<p explicitly in the next batch API: current frozen fixtures satisfy it, but the C95 kernel validates only M<=1023 and assumes canonical input. Do not expose that prototype as a validated unrestricted API.

Fresh experiment: independent literal all-pair controls for tangent, zero, mixed/equal nonsquare and char3; then freeze target counts1/4/16/64 and source sizes31/63/127 across all five NIST fields before timing. Compare direct source, cached coefficient and cached chord variants with the SAME optimized root context and full setup/output costs. 90s initial worker,4GiB,onecore; no discarded slow fields or deadline failures. Independent exhaustive original-root verification for tractable domains. Larger targets/root sets, whole-chain sparse-block propagation and genuine matching F4/F5/F6 remain separate streams.
