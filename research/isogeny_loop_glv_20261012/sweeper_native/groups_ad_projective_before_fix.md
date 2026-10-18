## Native isogeny-loop sweep — cost model projective, doubling 8 M, primes ℓ ≤ 100, radius factor 1

start (UTC): 2026-10-18T15:46:33Z

### ECC2K-130

- t² − 4q = f²·D_K re-verified: true; D_K = -7 (3 bits), f = 38531015900842053623
- Cl(O_K): h = 1, cyc = []; 0.10 s (96669 relations, FB bound 100 = Bach·1, dense block 0), log(h/h_approx) = -0.0000 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 1, cyc = [] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 2,11,23,29,37,43,53,67,71,79 ; cost model projective (c_dbl = 8 M), GLV saving 520.0 M; radius 2.0 M, LLL upper bound 2.0 M, exhaustive within radius: true, 1 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 2.0 | 0.004 | {2: 1} | 1 | 2 | (1 + -1√D)/2 | — | no | 484053659726…(39 digits) |
| 2 | 10.0 | 0.019 | {2: -1} | 1 | 2 | (-1 + -1√D)/2 | — | no | 484053659726…(39 digits) |
| 3 | 82.5 | 0.159 | {11: 1} | 1 | 4 | (-4 + -2√D)/2 | — | no | 287542585610…(39 digits) |
| 4 | 82.5 | 0.159 | {11: -1} | 1 | 4 | (-4 + 2√D)/2 | — | no | 393022148231…(39 digits) |
| 5 | 172.5 | 0.332 | {23: 1} | 1 | 5 | (-8 + -2√D)/2 | — | no | 287542585610…(39 digits) |
| 6 | 172.5 | 0.332 | {23: -1} | 1 | 5 | (-8 + 2√D)/2 | — | no | 393022148231…(39 digits) |

- cheapest loop that is not a Frobenius/Verschiebung power: 2.0 M (0.004× saving), {"2":1}, chain 1
- frozen PARI loops[0].cost_M = Some(2.0), native = Some(2.0) → equal: Some(true)
- wall time 0.10 s

### icv1-f2m83-tm6151469093347-debefd74

- t² − 4q = f²·D_K re-verified: true; D_K = -7 (3 bits), f = 347450761417
- Cl(O_K): h = 1, cyc = []; 0.10 s (96669 relations, FB bound 100 = Bach·1, dense block 0), log(h/h_approx) = -0.0000 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 1, cyc = [] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 2,11,23,29,37,43,53,67,71,79 ; cost model projective (c_dbl = 8 M), GLV saving 328.0 M; radius 2.0 M, LLL upper bound 2.0 M, exhaustive within radius: true, 1 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 2.0 | 0.006 | {2: 1} | 1 | 2 | (-1 + -1√D)/2 | — | no | 254512724090651164922414 |
| 2 | 10.0 | 0.030 | {2: -1} | 1 | 2 | (1 + -1√D)/2 | — | no | 254512724090651164922415 |
| 3 | 82.5 | 0.252 | {11: 1} | 1 | 4 | (-4 + -2√D)/2 | — | no | 509025448181302329844827 |
| 4 | 82.5 | 0.252 | {11: -1} | 1 | 4 | (-4 + 2√D)/2 | — | no | 190882619104…(25 digits) |
| 5 | 172.5 | 0.526 | {23: 1} | 1 | 5 | (-8 + -2√D)/2 | — | no | 509025448181302329844825 |
| 6 | 172.5 | 0.526 | {23: -1} | 1 | 5 | (-8 + 2√D)/2 | — | no | 190882619104…(25 digits) |

- cheapest loop that is not a Frobenius/Verschiebung power: 2.0 M (0.006× saving), {"2":1}, chain 1
- frozen PARI loops[0].cost_M = Some(2.0), native = Some(2.0) → equal: Some(true)
- wall time 0.10 s

### icv1-f2m83-t6151469093347-cdcc5432

