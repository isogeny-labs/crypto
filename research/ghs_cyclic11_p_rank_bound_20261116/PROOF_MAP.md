# Lean instance gate: cyclic-11 orbit bound

1. **Structure above.** The endpoint is a statement in the natural-number
   preorder from Lean `Init`. Its operations are `≤`, equality and zero.
   The finite certificate uses `List.all` and decidable equality.
2. **Theorems used as stated.** `Nat.eq_zero_of_le_zero` turns a zero upper
   bound into zero. `List.all_eq_true` turns the checked list Boolean into
   the row-membership statement, and `of_decide_eq_true` turns each Boolean
   equality into a proposition. No p-rank theorem is re-proved in Lean.
3. **Object as instance.** `orbitBound a b` is the concrete natural-number
   bound computed from the two length-five signature orbits. `rows` is the
   finite list of 90 nonzero inertia exponent pairs.
4. **Assembly probe.** The standalone `lean /tmp/ghs-lean-probe.lean` checked
   `example {f b : Nat} (hb : b = 0) (hf : f ≤ b) : f = 0 := by
   exact Nat.eq_zero_of_le_zero (hb ▸ hf)` under Lean 4.35.0.
   The study endpoint adds `ab ∈ rows`, checks `orbitBound ab = 0`, then
   applies that generic theorem to any `f ≤ orbitBound ab`.
5. **Missing components.** Only the finite instance obligation
   `all_rows_orbit_bound_zero` remains to be discharged by kernel `decide`.
   The geometric bridge from an actual cover to the signature and Bouw
   bound is external and cited from Ozman--Pries--Weir (2022), Lemma 2.1
   and Theorem 2.3. No claimed Lean theorem includes that bridge.
6. **Inheritance table.** `f ≤ orbitBound` is a supplied hypothesis;
   `orbitBound = 0` is the checked instance; equality substitution is `hb ▸ hf`;
   the zero conclusion is inherited from `Nat.eq_zero_of_le_zero`.

| Statement | Kind | Owner | Quantified structure | Hypotheses |
|---|---|---|---|---|
| `all_rows_orbit_bound_zero` | instance obligation | this study | finite list of inertia pairs | none |
| `zero_rank_if_bouw_bound` | endpoint | this study | natural-number preorder | row membership, published-style upper bound |

The only proof chain is `f ≤ orbitBound a b = 0`, then
`Nat.eq_zero_of_le_zero`. The theorem's `f` may be read as a p-rank only
after the cited external geometric statement supplies the inequality.

Toolchain: Lean 4.35.0, arm64-apple-darwin24.6.0, commit
`eb6cafb7290e8bd5f2937522d8d804f58f02bafc`; the standalone file
imports `Init` and no external Lean package. The focused repository proof
gate returned `PASS`, five theorems, zero trusted exceptions; build and
axiom-audit logs are in `lean-gate/`. `lean4checker` was unavailable in this
environment.
