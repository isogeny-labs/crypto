# Formatting and source-binding clarification

Core qualification1d67ff remains60/61/62 passing release tests and strict lint39. The phrase no tokens/semantics changed in core-tests-39.md was too strong: rustfmt may change syntactic punctuation while preserving semantics. The intended assertion is formatting only, with final root compilation/testing/lint on the formatted source. No test or timing outcome changes. [pcc:unverified]

The earlier claim pins a mutable production source path. quotient-core-source-39.rs.txt is an immutable byte snapshot of that exact formatted source for future historical recovery; this sibling is the canonical source-binding clarification. Preserve the earlier claim and subject without rewriting them. Exact source commitc394bb471 and source bundle retain the production version. [pcc:unverified]