- t² − 4q = f²·D_K re-verified: true; D_K = -7 (3 bits), f = 347450761417
- Cl(O_K): h = 1, cyc = []; 0.09 s (96669 relations, FB bound 100 = Bach·1, dense block 0), log(h/h_approx) = -0.0000 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 1, cyc = [] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 2,11,23,29,37,43,53,67,71,79 ; cost model projective (c_dbl = 8 M), GLV saving 216.0 M; radius 2.0 M, LLL upper bound 2.0 M, exhaustive within radius: true, 1 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 2.0 | 0.009 | {2: 1} | 1 | 2 | (1 + -1√D)/2 | — | no | 2998004823950457 |
| 2 | 10.0 | 0.046 | {2: -1} | 1 | 2 | (-1 + -1√D)/2 | — | no | 2998004823950456 |
| 3 | 82.5 | 0.382 | {11: 1} | 1 | 4 | (-4 + -2√D)/2 | — | no | 5996009647900911 |
| 4 | 82.5 | 0.382 | {11: -1} | 1 | 4 | (-4 + 2√D)/2 | — | no | 2573776459948144 |
| 5 | 172.5 | 0.799 | {23: 1} | 1 | 5 | (-8 + -2√D)/2 | — | no | 5996009647900909 |
| 6 | 172.5 | 0.799 | {23: -1} | 1 | 5 | (-8 + 2√D)/2 | — | no | 2573776459948142 |

- cheapest loop that is not a Frobenius/Verschiebung power: 2.0 M (0.009× saving), {"2":1}, chain 1
- frozen PARI loops[0].cost_M = Some(2.0), native = Some(2.0) → equal: Some(true)
- wall time 0.10 s

### ECC2K-95

- t² − 4q = f²·D_K re-verified: true; D_K = -7 (3 bits), f = 264777735625649
- Cl(O_K): h = 1, cyc = []; 0.09 s (96669 relations, FB bound 100 = Bach·1, dense block 0), log(h/h_approx) = -0.0000 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 1, cyc = [] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 2,11,23,29,37,43,53,67,71,79 ; cost model projective (c_dbl = 8 M), GLV saving 384.0 M; radius 2.0 M, LLL upper bound 2.0 M, exhaustive within radius: true, 1 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 2.0 | 0.005 | {2: 1} | 1 | 2 | (-1 + -1√D)/2 | — | no | 239863891055…(29 digits) |
| 2 | 10.0 | 0.026 | {2: -1} | 1 | 2 | (1 + -1√D)/2 | — | no | 239863891055…(29 digits) |
| 3 | 82.5 | 0.215 | {11: 1} | 1 | 4 | (-4 + -2√D)/2 | — | no | 835869695405…(28 digits) |
| 4 | 82.5 | 0.215 | {11: -1} | 1 | 4 | (-4 + 2√D)/2 | — | no | 312553843030…(29 digits) |
| 5 | 172.5 | 0.449 | {23: 1} | 1 | 5 | (-8 + -2√D)/2 | — | no | 835869695405…(28 digits) |
| 6 | 172.5 | 0.449 | {23: -1} | 1 | 5 | (-8 + 2√D)/2 | — | no | 312553843030…(29 digits) |

- cheapest loop that is not a Frobenius/Verschiebung power: 2.0 M (0.005× saving), {"2":1}, chain 1
- frozen PARI loops[0].cost_M = Some(2.0), native = Some(2.0) → equal: Some(true)
- wall time 0.09 s

### ECC2K-108

