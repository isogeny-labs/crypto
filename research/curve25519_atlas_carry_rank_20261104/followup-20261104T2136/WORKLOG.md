# Completion work log

This follow-up preserves the original PR, source checks and integer timeouts.
It adds a bounded bit-vector representation, native solver controller, canonical
protocol/export adapters, and a fresh ledger import. All component observations
remain producer observations [pcc:unverified].

The first scratch protocol-format attempt was frozen with an incorrect literal
commit id. It was caught before any run consumed it. The unexecuted file remains
at `/tmp/curve25519-control-example-1/protocol.json`; it is invalid setup metadata,
not an experiment or evidence. Subsequent examples derive their revision from
`git rev-parse HEAD` and check it with `git cat-file` before use. Do not use or
cite the first scratch protocol as a source binding [pcc:unverified].

The original published branch ledger is retained byte-for-byte in
`original-branch-ledger.jsonl.gz`; `ledger-import.json` records its digest and
the unchanged claim imported using the maintained append helper. The fresh
ledger extends main; neither published history was rewritten [pcc:unverified].
