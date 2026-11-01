# Binary Semaev SAT formula and initial-state checkers

The binary Semaev chain encoder now has a checked construction path and a separate checker for the native solver's fresh insertion snapshot. These are reusable validation components for original-bound binary SAT datasets. They do not activate a new dataset worker or change the existing dispatch capability.

## Formula admission

`provenance::Original` supplies caller-authenticated field, curve, target, domain, pattern and binding inputs. `build_original` constructs a gate map and verifies its complete ordered CNF/XOR formula; `verify_original` can check a supplied map against those originals. Scalar dimensions are checked before content scans. The checker independently constructs its polynomial-reduction cache rather than trusting the encoder's table. Formula ordering, finite-domain constraints, wire identities and actual retained incidences are checked.

The caller's Admission implementation owns the cumulative resource limit and deadline. Validation, independent cache construction, and actual formula materialization/checking require separate cumulative reservations with periodic probes. A failed build retains a completed but unchecked map when available. A raw deserialized Map is not authority: callers must authenticate originals and bound decoding before invoking the checker. CheckedMap is obtained only through semantic verification.

## Native initial-state admission

`initial::verify_initial` compares actual fresh SolverEvent records against the checked formula without calling the native Solver. It reproduces initial clause normalization, root-value simplification, arena offsets, units/reasons and XOR duplicate cancellation. Sparse expected rows are compared against observed dense masks. Exact event order and the fresh, pre-search state are required.

VerifiedInitial borrows both the CheckedMap and the actual events. Its counts describe checked insertion state; they do not certify later search, learned-clause entailment, an UNSAT proof, durable artifact publication or permission to emit a public training prefix. An authenticated owner, bounded native observer, durable snapshot publication, worker failure transfer and central causal replay remain the required integration path.

## Validation

The focused binary_semaev_chain_sat suite passed 23 tests with 0 failures, including the new formula and initial-state controls, with source/fixture hashes unchanged. The combined dataset suite later passed 611 tests with 0 failures and 2 ignored. Required full-library validation is running on the same implementation. These are local software checks, with no new scientific cloud campaign or instruction-cost claim.
