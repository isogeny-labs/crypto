# Multi-target IC vs a multi-target distinguished-point rho and the tuned BSGS (frozen evaluation)

Status: [pcc:unverified] producer's own measurement; not independently replayed; same host, 1 thread, wall clock, toy sizes (25 to 31 bits).

New arm: `examples/prime_fast_rho_multi.rs`, a multi-target DP rho on the same native engine. 32-step r-adding walk over multiples of G only (a walk from `u G + Q_t` stays `a G + Q_t`), 2002 lanes advanced in lock-step with one batched inversion per round, a walker restarts at `Q_t + A_i + B_j` (two tables of 4096 random multiples) after every distinguished point, no negation map. A collision of walks of targets t1, t2 gives `a1 + k_t1 = +-(a2 + k_t2)`; a signed union-find over the targets with an anchor target Q_0 = G (log 1) determines the logs; the run stops when every target is determined and every log is verified `k G = Q`. First version (no restarts): only 872 and 877 of 1000 targets resolved after 400 x sqrt(nT) steps (9.3 s) at 27 bits: prediction failed (arm weak); with restarts: 1000/1000 in 142 and 158 ms. Predictions and the failed first run are in `results/pf_rho/PREDICTION.txt`.

Protocol: frozen in `results/pf_rho2/frozen.json` (commit 4a20ce1da4, pushed and checked on the remote before the evaluation, prediction and abandon rule stated). Identical planted targets for all arms (same seed formula). Curves 25, 29, 31 bits; T = 1000 and 10 000; IC arms: batched multi-target with the bucketed pair table, c = 1.0 and 2.0 (best of the two taken per row); tuned BSGS; the multi-target rho. 20 rows per arm, 80 arm-runs, every target of every run verified (the rho resolved all T targets in every run), no failed run. Provenance: `provenance.txt`.

Results (medians of per-row ratios; total time; best IC = min over c = 1, 2):

| T | rows | best IC / rho | rho / BSGS | best IC / BSGS |
|---|---|---|---|---|
| 1000 | 10 | 0.53 (0.19 to 1.48) | 3.38 | 2.02 |
| 10 000 | 10 | 0.58 (0.30 to 1.28) | 2.69 | 1.94 |

By size (best IC / rho; rho ms; best IC ms; BSGS ms; rho steps):
- T = 1000: 25 bits 0.19 (171; 35; 17; 5.1e5); 29 bits 0.61 (194; 121; 62; 2.2e6); 31 bits **1.31** (272; 359; 119; 4.4e6).
- T = 10 000: 25 bits 0.37 (371; 139; 70; 2.2e6); 29 bits 0.70 (709; 493; 289; 1.0e7); 31 bits **1.10** (963; 1043; 460; 2.0e7).

Prediction check: "best IC / rho <= 0.7 at T = 1000 at 25, 29 and 31 bits" holds at 25 and 29 bits and the pooled median (0.53) but **fails at 31 bits (1.31)**; the abandon rule (median >= 1.0 at 29 bits, T = 1000, or an unresolved rho run) was not triggered.

Finding: the batched, bucketed multi-target IC is faster than this multi-target rho at 25 and 29 bits (by 5x down to 1.6x at T = 1000) and slower at 31 bits (1.3x at T = 1000, 1.1x at T = 10 000). The advantage shrinks monotonically with the field size and has gone by 31 bits, in line with the exponent argument (IC setup about p^(2/3), per-target about p^(1/3); rho about sqrt(nT)). So there is **no sustained improvement over a rho-type multi-target attack either**: at best a small-size win that reverses by about 30 bits. This is the first same-run comparison of the IC with an independent rho-type multi-target arm.

Limits and caveats that all favour the rho being faster than measured here: no negation map (about sqrt(2) fewer steps), no walk or DP tuning (lanes, table size, DP bits chosen once from a formula), a simple restart scheme (the arm is 3.4x slower than the tuned BSGS), one thread. Against that, the rho arm is my own implementation and the IC rows come from earlier-tuned constants. One instance at 31 bits per residue (2 rows), three sizes only; the BSGS arm is not compacted; no §5a pricing band; not independently replayed; no certificate file, bench record or `VALIDATION.json`.

## Extension to 33 and 35 bits (exploratory; prediction in `results/pf_rho2/PREDICTION_33_35.txt`, written before the run)

T = 1000, one instance per residue, identical targets as the existing IC (c = 1.0; `results/pf_size/ic_c1_33.jsonl`, `ic_c1_35.jsonl`) and tuned-BSGS rows. The multi-target rho resolved and verified all 1000 targets in every run.

| bits | residue | rho ms | rho steps | IC ms (c = 1) | IC / rho | tuned BSGS ms | rho / BSGS |
|---|---|---|---|---|---|---|---|
| 33 | 3 | 501 | 7.9e6 | 956 | 1.91 | 297 | 1.69 |
| 33 | 1 | 418 | 8.3e6 | 818 | 1.96 | 253 | 1.65 |
| 35 | 3 | 817 | 1.73e7 | 2162 | 2.65 | 537 | 1.52 |
| 35 | 1 | 848 | 1.77e7 | 1977 | 2.33 | 495 | 1.71 |

Prediction (rho about 550 ms at 33 bits and about 1.1 s at 35 bits; IC / rho about 1.5 to 1.7 and 1.8 to 2.0): the rho is faster than predicted (418 to 501 ms and 817 to 848 ms) and IC / rho is higher than predicted (1.9 to 2.0 and 2.3 to 2.7). Together with the frozen evaluation (best IC / rho = 0.19, 0.61, 1.31 at 25, 29, 31 bits for T = 1000) the picture is: IC faster than this rho up to about 29 bits, slower from 31 bits, and the gap grows with the size (about 2x at 33 bits, 2.3x to 2.7x at 35 bits). The rho is 1.5x to 1.7x behind the tuned BSGS here (3.4x at the frozen sizes at T = 1000: the ratio improves with size because the DP overhead per step shrinks), so the BSGS stays the strongest multi-target reference. Limits as above (no negation map or rho tuning in this arm, one instance per residue, exploratory, same host, not independently replayed).
