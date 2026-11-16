# R5.2: how much of a binary isogeny class is within GHS reach?

**Date.** 2026-10-18. **Lead.** R5.2 (round 5). **Example.** `examples/r5_2_ghs_class.rs`.
**Status labels.** *measured* = exact count printed by the example; *derived* = arithmetic
on measured counts or on a stated formula; *recalled* = a literature statement used without
re-checking the source; *model* = a cost model used only as a model. Toy fields only
(N ≤ 20). Nothing here is about a deployed curve. Prime N is out of scope: no
factorisation N = l·n with n ≥ 2 exists, so GHS does not apply.

Conductor was not installed in this container (`conductor check` is not on PATH), so no
task was reserved. The only files touched are the new example and this directory.

## 0. Verdict

* **The question as a cost statement is inconclusive at toy sizes. In the literal sense it is
  refuted at these sizes.** Under the concrete model P (min of Gaudry's g!·q^{2−2/g} and
  Enge–Gaudry L_{q^g}[1/2, √2], against rho √(πr/4)), **no curve at any N ∈ {12, 14, 15, 16,
  18, 20} is weak**: 0 of 2 858 996 curves (measured counts, derived verdicts). The best
  class still sits 1.65–2.42 bits above rho. So the falsifier "weak curves absent" holds, but
  trivially: at these sizes no curve is weak. That says nothing about how weak curves
  sit inside isogeny classes.
* **Structurally the hypothesis holds.** Take "low-genus reach" (LG) to mean an applicable
  descent with g ≤ 2n, transfer possible, and n ≥ 5 (for n = 3, 4 every descent has g ≤ 8).
  LG curves are then a small but nonzero share: 0.09 %–3.1 % of curves at the factorisations
  where they exist. Every class that contains one is mixed (1–160 classes per N; the median
  in-class share is 3.6 %–25 %). With ℓ ∈ {3, 5, 7}, 66 %–100 % of the strong curves in those
  classes reach an LG curve. The mean distance is 1.3–3.0 isogeny steps (max 15). The
  counted search from a random strong curve expands on average 1.5–8.4 curves (max 53).
  0 %–34 % of the strong curves are in ℓ ≤ 7 components with no LG curve.
* **The literal models M1 and A are degenerate** (§3). The constant-free Gaudry exponent
  (M1) marks most curves over F_2 and F_4 weak, because q^{2−2/g} ≤ q² ≤ 16 there whatever g
  is. The fixed-n exponent model (A) marks every non-subfield curve weak for n ≥ 4. Neither
  separates curves in a meaningful way.
* **Derived, from recalled exponents.** In the fixed-n regime (n fixed, q → ∞), GHS never
  beats the exponent Õ(q^{2−2/n}) of Gaudry's decomposition attack, which applies to *every*
  curve over F_{q^n}. Transfer needs g ≥ n, so 2 − 2/g ≥ 2 − 2/n. Within an isogeny class,
  GHS can therefore separate curves in exponent only when n grows with N. That is the
  "weak fields" regime of Menezes–Teske–Weng (recalled).
* **A library discrepancy (should move into `src/`).** `ec_trapdoor::audit_curve` / `ghs_genus_for_type`
  differ from the formula derived in §2 on 1 248 of 1 500 sampled rows at N = 12 (sum of
  the mismatch rows below). Examples: genus 0 for b ∈ F_2 (it should be 1); genus 7 at n = 4
  (impossible, since x⁴ + 1 = (x + 1)⁴); m' reported where m = m' + 1. The √b-orbit rank
  `magic_number` itself agrees (1 500/1 500).

## 1. Question, hypothesis, falsifier

