# Amendments (PROTOCOL.md, sha256 57c95f99…, is unchanged)

## A1 — 2026-11-14 19:50 UTC: two diagnostic arms, added after the smoke test of K, S, T at m=2, t=1,2
Disclosure: the smoke test (seed 1, m=2, t=1,2) gave K: 4,4; S: 4,5; T: 4,5 (raw PC degree). So the
Möbius-coordinate arms are already one degree above K at t=2. Two arms are added to isolate the cause,
with predictions recorded before they are run:
- Arm M: Kummer tower in y (polynomial doubling y_{j+1} = y_j², top y_{t−1}² = c) with the Möbius link
  x(y_0 − 1) = y_0 + 1 (same V as arm S). Prediction: D(M) = D(S) (the bilinear link, not the rational
  doubling, costs the degree).
- Arm C: T₂ as the circle u²+v²=1 with polynomial doubling (u,v) ↦ (u²−v², 2uv) and the stereographic
  link x(1 − u_0) = v_0 (same V as arm T). Prediction: D(C) = D(M) (over F_{p²} the circle tower is the
  split Kummer tower with the same bilinear link, and PC degree is invariant under field extension).
The frozen decision rule is unchanged; K, S, T are the arms it is applied to. M and C are diagnostics.
