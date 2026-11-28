# `B > 2` as a first-class factor base in the harness

Date: 2026-11-27.

`prime-mixed-isogeny:degrees=3;3;5;11,seed=S[,max_curves=C]` is now a
`FactorBaseBuilder<PrimeCurve>` (`src/cryptanalysis/ic_framework/prime_tower.rs`,
registered in `src/bin/ic/bench.rs`). It runs
`pkm_smooth_tower::search_aux_exact` → `isogeny_tower_on` → `V = x(R' + ⟨Q⟩)`,
verifies the tower, and hands `V` to the same `lift_abscissae` every other
prime base uses. The auxiliary search and chain build are recorded as counters
(`aux_curves_tried`, `aux_order`, `tower_levels`, `chain_degree`) and are
**uncharged** in this cut, as its description says.

Same instance (`icv1-fp18-t175-45770ebc`, `p = 261847`, `r = 261673`,
`|V| ≈ p^{1/2}`), same oracle (`subtract`, the linear scan), same targets:

| base | degrees | \|F\| | cols | hit | trials | rank | S | vs ρ | verified |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|:--|
| `prime-isogeny:t=9` | 2⁹ | 524 | 262 | 0.406 | 377 | 153 | **289.0** | 213.9× | yes |
| `prime-mixed-isogeny` | 3;3;5;11 | 492 | 246 | 0.331 | 486 | 161 | 366.0 | 270.9× | yes |

Rows: `harness_b2.json`, `harness_b11.json` (both `status: complete`,
`all_verified: true`).

## What this says, and it is not what I expected

1. **The mixed chain runs in the harness with the full ledger** — GAE `S`,
   matched-rho ratio, base-column checks, `verified`. That was the goal, and it
   is done.

2. **The smoothness bound does not help the linear scan.** With the same oracle
   the `3·3·5·11` base is 1.27× *worse* (366 vs 289 GAE), not better: the scan
   costs `≈ |F|` group operations per target, both bases are the same size, and
   the mixed chain's hit rate was slightly lower, so it needed 29 % more
   targets. The `B` lever only shows up in the *algebraic* solver, which is
   ~10⁵ GAE behind the scan anyway (`prime-isogeny:t=9` + `pkm-semaev` at this
   instance: `S = 32 512 674`, 24 066 604× rho).

3. **So for the algorithm as a whole, the map family is nearly irrelevant.**
   The scan's cost is `|F|` per target × `p^{1/m}` targets = `p` per DLP, and
   neither the chain degree nor the algebraic engine changes that. This is the
   same `p` bound as the dense argument in `SUBEXP_STATUS.md`, now measured in
   the harness's own currency on a base built by the `B > 2` construction.

## Caveats

* The auxiliary search and chain build are uncharged here; the mixed base's
  `S` is therefore a floor by whatever that search costs (124 curves for this
  `S`, and a point count per curve).
* `subtract` is the linear scan, not an algebraic oracle. The row measures the
  base and the scan, not `pkm-semaev` on a mixed chain — that still needs the
  oracle's isogeny membership generalised past a linear `ψ`, which is the next
  piece of work.
* One instance, one seed, one curve.

## Reproducibility

Both rows were re-run after rebuilding the binary from the same patch: `S =
289.0` (213.9× ρ) and `S = 366.0` (270.9× ρ), identical to the committed rows,
`verified: true` in both cases.

The branch commit applies this patch to the branch's newer `bench.rs` and
`prime_tower.rs`. That combination is **not buildable in the build checkout**
used here — the branch's `bench.rs` needs a newer library API than the checkout
has (`E0061: this function takes 5 arguments but 4 were supplied`) — so the
numbers above were produced by the checkout's own files plus the same patch,
and the branch-tip build needs CI or a full checkout to confirm. The patch
itself is `+84/−1` on those two files; an earlier commit that staged the
checkout's older copies over the branch's newer ones was repaired in
`946b17734f4`.
