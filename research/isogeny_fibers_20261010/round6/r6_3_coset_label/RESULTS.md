# R6.3 — the trace pair as a label of E/4E, and a certificate for the 4E identity

Curve: y² + xy = x³ + 1 over F_{2^n}, n odd (toy fields n = 5..25; sampled
fields of the standard Koblitz curves sect233k1, sect239k1, sect283k1,
sect409k1, sect571k1 and the ECC2K-130 curve at n = 131). No security claim
about any deployed curve; this is a labelling/test-cost result only.

## Question
Is L(P) = (Tr(x), Tr(y + x·H(x))) (L(O) = (0,0)) a well-defined map
E/4E → F_2², and what decides the swap of the odd-coset labels between
n = 131 and n = 233 seen in round 4 (16_2_fast_4e)?

## Hypothesis / falsifier (PROGRAM.md R6.3)
Bit 1 is E → E/2E; bit 2 is constant on each coset of 4E, is 2E → 2E/4E on
2E, and on the odd cosets is a fixed function of n mod 8. Falsifier: any odd
n ≤ 25 with a non-constant coset, or an odd-coset labelling not periodic in
n mod 8 (or mod 4, or constant).

## Commands (from this directory; binary built with the shared r6target)
```
B=$CARGO_TARGET_DIR/release/examples/r6_3_coset_label
$B exhaustive --nmin 5 --nmax 25 --out exhaustive.json      > exhaustive.txt
$B sample --ns 131,233,239,283,409,571 --quads 2000 --uniform 2000 --seed 63 --out sample.json > sample.txt
$B bench  --ns 131,233,239,283,409,571 --batch 1024 --reps 15 --inner 8 --seed 64 --out bench.json > bench.txt
$B cert --outdir .                                           > cert.txt
venv/bin/python plot.py   # labels.svg, cost.svg, plot_data.json
```
Cosets are read from [r]P, r = #E/4 (odd), compared with the order-4 point
P4 = (1,0) and T = (0,1); k(P) = j·r mod 4 when [r]P = j·P4.

## Results (measured)
Exhaustive: every point of E(F_{2^n}) for all 11 odd n in 5..25,
44,740,508 points in total (n = 25: 33,558,844 points, 333 s contended).
Point counts match the Frobenius formula at every n. Sampled: 10,000 points
per n (min 2,459 per coset) at n ∈ {131, 233, 239, 283, 409, 571}.

| n | n mod 8 | k=0 (4E) | k=1 (P4+4E) | k=2 (T+4E) | k=3 (−P4+4E) | points |
|---|---|---|---|---|---|---|
| 5 | 5 | 00 | 11 | 01 | 10 | 44 (all) |
| 7 | 7 | 00 | 10 | 01 | 11 | 116 (all) |
| 9 | 1 | 00 | 11 | 01 | 10 | 508 (all) |
| 11 | 3 | 00 | 10 | 01 | 11 | 2,116 (all) |
| 13 | 5 | 00 | 11 | 01 | 10 | 8,012 (all) |
| 15 | 7 | 00 | 10 | 01 | 11 | 33,044 (all) |
| 17 | 1 | 00 | 11 | 01 | 10 | 130,972 (all) |
| 19 | 3 | 00 | 10 | 01 | 11 | 523,492 (all) |
| 21 | 5 | 00 | 11 | 01 | 10 | 2,099,948 (all) |
| 23 | 7 | 00 | 10 | 01 | 11 | 8,383,412 (all) |
| 25 | 1 | 00 | 11 | 01 | 10 | 33,558,844 (all) |
| 131 | 3 | 00 | 10 | 01 | 11 | 10,000 (sampled) |
| 233 | 1 | 00 | 11 | 01 | 10 | 10,000 (sampled) |
| 239 | 7 | 00 | 10 | 01 | 11 | 10,000 (sampled) |
| 283 | 3 | 00 | 10 | 01 | 11 | 10,000 (sampled) |
| 409 | 1 | 00 | 11 | 01 | 10 | 10,000 (sampled) |
| 571 | 3 | 00 | 10 | 01 | 11 | 10,000 (sampled) |

