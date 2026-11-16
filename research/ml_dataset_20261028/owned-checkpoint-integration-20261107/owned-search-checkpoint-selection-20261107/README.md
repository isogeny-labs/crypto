# Historical checkpoint candidate selection

Candidate selection is deliberately separate from admission to row export. Given an ordered, bounded list of classified snapshots, select the latest complete snapshot. One terminal bound truncated prefix permits selecting only its immediate complete predecessor. Reject gaps, duplicate indices, any malformed/unavailable classification, a nonterminal prefix, multiple prefixes, no complete predecessor, and owner/deadline refusal. The helper scans every classification, performs no allocation, and charges one work unit per descriptor to the supplied original owner.

State classifications are ordinary internal data, not unforgeable authority. Integration must obtain Complete only after bounded full decode and original identity/map binding; BoundTruncatedPrefix requires strict schema-prefix validation and complete identity binding. A plain Truncated decoder error is insufficient. Local caps or identity uncertainty become Unavailable, never a successful prefix. Invalid input cannot be bypassed by an earlier positive. Candidate selection alone provides no row or mathematical claim. Selected original descent, equation lineage and committed basis identities must subsequently verify; a failed mathematical audit must not fall back to another candidate.

The proposed central route must retain and revalidate every source/chunk/outcome, distinguish selected snapshot from terminal head, and use a new historical-checkpoint scope with missing producer report joins explicitly false. Existing v1 reporting is not changed. Full integration remains outstanding.

Thirteen focused tests pass. Replay: rustc +1.98 --edition=2021 --test selection.rs -o tests; ./tests --test-threads=1. Current production source is unchanged. No cloud job, scientific generation or full crate build occurred.

Sources: current owned_search_central.rs decode/revalidation and latest-head report join, owned_search_report.rs stopped-budget behavior, owned_search_wire_decode.rs bounded decoder, and the independently reviewed partial-export design. This is software custody/control-flow work, not a fall-degree claim.
