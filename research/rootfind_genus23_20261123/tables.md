# Tables (`analyze.py`) [pcc:unverified]

## T1 Counted Cantor cost (mults + 10 inv) per candidate, per curve

| g | p | curve | A /step (mul, inv) | lab /group-op (mul, inv) | c (sibling marginal / step) | N sib / A step | B/A per candidate | AA/A per step | sib dup | B exact mismatches | verified ok |
|--:|--:|--:|--|--|--:|--:|--:|--:|--:|--:|--:|
| 2 | 65537 | 17 | 88.0, 4.00 | 88.4, 4.00 | 0.445 | 1.000 | 0.727 | 1.0000 | 5790 | 0 | 200/200 |
| 2 | 65537 | 20 | 88.0, 4.00 | 88.4, 4.00 | 0.445 | 1.000 | 0.726 | 1.0000 | 139 | 0 | 200/200 |
| 2 | 65537 | 85 | 88.0, 4.00 | 88.4, 4.00 | 0.445 | 1.000 | 0.727 | 0.9999 | 3400 | 0 | 200/200 |
| 2 | 524309 | 10 | 88.0, 4.00 | 88.3, 4.00 | 0.445 | 1.000 | 0.726 | 1.0000 | 0 | 0 | 200/200 |
| 2 | 524309 | 19 | 88.0, 4.00 | 88.3, 4.00 | 0.445 | 1.000 | 0.726 | 1.0000 | 0 | 0 | 200/200 |
| 2 | 524309 | 54 | 88.0, 4.00 | 88.3, 4.00 | 0.445 | 1.000 | 0.726 | 1.0000 | 0 | 0 | 200/200 |
| 3 | 65537 | 64 | 219.0, 5.00 | 219.7, 5.00 | 0.572 | 1.000 | 0.784 | 1.0000 | 0 | 0 | 200/200 |
| 3 | 65537 | 86 | 219.0, 5.00 | 219.7, 5.00 | 0.572 | 1.000 | 0.783 | 1.0000 | 0 | 0 | 200/200 |
| 3 | 65537 | 157 | 219.0, 5.00 | 219.7, 5.00 | 0.572 | 1.000 | 0.783 | 1.0000 | 0 | 0 | 200/200 |
| 3 | 65537 | 158 | 219.0, 5.00 | 219.7, 5.00 | 0.572 | 1.000 | 0.783 | 1.0000 | 0 | 0 | 200/200 |
| 3 | 65537 | 217 | 219.0, 5.00 | 219.7, 5.00 | 0.573 | 1.000 | 0.784 | 1.0000 | 0 | 0 | 200/200 |
| 3 | 65537 | 250 | 219.0, 5.00 | 219.7, 5.00 | 0.573 | 1.000 | 0.783 | 1.0000 | 0 | 0 | 200/200 |

## T2 Split rates: forward vs sibling (pooled over curves), A/A

| g | p | fwd split rate | sibling split rate | diff [95%] | half-width / rate | A/A diff [95%] | sibling emission rate | dup |
|--:|--:|--:|--:|--|--:|--|--:|--:|
| 2 | 65537 | 0.5029 | 0.5033 | +0.0004 [-0.0029, +0.0038] | 0.007 | -0.0025 [-0.0057, +0.0008] | 0.9382 | 9329 |
| 2 | 524309 | 0.4997 | 0.5017 | +0.0020 [-0.0013, +0.0053] | 0.007 | -0.0003 [-0.0036, +0.0030] | 0.9372 | 0 |
| 3 | 65537 | 0.1677 | 0.1655 | -0.0022 [-0.0043, -0.0000] | 0.013 | -0.0003 [-0.0024, +0.0018] | 0.9377 | 0 |

## T3 Distance to matched rho under the lab's section 5a GAE band (derived projection, not a measurement)

Readings as in `src/bin/genus_ladder_analyze.rs` `costs()`: gae_w = group_ops + w (rel_mul + la_mul + native), for the IC arm and for rho (`R_rho_neg`, its `native` canonicalisation work included); ratio = arm mean / rho mean over the same curves and seeds. Sibling projection: the IC arm's walk_ops scaled by rho_w (B/A per candidate, inversion = 10 mults); setup/verify ops, relation phase and LA unchanged (same relations needed). The current-arm column is this study's recomputation; it reproduces genus_ladder_scale_20261115/tables_g2.md on the g = 2 rows.

