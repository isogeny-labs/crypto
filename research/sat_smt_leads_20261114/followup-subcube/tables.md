rows 1808 OK 1744 NOT_RUN 0 SKIPPED_AFTER_CENSOR 64

## Part A: conflicts per leaf (non-decomposable targets)

| l | n | A-plain-inc mean (max; zero-conflict share; censored leaves; targets) | A-plain-cold mean (max; zero-conflict share; censored leaves; targets) | A-dual-inc mean (max; zero-conflict share; censored leaves; targets) | A-dual-cold mean (max; zero-conflict share; censored leaves; targets) |
|---|---|---|---|---|---|
| 4 | 13 | 151.39 (695; 0.06; 0; 8) | 248.42 (685; 0.06; 0; 8) | 3.36 (73; 0.32; 0; 8) | 17.98 (81; 0.07; 0; 8) |
| 5 | 17 | 551.03 (1822; 0.03; 0; 8) | 727.56 (2098; 0.03; 0; 8) | 1.64 (26; 0.22; 0; 8) | 23.44 (169; 0.03; 0; 8) |
| 6 | 19 | 1949.05 (8192; 0.02; 0; 8) | 2735.07 (8252; 0.02; 0; 8) | 1.75 (31; 0.28; 0; 8) | 19.08 (197; 0.02; 0; 8) |
| 7 | 23 | 8390.23 (20003; 0.02; 15; 8) | 10015.91 (20002; 0.02; 25; 8) | 1.68 (31; 0.24; 0; 8) | 29.73 (319; 0.00; 0; 8) |
| 8 | 29 | 19569.87 (20004; 0.00; 239; 8) | 19594.40 (20003; 0.00; 240; 8) | 1.60 (46; 0.26; 0; 8) | 26.40 (420; 0.00; 0; 8) |
slope of log2(conflicts/leaf) in l, A-plain-inc: 1.80 bits per l over l = [4, 5, 6, 7, 8]
slope of log2(conflicts/leaf) in l, A-plain-cold: 1.64 bits per l over l = [4, 5, 6, 7, 8]
slope of log2(conflicts/leaf) in l, A-dual-inc: -0.21 bits per l over l = [4, 5, 6, 7, 8]
slope of log2(conflicts/leaf) in l, A-dual-cold: 0.15 bits per l over l = [4, 5, 6, 7, 8]
engine l=4: leaves/target 9.0, sub-nodes per leaf 2.03, row XORs per leaf 1760, pruned above depth l: 0
engine l=5: leaves/target 16.0, sub-nodes per leaf 2.00, row XORs per leaf 2304, pruned above depth l: 0
engine l=6: leaves/target 33.0, sub-nodes per leaf 2.03, row XORs per leaf 4974, pruned above depth l: 0
engine l=7: leaves/target 63.0, sub-nodes per leaf 2.00, row XORs per leaf 5297, pruned above depth l: 0
engine l=8: leaves/target 120.0, sub-nodes per leaf 2.00, row XORs per leaf 12177, pruned above depth l: 0

## Part B: keep learnt clauses, std order, non-decomposable

