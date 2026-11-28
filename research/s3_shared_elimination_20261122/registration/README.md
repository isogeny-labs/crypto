# Unsigned development statement

The native metadata writer registered
[PCC-20261122-2fb226](../../../pcc/claims/PCC-20261122-2fb226.json)
after publishing source/evidence commit
`a9075a333a235a0e64557235e40a556928a702f1`.

The statement pins the original development receipt at SHA-256
`2fb226cbeb9f4d10eed030299a5e2cd92ad585899d0d3a6545c5507d2eb54293`.
That receipt and all its 43 referenced artifacts remain unchanged. These
later registration records are outside its artifact list, avoiding a circular
binding. [pcc:unverified]

- `preview.json` contains the exact proposed claim and ledger entry reviewed
  before the append.
- `append.postcall.json` records one appended entry after checking the 1,368
  existing entries, committed prefix, expected HEAD and tail, receipt bytes,
  and reviewed claim digest.
- `source-bindings-after-main.postcall.log` records the unchanged captured
  source hashes after merging main at
  `8cc9e4853f709561e7bab198015c9a008044d606`.

Claim digest:
`cecca2493788fe2f4358557e2cceba6ae3b3952b3cfb592032e65f1a387155cf`.
Appended ledger-entry digest:
`e03d95684ad1e73aca98f20d06003ce4ce51548155e8bc5b439aa08015f45962`.

This is a `kind=statement`, `claim_type=data` record with an unsigned receipt
attestation. The writer did not run a scientific verifier, consult signed
standing, or create a signature. It cannot promote a statement to verified.
Same-team source review also supplies no signed standing. Results remain
`[pcc:unverified]`; performance and full-library gates remain open.
