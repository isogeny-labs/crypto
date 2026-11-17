# Valuation leading-digit sublemma

`ValuationProbe.lean` contains two checked theorems in namespace
`TernaryValuationProbe`:

- `quotient_not_dvd`: for every natural `p ≠ 1` and integer `t ≠ 0`,
  `¬ (p : Int) ∣ t / (p : Int) ^ padicValInt p t`.
- `leading_ternary_nonzero`: for every integer `t ≠ 0`,
  `((t / (3 : Int) ^ padicValInt 3 t : Int) : ZMod 3) ≠ 0`.

Both include all negative nonzero integers and require no primality instance.
The proof uses the actual Mathlib valuation definition and its maximal-power
divisibility characterization. `padicValInt_dvd t` gives the exact division.
If the quotient were divisible by `p`, then multiplication and exact division
cancellation would make `p^(v+1)` divide `t`. The theorem
`padicValInt_dvd_iff_of_ne_one` contradicts either `t≠0` or `v+1≤v`.
`ZMod.intCast_zmod_eq_zero_iff_dvd` gives the final cast statement.

Pinned Lean/Mathlib/shared dependencies are copied from the frozen parent
project. The `.lake` link reuses its build cache; no frozen source or pin was
modified. Build command:

```sh
PATH=/workspace/audit/toolchains/lean-4.35.0-linux/bin:$PATH lake env lean -o ValuationProbe.olean ValuationProbe.lean
```

Build 1 failed solely because the generic `mul_dvd_mul_right` takes its
divisibility proof before its scalar argument. Its exact source is retained
as `receipts/ValuationProbe.build-01.lean.txt`, with `build-01.log`. Corrected
build 2 succeeded; both `#print axioms` show only `propext`, `Classical.choice`,
and `Quot.sound`. The pinned shared transitive auditor also passed both
theorems with zero failures and zero native-trusted proofs.

This sublemma proves leading digit nonzeroness. It does not prove the separate
division/digit-addition relation or the full contraction theorem.