- t² − 4q = f²·D_K re-verified: true; D_K = -7 (3 bits), f = 2288392606915201
- Cl(O_K): h = 1, cyc = []; 0.10 s (96669 relations, FB bound 100 = Bach·1, dense block 0), log(h/h_approx) = -0.0000 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 1, cyc = [] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 2,11,23,29,37,43,53,67,71,79 ; cost model projective (c_dbl = 8 M), GLV saving 432.0 M; radius 2.0 M, LLL upper bound 2.0 M, exhaustive within radius: true, 1 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 2.0 | 0.005 | {2: 1} | 1 | 2 | (1 + -1√D)/2 | — | no | 138423345589…(33 digits) |
| 2 | 10.0 | 0.023 | {2: -1} | 1 | 2 | (-1 + -1√D)/2 | — | no | 138423345589…(33 digits) |
| 3 | 82.5 | 0.191 | {11: 1} | 1 | 4 | (-4 + -2√D)/2 | — | no | 276846691179…(33 digits) |
| 4 | 82.5 | 0.191 | {11: -1} | 1 | 4 | (-4 + 2√D)/2 | — | no | 476718624790…(32 digits) |
| 5 | 172.5 | 0.399 | {23: 1} | 1 | 5 | (-8 + -2√D)/2 | — | no | 276846691179…(33 digits) |
| 6 | 172.5 | 0.399 | {23: -1} | 1 | 5 | (-8 + 2√D)/2 | — | no | 476718624790…(32 digits) |

- cheapest loop that is not a Frobenius/Verschiebung power: 2.0 M (0.005× saving), {"2":1}, chain 1
- frozen PARI loops[0].cost_M = Some(2.0), native = Some(2.0) → equal: Some(true)
- wall time 0.10 s

### secp256k1

- t² − 4q = f²·D_K re-verified: true; D_K = -3 (2 bits), f = 303414439467…(39 digits)
- Cl(O_K): h = 1, cyc = []; 0.09 s (96711 relations, FB bound 100 = Bach·1, dense block 0), log(h/h_approx) = -0.0000 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 1, cyc = [] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 7,13,19,31,37,43,61,67,73,79,97 ; cost model projective (c_dbl = 8 M), GLV saving 1024.0 M; radius 52.5 M, LLL upper bound 52.5 M, exhaustive within radius: true, 2 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 52.5 | 0.051 | {7: 1} | 1 | 3 | (5 + -1√D)/2 | — | no | 377180803631…(77 digits) |
| 2 | 52.5 | 0.051 | {7: -1} | 1 | 3 | (-5 + -1√D)/2 | — | no | 377180803631…(77 digits) |
| 3 | 97.5 | 0.095 | {13: 1} | 1 | 4 | (7 + -1√D)/2 | — | no | 377180803631…(77 digits) |
| 4 | 97.5 | 0.095 | {13: -1} | 1 | 4 | (-7 + -1√D)/2 | — | no | 377180803631…(77 digits) |
| 5 | 142.5 | 0.139 | {19: 1} | 1 | 5 | (7 + -3√D)/2 | — | no | 113154241089…(78 digits) |
| 6 | 142.5 | 0.139 | {19: -1} | 1 | 5 | (-8 + 2√D)/2 | — | no | 403559285110…(77 digits) |

- cheapest loop that is not a Frobenius/Verschiebung power: 52.5 M (0.051× saving), {"7":1}, chain 1
- frozen PARI loops[0].cost_M = Some(52.5), native = Some(52.5) → equal: Some(true)
- wall time 0.09 s

### id-GostR3410-2001-CryptoPro-B-ParamSet

- t² − 4q = f²·D_K re-verified: true; D_K = -619 (10 bits), f = 464640250601…(37 digits)
- Cl(O_K): h = 5, cyc = ["5"]; 0.14 s (97593 relations, FB bound 248 = Bach·1, dense block 1), log(h/h_approx) = 0.0001 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 5, cyc = ["5"] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 5,7,23,31,37,41,53,61,71,73,79,89 ; cost model projective (c_dbl = 8 M), GLV saving 1024.0 M; radius 127.5 M, LLL upper bound 127.5 M, exhaustive within radius: true, 2 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 127.5 | 0.125 | {5: 2, 7: 1} | 3 | 8 | (-9 + -1√D)/2 | — | no | 321506601824…(77 digits) |
| 2 | 127.5 | 0.125 | {5: -2, 7: -1} | 3 | 8 | (9 + -1√D)/2 | — | no | 321506601824…(77 digits) |
| 3 | 142.5 | 0.139 | {5: 1, 7: -2} | 3 | 8 | (-19 + -1√D)/2 | — | no | 321506601824…(77 digits) |
| 4 | 142.5 | 0.139 | {5: -1, 7: 2} | 3 | 8 | (19 + -1√D)/2 | — | no | 321506601824…(77 digits) |
| 5 | 225.0 | 0.220 | {7: 1, 23: 1} | 2 | 8 | (5 + -1√D)/2 | — | no | 321506601824…(77 digits) |
| 6 | 225.0 | 0.220 | {7: -1, 23: -1} | 2 | 8 | (-5 + -1√D)/2 | — | no | 321506601824…(77 digits) |

