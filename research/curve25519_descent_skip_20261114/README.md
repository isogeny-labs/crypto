# Do cofactor lifts pay on the k = 20 Curve25519 miniature, and does a descent-class skip help? No, and it is already implicit.

2026-11-14. Protocol: [`PROTOCOL.md`](PROTOCOL.md) (frozen in `1edbb0262c`; amendments A1 to A3 dated and
committed before the data they govern). Follows
[`research/curve25519_miniature_descent_class_20261111`](../curve25519_miniature_descent_class_20261111/README.md).
Everything is [pcc:unverified]: no claim, replay certificate, validator report or matched rho. **No rho arm was
measured, so there is no cross-method comparison and no distance to the matched rho.** Units are counted F4
multiply-adds (deterministic), not seconds. No benchmark record, no leaderboard row, no figure or PDF
(matplotlib is not installed here).

## Answer

1. **The lift oracle already avoids class −1 lifts on a `square-bits` base, implicitly.** `cofactor_fiber`
   (`ic_framework/fiber.rs`) generates E[8] from `[r]P` over the *base points*. On `square-bits` every base point is
   in 2E(F_p), so every `[r]P` lies in 2E[8] and the fiber has 4 elements, not 8. The oracle then tries 4 lifts per
   target (all of them class +1). Evidence: `lifts=1` costs 4.00×, 3.98×, 4.00× the blind arm in the three Part A
   cells and 3.99× in Part C and C2, with `pkm_lift_targets_tried` = 4 per target; the post hoc control on the mixed-class
   `bits` base tries 7.99 lifts per target and costs 7.99×.
2. **An explicit descent skip is a no-op.** The opt-in `descent_skip=1` flag fired 0 times in all three cells and every
   count equals plain `lifts=1`; it only adds 4 Legendre symbols per target. My registered prediction that it cuts F4
   calls to 0.45× to 0.62× (P2) is **falsified** by the reading above. The flag is not kept in `src/`
   ([`descent_skip.patch`](descent_skip.patch)).
3. **A one-bit class label is enough for frame selection** (Part B, registered rule met).
4. **Lifts do not pay in the counted unit on `square-bits`, and do not resolve on `bits`** (Part C2, below).

## Part A: F4 cost of the arms (100 targets, seed 2026111401, G = 3; `runs/`, `runs_summary.txt`)

| cell | arm | hits | F4 calls | F4 mul-adds per target | lifts tried |
|---|---|---|---|---|---|
| square-bits@414271 t=5 | blind | 0 | 800 | 9.89e6 (1.00×) | |
| | lifts=1 | 0 | 3200 | 3.96e7 (4.00×) | 400 |
| | lifts=1 + skip | 0 | 3200 | 3.96e7 (4.00×) | 400, skipped 0 |
| square-bits@984071 t=4 | blind / lifts=1 / +skip | 0 / 1 / 1 | 800 / 3187 / 3187 | 1.87e5 / 7.45e5 (3.98×) / same | 399 |
| square-bits@0 t=5 | blind / lifts=1 / +skip | 0 / 1 / 1 | 800 / 3196 / 3196 | 9.89e6 / 3.95e7 (4.00×) / same | 400 |
| bits@0 t=5, post hoc control | blind / lifts=1 | 0 / 2 | 800 / 6370 | 3.04e6 / 2.42e7 (7.96×) | 798 |

## Part B: frame selection by the one-bit class (2,000 frames, t = 5; `frame_selection.out.txt`)

| quantity | value |
|---|---|
| Spearman ρ(full coset label score, exact coverage) | 1.0000 |
| Spearman ρ(one-bit class score, exact coverage) | 0.9245 |
| mean coverage of the frame chosen from a random pool of 100: random / one-bit / full label / exact | 35.4 / 70.8 / 77.1 / 77.1 |

The one-bit choice reaches 91.9% of the full-label choice (rule: at least 90%) and 2.0× a random frame. The parity of
the coset label equals the Legendre class chi(x_M) on all 1,547 test points (`parity_check.out.txt`), so the one-bit
score costs one Legendre symbol per column where the full label needs `[r]P` and a discrete log. No saving is claimed
without a priced comparison.

## Part C and C2: yield per counted F4 mul-add (G = 3, t = 5)

