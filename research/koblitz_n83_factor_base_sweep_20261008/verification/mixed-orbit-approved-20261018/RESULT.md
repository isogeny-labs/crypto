# Approved K1182/K2048 mixed-orbit replication

The single approved, S3-backed replication imported the exact primary
`a=0`, `public_x_hash`, seed `2026100801` K1182 small base and K2048
envelope, then ran one public fixture-zero 3+2 partial-relation search.
The import passed with 196,212 small points in 1,182 signed-Frobenius
orbits and 143,756 residual points in 866 orbits. The search reached
`UNKNOWN_search_cap` at exactly 5,000 conflicts in one SAT call. It emitted
zero models, partial candidates and completed rows **within that cap**;
natural relation yield and useful rank remain unknown. This reproduces the
earlier [bounded K1182/K2048 observation](../mixed-orbit-search-15000-20261012/RESULT.md)
without creating a complete-runtime comparison.

Both input objects were freshly fetched from their content-addressed S3 keys.
The K1182 object has BLAKE3
`76e1cbe4ea5a9d26e5b24a2d27f9401271dc471846529190a0416d8e09837a92`,
SHA-256
`eb342b574e244e1e67e8e78c805f33ee88edcdb4f3a14d4114e6f413beede380`
and 5,956,114 compressed bytes. The K2048 object has BLAKE3
`82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851`,
SHA-256
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`
and 10,313,647 compressed bytes. The retained [S3 receipt](evidence/inputs/inputs-receipt.json)
binds their full URIs, lengths, hashes, metadata and equal HEAD/GET ETags;
S3 reported AES-256 encryption. The worker ELF was built offline from clean
source commit `d5d3ec3a72cb801812e75a60776b5768e6f218be`, with SHA-256
`68d03989b8f2a4c3bec9c7fc52af42a8f2ddbc1da8cadd4b2d9fede8199ddca2`.

| Stage | Observed wall | Conservative round charge | Disposition |
| --- | ---: | ---: | --- |
| Offline Linux build | 272.240 s | 600 s | `PASS_build_only`; live 4-CPU/8-GiB guard, clean exit and cleanup |
| Exact nested-base import | 8.156 s | 360 s | `PASS`; no search in import stage |
| One 3+2 search | 429.535 s container | 1,200 s | `UNKNOWN_search_cap`; 5,000 conflicts, zero models |
| Cleanup, audit and receipts | Included in 439.580 s run-supervisor wall | 240 s | Three containers removed; source and event replay passed |
| **This round** | 711.821 s build plus run supervisor | **2,400 s** | `PASS_VALIDATED_CENSORED` audit |

The event records 101,636 peak SAT variables, 465,738 orbit-domain clauses,
714,072 peak clauses and 4,980 XOR rows. Its first-trial coefficients match
the frozen public schedule; there was no candidate for group or graph
completion. Peak search cgroup memory was 454,385,664 bytes under a live
one-CPU, 4-GiB, zero-swap, networkless, read-only-root guard. These are
single capped stage observations on a shared host, not an index-calculus
speed ratio.

The [protocol](PROTOCOL.md) and [authorization](authorization.json) bind the
additional 2,400-second grant. The v2 supervisor records its historical
12,600-second baseline as required by that frozen source; later retained
runs make it an obsolete **current** cumulative figure. The conservative
charge for **this round** is 2,400 seconds, with zero of that grant left.
The current all-study cumulative charge is left uncomputed pending a
reconciliation of intervening gates. The
[independent audit](evidence/audit.json) passed against the original and a
[retained evidence copy](evidence/retained-copy-audit.json) with byte-identical
outputs; a [forged event](evidence/negative-control.json) changing 5,000
conflicts to 4,999 was rejected. [Validation](VALIDATION.md) and the
[evidence manifest](evidence/evidence-manifest.json) retain the exact scope.
`total_index_calculus_runtime_ms` and `selected_best_total_runtime` remain
`null`.