Round 4 (§17.7, Prop. 9) gives oracle-DLP ≤ min over the class. Over F_{2^N} with composite N
an isogeny class can contain curves to which GHS applies and curves to which it does not
(Gaudry–Hess–Smart 2002; Galbraith–Hess–Smart, EUROCRYPT 2002, "Extending the GHS Weil
descent attack", which uses isogenies to reach weak curves). **Hypothesis:** for composite N
the weak share is small but nonzero in many classes, and classes with a weak curve are
reachable by short walks. **Falsifier:** at toy sizes, weak curves are absent from mixed
classes or unreachable.

## 2. Magic number and genus (derived; stated exactly)

E_{a,b}: y² + xy = x³ + ax² + b over K = F_{q^n}, q = 2^l, σ = (x ↦ x^q), c_i = σ^i(√b).
Divide by x² and put w = y/x + √b/x. This gives w² + w = x + a + √b/x. The GHS function field
is the compositum over K(x) of the Artin–Schreier extensions w_i² + w_i = x + σ^i(a) + c_i/x.
For a ∈ k (or any representative of a in the same Tr class that σ fixes up to ℘), these are
classified by U = Span_{F_2}{(c_i, 1)} ⊂ K × F_2:

    m = dim U,   m' = dim Span{c_i} = deg f,   f = Ord_σ(√b) | x^n − 1
    m = m' + [(0,1) ∈ U] = m' + [f(1) ≠ 0]                      (MQ form, recalled)
    g = Σ_{u ∈ U∖0} g(F_u),  g(F_u) = #poles − 1 = [c_u ≠ 0 and d_u ≠ 0]
      = 2^{m−1} − [(0,1) ∈ U]

So **g = 2^{m−1} − 1 iff (x + 1) ∤ f** (an odd-size subset of the c_i sums to 0); otherwise
**g = 2^{m−1}**. The genus sum over the quadratic subfields is the elementary-abelian genus
formula (Garcia–Stichtenoth, recalled). The example computes m from U directly, finds f from
the first linear dependency, and checks m = m' + f(1) on every b and every factorisation:
0 mismatches out of all 2 858 996 × (factorisations) evaluations (measured).

Applicability (Menezes–Qu condition, recalled): n odd, or m = n, or Tr_{K/F_2}(a) = 0. b must
not lie in a proper intermediate field F_{q^d}, d < n; those subfield curves are excluded
from every "weak" count and counted separately. Every isogeny class is one twist: t ≡ 1 mod 4
for a = 0 and t ≡ 3 mod 4 for a = ω, since #E_{0,b} ≡ 0 mod 4. So for n even, whole twist
classes lose the descent unless m = n (`twist_excluded_n_even` in `census.json`).

Hand-checked cases (asserted in `selftest`, N = 12):

* b = 1 at every factorisation: m = 1, g = 1 (E descends to itself over k).
* b ∈ F_16 at (4, 3): m = 1, g = 1.
* √b = c + σ(c) at (4, 3): f = x² + x + 1, m' = 2, m = 3, g = 3.
* Generic b: (4, 3) gives m = 3, g = 4; (6, 2) gives m = 2, g = 2; (3, 4) gives m = 4, g = 8.
* Squaring b preserves m: 1 000/1 000.

The (separable) 2-isogeny satisfies Φ₂ ≡ (X² − Y)(X − Y²) mod 2 (Kronecker, recalled). It
goes to a Galois conjugate, so **2-isogenies never change m** (derived; measured invariance
above). The walk therefore uses odd ℓ only.

Library comparison at N = 12, 300 random b (measured; `selftest.json`):
| library `audit_curve` (m, g) vs this lead (m, g) | rows |
|--|--|
| lib(m=1,g=0) ours(m=1,g=1) | 11 |
| lib(m=10,g=512) ours(m=10,g=512) | 57 |
| lib(m=11,g=1024) ours(m=11,g=1024) | 61 |
| lib(m=12,g=2047) ours(m=12,g=2048) | 117 |
| lib(m=2,g=1) ours(m=2,g=2) | 297 |
| lib(m=2,g=2) ours(m=3,g=3) | 18 |
| lib(m=3,g=3) ours(m=3,g=4) | 293 |
| lib(m=3,g=4) ours(m=3,g=4) | 24 |
| lib(m=4,g=7) ours(m=4,g=8) | 268 |
| lib(m=4,g=8) ours(m=4,g=8) | 19 |
| lib(m=4,g=8) ours(m=5,g=15) | 12 |
| lib(m=5,g=16) ours(m=5,g=16) | 49 |
| lib(m=6,g=31) ours(m=6,g=32) | 220 |
| lib(m=6,g=32) ours(m=6,g=32) | 3 |
| lib(m=7,g=64) ours(m=7,g=64) | 6 |
| lib(m=8,g=128) ours(m=8,g=128) | 10 |
| lib(m=8,g=128) ours(m=9,g=255) | 12 |
| lib(m=9,g=256) ours(m=9,g=256) | 23 |

## 3. Point counts, classes and cost models

**Counting (derived, validated).** #E_{a,b} = 2^N + 1 + (−1)^{Tr a}(W(√b) − 1) with
W(c) = Σ_u (−1)^{Tr(1/u) + Tr(cu)}. One Walsh–Hadamard transform of u ↦ (−1)^{Tr(1/u)} gives
W at every c, so a full census costs O(N·2^N). Validation, measured: 1 979/1 979 against a naive
count for N = 6..10 (all b, both twists); 80/80 against `ic_boundary::binary_point_count` at
N = 12, 16.

**Cost models.** r is the largest prime factor of #E and rho costs √(πr/4). Transfer is
possible only if log₂ r ≤ 2g·log₂(√q + 1) (Hasse–Weil; a necessary condition, used as the
transfer test). A curve is weak at (l, n) if the descent applies, transfer is possible, and
the model's IC cost is below rho; over the class it is weak if this holds at any factorisation.

* **M1** (asked-for, constant-free Gaudry/GTTD double-large-prime exponent, recalled): q^{2−2/g}.
* **M2**: g!·q^{2−2/g} (g! is the splitting-probability factor, recalled).
* **P** (concrete): min(M2, L_{q^g}[1/2, √2]) (Enge–Gaudry subexponential, constant recalled; o(1) dropped).
* **LG** (structural): applicable, transfer possible, n ≥ 3 and g ≤ 2n. When g = O(n), the
  Jacobian group has size 2^{O(N)} and the subexponential algorithms are asymptotically below
  2^{N/2}. The walk target **LG5** also requires n ≥ 5.
* **A** (exponent at fixed n, q → ∞): g ≥ n and 2 − 2/g < n/2.

M1 was the model planned first. P, LG and the walk targets were added **after** M1 and A came
out degenerate in the first census. They are reported as post-hoc definitions.

### Per factorisation (exhaustive; measured counts of curves, both twists)

| N | l | n | m values (count) | applicable | LG (g≤2n) | M1 | M2 | P | A | classes w/ LG / mixed | median LG share in mixed |
|--|--|--|--|--|--|--|--|--|--|--|--|
| 12 | 1 | 12 | 1:2, 2:4, 3:20, 4:28, 5:72, 6:96, 7:288, 8:384, 9:1152, 1… | 5556 | 12 | 4464 | 0 | 0 | 5556 | 1 / 1 | 0.167 |
| 12 | 2 | 6 | 1:6, 2:24, 3:120, 4:360, 5:1920, 6:5760 | 6900 | 180 | 2436 | 0 | 0 | 6900 | 14 / 14 | 0.25 |
| 12 | 3 | 4 | 1:14, 2:112, 3:896, 4:7168 | 7616 | 7616 | 1020 | 0 | 0 | 7616 | 126 / 32 | 0.75 |
| 12 | 4 | 3 | 1:30, 3:8160 | 8160 | 8160 | 48 | 0 | 0 | 510 | 128 / 8 | 0.958 |
| 12 | 6 | 2 | 1:126, 2:8064 | 8064 | 0 | 0 | 0 | 0 | 0 | 0 / 0 |  |
| 14 | 1 | 14 | 1:2, 2:4, 4:56, 5:56, 7:644, 8:644, 10:3136, 11:3136, 13:… | 22526 | 28 | 20174 | 0 | 0 | 22526 | 2 / 2 | 0.167 |
| 14 | 2 | 7 | 1:6, 4:1008, 7:31752 | 32760 | 1008 | 15190 | 0 | 0 | 32760 | 70 / 70 | 0.111 |
| 14 | 7 | 2 | 1:254, 2:32512 | 32512 | 0 | 0 | 0 | 0 | 0 | 0 / 0 |  |
| 15 | 1 | 15 | 1:2, 3:12, 5:180, 7:540, 9:2700, 11:8100, 13:13500, 15:40500 | 65460 | 120 | 61185 | 0 | 0 | 65460 | 8 / 8 | 0.059 |
| 15 | 3 | 5 | 1:14, 5:65520 | 65520 | 0 | 13215 | 0 | 0 | 65520 | 0 / 0 |  |
| 15 | 5 | 3 | 1:62, 3:65472 | 65472 | 65472 | 150 | 0 | 0 | 2046 | 362 / 12 | 0.968 |
| 16 | 1 | 16 | 1:2, 2:4, 3:8, 4:16, 5:32, 6:64, 7:128, 8:256, 9:512, 10:… | 98048 | 0 | 94208 | 0 | 0 | 98048 | 0 / 0 |  |
| 16 | 2 | 8 | 1:6, 2:24, 3:96, 4:384, 5:1536, 6:6144, 7:24576, 8:98304 | 114432 | 768 | 68448 | 0 | 0 | 114432 | 40 / 40 | 0.062 |
| 16 | 4 | 4 | 1:30, 2:480, 3:7680, 4:122880 | 126720 | 126720 | 10784 | 0 | 0 | 126720 | 510 / 126 | 0.875 |
| 16 | 8 | 2 | 1:510, 2:130560 | 130560 | 0 | 0 | 0 | 0 | 0 | 0 / 0 |  |
| 18 | 1 | 18 | 1:2, 2:4, 3:12, 4:12, 5:48, 6:48, 7:252, 8:252, 9:756, 10… | 358344 | 0 | 349434 | 0 | 0 | 358344 | 0 / 0 |  |
| 18 | 2 | 9 | 1:6, 3:120, 7:32760, 9:491400 | 524160 | 0 | 378675 | 0 | 0 | 524160 | 0 / 0 |  |
| 18 | 3 | 6 | 1:14, 2:112, 3:1008, 4:7056, 5:64512, 6:451584 | 487368 | 3528 | 178290 | 0 | 0 | 487368 | 160 / 160 | 0.036 |
| 18 | 6 | 3 | 1:126, 3:524160 | 524160 | 524160 | 612 | 0 | 0 | 8190 | 1024 / 16 | 0.989 |
| 18 | 9 | 2 | 1:1022, 2:523264 | 523264 | 0 | 0 | 0 | 0 | 0 | 0 / 0 |  |
| 20 | 1 | 20 | 1:2, 2:4, 3:8, 4:16, 5:60, 6:60, 7:120, 8:240, 9:960, 10:… | 1539060 | 0 | 1522660 | 0 | 0 | 1539060 | 0 / 0 |  |
| 20 | 2 | 10 | 1:6, 2:24, 5:2040, 6:6120, 9:522240, 10:1566720 | 1830900 | 0 | 1476100 | 0 | 0 | 1830900 | 0 / 0 |  |
| 20 | 4 | 5 | 1:30, 5:2097120 | 2097120 | 0 | 483575 | 0 | 0 | 2097120 | 0 / 0 |  |
| 20 | 5 | 4 | 1:62, 2:1984, 3:63488, 4:2031616 | 2063360 | 2063360 | 209440 | 0 | 0 | 2063360 | 2046 / 492 | 0.94 |
| 20 | 10 | 2 | 1:2046, 2:2095104 | 2095104 | 0 | 0 | 0 | 0 | 0 | 0 / 0 |  |


Readings (derived):

* **n = 2:** g ≤ 2, never weak.
* **n = 3, 4:** g ≤ 8 for every curve, so LG covers almost the whole field, and the classes
  are uniform up to subfield and twist exclusions.
* **n ≥ 5:** LG curves exist only when x^n − 1 has a small σ-invariant factor reachable by √b:
  (1, 12), (2, 6), (1, 14), (2, 7), (1, 15), (2, 8) and (3, 6). They are absent at (3, 5)
  (m ∈ {1, 5}, since x⁵ + 1 = (x + 1)(x⁴ + … + 1)), at (4, 5) and (2, 10), and at every
  l = 1 row from N = 16 up.

### Over all factorisations

| N | curves | classes | any-factorisation M1 | M2 | P | LG | A | best class P margin (bits) | median |
|--|--|--|--|--|--|--|--|--|--|
| 12 | 8190 | 128 | 5328 | 0 | 0 | 8180 | 7886 | 1.652 | 3.625 |
| 14 | 32766 | 256 | 25354 | 0 | 0 | 1008 | 32760 | 1.663 | 4.029 |
| 15 | 65534 | 362 | 61185 | 0 | 0 | 65472 | 65526 | 2.418 | 6.989 |
| 16 | 131070 | 512 | 107216 | 0 | 0 | 126720 | 126720 | 1.669 | 4.388 |
| 18 | 524286 | 1024 | 479295 | 0 | 0 | 524160 | 524160 | 1.671 | 4.836 |
| 20 | 2097150 | 2048 | 1849355 | 0 | 0 | 2063360 | 2097140 | 1.673 | 5.191 |


The best-class P margin sits at about +1.67 bits whenever N is even. That comes from the
n = 2, g = 2 route (cost 2·q against rho ≈ 0.89·q), which applies to almost every curve. So
under P the classes are flat at even N: the median class has spread 0. At N = 15 there is no
n = 2 route, and the margin spread inside classes is real (the BEST walk target, §4).
Figure: `fig2_model_p_margin.svg`.

## 4. Isogeny walk (ℓ ∈ {3, 5, 7})

**Method.** Elkies kernels come from factoring the division polynomial ψ_ℓ over K. Roots in K
(their doubling orbits under x ↦ x² + b/x²) give the split kernels, and irreducible factors of
degree (ℓ − 1)/2 that are stable under doubling give the irreducible ones. The codomain comes
from binary Vélu, b' = b + t + t², as in `binary_velu::velu_codomain`. The graph is built on
Galois-orbit representatives. That is exact for distances, because every target set is
Galois-stable.

**Validation (measured).**

* ψ₃, ψ₅, ψ₇ match `binary_velu::division_polynomial` (30/30).
* ℓ = 3 neighbours match the Φ₃ roots from `binary_isogeny` (12/12).
* Codomain trace mismatches: 0. Asymmetric edges: 0. Kernel-orbit errors: 0 (all N).
* Kernel counts per (curve, ℓ) lie only in {0, 1, 2, ℓ + 1} (`kernel_count_hist`).
* Mean cost is 13 800–27 700 counted F_{2^N} multiplications per (curve, ℓ) kernel search.

**Search.** For up to 20 random strong orbit representatives per mixed class, BFS runs until
it hits a target. "Expanded" counts the curves whose neighbours were computed (each costs 3
kernel searches). "Path" counts the isogeny evaluations needed to transport the DLP.

| N | target | mixed classes | strong curves | unreachable | components (w/o target) | distance hist (curves) | search starts found/failed | mean (max) curves expanded | mean (max) path |
|--|--|--|--|--|--|--|--|--|--|
| 12 | LG5 | 14 | 780 | 144 | 19 (5) | 1:432, 2:192, 3:12 | 53/12 | 1.49 (5) | 1.32 (3) |
| 12 | BEST | 16 | 114 | 28 | 20 (4) | 1:86 | 18/6 | 1.0 (1) | 1.0 (1) |
| 12 | M1 | 26 | 460 | 62 | 34 (4) | 1:397, 2:1 | 40/7 | 1.0 (1) | 1.0 (1) |
| 14 | LG5 | 70 | 8486 | 2002 | 130 (58) | 1:2494, 2:2646, 3:1204, 4:140 | 457/151 | 2.63 (13) | 1.8 (4) |
| 14 | BEST | 22 | 240 | 72 | 30 (8) | 1:168 | 24/12 | 1.0 (1) | 1.0 (1) |
| 14 | M1 | 35 | 1224 | 152 | 48 (7) | 1:890, 2:56, 3:84, 4:42 | 79/17 | 1.76 (9) | 1.28 (3) |
| 15 | LG5 | 8 | 1650 | 0 | 8 (0) | 1:420, 2:660, 3:450, 4:120 | 110/0 | 4.41 (11) | 2.22 (4) |
| 15 | BEST | 106 | 18242 | 2142 | 236 (102) | 1:5590, 2:5560, 3:2640, 4:1290, 5:540, 6:210, 7:90, 8:60, 9:60, 10:60 | 1007/151 | 3.99 (21) | 2.3 (10) |
| 15 | M1 | 15 | 69 | 54 | 51 (12) | 1:15 | 3/14 | 1.0 (1) | 1.0 (1) |
| 16 | LG5 | 40 | 11208 | 3552 | 105 (64) | 1:1968, 2:2744, 3:2032, 4:752, 5:112, 6:32, 7:16 | 381/216 | 4.43 (24) | 2.19 (6) |
| 16 | BEST | 30 | 478 | 214 | 64 (14) | 1:264 | 34/32 | 1.0 (1) | 1.0 (1) |
| 16 | M1 | 98 | 8764 | 1728 | 282 (55) | 1:7036 | 450/121 | 1.0 (1) | 1.0 (1) |
| 18 | LG5 | 160 | 95136 | 32571 | 608 (437) | 1:10422, 2:17664, 3:16587, 4:9234, 5:4284, 6:1782, 7:900, 8:648, 9:414, 10:234, 11:180, 12:108, 13:36, 14:36, 15:36 | 1836/1262 | 8.35 (53) | 2.97 (15) |
| 18 | BEST | 44 | 986 | 348 | 140 (22) | 1:638 | 74/40 | 1.0 (1) | 1.0 (1) |
| 18 | M1 | 78 | 8277 | 584 | 227 (23) | 1:7329, 2:292, 3:72 | 452/43 | 1.09 (5) | 1.07 (3) |
| 20 | LG5 | 0 | 0 | 0 | 0 (0) |  | 0/0 |  () |  () |
| 20 | BEST | 64 | 1986 | 934 | 310 (28) | 1:1052 | 112/96 | 1.0 (1) | 1.0 (1) |
| 20 | M1 | 431 | 151623 | 40715 | 2981 (626) | 1:110628, 2:280 | 3747/1632 | 1.0 (2) | 1.0 (2) |


For comparison, the median class-size-to-LG5-count ratio is 5, 9, 17, 16 and 28 at N = 12,
14, 15, 16 and 18 (derived). That is the expected number of draws if one could sample the class
uniformly, and the walk needs fewer expansions on average. Unreachable starts lie in ℓ ≤ 7
components without a target. These components come from different endomorphism-ring levels
and from ℓ ∈ {3, 5, 7} not generating the class group; the components were not analysed
further. Figure: `fig3_walk_distance.svg`.

## 5. Commands

```sh
export CARGO_TARGET_DIR=$PWD/target CARGO_BUILD_JOBS=2
cargo build --release --example r5_2_ghs_class
D=research/isogeny_fibers_20261010/round5/r5_2_ghs_class
target/release/examples/r5_2_ghs_class selftest --out $D/selftest.json
target/release/examples/r5_2_ghs_class census 12 14 15 16 18 20 --out $D/census.json    # ~35 s contended
target/release/examples/r5_2_ghs_class walk 12 14 15 16 18 --samples 20 --out $D/walk.json  # ~25 s contended
target/release/examples/r5_2_ghs_class walk 20 --samples 20 --out $D/walk_20.json        # ~80 s contended
$VENV/bin/python $D/plot.py      # figures + figure_data.tsv from the JSON only
$VENV/bin/python -c "import typst; typst.compile('$D/report.typ', output='$D/report.pdf')"
```

All runs are deterministic: the seeds are fixed, and a rerun reproduced `selftest.json` and
`walk.json` (N = 12, 14) byte for byte or value for value.

## 6. Limits

* No descent was constructed and no Jacobian order was computed. "Transfer" is the
  Hasse–Weil size test, and r | #J_C(k) is assumed, not checked.
* The Menezes–Qu applicability condition, the genus-sum formula, the IC exponents and the
  constants g! and √2 are recalled. The cost models are models, with o(1) and poly-log
  factors dropped.
* Toy N ≤ 20, exhaustive. N = 24 and sampling at larger N were not run.
* Only ℓ ∈ {3, 5, 7}; no ℓ = 11, 13 and no volcano/conductor analysis of the unreachable
  components. The attacker search samples ≤ 20 starts per class.
* Hess's generalised descent (more general a, non-hyperelliptic covers) is not modelled, and
  neither are Galbraith–Hess–Smart's isogeny-then-descent costs beyond the walk length.

## 7. Next step

Push the LG census to N ∈ {21, 24, 28} with l ∈ {1, 2, 3} (sampled b), where the n ≥ 5 rows
grow and the L(1/2) model starts to cross rho. Then construct the descent for one LG curve per
class at N = 14 and compute #J_C(k), to replace the transfer heuristic with a check. Finally,
fix `ec_trapdoor`'s genus rule (§0) and add a `binary_velu` Elkies-kernel enumerator for
odd ℓ ≤ 7 (the `rational_kernels` code in the example) to the library.

## Files

`census.json` (per-N, per-factorisation counts and per-class rows; the schema is in the
file), `walk.json`, `walk_20.json`, `selftest.json`, `plot.py`, `figure_data.tsv`,
`fig1_low_genus_share.svg`, `fig2_model_p_margin.svg`, `fig3_walk_distance.svg`,
`report.typ`, `report.pdf`.
