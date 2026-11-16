# PROTOCOL (follow-up 2): c_D = 1.7 combined with the square-root chain

Frozen 2026-11-14 in the commit that adds this file, before any data of this follow-up. Not edited
afterwards. Every result is an unverified assertion `[pcc:unverified]`.

## Why

`../cd_sweep/` found c_D = 1.7 the robust c_D for DL (0.73–0.75 of c_D = 2.0 with additions free,
0.87–0.88 with additions at 1 M-eq, k = 18..22). The main study's H1 showed the square-root chain
changes only the relation stage's multiplication count. This follow-up measures the two together.

## Units

Published binaries (`../launch/image.json`): v2 `183810cf…` (frozen driver), v3 `de500bc1…` (chain).
Arms: **BASE** (v2, c_D = 2.0), **C170** (v2, 1.7), **B170** (v3, 1.7); k = 18..22 (q = 262111 ...
4194271), seeds 1–3 (frozen N4 v2 instances). 45 units, `units.tsv` from `make-units.sh`, run one at a
time, largest size first, the three arms rotated by seed.

## Measures

As `../cd_sweep/PROTOCOL.md`: cold M-eq with the frozen grid's per-size constants, failed attempts
charged; F_q `add` and mod-ell `ladd` priced at w_add = 0, 0.5, 1.0 M-eq; CPU seconds diagnostic only.

## Gates

* **A/A:** BASE and C170 reproduce every counted field of the sweep's C200 and C170 units.
* **Chain identity:** B170 differs from C170 only in `relations.mul` (and timing), with the same key.
* Every unit verifies `kG = Q` for D, DL1, DL.

## Decision rule (fixed now)

For IC arm Y in {DL, DL1, D}, per size, `r_s = M-eq(B170, s) / M-eq(BASE, s)`.

* **Robust improvement at a size:** `max_s r_s < 1` at w_add = 0 and at w_add = 1.0.
* **Headline:** DL robust at every k = 18..22.
* **Chain contribution:** `M-eq(B170) / M-eq(C170)` per size (reported).

## Predictions (before data)

DL B170 / BASE: 0.68–0.74 (w_add = 0), from the chain's 0.93–0.99 times c_D = 1.7's 0.73–0.75; at
w_add = 1.0 about 0.82–0.87. D still dearer than BASE (c_D = 1.7 alone: 1.09–1.13; the chain removes a few
percent). B170 / C170 for DL: 0.93–0.99.

## Execution

Local, session container (the monthly IsoLab budget is still exceeded; owner chose local runs for this
line). `run-local.sh` here, unchanged instruments.
