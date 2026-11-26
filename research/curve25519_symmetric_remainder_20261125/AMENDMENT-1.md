# Amendment 1: paired residue schedule in the direct arm

**Filed 2026-11-25 before implementation, correctness execution, or any solver row.**

Section 3's sentence that no intermediate residue remains in arm D is
incorrect. Read arm D as follows:

- D and Q use the identical binary square-and-multiply schedule.
- Each schedule step has the same explicit output residue
  `R = r0 + r1 X + r2 X^2` in both arms, except that the final residue may be
  constrained directly to `(1,0,0)` without allocating an output variable.
- D symbolically reduces `A B` modulo `q` and emits the three coefficient
  equations `R - (A B mod q) = 0`. It eliminates only that step's quotient.
- Q emits `A B - H q - R = 0` and retains the two coefficients of `H`.

Thus Q adds quotient variables and two coefficient equations per multiplication
but keeps every gadget quadratic; D has fewer variables and equations but its
symbolically reduced coefficient equations may have higher degree. Both arms
represent the same multiplication circuit and expose the same residue cut
points to F4.

This clarification prevents a fully expanded three-variable D system from
serving as an artificially weak baseline. No curve, cell, seed, budget, metric,
correctness gate, or stop/go threshold changes.
