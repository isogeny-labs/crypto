# Signed recovery data dictionary

The frozen input and gate order are in [PROTOCOL.md](PROTOCOL.md). All rows
are public-field arithmetic; no private-key material is used.

| File | Unit | Fields needed for review |
| --- | --- | --- |
| `results/zero-shift.jsonl` | one of 175 zero xADD cases | original case, five 2-torsion shifts, projective outputs, selected root, verified flag |
| `results/literal-recover-failures/recover-target-*.jsonl` | one of 1,260 target attempts with literal displayed equations | source id, expected signed Mumford divisor, literal output or denominator status, quotient coordinates |
| `results/recover-target-*.jsonl` | one corrected target attempt | source id, signed anchor, Kummer Q/Q±P, general coordinates, recovered signed divisor, exact status, exceptional search count, independent expected divisor |
| `results/summary.json` | aggregate | seven bit slices, fast and exceptional counts, explicit null cost fields |

The three `enumerated_exception_lift` rows are a bounded field search,
followed by arithmetic validity and a signed-anchor check. Their per-row
`exceptional_candidates_tested` is a count of candidate Mumford divisors,
not a multiplication count. No priced DLP or candidate/rho ratio is implied.

`validate.py` independently recomputes the quintic from theta parameters,
checks every recovered divisor's polynomial relation and general-Kummer
projection, joins all 1,260 source rows, and recomputes aggregate counts.
It does not rerun the producer's Jacobian group law; the native producer
checks every recovered divisor against the independent source-derived signed
Jacobian target.