- cheapest loop that is not a Frobenius/Verschiebung power: 127.5 M (0.125× saving), {"5":2,"7":1}, chain 3
- frozen PARI loops[0].cost_M = Some(127.5), native = Some(127.5) → equal: Some(true)
- wall time 0.14 s

### id-GostR3410-2001-CryptoPro-A-ParamSet

- t² − 4q = f²·D_K re-verified: true; D_K = -424665378973637295795640888511775743925042728546036986688706749501354362291267 (258 bits), f = 1
- skipped: |D_K| has 258 bits > --max-disc-bits 140
### sect113r1

- t² − 4q = f²·D_K re-verified: true; D_K = -26504973335422840129609279124569399 (115 bits), f = 1
- Cl(O_K): h = 118323207158487408, cyc = ["29580801789621852", "2", "2"]; 120.93 s (4271 relations, FB bound 37696 = Bach·1, dense block 732), log(h/h_approx) = 0.0002 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 118323207158487408, cyc = ["29580801789621852", "2", "2"] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 2,5,11,13,17,23,31,41,53,73,83,89 ; cost model projective (c_dbl = 8 M), GLV saving 456.0 M; radius 226.0 M, LLL upper bound 226.0 M, exhaustive within radius: true, 1 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 226.0 | 0.496 | {2: 113} | 113 | 114 | (122610772499221213 + -1√D)/2 | π^1 | Z[π] = O_K | 519229685853…(34 digits) |
| 2 | 1130.0 | 2.478 | {2: -113} | 113 | 114 | (-122610772499221213 + -1√D)/2 | π̄^1 | Z[π] = O_K | 519229685853…(34 digits) |
| 3 | 9594.5 | 21.041 | {2: 16, 11: 14, 13: -3, 17: -26, 23: -3, 41: -4, 53: 6, 89: 1} | 73 | 258 | (106572387326…(40 digits) + 3471224231286831857271√D)/2 | — | Z[π] = O_K | 927037341735…(33 digits) |
| 4 | 9722.5 | 21.321 | {2: -16, 11: -14, 13: 3, 17: 26, 23: 3, 41: 4, 53: -6, 89: -1} | 73 | 258 | (-10657238732…(40 digits) + 3471224231286831857271√D)/2 | — | Z[π] = O_K | 117628151075…(34 digits) |
| 5 | 11742.5 | 25.751 | {2: 40, 5: 39, 11: 12, 13: 20, 17: 7, 23: 3, 31: -8, 41: -1, 53: 2, 73: -3, 83: 2} | 137 | 377 | (534392998625…(57 digits) + -35897013690…(40 digits)√D)/2 | — | Z[π] = O_K | 108982013889…(34 digits) |
| 6 | 12062.5 | 26.453 | {2: -40, 5: -39, 11: -12, 13: -20, 17: -7, 23: -3, 31: 8, 41: 1, 53: -2, 73: 3, 83: -2} | 137 | 377 | (534392998625…(57 digits) + 358970136907…(40 digits)√D)/2 | — | Z[π] = O_K | 196008602657…(34 digits) |

- cheapest loop that is not a Frobenius/Verschiebung power: 9594.5 M (21.041× saving), {"11":14,"13":-3,"17":-26,"2":16,"23":-3,"41":-4,"53":6,"89":1}, chain 73 — above the radius, from the LLL basis (upper bound)
- frozen PARI loops[0].cost_M = Some(226.0), native = Some(226.0) → equal: Some(true)
- wall time 120.93 s