Part C (2,000 targets, seed 2026111402, `runs_c_summary.txt`) found 2 to 19 hits per arm and no separable
interval. Part C2 (20,000 targets, fresh seed 2026111403, hypotheses frozen in A3 before its data,
`runs_c2_summary.txt`):

| base | lifts | hits | F4 mul-adds (1e9) | lifts/target | hits per 1e9 mul-adds [95% Poisson] |
|---|---|---|---|---|---|
| bits@0 | 0 | 6 | 60.7 | | 0.099 [0.036, 0.215] |
| bits@0 | 1 | 39 | 485.4 | 7.99 | 0.080 [0.057, 0.110] |
| square-bits@0 | 0 | 37 | 197.5 | | 0.187 [0.132, 0.258] |
| square-bits@0 | 1 | 107 | 788.5 | 3.99 | 0.136 [0.111, 0.164] |
| square-bits@414271 | 0 | 50 | 197.5 | | 0.253 [0.188, 0.334] |
| square-bits@414271 | 1 | 133 | 787.7 | 3.99 | 0.169 [0.141, 0.200] |

Registered hypotheses (A3):
- **H3 supported**: lifts=1 multiplies the cost per target by 7.99× on `bits` and 3.99× on `square-bits`.
- **H1 supported, barely**: `square-bits@0` lifts=1 (0.136, lower end 0.111) over `bits` lifts=1 (0.080, upper end 0.110).
- **H2 unresolved**: on `bits`, lifts=1 is lower in point estimate (0.080 against 0.099) but the intervals overlap broadly (6 blind hits).
- **P5 (registered in A2) falsified** (`bits` lifts=1 is the lowest of the three lifts=1 arms, not the highest). P4 (lifts lower yield per mul-add) held in point
  estimate on `bits` and both `square-bits` cells in C2; the first Part C (2,000 targets) disagreed on `square-bits@0`.

Post hoc, not registered (`rate_ratio.out.txt`; exact conditional binomial test on hits against counted mul-adds,
four tests, no further correction): lifts=1 against blind, rate ratio **0.69 pooled over the two `square-bits`
cells, p = 0.0045** (cells alone: 0.72, p = 0.096; 0.67, p = 0.016), and **0.81 on `bits`, p = 0.63**.

Lifts raise hits per target (6.5× on `bits`, 2.9× and 2.7× on the `square-bits` cells) but cost 8× and 4×.
Unpriced work is small: fiber set-up of 34, 791 and 891 group operations, against 6e10 to 8e11 F4 mul-adds.
The best arm in this table is the blind `square-bits@414271` (frame chosen by coverage): 0.253, about 2.6× the best `bits` arm in
point estimate (blind `bits`, 0.099); against `bits` blind the intervals overlap, against `bits` lifts=1 (0.080 [0.057, 0.110]) they do not.

## What this does and does not say

- It says: on this miniature and at this size, the cofactor lifts cost more F4 work than the extra relations
  return, by about a third on `square-bits` (resolved) and by an unresolved amount on `bits`, and a class-based
  skip has nothing left to remove. That fits "a constant, not an asymptotic, gain", and here the constant is below 1 in the counted unit.
- It does not say anything about rho, other k, other t, G other than 3, other bases, or timing. Hit counts of 6 to 133 give
  wide intervals. Isogeny, edwards-bits, t0-bits and digits bases were not run in the engine.

## Corrections to my earlier notes

- "Exactly half of the 8 lifts are provably dead for `square-bits`" is true of the group but the oracle never generates
  them (point 1). "I found no class-based skip" was right about the code and missed that `cofactor_fiber` does the work.
- The earlier note that the mechanism is the repository's own lever T2 stands; the new part is that its lift side is
  already handled.

## Provenance and deviations

- Binary: `curve25519_family`, release, built from `1edbb0262c` (flag present, default off, which leaves the other
  arms' counts equal to the unflagged oracle), sha256
  `ed99cb1653aaa35c9a87c0aa0817a63de0687dc498600e5694b3f7bc443f99bd` (the build log is not committed). Runs: `runs/run_cell.sh`, `runs_c/run_c.sh`, `runs_c2/run_c2.sh`.
- No Conductor task, no isolab-eks run, no S3 resume checkpoint (the AWS route is absent in this container; counted
  mul-adds are deterministic), no research-kb query. A first Poisson-interval script overflowed at 107 hits; the log-space
  fix reproduced the Part C table byte for byte.
