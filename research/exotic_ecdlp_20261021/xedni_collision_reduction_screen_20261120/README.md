# Fixed-prime reduction leaves six collision pairs for wider CRT search

The fixed order-43 curve over `F₄₇` supplies a necessary relation gate for every rational curve in the frozen auxiliary-collision construction. An independent finite-field group-law screen found **8** relation-bearing inputs among the **336** ordered auxiliary-x-collision pairs. Two of the eight have no admissible local gate at 7 or 11; the remaining **six** account for **600** constructed centered CRT curves. The other 328 source pairs, including all 32 same-point pairs, cannot satisfy `P + 2Q = O` on a rational lift with this prescribed good reduction at 47. The checked Sage replay verified all 336 finite-field sums and the exact incidence and fixed-curve reduction of all 18,576 constructed rational curves.

| Source class | Pairs | Constructed centered curves |
| --- | ---: | ---: |
| Reduced relation; auxiliary local gate present | 6 | 600 |
| Reduced relation; auxiliary local gate absent | 2 | 0 |
| Reduced relation absent | 328 | 17,976 |
| **Total** | **336** | **18,576** |

The six admitted source indices are `273, 315, 1160, 1202, 1500, 1542`, with constructed counts `168, 168, 66, 66, 66, 66`. Indices `663` and `703` pass the fixed-prime relation but have no local gate. The [result](result.json) retains every selected pair, finite point coordinates, relation bit, local status, and constructed count. The [protocol](protocol.json) and independent finite-field [runner](screen.py) were committed and pushed before execution as `c7cc8118658`. Selection never uses the recorded relation flag; the runner computes `P+2Q` and compares that flag only afterward.

## Reduction argument and proof map

For a rational elliptic curve with good reduction at 47 and two rational points whose coordinates are integral there, reduction is a group homomorphism. A rational equality `P+Q+Q=O` therefore implies the same equality for their points on the reduced curve. The [Lean theorem](formal/ReductionNecessary.lean), `XedniCollisionReduction.no_relation_of_reduced_sum`, proves the contrapositive for any additive homomorphism: if the reduced sum is nonidentity, the original sum is nonidentity. Its exact hypotheses are additive commutative groups `A,B`, a homomorphism `red:A→+B`, points `P,Q:A`, and `red P+red Q+red Q≠0`.

The theorem is checked under pinned Lean `v4.35.0-rc4` and Mathlib `021c46bdd042f67b538b9f9871d556a93713a9a9`; [build output](formal/build-output.txt) reports only `propext`, with empty stderr. There are no `sorry`, `admit`, custom claim axioms, or native evaluation. `lean4checker --fresh` is unavailable on this host. A Lean construction of the actual rational-curve good-reduction homomorphism and its application to all 18,576 rows remains open. The exact coefficient and point checks below establish the concrete reduction conditions computationally.

The [Sage audit](sage_audit.py), through the repository's checked launcher, independently constructs `y²=x³−x−2` over `F₄₇`, verifies group order 43 and nonsingularity, evaluates every selected `P+2Q`, and checks all 18,576 rational candidate coefficient tuples and their two source points. Every coefficient denominator is a 47-unit; each tuple reduces to `[a1,a2,a3,a4,a6]=[0,0,0,46,45]`, the fixed source curve. Sage also constructs the 600 rational curves in the eligible class, verifies their point sums, and finds zero `P+2Q=O` relations among these centered representatives. The [audit receipt](sage-audit-result.json) is `PASS` with empty stderr, and the [Sage runtime receipt](sage-runtime-info.json) records the accepted build. The prior local CRT packet separately checked the auxiliary orders.

## Replay and next test

For a fresh checkout, first reconstruct the prior candidate-row gzip from its checked shards. Then:

```sh
python3 research/exotic_ecdlp_20261021/xedni_local_collision_crt_20261120/rebuild_artifacts.py
python3 research/exotic_ecdlp_20261021/xedni_collision_reduction_screen_20261120/screen.py > research/exotic_ecdlp_20261021/xedni_collision_reduction_screen_20261120/result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_collision_reduction_screen_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_collision_reduction_screen_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_collision_reduction_screen_20261120/sage-audit-result.json
sh research/exotic_ecdlp_20261021/xedni_collision_reduction_screen_20261120/formal/build.sh
```

A wider-height coefficient experiment can now hold these six source pairs and their 600 local witness pairs fixed, specify a bounded set of higher CRT shifts, and retain every rational relation result. The first centered representative for each of those 600 had no rational `P+2Q` relation in the preceding census; new representatives test a different coefficient-height policy.
