# Phase 5 errata (post-run, from the director's assessment; no value or result changes)

`PROTOCOL.md` stays as frozen. The frozen tool (`fp31-class-census p5-spec`, bound by hash) differs from two
sentences of its text, as the director recorded in
`cards/CARD-fp31-phase5-percurve-20261112-1.assessment.md`:

1. **Attempts per IC run.** §3 gives 5,965 (m = 3) and 102,425 (m = 2), which are the pilot's values. The tool applies
   p2tool's rule to each class's own group order `n`, so `max_trials` is 5,964–5,965 for m = 3 and 102,418–102,427 for
   m = 2. Within a class the value is the same on every curve, so the paired within-class tests are unaffected, and
   the metric is time per attempt.
2. **Distinct permutations.** §3 says each session uses "a different permutation". The seeded shuffle repeated an
   order between two sessions in 9 of 142 (class, kind) combinations: c009 rho, c010 ic3 and ic2, c014 rho, c018 rho,
   c019 rho, and c022 rho, ic3 and ic2. For c018 rho, 3 curves over 8 sessions, distinct orders are impossible by
   count. A repeated order still has fresh targets and seeds, and in every class most sessions have distinct orders.

The assessment also notes that 5,789 measured rows are graded L1 rather than L2. Reasons include context switches,
run-queue delay, and rho runs shorter than 1 ms. §5 states no exclusion rule, so the analyses include them, as frozen.