### ECC2-89

- t² − 4q = f²·D_K re-verified: true; D_K = -38022505797344815572614143 (85 bits), f = 7
- Cl(O_K): h = 2844564357920, cyc = ["1422282178960", "2"]; 8.49 s (3801 relations, FB bound 20816 = Bach·1, dense block 289), log(h/h_approx) = -0.0000 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 2844564357920, cyc = ["1422282178960", "2"] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 2,7,13,17,29,47,53,59,61,67,79,83,89 ; cost model projective (c_dbl = 8 M), GLV saving 360.0 M; radius 360.0 M, LLL upper bound 7377.5 M, exhaustive within radius: true, 2 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 178.0 | 0.494 | {2: 89} | 89 | 90 | (-24754338902521 + 7√D)/2 | π^1 | yes (1) | 1 |
| 2 | 356.0 | 0.989 | {2: 178} | 178 | 179 | (-62516274478…(27 digits) + -173280372317647√D)/2 | π^2 | yes (-61897001964…(27 digits)) | 1 |
| 3 | 890.0 | 2.472 | {2: -89} | 89 | 90 | (24754338902521 + 7√D)/2 | π̄^1 | yes (24754338902522) | 24754338902522 |
| 4 | 1780.0 | 4.944 | {2: -178} | 178 | 179 | (625162744784…(27 digits) + -173280372317647√D)/2 | π̄^2 | yes (619272514180…(25 digits)) | 619272514180…(25 digits) |
| 5 | 7377.5 | 20.493 | {2: 25, 7: 1, 13: -6, 17: 2, 29: 1, 47: 1, 53: 1, 59: 5, 61: -1, 67: -3, 83: 1, 89: -1} | 48 | 141 | (2982930893027783736107 + 85091627√D)/2 | — | no | 265274507482…(27 digits) |
| 6 | 7577.5 | 21.049 | {2: -25, 7: -1, 13: 6, 17: -2, 29: -1, 47: -1, 53: -1, 59: -5, 61: 1, 67: 3, 83: -1, 89: 1} | 48 | 141 | (-2982930893027783736107 + 85091627√D)/2 | — | no | 265271524551…(27 digits) |

- cheapest loop that is not a Frobenius/Verschiebung power: 7377.5 M (20.493× saving), {"13":-6,"17":2,"2":25,"29":1,"47":1,"53":1,"59":5,"61":-1,"67":-3,"7":1,"83":1,"89":-1}, chain 48 — above the radius, from the LLL basis (upper bound)
- frozen PARI loops[0].cost_M = Some(178.0), native = Some(178.0) → equal: Some(true)
- wall time 8.50 s

### ECCp-89

