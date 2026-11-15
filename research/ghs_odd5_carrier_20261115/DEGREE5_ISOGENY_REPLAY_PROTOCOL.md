# Independent direct-linear replay of alternate trace +26 targets

Frozen before replay execution on 15 November 2026. The source
`degree5_isogeny_linear_replay.sage` adapts the independently structured
`degree5_linear_replay.sage` from `DEGREE5_REPLAY_PROTOCOL.md`. It solves
quotient-ring coefficient systems directly for each b-mask
`27,30,103,139,167,182,194,230,233`; it does not read producer receipts or
use the producer CRT construction. It recomputes 2-torsion and full elliptic
equation for each exact target. The six denominator shapes and 14,446,377
logical assignment counts are identical because Y and K are fixed.

Run each mask via the frozen Sage-preparsed Python file in a separate process;
the committed `.sage` SHA-256 is `ba1506eb959bd1a93f8947d1e779b581d6b4aa667ddaceb89ecfd899acfbb525`
and the Sage-preparsed `.sage.py` SHA-256 is
`c47f9af2b8557eccedab5ef9625b39eef622158b132c8c4b7b0f287e03f6a3af`.
retain complete JSON/stderr and a manifest with source and generated hashes,
exit status and output hash. Require all nine source hashes, exit-zero statuses,
empty stderr, expected counts, exact `(A,B)` survivor-set equality with the
producer (ignore ordering and branch-root labels), and agreement on zero
full-equation hits. Only then use these runs toward a class-wide degree-five
exclusion. A negative result covers these ten exact K-isomorphism classes and
the one displayed genus-five Y, not all carriers or all fields.
