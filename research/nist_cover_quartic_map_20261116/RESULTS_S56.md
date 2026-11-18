# Results, amendment S56: the 56-bit ladder prime

Amendment `PROTOCOL_S56.md` (sha256 `c7a3ef99a143184012ab9466d27a0accb3967b55f9f02583be7a9c2871827737`,
`frozen_s56.sha256`), frozen 2026-11-16T18:47:20Z, before the first 56-bit run.
Data `runs_s56.json` (`data_s56.sha256`), log `run_s56.log` (exit 0), binary and
source hashes `binary_s56.sha256`. Figure and PDF: `reports/s56/`.

Every number is `[pcc:unverified]`: one producer, one host, no independent
replay, no pcc claim filed. Exact-correctness construction; no timing or cost
claim; no leaderboard, `benchmarks/README.md` row or bench record.

## Target

`prime_fast::find_a3_curve(56, 3)`, the 56-bit rung of the lab's prime-field
ladder (NIST shape: prime field, `a = -3`, cofactor 1):

* `p = 2^56 - 5 = 72057594037927931`
* `E: y^2 = x^3 - 3x + 60`, `#E = n = 72057594202405003` (prime)
* `G = (2, 795920743654909)`

## Result: every check of the amendment holds

| check | result |
|---|---|
| `[k]G`, planted `k = 19837203754012257`, by two independent implementations (`Ecc`, u128 affine; `prime_fast`, Montgomery) | agree: `Q = (55260850564970305, 14269207575648636)` |
| `[n]G = O` | yes |
| first Smith-positive `gamma` | `gamma = 15` (14 earlier values: "not smith-positive") |
| novel cover `H_15` | `v^2 = u^7 + 42 u^5 + 60 u^4 + 630 u^3 + 3375 u` (`= u[(u^2+15)^3 - 3u^2(u^2+15) + 60u^3]`) |
| Smith's trigonal map (normal form) | `N = x^3 + n1 x + n0`, `D = x^2 + d1 x + d0` with `n1 = 43179290915374657`, `n0 = 67832436536749340`, `d1 = 37767896783818060`, `d0 = 0` |
| `phi^*(G) != 0` | yes |
| `[n] phi^*(G) = 0` (`n` prime, so `ord phi^*(G) = n`) | yes |
| `phi^*([k]G) = [k] phi^*(G)` | yes |
| `phi^*(G + Q) = phi^*(G) + phi^*(Q)` | yes |
| negative control `[k+1] phi^*(G) = phi^*(Q)` | no (as required) |
| quartic fit (40 sampled image points: coordinates independent, no conic, no cubic, exactly one quartic) | yes, 38 draws of points of `H` |
| held-out image points on `C` | **120 / 120** (50 rational, 70 over `F_{p^2}`) |
| negative control: `C` with one coefficient perturbed | 0 / 120 |
| smoothness certificate | **certified**: chart `z = 1`: `deg R_a = deg R_b = 12`, `deg gcd = 0`; chart `y = 1`: same; `(1:0:0)` nonsingular |
| wall time, whole chain | 1.97 s (shared host; not a result) |

The explicit same-field plane quartic `C` over `F_p`, `p = 2^56 - 5`, in
`mons_of_degree(4)` order (`x^4, x^3y, x^3z, x^2y^2, x^2yz, x^2z^2, xy^3,
xy^2z, xyz^2, xz^3, y^4, y^3z, y^2z^2, yz^3, z^4`):

```
[1, 44858746782548416, 0, 37572650059108326, 60844269658931211,
 56389912100593629, 6018521062645660, 29095700753429674, 61315470249296059, 0,
 71497452090043392, 29684774564308524, 41647834419698634, 49418112992754898,
 398084569319130]
```

A rerun from the rebuilt (format-only) binary gave a byte-identical record
apart from `wall_seconds`.

## What changed in the code, and the regression evidence

The Smith stack was bounded to `p < 2^31`. Widened to `p < 2^62`
(semantics-preserving below `2^32`; see the amendment): the modular product
in `smith_prop24`, `smith_trigonal`, `smith_canonical`; the `(p^d - 1)/2`
powers in `edf` and `kp_one_root` via the Frobenius decomposition
`(prod_{i<d} a^(p^i))^((p-1)/2)` when `p^d` overflows a u128; the two `p`
bounds; a `u64` core for `fit_quartic`. New: `nist_cover_quartic::wide`
(sampled canonical fit, held-out check, resultant smoothness certificate) and
the `wide` subcommand.

* Regression gate on the widened stack, before the new module: 31 passed,
  1 ignored (`smith_prop24`, `smith_trigonal`, `smith_canonical`,
  `nist_cover_quartic`; Smith's Example 1 positive control among them).
* With the new module: 34 passed, 1 ignored. The three new controls: the
  Fermat quartic certifies smooth at `p = 1009` and `p = 2^56 - 5`; a quartic
  with a node at `(0:0:1)` does not; `F(Ax)` evaluated at `x` equals `F`
  evaluated at `Ax` on 20 random points at `p = 2^56 - 5`.
* `rustfmt --check` clean; `cargo clippy --release --lib --bin
  nist_cover_quartic_map --tests -- -D warnings`: no errors (the one warning
  per target is the crate's pre-existing `E0602` unknown-lint name under this
  host's toolchain).

## Not run at 56 bits

* `Psi: J_H -> Pic^0(C)` and the planted-log recovery on `C`: the `Pic^0`
  arithmetic (`plane_quartic_pic`) is `u32`-based (`q < 2^31`). This is the
  remaining port; at the toy primes the full chain including `Psi` passed
  (`RESULTS.md`).
* `#C(F_p) = #H(F_p)`: point counting is `O(p)`; not possible at this size.
  The held-out test checks that the fitted `C` is the image of the canonical
  map, and the resultant certificate that `C` is smooth; neither is a zeta
  function check.

## Distance to rho (derived, not measured here)

Matched negation rho on `E` needs about `sqrt(pi n / 4) = 2.38e8` group
operations (the lab's 56-bit ladder measurement, 2.6e8 steps,
`RESEARCH_PRIME_FAST_ECDLP.md`, `[pcc:unverified]`). Index calculus on a
same-field genus-3 `C` is `O~(p)` = about `7.2e16` group operations before log
factors, about `3.0e8` times more, and each `Pic^0(C)` operation costs far more
than an elliptic addition. For a prime-field target this route is far above
rho; the S56 result is the explicit construction at the requested size, not an
improvement.
