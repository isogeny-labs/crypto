# Kernel-construction development checks

The initial and attempt-2 source hashes are distinct and retained. Both native
standalone core commands pass13tests. Attempt-2 adds an abstract4232cofactor
construction case; the updated small binary/reference adapter is implemented
but its Cargo gate has not yet run on this source. A separate direct native
rustc link against the verified974f2e9cd46b library passes14tests, including
the independent binary/reference fixture; see the sibling
projected-kernel-builder-binary-development-20261027 receipt. These are development
correctness tests on public software fixtures, not research scaling/timing
measurements or a certificate for the actual E0 curve.

`certify_from_points` validates supplied input, projects by[r], enumerates
successive cosets in input order, fails on a proper span and reuses the full
kernel verifier. Correct backend arithmetic and certified ambient order remain
assumptions. No candidate discovery, random sampler, relation oracle, log-LA,
full cold accounting or general formal counting proof is provided.
