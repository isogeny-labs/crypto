# Amendment 1: Phase B arms, fixed from Phase A

Frozen 2026-10-26, after Phase A (sessions committed in b3f3bd6dd) and before
any Phase B session. It fixes only what PROTOCOL.md left to be read from
Phase A. The values come from `analysis/derivation.json` (`scripts/derive.jq`
over `ecbench band` at the five weights):

| w | 0 | 0.03 | 0.1 | 0.3 | 1 |
|:--|--:|--:|--:|--:|--:|
| binary `m*(w)` | 1 | 1 | 1 | 1 | 1 |
| Koblitz `m*(w)` | 1 | 1 | 1 | **0.71** | **0.71** |

- **Binary:** `m*(w) = 1` at every weight, so binary has no Phase B and the
  planner keeps its binary rule. At w = 1 the closest alternative is `d₀ + 1`,
  with G = 1.061.
- **Koblitz:** the one distinct `m*(w) ≠ 1` is 0.71. It holds at w = 0.3
  (G = 0.928) and at w = 1 (G = 0.847; m = 0.5 reads 0.848). The base
  **shrinks** with the weight, so P1 (non-decreasing) fails for Koblitz.

Phase B arms (`specs/B-<tag>.json` from `scripts/make_specs.sh B "" 0.71`):
the reference `rho.signed_frobenius`, `plan` (c₀ columns), and
`c<max(1, round(0.71·c₀))>`:

| tag | slug | c₀ (plan) | alternative |
|:--|:--|--:|--:|
| hk19 | `icv1-f2m19-tm797-9c54981b` | 2 | c1 |
| hk23 | `icv1-f2m23-tm5197-1f85e9e1` | 4 | c3 |
| hk39 | `icv1-f2m39-t1481485-829ef1d1` | 6 | c4 |

Design as PROTOCOL.md Phase B: target seed 20261029, measurement seed 20261030.

P3 is tested at w = 0.3 and w = 1, the weights with `m*(w) ≠ 1`, using
`ecbench compare --weight`. It needs `S_w(alt)/S_w(plan) < 1` with the paired
interval below 1 on 2 of the 3 curves, at both weights. The band at the five
weights is reported too.
