# Validation scope

The frozen native analyzer and separate strict `check-frontier.rs` audit pass
all 12 prescribed executions, including both targets at 60 bits. Every solve
succeeded and passed fresh CPU certificate replay and scalar-plus-one rejection.
Matched-arm certificates are byte-identical. No cell was omitted or censored.

Canonical validation is recorded separately in `schema-check-venv.txt` and
`protocol-check-venv.txt`. The latter retains its rectangular-coverage findings;
it is not a full protocol-checker pass. The protocol explicitly prescribes
b/u/x only at 52 bits and u only at 56/58/60 bits.

The first administration-side invocation used the system Python, whose
jsonschema module lacks Draft202012Validator. Its tracebacks remain in
schema-check.txt/protocol-check.txt. Re-running with the established tools
virtual environment changes no raw evidence or frozen protocol.

All wall clocks remain L0 and wall_valid=false. Coverage auditing is distinct
from independent scientific review, matched rho, or fastest-tool comparison.
