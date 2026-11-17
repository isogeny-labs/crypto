#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm), numbering: "1 / 1")
#set text(size: 8.6pt)
#set par(justify: true, leading: 0.5em)
#show table: set text(size: 7pt)
#show raw: set text(size: 7.2pt)
= Run report: run-20261018T1605Z

== Environment and command

```
run: 20261018T1605Z (Parts G and H)
protocol commit: d5bcdc081 (freeze 095c45c8a + amendment 1)
rustc 1.97.0 (2d8144b78 2026-07-07)
host: Linux 6.18.44-fc-v139 x86_64, 4 cores, Intel(R) Xeon(R) Processor @ 2.80GHz, 15 GB
command: rustc --edition 2021 -O examples/weil_restriction_isogeny_census.rs -o wric && ./wric research/weil_restriction_isogeny_census_20261018/results/run-20261018T1605Z G H
tool sha256: 3a87f373cae06cd24cd5f80950a7017a26a4087bdc56cd99ffa9a96689645d8c (rustfmt-formatted; the committed file)
threads: 1
smoke runs before this run (not evidence, outputs discarded): one Part G run (75 s, checks 1-4 as here) and one Part H run at q = 5, 7 that exposed a wrong expected degree in check 6 for the ℓ = 2 cubic (degree 3, not (ℓ^2 − 1)/2); the check was corrected in the tool before this run. No other change.

note (after the run): the committed tool was changed after this run in two ways that touch no arithmetic: two regression tests were added (the Jordan class at ℓ = 7 has 7 submodules of dimension 3, 3 kernels, 2 Tian kernels; nonsplit classes have no submodule of dimension 3) and clippy lints were fixed (is_multiple_of, an iterator loop in rref, type aliases). sha256 of the committed file: 04dead65a5847933f25609e29dcbea99349c8720d437a70613c93d118ba27c6c. Verified: kernels.csv of a rerun is byte-identical to this run's (seconds column excluded), and the q = 5, 7 rows of curves.csv and by_class.csv are byte-identical.
```

== Tool log

```
part G cell n=2 l=2: 0.00s
part G cell n=2 l=3: 0.00s
part G cell n=2 l=5: 0.02s
part G cell n=2 l=7: 0.13s
part G cell n=3 l=2: 0.00s
part G cell n=3 l=3: 0.01s
part G cell n=3 l=5: 0.92s
part G cell n=3 l=7: 15.25s
part G cell n=4 l=2: 0.00s
part G cell n=4 l=3: 0.28s
part G cell n=4 l=5: 55.55s
check1 (n=3, l=2: Tian kernels only for phi = I): violations=0
check2 (n=2: every Tian kernel is a graph with det psi = -qbar^-1 and phi psi^2 = I): violations=0
check3 (P2: at (n,l,qbar)=(3,3,1) a class without eigenvalue 1 has a Tian kernel): classes=1
check4 (representative independence): mismatches=0
part H q=3: 56 curves, 0.0s
part H q=5: 252 curves, 0.2s
part H q=7: 690 curves, 0.2s
part H q=11: 2662 curves, 2.2s
part H q=13: 4400 curves, 5.6s
q=3 check5 classes=56 expected=56 ok
q=3 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=3 check9 prime_order=8 prime_with_tian_some_odd_ell=0 all=56 all_with_tian_some_odd_ell=28
q=5 check5 classes=252 expected=252 ok
q=5 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=5 check9 prime_order=18 prime_with_tian_some_odd_ell=0 all=252 all_with_tian_some_odd_ell=156
q=7 check5 classes=690 expected=690 ok
q=7 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=7 check9 prime_order=61 prime_with_tian_some_odd_ell=24 all=690 all_with_tian_some_odd_ell=372
q=11 check5 classes=2662 expected=2662 ok
q=11 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=11 check9 prime_order=150 prime_with_tian_some_odd_ell=57 all=2662 all_with_tian_some_odd_ell=1984
q=13 check5 classes=4400 expected=4400 ok
q=13 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=13 check9 prime_order=323 prime_with_tian_some_odd_ell=144 all=4400 all_with_tian_some_odd_ell=2714
total 80.4s
```

== Checks

