# Total progression shifts with cached samples

See DERIVATION.md for the segment proof, validation scope and replay command; report.pdf is the inspected visual report. Producer evidence is unsigned, PCC-20261107-020740. [pcc:unverified]

## Storage clarification

The d+1 persistent sample elements in DERIVATION.md count live values, not allocated bytes. The retained Rust Vec can have spare capacity after the final setup sample is pushed. Charge its full allocation capacity, Vec metadata and allocator overhead; the storage bound is O(d). No exact byte-allocation claim is made. The implementation and validation receipt are unchanged by this clarification.

See ERRATA.md for the additive correction of report-render status after the derivation was pinned.
