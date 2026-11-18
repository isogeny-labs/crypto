# Results, amendment S56b: the transfer `Psi` into `Pic^0(C)` at the 56-bit ladder prime

Amendment `PROTOCOL_S56B.md` (sha256 `da174929f1cb9be1038c2d8b2241a90e4063f4b9fa3a3d9f3a58a36dcdc39a3a`,
`frozen_s56b.sha256`), frozen 2026-11-17T06:21:48Z before any 56-bit `Psi`
data. Data `runs_s56b.json` (`data_s56b.sha256`), log `run_s56b.log` (exit 0),
binary and source hashes `binary_s56b.sha256`; aborted first attempt
`run_s56b_aborted_proots.log`. Figure and PDF: `reports/s56b/`.

Every number is `[pcc:unverified]`: one producer, one host, no independent
replay, no pcc claim filed. Exact-correctness construction; no timing or cost
claim; no leaderboard, `benchmarks/README.md` row or bench record.

## Result: the planted logarithm is carried onto `C` at 56 bits

Instance (unchanged from S56, every S56 field of the record identical):
`p = 2^56 - 5`, `E: y^2 = x^3 - 3x + 60`, `n = 72057594202405003` (prime),
`G = (2, 795920743654909)`, planted `k = 19837203754012257`, cover `H_15`, the
explicit quartic `C` of `RESULTS_S56.md`.

| check (amendment S56b) | result |
|---|---|
| `Psi(phi^*(G))` from two independent random representatives | equal (well defined) |
| `Psi(phi^*(G)) != 0` | yes |
| `[n] Psi(phi^*(G)) = 0` (`n` prime, so the order is exactly `n`) | yes |
| `Psi(phi^*(Q)) = [k] Psi(phi^*(G))` | **yes**: the planted logarithm is carried onto `C` |
| `Psi(phi^*(G + Q)) = Psi(phi^*(G)) + Psi(phi^*(Q))` | yes |
| negative control `[k+1] Psi(phi^*(G)) = Psi(phi^*(Q))` | no (as required) |
| pool of sampled points of `H` | 48 |
| `Psi` stage / whole chain (shared host; not a result) | 0.04 s / 2.18 s |

The canonical keys of `Psi(phi^*(G))` and `Psi(phi^*(Q))` (classes of
`Pic^0(C)` in the 64-bit representation, base point the first rational
held-out image point) are the `psi_G_key` and `psi_Q_key` fields of
`runs_s56b.json`. The full chain `E(F_p) -> J_H(F_p) -> Pic^0(C)` now runs at
56 bits as it did at the toy primes; recovering `k` from the classes on `C` by
search was not attempted (a generic `~2^28`-operation search adds nothing to
the identity checked above).

## Code

* `plane_quartic_pic64`: a 64-bit copy of `plane_quartic_pic` (`q < 2^62`):
  `u64` field elements, `u128` products in `Fq` and the `poly_roots` helpers,
  the two long accumulators reduced at every step, deterministic
  Miller-Rabin. Algorithms unchanged; the `u32` original and its callers are
  untouched.
* `wide::Transfer64`: `Psi` ported from `smith_canonical::Transfer`, with base
  points from sampled rational image points of `C`, a sampled pool of points
  of `H`, and split representatives accepted when their `e_class` is defined.
* `wide::split64`: the points of a split class of `J_H` by factoring `u`
  (polylog), replacing `HJac::split` on this path (see the deviation).

Regression gate, all before the result: 21 passed, 1 ignored
(`plane_quartic_pic64`, `nist_cover_quartic`, `smith_canonical`), including:

* `plane_quartic_pic64::matches_u32_original_on_random_quartics`: identical
  canonical keys to the `u32` original for point classes, sums, differences,
  negation and scalar multiples (`k` up to `~2^32`) at `q = 1009, 65521,
  2^31 - 1`;
* `transfer64_matches_u32_transfer_at_toy_prime`: the 64-bit transfer gives
  the same class of `Pic^0(C)` as `smith_canonical::Transfer` at `p = 1009`
  (same trigonal map, canonical map, base point), and `Psi([77]D) = [77]Psi(D)`;
* `split64_matches_hjac_split_at_toy_prime`: agreement with `HJac::split` on
  400 random classes at `p = 1009`;
* `primality_matches_trial_division_below_2_31`.

## Deviation

The first run of the frozen command (06:21:48Z) did not finish: `Psi` used
`HJac::split`, whose root finder `smith_canonical::proots` scans `0..p`, which
does not terminate at `p = 2^56 - 5`. It was stopped after about 10 minutes
of CPU, before any `Psi` output; its log is kept as
`run_s56b_aborted_proots.log` (the S56 portion it printed is identical to the
completed run). The instrument was fixed (`split64`, tested against
`HJac::split`), and the frozen command was run unchanged; parameters, seed
and checks were not altered. `proots` in the shared module was left as is
(its root order feeds the operation counts of the frozen
`smith_transfer_cost` study).

After the result, clippy 1.98 (`-D warnings`) required removing same-type
casts left by the `u32 -> u64` rename in `plane_quartic_pic64` (55 sites, no
semantic change). The regression tests (21 passed) and the frozen command were
run again on that final source: the record is identical apart from the timing
fields, and `binary_s56b.sha256` / `data_s56b.sha256` refer to this final
build and record.

## Distance to rho (derived, unchanged from S56)

Matched negation rho on `E` needs about `2.4e8` group operations; index
calculus on a same-field genus-3 `C` is `O~(p)`, about `7.2e16`, roughly
`3e8` times more. The S56b result completes the construction at the
requested size; it is not an improvement. `[pcc:unverified]`