```
check1 (n=3, l=2: Tian kernels only for phi = I): violations=0
check2 (n=2: every Tian kernel is a graph with det psi = -qbar^-1 and phi psi^2 = I): violations=0
check3 (P2: at (n,l,qbar)=(3,3,1) a class without eigenvalue 1 has a Tian kernel): classes=1
check4 (representative independence): mismatches=0
q=3 check5 classes=56 expected=56 ok
q=3 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=3 check9 prime_order=8 prime_with_tian_some_odd_ell=0 all=56 all_with_tian_some_odd_ell=28
q=5 check5 classes=252 expected=252 ok
q=5 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=5 check9 prime_order=18 prime_with_tian_some_odd_ell=0 all=252 all_with_tian_some_odd_ell=156
q=7 check5 classes=690 expected=690 ok
q=7 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=7 check9 prime_order=61 prime_with_tian_some_odd_ell=24 all=690 all_with_tian_some_odd_ell=372
q=11 check5 classes=2662 expected=2662 ok
q=11 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=11 check9 prime_order=150 prime_with_tian_some_odd_ell=57 all=2662 all_with_tian_some_odd_ell=1984
q=13 check5 classes=4400 expected=4400 ok
q=13 hasse_violations=0 class_not_found=0 check6_violations=0 check7_violations=0 check8_violations=0
q=13 check9 prime_order=323 prime_with_tian_some_odd_ell=144 all=4400 all_with_tian_some_odd_ell=2714
```

== Part H summary (summary.csv)

#table(columns: 11, align: right,
  [*q*], [*classes*], [*prime\_order*], [*cofactor\_2*], [*full\_2\_torsion*], [*ell*], [*with\_tian*], [*prime\_with\_tian*], [*cof2\_with\_tian*], [*prime\_any\_odd\_ell*], [*all\_any\_odd\_ell*],
  [3], [56], [8], [7], [9], [2], [9], [0], [0], [0], [28],
  [3], [56], [8], [7], [9], [5], [26], [0], [6], [0], [28],
  [3], [56], [8], [7], [9], [7], [8], [0], [0], [0], [28],
  [5], [252], [18], [25], [42], [2], [42], [0], [0], [0], [156],
  [5], [252], [18], [25], [42], [3], [124], [0], [0], [0], [156],
  [5], [252], [18], [25], [42], [7], [40], [0], [0], [0], [156],
  [7], [690], [61], [48], [115], [2], [115], [0], [0], [24], [372],
  [7], [690], [61], [48], [115], [3], [58], [3], [6], [24], [372],
  [7], [690], [61], [48], [115], [5], [342], [24], [0], [24], [372],
  [11], [2662], [150], [150], [443], [2], [443], [0], [0], [57], [1984],
  [11], [2662], [150], [150], [443], [3], [1330], [0], [0], [57], [1984],
  [11], [2662], [150], [150], [443], [5], [710], [57], [78], [57], [1984],
  [11], [2662], [150], [150], [443], [7], [758], [12], [12], [57], [1984],
  [13], [4400], [323], [288], [734], [2], [734], [0], [0], [144], [2714],
  [13], [4400], [323], [288], [734], [3], [368], [12], [24], [144], [2714],
  [13], [4400], [323], [288], [734], [5], [2198], [135], [114], [144], [2714],
  [13], [4400], [323], [288], [734], [7], [732], [0], [0], [144], [2714],
)

== Part G, n = 3 (kernels.csv)

