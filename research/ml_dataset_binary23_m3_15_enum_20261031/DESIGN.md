# Natural ordinary-binary n23 arity block: pre-data design

This frozen block has 26 native cells: one seeded ordinary-binary curve,
generator and key for each twist, crossed with arities m=3..15. The same
public input is reused across m within each twist. All 26 indices remain in
the denominator after success, timeout, budget stop or construction failure.
It is the binary block of the requested 78-cell Koblitz/binary/prime portfolio;
the separate prime block has not been frozen here.

The pinned clean native build is source `c41bec150e16e4c4f9a00634002b633b821de262`,
binary SHA-256 `b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.
Its preflight reports 26 planned and zero generated cells. The native target is
an unconditioned sampled subgroup scalar, so a PDP witness is not guaranteed.

The requested factor base is three points. Construction can retain the second
sign and return four; the actual size and negation action are recorded per
cell. Enumeration visits a depth-first prefix and fsyncs each group-add
transition. With four points, a complete m=3 search takes at most 84
transitions, m=4 at most 340, and m=5 at most 1364. The fixed 4096-transition
limit bounds m>=6 prefixes; it does not prove decomposition absent. The 60s
inner and 150s outer limits separately censor slow curve construction or
filesystem work. These limits respond to the earlier eight-point Koblitz
wave, where m>=5 reached its 30s cap, while preserving every requested arity
and both twists. The 26 outer generation calls have a hard 3900s aggregate
bound; the campaign ceiling is eight hours including finite verification and
custody stages. No timing comparison is planned.

Each attempted cell retains native status and counted transitions, exact
curve/factor-base/Frobenius metadata, verify/report/export receipts, restricted
archive, and fresh S3 archive/pointer readback. Ordinary-binary coefficients
may lie in the full field; no nontrivial Frobenius symmetry reduction is
credited without measured coefficient-field degree and physical transport.
Strict public ML export excludes private scalar and construction witness
features. Scalar labels require independent native recovery and group replay.

The CLI cannot currently plant a target as a prescribed sum of factor-base
points. A future positive-control arm requires a versioned native
target-construction mode and a restricted planting record, followed by
independent replay. It needs its own frozen protocol and denominator; planted
cells do not replace any of these 26 natural cells.
