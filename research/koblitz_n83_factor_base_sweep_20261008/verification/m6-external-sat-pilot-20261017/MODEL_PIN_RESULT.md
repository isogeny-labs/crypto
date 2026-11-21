# External XOR syntax control: complete model accepted

The [frozen protocol](MODEL_PIN_PROTOCOL.md) pinned the already audited native
118,684-variable assignment on the exact Gray `all_x` XOR-DIMACS export. The
original 554,504 rows were byte-identical in each derived input. One derived
formula added all 118,684 model bits as unit clauses; the other flipped only
variable 1 among those units. The independent [audit](audit_model_pin_control.py)
checked every appended unit and original byte, both solver outputs, hashes,
resource receipts and raw tree. Its [sanitized receipt](model-pin-audit.json)
reports `PASS_DERIVED_INPUTS_AND_SOLVER_OUTPUTS`.

| Derived input | SHA-256 | CryptoMiniSat 5.14.7 | Exit | Solver wall, local L0 | Peak RSS, local L0 |
| --- | --- | --- | ---: | ---: | ---: |
| Complete model | `7307c93fbec3576e4edff8d519ba9bc62794bbbaac13cc4de25a2170fe7099cb` | SAT | 10 | 0.231 s | 48.0 MB |
| Bit 1 flipped | `77fd53d9b9f1af477930801764744987a7e55b17b50eff735929f485399f1cc6` | UNSAT | 20 | 0.185 s | 45.9 MB |

The successful raw six-file tree has SHA-256
`84839e2873f4adc94cde6e4a566337e34bf70e22eae709f42bef3ab2f6f48e3e`.
The first sandboxed pair produced the same SAT and UNSAT status lines, but
macOS `/usr/bin/time -l` could not read `kern.clockrate` there. Its wrapper
returned 1 and omitted RSS. That pair is retained as an infrastructure failure
with raw tree SHA-256
`29cca7043ea87082201ab871098927ad880054cee3b490df8fbe95d1e3f86937`;
it is not counted as a passing control. An earlier Python preflight found that
the default interpreter lacked `blake3`; the runner was made standard-library
only before any solver call. The two pairs are conservatively charged at four
full 150-second envelopes, **600 seconds**, with validation cost separate.

This confirms that CryptoMiniSat accepts the exported XOR convention on the
known satisfying assignment, and rejects its one-bit contradiction. The
previous 5,000, 100,000 and 1,000,000-conflict indeterminate results therefore
remain censored search results on a satisfiable external formula. They do not
measure natural relation yield or rank and cannot select a minimum-total-runtime
factor base. `selected_best_total_runtime` remains null.