- t² − 4q = f²·D_K re-verified: true; D_K = -632047307585397860799569011 (90 bits), f = 1
- Cl(O_K): h = 3470465617152, cyc = ["1735232808576", "2"]; 14.45 s (2639 relations, FB bound 22850 = Bach·1, dense block 397), log(h/h_approx) = 0.0002 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 3470465617152, cyc = ["1735232808576", "2"] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 5,11,29,37,43,71,83,97 ; cost model projective (c_dbl = 8 M), GLV saving 360.0 M; radius 360.0 M, LLL upper bound 17790.0 M, exhaustive within radius: true, 0 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 17790.0 | 49.417 | {5: 18, 11: -26, 29: 8, 37: 31, 43: 3, 71: 2, 83: -3, 97: 1} | 92 | 387 | (-19190937844…(59 digits) + -85649018899…(45 digits)√D)/2 | — | Z[π] = O_K | 338574086532…(27 digits) |
| 2 | 17790.0 | 49.417 | {5: -18, 11: 26, 29: -8, 37: -31, 43: -3, 71: -2, 83: 3, 97: -1} | 92 | 387 | (-19190937844…(59 digits) + 856490188990…(45 digits)√D)/2 | — | Z[π] = O_K | 176300170939…(27 digits) |
| 3 | 20880.0 | 58.000 | {5: 54, 11: 2, 29: -1, 37: 2, 43: -13, 71: 1, 83: 6, 97: 13} | 92 | 349 | (-48993474654…(53 digits) + -85588784771…(39 digits)√D)/2 | — | Z[π] = O_K | 260602585251…(27 digits) |
| 4 | 20880.0 | 58.000 | {5: -54, 11: -2, 29: 1, 37: -2, 43: 13, 71: -1, 83: -6, 97: -13} | 92 | 349 | (-48993474654…(53 digits) + 855887847713…(39 digits)√D)/2 | — | Z[π] = O_K | 138551654101…(26 digits) |
| 5 | 21532.5 | 59.812 | {5: 8, 11: -8, 29: -15, 43: 20, 71: 5, 83: 12, 97: 1} | 69 | 342 | (486861760717…(52 digits) + -49503232318…(38 digits)√D)/2 | — | Z[π] = O_K | 288262894750…(27 digits) |
| 6 | 21532.5 | 59.812 | {5: -8, 11: 8, 29: 15, 43: -20, 71: -5, 83: -12, 97: -1} | 69 | 342 | (-48686176071…(52 digits) + -49503232318…(38 digits)√D)/2 | — | Z[π] = O_K | 257714682663…(27 digits) |

- cheapest loop that is not a Frobenius/Verschiebung power: 17790.0 M (49.417× saving), {"11":-26,"29":8,"37":31,"43":3,"5":18,"71":2,"83":-3,"97":1}, chain 92 — above the radius, from the LLL basis (upper bound)
- frozen PARI loops[0].cost_M = Some(17790.0), native = Some(17790.0) → equal: Some(true)
- wall time 14.45 s

### ECC2-79

- t² − 4q = f²·D_K re-verified: true; D_K = -2064574834664024323763143 (81 bits), f = 1
- Cl(O_K): h = 551670148760, cyc = ["137917537190", "2", "2"]; 6.04 s (5397 relations, FB bound 18808 = Bach·1, dense block 257), log(h/h_approx) = 0.0001 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 551670148760, cyc = ["137917537190", "2", "2"] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 2,11,17,19,23,41,59,83,89 ; cost model projective (c_dbl = 8 M), GLV saving 320.0 M; radius 158.0 M, LLL upper bound 158.0 M, exhaustive within radius: true, 1 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 158.0 | 0.494 | {2: 79} | 79 | 80 | (594370931797 + -1√D)/2 | π^1 | Z[π] = O_K | 302231454903954479142442 |
| 2 | 790.0 | 2.469 | {2: -79} | 79 | 80 | (-594370931797 + -1√D)/2 | π̄^1 | Z[π] = O_K | 302231454903360108210645 |
| 3 | 7500.0 | 23.438 | {2: 15, 11: -1, 17: -8, 19: -7, 23: -1, 41: 12, 59: -2, 83: 1} | 47 | 168 | (-36613568408…(26 digits) + 1897400086181√D)/2 | — | Z[π] = O_K | 88751363608233665038990 |
| 4 | 7620.0 | 23.812 | {2: -15, 11: 1, 17: 8, 19: 7, 23: 1, 41: -12, 59: 2, 83: -1} | 47 | 168 | (366135684086…(26 digits) + 1897400086181√D)/2 | — | Z[π] = O_K | 132313728898806770903300 |
| 5 | 8668.0 | 27.087 | {2: 29, 11: -7, 19: 4, 23: -21, 41: 7, 59: 1, 83: -2} | 71 | 222 | (-40211950803…(34 digits) + -649192399652211087439√D)/2 | — | Z[π] = O_K | 126013851973755552753329 |
| 6 | 8900.0 | 27.812 | {2: -29, 11: 7, 19: -4, 23: 21, 41: -7, 59: -1, 83: 2} | 71 | 222 | (402119508036…(34 digits) + -649192399652211087439√D)/2 | — | Z[π] = O_K | 145419561973299067482761 |

