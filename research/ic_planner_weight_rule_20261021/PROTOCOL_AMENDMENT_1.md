# Amendment 1: Phase B arms, fixed from Phase A

Frozen 2026-10-21, after Phase A (commit 1800a85b2) and before any Phase B
session. It fixes only what PROTOCOL.md left to be read from Phase A.

Phase A's rule (`analysis/derivation.json`) gives:

| w | 0 | 0.03 | 0.1 | 0.3 | 1 |
|:--|--:|--:|--:|--:|--:|
| m*(w) | 1 | 1 | 1 | 1 | 1.41 |

So `m*(w) ≠ 1` only at `w = 1`, and Phase B has one `v2` arm per curve,
`v2-m141`, sized `round(1.41 · N₀)`:

| tag | slug | `N₀` (`v1`) | `v2-m141` |
|:--|:--|--:|--:|
| h21 | `icv1-fp21-t377-d7bba3fb` | 58 | 82 |
| h22 | `icv1-fp22-tm2947-e15fed69` | 67 | 94 |
| h24 | `icv1-fp24-t2705-27cbb01b` | 111 | 157 |
| h29 | `icv1-fp29-t47-14392892` | 342 | 482 |

Arms per curve: `rho-neg` (reference), `v1` (candidate, the `ic-planner/1`
plan), `v2-m141` (baseline); every other stage as the plan. Specs
`specs/B-<tag>.json` from `scripts/make_specs.sh B`. Design as PROTOCOL.md
Phase B: 8 targets (target seed 20261025), 3 rounds + 1 warm-up,
alternate, measurement seed 20261026, L2, 1,800 s.

Tested predictions, unchanged: P3 at `w = 1` only (the one weight with
`m*(w) ≠ 1`): `S_1(v2-m141) / S_1(v1)` below 1 with its paired cluster
interval below 1 on at least 3 of the 4 curves (`ecbench compare --weight 1`).
P4 at `w = 0`: `S_0(v2-m141) / S_0(v1) ≥ 1`. The band at the five weights is
reported too.

`ic-planner/2` is implemented only after Phase B, and only if P3 holds.
