# Cross-check list (Rust lane, sd_static) [pcc:unverified]

All values from `rust_check/target/release/sd_static` (commit-free, no external crates). Coefficients of g^{-1} are intrinsic (polynomial in x_i, x_i^N=1) and do not depend on the generator w [pcc:unverified]
(verified by `--selftest-w`); the w column is only the value used in the run, any generator of mu_{2^k} gives the same maxwt and sd.
Each row was re-run by the given-instance command below.
sd = delta + maxwt, delta = 2 (m = 2) or weight of g (m = 3, here 6).

| # | m | p | k | w | a | b | t | maxwt | delta | sd |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 2 | 12289 | 5 | 9094 | 823 | 1987 | 4566 | 10 | 2 | 12 |
| 2 | 2 | 1073750017 | 8 | 469253809 | 441877825 | 492201951 | 88475808 | 16 | 2 | 18 |
| 3 | 2 | 1073750017 | 12 | 948056387 | 207297763 | 252473285 | 917222864 | 24 | 2 | 26 |
| 4 | 3 | 1073750017 | 4 | 388242986 | 181111622 | 468591368 | 423048570 | 12 | 6 | 18 |
| 5 | 3 | 12289 | 3 | 7143 | 8776 | 3959 | 12122 | 9 | 6 | 15 |
| 6 | 2 | 17 | 3 | 9 | 10 | 9 | 6 | 5 | 2 | 7 |
| 7 (k=1) | 2 | 101 | 1 | 100 | 17 | 72 | 97 | 2 | 2 | 4 |

Row 6 is an exception (top coefficient of g^-1 is zero): sd = 2k+1 = 7 < 2k+2 = 8.
Row 7 (k=1): coefficients of g^{-1} in K[(Z/2)^2], index (e1,e2): (0,0)=91, (0,1)=81, (1,0)=81, (1,1)=95.
Reproduces the main session's values exactly (all four nonzero, maxwt = 2, sd = 4).

Re-run commands (verification output appended below by the same build):
```
sd_static --m 2 --k 5 --p 12289 --a 823 --b 1987 --t 4566 --w 9094
sd_static --m 2 --k 8 --p 1073750017 --a 441877825 --b 492201951 --t 88475808 --w 469253809
sd_static --m 2 --k 12 --p 1073750017 --a 207297763 --b 252473285 --t 917222864 --w 948056387
sd_static --m 3 --k 4 --p 1073750017 --a 181111622 --b 468591368 --t 423048570 --w 388242986
sd_static --m 3 --k 3 --p 12289 --a 8776 --b 3959 --t 12122 --w 7143
sd_static --m 2 --k 3 --p 17 --a 10 --b 9 --t 6 --w 9
sd_static --m 2 --k 1 --p 101 --a 17 --b 72 --t 97 --w 100
```

```
{"p":12289,"k":5,"m":2,"w":9094,"a":823,"b":1987,"t":4566,"unit":true,"maxwt":10,"delta":2,"gweight":2,"sd_static":12,"top_coeff_zero":false,
{"p":1073750017,"k":8,"m":2,"w":469253809,"a":441877825,"b":492201951,"t":88475808,"unit":true,"maxwt":16,"delta":2,"gweight":2,"sd_static":18,"top_coeff_zero":false,
{"p":1073750017,"k":12,"m":2,"w":948056387,"a":207297763,"b":252473285,"t":917222864,"unit":true,"maxwt":24,"delta":2,"gweight":null,"sd_static":26,"top_coeff_zero":false,
{"p":1073750017,"k":4,"m":3,"w":388242986,"a":181111622,"b":468591368,"t":423048570,"unit":true,"maxwt":12,"delta":6,"gweight":6,"sd_static":18,"top_coeff_zero":false,
{"p":12289,"k":3,"m":3,"w":7143,"a":8776,"b":3959,"t":12122,"unit":true,"maxwt":9,"delta":6,"gweight":6,"sd_static":15,"top_coeff_zero":false,
{"p":17,"k":3,"m":2,"w":9,"a":10,"b":9,"t":6,"unit":true,"maxwt":5,"delta":2,"gweight":2,"sd_static":7,"top_coeff_zero":true,
{"p":101,"k":1,"m":2,"w":100,"a":17,"b":72,"t":97,"unit":true,"maxwt":2,"delta":2,"gweight":2,"sd_static":4,"top_coeff_zero":false,
```
