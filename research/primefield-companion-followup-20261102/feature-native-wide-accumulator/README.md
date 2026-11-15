# Exact wide accumulation with one final reduction

The private Horner fold uses exact i128 arithmetic and one final Euclidean
remainder. A range proof covers every intermediate operation for at most63 digits
and a term count bounded by a64-bit usize. A compile-time guard enforces the
pointer-width assumption. Constructor, digit generation, recovery and source
remain unchanged.

All283 native methods pass with overflow checking enabled, including1024 extreme
digit vectors,4096 modular comparisons and all8 complete recovery receipts. For
the saved packet the accumulation uses1 reduction instead of9; this is a derived
operation count, not a runtime measurement. See DERIVATION.md, control.json,
source-lock.json, raw logs and the inspected report. No cloud/panel run occurred.