#table(columns: 10, align: right,
  [*ell*], [*qbar*], [*class*], [*tr*], [*det*], [*eigenvalue\_one*], [*submodules\_dim\_n*], [*kernels*], [*tian\_kernels*], [*types\_d01*],
  [2], [1], [scalar], [0], [1], [true], [15], [9], [6], [1:6],
  [2], [1], [jordan], [0], [1], [true], [1], [1], [0], [-],
  [2], [1], [nonsplit], [1], [1], [false], [0], [0], [0], [-],
  [3], [1], [nonsplit], [0], [1], [false], [0], [0], [0], [-],
  [3], [1], [scalar], [1], [1], [false], [40], [16], [12], [1:12],
  [3], [1], [jordan], [1], [1], [false], [1], [1], [0], [-],
  [3], [1], [scalar], [2], [1], [true], [40], [16], [12], [1:12],
  [3], [1], [jordan], [2], [1], [true], [1], [1], [0], [-],
  [3], [2], [split], [0], [2], [true], [4], [4], [2], [1:2],
  [3], [2], [nonsplit], [1], [2], [false], [0], [0], [0], [-],
  [3], [2], [nonsplit], [2], [2], [false], [0], [0], [0], [-],
  [5], [1], [split], [0], [1], [false], [4], [4], [2], [1:2],
  [5], [1], [nonsplit], [1], [1], [false], [0], [0], [0], [-],
  [5], [1], [scalar], [2], [1], [true], [156], [36], [30], [1:30],
  [5], [1], [jordan], [2], [1], [true], [1], [1], [0], [-],
  [5], [1], [scalar], [3], [1], [false], [156], [36], [30], [1:30],
  [5], [1], [jordan], [3], [1], [false], [1], [1], [0], [-],
  [5], [1], [nonsplit], [4], [1], [false], [0], [0], [0], [-],
  [5], [2], [nonsplit], [0], [3], [false], [0], [0], [0], [-],
  [5], [2], [split], [1], [3], [false], [4], [4], [2], [1:2],
  [5], [2], [nonsplit], [2], [3], [false], [0], [0], [0], [-],
  [5], [2], [nonsplit], [3], [3], [false], [0], [0], [0], [-],
  [5], [2], [split], [4], [3], [true], [4], [4], [2], [1:2],
  [5], [3], [nonsplit], [0], [2], [false], [0], [0], [0], [-],
  [5], [3], [nonsplit], [1], [2], [false], [0], [0], [0], [-],
  [5], [3], [split], [2], [2], [false], [4], [4], [2], [1:2],
  [5], [3], [split], [3], [2], [true], [4], [4], [2], [1:2],
  [5], [3], [nonsplit], [4], [2], [false], [0], [0], [0], [-],
  [5], [4], [split], [0], [4], [true], [4], [4], [2], [1:2],
  [5], [4], [scalar], [1], [4], [false], [156], [36], [30], [1:30],
  [5], [4], [jordan], [1], [4], [false], [1], [1], [0], [-],
  [5], [4], [nonsplit], [2], [4], [false], [0], [0], [0], [-],
  [5], [4], [nonsplit], [3], [4], [false], [0], [0], [0], [-],
  [5], [4], [scalar], [4], [4], [false], [156], [36], [30], [1:30],
  [5], [4], [jordan], [4], [4], [false], [1], [1], [0], [-],
  [7], [1], [nonsplit], [0], [1], [false], [0], [0], [0], [-],
  [7], [1], [split], [1], [1], [false], [2], [2], [0], [-],
  [7], [1], [scalar], [2], [1], [true], [560], [80], [72], [1:72],
  [7], [1], [jordan], [2], [1], [true], [7], [3], [2], [1:2],
  [7], [1], [nonsplit], [3], [1], [false], [0], [0], [0], [-],
  [7], [1], [nonsplit], [4], [1], [false], [0], [0], [0], [-],
  [7], [1], [scalar], [5], [1], [false], [560], [80], [72], [1:72],
  [7], [1], [jordan], [5], [1], [false], [7], [3], [2], [1:2],
  [7], [1], [split], [6], [1], [false], [2], [2], [0], [-],
  [7], [2], [nonsplit], [0], [1], [false], [0], [0], [0], [-],
  [7], [2], [split], [1], [1], [false], [2], [2], [0], [-],
  [7], [2], [scalar], [2], [1], [true], [560], [80], [72], [1:72],
  [7], [2], [jordan], [2], [1], [true], [7], [3], [2], [1:2],
  [7], [2], [nonsplit], [3], [1], [false], [0], [0], [0], [-],
  [7], [2], [nonsplit], [4], [1], [false], [0], [0], [0], [-],
  [7], [2], [scalar], [5], [1], [false], [560], [80], [72], [1:72],
  [7], [2], [jordan], [5], [1], [false], [7], [3], [2], [1:2],
  [7], [2], [split], [6], [1], [false], [2], [2], [0], [-],
  [7], [3], [split], [0], [6], [true], [20], [8], [6], [1:6],
  [7], [3], [nonsplit], [1], [6], [false], [0], [0], [0], [-],
  [7], [3], [split], [2], [6], [false], [2], [2], [0], [-],
  [7], [3], [nonsplit], [3], [6], [false], [0], [0], [0], [-],
  [7], [3], [nonsplit], [4], [6], [false], [0], [0], [0], [-],
  [7], [3], [split], [5], [6], [false], [2], [2], [0], [-],
  [7], [3], [nonsplit], [6], [6], [false], [0], [0], [0], [-],
  [7], [4], [nonsplit], [0], [1], [false], [0], [0], [0], [-],
  [7], [4], [split], [1], [1], [false], [2], [2], [0], [-],
  [7], [4], [scalar], [2], [1], [true], [560], [80], [72], [1:72],
  [7], [4], [jordan], [2], [1], [true], [7], [3], [2], [1:2],
  [7], [4], [nonsplit], [3], [1], [false], [0], [0], [0], [-],
  [7], [4], [nonsplit], [4], [1], [false], [0], [0], [0], [-],
  [7], [4], [scalar], [5], [1], [false], [560], [80], [72], [1:72],
  [7], [4], [jordan], [5], [1], [false], [7], [3], [2], [1:2],
  [7], [4], [split], [6], [1], [false], [2], [2], [0], [-],
  [7], [5], [split], [0], [6], [true], [20], [8], [6], [1:6],
  [7], [5], [nonsplit], [1], [6], [false], [0], [0], [0], [-],
  [7], [5], [split], [2], [6], [false], [2], [2], [0], [-],
  [7], [5], [nonsplit], [3], [6], [false], [0], [0], [0], [-],
  [7], [5], [nonsplit], [4], [6], [false], [0], [0], [0], [-],
  [7], [5], [split], [5], [6], [false], [2], [2], [0], [-],
  [7], [5], [nonsplit], [6], [6], [false], [0], [0], [0], [-],
  [7], [6], [split], [0], [6], [true], [20], [8], [6], [1:6],
  [7], [6], [nonsplit], [1], [6], [false], [0], [0], [0], [-],
  [7], [6], [split], [2], [6], [false], [2], [2], [0], [-],
  [7], [6], [nonsplit], [3], [6], [false], [0], [0], [0], [-],
  [7], [6], [nonsplit], [4], [6], [false], [0], [0], [0], [-],
  [7], [6], [split], [5], [6], [false], [2], [2], [0], [-],
  [7], [6], [nonsplit], [6], [6], [false], [0], [0], [0], [-],
)

