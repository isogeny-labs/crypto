# The remaining Curve25519-miniature bases in the F4 engine: lifts rarely pay, and the isogeny base yields the most per mul-add

2026-11-14. Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen in `354910a36f` before any data (a 100-target calibration
pilot only sized the runs). Follows [`research/curve25519_descent_skip_20261114`](../curve25519_descent_skip_20261114/README.md),
whose `bits` and `square-bits` arms are the references here (same binary, seed 2026111403, G = 3, t = 5).
Everything is [pcc:unverified]; no claim, replay certificate, validator report or matched rho. **No rho arm was measured, so
there is no cross-method comparison and no distance to the matched rho.** The unit is counted F4 multiply-adds, not
seconds. No benchmark record, leaderboard row, figure or PDF (matplotlib is not installed here).

## Answer

1. **Lifts cost 8× per target on every base here** (H4: 7.98×, 7.99×, 8.00×, 7.98× on `isogeny`, `edwards-bits`, `t0-bits`,
   digit set 1; digit set 2 reads 7.94×, 0.7% under 8, which misses the frozen 0.5% tolerance; the cause is early exits after
   hits, 8 hits in 294 targets). Each base has the full 8-element fiber, so there is no class pruning to exploit, as expected
   for bases that contain both descent classes.
2. **Lifts lower the hits per counted mul-add on most bases, and not on `isogeny`** (H5, five tests, exact two-sided
   conditional binomial): `t0-bits` 0.25 (p = 0.0125), digit set 1 0.25 (p < 0.0001), digit set 2 0.38 (p = 0.016): supported;
   `edwards-bits` 0.79 (p = 0.45): unresolved; `isogeny` 1.03 (p = 1.0): **not supported, no difference**. With the references
   the picture is: ratio below 1 on 7 of 8 bases in point estimate, resolved at p < 0.05 on 4 of them (`t0-bits`, both digit sets,
   `square-bits@414271`).
3. **The isogeny base yields the most hits per counted F4 mul-add of any base tested**: 3.4 to 3.5, which is 2.8× to 2.9× the next
   best arm (digit set 1, blind, 1.205), 13× to 14× the best `square-bits` arm (0.253) and 34× to 35× the `bits` arms (0.099 blind).
   The reason is that its F4 system is 17× cheaper per target (1.77e5 against 3.04e6) while its hit rate per target is about twice
   `bits`' (12 against 6 hits in 20,000 targets). This counts F4 mul-adds only; see the caveats below.
4. **The coverage-selected digit frame is the best non-isogeny base blind** (1.205 [0.945, 1.515] against `bits` 0.099): 73 hits in
   20,000 targets against 6 at identical F4 cost per target. Lifts then raise hits per target by only 2.0× (3.65e-3 to 7.22e-3)
   for 8× the cost.

## Results (inference runs, `runs/`, `summary.txt`; N set by the frozen rule)

| base | lifts | targets | hits | F4 mul-adds (1e9) | hits per 1e9 mul-adds [95% Poisson] | lifts/target |
|---|---|---|---|---|---|---|
| isogeny t=5 seed 0 | 0 | 20,000 | 12 | 3.5 | 3.39 [1.75, 5.92] | |
| | 1 | 20,000 | 99 | 28.3 | 3.50 [2.85, 4.26] | 7.98 |
| edwards-bits t=5 | 0 | 20,000 | 10 | 60.7 | 0.165 [0.079, 0.303] | |
| | 1 | 16,464 | 52 | 399.3 | 0.130 [0.097, 0.171] | 7.99 |
| t0-bits t=5 | 0 | 20,000 | 6 | 60.7 | 0.099 [0.036, 0.215] | |
| | 1 | 16,606 | 10 | 403.2 | 0.025 [0.012, 0.046] | 8.00 |
| digits set 1 t=5 | 0 | 20,000 | 73 | 60.6 | 1.205 [0.945, 1.515] | |
| | 1 | 16,488 | 119 | 398.6 | 0.299 [0.247, 0.357] | 7.97 |
| digits set 2 t=6 | 0 | 2,319 | 21 | 397.8 | 0.053 [0.033, 0.081] | |
| | 1 | 294 | 8 | 400.5 | 0.020 [0.009, 0.039] | 7.92 |
| reference bits@0 t=5 | 0 / 1 | 20,000 | 6 / 39 | 60.7 / 485.3 | 0.099 / 0.080 | 7.99 |
| reference square-bits@0 | 0 / 1 | 20,000 | 37 / 107 | 197.5 / 788.5 | 0.187 / 0.136 | 3.99 |
| reference square-bits@414271 | 0 / 1 | 20,000 | 50 / 133 | 197.5 / 787.7 | 0.253 / 0.169 | 3.99 |

