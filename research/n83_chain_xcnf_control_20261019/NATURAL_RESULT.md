# Paired K600 first ordinary-query XCNF gate

The exported native-XOR formulas reproduce the hosted K600 Gray/hash pilot's
first query and its SAT dimensions exactly. The independently computed query
point is
`(3547082699798970859000627, 6877207017147849837699235)` on the primary
N83 curve. Both one-million-conflict external-solver calls ended
`UNKNOWN_solver_cap`, with raw solver wall times of 122.117 s for Gray and
103.886 s for hash. The [archive audit](natural_audit.py) replays the three
source bases, both domain extractions, the group query, every XCNF row count,
and all six raw solver receipts.

## Frozen input and run boundary

The [protocol](natural_protocol.json) was committed and pushed at
`1e7b35ddaafebd277083a24b741bd26bd7d7316e` before either one-million
call. It fixes the first-trial coefficients
`a=1833078656608906180176350` and `b=163089659398503559150141`, the public
target `(1834913121178197242856238, 8797964190732846213981377)`,
`identity_mask=0`, three small summands, two residual summands, an ordered
first pair, 180 s solver wall cap, one million conflicts, and one solver
thread per arm. The raw `plan.json` SHA-256 is
`fbd3fb943bc0f181797d106c41ee9961b8d70768b5c65a5584002e3c1b5e488f`.
The same 600-orbit small base appears in both arms. Gray has 97 disjoint
residual signed orbits after removing 503 shared pool orbits; hash has 600
disjoint residual orbits. Each orbit has 166 points before folding. The
small base therefore contains 99,600 points and 600 folded columns; the
residual domains contain 16,102 and 99,600 points respectively. This is a
partial-relation search: residual points are not silently counted as solved
factor-base logarithms.

The original compressed S3 archive objects, exact extracted domain inputs,
XCNF streams, and first-attempt receipts are under
[`evidence/natural/`](evidence/natural/). Their SHA-256 values are bound in
the protocol and checked by `natural_audit.py`. The first extraction attempt
erroneously required every signed-orbit row's coefficient to equal 1; its
failure is retained as `natural_extract_first.log`. The corrected extractor
checks the row labels and signed point pairs, then reproduces the frozen
input bytes from the archived objects.

## Observations

The 5,000 and 100,000 conflict calls preceded the sealed one-million panel;
they are exploratory stage rows. The 1,000,000-conflict calls ran in the
protocol's Gray-then-hash order. All calls used local macOS arm64,
CryptoMiniSat 5.14.7, one thread, and native XORs. Solver wall includes XCNF
input loading and excludes export and receipt replay. This is an L0 host
diagnostic; Tier 1 same-point IC/rho timing requires a separately frozen Pod
or instance envelope.

| Arm | Variables | CNF clauses | XOR rows | Conflicts cap | Solver wall ms | Status |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| Gray | 95,068 | 431,648 | 4,980 | 5,000 | 740.140 | `UNKNOWN_solver_cap` |
| Hash | 97,080 | 518,164 | 4,980 | 5,000 | 500.438 | `UNKNOWN_solver_cap` |
| Hash | 97,080 | 518,164 | 4,980 | 100,000 | 12,987.721 | `UNKNOWN_solver_cap` |
| Gray | 95,068 | 431,648 | 4,980 | 100,000 | 13,309.038 | `UNKNOWN_solver_cap` |
| Gray | 95,068 | 431,648 | 4,980 | 1,000,000 | 122,116.859 | `UNKNOWN_solver_cap` |
| Hash | 97,080 | 518,164 | 4,980 | 1,000,000 | 103,886.422 | `UNKNOWN_solver_cap` |

The two formulas share the exact query point. Their SHA-256 values are
`9522505aaf0fa38f298922faad26a0804aede231764ff1f4c9043a7760d0a3c5`
and `98a9d6b7accb111bbcda154e47af45d48eddff022795857af37f095334b253dc`
for Gray and hash. The completed calls produced no SAT model, relation,
rank row, or target-dependent DLP measurement. Their statuses remain
censored; `candidate_id` and `selected_best_total_runtime` stay null.

The known-witness phase control in [README.md](README.md) returned a checked
model when its five Frobenius phases were supplied, while pinning only the
representatives stayed unknown through 100,000 conflicts. The next
discriminating change is to enumerate or otherwise constrain Frobenius
phases without witness information on frozen ordinary queries, retaining all
83 phase choices and charging their full search cost. Its success gate is
verified natural relations and useful rank per query, followed by matrix
solving and one-target descent before a factor-base selection.

Recheck from the repository root with:

```sh
python3 research/n83_chain_xcnf_control_20261019/natural_audit.py
```