* The pair is constant on every coset at every n (0 exceptions).
* The odd-coset labelling depends on **n mod 4** only (period 4, a fortiori
  period 8): n ≡ 1 → (k=1: 11, k=3: 10); n ≡ 3 → (k=1: 10, k=3: 11). This
  explains the round-4 swap: 131 ≡ 3, 233 ≡ 1 (mod 4).
* Cross-checks, all 0 disagreements: library `BinaryCurve::is_in_4e` vs
  scalar-multiplication truth; table vs both constant-time variants; three
  half-trace implementations; label via library vs example field; 218,978
  off-curve inputs; proof step (iii) on 22,370,232 points.

**Verdict: hypothesis confirmed, falsifier did not fire** (refined: the
dependence is on n mod 4, not just n mod 8). Proved below.

## Lemma R6.3 (full E/4E label; extends Prop. 14.2)
On y² + xy = x³ + 1 over F_{2^n}, n odd, let P4 = (1,0), T = 2P4 = (0,1),
k(P) ∈ Z/4 the class of P in E/4E = ⟨P4 + 4E⟩, b(P) = Tr(y) + Tr(x·H(x)).
Then L(P) = (Tr(x), b(P)) (L(O) = (0,0)) depends only on k(P):

    k=0 → (0,0),  k=2 → (0,1),  k=1 → (1, (n+1)/2 mod 2),  k=3 → (1, (n−1)/2 mod 2).

*Proof.* (i) Tr(x) = 0 ⟺ P ∈ 2E (halving: λ² + λ = x solvable), so bit 1
is the homomorphism E → E/2E ≅ F_2. (ii) On 2E, Prop. 14.2 gives
P ∈ 4E ⟺ b(P) = 0; T gives b = Tr(1) = 1. (iii) Odd cosets, Tr(x) = 1,
P ≠ ±P4. Put l = y/(x+1) (slope of P, P4), Q = P + P4,
x_Q = l² + l + x + 1, y_Q = l(x + x_Q) + x_Q + y. Let h = H(x); since
Tr(x) = 1, h² + h = x + 1, so x_Q = m² + m with m = l + h, Tr(x_Q) = 0
and H(x_Q) = m + Tr(m) (from H(c²) = H(c) + c + Tr(c)). Using Tr(z²) = Tr(z):
b(Q) = Tr(lx) + Tr(l²h) + Tr(lh²) + Tr(h³) + Tr(h) and
b(P) = Tr(lx) + Tr(l) + Tr(xh), with Tr(xh) = Tr(h³) from x = h² + h + 1.
The curve equation in the form (certified, `certificate_label_core.json`)
x(l² + l) + l = g² + g + 1, g = x + l, gives after taking traces and
substituting x = h² + h + 1: Tr(hl²) + Tr(h²l) + Tr(l) = 1. Hence
b(P) + b(Q) = 1 + Tr(h). For n odd and Tr(x) = 1,
Tr(H(x)) = ((n+1)/2)·Tr(x) = (n+1)/2 mod 2, so
**b(P) = b(P + P4) + (n−1)/2 (mod 2)** (measured 0 violations, 22.4 M points).
If k(P) = 3, Q ∈ 4E, b(Q) = 0, b(P) = (n−1)/2; if k(P) = 1, Q ∈ T + 4E,
b(Q) = 1, b(P) = (n+1)/2. The points ±P4 = (1,0), (1,1) give
b = Tr(H(1)) = (n+1)/2 and 1 + (n+1)/2, the same values. ∎

*Not a homomorphism.* E/4E ≅ Z/4 while F_2² has exponent 2: any additive
map Z/4 → F_2² sends 2·(generator) to 0, so it cannot separate 4E from
T + 4E. L is injective on E/4E, so it is a set bijection, not a group map.
Bit 1 alone is a homomorphism (onto E/2E); bit 2 is one only on 2E.

