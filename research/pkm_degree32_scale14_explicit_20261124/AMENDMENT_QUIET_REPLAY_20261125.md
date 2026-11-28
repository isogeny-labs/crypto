# Fresh quiet-load mirror after a censored start

The source-pinned selection and all size-only preflights in
`source_pinned_explicit14_20261125.jsonl` remain fixed. Its first curve-6
control invocation was terminated when unrelated Rust compilation began; that
store is immutable and its abort record remains evidence.

The qualifying timed mirror uses
`source_pinned_explicit14_qualifying_20261125.jsonl`. Before timing, copy the
original header, all preflight and size-check records, `selection`,
`amended_selection`, and `selection-correction:1` byte for byte into the new
store. Exclude the prior `load-check` and abort records, and add a provenance
record with the source-store SHA-256 and the copied-record count. This copies
only frozen instance and base-size evidence. The prior abort stays in the
source store; it is not a qualifying timed call.

Run curves 6, 13, 36 in that order, each in the frozen
control/candidate/candidate/control order. Use the retained `ic` SHA-256
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`.
The guarded runner must find no competing build or research solver before and
during each call. Preserve any failed or censored call in this new store. If
load contamination occurs, stop this qualifying sequence and report it.
