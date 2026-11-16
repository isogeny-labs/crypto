# Projected signed-orbit map placed one factor-base orbit at a time (exploration, dropped)

- **Status:** an exploration on this session's container, 2026-11-15, three
  isolated rounds an arm. This is a record, not rounds, and supports no
  claim. The change is reverted from code.
- **Why:** `select_projection` costs 4–7 ms at k0n53 and 6–10 ms at
  k0n61. Its single-word map hashes every point twice, into a seen set
  and a location map, and then looks each point up.
- **The change tried:** walk each factor-base orbit's projection once,
  take its least signed member as the canonical point, and place the
  orbit's points by Frobenius phase.
  - The walk, full-length orbits, distinct signed members and full
    placement were checked. Any failure fell back to the hashed route.
  - The existing projection tests passed, including against the general
    map, and a new assertion showed the route answering on subgroup
    bases.
  - `KIC_ORBIT_MAP=0` kept the hashed route.
- **Result** ([`explore/`](explore/)), `select_projection` in ms,
  repetitions 1–3:

  | row | hashed | by orbit |
  |:--|:--|:--|
  | k0n53 | 5.0–6.7 / 4.3–4.9 / 3.8–3.9 | 4.5–7.0 / 3.8–4.0 / 3.4–3.7 |
  | k0n61 | 8.2–10.6 / 7.0–7.4 / 6.6–7.2 | 7.2–9.2 / 6.2–6.7 / 6.0–7.3 |

  - Certificates and counts were identical in all 12 processes.
- **Reading:**
  - About 0.4 ms at k0n53, about 0.1% of set-up, so the hash passes were
    not where the phase's time goes.
  - The first repetition is always the slowest, which points to one-time
    warm-up in the phase.
  - Not worth the added code, so no A/B was run.
