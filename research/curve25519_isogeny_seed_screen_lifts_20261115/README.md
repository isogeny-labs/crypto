# Lifts arms on the screened isogeny seeds

All claims `[pcc:unverified]`. Protocol: [PROTOCOL.md](PROTOCOL.md) (`e83263d48e`, frozen before the arms; model fitted on seeds 0-20 lifts only).
Same engine, binary and conditions as PR #4213; the 16 `lifts=1` arms of the top and bottom seeds chosen in `curve25519_isogeny_seed_screen_20261115`.
Output: [summary.txt](summary.txt); `analyse.py`; raw JSON in `runs/`.

## Result: all five hypotheses supported

| | outcome |
|---|---|
| H17 top > bottom under lifts | top 1014 hits / 225.9e9 mul-adds = **4.488** [4.216, 4.773]; bottom 368 / 226.4e9 = **1.625** [1.463, 1.800]; ratio 2.76, one-sided p = 1.2e-70. |
| H18 top beats unscreened | top lower bound 4.216 > 2.864 (seeds 1-20 lifts). Top is 1.57x the unscreened lifts rate. |
| H19 lifts neutral | pooled lifts 3.055 vs blind 3.477 on these 16 seeds, ratio 0.879, two-sided p = 0.093: not rejected, but close to the 0.05 line. |
| H20 model transfers | the lifts model (hits = 0.0022857 x exposure x base_points^2.05, fitted on seeds 0-20 only) predicts 1021.8 for the top group (interval 960 to 1085; observed 1014) and 376.7 for the bottom (interval 339 to 415; observed 368). |
| H21 cost shape | lifts mul-adds per target 1.4117e6 to 1.4156e6, spread 0.28%. |

Spearman of on-curve count vs lifts arm rate over 16 arms: 0.940.

## Notes
- The model with exponent about 2 fitted on the unscreened seeds predicted the screened groups to within 1% in the lifts arm, which is the out-of-sample test the blind arm's linear model failed (PR #4233). A hits-grow-as-pairs-of-base-points picture is now supported by two arms; the mechanism itself is still untested.
- Lifts/blind: 0.87 (top) and 0.90 (bottom) here, so both groups sit below 1 as the blind arm is a little ahead per mul-add. Pooling these 16 seeds with seeds 1-20 (descriptive, not pre-registered): lifts 2.949 vs blind 3.161, ratio 0.933, two-sided p = 0.19. Neutrality is not rejected, and a small lifts penalty is not excluded.
- Lifts cost 8x per target but pay back the same hits per mul-add at about 0.9x of blind: no reason to prefer lifts on this base. Screening is what moves the rate (1.57x unscreened under lifts, 1.77x under blind).
- Limits: one k = 20 toy, t = 5, one target set; seed selection not priced in the 0 to 1 GAE band; no rho arm, so the gap to rho is unchanged.
