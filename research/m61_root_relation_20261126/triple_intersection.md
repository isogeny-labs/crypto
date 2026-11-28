# Target-translate intersection of signed triple sums

Run the native Rust probe from the repository root:

`cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example triple_intersection`

Full deterministic output is in `triple_intersection.out`. The executable independently enumerates each toy curve order by Legendre symbols and checks `N=h·r` with prime `r`: `(p,N,h,r)=(8191,8024,136,59),(131071,131017,37,3541),(524287,525472,32,16421)`. It also verifies `[r][h]P=O` for every root-base point. Thus these are actual cofactor projections. Equal-size random-x controls use the same deterministic x sampling rule without excluding h-killed points. One B54 root point and two B20 random points become O after projection; output labels those cases. A B53 root sensitivity case removes its one projected-zero point and confirms the same complete projected support as its equal-size random control.

The inventory contains all `M=8·C(B,3)` signed triples on distinct base indices. The squarefree degree is the number of finite x coordinates of triple sums. For each target R, `intersection=|S∩(R−S)|`, where S is the set of **distinct full group points** represented by triples; this is the rank of the point-indexed 3+3 target indicator. The first target is planted as the sum of the first six base points. The next five are fixed scalar multiples `[1,5,17,101,513]P_0`; each is translated independently. The code recovers and verifies a disjoint 3+3 witness for the planted target.

| p, B, base | group | M | distinct S | squarefree x degree | six target intersections | triple-build additions |
|---|---|---:|---:|---:|---|---:|
| 8191, 8, root | full | 448 | 428 | 214 | 20,54,24,6,14,12 | 896 |
| 8191, 8, root | projected | 448 | 59 | 29 | 59,59,59,59,59,59 | 896 |
| 131071, 8, root | full | 448 | 448 | 224 | 20,0,0,0,48,0 | 896 |
| 131071, 8, root | projected | 448 | 444 | 222 | 82,40,126,52,72,66 | 896 |
| 524287, 8, root | full | 448 | 448 | 224 | 20,0,6,0,0,0 | 896 |
| 524287, 8, root | projected | 448 | 448 | 224 | 20,6,12,10,20,20 | 896 |
| 8191, 20, root | full | 9120 | 5533 | 2767 | 3990,3778,3816,3810,3772,3808 | 18240 |
| 8191, 20, root | projected | 9120 | 59 | 29 | 59 each | 18240 |
| 131071, 54, root | full | 198432 | 102153 | 51076 | 79778,79615,79439,79529,79311,79569 | 396864 |
| 131071, 54, root | projected | 198432 | 3541 | 1770 | 3541 each | 396864 |
| 524287, 79, root | full | 632632 | 366597 | 183298 | 255948,255350,256887,255686,256393,254596 | 1265264 |
| 524287, 79, root | projected | 632632 | 16421 | 8210 | 16421 each | 1265264 |

Equal-size random controls are close in the sparse B8 cases: at p131071, full support 448 and projected support 440; at p524287, 448 and 448. In the large-B cases, both root and random projected triple supports saturate the entire prime-order subgroup. This saturation is a **small-field artifact**: B=20,54,79 greatly exceeds the respective p^(1/6) scales. The small witness-search probe counts from these rows must not be extrapolated to M61. At p524287 B8, projection causes no squarefree-degree reduction at all; target intersections rise roughly as the group size falls by h.

## Exact six-term target coverage at B=8

The Rust example now enumerates all `2^6·C(8,6)=1,792` signed tuples on six distinct columns. It records the number of **distinct target group points** reached, without assuming uniform sums. Each inventory costs 8,960 group additions; hash operations and memory allocation are separate. This is an exact target-descent coverage check for these finite bases, not an asymptotic solver.

| p | base | full-group targets covered | cofactor-projected targets covered | projected subgroup order |
|---:|---|---:|---:|---:|
| 8,191 | root | 1,503 | 59 | 59 |
| 8,191 | random control | 1,585 | 59 | 59 |
| 131,071 | root | 1,792 | 1,532 | 3,541 |
| 131,071 | random control | 1,784 | 1,376 | 3,541 |
| 524,287 | root | 1,792 | 1,756 | 16,421 |
| 524,287 | random control | 1,792 | 1,750 | 16,421 |

The p=8,191 projected rows saturate their 59-point subgroup. At p=524,287, the first root window and one control cover 10.69% and 10.66% of projected targets respectively. The full-group root inventory there has no collisions among the 1,792 signed six-tuples, while projection has 36; that one control has 42 projected collisions. Even here, producing the complete inventory costs `Θ(B^6)` tuple work as B grows, so exact toy coverage does not deliver the `Õ(B²)` witness locator.

To test whether the first eight root points were representative, the Rust example also checks **every cyclic window of eight** among the ordered lift-valid root points, alongside 128 deterministic random-x controls (seeds 2026 through 2153). Each additional six-tuple inventory costs 8,960 group additions, excluding projection. The first window is near the high end of both distributions, especially at p=131,071; it must not be presented as typical root-base coverage.

| p | First root window | Root-window count, mean, range | Random control mean, range | Controls at least first window |
|---:|---:|---|---|---:|
| 131,071 | 1,532 | 54 windows; 1,337.37; 878–1,589 | 1,411.69; 958–1,578 | 8/128 |
| 524,287 | 1,756 | 79 windows; 1,703.81; 1,540–1,782 | 1,694.42; 1,552–1,782 | 9/128 |

At p=131,071, one root column projects to infinity and occurs in eight cyclic windows. Removing those windows leaves 46 root windows with mean coverage **1,402.00**, near the 128-control mean **1,411.69**; one control base also contains a projected infinity point. At p=524,287, no root window or control has a projected infinity point. At p=8,191, every one of the 16 initially sampled projected controls covers all 59 subgroup points, so that field cannot resolve a coverage difference. These finite controls reveal no consistent root-window advantage. The windows overlap heavily and are not independent samples. The result is a check against a misleading first-window comparison, not a statistical hardness claim.

The replay now also counts **unordered pairs of distinct signed six-tuples
with the same projected group sum**. Such pairs yield partial-relation
recombinations but need direct row and rank checks; shared columns can
cancel. At p=524,287 the first root window has 36 matching pairs, all
79 root windows average 91.18, and 128 random controls average 100.47.
The independent-uniform pair estimate is `C(1792,2)/16421≈97.72`.
At p=131,071 the first root window has 284, all 54 windows average
551.80, and controls average 442.98 versus `C(1792,2)/3541≈453.19`;
the root-window mean includes the eight windows with a projected-zero
column. These counts are direct finite-instance evidence for the
[large-prime recombination ledger](large_prime_recombination.md), not
an `Õ(B²)` collector.

Costs are explicit in the output. Building M triple sums costs exactly `2M` group additions. Projecting the base costs additional scalar-ladder additions (e.g. 474 for B79/h32). Testing six target translates costs exactly `6|S|` group additions, e.g. 2,199,582 for the p524287 B79 full case and 98,526 after projection. The output also gives deterministic planted-witness lookup and verification costs. Base square roots, hash operations, and memory allocation are not included in those group-addition totals.

For M61's B=2447 root basis, `M=19,512,280,120` triples, while `B²=5,987,809`. If projected sums behave near uniform in the order-`329406144237565159` subgroup, expected duplicate-pair collisions among triples are only about 578, so squarefree x degree remains near `M/2≈9.76·10^9`, over 1,600 times B². This extrapolation is heuristic. The toy data show that squarefree reduction and cofactor projection do not collapse the sparse-regime cubic inventory into an O(B²) object; they do not rule out an implicit operator or a different target-intersection algorithm.
