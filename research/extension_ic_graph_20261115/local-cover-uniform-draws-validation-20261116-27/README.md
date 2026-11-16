# Selected modular uniform draw validation

[Report](report.pdf) · [Complete Rust summary](summary.json) · [Flow excerpt](flow.jsonl)
· [Prospective protocol](../AMENDMENT_22_UNIFORM_DRAWS.md) · [Custody](local-evidence.json)

Source15121ac779cef32bc173916e74ce26e2a3976f09 was published before the sole
local call. Exit0in69.824seconds, two starts/two completions, no retries.
Both public-input workers recover scalar119700578327681984 and pass native
and outer group replay. Pivotgraph projects520rows to390, replays every
output witness and uses no fallback. The original copied Rust qualifier
reports no global problems and passes all three required-counter and
endpoint checks. [pcc:unverified]

| Selected sampler event | Native IC | Pivot graph IC |
|---|---:|---:|
| Calls |2540|2410|
| Actual raw draws |2618|2479|
| Rejected draws |78|69|
| Known native integer price, bitops |64315008|60901824|
| Unpriced integer_sub64 |5080|4820|
| Unpriced integer_clz64 |2540|2410|
| Unpriced integer_shl64 |2540|2410|

All per-phase/conservation/repricing checks pass; foreign and other-modulus
hooks are zero. These native integer prices are separate from the primitive
subtotal. Generator internals, other samplers and native/container work
remain incomplete. Complete totals stay null and comparative/followup
eligibility stays false. This is accounting validation without a speed or
rho-parity claim. [pcc:unverified]

The complete56332-byte summary is copied unchanged into Git. The original
71991-byte raw exceeds the64KiB file limit and remains exact, with its hash,
under /workspace/ic-local-runs/local-cover-uniform-draws-validation-20261116-27.
The local directory also retains immutable copied qualifier/driver/source
checkpoint, original progress, host facts and pre/post receipts. The Rust
flow exporter preserves record order/identity/status/scalars plus each
original line's BLAKE3 digest; it is an excerpt, not replacement raw. Source
and release build/strictClippy receipts are retained. [pcc:unverified]

All119frozen source and pre/post hashes pass. The single final PDF page was
visually inspected; exactPDF/SVG/150dpiPNG benchmark copies are hash-checked.
The predeclared Typst template used reserved helper name show; its original
failed source/log remain local and the log is copied here. The presentation
copy renames the helper to display_value; numerical data and frozen trial
remain unchanged. [pcc:unverified]

No independent signed verdict, canonical encrypted-schema admission,
strongest-external comparison or successful S3 checkpoint upload is asserted.
The user-authorized local backend and Git/source/local evidence custody
were used. [pcc:unverified]
