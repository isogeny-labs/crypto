# Half-GCD projected-resultant backend

The successor source uses recursive half-GCD matrix batches with lifted degree/leading-coefficient metadata. It preserves private multiplicative quotient classes and the single terminal geometry projection. Previous backends remain immutable references.

Build plus eleven selected native methods pass; each filters121 other methods. New exact-trace panel:960 cases,1679 recursive calls,17 final remainder degrees changed by lifting. Every transition, matrix action/determinant, returned pair and projected resultant agrees with the references. Synthetic geometry:2500 cases,1349 valid/1151 rejected,174 recursive calls,one projection per valid case. Earlier determinant,root,quotient and paired-source checks remain passing. [pcc:unverified]

The trace checker uses separately written schoolbook polynomial division; the resultant reference is the preceding Euclidean implementation. These are producer-authored checks, not independent review. Sixteen compiler warnings are retained in build.log. No native failure. Source regeneration is byte-identical; dependency/compiler/binary hashes and exact commands are recorded. Build90s,major methods60s,others30s,one local invocation. No cloud, timing, or recovery comparison; original4563-case cohort unchanged.

Replay generate.py then run-record.py from this directory, invoking them from repository root in a disposable checkout: run-record.py overwrites its outputs. DERIVATION.md records the lifting invariant, final-degree correction, cost boundaries and primary source. report.pdf is rendered and visually inspected.

Next: count complete polynomial work against both preserved backends before selecting a cutoff, then address implicit orbit-product evaluation and compact nonlinear decoder construction. The present batching does not remove geometry point lists or dense compiler rows.

Evidence: PCC-20261107-005cac, unsigned producer attestations.
