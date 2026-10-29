# Fast-Kummer recovery and zero-coordinate continuation

Frozen 2026-10-22 UTC before the following candidate runs. Input is the
source-verified 1,260-target coordinate cohort and all 175 zero-output cases
in `research/kummer_coordinates_20261022/`, including the five independently
checked rational two-torsion projective matrices per model.

First test a quotient-only zero-coordinate repair. For an xADD triple
`K(A), K(B), K(A-B)`, translate the first and difference inputs by each
rational two-torsion class `T`: call xADD on `K(A+T), K(B), K(A-B+T)`, then
translate the output by `T` again. Compare against the recorded K(A+B).
Try the five branches in recorded root order; record every attempt and all
175 outcomes, including failures. This does not use the Jacobian oracle in
the repair itself; the recorded expected output is only the validation oracle.

Then implement [Chung, Costello, Smith §§6.4 and 7.4](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf):
invert the explicit fast/general Kummer linear map, compute K(Q-P) with
xADD (using the preceding exceptional branch if needed), and evaluate
equations (7)–(14) to recover signed Q from signed anchor P, K(Q), K(Q+P).
Use all 1,260 public targets, retaining zero denominators, special Mumford
degree, wrong recovered points and source controls as explicit rows.

Only if signed recovery and exception coverage pass, attempt a closed
arbitrary-jump label-carrying collector and paired cost run. No inferred
candidate/rho cost ratio is allowed before that run. Archive raw JSONL,
independent validator, a PDF and graph for every executed stage.
