# N83 known-witness phase-prefix search gate

Fixing the first four of five known Frobenius phases gives a checked SAT
model at 5,000 conflicts. CryptoMiniSat returned it in 687.398 ms of local
solver wall time. The [independent replay](phase_audit.py) checked all
714,764 CNF clauses, 4,980 XOR rows, eight fixed x blocks, four original-field
S3 equations, and the five-point sum to the retained target. Prefixes of
one, two, and three phases stayed `UNKNOWN_solver_cap` through 100,000
conflicts; the three-phase formula also stayed unknown through the separately
sealed one-million-conflict call.

The [first panel](phase_protocol.json) was pushed at
`d476b49cb6a856e431864b7c6a0b9320ced6445b` before its seven calls. The
[extended three-phase cap](phase_extension.json) was pushed at
`70f28e4958f3189bbf837265875e9a6895398af8` before its call. Both retain
the same 13,202,037-byte parent XCNF, SHA-256
`c2cc36eee7a72dbc583a217a0d42d02c74d5d3fde241536941b6614fc565b81e`,
and add only phase unit clauses from the independently identified witness.
All four formulas and eight first-attempt solver receipts are under
[`evidence/phase/`](evidence/phase/). The producer's raw SAT model is stored
byte-for-byte as gzip.

| Pinned first phases | Conflict cap | Wall ms | Status |
| ---: | ---: | ---: | --- |
| 4 | 5,000 | 687.398 | checked SAT model |
| 3 | 5,000 | 489.044 | `UNKNOWN_solver_cap` |
| 2 | 5,000 | 250.672 | `UNKNOWN_solver_cap` |
| 1 | 5,000 | 281.948 | `UNKNOWN_solver_cap` |
| 1 | 100,000 | 7,133.676 | `UNKNOWN_solver_cap` |
| 2 | 100,000 | 6,232.341 | `UNKNOWN_solver_cap` |
| 3 | 100,000 | 7,618.578 | `UNKNOWN_solver_cap` |
| 3 | 1,000,000 | 56,286.797 | `UNKNOWN_solver_cap` |

The earlier five-phase model was checked at 125.253 ms under its own
5,000-conflict cap. These are local arm64 known-answer diagnostics with
different supplied phase information. They quantify the solver access path
for this fixed witness, not natural relation yield, rank, or a complete
factor-base runtime. The four-phase result establishes satisfiability with
one phase free; capped unresolved prefixes remain censored. `candidate_id`
and `selected_best_total_runtime` remain null.

The next solver experiment varies native-XOR Gaussian-elimination settings
on the original unpinned formula under a separately frozen A/B/A schedule.
Any returned model must pass the same independent replay. A factor-base
selection still requires verified ordinary-query relations, useful rank,
matrix solving, target descent, and same-point rho timing.

Recheck from the repository root with:

```sh
python3 research/n83_chain_xcnf_control_20261019/phase_audit.py
```
