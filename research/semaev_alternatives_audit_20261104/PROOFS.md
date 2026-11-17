# Route ledger under the strict standard: "ruled out" = Lean theorem

**Standard.** A route is *ruled out* only if there is a machine-checked Lean 4 theorem that excludes
it. Measurements, estimates and short arguments are recorded as **not ruled out** with their status.
This supersedes the earlier draft of this file, which used "ruled out" more loosely.

Everything below is checked by

```
cd lean && lean Routes.lean && lean Roots.lean
```

core Lean 4.35, no Mathlib (10 theorems). Note the limit of the environment: core Lean has no `ring`
tactic, so multi-variable polynomial identities of degree > 2 cannot be closed here. The Vieta conic
identity of §6.1 is therefore SymPy-checked (`scripts/verify_core.py`), **not** Lean-checked, and is
not used as a premise for any exclusion below.

## 1. The formal core

The cost model is an assumption, stated once:

* `p = 2^n`, `B = 2^b`, `T = 2^t`;
* relation search costs `p·T/B^(m−1)` and linear algebra costs `mB²`;
* beating rho means a total below `p^{1/2}`, which forces `4b ≤ n`.

**`lean/Routes.lean`**

| theorem | statement |
|---|---|
| `m2_never` | `4b ≤ n`, `t ≥ 0` ⇒ `3n ≤ 4(n + t − b)`: arity 2 cost ≥ `p^{3/4} > p^{1/2}` |
| `m3_at_best_ties` | `4b ≤ n`, `t ≥ 0` ⇒ `n ≤ 2(n + t − 2b)`: arity 3 cost ≥ `p^{1/2}`, tie only |
| `m4_win_forces_t_lt_quarter` | m = 4 win ⇒ `4t < n` (T < p^{1/4}) |
| `m5_win_forces_t_lt_half` | m = 5 win ⇒ `2t < n` (T < p^{1/2}) |
| `mitm_m4_cannot_win` | MITM (`T = B²`, `B⁵ = p`) has `n ≤ 2(n + 2b − 3b)`: fails the m = 4 gate |
| `mitm_work` | two-table MITM work = `n(n−1)` group operations |
| `m3_win_impossible` | m = 3: no `t ≥ 0` wins (strict form of `m3_at_best_ties`) |
| `mitm_m4_m5_lower` | MITM at m = 4 (`T = B²`) and m = 5 (2+3, `T = B³`): cost exponent ≥ 2n/3 for every base size |
| `m6_win_forces_t_lt_three_quarters` | m = 6: win ⇒ `4t < 3n` (T < p^{3/4}) |

**`lean/Roots.lean`**

| theorem | statement |
|---|---|
| `no_three_roots` | for a modulus `p` with Euclid's lemma, `p ∤ a`, at most two residues can satisfy `a x² + b x + c ≡ 0` |

(`p` primality enters only as the hypothesis `∀ u v, p ∣ u*v → p ∣ u ∨ p ∣ v`; core Lean has no
`Nat.Prime`.)

## 2. What is ruled out, and why

**R1 (arity 2).** Any decomposition oracle at arity 2 cannot beat rho. `m2_never` is unconditional in
the oracle: `t ≥ 0` is all it needs, so *no* m = 2 method wins. This rules out every route whose
oracle is a 2-decomposition:

* **Idea 2** (Vieta conic), **Idea 3** (Γ₀(p) forms), **Idea 5** (Kummer ladders, P + Q),
  **Idea 18** (determinant method on S₃).

Their internal claims (O(B) enumeration, census counts, h_R ≈ √p, empty box) are measurements and are
listed in §3 as such — but the *route* is dead by R1 alone.

**R2 (arity 3).** Arity 3 ties at best and needs `T = O(1)`; `m3_at_best_ties`. This rules out
**Idea 6** (special-j S₃/S₄), **Idea 7** (integer programming tested at m = 2, 3) and the m ≤ 3 half
of **Idea 12** as exponent-changing routes.

**R3 (gate at m = 4).** Any m = 4 method with `T ≥ p^{1/4}` loses:
`m4_win_forces_t_lt_quarter`. This rules out, *for every* such method.