== Part H: classes with a Tian kernel at odd ℓ (by_class.csv)

#table(columns: 9, align: right,
  [*q*], [*ell*], [*qbar*], [*class*], [*tr*], [*det*], [*curves*], [*prime\_order\_curves*], [*cofactor\_2\_curves*],
  [3], [5], [3], [split], [2], [2], [13], [0], [6],
  [3], [5], [3], [split], [3], [2], [13], [0], [0],
  [3], [7], [3], [split], [0], [6], [8], [0], [0],
  [5], [3], [2], [split], [0], [2], [124], [0], [0],
  [5], [7], [5], [split], [0], [6], [40], [0], [0],
  [7], [3], [1], [scalar], [1], [1], [29], [3], [6],
  [7], [3], [1], [scalar], [2], [1], [29], [0], [0],
  [7], [5], [2], [split], [1], [3], [171], [24], [0],
  [7], [5], [2], [split], [4], [3], [171], [0], [0],
  [11], [3], [2], [split], [0], [2], [1330], [0], [0],
  [11], [5], [1], [scalar], [2], [1], [22], [0], [0],
  [11], [5], [1], [scalar], [3], [1], [22], [3], [0],
  [11], [5], [1], [split], [0], [1], [666], [54], [78],
  [11], [7], [4], [jordan], [2], [1], [370], [0], [0],
  [11], [7], [4], [jordan], [5], [1], [370], [12], [12],
  [11], [7], [4], [scalar], [2], [1], [9], [0], [0],
  [11], [7], [4], [scalar], [5], [1], [9], [0], [0],
  [13], [3], [1], [scalar], [1], [1], [184], [12], [24],
  [13], [3], [1], [scalar], [2], [1], [184], [0], [0],
  [13], [5], [3], [split], [2], [2], [1099], [135], [114],
  [13], [5], [3], [split], [3], [2], [1099], [0], [0],
  [13], [7], [6], [split], [0], [6], [732], [0], [0],
)

== Graph: Curves with a Tian kernel by q and ℓ

#image("/figures/affected.svg", width: 100%)

== Graph: n = 3: Tian kernels by class of φ

#image("/figures/kernel_map.svg", width: 100%)

== Graph: Kernel census by (n, ℓ)

#image("/figures/n2n4.svg", width: 100%)

