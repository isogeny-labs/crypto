# Protocol addendum 2: lossless seed-custody encoding

Filed before any timing run.

This addendum supersedes only the physical representation named in section 6
of `PROTOCOL.md` and in `seed-manifest.json.full_mapping_artifact`. The complete
derived mapping is committed as `derived-seeds.json`, using schema
`blocked-bsgs-rho-wall/derived-seeds/v2`, instead of one JSON object per line in
`derived-seeds.jsonl`.

The v2 object is a lossless indexed encoding. Curves appear in the order frozen
in `seed-manifest.json`; panels use the frozen panel names; and entries within a
panel are ordered by replicate and then item. It stores every target-derived
u64 together with its target scalar, every rho-walk-derived u64, every schedule
code, and every bootstrap ordered-selection-stream digest. Derived u64 values
are fixed-width 16-digit lowercase hexadecimal strings. Explicit dimension and
count fields make every logical v1 row recoverable without executing a solver.

Before timing, the native emitter and an independent verifier must expand the
v2 object to the same logical records, reproduce every value from the frozen
SHA-256 derivation, reject duplicate derived u64 values, verify the fixed order
and counts, and agree on the SHA-256 of the canonical committed file.

This amendment changes no seed, stream, curve, panel, replicate, item,
schedule, bootstrap selection, algorithm, estimand, censoring rule, or decision
threshold. The representation was changed because the fully expanded JSONL is
4,650,727 bytes and requires an unavailable external artifact store, whereas
the indexed object retains all values within the repository's ordinary tracked
file limit. No timing data had been collected when this correction was filed.