| budget | l=4 rho (rho'; leaves; conflicts; targets) | l=5 rho (rho'; leaves; conflicts; targets) | l=6 rho (rho'; leaves; conflicts; targets) | l=7 rho (rho'; leaves; conflicts; targets) | l=8 rho (rho'; leaves; conflicts; targets) | trend bits/l |
|---|---|---|---|---|---|---|
| B=0 | 3.88 (4.81; 16.0; 46; 8) | 3.41 (4.38; 32.0; 77; 8) | 3.40 (4.38; 64.0; 154; 8) | 2.49 (3.48; 128.0; 191; 8) | 2.48 (3.47; 256.0; 378; 8) | -0.17 |
| B=10 | 5.93 (6.60; 1.2; 94; 8) | 4.71 (5.22; 1.8; 149; 8) | 4.44 (4.95; 2.0; 282; 8) | 3.37 (3.77; 1.8; 430; 8) | 3.75 (4.18; 3.8; 957; 8) | -0.18 |
| B=100 | 20.34 (20.71; 0.0; 325; 8) | 15.07 (15.30; 0.0; 482; 8) | 12.35 (12.54; 0.0; 790; 8) | 11.65 (11.83; 1.2; 1490; 8) | 7.95 (8.07; 1.8; 2034; 8) | -0.31 |
| B=1000 | 108.98 (109.19; 0.0; 1744; 8) | 96.86 (97.07; 0.0; 3100; 8) | 59.64 (59.76; 0.0; 3817; 8) | 42.91 (42.98; 0.0; 5492; 8) | 43.32 (43.40; 0.0; 11089; 8) | -0.38 |
| 0.25*2^j | 4.35 (5.27; 15.5; 54; 8) | 3.41 (4.27; 23.5; 86; 8) | 3.95 (4.80; 45.5; 207; 8) | 3.30 (4.08; 72.5; 350; 8) | 3.52 (4.30; 148.0; 754; 8) | -0.07 |
| 0.5*2^j | 4.60 (5.49; 7.0; 67; 8) | 3.79 (4.50; 4.5; 117; 8) | 4.21 (4.93; 8.2; 261; 8) | 4.03 (4.71; 8.5; 507; 8) | 4.39 (4.96; 16.2; 1108; 8) | -0.00 |
| 1*2^j | 5.73 (6.45; 4.2; 87; 8) | 4.74 (5.27; 3.0; 149; 8) | 6.01 (6.62; 5.5; 379; 8) | 5.50 (5.90; 3.2; 701; 8) | 6.01 (6.33; 9.0; 1531; 8) | +0.04 |

## Part B: discard (fresh solver per node), std order, non-decomposable

| budget | l=4 rho (rho'; leaves; conflicts; targets) | l=5 rho (rho'; leaves; conflicts; targets) | l=6 rho (rho'; leaves; conflicts; targets) | l=7 rho (rho'; leaves; conflicts; targets) | l=8 rho (rho'; leaves; conflicts; targets) | trend bits/l |
|---|---|---|---|---|---|---|
| B=0 | 18.98 (19.91; 16.0; 288; 8) | 24.44 (25.41; 32.0; 750; 8) | 20.08 (21.06; 64.0; 1221; 8) | 29.72 (30.71; 128.0; 3676; 8) | 26.46 (27.46; 256.0; 6518; 8) | +0.12 |
| B=10 | 28.76 (29.70; 15.0; 445; 8) | 35.84 (36.81; 31.8; 1115; 8) | 31.77 (32.75; 64.0; 1969; 8) | 41.30 (42.29; 125.8; 5161; 8) | not run | +0.14 |
| B=100 | 58.07 (58.87; 0.5; 929; 8) | 76.69 (77.64; 2.5; 2452; 8) | 75.40 (76.35; 7.0; 4818; 8) | 88.81 (89.80; 34.0; 11334; 8) | 108.30 (109.30; 149.2; 27577; 8) | +0.20 |
| B=1000 | 113.72 (113.89; 0.0; 1820; 8) | 234.16 (234.50; 0.0; 7493; 8) | 263.74 (264.09; 0.0; 16880; 8) | 312.30 (312.71; 0.0; 39974; 8) | not run | +0.45 |
| 0.25*2^j | 20.35 (21.29; 16.0; 310; 8) | 26.13 (27.10; 32.0; 804; 8) | 22.04 (23.03; 64.0; 1347; 8) | 31.98 (32.97; 128.0; 3965; 8) | not run | +0.17 |
| 0.5*2^j | 22.47 (23.41; 16.0; 344; 8) | 28.55 (29.52; 32.0; 882; 8) | 24.83 (25.82; 64.0; 1525; 8) | 34.94 (35.93; 126.0; 4346; 8) | not run | +0.17 |
| 1*2^j | 24.73 (25.66; 16.0; 380; 8) | 31.21 (32.18; 32.0; 967; 8) | 27.92 (28.91; 64.0; 1723; 8) | 38.49 (39.49; 126.0; 4801; 8) | 36.34 (37.34; 254.0; 9049; 8) | +0.14 |

## Part B: control: random coordinate order, keep, non-decomposable

| budget | l=4 rho (rho'; leaves; conflicts; targets) | l=5 rho (rho'; leaves; conflicts; targets) | l=6 rho (rho'; leaves; conflicts; targets) | l=7 rho (rho'; leaves; conflicts; targets) | l=8 rho (rho'; leaves; conflicts; targets) | trend bits/l |
|---|---|---|---|---|---|---|
| B=0 | 4.16 (5.09; 16.0; 50; 8) | 3.02 (3.98; 32.0; 64; 8) | 3.22 (4.21; 64.0; 142; 8) | 2.82 (3.82; 128.0; 234; 8) | 2.61 (3.61; 256.0; 413; 8) | -0.14 |
| B=10 | 5.67 (6.33; 1.5; 89; 8) | 4.60 (5.16; 1.2; 146; 8) | 4.61 (5.13; 3.2; 292; 8) | 4.43 (4.95; 2.5; 564; 8) | 3.77 (4.18; 5.2; 959; 8) | -0.12 |
| B=100 | 19.38 (19.73; 0.0; 310; 8) | 13.92 (14.14; 0.0; 446; 8) | 13.51 (13.71; 0.0; 864; 8) | 11.43 (11.61; 0.8; 1463; 8) | 7.86 (7.98; 1.5; 2011; 8) | -0.29 |
| B=1000 | not run | not run | not run | not run | not run | +nan |
| 0.25*2^j | not run | not run | not run | not run | not run | +nan |
| 0.5*2^j | not run | not run | not run | not run | not run | +nan |
| 1*2^j | 5.49 (6.23; 4.0; 84; 8) | 5.12 (5.77; 3.2; 160; 8) | 5.29 (5.78; 4.2; 334; 8) | 6.01 (6.50; 4.2; 764; 8) | 5.90 (6.20; 12.0; 1498; 8) | +0.04 |

## Part B: control: second basis of V, keep, non-decomposable

| budget | l=4 rho (rho'; leaves; conflicts; targets) | l=5 rho (rho'; leaves; conflicts; targets) | l=6 rho (rho'; leaves; conflicts; targets) | l=7 rho (rho'; leaves; conflicts; targets) | l=8 rho (rho'; leaves; conflicts; targets) | trend bits/l |
|---|---|---|---|---|---|---|
| B=0 | 4.05 (4.99; 16.0; 49; 8) | 3.53 (4.50; 32.0; 81; 8) | 3.23 (4.21; 64.0; 142; 8) | 2.78 (3.77; 128.0; 228; 8) | 2.47 (3.47; 256.0; 377; 8) | -0.18 |
| B=10 | 5.70 (6.36; 0.8; 90; 8) | 5.57 (6.15; 3.8; 175; 8) | 4.38 (4.89; 1.0; 279; 8) | 4.34 (4.83; 3.2; 552; 8) | 4.17 (4.63; 7.8; 1059; 8) | -0.13 |
| B=100 | 17.81 (18.12; 0.0; 285; 8) | 15.18 (15.43; 0.0; 486; 8) | 13.49 (13.70; 0.0; 864; 8) | 10.76 (10.93; 0.0; 1378; 8) | 8.16 (8.29; 1.2; 2088; 8) | -0.27 |
| B=1000 | not run | not run | not run | not run | not run | +nan |
| 0.25*2^j | not run | not run | not run | not run | not run | +nan |
| 0.5*2^j | not run | not run | not run | not run | not run | +nan |
| 1*2^j | 5.41 (6.23; 3.2; 83; 8) | 5.40 (6.07; 3.8; 169; 8) | 5.99 (6.60; 5.8; 378; 8) | 6.13 (6.64; 4.8; 780; 8) | 6.13 (6.46; 9.0; 1560; 8) | +0.05 |

## Part B: decomposable (planted) targets, keep, std order

| budget | l=4 rho (rho'; leaves; conflicts; targets) | l=5 rho (rho'; leaves; conflicts; targets) | l=6 rho (rho'; leaves; conflicts; targets) | l=7 rho (rho'; leaves; conflicts; targets) | l=8 rho (rho'; leaves; conflicts; targets) | trend bits/l |
|---|---|---|---|---|---|---|
| B=0 | 1.89 (2.17; 3.0; 27; 4) | 1.19 (1.45; 6.2; 32; 4) | 0.91 (1.15; 12.5; 46; 4) | 0.39 (0.59; 22.2; 28; 4) | 0.94 (1.33; 96.2; 146; 4) | -0.36 |
| B=10 | 2.73 (3.02; 0.5; 43; 4) | 2.07 (2.28; 0.5; 66; 4) | 1.48 (1.62; 1.8; 93; 4) | 0.92 (1.03; 0.5; 117; 4) | 1.65 (1.84; 1.5; 422; 4) | -0.26 |
| B=100 | 12.95 (13.12; 0.0; 207; 4) | 12.30 (12.47; 0.0; 394; 4) | 9.27 (9.41; 0.0; 593; 4) | 4.98 (5.06; 0.0; 638; 4) | 5.27 (5.35; 0.5; 1348; 4) | -0.39 |
| B=1000 | 67.27 (67.38; 0.0; 1076; 4) | 99.99 (100.14; 0.0; 3200; 4) | 58.46 (58.55; 0.0; 3742; 4) | 35.51 (35.56; 0.0; 4545; 4) | 33.27 (33.32; 0.0; 8516; 4) | -0.35 |
| 0.25*2^j | 1.50 (1.78; 3.0; 21; 4) | 1.11 (1.34; 5.0; 30; 4) | 1.30 (1.51; 9.0; 74; 4) | 1.04 (1.23; 20.8; 113; 4) | 1.71 (2.02; 63.8; 373; 4) | +0.03 |
| 0.5*2^j | 1.84 (2.12; 3.0; 26; 4) | 1.68 (1.91; 3.0; 51; 4) | 1.82 (2.04; 3.8; 113; 4) | 1.29 (1.41; 1.5; 164; 4) | 2.30 (2.55; 8.0; 582; 4) | +0.03 |
| 1*2^j | 2.55 (2.83; 1.5; 39; 4) | 2.51 (2.72; 2.0; 78; 4) | 2.81 (2.98; 2.8; 177; 4) | 2.30 (2.41; 2.0; 293; 4) | 3.81 (3.97; 4.0; 971; 4) | +0.10 |

## rho relative to the solver's own scan (keep, B=0)

B=0 l=4: 1.00 l=5: 1.00 l=6: 1.00 l=7: 1.00 l=8: 1.00
B=10 l=4: 1.53 l=5: 1.38 l=6: 1.31 l=7: 1.36 l=8: 1.51
B=100 l=4: 5.25 l=5: 4.42 l=6: 3.63 l=7: 4.68 l=8: 3.21
B=1000 l=4: 28.12 l=5: 28.44 l=6: 17.55 l=7: 17.24 l=8: 17.48
0.25*2^j l=4: 1.12 l=5: 1.00 l=6: 1.16 l=7: 1.33 l=8: 1.42
0.5*2^j l=4: 1.19 l=5: 1.11 l=6: 1.24 l=7: 1.62 l=8: 1.77
1*2^j l=4: 1.48 l=5: 1.39 l=6: 1.77 l=7: 2.21 l=8: 2.43

## prune-depth histograms (keep, std, non-decomposable; mean nodes pruned per target at depth k = 0..l)

l=4 B=0       0.0 0.0 0.0 0.0 16.0  share of leaves covered by prunes above depth l: 0.00
l=4 B=10      0.0 0.0 2.1 3.1 1.2  share of leaves covered by prunes above depth l: 0.92
l=4 B=100     0.0 0.9 1.9 0.8 0.0  share of leaves covered by prunes above depth l: 1.00
l=4 B=1000    0.5 0.5 0.9 0.2 0.0  share of leaves covered by prunes above depth l: 1.00
l=4 0.25*2^j  0.0 0.0 0.1 0.0 15.5  share of leaves covered by prunes above depth l: 0.03
l=4 0.5*2^j   0.0 0.0 0.4 3.8 7.0  share of leaves covered by prunes above depth l: 0.56
l=4 1*2^j     0.0 0.0 1.8 2.4 4.2  share of leaves covered by prunes above depth l: 0.73
l=5 B=0       0.0 0.0 0.0 0.0 0.0 32.0  share of leaves covered by prunes above depth l: 0.00
l=5 B=10      0.0 0.0 0.9 4.8 2.1 1.8  share of leaves covered by prunes above depth l: 0.95
l=5 B=100     0.0 0.9 1.2 1.9 0.2 0.0  share of leaves covered by prunes above depth l: 1.00
l=5 B=1000    0.0 1.0 1.1 1.8 0.0 0.0  share of leaves covered by prunes above depth l: 1.00
l=5 0.25*2^j  0.0 0.0 0.0 1.9 0.5 23.5  share of leaves covered by prunes above depth l: 0.27
l=5 0.5*2^j   0.0 0.0 0.4 3.1 6.0 4.5  share of leaves covered by prunes above depth l: 0.86
l=5 1*2^j     0.0 0.0 1.0 4.0 2.5 3.0  share of leaves covered by prunes above depth l: 0.91
l=6 B=0       0.0 0.0 0.0 0.0 0.0 0.0 64.0  share of leaves covered by prunes above depth l: 0.00
l=6 B=10      0.0 0.0 0.0 2.6 7.2 6.0 2.0  share of leaves covered by prunes above depth l: 0.97
l=6 B=100     0.0 0.1 2.1 2.4 1.4 0.8 0.0  share of leaves covered by prunes above depth l: 1.00
l=6 B=1000    0.0 1.0 1.0 1.8 0.5 0.0 0.0  share of leaves covered by prunes above depth l: 1.00
l=6 0.25*2^j  0.0 0.0 0.0 0.0 4.2 0.8 45.5  share of leaves covered by prunes above depth l: 0.29
l=6 0.5*2^j   0.0 0.0 0.0 0.6 6.6 12.1 8.2  share of leaves covered by prunes above depth l: 0.87
l=6 1*2^j     0.0 0.0 0.0 1.4 7.8 8.2 5.5  share of leaves covered by prunes above depth l: 0.91
l=7 B=0       0.0 0.0 0.0 0.0 0.0 0.0 0.0 128.0  share of leaves covered by prunes above depth l: 0.00
l=7 B=10      0.0 0.0 0.0 0.0 9.1 10.6 5.4 1.8  share of leaves covered by prunes above depth l: 0.99
l=7 B=100     0.0 0.0 0.9 4.2 2.6 1.6 1.6 1.2  share of leaves covered by prunes above depth l: 0.99
l=7 B=1000    0.0 1.0 1.0 1.1 1.4 0.8 0.0 0.0  share of leaves covered by prunes above depth l: 1.00
l=7 0.25*2^j  0.0 0.0 0.0 0.0 0.0 13.9 0.0 72.5  share of leaves covered by prunes above depth l: 0.43
l=7 0.5*2^j   0.0 0.0 0.0 0.0 1.6 15.0 23.2 8.5  share of leaves covered by prunes above depth l: 0.93
l=7 1*2^j     0.0 0.0 0.0 1.5 5.5 10.8 6.9 3.2  share of leaves covered by prunes above depth l: 0.97
l=8 B=0       0.0 0.0 0.0 0.0 0.0 0.0 0.0 0.0 256.0  share of leaves covered by prunes above depth l: 0.00
l=8 B=10      0.0 0.0 0.0 0.0 0.0 16.5 23.8 12.6 3.8  share of leaves covered by prunes above depth l: 0.99
l=8 B=100     0.0 0.0 0.9 3.6 2.8 2.9 2.6 2.4 1.8  share of leaves covered by prunes above depth l: 0.99
l=8 B=1000    0.0 0.1 1.8 2.1 2.0 2.8 1.4 0.2 0.0  share of leaves covered by prunes above depth l: 1.00
l=8 0.25*2^j  0.0 0.0 0.0 0.0 0.0 1.0 24.6 0.8 148.0  share of leaves covered by prunes above depth l: 0.42
l=8 0.5*2^j   0.0 0.0 0.0 0.0 0.9 7.5 26.0 30.9 16.2  share of leaves covered by prunes above depth l: 0.94
l=8 1*2^j     0.0 0.0 0.0 2.0 3.1 7.8 12.5 10.5 9.0  share of leaves covered by prunes above depth l: 0.96

## engine E (linearisation at internal nodes)

E std l=4: nodes 31.0 leaves 9.0 (leaves/2^l 0.56) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 49 row XORs 15838; targets 8
E std l=5: nodes 63.0 leaves 16.0 (leaves/2^l 0.50) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 95 row XORs 36863; targets 8
E std l=6: nodes 127.0 leaves 33.0 (leaves/2^l 0.52) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 194 row XORs 164155; targets 8
E std l=7: nodes 255.0 leaves 63.0 (leaves/2^l 0.49) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 381 row XORs 333739; targets 8
E std l=8: nodes 511.0 leaves 120.0 (leaves/2^l 0.47) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 751 row XORs 1461193; targets 8
E rand l=4: nodes 31.0 leaves 9.0 (leaves/2^l 0.56) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 49 row XORs 15892; targets 8
E rand l=5: nodes 63.0 leaves 16.0 (leaves/2^l 0.50) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 95 row XORs 37410; targets 8
E rand l=6: nodes 127.0 leaves 33.0 (leaves/2^l 0.52) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 194 row XORs 164839; targets 8
E rand l=7: nodes 255.0 leaves 63.0 (leaves/2^l 0.49) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 381 row XORs 337952; targets 8
E rand l=8: nodes 511.0 leaves 120.0 (leaves/2^l 0.47) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 751 row XORs 1463538; targets 8
E alt l=4: nodes 31.0 leaves 9.0 (leaves/2^l 0.56) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 49 row XORs 16216; targets 8
E alt l=5: nodes 63.0 leaves 16.0 (leaves/2^l 0.50) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 95 row XORs 40710; targets 8
E alt l=6: nodes 127.0 leaves 33.0 (leaves/2^l 0.52) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 194 row XORs 173752; targets 8
E alt l=7: nodes 255.0 leaves 63.0 (leaves/2^l 0.49) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 381 row XORs 365235; targets 8
E alt l=8: nodes 511.0 leaves 120.0 (leaves/2^l 0.47) pruned at depth<l by fixpoint: 0, pruned on arrival at depth l: n/a; fix calls 751 row XORs 1555090; targets 8

## subcube refutation cost vs dimension j (fresh solver, cap 20000)

l=4: j=0: 16 (x16.50 of 2^j; cens 0/16); j=1: 43 (x21.44 of 2^j; cens 0/16); j=2: 206 (x51.62 of 2^j; cens 0/16); j=3: 1422 (x177.69 of 2^j; cens 0/16); j=4: 2531 (x158.16 of 2^j; cens 0/8)
   alpha(l=4) = 1.81 over j = [2, 3, 4]
l=5: j=0: 31 (x30.94 of 2^j; cens 0/16); j=1: 48 (x24.03 of 2^j; cens 0/16); j=2: 353 (x88.22 of 2^j; cens 0/16); j=3: 2021 (x252.64 of 2^j; cens 0/16); j=4: 14321 (x895.08 of 2^j; cens 8/16); j=5: 4253 (x132.91 of 2^j; cens 3/8)
   alpha(l=5) = nan over j = [2, 3]
l=6: j=0: 20 (x19.81 of 2^j; cens 0/16); j=1: 53 (x26.56 of 2^j; cens 0/16); j=2: 444 (x110.89 of 2^j; cens 0/16); j=3: 2163 (x270.34 of 2^j; cens 0/16); j=4: 8976 (x561.01 of 2^j; cens 3/16); j=5: 14020 (x438.13 of 2^j; cens 9/16); j=6: 9068 (x141.68 of 2^j; cens 4/8)
   alpha(l=6) = nan over j = [2, 3]
l=7: j=0: 10 (x9.50 of 2^j; cens 0/16); j=1: 64 (x31.91 of 2^j; cens 0/16); j=2: 609 (x152.25 of 2^j; cens 0/16); j=3: 1658 (x207.30 of 2^j; cens 0/16); j=4: 12281 (x767.59 of 2^j; cens 6/16); j=5: 17287 (x540.21 of 2^j; cens 11/16); j=6: 19566 (x305.71 of 2^j; cens 15/16); j=7: 20002 (x156.27 of 2^j; cens 8/8)
   alpha(l=7) = nan over j = [2, 3]
l=8: j=0: 14 (x14.50 of 2^j; cens 0/16); j=1: 170 (x84.81 of 2^j; cens 0/16); j=2: 863 (x215.84 of 2^j; cens 0/16); j=3: 5155 (x644.41 of 2^j; cens 0/16); j=4: 13056 (x816.03 of 2^j; cens 6/16); j=5: 18088 (x565.26 of 2^j; cens 14/16); j=6: 20002 (x312.53 of 2^j; cens 16/16); j=7: not run; j=8: not run
   alpha(l=8) = nan over j = [2, 3]

## decision inputs

largest l: 8 min rho over CMS std arms: 2.4775390625 arms: 10
arms with rho < 1 at largest l: []
alpha at largest l that has it (l=4): 1.8076400255721876
