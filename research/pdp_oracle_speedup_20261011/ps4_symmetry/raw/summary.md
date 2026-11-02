### Table 1. The bases and their Frobenius orbits

`m` = abscissae of weight `<= w` that lift (one point per abscissa; the base has `2m - 1` points, `x = 0` being `T`). `reps` = Frobenius orbits of the base. Pairs: ordered `m^2`, unordered `m(m+1)/2` (the PS-0/PS-2 loop), reduced `reps * m`.

| n | w | words wt<=w | m | base points (= panel's) | reps | orbit sizes cover m | short orbits (x, size) | pairs ordered | pairs unordered | pairs reduced | factor vs ordered (m/reps) | n | factor vs unordered | n/2 | measured / predicted (ordered) |
|--:|--:|--:|--:|--:|--:|:-:|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| 11 | 2 | 67 | 23 | 45 (lib and panel header match) | 3 | yes | (0x0, 1) | 529 | 276 | 69 | 7.667 | 11 | 4.000 | 5.5 | 0.697 |
| 11 | 3 | 232 | 100 | 199 (lib and panel header match) | 10 | yes | (0x0, 1) | 10000 | 5050 | 1000 | 10.000 | 11 | 5.050 | 5.5 | 0.909 |
| 23 | 2 | 277 | 162 | 323 (lib and panel header match) | 8 | yes | (0x0, 1) | 26244 | 13203 | 1296 | 20.250 | 23 | 10.188 | 11.5 | 0.880 |
| 23 | 3 | 2048 | 1197 | 2393 (lib and panel header match) | 53 | yes | (0x0, 1) | 1432809 | 717003 | 63441 | 22.585 | 23 | 11.302 | 11.5 | 0.982 |
| 29 | 2 | 436 | 175 | 349 (lib and panel header match) | 7 | yes | (0x0, 1) | 30625 | 15400 | 1225 | 25.000 | 29 | 12.571 | 14.5 | 0.862 |
| 29 | 3 | 4090 | 2031 | 4061 (lib and panel header match) | 71 | yes | (0x0, 1) | 4124961 | 2063496 | 144201 | 28.606 | 29 | 14.310 | 14.5 | 0.986 |
| 33 | 2 | 562 | 232 | 463 (lib and panel header match) | 8 | yes | (0x0, 1) | 53824 | 27028 | 1856 | 29.000 | 33 | 14.562 | 16.5 | 0.879 |
| 33 | 3 | 6018 | 3246 | 6491 (lib and panel header match) | 100 | yes | (0x0, 1) (0x142, 11) | 10536516 | 5269881 | 324600 | 32.460 | 33 | 16.235 | 16.5 | 0.984 |
| 35 | 2 | 631 | 211 | 421 (lib and panel header match) | 7 | yes | (0x0, 1) | 44521 | 22366 | 1477 | 30.143 | 35 | 15.143 | 17.5 | 0.861 |
| 35 | 3 | 7176 | 3151 | 6301 (lib and panel header match) | 91 | yes | (0x0, 1) | 9928801 | 4965976 | 286741 | 34.626 | 35 | 17.319 | 17.5 | 0.989 |
| 39 | 2 | 781 | 430 | 859 (lib and panel header match) | 12 | yes | (0x0, 1) | 184900 | 92665 | 5160 | 35.833 | 39 | 17.958 | 19.5 | 0.919 |
| 39 | 3 | 9920 | 4993 | 9985 (lib match, no panel run) | 129 | yes | (0x0, 1) | 24930049 | 12467521 | 644097 | 38.705 | 39 | 19.357 | 19.5 | 0.992 |
| 41 | 2 | 862 | 288 | 575 (lib and panel header match) | 8 | yes | (0x0, 1) | 82944 | 41616 | 2304 | 36.000 | 41 | 18.062 | 20.5 | 0.878 |
| 41 | 3 | 11522 | 5372 | 10743 (lib match, no panel run) | 132 | yes | (0x0, 1) | 28858384 | 14431878 | 709104 | 40.697 | 41 | 20.352 | 20.5 | 0.993 |

### Table 2. Candidates and counted operations per target, reduced against unreduced (sums over the targets of the setting)

Full-scan mode. `pairs` = `(P1, P2)` pairs visited; `thirds` = candidate thirds tested (4 per pair); `M` = counted field multiplications (`mul` + `batch_mul`, the library's accounting, inversion products included); `bits` = the bit-operation model of `model.rs` applied to the counted operations (a convention, not a measurement). `U` = unreduced, `R` = reduced (one representative per Frobenius orbit as first summand, every base point as second). Op factor / pair factor is the fraction of the candidate factor that the counted operations keep (the PS-4 success criterion asks for at least 0.8).

| n | w | targets | pairs U-unord | pairs U-ord | pairs R | pair factor vs unord | pair factor vs ord | M U-unord | M U-ord | M R | M factor vs unord | M factor vs ord | M/pair U-unord | M/pair R | bits factor vs unord | bits factor vs ord | op/pair factor (unord) | op/pair factor (ord) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 11 | 2 | 8 | 2208 | 4232 | 552 | 4.000 | 7.667 | 25047 | 45310 | 5927 | 4.226 | 7.645 | 11.344 | 10.737 | 4.211 | 7.647 | 1.056 | 0.997 |
| 11 | 3 | 8 | 40400 | 80000 | 8000 | 5.050 | 10.000 | 419390 | 819093 | 81570 | 5.141 | 10.042 | 10.381 | 10.196 | 5.135 | 10.038 | 1.018 | 1.004 |
| 23 | 2 | 8 | 105624 | 209952 | 10368 | 10.188 | 20.250 | 1084752 | 2128032 | 105088 | 10.322 | 20.250 | 10.270 | 10.136 | 10.318 | 20.250 | 1.013 | 1.000 |
| 23 | 3 | 8 | 5736024 | 11462472 | 507528 | 11.302 | 22.585 | 57570978 | 114835524 | 5084608 | 11.323 | 22.585 | 10.037 | 10.018 | 11.322 | 22.585 | 1.002 | 1.000 |
| 29 | 2 | 8 | 123200 | 245000 | 9800 | 12.571 | 25.000 | 1262800 | 2480800 | 99232 | 12.726 | 25.000 | 10.250 | 10.126 | 12.722 | 25.000 | 1.012 | 1.000 |
| 29 | 3 | 8 | 16507968 | 32999688 | 1153608 | 14.310 | 28.606 | 165437169 | 330354402 | 11548576 | 14.325 | 28.606 | 10.022 | 10.011 | 14.325 | 28.606 | 1.001 | 1.000 |
| 33 | 2 | 8 | 216224 | 430592 | 14848 | 14.562 | 29.000 | 2197504 | 4341184 | 149696 | 14.680 | 29.000 | 10.163 | 10.082 | 14.677 | 29.000 | 1.008 | 1.000 |
| 33 | 3 | 8 | 42159048 | 84292128 | 2596800 | 16.235 | 32.460 | 422083872 | 843414672 | 25983200 | 16.244 | 32.460 | 10.012 | 10.006 | 16.244 | 32.460 | 1.001 | 1.000 |
| 35 | 2 | 8 | 178928 | 356168 | 11816 | 15.143 | 30.143 | 1826416 | 3598816 | 119392 | 15.298 | 30.143 | 10.208 | 10.104 | 15.294 | 30.143 | 1.010 | 1.000 |
| 35 | 3 | 8 | 39727808 | 79430408 | 2293928 | 17.319 | 34.626 | 397832656 | 794858656 | 22955296 | 17.331 | 34.626 | 10.014 | 10.007 | 17.330 | 34.626 | 1.001 | 1.000 |
| 39 | 2 | 8 | 741320 | 1479200 | 41280 | 17.958 | 35.833 | 7499200 | 14878000 | 415200 | 18.062 | 35.833 | 10.116 | 10.058 | 18.060 | 35.833 | 1.006 | 1.000 |
| 39 | 3 | 8 | 99740168 | 199440392 | 5152776 | 19.357 | 38.705 | 998400280 | 1995402520 | 51553560 | 19.366 | 38.705 | 10.010 | 10.005 | 19.366 | 38.705 | 1.001 | 1.000 |
| 41 | 2 | 8 | 332928 | 663552 | 18432 | 18.062 | 36.000 | 3379968 | 6686208 | 185728 | 18.198 | 36.000 | 10.152 | 10.076 | 18.196 | 36.000 | 1.008 | 1.000 |
| 41 | 3 | 8 | 115455024 | 230867072 | 5672832 | 20.352 | 40.697 | 1155495712 | 2309616192 | 56751552 | 20.361 | 40.697 | 10.008 | 10.004 | 20.360 | 40.697 | 1.000 | 1.000 |

### Table 2b. Counted field operations per pair, by component

Full scans pooled over the targets of the setting. `mul` includes the products inside inversions (the library's accounting); `sqr`, `sqr_k` are squarings (free coordinate permutations here, counted so the operation vocabulary matches the control); `inv` is whole inversions (two per row for `U+-`, one for the row's Montgomery batch); `batch_mul` is `3` per inverted denominator; `add` is field additions; `popcnt` is the weight tests (4 per pair). Per candidate third, divide by 4.

| n | w | loop | pairs | mul/pair | sqr/pair | sqr_k/pair | inv/pair | batch_mul/pair | M/pair | add/pair | popcnt/pair |
|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 11 | 2 | U-unord | 2208 | 5.3451 | 4.6721 | 2.0181 | 0.25226 | 5.9986 | 11.3438 | 15.994 | 4.000 |
| 11 | 2 | U-ord | 4232 | 4.7079 | 4.3535 | 1.0624 | 0.13280 | 5.9986 | 10.7065 | 15.993 | 4.000 |
| 11 | 2 | R | 552 | 4.7428 | 4.3696 | 1.1159 | 0.13949 | 5.9946 | 10.7373 | 15.975 | 3.998 |
| 11 | 3 | U-unord | 40400 | 4.3915 | 4.1924 | 0.5895 | 0.07369 | 5.9895 | 10.3809 | 15.960 | 3.998 |
| 11 | 3 | U-ord | 80000 | 4.2523 | 4.1217 | 0.3807 | 0.04759 | 5.9863 | 10.2387 | 15.951 | 3.998 |
| 11 | 3 | R | 8000 | 4.2015 | 4.0992 | 0.3040 | 0.03800 | 5.9947 | 10.1962 | 15.977 | 3.998 |
| 23 | 2 | U-unord | 105624 | 4.2699 | 4.1350 | 0.6994 | 0.03681 | 6.0000 | 10.2699 | 16.000 | 4.000 |
| 23 | 2 | U-ord | 209952 | 4.1358 | 4.0679 | 0.3519 | 0.01852 | 6.0000 | 10.1358 | 16.000 | 4.000 |
| 23 | 2 | R | 10368 | 4.1358 | 4.0679 | 0.3519 | 0.01852 | 6.0000 | 10.1358 | 16.000 | 4.000 |
| 23 | 3 | U-unord | 5736024 | 4.0367 | 4.0184 | 0.0952 | 0.00501 | 6.0000 | 10.0367 | 16.000 | 4.000 |
| 23 | 3 | U-ord | 11462472 | 4.0184 | 4.0092 | 0.0477 | 0.00251 | 6.0000 | 10.0184 | 16.000 | 4.000 |
| 23 | 3 | R | 507528 | 4.0184 | 4.0092 | 0.0476 | 0.00251 | 6.0000 | 10.0184 | 16.000 | 4.000 |
| 29 | 2 | U-unord | 123200 | 4.2500 | 4.1250 | 0.8523 | 0.03409 | 6.0000 | 10.2500 | 16.000 | 4.000 |
| 29 | 2 | U-ord | 245000 | 4.1257 | 4.0629 | 0.4286 | 0.01714 | 6.0000 | 10.1257 | 16.000 | 4.000 |
| 29 | 2 | R | 9800 | 4.1257 | 4.0629 | 0.4286 | 0.01714 | 6.0000 | 10.1257 | 16.000 | 4.000 |
| 29 | 3 | U-unord | 16507968 | 4.0217 | 4.0108 | 0.0738 | 0.00295 | 6.0000 | 10.0217 | 16.000 | 4.000 |
| 29 | 3 | U-ord | 32999688 | 4.0108 | 4.0054 | 0.0369 | 0.00148 | 6.0000 | 10.0108 | 16.000 | 4.000 |
| 29 | 3 | R | 1153608 | 4.0108 | 4.0054 | 0.0369 | 0.00148 | 6.0000 | 10.0108 | 16.000 | 4.000 |
| 33 | 2 | U-unord | 216224 | 4.1631 | 4.0429 | 0.7983 | 0.02575 | 6.0000 | 10.1631 | 16.000 | 4.000 |
| 33 | 2 | U-ord | 430592 | 4.0819 | 4.0216 | 0.4009 | 0.01293 | 6.0000 | 10.0819 | 16.000 | 4.000 |
| 33 | 2 | R | 14848 | 4.0819 | 4.0216 | 0.4009 | 0.01293 | 6.0000 | 10.0819 | 16.000 | 4.000 |
| 33 | 3 | U-unord | 42159048 | 4.0117 | 4.0031 | 0.0573 | 0.00185 | 6.0000 | 10.0117 | 16.000 | 4.000 |
| 33 | 3 | U-ord | 84292128 | 4.0059 | 4.0015 | 0.0287 | 0.00092 | 6.0000 | 10.0059 | 16.000 | 4.000 |
| 33 | 3 | R | 2596800 | 4.0059 | 4.0015 | 0.0287 | 0.00092 | 6.0000 | 10.0059 | 16.000 | 4.000 |
| 35 | 2 | U-unord | 178928 | 4.2075 | 4.0755 | 0.9057 | 0.02830 | 6.0000 | 10.2075 | 16.000 | 4.000 |
| 35 | 2 | U-ord | 356168 | 4.1043 | 4.0379 | 0.4550 | 0.01422 | 6.0000 | 10.1043 | 16.000 | 4.000 |
| 35 | 2 | R | 11816 | 4.1043 | 4.0379 | 0.4550 | 0.01422 | 6.0000 | 10.1043 | 16.000 | 4.000 |
| 35 | 3 | U-unord | 39727808 | 4.0140 | 4.0051 | 0.0609 | 0.00190 | 6.0000 | 10.0140 | 16.000 | 4.000 |
| 35 | 3 | U-ord | 79430408 | 4.0070 | 4.0025 | 0.0305 | 0.00095 | 6.0000 | 10.0070 | 16.000 | 4.000 |
| 35 | 3 | R | 2293928 | 4.0070 | 4.0025 | 0.0305 | 0.00095 | 6.0000 | 10.0070 | 16.000 | 4.000 |
| 39 | 2 | U-unord | 741320 | 4.1160 | 4.0510 | 0.4872 | 0.01392 | 6.0000 | 10.1160 | 16.000 | 4.000 |
| 39 | 2 | U-ord | 1479200 | 4.0581 | 4.0256 | 0.2442 | 0.00698 | 6.0000 | 10.0581 | 16.000 | 4.000 |
| 39 | 2 | R | 41280 | 4.0581 | 4.0256 | 0.2442 | 0.00698 | 6.0000 | 10.0581 | 16.000 | 4.000 |
| 39 | 3 | U-unord | 99740168 | 4.0100 | 4.0044 | 0.0421 | 0.00120 | 6.0000 | 10.0100 | 16.000 | 4.000 |
| 39 | 3 | U-ord | 199440392 | 4.0050 | 4.0022 | 0.0210 | 0.00060 | 6.0000 | 10.0050 | 16.000 | 4.000 |
| 39 | 3 | R | 5152776 | 4.0050 | 4.0022 | 0.0210 | 0.00060 | 6.0000 | 10.0050 | 16.000 | 4.000 |
| 41 | 2 | U-unord | 332928 | 4.1522 | 4.0554 | 0.7889 | 0.02076 | 6.0000 | 10.1522 | 16.000 | 4.000 |
| 41 | 2 | U-ord | 663552 | 4.0764 | 4.0278 | 0.3958 | 0.01042 | 6.0000 | 10.0764 | 16.000 | 4.000 |
| 41 | 2 | R | 18432 | 4.0764 | 4.0278 | 0.3958 | 0.01042 | 6.0000 | 10.0764 | 16.000 | 4.000 |
| 41 | 3 | U-unord | 115455024 | 4.0082 | 4.0030 | 0.0424 | 0.00112 | 6.0000 | 10.0082 | 16.000 | 4.000 |
| 41 | 3 | U-ord | 230867072 | 4.0041 | 4.0015 | 0.0212 | 0.00056 | 6.0000 | 10.0041 | 16.000 | 4.000 |
| 41 | 3 | R | 5672832 | 4.0041 | 4.0015 | 0.0212 | 0.00056 | 6.0000 | 10.0041 | 16.000 | 4.000 |

### Table 3. Verdict-only mode, split by what the oracle says

*Decomposable targets*: pairs and `M` until the first witness, mean per target, for the unreduced unordered loop and the reduced loop; `R says no` counts decomposable targets on which the reduced loop found nothing in its full scan. *Refuting targets*: the full scan of both loops (the reduced loop's `no` refutes only the decompositions that contain a representative; it is not a refutation of `R`).

| n | w | decomposable | mean pairs to witness U-unord | mean pairs to witness R | mean M U-unord | mean M R | M R / M U-unord | R says no | refuting | mean M U-unord (full) | mean M R (full) | M R / M U-unord |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 11 | 2 | 5 | 17 | 17 | 281 | 285 | 1.014 | 0 | 3 | 3128 | 738 | 0.236 |
| 11 | 3 | 8 | 6 | 6 | 638 | 638 | 1.000 | 0 | 0 | 0 | 0 | n/a |
| 23 | 2 | 6 | 3151 | 968 | 32439 | 10002 | 0.308 | 3 | 2 | 135594 | 13136 | 0.097 |
| 23 | 3 | 8 | 1925 | 1927 | 23898 | 23917 | 1.001 | 0 | 0 | 0 | 0 | n/a |
| 29 | 2 | 2 | 6454 | 1225 | 65868 | 12404 | 0.188 | 2 | 6 | 157850 | 12404 | 0.079 |
| 29 | 3 | 8 | 46872 | 55014 | 478150 | 558828 | 1.169 | 1 | 0 | 0 | 0 | n/a |
| 33 | 2 | 2 | 13789 | 1856 | 139894 | 18712 | 0.134 | 2 | 6 | 274688 | 18712 | 0.068 |
| 33 | 3 | 8 | 747884 | 274865 | 7494073 | 2754097 | 0.368 | 6 | 0 | 0 | 0 | n/a |
| 35 | 2 | 2 | 10319 | 1477 | 105033 | 14924 | 0.142 | 2 | 6 | 228302 | 14924 | 0.065 |
| 35 | 3 | 6 | 1547000 | 198308 | 15493190 | 1988849 | 0.128 | 4 | 2 | 49729082 | 2869412 | 0.058 |
| 39 | 2 | 2 | 39254 | 3976 | 396643 | 40654 | 0.102 | 1 | 6 | 937400 | 51900 | 0.055 |
| 39 | 3 | 5 | 4591098 | 641152 | 45958088 | 6420423 | 0.140 | 4 | 3 | 124800035 | 6444195 | 0.052 |
| 41 | 2 | 2 | 1360 | 1650 | 14970 | 17102 | 1.142 | 1 | 6 | 422496 | 23216 | 0.055 |
| 41 | 3 | 2 | 5540936 | 709104 | 55456000 | 7093944 | 0.128 | 2 | 6 | 144436964 | 7093944 | 0.049 |

### Table 4. Verdicts and solution sets

`dec` = targets the pair-table oracle calls decomposable. Panel columns: the panel's own records (`examples/hamming_nb_sat_panel`) where they exist (x_R equal / verdict equal / `ref_count` equal, out of the targets compared). `U = oracle`: kernel solution set equals the oracle's, as a set of sorted abscissa triples. `R subset`, `R = predicted`: the reduced loop's set is contained in the oracle's, and equals the oracle's triples that have a representative member. `verdict kept` = the reduced loop's verdict equals the unreduced one (fixed target). `kept sols` = distinct solutions of the reduced loop over those of the oracle, pooled over the targets (in brackets, the fraction expected if the three members of a solution were independent draws from the base, `1 - (1 - reps/m)^3`; a derived expectation, not a measurement).`closure = U`: the union over the target's Frobenius conjugates of the reduced loop's solutions, mapped back, equals the unreduced set.

| n | w | targets | dec | panel x_R | panel verdict | panel ref_count | U-unord = oracle | U-ord = oracle | R subset | R = predicted | verdict kept (fixed target) | uniform lost | planted lost | kept sols | closure = U |
|--:|--:|--:|--:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|--:|:-:|
| 11 | 2 | 8 | 5 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 0/6 | 0/2 | 21/66 = 0.318 (independence: 0.342) | 8/8 |
| 11 | 3 | 8 | 8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 0/6 | 0/2 | 1240/4797 = 0.258 (independence: 0.271) | 8/8 |
| 23 | 2 | 8 | 6 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 5/8 | 2/6 | 1/2 | 3/19 = 0.158 (independence: 0.141) | 8/8 |
| 23 | 3 | 8 | 8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 0/6 | 0/2 | 250/2034 = 0.123 (independence: 0.127) | 8/8 |
| 29 | 2 | 8 | 2 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 6/8 | 0/6 | 2/2 | 0/2 = 0.000 (independence: 0.115) | 8/8 |
| 29 | 3 | 8 | 8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 7/8 | 1/6 | 0/2 | 21/165 = 0.127 (independence: 0.101) | 8/8 |
| 33 | 2 | 8 | 2 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 6/8 | 0/6 | 2/2 | 0/2 = 0.000 (independence: 0.100) | 8/8 |
| 33 | 3 | 8 | 8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 2/8 | 5/6 | 1/2 | 5/76 = 0.066 (independence: 0.090) | 8/8 |
| 35 | 2 | 8 | 2 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 6/8 | 0/6 | 2/2 | 0/2 = 0.000 (independence: 0.096) | 8/8 |
| 35 | 3 | 8 | 6 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 4/8 | 3/6 | 1/2 | 2/15 = 0.133 (independence: 0.084) | 8/8 |
| 39 | 2 | 8 | 2 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 7/8 | 0/6 | 1/2 | 1/2 = 0.500 (independence: 0.081) | 8/8 |
| 39 | 3 | 8 | 5 | n/a | n/a | n/a | 8/8 | 8/8 | 8/8 | 8/8 | 4/8 | 2/6 | 2/2 | 1/9 = 0.111 (independence: 0.076) | 8/8 |
| 41 | 2 | 8 | 2 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 7/8 | 0/6 | 1/2 | 1/2 = 0.500 (independence: 0.081) | 8/8 |
| 41 | 3 | 8 | 2 | n/a | n/a | n/a | 8/8 | 8/8 | 8/8 | 8/8 | 6/8 | 0/6 | 2/2 | 0/2 = 0.000 (independence: 0.072) | 8/8 |

**Totals over the 14 settings (112 targets).** Oracle: 66 decomposable, 46 not. Panel compared on 96 targets: x_R equal 96, verdict equal 96, `ref_count` equal 96. Unreduced kernel = oracle: unordered 112/112, ordered 112/112. Reduced loop subset of oracle 112/112, equal to the predicted set 112/112. Reduced verdict equals the unreduced one on 84/112 targets; on the 66 decomposable targets the reduced loop alone answered no on 28. Closure over conjugates equals the unreduced set on 112/112.


### Table 5. What the orbit-complete reading costs, and the yield per candidate

`closure pairs` = pairs the reduced loop visits over all conjugates of the target (what the closure check ran); compared with the ordered unreduced loop on the one target. `hits/third` = raw hits per candidate third, pooled over the setting's targets (the stage yield per candidate).

| n | w | closure pairs / U-ord pairs | closure M / U-ord M | closure M / U-unord M | hits U-unord | thirds U-unord | hits R | thirds R | hits per third U-unord | hits per third R | R / U-unord |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 11 | 2 | 1.435 | 1.434 | 2.595 | 202 | 8831 | 54 | 2207 | 2.287e-2 | 2.447e-2 | 1.070 |
| 11 | 3 | 1.100 | 1.099 | 2.147 | 14505 | 161516 | 2872 | 31986 | 8.981e-2 | 8.979e-2 | 1.000 |
| 23 | 2 | 1.136 | 1.136 | 2.228 | 57 | 422496 | 6 | 41472 | 1.349e-4 | 1.447e-4 | 1.072 |
| 23 | 3 | 1.018 | 1.018 | 2.031 | 6105 | 22944094 | 536 | 2030112 | 2.661e-4 | 2.640e-4 | 0.992 |
| 29 | 2 | 1.160 | 1.160 | 2.279 | 6 | 492800 | 0 | 39200 | 1.218e-5 | 0.000e0 | 0.000 |
| 29 | 3 | 1.014 | 1.014 | 2.024 | 496 | 66031871 | 44 | 4614432 | 7.512e-6 | 9.535e-6 | 1.269 |
| 33 | 2 | 1.138 | 1.138 | 2.248 | 6 | 864896 | 0 | 59392 | 6.937e-6 | 0.000e0 | 0.000 |
| 33 | 3 | 1.017 | 1.017 | 2.031 | 228 | 168636192 | 12 | 10387200 | 1.352e-6 | 1.155e-6 | 0.854 |
| 35 | 2 | 1.161 | 1.161 | 2.288 | 6 | 715712 | 0 | 47264 | 8.383e-6 | 0.000e0 | 0.000 |
| 35 | 3 | 1.011 | 1.011 | 2.020 | 45 | 158911232 | 4 | 9175712 | 2.832e-7 | 4.359e-7 | 1.539 |
| 39 | 2 | 1.088 | 1.088 | 2.159 | 6 | 2965280 | 2 | 165120 | 2.023e-6 | 1.211e-5 | 5.986 |
| 39 | 3 | 1.008 | 1.008 | 2.014 | 27 | 398960672 | 2 | 20611104 | 6.768e-8 | 9.704e-8 | 1.434 |
| 41 | 2 | 1.139 | 1.139 | 2.253 | 6 | 1331712 | 2 | 73728 | 4.505e-6 | 2.713e-5 | 6.021 |
| 41 | 3 | 1.007 | 1.007 | 2.014 | 6 | 461820096 | 0 | 22691328 | 1.299e-8 | 0.000e0 | 0.000 |

### Table 6. Exact-path pairs

Pairs that left the batched formula (a zero denominator: `U = ±P2`, or `P1 = ±R`) and were done by the exact path, in the unordered full scans.

| n | w | exact pairs | pairs | targets with `wt(x_R) <= w` or `wt(1/x_R) <= w` |
|--:|--:|--:|--:|--:|
| 11 | 2 | 1 | 2208 | 0 |
| 11 | 3 | 112 | 40400 | 2 |
| 23 | 2 | 0 | 105624 | 0 |
| 23 | 3 | 2 | 5736024 | 0 |
| 29 | 2 | 0 | 123200 | 0 |
| 29 | 3 | 1 | 16507968 | 0 |
| 33 | 2 | 0 | 216224 | 0 |
| 33 | 3 | 0 | 42159048 | 0 |
| 35 | 2 | 0 | 178928 | 0 |
| 35 | 3 | 0 | 39727808 | 0 |
| 39 | 2 | 0 | 741320 | 0 |
| 39 | 3 | 0 | 99740168 | 0 |
| 41 | 2 | 0 | 332928 | 0 |
| 41 | 3 | 0 | 115455024 | 0 |

### Table 7. The subspace-leaf control (PS-2 group-law kernel on the PS-0 corpus, library counters)

Per pair, full scan of `i1 <= i2`, the first 6 uniform and 2 planted corpus targets of each rung. Polynomial basis, `a = b = 1`; leaf `x3 < 2^l`.

| l | n | targets | base m | pairs | mul/pair | sqr/pair | sqr_k/pair | inv/pair | batch_mul/pair | M/pair | model bits/pair | verdicts agree with corpus |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 8 | 24 | 8 | 128 | 66048 | 4.5271 | 4.2171 | 0.8837 | 0.04651 | 6.0000 | 10.5271 | 13254.1 | 8/8 |
| 9 | 27 | 8 | 253 | 257048 | 4.2441 | 4.0866 | 0.5433 | 0.02362 | 6.0000 | 10.2441 | 16171.3 | 8/8 |
| 10 | 30 | 8 | 464 | 863040 | 4.1462 | 4.0602 | 0.3226 | 0.01290 | 6.0000 | 10.1462 | 18874.8 | 8/8 |
| 11 | 33 | 8 | 1007 | 4060224 | 4.0556 | 4.0099 | 0.1845 | 0.00595 | 6.0000 | 10.0556 | 22591.0 | 8/8 |
| 12 | 36 | 8 | 1992 | 15880224 | 4.0341 | 4.0110 | 0.0963 | 0.00301 | 6.0000 | 10.0341 | 26756.8 | 8/8 |

### Table 8. Leaf cost per pair: weight base in the ONB against the subspace control

Weight side: the unordered full scans of Table 2, pooled over the setting's targets, per pair. Control: Table 7. `x` columns are weight / control. `n-near` = the control rung with `3l` nearest `n` (`dn` = its `n` minus this `n`); `m-near` = the rung whose base size is nearest in ratio. `bits x` prices both sides with the model of `model.rs` at each side's own field degree; `bits x (control at n)` scales the control's modelled bits by `(n / n_l)^2`, the leading term of the schoolbook model, to put it at this setting's degree. The stop condition of PS-4 is a ratio above 2. The model is a convention, not a measurement; the wall note is Table 9.

| n | w | M/pair (ONB) | popcount/pair | model bits/pair (ONB) | n-near l | dn | M/pair x | bits x | bits x (control at n) | m-near l | M/pair x | bits x | range of bits x over l=8..12 |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 11 | 2 | 11.3438 | 3.9995 | 4132.1 | 8 | +13 | 1.078 | 0.312 | 1.484 | 8 | 1.078 | 0.312 | 0.154 to 0.312 |
| 11 | 3 | 10.3809 | 3.9979 | 3803.4 | 8 | +13 | 0.986 | 0.287 | 1.366 | 8 | 0.986 | 0.287 | 0.142 to 0.287 |
| 23 | 2 | 10.2699 | 4.0000 | 16378.0 | 8 | +1 | 0.976 | 1.236 | 1.345 | 8 | 0.976 | 1.236 | 0.612 to 1.236 |
| 23 | 3 | 10.0367 | 4.0000 | 16018.6 | 8 | +1 | 0.953 | 1.209 | 1.316 | 11 | 0.998 | 0.709 | 0.599 to 1.209 |
| 29 | 2 | 10.2500 | 4.0000 | 25962.2 | 10 | +1 | 1.010 | 1.375 | 1.472 | 8 | 0.974 | 1.959 | 0.970 to 1.959 |
| 29 | 3 | 10.0217 | 4.0000 | 25399.4 | 10 | +1 | 0.988 | 1.346 | 1.440 | 12 | 0.999 | 0.949 | 0.949 to 1.916 |
| 33 | 2 | 10.1631 | 4.0000 | 33324.1 | 11 | +0 | 1.011 | 1.475 | 1.475 | 9 | 0.992 | 2.061 | 1.245 to 2.514 |
| 33 | 3 | 10.0117 | 4.0000 | 32839.5 | 11 | +0 | 0.996 | 1.454 | 1.454 | 12 | 0.998 | 1.227 | 1.227 to 2.478 |
| 35 | 2 | 10.2075 | 4.0000 | 37638.2 | 12 | +1 | 1.017 | 1.407 | 1.488 | 9 | 0.996 | 2.327 | 1.407 to 2.840 |
| 35 | 3 | 10.0140 | 4.0000 | 36940.3 | 12 | +1 | 0.998 | 1.381 | 1.461 | 12 | 0.998 | 1.381 | 1.381 to 2.787 |
| 39 | 2 | 10.1160 | 4.0000 | 46306.3 | 12 | -3 | 1.008 | 1.731 | 1.475 | 10 | 0.997 | 2.453 | 1.731 to 3.494 |
| 39 | 3 | 10.0100 | 4.0000 | 45830.9 | 12 | -3 | 0.998 | 1.713 | 1.459 | 12 | 0.998 | 1.713 | 1.713 to 3.458 |
| 41 | 2 | 10.1522 | 4.0000 | 51349.3 | 12 | -5 | 1.012 | 1.919 | 1.480 | 9 | 0.991 | 3.175 | 1.919 to 3.874 |
| 41 | 3 | 10.0082 | 4.0000 | 50634.6 | 12 | -5 | 0.997 | 1.892 | 1.459 | 12 | 0.997 | 1.892 | 1.892 to 3.820 |

### Table 9. Unisolated wall time per pair (a labelled note, not a result)

`ms_unisolated` fields of the unordered full scans and of the control, on a shared container with no pinning. The weight kernel is portable Rust over a software cyclic convolution; the control uses `pclmulqdq`. Different code paths, so this is not a like-for-like measure of the two leaves. The run was resumed on a second host class after a container restart; each record is attributed to the host whose manifest (`manifest.txt`, `host_*.txt`) is the latest one not later than the record's stage marker (`raw/done/`), and rows of different hosts are never pooled.

| setting | host (cpu) | ns/pair (weight, ONB) |
|:--|:--|--:|
| n=11 w=2 | Intel(R) Xeon(R) Processor @ 2.80GHz | 507.2 |
| n=11 w=3 | Intel(R) Xeon(R) Processor @ 2.80GHz | 479.6 |
| n=23 w=2 | Intel(R) Xeon(R) Processor @ 2.80GHz | 701.7 |
| n=23 w=3 | Intel(R) Xeon(R) Processor @ 2.80GHz | 718.1 |
| n=29 w=2 | Intel(R) Xeon(R) Processor @ 2.80GHz | 851.0 |
| n=29 w=3 | Intel(R) Xeon(R) Processor @ 2.80GHz | 865.6 |
| n=33 w=2 | Intel(R) Xeon(R) Processor @ 2.80GHz | 944.3 |
| n=33 w=3 | Intel(R) Xeon(R) Processor @ 2.80GHz | 970.3 |
| n=35 w=2 | Intel(R) Xeon(R) Processor @ 2.80GHz | 1198.2 |
| n=35 w=3 | Intel(R) Xeon(R) Processor @ 2.10GHz | 728.7 |
| n=39 w=2 | Intel(R) Xeon(R) Processor @ 2.80GHz | 1110.5 |
| n=39 w=3 | Intel(R) Xeon(R) Processor @ 2.80GHz | 1126.4 |
| n=41 w=2 | Intel(R) Xeon(R) Processor @ 2.80GHz | 1233.1 |
| n=41 w=3 | Intel(R) Xeon(R) Processor @ 2.10GHz | 910.8 |
| control l=8 (n=24) | Intel(R) Xeon(R) Processor @ 2.10GHz | 39.4 |
| control l=9 (n=27) | Intel(R) Xeon(R) Processor @ 2.10GHz | 35.8 |
| control l=10 (n=30) | Intel(R) Xeon(R) Processor @ 2.10GHz | 25.6 |
| control l=11 (n=33) | Intel(R) Xeon(R) Processor @ 2.10GHz | 30.5 |
| control l=12 (n=36) | Intel(R) Xeon(R) Processor @ 2.10GHz | 31.2 |

### Table 10. Where the 2-torsion symmetry applies: how much of each weight base `T` preserves

`x` frame: `T` is `x -> 1/x`. `u` frame (`u = 1/(x+1)`): `T` is `u -> u + 1`, the complement in the ONB (`1` is the all-ones word), of weight `n - wt(u)`. Counts are over abscissae that lift.

| n | w | x-frame: base | x-frame: image in base | u-frame: base | u-frame: image in base | `u -> u+1` verified | n - w > w |
|--:|--:|--:|--:|--:|--:|:-:|:-:|
| 11 | 2 | 22 | 0 | 44 | 0 | true | true |
| 11 | 3 | 99 | 0 | 132 | 0 | true | true |
| 23 | 2 | 161 | 0 | 161 | 0 | true | true |
| 23 | 3 | 1196 | 0 | 1012 | 0 | true | true |
| 29 | 2 | 174 | 0 | 232 | 0 | true | true |
| 29 | 3 | 2030 | 0 | 1972 | 0 | true | true |
| 33 | 2 | 231 | 0 | 363 | 0 | true | true |
| 33 | 3 | 3245 | 0 | 2816 | 0 | true | true |
| 35 | 2 | 210 | 0 | 280 | 0 | true | true |
| 35 | 3 | 3150 | 0 | 3430 | 0 | true | true |
| 39 | 2 | 429 | 0 | 390 | 0 | true | true |
| 39 | 3 | 4992 | 0 | 4927 | 0 | true | true |
| 41 | 2 | 287 | 0 | 328 | 0 | true | true |
| 41 | 3 | 5371 | 0 | 5330 | 0 | true | true |

### Table 11. The T reduction on the T-closed base `B^+- = {wt(u) <= w or wt(u) >= n - w}`

Same panel targets, ordered loops on `B^+-` with the leaf a table probe (membership in `B^+-` by `x`; the `u`-weight test would need an inversion per candidate). `T-rep` = first summand one per class `{x, 1/x}` (plus `T` alone), second summand every base point. `closure` = the `T-rep` set closed under translating two non-`T` members by `T`.

| n | w | targets | m (B^+-) | classes | pairs U-ord | pairs T-rep | pair factor | M factor | U = oracle | T-rep subset | closure = oracle | verdict kept |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|:-:|:-:|:-:|
| 11 | 2 | 8 | 89 | 45 | 63368 | 32040 | 1.9778 | 1.9711 | 8/8 | 8/8 | 8/8 | 8/8 |
| 11 | 3 | 8 | 265 | 133 | 561800 | 281960 | 1.9925 | 1.9901 | 8/8 | 8/8 | 8/8 | 8/8 |
| 23 | 2 | 8 | 323 | 162 | 834632 | 418608 | 1.9938 | 1.9938 | 8/8 | 8/8 | 8/8 | 8/8 |
| 23 | 3 | 8 | 2025 | 1013 | 32805000 | 16410600 | 1.9990 | 1.9990 | 8/8 | 8/8 | 8/8 | 8/8 |
| 29 | 2 | 8 | 465 | 233 | 1729800 | 866760 | 1.9957 | 1.9957 | 8/8 | 8/8 | 8/8 | 8/8 |
| 29 | 3 | 8 | 3945 | 1973 | 124504200 | 62267880 | 1.9995 | 1.9995 | 8/8 | 8/8 | 8/8 | 8/8 |
| 33 | 2 | 8 | 727 | 364 | 4228232 | 2117024 | 1.9973 | 1.9973 | 8/8 | 8/8 | 8/8 | 8/8 |
| 33 | 3 | 8 | 5633 | 2817 | 253845512 | 126945288 | 1.9996 | 1.9996 | 8/8 | 8/8 | 8/8 | 8/8 |
| 35 | 2 | 8 | 561 | 281 | 2517768 | 1261128 | 1.9964 | 1.9964 | 8/8 | 8/8 | 8/8 | 8/8 |
| 35 | 3 | 2 | 6861 | 3431 | 94146642 | 47080182 | 1.9997 | 1.9997 | 2/2 | 2/2 | 2/2 | 2/2 |
| 39 | 2 | 8 | 781 | 391 | 4879688 | 2442968 | 1.9974 | 1.9974 | 8/8 | 8/8 | 8/8 | 8/8 |
| 39 | 3 | 2 | 9855 | 4928 | 194242050 | 97130880 | 1.9998 | 1.9998 | 2/2 | 2/2 | 2/2 | 2/2 |
| 41 | 2 | 8 | 657 | 329 | 3453192 | 1729224 | 1.9970 | 1.9970 | 8/8 | 8/8 | 8/8 | 8/8 |
| 41 | 3 | 2 | 10661 | 5331 | 227313842 | 113667582 | 1.9998 | 1.9998 | 2/2 | 2/2 | 2/2 | 2/2 |

### Table 12. The normal-basis field against the library's `Type2Onb`

| n | exhaustive | products compared | product mismatches | squarings | mismatches | inversions | mismatches | inversion products counted / formula | lifts compared | lift mismatches |
|--:|:-:|--:|--:|--:|--:|--:|--:|:-:|--:|--:|
| 3 | true | 64 | 0 | 8 | 0 | 7 | 0 | 1/1 | 8 | 0 |
| 5 | true | 1024 | 0 | 32 | 0 | 31 | 0 | 2/2 | 32 | 0 |
| 9 | true | 262144 | 0 | 512 | 0 | 511 | 0 | 3/3 | 512 | 0 |
| 11 | true | 4194304 | 0 | 2048 | 0 | 2047 | 0 | 4/4 | 2048 | 0 |
| 23 | false | 200000 | 0 | 20000 | 0 | 20000 | 0 | 6/6 | 20000 | 0 |
| 29 | false | 200000 | 0 | 20000 | 0 | 20000 | 0 | 6/6 | 20000 | 0 |
| 33 | false | 200000 | 0 | 20000 | 0 | 20000 | 0 | 5/5 | 20000 | 0 |
| 35 | false | 200000 | 0 | 20000 | 0 | 20000 | 0 | 6/6 | 20000 | 0 |
| 39 | false | 200000 | 0 | 20000 | 0 | 20000 | 0 | 7/7 | 20000 | 0 |
| 41 | false | 200000 | 0 | 20000 | 0 | 20000 | 0 | 6/6 | 20000 | 0 |

### Table 13. The curve `K_0: y^2 + xy = x^3 + 1` (a = 0, b = 1): constants, point count, translation by `T`, `S3`

| n | `T` on curve | `T+T = O` | points | points expected (Koblitz recurrence) | `T`-translation points tested (exhaustive?) | `x(P+T) = 1/x` mismatches (P, -P) | group law vs library: pairs / mismatches | `S3` zero on sums: sums / nonzero | `S3` on a random third: nonzero |
|--:|:-:|:-:|--:|--:|:--|:-:|:-:|:-:|--:|
| 3 | true | true | 4 | 4 | 1 (true) | 0 / 0 | 12 / 0 | 0 / 0 | 0 |
| 5 | true | true | 44 | 44 | 21 (true) | 0 / 0 | 1892 / 0 | 0 / 0 | 0 |
| 9 | true | true | 508 | 508 | 253 (true) | 0 / 0 | 257556 / 0 | 0 / 0 | 0 |
| 11 | true | true | 2116 | 2116 | 1057 (true) | 0 / 0 | 200000 / 0 | 299606 / 0 | 299373 |
| 23 | true | true | 8383412 | 8383412 | 4191705 (true) | 0 / 0 | 200000 / 0 | 299924 / 0 | 299924 |
| 29 | true | true | n/a | n/a | 100489 (false) | 0 / 0 | 200000 / 0 | 300210 / 0 | 300210 |
| 33 | true | true | n/a | n/a | 99837 (false) | 0 / 0 | 200000 / 0 | 299859 / 0 | 299859 |
| 35 | true | true | n/a | n/a | 100357 (false) | 0 / 0 | 200000 / 0 | 299938 / 0 | 299938 |
| 39 | true | true | n/a | n/a | 100573 (false) | 0 / 0 | 200000 / 0 | 300244 / 0 | 300244 |
| 41 | true | true | n/a | n/a | 99962 (false) | 0 / 0 | 200000 / 0 | 300069 / 0 | 300069 |

### Table 14. The exactness theorem of `ps1_algebra/README.md` section 3, re-checked for `K_0`

`V` = the set the summands are drawn from; `S4` roots in `V^3` by the resultant of `S3(x1,x2,t)` and `S3(x3,x_R,t)`; decompositions = sorted triples of the exact group law. A *degenerate* target has `x_R in V` or `1/x_R in V`. Extra roots are roots that are not decompositions.

| n | V | targets | degenerate | triples tested | S4 roots | decompositions | decompositions that are not roots | extra roots F1 | extra roots F2 | unexplained extras | extras at non-degenerate targets | targets with S4 roots = decompositions | targets with extras |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 3 | F | 1 | 1 | 288 | 11 | 2 | 0 | 6 | 3 | 0 | 0 | 0 | 1 |
| 5 | F | 21 | 21 | 354816 | 6857 | 6542 | 0 | 210 | 105 | 0 | 0 | 0 | 21 |
| 9 | F | 253 | 253 | 17011703808 | 10947437 | 10849526 | 0 | 65274 | 32637 | 0 | 0 | 0 | 253 |
| 11 | wt<=2 | 1057 | 44 | 161325682 | 9064 | 8096 | 0 | 968 | 0 | 0 | 0 | 1035 | 22 |
| 11 | wt<=3 | 1057 | 198 | 6627914272 | 681329 | 666083 | 0 | 13068 | 2178 | 0 | 0 | 859 | 198 |