Digit set 2 (t = 6) is a small sample by the frozen sizing rule (2,319 and 294 targets); its test is on 21 and 8 hits.
Set 1 is `digits@113971:289589;761384;273167;963514;602728`, set 2 is `digits@51134:293482;1015139;374525;914144;700453;505371`.
The blind arms of `edwards-bits`, `t0-bits`, digit set 1 and `bits` have the same F4 cost per target to within 0.3% (3.03e6 to 3.04e6): their
systems have the same shape, so the difference between them is the hit rate alone.

## Why the lifts do not pay where the frame is already coverage-selected (reading, not tested)

The digit sets in this study were selected for coverage on the even cosets (earlier note: 86% and 84% of their on-curve points
are in descent class +1). A frame that puts its sums on the subgroup already finds most of the targets a lift would; the lift
then raises hits per target by 2.0× for 8× the cost. On the unselected `isogeny` and `bits` bases lifts raise hits about 8× and 6.5× (isogeny
99 against 12, bits 39 against 6), which is why `isogeny` is neutral. I did not test the selection story directly.

## Caveats that bound these claims

- **The isogeny figure is F4-only.** `f4cost` reports `setup_native` empty for the isogeny arms and 0 or 34 set-up group
  operations; the cost of searching the auxiliary curve and building the tower (`IsogenyTowerCost`) is not in the F4 count,
  and I did not extract it. It is one-time and a few thousand group operations at most for p ≈ 2^20, against 3.5e9 to
  2.8e10 counted mul-adds, so I expect it to be negligible, but that is an expectation, not a measurement. With an unpriced
  native unit the cross-base order holds for any common price of a mul-add (§5a prices at 0, 0.1 and 1 GAE: the order does
  not depend on a common factor), and the set-up and lift-addition group operations (at most 891, plus 7 per target) are
  orders of magnitude below the F4 work at any of those prices.
- Hit counts of 6 to 133 (and 8 to 21 for digit set 2) give wide intervals; the five H5 tests are not corrected beyond
  reporting their number (only digit set 1 would survive a Bonferroni correction at 0.05/5).
- Only k = 20, t = 5 (t = 6 for digit set 2), G = 3, m = 2. Nothing is said about scaling, other seeds of the isogeny tower, other
  frames of the digit family, or timing. The `isogeny` result is seed 0 only; the earlier torsion study treats the tower seed as
  a lead, so seed variation is the first thing to check before reading anything into the isogeny base's lead (and it rests on 12 and 99 hits).
- The references come from PR #4111, run earlier with the same binary and seed; they were not rerun.

## Provenance and deviations

Binary `curve25519_family` (release, commit `1edbb0262c`, `descent_skip` flag off), sha256
`ed99cb1653aaa35c9a87c0aa0817a63de0687dc498600e5694b3f7bc443f99bd`; run lines in `runs/run_ob.sh` and `pilot/run_pilot.sh`;
tables by `summarise_other_bases.py` and `summarise_pilot.py` (`pilot_summary.txt`). No Conductor task, no isolab-eks run, no
S3 resume checkpoint (the AWS route is absent in this container; counted mul-adds are deterministic); no research-kb query.
