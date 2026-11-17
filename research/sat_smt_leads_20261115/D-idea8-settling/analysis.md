decision (primary rule): INCONCLUSIVE

## l=4, N*=1000, last third from idx 666, 2+1 scan reference 16 leaf solves per target
| seed | signed base | non-rational x in V | INC targets run | analysed | decomposable targets / decomposable abscissae | non-dec targets / non-dec abscissae | solves INC,SPEC,FP,AA | errors |
|---|---|---|---|---|---|---|---|---|
| 0 | 8 | 11 | 1000 | 1000 | 15 / 44 (34.1%) | 985 / 3957 (24.9%) | 1000,985,985,329 | 0 |
| 1 | 18 | 6 | 1000 | 1000 | 15 / 457 (3.3%) | 985 / 3539 (27.8%) | 1000,985,985,330 | 0 |
| 2 | 12 | 9 | 1000 | 1000 | 15 / 146 (10.3%) | 985 / 3853 (25.6%) | 1000,985,985,330 | 0 |

| ratio | metric | last third GM (95%) | per seed | n per seed | first third GM (95%) |
|---|---|---|---|---|---|
| INC/SPEC | propagations | 0.879 (0.685..1.057) | 0.684, 1.058, 0.937 | 329,330,330 | 1.004 (0.860..1.142) |
| INC/SPEC | conflicts | 0.262 (0.151..0.441) | 0.151, 0.441, 0.269 | 329,330,330 |  |
| INC/SPEC | secs_solve | 1.839 (1.411..2.293) | 1.407, 2.296, 1.924 | 329,330,330 |  |
| INC/FP | propagations | 0.384 (0.324..0.440) | 0.323, 0.440, 0.397 | 329,330,330 | 0.446 (0.419..0.476) |
| INC/FP | conflicts | 0.226 (0.137..0.369) | 0.137, 0.369, 0.229 | 329,330,330 |  |
| INC/FP | secs_solve | 0.664 (0.569..0.773) | 0.568, 0.775, 0.667 | 329,330,330 |  |
| FP/SPEC | propagations | 2.289 (2.119..2.403) | 2.115, 2.402, 2.360 | 329,330,330 | 2.251 (2.053..2.396) |
| FP/SPEC | conflicts | 1.155 (1.103..1.197) | 1.101, 1.195, 1.171 | 329,330,330 |  |
| FP/SPEC | secs_solve | 2.768 (2.485..2.971) | 2.479, 2.963, 2.885 | 329,330,330 |  |
| AA/SPEC | propagations | 0.999 (0.985..1.017) | 1.013, 0.997, 0.987 | 329,330,330 | n/a |
| AA/SPEC | conflicts | 0.983 (0.963..1.001) | 0.992, 0.992, 0.965 | 329,330,330 |  |
| AA/SPEC | secs_solve | 1.031 (1.007..1.059) | 1.057, 1.026, 1.012 | 329,330,330 |  |

A/A sd of ln(AA/SPEC) propagations, last third: 0.162

| fit ln ratio = a - beta ln p | ratio at p=1 | beta (95%) | per-seed beta | p where ratio = 1.0 (95%) | p where ratio = 0.5 (95%) | resamples never reaching 1.0 / 0.5 |
|---|---|---|---|---|---|---|
| INC/SPEC propagations | 1.664 | 0.0992 (0.0569..0.1521) | 0.1541, 0.0564, 0.0863 | 169 (55.1..1.68e+03) | 1.82e+05 (5.03e+03..2.96e+08) | 0% / 0% |
| INC/SPEC conflicts | 0.473 | 0.0898 (0.0675..0.1074) | 0.0761, 0.0890, 0.1006 | already below (already below..already below) | already below (already below..170) | 0% / 0% |
| INC/FP propagations | 0.774 | 0.1088 (0.0585..0.1668) | 0.1689, 0.0572, 0.0998 | already below (already below..already below) | 55.3 (45.5..74.1) | 0% / 0% |
| INC/FP conflicts | 0.440 | 0.1010 (0.0803..0.1199) | 0.0931, 0.0891, 0.1176 | already below (already below..already below) | already below (already below..22.1) | 0% / 0% |

medians per target, last third, non-decomposable: INC propagations 147486; INC conflicts 64; SPEC propagations 156330; SPEC conflicts 253; FP propagations 375150; FP conflicts 288; AA propagations 155402; AA conflicts 246
summed solver seconds on the last-third set, per seed: INC [3.3, 28.26, 8.81], SPEC [2.31, 12.31, 4.61]

