# PROTOCOL (follow-up): c_D = 1.7 and 1.8 against 1.55 and the frozen 2.0, with additions priced

Frozen 2026-11-13 in the commit that adds this file, before any data of this follow-up. Not edited
afterwards. Every result is an unverified assertion `[pcc:unverified]`.

## Why

The main study (`../PROTOCOL.md`, `../README.md`) found c_D = 1.55 lowers the frozen cold M-eq of DL to
0.58–0.68× at k = 18..22, but post hoc: near the percolation threshold the cycle rows are about 3×
heavier, the extra linear-algebra work is mostly additions, which the frozen M-eq model prices at zero,
and with additions at 1 M-eq the gain vanished except at k = 22. A c_D further from the threshold
should keep more of the relation-stage saving with lighter rows. This follow-up measures c_D = 1.7 and 1.8.

## Units

Frozen v2 driver only (no square-root chain: its effect is separate, shown by the main study's H1).
Binary: the published image's v2 (`../launch/image.json`, sha256 `183810cf…`). Arms by c_D:
C200 (2.0, frozen), C155 (1.55), C170 (1.7), C180 (1.8); k = 18..22 (q = 262111 ... 4194271);
seeds 1..3 (frozen N4 v2 instances). 60 units, `units.tsv` from `make-units.sh`, run **one at a time**
in a fixed interleaved order (for each size and seed, the four arms in a rotation that changes with the
seed), so the CPU diagnostic is not confounded by co-running units or by order.

## Measures

* **Primary (as the main study):** cold M-eq with the frozen grid's per-size constants, failed attempts
  charged, band w = 0, 0.1, 1 for hash/cmp.
* **Robustness (pre-registered here):** the same with F_q `add` and mod-ell `ladd` priced at
  w_add = 0.5 and w_add = 1.0 M-eq each.
* **Diagnostic:** CPU seconds per arm and phase (local container, L0; reported, not gated).

## Gates

* **A/A:** C200 and C155 reproduce every counted field of the main study's BASE and CRULE units for the
  same (q, seed) (`../runs/local-20261113/`).
* Every unit verifies `kG = Q` for D, DL1, DL.

## Decision rule (fixed now)

For c in {1.55, 1.7, 1.8} and IC arm Y in {DL, DL1, D}, per size, `r_s = M-eq(c, s) / M-eq(2.0, s)`.

* **Robust improvement at a size:** `max_s r_s < 1` at w_add = 0 **and** w_add = 1.0 (hash band w = 0).
* **Headline:** for DL, the c values with a robust improvement at every k = 18..22.
* **Best c per measure:** for each of w_add = 0, 0.5, 1.0 and each IC arm, the c with the lowest
  geometric-mean ratio at k = 22 and on average over k = 18..22 (reported; no single winner is forced).

## Predictions (before data; pilot 2 of the main study, frozen measure only)

DL at c = 1.7: about 0.76 / 0.74 / 0.73 (k = 18 / 20 / 22); at 1.8: about 0.82 / 0.82 / 0.81 (w_add = 0).
With w_add = 1.0 we expect 1.7 and 1.8 to stay below 1 at k = 20..22 (rows lighter than at 1.55: pilot 1
mean cycle-row weight 34–39 at c = 1.75 against 58–65 at c = 1.6, k = 16..20); k = 18 is uncertain.
D (plain Lanczos) is expected still dearer at 1.7 and possibly near 1 at 1.8.

## Execution

Local, on the session container, as the main study (the monthly IsoLab budget is still exceeded; the
owner chose local runs for this line on 2026-11-13). Unchanged instruments; `run-local.sh` here.