**R3′ (generic MITM, every base size).** At m = 4 the 2+2 MITM has `T = B²`, so its cost exponent is
`max(n−b, 2b)`; at m = 5 the 2+3 MITM has `T = B³` and the same exponent. `mitm_m4_m5_lower` gives
`2n ≤ 3·max(n−b, 2b)` for **every** b, i.e. cost ≥ `p^{2/3}` — a genuine exclusion, not just a failure
at `B = p^{1/5}`. (Beyond m = 5 the model needs memory accounting, so no claim is made.)

**R4 (gate at m = 5).** Any m = 5 method with `T ≥ p^{1/2}` loses: `m5_win_forces_t_lt_half`.

**R5 (root bound).** A quadratic congruence has at most two roots (`no_three_roots`). This bounds the
conic enumeration at ≤ 2 points per `e₁` and the box at ≤ 2 points per `x₁` — a *bound*, not a
ruling-out.

## 3. Not ruled out (measured / argued / open)

Per the standard, none of the following is "ruled out". Each line says what has actually been done.

| # | route | status | what exists |
|---|---|---|---|
| 1 | certified reach audit | open (m=4 only) | cuso: 1/6 (x-coords and Vieta), 1/24 (m=3); measured reach 0.13–0.16 |
| 4 | Riemann–Roch norm system | open (m=4) | resultant argument; msolve degree 2 at toy sizes |
| 8 | capacity-theory certificate | open | nothing |
| 9 | Heegner free relations | not ruled out | rank-deficiency argument only |
| 10 | squarefree collisions | not ruled out | measured 1 pair ≤ 2^16 |
| 11 | cavity degree prediction | open | nothing |
| 12 | learned shift families | not ruled out | random subset search: ≤16 shifts fail, ≥18 match F (`experiments/idea12_shift_search*.json`) |
| 13 | lattice-query lower bound | open | nothing |
| 14 | Yokoyama for compositions | open | nothing |
| 17 | function-field mirror | not ruled out | carry-loss argument only |

Measurements that are sometimes mistaken for proofs, with their exact scope:

* Idea 3 census: 6 cells at p ≤ 2^24; structured = decompositions in every cell.
* Idea 5: 400 targets; h_R ≈ √p; fraction with h_R ≤ p^0.2 is 0.
* Idea 4: msolve at p ≤ 2^30, B ≤ 64; basis degree 2.
* Idea 18: 0 box points at B = p^{1/5} for p = 2^20, 2^24, 2^28.
* Idea 10: 1 collision pair for B ≤ 2^16.
* Idea 1: reach table m = 2..5; univariate control 0.48/0.52.
* Idea 12: 48 random shift families sampled from F at p = 2^40, δ = 0.15 (4 trials each). Every subset
  of size ≤ 16 solved 0/4; subsets of size 18–20 matched F (1–2 of 4). No sampled family found a
  better structure than the diamond, and dropping two shifts costs nothing — a flat landscape near the
  top, so search is not the lever (the certificate is).

## 4. What the strict standard costs

Three claims in the earlier draft were withdrawn or downgraded:

1. "The conic sieve is O(B), hence closed" — the O(B) upper bound follows from `no_three_roots`, but
   *completeness* (every decomposition is found) needs the Vieta identity, which is not formalized
   here. Downgraded to: the route is ruled out by R1, and O(B) is measured.
2. "A degree-B base forces solving degree ≥ B" — false (msolve returns degree 2); withdrawn.
3. "Idea 10 is closed by Siegel" — Siegel is not formalized and not in Mathlib's core fragment here;
   the census is a measurement. Downgraded to not-ruled-out.

## 5. Summary

Ruled out (machine-checked): **arity 2 entirely (R1), arity 3 as a winning route (R2), every m = 4
method with T ≥ p^{1/4} (R3), every m = 5 method with T ≥ p^{1/2} (R4), the generic 2+2 MITM (R3 +
`mitm_m4_cannot_win`)**. Under R1/R2 that covers Ideas 2, 3, 5, 7, 18, the m ≤ 3 parts of 6 and 12, and the m ≤ 3 parts of 1 and 4.

Not ruled out: Ideas 1, 4 (m = 4), 8, 9, 10, 11, 12 (m = 4), 13, 14, 17.
