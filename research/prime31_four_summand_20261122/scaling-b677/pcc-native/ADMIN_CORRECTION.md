# Native metadata canonicalization guard

The first administrative build and preparation are preserved in
`admin-build-01/`, `admin-prepare-01/`, and `prepared-01/`.
`PCC-20261123-e96cbf` was prepared locally but was never registered in the
canonical ledger or copied into the canonical claims directory.

Independent source review identified a general input edge in the reused
canonical JSON helper when it is compiled with `serde_json`'s
`arbitrary_precision` feature: an overflowing exponent token such as `1e999`
can be retained while `Number::is_f64()` returns false. Python PCC would parse
that token differently. The actual prepared claim contained no such numeric
token; its original successful metadata check is retained as an observation of
that fixed artifact, not a proof of general canonicalization equivalence.

Before editing, the exact compiled writer source was copied directly to
`source-snapshots/admin-build-01/main.rs`. Its SHA256 is
`9d1487e712d1738c7184c1444026f39242ae1588a9a5eed8f3db9552ddd03b5c`.
The byte-selected production helper remains unchanged. A wrapper now rejects
decimal/exponent spellings and integer negative zero before canonical hashing.
The next preparation receives a new claim ID and supersedes the unregistered
first preparation. Because its live writer-source pin has changed, the first
preparation is historical; its preserved source copy identifies the original
bytes. No claim JSON, scientific source, raw qualification receipt, source
inventory, or canonical ledger entry was rewritten.