## Certificates (task 4)
The repository schema (`src/cryptanalysis/identity_certificate.rs`,
`identity.certificate/v1`) certifies **polynomial identities** over Z with
an integer witness (lhs = rhs + C·curve + 2K). The 4E statement itself is a
Boolean trace identity (Tr is F_2-linear, not a polynomial of bounded degree
in x, y in this calculus), so it does not fit. Emitted instead, the closest
well-formed records, the two polynomial cores of the proof, both accepted by
the library checker (also re-read from disk and re-checked with 64 extra
points):
* `certificate_halving_step.json` (IDC1h69fde95f7fb937ab): the Prop. 14.2
  halving step F(l²+l, u²+(l+1)(l²+l)) = F(u, u(l+u)) in char 2;
  Accept, 32 points, false-accept ≤ 2^−1869.
* `certificate_label_core.json` (IDC1h61e40c496fd9c2cf): step (iii) core
  (x+1)²(xl²+xl+l) = (x+1)²(g²+g+1) modulo 2 and the curve; Accept,
  false-accept ≤ 2^−1888.
* `certificates_check.json`: checker verdicts.
The trace steps (Tr(H(x)) = (n+1)/2·Tr(x), Tr(z²) = Tr(z)) are standard
and sit outside the certificate; the exhaustive enumeration covers them for
n ≤ 25.

## Constant-time is_in_4e (task 5)
Two variants in the example: `ct_mask` (masked sum over all n basis images
H(z^i)) and `ct_fong` (masked even-bit folding H(c²) = H(c)+c+Tr(c), then
masked sum over odd basis images; recalled: Fong–Hankerson–López–Menezes,
IEEE TC 2004). Branch-free trace (masked popcount of x & tmask), fixed-fold
reduction, no secret-indexed table. Both agree with the table version on all
44.7 M exhaustive points and all sampled points.

Members of 4E (no early exit), wall per call, median, **contended** (shared
4-core host), and counted word operations per call:

| n | table ns | ct_mask ns (×) | ct_fong ns (×) | library ns | words tab / mask / fong |
|---|---|---|---|---|---|
| 131 | 385 | 1078 (2.80) | 1024 (2.66) | 833 | 1016 / 2072 / 2329 |
| 233 | 360 | 1795 (4.98) | 1347 (3.74) | 1088 | 1156 / 3465 / 3692 |
| 239 | 462 | 1973 (4.27) | 1457 (3.16) | 1151 | 1379 / 3751 / 3984 |
| 283 | 749 | 2034 (2.72) | 2142 (2.86) | 1297 | 1915 / 5248 / 5241 |
| 409 | 1012 | 3614 (3.57) | 2508 (2.48) | 1831 | 2363 / 8710 / 7883 |
| 571 | 1643 | 5020 (3.06) | 4075 (2.48) | 2769 | 4129 / 15128 / 12833 |

**The constant-time variant does not meet the < 2× bar**: 2.5–5.0× in
contended wall and 2.0–3.7× in counted word ops against this example's
table version. Against the library's own `is_in_4e` (slower than the
example's table version) the best CT variant is 1.23–1.57× in wall. The cost
is the half-trace: the masked basis sum touches n·W words regardless of
input, while the table skips zero bytes. Per the program rule, no library PR
is proposed as a default; a CT variant could still be offered as an opt-in
API where side channels matter.

## Graphs
`labels.svg` (label per coset vs n; exhaustive left, sampled right),
`cost.svg` (wall contended and counted word ops), data in `plot_data.json`.

## Limits
Toy fields exhaustive only to n = 25; large n sampled (10,000 points each).
Wall times contended. Certificates cover polynomial cores only, not the trace
statements. One curve (a = 0, b = 1); the a = 1 curve was not run.

## Next step
Extend the lemma to y² + xy = x³ + x² + 1 (a = 1, E/2E trivial on the
2-part differently) and to E/8E; if a CT 4E test is wanted in `src/`, try a
normal-basis or bit-sliced half-trace to bring the ratio under 2×.
