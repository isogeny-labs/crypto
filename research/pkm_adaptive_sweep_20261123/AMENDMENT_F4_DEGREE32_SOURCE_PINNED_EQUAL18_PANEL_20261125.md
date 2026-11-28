# Amendment — source-pinned equal-18 three-instance panel — 2026-11-25

The base protocol `PROTOCOL_F4_DEGREE32_SOURCE_PINNED_EQUAL18_20261125.md`
selected seed `20261185` from the frozen size-only scan
`20261181..20261220`. Before any timed call on the extension set, the scan was
continued over `20261221..20261280` and written to
`source_pinned_size_scan_20261125.jsonl`.

Two further seeds matched the degree-32 isogeny arm and
`prime-abscissa:below=32` at exactly 18 abscissae:

- `20261221` — `icv1-fp13-tm101-1737775c`
- `20261276` — `icv1-fp13-tm137-9a89da76`

The three-instance panel is frozen at `20261185`, `20261221`, `20261276`.
Every instance uses the same source-pinned binary, 16 repeats, m=2 Semaev F4
degree 8, 1,000 target trials, a five-second solver-call budget, one rho
context run, mirrored interval/candidate/candidate/interval order and a
240-second whole-call cap. Each instance has its own append-only store.

The other exact matches in the scan (sizes 14, 15, 17, 20, 21) are not used in
this panel; they are recorded but not timed. This remains a selected-instance
component panel, not a random-instance replication and not a same-target rho
crossover.
