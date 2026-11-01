# R7.7 — the trace pair as a coset label for a = 1 and for E/8E

Curves: y² + xy = x³ + a·x² + b over F_{2^n}, n odd, b ≠ 0 (all a, all b at
n = 5..9; a ∈ {0, 1}, all b at n = 5..13; random a, b at n = 17..23). Toy
fields only. No claim about any deployed curve; this is a labelling and
test-cost result.

Notation. T = (0, √b) is the unique point of order 2. H is the half-trace
(H² + H = c + Tr(c) for n odd). v = v₂(#E). For a point P = (x, y):
bit1(P) = Tr(x) + Tr(a), b₂(P) = Tr(y) + Tr(x·H(x + a)) (for a = 0 this is the
R6.3 pair). k is the class of P in E/2^J E ≅ Z/2^J, counted from a reference
odd point Q0 (k(Q0) = 1).

## Hypotheses, falsifiers, verdicts

**H1 (as posed).** For a = 1 (Tr(a) = 1) "E/4E has the structure R6.3 describes"
and the trace pair is a full label of it, with the label rule set by n mod 4.
*Falsifier:* a curve/point where the predicted label is wrong, or a collision
between distinct cosets.

**Verdict H1: refuted as posed; the premise is false. Corrected statement
confirmed.** For Tr(a) = 1 the group has v = 1 (#E ≡ 2 mod 4), 2E has odd order
and E/4E = E/8E = E/2E ≅ Z/2. There is no Z/4 to label. What holds, and what
was tested (prediction written in the example's header before the first run):

* P1 (coset rule): the coset of P is bit1(P) = Tr(x) + Tr(a); **0 mispredictions
  in 148,598,015 labelled points** (exhaustive, all-a and random runs below).
* P1′ (the pair is not a coset label): b₂(−P) = b₂(P) + Tr(x). For P ∈ 2E
  (Tr(x) = 1, x ≠ 0) the points P and −P lie in the same coset and have
  different b₂. So the pair is never constant on 2E. **Measured: the pair is
  non-constant on 21,813 of 21,814 cosets (exhaustive) and 11,990 of 11,990
  (random).** The constant cosets (1 of 21,814 exhaustive; 16 of 278,880 in the all-a run)
  all occur at n = 5 and are non-2E cosets of 11 to 15 points, where P and −P
  share b₂ (Tr(x) = 0) and few independent bits remain, a chance outcome; a
  coset of 2E is never constant (it contains a point with x ≠ 0 and its negative). Collisions between distinct cosets: **0** (bit1
  separates them).
* P2 (what b₂ does for a = 1): for P ∈ 2E the root l = H(x + a) + b₂(P) gives
  the half of P that lies in the odd-order subgroup 2E. **0 failures in
  35,780,448 + 33,947,136 + 4,495,324 points** (exhaustive, all-a, random).

**H1′ (the live extension of R6.3).** For every curve with Tr(a) = 0 (any a,
any b), E/4E ≅ Z/4 and L(P) = (Tr(x), b₂(P)) is a full label: constant on each
coset of 4E, (0,0) on 4E, (0,1) on the class 2, and the odd cosets carry
(1, β) and (1, 1 + β). For v = 2, P₄ = (b^{1/4}, b^{1/4}(H(a) + b^{1/4} + 1)) is
the canonical odd generator and β = b₂(P₄) = **Tr(b·H(b))** (proved below).
For b = 1, a = 0 this is Tr(H(1)) = (n+1)/2 mod 2, which is R6.3's rule.
*Falsifier:* any coset with two labels, two cosets with one label, or a
point whose label differs from the table.
**Verdict H1′: not fired; confirmed** (constancy measured, closed form for β
and the 4E criterion proved; constancy for general (a, b) is not proved, only
the R6.3 case (0,1) is).

**H2.** A refined label (the trace pair plus the least extra datum) is a full
label of E/8E where E(F) has a Z/8 or Z/4 × Z/2 2-primary part.
*Falsifier:* a coset on which the refined label is not constant, a shared label,
a misprediction. Minimal counterexamples if no cheap complete label exists.

**Verdict H2: confirmed with a datum that is not cheap; Z/4 × Z/2 cannot
occur.**
* Z/4 × Z/2 is impossible for these curves: 2P = O ⟺ x = 0, so E(F)[2] = {O, T}
  and the 2-primary part is cyclic Z/2^v. (Proof: x(2P) = x² + b/x², and
  P = O or x(P) = 0 are the only points with 2P = O; T is the only point with
  x = 0.)
* The pair alone cannot label E/8E (four values, eight cosets). Smallest
  counterexample: n = 5, modulus 0x25, a = 0, b = 6, #E = 24, v = 3
  (`witness.json`): the cosets k = 0 and k = 4 both carry the pair (0,0), k = 2
  and 6 carry (0,1), k = 1 and 5 carry (1,0), k = 3 and 7 carry (1,1).
* Extra datum found: b₃(R) = b₂(½R) for even R (either half, because T ∈ 4E when
  v ≥ 3), and b₃(P) = b₃(P + Q0) for odd P. With it the triple (bit1, b₂, b₃) is a
  bijection E/8E → F₂³ with the table k = 0..7 → 000, 100, 010, 111, 001, 101,
  011, 110 (k from Q0, an odd point with b₂(Q0) = 0). The 8-division condition
  is the triple 000: P ∈ 8E ⟺ Tr(x) = Tr(a) = 0, b₂(P) = 0 and b₂(½P) = 0.
  **0 mispredictions, 0 non-constant cosets, 0 collisions** in 74,118,081 points
  at v ≥ 3 (exhaustive, all-a and random combined; table below).
* The same recursion gives labels of E/16E and E/32E (v ≥ 4, v ≥ 5) with the
  same result (37,628,863 and 18,181,938 points, 0 failures).
* No cheap datum for odd points: a search over 120 trace monomials (all
  products of at most three of x, y, H(x+a), H(b/x²), 1/x, y/x, H(H(x+a)),
  traced) found **no combination** that is constant on the odd cosets of 8E and
  separates k from k + 4 (480 pairs, 240 curves, n = 7..13). The same search
  at level 4E finds one (positive control: b₂). So the odd-point third bit costs
  a point addition (or arithmetic in F_{q⁴}).

**Cost verdict.** The pair costs one H, one M and three traces versus one
point addition (I + 3M + 2S) more for the halving route on odd points, and
about 60 M + 50 S + 30 I (n = 23) for the scalar-multiplication ground truth; E/8E
and higher give no counted-operation advantage over halving. The
constant-time variant of the Tr(a) = 0 pair still does **not** meet the
< 2× bar (not re-measured; see the CT section). For Tr(a) = 1 the label is one
trace and is constant-time by construction. Details below.

## Commands

```
rustc -O --edition 2021 examples/r7_7_coset_label.rs -o r7_7     # std only
sh run_all.sh ./r7_7         # from this directory: all of the below
./r7_7 selftest
./r7_7 exhaustive --nmin 5 --nmax 13 --out exhaustive.json
./r7_7 exhaustive --nmin 5 --nmax 9 --alla --out exhaustive_alla.json
./r7_7 random --ns 17,19,21,23 --curves 1000 --pts 1500 --seed 77 --out random.json
./r7_7 exhaustive --nmin 5 --nmax 7 --alla --control 1|2 ...      # negative controls
./r7_7 random --ns 17 --curves 200 --pts 600 --seed 79 --control 1 ...
./r7_7 cost --ns 17,19,21,23 --curves 60 --pts 400 --seed 78 --out cost.json
./r7_7 beta --nmin 5 --nmax 11 --out beta.json
./r7_7 witness --n 5 --out witness.json
./r7_7 search --ns 7,9,11,13 --curves 60 --pts 400 --level 3|2 --seed 81 ...
python3 plot.py              # fig1_labels.svg, fig2_evidence.svg, fig3_cost.svg, plot_data.json
```
The example is built with rustc and also with `cargo build --release --example`
from a minimal package that points at the file (the same binary output was
checked against the rustc build on n = 5..9, the witness and the search). The
sparse worktree cannot resolve the repository's path dependencies, so the full
crate was not built (memory/disk budget); `clippy-driver 1.98 -D warnings` and
`cargo +1.98 clippy --release --example … -D warnings` (minimal package) are
clean, rustfmt is clean. Arithmetic is a log/antilog table field local to the
example; `selftest` checks it against shift-and-add products, the half-trace
identity, associativity, group order and the 2E criterion on 4 curves at each of 7 field sizes.

## Results (measured)

Every point is classified by the ground truth: [r]P with r = #E/2^v, scaled by
2^{max(v−5,0)} and looked up in the multiples of [r]Q0 (a cyclic group of order
2^{min(v,5)}); coset sizes are checked equal in the exhaustive runs (0 failures).

| run | curves | points | v = 1 pts: bit1 wrong | pair non-const (v = 1) | J=2 pts / wrong / non-const / coll | J=3 | J=4 | J=5 |
|---|---|---|---|---|---|---|---|---|
| exhaustive n = 5..13, a ∈ {0,1}, all b | 21,814 | 143,143,616 | 0 of 71,571,803 | 21,813 of 21,814 | 71,571,813 / 0 / 0 / 0 | 35,774,997 / 0 / 0 / 0 | 17,600,531 / 0 / 0 / 0 | 8,972,942 / 0 / 0 / 0 |
| exhaustive n = 5..9, all (a, b) | 278,880 | 136,068,096 | 0 of 68,033,712 | 278,864 of 278,880 | 68,034,384 / 0 / 0 / 0 | 33,877,584 / 0 / 0 / 0 | 17,806,832 / 0 / 0 / 0 | 8,136,496 / 0 / 0 / 0 |
| random n = 17,19,21,23, 1000 curves × {a=1, a=0, a random} per n, 1500 pts | 12,000 | 18,000,000 | 0 of 8,992,500 | 11,990 of 11,990 | 9,007,500 / 0 / 0 / 0 | 4,465,500 / 0 / 0 / 0 | 2,221,500 / 0 / 0 / 0 | 1,072,500 / 0 / 0 / 0 |

(The n = 5..9 curves of the first and second run overlap; totals are per run, not
summed.) At n = 13, a = 1, all 8,191 curves and all 67,100,671 affine points were
labelled; a = 0: 4,096 curves with v = 2, 2,080 with v = 3, 988 with v = 4, 1,027
with v ≥ 5. The count of v = 2 curves equals the number of b with Tr(b) = 1 at
every n, as the structure lemma predicts (v ≥ 3 ⟺ Tr(b) = 0 when Tr(a) = 0).

Other checks, all with 0 failures in each run: closed form β = Tr(b·H(b)) on all
v = 2 curves (5,456 exhaustive, 69,888 all-a, 3,028 random); halving Q of
every even P satisfies 2Q = P; x-only forms Tr(b/x²) = bit1 (all points with
x ≠ 0) and b₂(P) = Tr(x·H(b/x²)) (even P, Tr(a) = 0). Root dependence of b₃
(the two halves of an even P): v ≥ 3, 0 dependent in 17,884,773 / 16,904,016 /
2,231,625 even points; v = 2, **all** 17,895,680 / 17,043,456 / 2,271,929 are
dependent, as predicted (T ∉ 4E so E/8E = E/4E, no third bit exists).
Probabilities of b₂ = 1 on v = 1 cosets: 0.49999 (k = 0) and 0.50003 (k = 1).

Negative controls (the test must fire on broken labels): label forgetting a
(`--control 1`, n = 5..7 all (a,b)): 515,444 of 1,056,336 v ≥ 2 points
mispredicted at J = 2, 258,096 root-selector failures; label dropping Tr(x·H)
(`--control 2`): 515,396 mispredicted at J = 2; non-constant cosets at the pair
level: 33,794 (control 1) and 34,354 (control 2) of 34,496; random n = 17, a ≠ 0, control 1: 27,385 of 174,600
mispredicted at J = 2. Real label: 0.

## Proofs

*Structure.* x(2P) = λ² + λ + a with λ = x + y/x, so P ∈ 2E ⟺ Tr(x + a) = 0 ⟺
Tr(x) = Tr(a); T = (0, √b) ∈ 2E ⟺ Tr(a) = 0. Hence v = 1 ⟺ Tr(a) = 1. For
Tr(a) = 0, the rational point P₄ of order 4 has x(2P₄) = x₄² + b/x₄² = 0, so
x₄⁴ = b and P₄ ∈ 2E ⟺ Tr(x₄) = Tr(b) = 0. So v = 2 ⟺ Tr(b) = 1 and v ≥ 3 ⟺
Tr(b) = 0.

*Halving formula.* If λ² + λ = x + a and Q has x(Q)² = y + (λ+1)x, then
2Q = P (Q = (x_Q, x_Q(λ + x_Q))). With λ = H(x+a) + c: Tr(x_Q) = Tr(x_Q²) =
b₂(P) + (c+1)·Tr(x). (A)

*Tr(a) = 1.* For P ∈ 2E, Tr(x) = 1, so (A) gives Tr(x_Q) = b₂(P) + c + 1, and
Q ∈ 2E ⟺ Tr(x_Q) = 1 ⟺ c = b₂(P): P2. Also b₂(−P) = Tr(y + x) + Tr(xH) =
b₂(P) + Tr(x): P1′.

*Tr(a) = 0, 4E criterion for every b and a.* For P ∈ 2E, Tr(x) = 0, so (A) gives
Tr(x_Q) = b₂(P), independent of c, and P ∈ 4E ⟺ b₂(P) = 0. (The b = 1 case is
Prop. 14.2 / R6.3; b does not enter.) The same identity gives b₂(−P) = b₂(P) +
Tr(x), so the two odd cosets (negatives of each other) always differ in b₂.

*β.* Put u = x₄ = b^{1/4}, Tr(u) = Tr(b) = 1. y₄ = u·z with z² + z = u + a + u²,
z = H(a) + u + 1 (check: H(a)² + H(a) = a because Tr(a) = 0, and u² + u from
u + 1) and Tr(z) = ((n+1)/2)·Tr(a) + Tr(u) + 1 = 0, the canonical sign. Then
Tr(y₄) = Tr(u·H(a)) + Tr(u²) + Tr(u) = Tr(u·H(a)) and b₂(P₄) = Tr(y₄) +
Tr(u·H(u+a)) = Tr(u·H(u)). Tr(u^{1+4^i}) = Tr(u^{4(1+4^i)}) = Tr(b^{1+4^i}), so
Tr(u·H(u)) = Σᵢ Tr(u^{1+4^i}) = Tr(b·H(b)). ∎

*x-only forms (P on E, x ≠ 0).* Dividing the curve equation by x²: z = y/x has
z² + z = x + a + b/x², so Tr(b/x²) = Tr(x) + Tr(a); and for P ∈ 2E with
Tr(a) = 0, y = x·(H(x+a) + H(b/x²) [+1]) gives b₂(P) = Tr(x·H(b/x²)). (The +1
carries the factor Tr(x) = 0.)

*What is proved and what is measured.* Proved here: the structure, P1, P1′, P2,
the 4E criterion for all (a, b), the closed form of β, the x-only forms, and
the cyclicity (no Z/4 × Z/2). **Measured, not proved:** that b₂ is constant on
each odd coset for general (a, b) (R6.3 proves it for (0, 1)), and, given that,
the E/8E, E/16E, E/32E tables (their derivation from the constancy is the
induction b_j(P) = b_{j−1}(½R(P)), valid when T ∈ 2^{j−1}E, i.e. v ≥ j; the
tables are then forced by the group law and were checked on the
297,211,712 points of the three runs, which overlap at n = 5..9). A proof of constancy
would need the analogue of R6.3's step (iii) with P₄ replaced by a point of
order 4 whose x-coordinate has Tr = Tr(b), which brings in H(x₄ + a + 1) and a
non-vanishing constant; not done.

## Cost (counted F_{2^n} operations; mean per point)

M product, S squaring or square root (a linear map), I inversion, H half-trace
(a linear map), T trace (a masked popcount). n = 23, 24,000 points per method for J ≤ 3
(60 curves × 400 points per v-class; J = 4: 14,000, J = 5: 8,800; `cost.json`, same table for n = 17, 19,
21). "halving" is the exact repeated-halving method on the same points, which
also needs Q0 (a precomputed odd point) and, for an odd point, a point
addition before each halving; "scalar" is the ground-truth class by [r]P.

| quantity | parity | M | S | I | H | T |
|---|---|---|---|---|---|---|
| bit1, a = 1 (the whole E/4E label) | any | 0 | 0 | 0 | 0 | 1 |
| scalar truth, a = 1 | any | 64.4 | 53.7 | 32.2 | 0 | 0 |
| pair label (J=2) | even | 1 | 0 | 0 | 1 | 3 |
| pair label (J=2) | odd | 1 | 0 | 0 | 1 | 3 |
| halving (J=2) | even | 2 | 1 | 0 | 1 | 2 |
| halving (J=2) | odd | 4 | 2 | 1 | 1 | 2 |
| scalar truth, J=2 | even / odd | 60.0 / 61.0 | 50.6 / 51.1 | 30.0 / 30.5 | 0 | 0 |
| triple label (J=3, E/8E) | even | 4 | 1 | 0 | 2 | 6 |
| triple label (J=3) | odd | 6 | 2 | 1 | 3 | 6 |
| halving (J=3) | even | 5.0 | 2.5 | 0.5 | 2 | 3 |
| halving (J=3) | odd | 7.0 | 3.5 | 1.5 | 2 | 3 |
| label (J=4, v ≥ 4) | even / odd | 8.0 / 10.0 | 2.5 / 3.5 | 0.5 / 1.5 | 3.5 / 4.5 | 9 |
| halving (J=4) | even / odd | 8.0 / 10.0 | 4.0 / 5.0 | 1.0 / 2.0 | 3 | 4 |

(J=2 on v ≥ 3 curves is identical; for scalar truth the ground-truth scaling
by 2^{v−5} adds doublings on curves with v > 5, which the means include.) The
pair label saves the point addition of the halving route on odd points
(I + 3M + 2S) and one M + S on even points, at the price of one more trace. The
triple label is not cheaper than halving: it trades one more H on odd points for
fewer M, S, I. From J = 3 on there is no counted-operation advantage; the label's
value for E/8E is that it is a closed-form function of the point and Q0 with no
secret-dependent control flow in the trace part, not that it is fewer operations.
Against scalar-multiplication truth, the pair is about 60× fewer M and 30 I fewer.
Wall time was not measured (shared host).

## Constant-time variant (R6.3's bar)

* Tr(a) = 1: the label is Tr(x) + Tr(a), a masked popcount of x against a fixed
  public mask: branch-free, no table, no secret-indexed access. A constant-time
  version performs the same single T operation, so its cost ratio to the
  variable-time one is 1 by construction: **the < 2× bar is met, trivially.**
* Tr(a) = 0: the pair needs H(x + a). **No new constant-time half-trace was
  found and nothing was re-timed in this round**, so R6.3's measurement stands:
  2.5–5.0× in contended wall time and 2.0–3.7× in counted word operations
  against the table version, i.e. **the bar is not met**. The E/8E refinement
  needs two or three H per point (table above), so any constant-time variant built
  from the same half-trace is at least as expensive; it does not meet the bar
  either. A quadratic-form formulation (Tr(x·H(x)) = x^T M x with a fixed public
  matrix M, Tr(x·H(a)) a fixed mask) costs the same n·W word operations as
  R6.3's masked basis sum. Not tested here.

## Limits

* Toy fields: exhaustive to n = 13 (a ∈ {0,1}) and n = 9 (all a), sampled to n = 23
  (1500 uniform points per curve, 1000 random curves per n and a-class). n odd only
  (H is the half-trace); for n even a = 1 has Tr(a) = 0 and falls in the v ≥ 2
  case, which needs a different quadratic solver and was not run.
* Constancy of b₂ on the odd cosets of 4E is verified, not proved, for
  general (a, b). The labels at J = 4, 5 inherit that status.
* The odd-point third bit uses a point addition; the 120-monomial search is a
  finite family (negative result for that family only).
* Cost is counted operations in a log-table implementation (M, S, I, H, T are
  each one count regardless of weight); no weights or wall times.
* Isomorphic curves (a, b) → (a + t² + t, b) (y → y + t·x) flip b₂ on odd points when
  Tr(t) = 1 (b₂ → b₂ + Tr(t)·Tr(x)); the rule is stated with the coordinates as given.

## Next step

Prove constancy of b₂ on the odd cosets for general (a, b) (a version of R6.3's step (iii) for a
point of order 4 with x₄⁴ = b, and a separate argument for v ≥ 3, where P₄ ∈ 2E);
an n-even version of the label with a general Artin–Schreier solver; a bit-sliced
constant-time half-trace if a constant-time Tr(a) = 0 test is wanted.

## Files

`examples/r7_7_coset_label.rs` (the experiment), `run_all.sh`, `selftest.txt`,
`exhaustive.{json,txt,log}`, `exhaustive_alla.{json,txt,log}`, `random.{json,txt,log}`,
`control1_wrong_a.*`, `control2_drop_term.*`, `control1_random.*`, `cost.{json,txt,log}`,
`beta.{json,txt}`, `witness.{json,txt}`, `search_level3.*`, `search_level2_control.*`,
`plot.py`, `plot_data.json`, `fig1_labels.svg`, `fig2_evidence.svg`, `fig3_cost.svg`,
`report.typ`, `report.pdf`.

## Note text (ready to paste into the consolidating section)

**R7.7 — the trace pair beyond a = 0, b = 1, and E/8E.** On y² + xy = x³ + a x² + b over
F_{2^n}, n odd, x(2P) = λ² + λ + a gives P ∈ 2E ⟺ Tr(x) = Tr(a) and E(F)[2] = {O, T}: the
2-primary part is always cyclic, so Z/4 × Z/2 cannot occur, and v = v₂(#E) is 1 for
Tr(a) = 1, 2 for Tr(a) = 0, Tr(b) = 1, and ≥ 3 for Tr(a) = Tr(b) = 0. The question's premise
for a = 1 was wrong: there E/4E = E/2E = Z/2, the label is Tr(x) + Tr(a), and the R6.3 second
bit b₂ = Tr(y) + Tr(x·H(x+a)) is not a coset invariant (b₂(−P) = b₂(P) + Tr(x), so on 2E a
point and its negative share a coset and differ in b₂); it is instead the root selector for
halving inside the odd-order subgroup. Measured: bit1 correct on 148,598,015 points, the pair
non-constant on 312,667 of 312,684 cosets (the 17 exceptions are tiny n = 5 cosets, counted with overlap), 0 cross-coset collisions. For Tr(a) = 0 the
R6.3 lemma holds for every b and a: P ∈ 4E ⟺ Tr(x) = 0 and b₂ = 0 (proved), the odd cosets
carry (1, β), (1, 1+β) with β = Tr(b·H(b)) for v = 2 (proved; (n+1)/2 mod 2 at b = 1), and
b₂ is constant on every coset (measured, not proved for general (a, b)): 0 failures in
71.6 M points (n = 5..13, a ∈ {0,1}, all b), 68.0 M (n = 5..9, all a, b) and 9.0 M sampled
(n = 17..23, 6,005 curves); broken labels fail in the same code.
For v ≥ 3 the pair cannot label E/8E (four values, eight cosets; smallest counterexample n = 5,
a = 0, b = 6). Adding b₃(R) = b₂(½R) (even R) and b₃(P) = b₃(P + Q0) (odd P) gives a bijection
E/8E → F₂³ (table 000, 100, 010, 111, 001, 101, 011, 110) and, recursively, labels of E/16E and
E/32E, with 0 failures in 74.1 M, 37.6 M and 18.2 M points. The 8-division condition is
Tr(x) = Tr(a), b₂(P) = 0, b₂(½P) = 0. The extra bit is not cheap: odd points need a point
addition, a search over 120 trace monomials found no closed-form substitute, and in counted
operations the pair saves a point addition over halving (J = 2) but the triple does not
beat halving. The constant-time Tr(a) = 1 label is one masked trace (bar met trivially); the
Tr(a) = 0 constant-time variant keeps R6.3's miss (not re-timed). Toy sizes only.
