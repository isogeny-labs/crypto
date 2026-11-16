# Six `(1,1,1,2)` ordinary-node configurations excluded

The six positions are exactly the three linearly independent and three linearly dependent rational-node cases listed by Kudo--Harashita, Section 4.1 (https://arxiv.org/pdf/2102.07270). Their published large-point computation used a threshold of 32 points over F9; these candidate isogeny classes require 10. This work applies their position classification to a different point-count constraint. The result below is a finite search for sextics with three F3-rational **ordinary** nodes and one F9-conjugate pair of ordinary nodes. It does not include repeated-tangent double points or the remaining Frobenius-orbit patterns.

The frozen source `search.py` has SHA-256 `0d05a73a410d4c771ca3ae724d9acfdae76e61704d8c6ad497ef7b0b` and was committed before the six full runs. Each run asserts a 13-dimensional coefficient kernel, enumerates all 1,594,322 nonzero vectors (797,161 projective classes), and records one direct F3/F9 point-evaluation control when an ordinary-node sextic exists. All six runs exited 0; raw receipts are `FULL_0.json` through `FULL_5.json`.

| Case | Rational nodes independent? | Ordinary-node sextics | N1=4 | N2=10 | N3=28 | N4=82 | N5=314 |
| ---: | :---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | yes | 210,195 | 12,106 | 118 | 22 | 0 | 0 |
| 1 | yes | 209,952 | 16,118 | 424 | 76 | 8 | 0 |
| 2 | yes | 209,952 | 16,263 | 454 | 60 | 4 | 0 |
| 3 | no | 209,952 | 17,211 | 348 | 60 | 24 | 0 |
| 4 | no | 209,952 | 17,423 | 846 | 110 | 8 | 0 |
| 5 | no | 209,952 | 17,423 | 846 | 110 | 8 | 0 |

The 52 fourth-count survivors have independently recomputed F243 point counts in `verify_n5.py`, SHA-256 `32d30b08d9f005296844373b0a35218247948e9a84120a93472576d64a98cef2`. It constructs the sextic coefficients over F3 and evaluates the 28 monomials over every projective F243 point, rather than using the kernel-basis evaluation matrix of the primary screen. `VERIFIED_N5.json` records all 52 exact coefficient vectors, plane counts, rational-node tangent corrections, and normalized N5 counts. Its SHA-256 is `35e7534f2fe78d1fe77c0028554f8945aad4ce21311e8aa177577296a64a0cee`. Every value agreed with the frozen run.

The verified N5 histogram is `{184:12, 224:8, 244:12, 249:12, 279:4, 319:2, 334:2}`. An L-polynomial `1-t*T^5+3^5*T^10` requires N5=`244-5t`; the eight trace values `t=±10,±14,±22,±26` require `{114,134,174,194,294,314,354,374}`. The sets are disjoint. Therefore none of these 52 sextics belongs to any of those eight isogeny classes. Earlier filters exclude every other sextic in these six ordinary-node positions.

This is a bounded geometric exclusion, not a new vulnerable curve or an attack-cost comparison. Other five-node orbit patterns, repeated-tangent double points, and genus-five curves without such sextic models remain to be checked.
