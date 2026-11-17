# ECCp-131 multiplicative-coset relation screen

Frozen before execution on 2026-10-22. This is a bounded test of a different
factor-base construction on the exact published curve and target, not a
full-rank experiment or a cost comparison to rho.

## Inputs and arms

Use the exact `CurveParams::eccp131` and `ic::params::named("eccp-131")`
constants from this commit. Verify both published points are on curve and
annihilated by the published prime subgroup order. Factor `p-1` independently
with PARI/GP. For each of the first 16 nonzero coset representatives, let
`H = mu_5` in `F_p`, lift `gH` to curve points, and retain both signs in
relations. Compare each base of at least two lifted abscissae to an equal-size
small-abscissa base. This is a screen of the only nontrivial small-order
multiplicative subgroup in `F_p`.

For each base: enumerate its complete signed pair-sum table; test a signed
three-point oracle by subtracting each signed base point from the candidate
and looking up the pair table. Scalar samples are `[a]G` for setup and
`Q+[a]G` for descent, with the published `Q` included first. Use seeds
`131` and `132`, 256 setup and 256 target samples per arm. Run eight planted
three-point witnesses per arm as positive controls. Recheck every hit as a
full-coordinate point equation. Rank is over the published order, with one
column per factor-base abscissa; only natural setup rows contribute to setup
rank. A target scalar may be declared only after a determined linear solve
and independent `[x]G=Q` replay.

## Measurement and stopping

Run each seed as a fresh process, with `/usr/bin/time -l` and captured stdout,
stderr, exit status, Python/GP versions, Git commit, and SHA-256 of the script.
Stop after both seeds and 16 cosets, or after 120 seconds or 1 GiB RSS for a
single process. Preserve failed attempts. Record base sizes, pair-table
distinct keys, planted controls, natural rows, independent rank, target rows,
point-check outcomes, and cold wall and peak memory. No rate or speed claim
is made from zero hits. Use GP to independently verify the parameter order,
Kummer divisibility, and a planted witness from the raw record.

The current three-point cardinality bound applies to every tested base:
at most `8 B^3` signed point triples, and at most `8 B^3/(n-1)` hit
probability for a uniform nonzero candidate. A null outcome narrows only
these representatives, seeds, and budgets. A valid natural row would trigger
an expanded, separately frozen rank/descent protocol.
