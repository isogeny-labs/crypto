# Recurrence audit and section-power qualification

PCC-20261108-0cc2d8 is unsigned producer evidence; independent signed review pending. [pcc:unverified]

Validation03 output:

```
quotient_cases=448 exact_block_degrees=384 wrong_order_controls=176 nonunit_quotient_cases=168 no_transition_cases=64 cap_rejections=pass
section_equalities=1008 small_algebra_products=768 large_prime=998244353 large_projection_equalities=56 large_algebra_products_each=34 mode1_length=71303168 slot_guards=pass
```

The block implementation matches dense mode evaluation in448 configurations, checks exact degree mb in384 nontrivial cases and detects176 reversed matrix-order discrepancies.168 cases use nonunit quotient variables. These establish the recurrence and its representation cost, not a fast block algorithm. [pcc:unverified]

Section powering matches1008 independently assembled dense residue-class evaluations. It also matches56 weighted roots-of-unity projections over F_998244353 using m7 and eight scalar quotient values. Each large-prime call uses34 seven-component algebra products; the corresponding mode1 vector has71303168 coefficients but is never built. This count is not a timing ratio or a base-field operation count. Inputs include zero and repeated quotient factors; m need not divide h for the section API. [pcc:unverified]

Validation01 passed the block-only audit. Validation02 failed to compile because a fixture moved g inside a repeated loop. The shell then ran the prior binary: its zero exit is explicitly invalid as evidence for the new code. Logs and failed source are preserved. Validation03 clones the fixture modulus and uses a fresh binary with stop-on-error; both build and all checks pass. No inherited recovery run is claimed: this standalone evaluator imports six compiler dependencies and leaves the recovery adapter unchanged.

Lean checks three limited commutative-ring lemmas for triangular composition, normalization and one-wrap substitution; axioms are propext, Classical.choice, Quot.sound. The quotient-module identity and runtime refinement are not fully formalized. Report and diagram are explanatory, not performance plots.

Next: integrate section powering into small active quotient evaluation and compare all-component multiplication cost against coefficient streaming with source/setup costs included. Initial candidate generation is still a separate large representation. The prior signed/mode2 timing panel remains unrun. No cloud job or original-cohort modification.
