# Independent code review: prospective pencil early rejection

Reviewed at `2026-10-30T04:44:07Z`. This was a read-only source and recorded-software-evidence review. It did not run a solver, cohort, benchmark, cloud job, or mathematical experiment.

## Result

No remaining concrete source or scoped validation defect was found after strengthening the candidate-origin fixture. The isolated fork is suitable for later protocol and source-freeze review. It has not produced a scientific, timing, speed, or cost-ratio result.

## Source fidelity

The frozen baseline and current default solver are byte-identical at SHA-256 `e71e9cf9b356db1f16781f394994b0ca13abbb05b4a7db82bfd2a6925a0ef4df`. The isolated fork is SHA-256 `6a8041b6ec015590d0f0ae105a483e4a4f0fb45357a2fd591ec9424542bf51d3`.

The complete runtime diff has 65 lines and two hunks, SHA-256 `add795e791c98560ef99100ed2d6fa67dad2e3230a96b85d51aea3ce3311cca2`. It replaces the original candidate-verification closure with a helper call, appends that helper with the existing bucket-length guard moved before `complement`, and appends a `cfg(test)` module. I found no other runtime difference. The pinned original helper, source manifest, and 17 dependency-source bindings all rehashed successfully.

Rejected bucket lengths `0,1,2,4,5` retain the guard and return before complement. The recorded exact delta is 24 base multiplications, 30 base additions, 78 collection visits, four copies, 18 initializations, and 27 predicates removed; the new rejected path retains one visit and one predicate. Group counts do not change. Length-three candidates retain all complement, quotient, root, y, Jacobian-sum, and point checks.

## Geometry and graph coverage

The final smooth `F_19` fixture traverses its complete bounded ordered rational-point-pair inventory. It independently reconstructs exceptional lambdas from `q0/(2v)` at `u(x)=0` and ordinary origins from pre-append rational-point lambda counts at `u(x)!=0`. It asserts every candidate has exactly one origin and records:

- 612 ordinary-origin candidates;
- 470 exceptional-only-origin candidates;
- 214 exceptional accepted candidates;
- 256 exceptional rejected candidates;
- 826 accepted candidates in total.

This resolves the earlier review defect where the ordinary counter incremented for every combined candidate and exceptional coverage only tracked divisor-level denominator presence.

Graph evidence is intentionally narrow. Accepted fixture outputs enter identical frozen graph code and produce equal selected cap results. Rejected helpers have no graph parameter and return before graph insertion. This supports source-bound selected-case parity, not global graph-state equivalence or every cap boundary.

## Recorded checks

The final debug selected-fixture log is SHA-256 `95fb00bd7e5be1dc8ff9fa805c48de6185fc8b66a0d0106ac182bea7e4023ebe`. The final release log is SHA-256 `2eb0f12a85978abed27624c3396543375883001889d63a8e545df87a633af424`. Each records five passing tests, zero failures, and twelve filtered inherited tests. The selected test source contains no `solve_one`, cohort, benchmark, or ecbench call. Strict Clippy and formatting records are retained, and the earlier failed attempts and weaker pre-origin-review passing logs remain preserved.

## Limits

The review did not examine an executed target, allocation equivalence, interruption behavior, deadline behavior, or timing. The field argument remains conditional on normalized residues and `p<=2^63`; the retained `solve_one` API does not enforce that narrower proof domain. The formatter log is empty, which is consistent with quiet `fmt --check` success but is not a separate machine-readable exit receipt. Any future execution needs prospective protocol and complete source/dependency bindings. Existing observations cannot be repriced with this fork.
