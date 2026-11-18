# H116 division-torsor Frobenius proof

The proof surface was mapped in `DIVISION_TORSOR_PROOF_MAP.md` before the Lean
source changed.  Lean now proves, in any additive commutative group, that:

1. translating `n`-torsion by a supplied `n`-division point stays in the same
   fiber;
2. the difference of two points in one fiber is `n`-torsion;
3. translating by a Frobenius-fixed point leaves `sigma(S)-S` unchanged;
4. this pointwise identity preserves the complete mapped multiset, including
   multiplicity; and
5. selecting the fixed point itself forces cocycle zero.

The pinned Lean 4.34.1 build completed all 3,164 jobs.  The repository's exact
transitive auditor reports:

```text
audited 224 theorems in [H116FiberTrace]; 0 failing, 0 trusted (native_decide)
```

The abstract theorem proves the cohomological mechanism and the conditional
selector collapse.  The finite Rust run supplies the curve-specific splitting
fields and selection evidence; Lean does not formalize the polynomial-basis
integer ordering. [pcc:unverified]