## l=5, N*=667, last third from idx 444, 2+1 scan reference 32 leaf solves per target
| seed | signed base | non-rational x in V | INC targets run | analysed | decomposable targets / decomposable abscissae | non-dec targets / non-dec abscissae | solves INC,SPEC,FP,AA | errors |
|---|---|---|---|---|---|---|---|---|
| 0 | 34 | 14 | 667 | 667 | 34 / 3205 (1.1%) | 633 / 62263 (1.0%) | 667,425,213,108 | 0 |
| 1 | 30 | 16 | 734 | 667 | 31 / 2197 (1.4%) | 636 / 63273 (1.0%) | 667,425,213,107 | 0 |
| 2 | 28 | 17 | 1000 | 667 | 35 / 1827 (1.9%) | 632 / 63644 (1.0%) | 667,422,212,106 | 0 |

| ratio | metric | last third GM (95%) | per seed | n per seed | first third GM (95%) |
|---|---|---|---|---|---|
| INC/SPEC | propagations | 0.951 (0.925..0.976) | 0.976, 0.952, 0.924 | 216,213,211 | 0.996 (0.976..1.013) |
| INC/SPEC | conflicts | 0.601 (0.569..0.637) | 0.637, 0.600, 0.568 | 216,213,211 |  |
| INC/SPEC | secs_solve | 2.292 (1.647..2.953) | 2.463, 2.976, 1.636 | 216,213,211 |  |
| INC/FP | propagations | 0.465 (0.451..0.474) | 0.473, 0.472, 0.450 | 108,107,106 | 0.488 (0.472..0.505) |
| INC/FP | conflicts | 0.503 (0.475..0.527) | 0.528, 0.507, 0.474 | 108,107,106 |  |
| INC/FP | secs_solve | 0.856 (0.620..1.102) | 0.913, 1.110, 0.616 | 108,107,106 |  |
| FP/SPEC | propagations | 2.036 (2.011..2.058) | 2.057, 2.011, 2.040 | 108,107,106 | 2.040 (2.000..2.075) |
| FP/SPEC | conflicts | 1.190 (1.174..1.205) | 1.201, 1.176, 1.193 | 108,107,106 |  |
| FP/SPEC | secs_solve | 2.649 (2.557..2.741) | 2.678, 2.584, 2.687 | 108,107,106 |  |
| AA/SPEC | propagations | 0.998 (0.992..1.005) | 0.998, 1.003, 0.994 | 108,107,106 | n/a |
| AA/SPEC | conflicts | 0.998 (0.988..1.008) | 0.997, 1.002, 0.995 | 108,107,106 |  |
| AA/SPEC | secs_solve | 1.029 (0.996..1.062) | 1.022, 1.026, 1.039 | 108,107,106 |  |

A/A sd of ln(AA/SPEC) propagations, last third: 0.041

| fit ln ratio = a - beta ln p | ratio at p=1 | beta (95%) | per-seed beta | p where ratio = 1.0 (95%) | p where ratio = 0.5 (95%) | resamples never reaching 1.0 / 0.5 |
|---|---|---|---|---|---|---|
| INC/SPEC propagations | 1.160 | 0.0323 (0.0219..0.0427) | 0.0263, 0.0378, 0.0329 | 98.6 (40.8..184) | 2.02e+11 (1.42e+09..4.14e+15) | 0% / 0% |
| INC/SPEC conflicts | 0.974 | 0.0773 (0.0670..0.0868) | 0.0676, 0.0814, 0.0830 | already below (already below..1.18) | 5.57e+03 (2.51e+03..1.76e+04) | 0% / 0% |
| INC/FP propagations | 0.574 | 0.0340 (0.0199..0.0507) | 0.0338, 0.0419, 0.0248 | already below (already below..already below) | 56.5 (6.75..140) | 0% / 0% |
| INC/FP conflicts | 0.819 | 0.0785 (0.0689..0.0871) | 0.0758, 0.0775, 0.0813 | already below (already below..already below) | 539 (261..1.06e+03) | 0% / 0% |

medians per target, last third, non-decomposable: INC propagations 2118060; INC conflicts 756; SPEC propagations 2223485; SPEC conflicts 1262; FP propagations 4468153; FP conflicts 1484; AA propagations 2233168; AA conflicts 1282
summed solver seconds on the last-third set, per seed: INC [214.8, 178.31, 74.67], SPEC [86.62, 60.69, 46.28]

## l-trend on matched positions [444, 667]
INC/SPEC propagations: l=4 0.886 (0.691..1.061); l=5 0.951 (0.925..0.975); ln GM(5) - ln GM(4) = +0.070 (-0.106..+0.313); beta l=4 (p <= N*(5)) 0.1189 (0.0675..0.1794), beta l=5 0.0323, difference -0.0866 (-0.1470..-0.0342)
INC/FP propagations: l=4 0.389 (0.328..0.443); l=5 0.465 (0.451..0.474); ln GM(5) - ln GM(4) = +0.178 (+0.050..+0.348); beta l=4 (p <= N*(5)) 0.1295 (0.0699..0.1942), beta l=5 0.0340, difference -0.0955 (-0.1598..-0.0359)
