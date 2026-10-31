# CP-C1 correction source review

Reviewed corrected `protocol.rs` SHA256
`384109e58a99ecb03200b8628bb285cff1c707839c796c357ff06f09db318537`.
The exact copy is `protocol.rs.corrected`. Date: 2026-10-29.

The correction replaces Rust lower-exponential digit selection with
`serde_json::to_writer` into a fixed 64-byte stack buffer. It then independently
extracts leading significant digits and decimal exponent from either fixed or
scientific output and applies the pinned fixed/scientific notation thresholds.
The existing 256-byte precharge covers the fixed formatter/digit scratch.
This addresses the concrete nearest-even decimal-tie mismatch in CP-C1 without
importing the producer canonicalizer or changing global JSON number handling.

The original 14 positive and 11 negative frozen vectors are unchanged. Five
additive exact-byte tie regressions cover the reported family. The consumer owner
reports those tests passed; this reviewer did not execute the actual module.
Final source-bound suite evidence remains the owner's gate. The retained
primitive comparison's 40 disagreements remain failed historical evidence and
must not be relabeled as a passing corrected-module test.

No additional actionable defect was identified in this correction. This narrow
endorsement does not prove equivalence for every binary64 value, protocol schema
semantics, actual filing time, parent integration, runtime or hardware truth.
