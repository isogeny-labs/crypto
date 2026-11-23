# Protocol addendum 2: the remaining bases (frozen before their data)

2026-11-11. Follow-up to PROTOCOL.md and PROTOCOL_isogeny.md. Same miniature (p = 1048573,
A = 926, B = 1), same class delta(P) = chi(x_M), x_M = x_W − A/3. Correction recorded here: for
y² = x(x² + Ax + 1) the class of the 2-torsion point (0,0) is B = 1 mod squares, not chi(A² − 4)
(the earlier script used the latter at x = 0, a point no base contains).

## Context found while preparing this (and why this is not new)

`research/curve25519_torsion_leads_20261103/README.md` already contains this mechanism as lever
T2 ("u(P) is a square ⟺ P ∈ 2E(F_p), so every base point is in an even coset"), implemented as
the `square-bits` base (x_W = w² + A/3), with 0 odd-coset points over 20 cells and a measured
engine verdict: coverage about 2× but F4 mul-adds per relation 1.28× to 1.71× higher, "T2 is
closed in its bit-encoded F4 form". The descent-class notes in `tracezero_toy_20261106` therefore
re-derived a known lever. This addendum only classifies the remaining bases.

## Bases and cells (exact enumeration)

- `square-bits`: lo = 0 with t ∈ {4, 5, 6}, plus the study's frames (lo = 984071, t = 4;
  lo = 414271, t = 5). Prediction (by construction, x_M = w²): every on-curve point has delta = +1.
- `edwards-bits`: x_M = (1 + y)/(1 − y), y ∈ [0, 2^t) ∖ {1}, t ∈ {4, 5, 6}. Prediction: mixed.
- `t0-bits`: lo = 0, w ∈ [0, 2^t), x_M the roots of z² − w·z + 1 = 0 in F_p (z and 1/z), t ∈ {4, 5, 6}.
  Prediction: mixed; z and 1/z share a class.
- `digits`: x_W = LO + Σ c_j b_j (b_j ∈ {0,1}) for the three digit sets used in the repo:
  `0:1;2;4;8;16` (= bits t = 5), `113971:289589;761384;273167;963514;602728`,
  `51134:293482;1015139;374525;914144;700453;505371`. Prediction: mixed.

## Decision rule

Per cell with at least 8 on-curve abscissae: **fixed class** if all have the same delta, **mixed** if
both classes are at least 10%. Cells with fewer than 8 on-curve abscissae are reported but do not
count.

Supplementary, unregistered: for `square-bits` t = 9, lo = 0, enumerate hits R + K = ±P_i ± P_j over
200 targets × 8 lifts and report hits by delta(R + K): the theory predicts zero hits on lifts with
delta = −1, so exactly half of the 8 lifts are provably dead for this base.

## Out of scope

No F4 cost and no claim about whether skipping dead lifts pays: the existing study found the
`square-bits` base itself net more expensive. I read the oracle only partially and did not find a
descent-class skip in it (it skips lifts whose abscissa was already tried); that is a reading, not
a verified absence.