- cheapest loop that is not a Frobenius/Verschiebung power: 7500.0 M (23.438× saving), {"11":-1,"17":-8,"19":-7,"2":15,"23":-1,"41":12,"59":-2,"83":1}, chain 47 — above the radius, from the LLL basis (upper bound)
- frozen PARI loops[0].cost_M = Some(158.0), native = Some(158.0) → equal: Some(true)
- wall time 6.04 s

### ECCp-79

- t² − 4q = f²·D_K re-verified: true; D_K = -1299233356027332017413179 (81 bits), f = 1
- Cl(O_K): h = 229211869248, cyc = ["14325741828", "4", "2", "2"]; 6.65 s (5973 relations, FB bound 18498 = Bach·1, dense block 274), log(h/h_approx) = -0.0003 (Euler product to 4194304), held-out relations 32/32 ok
- frozen PARI: h = 229211869248, cyc = ["14325741828", "4", "2", "2"] → h match Some(true), cyc match Some(true)
- split primes ℓ ≤ 100: 5,19,23,31,37,43,59,61,71,73,97 ; cost model projective (c_dbl = 8 M), GLV saving 320.0 M; radius 320.0 M, LLL upper bound 6517.5 M, exhaustive within radius: true, 0 loops within radius

| rank | cost M | cost / saving | loop (prime: exponent) | chain | N(α) bits | α = (A + B√D_K)/2 | Frobenius/Verschiebung power | α ∈ Z[π] (acts as) | λ |
|--:|--:|--:|:--|--:|--:|:--|:--|:--|:--|
| 1 | 6517.5 | 20.367 | {5: 4, 19: 11, 23: -2, 31: 6, 37: -1, 43: -1, 61: -1, 73: -1, 97: -2} | 29 | 131 | (-75173770348006859281 + 51628971√D)/2 | — | Z[π] = O_K | 466540787086994421823880 |
| 2 | 6517.5 | 20.367 | {5: -4, 19: -11, 23: 2, 31: -6, 37: 1, 43: 1, 61: 1, 73: 1, 97: 2} | 29 | 131 | (75173770348006859281 + 51628971√D)/2 | — | Z[π] = O_K | 18146025394785795944 |
| 3 | 7905.0 | 24.703 | {5: 4, 19: 3, 23: 1, 31: 6, 37: 2, 43: -1, 59: -2, 61: -2, 71: -1, 73: 2, 97: 2} | 26 | 128 | (27304444481944508878 + -12548848√D)/2 | — | Z[π] = O_K | 18377483845715754831 |
| 4 | 7905.0 | 24.703 | {5: -4, 19: -3, 23: -1, 31: -6, 37: -2, 43: 1, 59: 2, 61: 2, 71: 1, 73: -2, 97: -2} | 26 | 128 | (27304444481944508878 + 12548848√D)/2 | — | Z[π] = O_K | 8926960636228754047 |
| 5 | 8002.5 | 25.008 | {5: 16, 19: -2, 23: 1, 31: -2, 37: 3, 43: 2, 59: 1, 73: -7, 97: 1} | 35 | 143 | (1044243993623181497677 + -4621344657√D)/2 | — | Z[π] = O_K | 2262286709142843895904 |
| 6 | 8002.5 | 25.008 | {5: -16, 19: 2, 23: -1, 31: 2, 37: -3, 43: -2, 59: -1, 73: 7, 97: -1} | 35 | 143 | (1044243993623181497677 + 4621344657√D)/2 | — | Z[π] = O_K | 465379772116427980488990 |

- cheapest loop that is not a Frobenius/Verschiebung power: 6517.5 M (20.367× saving), {"19":11,"23":-2,"31":6,"37":-1,"43":-1,"5":4,"61":-1,"73":-1,"97":-2}, chain 29 — above the radius, from the LLL basis (upper bound)
- frozen PARI loops[0].cost_M = Some(6517.5), native = Some(6517.5) → equal: Some(true)
- wall time 6.65 s