| g | p | lab arm | runs | rho_w | w | IC / rho, current arm (gae_w, recomputed here) | IC+sib / rho (projected) | projected / current arm total | group-op share of arm total |
|--:|--:|--|--:|--:|--:|--:|--:|--:|--:|
| 2 | 65537 | C_double_lp+pre | 6 | 0.727 | 0.0 | 1.997 | 1.459 | 0.7306 | 1.0000 |
| 2 | 65537 | C_double_lp+pre | 6 | 0.727 | 0.1 | 29.39 | 28.9 | 0.9833 | 0.0619 |
| 2 | 65537 | C_double_lp+pre | 6 | 0.727 | 1.0 | 153.9 | 153.6 | 0.9982 | 0.0066 |
| 2 | 524309 | C_double_lp+pre | 6 | 0.726 | 0.0 | 1.111 | 0.8066 | 0.7262 | 1.0000 |
| 2 | 524309 | C_double_lp+pre | 6 | 0.726 | 0.1 | 18.65 | 18.37 | 0.9852 | 0.0542 |
| 2 | 524309 | C_double_lp+pre | 6 | 0.726 | 1.0 | 97.62 | 97.47 | 0.9984 | 0.0057 |
| 3 | 65537 | C_double_lp+pre+sge(8,3) | 12 | 0.783 | 0.0 | 0.07138 | 0.05594 | 0.7837 | 1.0000 |
| 3 | 65537 | C_double_lp+pre+sge(8,3) | 12 | 0.783 | 0.1 | 2.227 | 2.213 | 0.9937 | 0.0291 |
| 3 | 65537 | C_double_lp+pre+sge(8,3) | 12 | 0.783 | 1.0 | 11.93 | 11.92 | 0.9994 | 0.0030 |

## T3b Same rows in the lab's FPM readings (cross-genus; `genus_ladder_analyze` fpm_low/mid/high)

fpm = cantor_mul + I cantor_inv + rel_mul + kappa la_mul; low: I = 1, kappa = 1; mid (the lab's primary): I = 10, kappa = g^2; high: I = 30, kappa = g^2. Rho has no rel/la, so its fpm is its Cantor work only. Sibling projection: the Cantor part times rho_w recomputed at the same I.

| g | p | reading | rho_w(I) | IC / rho | IC+sib / rho (projected) | projected / current |
|--:|--:|--|--:|--:|--:|--:|
| 2 | 65537 | fpm_low | 0.763 | 5.284 | 4.81 | 0.9102 |
| 2 | 65537 | fpm_mid | 0.727 | 4.624 | 4.078 | 0.8819 |
| 2 | 65537 | fpm_high | 0.691 | 3.615 | 2.998 | 0.8293 |
| 2 | 524309 | fpm_low | 0.761 | 3.222 | 2.956 | 0.9175 |
| 2 | 524309 | fpm_mid | 0.726 | 2.999 | 2.693 | 0.8982 |
| 2 | 524309 | fpm_high | 0.691 | 2.273 | 1.928 | 0.8486 |
| 3 | 65537 | fpm_low | 0.818 | 0.1778 | 0.1648 | 0.9268 |
| 3 | 65537 | fpm_mid | 0.783 | 0.3459 | 0.3304 | 0.9552 |
| 3 | 65537 | fpm_high | 0.736 | 0.2715 | 0.2527 | 0.9306 |

## T3c Superseded: this study's first pricing (not the section 5a band; kept only for the record)

W_cantor + w (rel_mul + la_mul) against rho's W_cantor with rho's `native` left out, w per field multiplication. The red-team review showed this is not the lab's band; no conclusion in README.md rests on it.

| g | p | w | IC / rho | IC+sib / rho | projected / current |
|--:|--:|--:|--:|--:|--:|
| 2 | 65537 | 0.0 | 1.9999 | 1.4536 | 0.7268 |
| 2 | 65537 | 0.1 | 2.2360 | 1.6897 | 0.7557 |
| 2 | 65537 | 1.0 | 4.3614 | 3.8151 | 0.8747 |
| 2 | 524309 | 0.0 | 1.1129 | 0.8076 | 0.7257 |
| 2 | 524309 | 0.1 | 1.2644 | 0.9592 | 0.7586 |
| 2 | 524309 | 1.0 | 2.6281 | 2.3229 | 0.8839 |
| 3 | 65537 | 0.0 | 0.0716 | 0.0561 | 0.7835 |
| 3 | 65537 | 0.1 | 0.0804 | 0.0649 | 0.8073 |
| 3 | 65537 | 1.0 | 0.1600 | 0.1445 | 0.9031 |
