# Review data dictionary

`results/p739-markowitz.jsonl` and `results/p186379-markowitz-t0.jsonl`
contain one setup row followed by one row per attempted frozen target.
`workload_id`, `workload_sha256`, `algorithm_seed`, `target_index`, `planted`,
and `rho` identify the matched source problem and same-process control.
`solver_mode=markowitz` selects only the exact pivot priority in
`PROTOCOL.md`; `walks`, `candidate_lambdas`, and `verified_relations` record
how much of the deterministic relation stream was consumed.

`verified=true` means the recovered candidate scalar multiplied by the
archived generator to the archived target and matched the planted scalar.
`rho.verified=true` is the same independent source check for the rho
scalar. `failure=sparse_matrix_cap` means the candidate stopped above its
recorded `matrix_coefficient_cap`; `recovered` and candidate/rho price ratios
remain null. A process timeout or memory failure is retained in `.exit`,
`.stderr`, timestamps and native `.status.json`, even if no target row was
emitted.

`solver_row_updates` counts modular coefficient updates in exact row
elimination. `solver_index_ops` counts high-level relation copies, incidence
updates, heap events and shortest-row inspections. It does not separately
count BTree comparisons, allocations or cache traffic. The 0, 0.1 and 1
GAE-per-logical-operation band is required to interpret the result.

`results/price-grid.csv` has 36 cells per raw target: three `S/M` prices,
four `I/M` prices, and three native logical-work prices. `pencil_GAE` and
`pencil_over_rho` are empty for an unverified candidate; `rho_GAE` may still
be present when the matched rho scalar verified. The representative cell
uses `M+0.5S+10I` and 0.1 GAE per logical event. Paired CSV/JSON compares
the identical 60 p=739 workload IDs with the source-verified peel control,
retaining lower, equal and higher pricing cells.

`results/validation.json` is the native source-scalar, replay and price
recomputation report; `results/replay-certificate.json` lists the raw and
fresh process replay paths. `results/upper-process-replay.json` separately
checks the fixed upper-size setup and process outcome, including a
setup-only timeout if no target row is emitted. `results/status.json` is
the native updated distinct-cohort count and process outcome;
`results/lead-result.json` collects the same fields in a compact review
record. `VALIDATION.json` pins the reports and matched rho band for the
completed p=739 slice. `results/manifest.json` pins source, executable,
raw, replay, PDF, graph and derived-record hashes.
The full 1,260-target cohort remains open until each original workload has
a distinct source-verified candidate outcome or explicit failure record.
