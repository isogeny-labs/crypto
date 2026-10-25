# Complete primary-N83 XOR verifier inputs in S3

The [frozen manifest](manifest.json) names exactly 11 source-pinned inputs
for the four first-pattern primary `a=0` hash/Gray 5+1 and 4+2 XCNFs:
four pre-existing XCNFs, three compressed K600 factor-base archives and
four `export.json` files. The seven new objects were written under the
existing panel `inputs/` prefix using `If-None-Match: *`, explicit SHA-256
checksums and AES256 server-side encryption. The input role's existing
`GetObject` permission covers every new key; its policy was not changed.

The [publish receipt](receipt.json) records each object's byte length,
SHA-256 checksum header and encryption. Every object was fetched again with
S3 checksum mode enabled. The [independent byte auditor](audit.py) returned
`PASS_COMPLETE_INPUT_REPLAY` for all 11 downloaded files, comparing the
original XCNF admission receipt, source export receipt, compressed archive
SHA-256 and BLAKE3, formula BLAKE3 and export metadata. A scratch mutation
of `exports/hash_4_2/export.json` was rejected at its SHA-256 and length.
The [fetch command](fetch.py) was separately run against real S3 and returned
the same 11-object pass. A replayed download shows object bytes at the
observation time; this bucket has no asserted object-lock immutability.

This closes the *input custody* gap for the intended Linux external verifier.
The original `a0/objects/` keys remain the canonical factor-base archive
location; these three panel copies are byte-identical. No Linux instance or
solver ran in this extension. The previous EC2 account-admission block still
applies to that attempted account. The worker role lacks `GetObject` on
`outputs/*`, so a separate reader must fetch and audit uploaded results.
Natural relation yield, graph closure, rank, linear algebra, unseen-target
descent, total cold runtime and a matched rho control remain unmeasured;
`selected_best_total_runtime` remains `null`.
