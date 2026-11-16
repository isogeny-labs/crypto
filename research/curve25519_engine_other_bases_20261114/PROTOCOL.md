# Protocol: the remaining Curve25519-miniature bases in the F4 engine (frozen before any inference data)

2026-11-14. Follows `research/curve25519_descent_skip_20261114` (PR #4111), whose engine runs covered only `bits` and
`square-bits`. Same instrument and conditions: `curve25519_family f4cost`, k = 20 miniature (p = 1048573, A = 926),
G = 3, m = 2, mode full, counted F4 multiply-adds (deterministic). Binary: the release build of commit `1edbb0262c`
(sha256 `ed99cb1653aaa35c9a87c0aa0817a63de0687dc498600e5694b3f7bc443f99bd`; its opt-in `descent_skip` flag stays off).
All results [pcc:unverified]. No rho arm, so no cross-method claim and no distance to the matched rho.

## Cells

Five bases, each at `lifts=0` (blind) and `lifts=1`:
1. `isogeny` t = 5, seed 0
2. `edwards-bits` t = 5
3. `t0-bits` t = 5 (lo = 0)
4. `digits@113971:289589;761384;273167;963514;602728` (t = 5)
5. `digits@51134:293482;1015139;374525;914144;700453;505371` (t = 6)

References (not rerun; PR #4111 Part C2, same binary and conditions, 20,000 targets, seed 2026111403): `bits@0` t = 5 and
`square-bits@414271` t = 5, `lifts=0` and `lifts=1`.

## Sizing (calibration only, not used for inference)

A pilot of 100 targets per arm (seed 2026111404) measures F4 mul-adds per target. The inference run then uses
N = min(20000, floor(4e11 / mul-adds-per-target-of-the-pilot-arm)) targets, seed 2026111403 (the references' seed), with no
time cap that could truncate an arm. A cell that fails to run in the pilot (an argument error or an unsupported spec) is
reported as not run and is not fixed by changing the base. If a blind arm's N would exceed 20,000, it is capped there.

## Hypotheses (directional, fixed now; informed by PR #4111)

- H4 (structure): on each base `lifts=1` multiplies the F4 cost per target by 8 within 0.5% (a full 8-element fiber);
  fewer would mean the oracle already prunes lifts on that base.
- H5 (yield per mul-add): on each base, hits per 10^9 mul-adds with `lifts=1` are below the blind arm's. Per base, tested by
  the exact conditional binomial test of PR #4111; "supported" only where two-sided p < 0.05 for the base alone,
  otherwise "unresolved". Five tests, no correction beyond reporting the count.

Descriptive, no prediction: hits per 10^9 mul-adds with exact Poisson 95% intervals for every arm, and where each base falls
against the two references.

## Out of scope

Other k, t, G, m; timing; rho; any claim about asymptotics. Deviations as in PR #4111: no Conductor task, no isolab-eks or
S3 resume checkpoint (absent here), counted mul-adds run locally, no research-kb query, no figure or PDF.
