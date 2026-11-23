# H115 Lean proof gate

Gate date: 2026-11-17

Status: **PASS** for the frozen generic algebra, finite-score handoff,
radix-six decoder, independent verification and explicit cost-premise layer.
This gate does not score a moment predicate or establish an efficient ECDLP
algorithm.

## Pinned environment

- Lean: `4.34.1` (`5045d0056413266e57c625dcd7c365b10e377c52`)
- mathlib: `d13f23b723b8a846827a245b89c10fc7d3f11612`
- source: `H115FrobeniusKummerResolvent.lean`
- source SHA-256:
  `55137b5f366e8dc345a42b67c3b815e23a07051f0042fc319580d02b76c79ee0`
- manifest SHA-256:
  `05bb6c0943a46ac7b9afe4831996b3fcd2091949c7c8b9c085b5ea269f818dfb`

The H115 `lake update` resolved and wrote the checked-in manifest. Its
post-update hook then attempted a full 8,908-object mathlib cache download and
exhausted the workspace disk. The partial H115 cache was removed. Compilation
therefore reused the already compiled H113 cache with the same Lean toolchain
and exact same mathlib commit; the H115 source itself was passed as an absolute
path.

## Strict command

From `research/h113_binary_trinary_winding_20261117`:

```text
ELAN_HOME=/workspace/.elan-h113 \
  /workspace/.elan-h113/bin/lake env lean -DwarningAsError=true \
  /workspace/h114/research/h115_frobenius_kummer_resolvent_20261117/H115FrobeniusKummerResolvent.lean
```

Exit status: `0`.

The `#print axioms` audit contains only mathlib's standard `propext`,
`Classical.choice` and `Quot.sound` dependencies. A source scan found no
`sorry`, `admit` or declared `axiom`.

## Discharged obligations

1. Both orders of `(V-1)(pi-1)=[r]` from the exposed Frobenius/Verschiebung
   premises.
2. Division-point cocycle mapping and change under an `r`-torsion translate.
3. Exact `(V-1)` fiber/coset equivalence.
4. Certified rational section using an explicit inverse modulo the subgroup
   order.
5. Image/kernel equality only under an explicit reverse-lifting premise.
6. Every nonempty homomorphism fiber is equivalent to the kernel; multiplicity
   `r` requires a finite kernel-cardinality premise equal to `r`.
7. Full finite score implies a pointwise radix-six oracle; CRT-6 reconstruction
   is unique below six.
8. Radix-six arithmetic recovery and a scalar-blind cyclic group update using
   an explicit inverse-six certificate.
9. Independent final scalar verification.
10. Named extension, torsion/kernel, fiber, scoring and decoder cost terms;
    subexponential total cost appears only as a premise of the final theorem.

## Still empirical or open

- Public moments matching the native implementation.
- Any predicate ID attaining a full frozen score.
- A uniform shortcut avoiding the producer's explicit `Theta(r)` fiber sum.
- A generic-prime-field or subexponential ECDLP result.
