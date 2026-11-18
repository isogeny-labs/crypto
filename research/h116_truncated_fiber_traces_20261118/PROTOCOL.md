# H116 frozen protocol (human form)

The machine record is `protocol.json`; `COHORT_RULE.json` fixes the structural
selection before the selector is run.

## Question

Can one public, fixed-length Laurent calculation reproduce all 16 H115 moments
on a new five-curve prime-field cohort without reading scalar labels or
constructing the degree-`r` fiber, and does any one of the already frozen 648
radix-six predicates remain exact across every row and target?

## Arms

- **candidate:** normalized-isogeny Laurent recurrence, denominator-cancelled
  relative traces and residue `Y` moments;
- **direct:** explicit extension-field kernel/fiber construction and `r`-term
  sums;
- **direct A/A:** byte-identical direct path under a separate arm label;
- **positive:** planted separable polynomial/fiber with independently known
  roots and moments;
- **negative boundary:** the H115 `p=5,7,13` fixtures must be rejected by the
  large-characteristic recurrence rather than silently scored.

The selector excludes every exact H115 `(p,a,b)` fixture. Every selected curve
uses three public generator labels and every nonidentity target. Target order is
seeded and shared across arms. The producer never reads the scalar label while
constructing a moment or predicate value.

## Decisions

1. **Identity component:** zero candidate/direct mismatches across all 16
   moments, every target, row and coordinate isomorphism; otherwise refuted for
   the frozen scope.
2. **Cost component:** report cold, reusable setup and online counts/times. A
   lower candidate stage cost is only a component result.
3. **Digit component:** one predicate ID, chosen by the frozen all-row rule,
   must be exact across every target and row. Holm correction treats all 648
   IDs as one family. A positive discovery must replay independently and then
   pass a separately frozen held-out cohort before any decoder claim.
4. **Decoder component:** run only if item 3 passes. Verify `[m]G=Q` for every
   recovered scalar and include all moment, update and verification costs.

No survivor-only curve selection, per-curve predicate, fitted coefficient,
unpriced kernel table, rerun under a new seed, or promotion from toy data to an
asymptotic ECDLP claim is admissible.
