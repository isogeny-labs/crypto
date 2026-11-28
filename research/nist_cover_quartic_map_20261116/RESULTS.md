# Results: a novel genus-3 cover of a NIST-shaped curve, its explicit same-field quartic, and the checked transfer map

Study `nist_cover_quartic_map_20261116`. Module
`src/cryptanalysis/nist_cover_quartic.rs`, binary
`src/bin/nist_cover_quartic_map.rs`. Protocol `PROTOCOL.md`
(sha256 `215befa26da331ebc76cdf7e55e29e62ff1c94c5e7bcd994d594822010cc061e`,
`frozen.sha256`). Run `20261116T1637Z` (`run.log`, exit 0). Data `runs.jsonl`
(`data.sha256`); binary and source hashes `binary.sha256`. Report and figures
in `reports/`.

Every statement here is `[pcc:unverified]`: one producer, one host, no
independent replay, no pcc claim filed. Toy primes. No leaderboard row, no
`benchmarks/README.md` row, no bench record — this is an exact-correctness
construction with no timing or cost measurement.

## The milestone

One explicit, same-field plane quartic `C` and a checked map from the Jacobian
of a **novel genus-3 hyperelliptic cover** `H` of a NIST-shaped elliptic curve
`E`, carrying a planted discrete logarithm end to end:

```
E(F_p)  --phi^*-->  J_H(F_p)  --Psi-->  Pic^0(C).
```

This connects the cover/transfer stage to the non-hyperelliptic index-calculus
stage. It is a different route from Tian's degree-27 `(3,3,3)` isogeny
construction: here `phi` is a degree-2 cover and `Psi` is Smith's `(2,2,2)`
trigonal transfer.

## The novel cover family

For `E: y^2 = x^3 + a x + b` and free `gamma != 0`,
`phi(u,v) = ((u^2+gamma)/u, v/u^2)` pulls `E` back to

```
H_gamma: v^2 = f(u) = u * [ (u^2+gamma)^3 + a u^2 (u^2+gamma) + b u^3 ],
```

`f` ascending `[0, gamma^3, 0, 3 gamma^2 + a gamma, b, 3 gamma + a, 0, 1]`
(monic degree 7 => genus-3 hyperelliptic, one infinity). The expansion and the
identity `f(u)/u^4 = x0^3 + a x0 + b = y0^2` (every pulled-back point on `H`)
were verified symbolically and in the unit tests. At `gamma = 1, a = -3`,
`f = u^7 + b u^4 + u` — the known degree-2 cover of the `a=-3` curve
(`research/p256_genus_dlp_20261115`). `gamma` free is the one-parameter family;
it is the search space for a Smith-positive member of a *fixed* `E`. `[pcc:unverified]`

Novelty scope: the family generalizes the repository's existing `gamma=1`
cover; it is not asserted novel against the full literature (the shared
research-KB was unavailable this session — no executable, no MCP).

## Sweep (run `20261116T1637Z`, grid `{1009, 2003, 4001, 10007}`, `a = -3`)

For each prime: the first `a=-3` nonsingular curve `E` of **prime** order
(NIST cofactor 1), the first `gamma` in `[1,800]` giving a Smith-positive cover
with a smooth quartic and `#C(F_p)=#H(F_p)`, then the full chain on a generator
`P` of `E` and target `Q=[k]P`, `k = r/3 + 1`.

| `p` | `b` | `#E` (prime `r`) | `gamma` | `#C(F_p)=#H(F_p)` | usable pts over `U` | `ord phi^*(P)` | `phi^*` hom | planted `k` | log on `C` |
|--:|--:|--:|--:|--:|--:|--:|:-:|--:|--:|
| 1009 | 4 | 967 | 16 | 918 | 916 | 967 `= r` | yes | 323 | 323 ✓ |
| 2003 | 1 | 2039 | 3 | 2034 | — | 2039 `= r` | yes | 680 | 680 ✓ |
| 4001 | 9 | 4079 | 5 | 4082 | — | 4079 `= r` | yes | 1360 | 1360 ✓ |
| 10007 | 7 | 10193 | 9 | 10218 | 10210 | 10193 `= r` | yes | 3398 | 3398 ✓ |

All four: `#E` prime (cofactor 1); `H_gamma` genus 3; the quartic `C` smooth;
`#C=#H` over `F_p` (so equal zeta at `k=1`); `phi^*` injective on `<P>`
(`ord phi^*(P) = r`); `phi^*` a homomorphism on the planted pair
(`phi^*([k]P) = [k] phi^*(P)` in `J_H`); `Psi(phi^*(P))` nonzero in `Pic^0(C)`;
and BSGS on `C` for `log_{Psi(phi^*(P))} Psi(phi^*(Q))` returns `k`.
`all_checks_ok = true` on 4/4 (`runs.jsonl`).

## Explicit artifact (`p = 1009`)

* `E: y^2 = x^3 - 3x + 4` over `F_1009`, `#E = 967` (prime).
* Novel cover `H_16: v^2 = u^7 + 45 u^5 + 4 u^4 + 720 u^3 + 60 u`
  (`f` ascending `[0, 60, 0, 720, 4, 45, 0, 1]`).
