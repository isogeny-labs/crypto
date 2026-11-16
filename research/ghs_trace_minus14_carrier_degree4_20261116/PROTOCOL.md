# Trace -14 carrier census, corrected degree-four denominator

Frozen 16 November 2026 before execution. The previous exact-pair source
`../ghs_trace_minus14_20261116/degree4_all.sage` used `B=x+1` in its
`linear_only` arm, while its carrier's unique rational branch factor is `x`.
That arm does not cover `B=x`, so the previous degree-four exclusion must be
rechecked. The source was adapted from the trace +26 carrier, whose branch
factor was `x+1`. This amendment preserves the old receipt and tests the
correct denominator rather than rewriting the historical run.

The new source tests all twelve trace -14 monic odd-degree carrier masks in
the earlier 84-model census: `9735,17371,21624,44227,64725,90937,92153,
100118,124586,144548,156795,159256`. The field is
`K=F3[a]/(a^5+2a+1)`, the source is `v²=u³+u²+a²+2` of order 258, and each
carrier is `y²=f_m(x)` with `f_m=x^11+Σ digit_3(m,i)x^i`.

For each carrier, find its unique monic rational branch factor `L` and five
quadratic factors `Q_i`. Enumerate the four complete normalized degree-four
denominator shapes `L`, `L³`, `L(x-c)²` for every nonbranch rational `c`, and
`LQ_i`. At every unpoled `Q_i`, enumerate all three compatible elliptic
two-torsion residues, use CRT to recover the unique numerator of degree at
most four, and verify every surviving candidate against
`f C²=B(A³+A²B+bB³)`. Save all survivors and exact hits. There should be
59,697 branch/parameter assignments per carrier. Any hit requires separate
map, subgroup, and same-target cost verification before a vulnerability claim.

Prior-work context: [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf)
and [Howe 2022](https://arxiv.org/pdf/2110.04221) are reviewed in the previous
trace -14 protocol. This is a local correction and a broader finite screen;
the source audit does not establish publication priority.
