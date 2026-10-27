# Cover-transfer injectivity lemma in Lean 4

`CoverTransfer.lean` proves one conditional group-theoretic claim used in the
cover assessment.  Let `S` be a finite additive subgroup with `Nat.card S = r`.
Given additive homomorphisms `pull : E →+ J` and `push : J →+ E`, if

```text
push (pull x) = d • x                 for every x in S
Nat.Coprime d r
```

then `pull` restricted to `S` is injective.  The proof applies the pinned
Mathlib theorem `Nat.Coprime.nsmul_right_bijective`: multiplication by `d` is
bijective on a finite group whose cardinality is coprime to `d`.

The formal result is slightly more general than a cyclic prime-order subgroup:
only finiteness, exact cardinality, and coprimality are required.

## Reproduction

This package follows the repository's pinned Mathlib proof setup:

- Lean: `leanprover/lean4:v4.34.1`
- Mathlib: `d13f23b723b8a846827a245b89c10fc7d3f11612`

From this directory:

```sh
lake exe cache get
lake build
lake env lean AxiomAudit.lean
```

`AxiomAudit.lean` rejects proof dependencies outside the repository's standard
allowlist (`propext`, `Classical.choice`, and `Quot.sound`) and also prints the
kernel's transitive axiom report for the theorem.

### Local validation status (2026-10-25)

The original 2026-10-13 replay stopped when its cache ran out of workspace
space.  A later replay used Lean `v4.34.1` and the exact package revisions in
the committed `lake-manifest.json`, including Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`.  The tracked theorem,
audit, manifest, build file, and toolchain were copied byte for byte into an
isolated project with a matching local package cache.  The missing Mathlib
order-of-element object was restored using `lake exe cache get
Mathlib.GroupTheory.OrderOfElement`.

In that project, `lake build CoverTransfer` completed successfully (1,131
jobs), and `lake env lean AxiomAudit.lean` exited zero.  The audit printed:

```text
AXIOM_AUDIT|CoverTransfer.pull_injective_on_subgroup|[propext, Classical.choice, Quot.sound]
```

This is a local kernel and axiom check of the conditional theorem as stated.
The exact-head hosted proof job must also pass before this PR merges.  Neither
check establishes the geometric or arithmetic hypotheses for any particular
catalog row; see the boundary below.

## Boundary

The theorem does **not** prove any of the following:

- that a catalogued cover, pullback, or pushforward exists over the stated
  field;
- that divisor formulas are well-defined on Jacobian classes or are additive;
- the composition identity `push ∘ pull = [d]`; it assumes that identity on
  every element of the named subgroup;
- the subgroup's stated order, the coprimality calculation, or membership of a
  catalogued generator;
- injectivity outside `S`, surjectivity of pullback, injectivity of pushforward,
  or an isomorphism between the source and target groups;
- preservation of a factor base, index-calculus relations, target
  decomposition, or any discrete-logarithm speedup or cost bound.

If `gcd(d, r) ≠ 1`, the theorem is merely inapplicable; it does not prove that
pullback has a nontrivial kernel.  Each catalog row still needs independent
certificates for its geometric and arithmetic hypotheses before this lemma can
promote an injectivity claim.
