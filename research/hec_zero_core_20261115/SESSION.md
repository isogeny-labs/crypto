# Genus2zero-core session, 2026-11-15

## Synthetic gate
### What was done
Protocolee702c60ebpreceded engine/gate source. Implemented shared public-input
engine for H0/Hpositive with same catalogue,16public jumps,oracle,raw-label
stream. Synthetic prefixes compare all partitions against solve_fast; explicit
unbalanced-cycle pin then target control included. Real-stream source freeze
and checkpoint precede execution. Expected scalar stays in harness.
### Decisions made
Use existing full weighted forest/Corewithout root-only edge rejection. Engine
has a stopping API for future cold comparison, but no timing is measured now.
### Open threads
Commit/build/checkpoint and execute fixed six-target real-stream gate; compare
first target trials to prior calibration, preserving all errors/caps.
### Errors
Initial build had a cast/comparison parsing error; compiler stderr retained.
Parenthesized cast fixes syntax before scientific execution; no real stream
ran and no protocol outcome or cap was changed.
