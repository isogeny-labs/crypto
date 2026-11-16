# L05 implementation notes (choices where SPEC.md is silent)

- `l05/ctx.py` builds per-run context (curve, setup Counter, even tables, `s`, odd map, optional odd tables). Setup counts are reported as `setup_mul` / `setup_inv` and excluded from arm FPM.
- `random_divisor` uses the odd tables' `hx` and `sq` (same values as SPEC 6.9).
- Cantor: uncounted pure-Python polynomial arithmetic in `l05/poly.py`; `smul(D, 0)` returns the identity.
- `order_of_delta` trial-divides `#J` and uses Cantor `smul`.
- F_{p^3} norm is the determinant of the 3x3 multiplication matrix (explicit formula); F_{p^2} norm is computed as `a^2 - ns b^2` (equal to the determinant of the 2x2 matrix).
- V2 sampling for root relation-phase runs uses `mix64(i ^ 0x5632) % 1000 < min(1000, ceil(200000 / N_target))` with `i` the index of accepted relations; rootless even arms get no V2; oddrand and rootless O arms use the same sampling on their own model (`v2odd` mode).
- Relation `a` polynomial is stored trimmed; V1 uses `len(N)-1 == len(pts)`.
- Mutation m2 replaces the last element of the sorted residual.
- A failed verification raises `GateFailure` carrying the relation; `run.py` writes the failure record and exits 2 (none occurred).
- `analyze.py` reads `PROTOCOL.md` for the limits paragraph copied into `summary.md`.
