# Unvalidated benchmark archive

This archive retains dated benchmark summaries that are not admitted by the
results table in [`README.md`](README.md). Each entry in
[`ARCHIVE.json`](ARCHIVE.json) stores the exact former row text, its SHA-256,
the disposition and reason for removing it from the admitted table, and paths
to the supporting study records. The archived wording is historical evidence;
it is not a validation receipt and does not establish the truth of its numeric
claims.

Allowed dispositions are `incomplete`, `censored`, `documentary`, and
`superseded`. The validation gate checks that the row is absent from the
admitted table, the digest matches the exact stored text, and the cited source
files exist. For pull requests it also requires a disposition for every row
removed from the base table and prevents prior archive entries from being
deleted or changed. An archived row that remains in the admitted table still
fails the normal validation gate.