* Explicit same-field plane quartic `C` over `F_1009`, coefficients in
  `mons_of_degree(4)` order:
  `[1, 1007, 0, 492, 875, 624, 339, 134, 871, 0, 34, 942, 452, 639, 311]`.
* `P = (0, 2)` (order 967); `Q = (601, 861) = [323] P`.
* `phi^*(P)` has order 967; `phi^*([323]P) = [323] phi^*(P)`;
  `Psi(phi^*(Q)) = [323] Psi(phi^*(P))`; BSGS on `C` returns `323`.

The explicit cover `f` and quartic `C` for every instance are the
`cover_f_ascending` and `quartic_C_mons_deg4` fields of `runs.jsonl`.

## Unit tests (`cargo test --release --lib cryptanalysis::nist_cover_quartic`)

4/4 passed (0.25 s):

* `cover_matches_known_p256_form_at_gamma1_am3` — `gamma=1,a=-3` gives
  `u^7 + b u^4 + u`;
* `cover_is_a_genus3_cover_pullback_points_lie_on_h` — monic degree 7, and
  `w^2 = f(u)` across the split fibre (> 20 checks);
* `full_chain_transfers_a_planted_dlp` — the end-to-end chain and BSGS recovery;
* `negative_control_wrong_scalar_fails` — a wrong scalar is not accepted.

## Distance to rho (honest accounting; not a gate)

The matched reference is negation-map rho on `E` over the order-`r` subgroup,
`O~(sqrt r)` group operations. Index calculus on a same-field genus-3 `C` is
`O~(p) = O~(r)` group operations (Diem), i.e. **above** rho for a prime-field
target. This study makes **no** speed or complexity-improvement claim; the gap
to rho is the distance remaining. The deliverable is the verified explicit
artifact and the checked map — which the `(3,3,3)` route does not supply on
this family. `[pcc:unverified]`

## What is established, and the gaps

Established by exact computation on each instance: the novel cover, its explicit
same-field quartic, the explicit pullback homomorphism `phi^*`, and the
end-to-end transfer of a planted logarithm `E -> J_H -> Pic^0(C)` recovered on
`C`.

Gaps (recorded, not done):

* Nothing at the real NIST prime sizes: `smith_*`/`smith_canonical` are
  `3 < p < 2^31`; a bignum port is needed and is out of scope here. The
  construction itself is field-agnostic (polynomial in `a, b, gamma`).
* Larger toy primes: `Transfer::usable_points` enumerates the ~`p` points of
  `H` over `U`, so instance build is `O(p)`. The prior Smith studies stopped at
  `p = 211` (`smith_canonical_20261115`) and `p = 8209`
  (`smith_transfer_cost_20261115`); `p = 10007` already exceeds both. A larger
  grid needs a faster usable-point pass.
* Smith's isogeny theorem (`R` induces `J_H -> J_X` with kernel `S`) is
  assumed, as in `smith_canonical_20261115`; only its consequences are tested.
* Correctness of the reused `smith_*` / `plane_quartic_pic` modules is taken
  from their own studies.
* No independent validator has replayed this run; no pcc claim filed. `[pcc:unverified]`

## Deviations from the protocol

* The grid was fixed to `{1009, 2003, 4001, 10007}` before the clean run
  (the protocol was updated to pin it and re-frozen, sha256 `215befa2…`). An
  earlier exploratory invocation included `40009, 100003`; those did not
  complete within the session budget (the `O(p)` usable-point pass) and are not
  part of the frozen run; no partial row from them is used.
* `usable pts over U` is recorded only for `p = 1009` and `p = 10007` in this
  table (the field is in every `runs.jsonl` row).

## Not produced

No benchmark record or leaderboard row (no timing/cost). Figures: the transfer
chain diagram (`reports/chain.svg`) and the `#C=#H` graph (`reports/counts.svg`,
`.png`); PDF `reports/report.pdf` (inspected, 2 pages).

## Amendment S56 (56-bit ladder prime)

The same construction at `p = 2^56 - 5` (`find_a3_curve(56, 3)`, prime-order
`E: y^2 = x^3 - 3x + 60`): Smith-positive at `gamma = 15`, `phi^*` checked on
the full prime-order group, explicit quartic `C` verified on 120/120 held-out
image points and certified smooth by resultants. `Psi` into `Pic^0(C)` not run
at that size (u32 `plane_quartic_pic`). See `RESULTS_S56.md` and
`reports/s56/report_s56.pdf`. `[pcc:unverified]`

## Amendment S56b (Psi into Pic^0(C) at 56 bits)

With a 64-bit copy of the `Pic^0` arithmetic, `Psi` runs at `p = 2^56 - 5`:
well defined, order `n` preserved, and `Psi(phi^*(Q)) = [k] Psi(phi^*(G))` for
the planted `k`; the wrong-`k` control fails. See `RESULTS_S56B.md` and
`reports/s56b/report_s56b.pdf`. `[pcc:unverified]`
