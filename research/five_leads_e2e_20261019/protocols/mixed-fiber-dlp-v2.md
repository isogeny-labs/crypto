# Mixed-fiber DLP v2: cold selection accounting correction

The first v1 execution recovered all 540 source scalars, but reset the field
counters inside the solver after coset/fiber selection. Its relation-search
costs are retained as `correctness/mixed-fiber-dlp-v1-unsealed.jsonl` and are
not used for complete-cost ranking. All v1 mathematical checks remain useful.

V2 keeps the source-curve selection rule, nine selected curves, 20 exact-order
targets per curve, three arms, target seeds, rho reference, relation algorithm
and cap of v1. On **every** target and arm, it now creates a fresh counted
curve, discovers the primitive root, enumerates and scores cosets, chooses
the homogeneous/mixed fibers, performs any random matching, and then builds
the relation table. The field counter is never reset between these steps.
The exact selection M/S/I vector is preserved separately in each record;
complete candidate cost includes it. The independently checked source rho
receives the same public curve and target fixture.

The end-to-end evidence criterion is unchanged: all 540 candidate scalars
must be source checked; failures and losses are retained. This implementation
still uses a pair table as its complete relation oracle. It makes no claim
about the proposed polynomial F4/msolve route.
