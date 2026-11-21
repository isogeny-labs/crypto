# AVX-512 IFMA kmitm lockstep (PLAN lever 2 vector) — 2026-11-21

`[pcc:unverified]` shared VM. Engineering wall only; charged `S` unchanged.

## What landed

- `rootfind_kmitm_ifma::IfmaRight1` — 8-wide IFMA Montgomery walk (`R = 2^52`)
  over the factor base for `right = 1` probes.
- Prepare builds a canonical affine-x Bloom + witness map beside the existing
  Montgomery maps; `decompose_lanes` prefers IFMA when prepared
  (`set_ifma_enabled(false)` keeps scalar 8-wide for A/B).
- Unit test `kmitm_ifma_lanes_recover_planted_m4` (skips cleanly without IFMA).

## Oracle wall (ns/target, miss-dominated)

| cell | m | scalar8 | IFMA8 | IFMA / scalar |
|---|---:|---:|---:|---:|
| bits=24 \|Fₓ\|=14, 64 tgt | 3 | 548 | 478 | **1.15×** |
| | 4 | 677 | 492 | **1.38×** |
| | 5 | 569 | 516 | **1.10×** |
| | 6 | 684 | 555 | **1.23×** |
| | 8 | 1009 | 1581 | 0.64× |
| bits=32 \|Fₓ\|=32, 128 tgt | 3 | 1277 | 871 | **1.47×** |
| | 4 | 1227 | 892 | **1.38×** |
| | 5 | 1311 | 1275 | **1.03×** |
| | 6 | 3142 | 4879 | 0.64× |
| bits=32 \|Fₓ\|=64, 128 tgt | 3 | 2271 | 1740 | **1.30×** |
| | 4 | 2231 | 2140 | **1.04×** |
| | 5 | 3055 | 3277 | 0.93× |
| | 6 | 3677 | 6041 | 0.61× |

Raw: `kmitm_ifma_wall_b24.txt`, `kmitm_ifma_wall_b32_a32.txt`,
`kmitm_ifma_wall_b32_a64.txt`.

## Reading

- **Win band (after batch Mont-inv of the IFMA prefix, 2026-11-21 tick1):**
  m=3–4 at A≤64, and m=5 at modest A — oracle wall IFMA vs scalar8 ≈**1.5–1.8×**
  (bits32 A32); see `research/rootfind_optlevel_20261121/`.
- **Loss band:** very large left tables (m=6 at A=64, left ≳10⁸) — Bloom /
  map / cache dominate; auto-disable when left > ~500k entries.
- Default `decompose_lanes` prefers IFMA when prepared and left is not huge.
  Callers can `set_ifma_enabled(false)`.
- Does **not** move e2e single-DLP cold wall (lanes collect already lost to
  walk at the pinned recipe). Does **not** pass G3.

Optlevel (T-timing, honest IFMA lane-mul count): kmitm:m=4 IFMA A_single ≈**0.97**
(grade A) on this host — near the compute roof for counted mont muls.

## Claim hygiene

Constant-factor engineering on the oracle probe path. No security / sub-rho /
asymptotic claim. Still `[pcc:unverified]`.
