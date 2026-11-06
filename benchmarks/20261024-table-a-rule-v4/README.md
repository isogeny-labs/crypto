# Table A rule v4 (2026-10-24): not admitted

Copies of the figures and report of
[`research/ic_tool_program/rule/v4/`](../../research/ic_tool_program/rule/v4/README.md).

There is deliberately no row in `benchmarks/README.md`. The validation gate
(`docs/VALIDATION_GATE.md`) admits a row only with a complete
`VALIDATION.json`. The independent validator recomputed all 157 headline
numbers and found them in agreement
(`research/ic_tool_program/rule/v4/validator-report.json`). It still withheld
`VALIDATION.json`, for this reason:

- **The rho reference may not be the strongest available.** The protocol's
  admissibility check failed at 4 of 6 sizes. On the same targets, the strong
  fixture needs about 0.845 of ParallelRho's steps, so the reference priced
  here may be weaker than the strongest rho.

The result is also a null result for the board. The v4 recipe improves
on v3 at 1 of 6 sizes (n57, 0.953 [0.904, 0.998]), and the frozen rule needed 4.
At rho's canonical step, the distance remaining to matched rho is 9.9x to
43.5x.
