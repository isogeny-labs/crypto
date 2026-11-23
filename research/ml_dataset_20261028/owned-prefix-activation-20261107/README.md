# Production strict-prefix parser activation

Activated `src/ml_dataset/owned_search_wire_prefix.rs` as the `strict_prefix` child of the production decoder. Its bytes exactly match the frozen, reviewed strict_prefix.rs proposal. The existing decoder is unchanged except for child registration and a comment. The parser shares the decoder Reader, Meter, schema, limits, owner and absolute deadline.

The standalone harness imports the actual production decoder by absolute path and runs the same 13 frozen controls. Compilation and tests exit 0. Every 6,762 byte cut of the 6,761-byte durable snapshot is checked; identity first becomes available at byte 789. The actual refused 4,096-byte capture prefix is admitted only as an incomplete identity-bearing prefix. Malformed suffixes, wrong shape, duplicate keys, impossible UTF-8, resource refusals and deadline expiry cannot become accepted prefixes. Retained raw input and accepted accounting are checked.

This makes the parser available to production callers but does not wire historical positive export. Prefix shape is not original-source authentication, arithmetic validation or report/capture authorization. No full Cargo build, CLI acceptance, cloud work or dataset generation occurred. Source and fixture hashes were reverified after tests.

Sources checked before activation: the frozen owned-search-strict-prefix-20261107 README, SOURCE_BINDINGS.json, VALIDATION.json, parser and 13-test harness; the production decoder; repository AGENTS.md and latest session log. This activation preserves the already reviewed method and makes no new literature, performance or mathematical claim. Commands and raw logs are retained alongside a patch. Parent owns session-log publication and Git delivery.

Git log copies normalize terminal whitespace; immutable original bytes are retained in the S3 archive named by S3_RECEIPT.json. Packet manifests describe those archived originals.
