# Sibling points: two keyed points per inversion in a stored-table walk (mechanism proposal, 2026-11-10)

Owner request: a new mechanism, not a restatement of the closed table. Everything here is derivation in the harness's own `bitop/v1` cost model plus one measured baseline [pcc:unverified]. No run of the mechanism exists yet.

## 1. The assumption the obstruction leaves open

`OBSTRUCTION.md` and `STRUCTURAL-20261110.md` count the floor in stored points and the cost in group additions, and treat them as the same number: one addition per stored point. The floor in points is tight (NC-1, NC-2). The price per point is not. Two facts separate them, and only for methods that store what they compute:

1. An affine addition `P + J` and the subtraction `P - J` share the inversion `1 / (x_J - x_P)`. With `inv` in hand, `lambda_+ = (y_J - y_P) inv` and `lambda_- = -(y_J + y_P) inv`, and `x(P +- J) = lambda^2 - x_P - x_J`. The second point costs one multiplication and one squaring, not an inversion.
2. A stored-table walk keys points by `x` (negation fold), so an x-only point is a full table entry. Its coefficient vector is known (`c_P -+ c_J`), so a key collision on it is a relation row like any other.

A memoryless rho cannot use `P - J`: it has nowhere to put it. A table method stores it. That is an asymmetry between the two classes that the group-addition count hides.

## 2. Price per point in the harness's unit

`bitop/v1` for a prime field of `b` bits (`src/cryptanalysis/bitops.rs`): `add = 13b`, `mul = sqr = 2(b^2 + 5b(b-1)) + 13b`, `inv = 2b * add = 26 b^2`. A rho-neg affine step is `1 inv + 2 mul + 1 sqr + about 6 add`; the k = 28 records price it at 53,022 bit-ops per counted GAE (`snc-panel/sessions/k28`, rho-neg, 4,815 GAE, 2.553e8 bit-ops) [pcc:unverified], against the formula's 51,044 plus word operations.

| bits | add | mul | inv | rho step | sibling pair / 2 | per point ratio |
|--:|--:|--:|--:|--:|--:|--:|
| 20 | 260 | 4,860 | 10,400 | 26,540 | 15,700 | 0.592 |
| 24 | 312 | 6,984 | 14,976 | 37,800 | 22,392 | 0.592 |
| 28 | 364 | 9,492 | 20,384 | 51,044 | 30,268 | 0.593 |
| 32 | 416 | 12,384 | 26,624 | 66,272 | 39,328 | 0.593 |

The sibling pair is `1 inv + 2 (mul + sqr) + about 6 add` for two stored points. Montgomery batching of several inversions costs `3 mul` each, which is more than `inv` in this model, so batching does not pay here and the unbatched rho-neg reference is already the strongest form of the reference. Exactly two points share one inversion (the chord through `P` with abscissa `x_J` meets the curve only at `+-J`), so 0.59 per point is the floor of this mechanism, not a tuning knob.

## 3. Predicted ratios to rho-neg, by unknown count

The point floors are unchanged: `Gamma(K + 1/2) / Gamma(K)` with `K = C + 1`, plus the 3 to 5 percent walk excess measured in `../nc2` and `../snc`. Multiplying by the per-point price:

| track | K | point floor / rho floor | x 0.59 | with walk excess and set-up (estimate) |
|:--|--:|--:|--:|--:|
| stored rho (BSGS-class, excluded) | 1 | 1.00 | 0.59 | about 0.62 |
| C = 1 boundary track | 2 | 1.50 | 0.89 | about 0.93 |
| C = 2 main track | 3 | 1.875 | 1.11 | about 1.15 |

Measured baseline in the same unit today, k = 28, five fresh curves [pcc:unverified]: `tw2cj-lazy` / rho-neg = 1.69 and `inc` / rho-neg = 1.88 in bit-ops per sqrt(r). The prediction for the main track is therefore from 1.69 to about 1.15, and for the boundary track from about 1.40 (GAE, `../c1`) to about 0.93.

It does not reach 0.75 for index calculus. It reaches 0.75 only at K = 1, which is the class the owner excluded. Stated plainly before any run.

## 4. What a measurement needs

- Unit: `bitop/v1` per sqrt(r) (`work.bitops_per_sqrt_r`, already written on every record), or GAE under the full-price convention the Koblitz strong reference uses (`GAE = field multiplications / multiplications per addition`, `methods.rs` summary of `negation_strong`). Counting siblings as group additions would charge them 1.0 each and hide the mechanism; counting them as free native work would be the accounting trap. The full-price convention is neither: it is the harness's existing rule for arms whose steps are not plain additions. The owner rules which of the two units the comparison is made in; the prediction above is the same in both.
- Rho-neg is priced in the same unit on the same records (it already is).

## 5. Stage 0 (zero group operations): does the Poisson law survive?

Children of one parent never collide informatively with each other, and `P_(t+1) - J_h'` is a known-difference relative of `P_t`'s children, so the question is whether the pair count among sibling points is the random one. `nc2_stage0 --siblings m` (branch `claude/siblings-stage0`, pre-registered before running): E in stored points per sqrt(r) must equal the D0 walk's value within 3 percent for m in {1, 2, 4, 8} at C = 1 and C = 2 on the nine NC-2 curves; dead if above 1.08 x D0 at two of three sizes for any m <= 8.

## 6. Stage 1 (if Stage 0 holds)

`wwalk:...,sib=m` on `TargetWalk` (tw = 1 and tw = 2) and `WalkTable`: per parent, `m` inversions, `2m` x-only children keyed and stored with their coefficient vectors, the walk continuing from `P + J_(h_t)` (its `y` costs one more multiplication). Field operations counted exactly as now (`field_ops` on every record). Panel per the SNC protocol (five fresh curves, 12 targets x 8 rounds, rho-neg, the incumbent, `tw2cj-lazy`, `tw2cj-lazy,sib=m` for m in {1, 4}), audited with replays, compared with `ecbench compare` in bit-ops per sqrt(r) and in full-price GAE, band at w = 0, 0.1, 1 unchanged. Pre-registered predictions: `sib=4` / `tw2cj-lazy` = 0.66 [0.60, 0.72] in bit-ops at every size; ratio to rho-neg 1.15 [1.05, 1.25] at k = 28 (C = 2); at C = 1, k = 20, 0.93 [0.85, 1.02]. Dead if `sib=4` / `tw2cj-lazy` has its upper end above 0.80, or if any accepted answer fails the check, or if the stored-point count per sqrt(r) exceeds the no-sibling walk's by more than 8 percent.

## 7. Honest placement

This is the first mechanism in this loop that moves the cost per point rather than the point count, and it is specific to table methods. It brings the main track to about parity with rho-neg in the harness's bit-op unit and the boundary track to about 0.93, both from a floor argument, both still to be measured. The 0.75 line stays out of reach for any K >= 2 because 0.59 x 1.329 / 0.886 = 0.89 > 0.75; a second independent per-point lever of 0.84 or better would be needed on top, and none is known.
