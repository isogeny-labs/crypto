# F4 post-hoc Hilbert diagnostics

Problem: commits `8bb0e75a961f45d28aa06ef00909211b4539e663` and
`19986505835d3fd95cf92b0f5fcc11c22826b358` compare standard monomial counts
of the final computed basis with a semi-regular expectation, then label the
first/last deviations as first/last fall degree and staircase extinction as
observed regularity. Their complete implementation was recovered during compiler
repair; this audit found the mathematical labels unsupported by that procedure.

The definitions require closure-space or top-form information; see Caminata and
Gorla, *Solving degree, last fall degree, and related invariants*, sections 1,
2 and 4, [arXiv:2112.05579](https://arxiv.org/pdf/2112.05579). A final-basis scan
does not retain that information, especially after a degree-truncated run.

A retained counterexample uses `x^2-1`, `y^2-1`, `xy-x` over F_7. The scan
reports a deviation in degree 1 although all inputs have degree 2, so the
closure space V_{F,1} contains no nonzero input and cannot exhibit a degree-1
fall. The regression retains the actual scan and checks its values under the
correct diagnostic names. It does not infer a formal invariant from them.

F4 and explicit APS diagnostic APIs expose `first_hilbert_deviation_degree`,
`last_hilbert_deviation_degree` and `staircase_extinction_degree`. The ordinary
APS entrypoints leave diagnostics off and omit their absent fields from JSON,
preserving frozen method records and their counter namespaces. Opt-in callers
use `visit_system_recording_hilbert` or `leads::visit_plan_recording_hilbert`.
The reference scan does not compute observations that its report cannot carry.

The replay parser explicitly recognizes both current names and the historical
reported-only names while retaining its strict unknown/duplicate-field checks.
Neither vocabulary attests cost, execution or formal fall-degree invariants.
Raw producer bytes remain the byte-binding input. Original commits, records,
workloads and receipts are unchanged and are not retrospectively certified.

Diagnostic staircase counting has no native-counter side effect and cooperates
with the original deadline. Its results are discarded if the count cannot be
completed. The active solver and priced operation counters are unchanged.

Checked coefficient arithmetic and machine-word conversion make an oversized
Hilbert expectation unavailable rather than publishing wrapped counts. A separate
overflow regression covers this engineering failure mode.

Validation is pending the current-source compiler pass and executable controls.
This note is an engineering audit record, not a performance or scientific result.

The newer JV cover caller uses the same optional observation names and keeps
its ordinary options uninstrumented. Its explicit diagnostic control compares
solutions and counted arithmetic with the default. Newly generated optional
growth data uses `ecbench.growth-dataset/v2` and labels staircase extinction;
existing growth files are retained. Integration validation is recorded in the
session log. [pcc:unverified]
