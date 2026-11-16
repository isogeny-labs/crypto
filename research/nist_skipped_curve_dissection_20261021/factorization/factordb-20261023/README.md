# FactorDB check: 2026-10-23 UTC

FactorDB supplied **no additional factors** for the nine exact DiSSECT
timeout targets or the remaining 444-bit cofactor. Native PARI verifies
all ten returned products. Every API partition is a grouping of whole
pieces of the [existing local partitions](../rounds/R02-20261022/results.json),
so none splits a local residual. This is a dated database observation,
not a research novelty or priority claim.

The residual query returned `C`, with the input itself as its only piece.
Its retained HTTP date is 00:57:58 UTC. The original-target requests ran
from 00:59:23 to 00:59:32 UTC. `FF` means fully factored; `CF` means
composite with factors known; `C` means composite. API statuses are
observations; local primality claims use replayed PARI certificates.

| Exact target ID | FactorDB | Certified local result |
|---|---|---|
| p256-4p-minus1 | FF | complete; same partition |
| published_224-twist2 | C | complete; more factors locally |
| published_256-6n-plus1 | CF | complete; more factors locally |
| published_256-7n-minus1 | CF | complete; more factors locally |
| published_256-twist2 | CF | complete; more factors locally |
| skipped_224_1485-conductor4 | CF | complete; more factors locally |
| skipped_224_5736-twist2 | CF | complete; more factors locally |
| skipped_256_876-4n-minus1 | CF | complete; more factors locally |
| skipped_256_876-twist2 | CF | partial; same partition |

[Raw responses](raw/), exact request integers, HTTP headers, curl statuses
and UTC timestamps are retained verbatim. The web reader could not access
the initial URLs; direct HTTPS GET requests to the public API succeeded.
No factor submission was made. Large numeric FactorDB record IDs remain
in untouched raw JSON and are not rounded into summary fields.

The [offline reader](check_snapshot.sh) validates decimal strings and
positive exponents before emitting GP literals. The [native checker](verify_partitions.gp)
verifies products and whole-piece grouping. A wrong-integer control fails
both product checks. All ten real checks and that negative control pass in
the [retained replay](verification/verification.jsonl). This computes no
new factorization and treats no probable prime as a proven prime.

```bash
GP_BIN=gp bash check_snapshot.sh /tmp/nist-factordb-offline-replay
```

See the [completion report and graph](../rounds/R05-20261022/reports/completion-20261023T0100Z/REPORT.md)
and [inspected PDF](../rounds/R05-20261022/reports/completion-20261023T0100Z/report.pdf).
Requests use `https://factordb.com/api?query=<integer>`; exact integers are
in [queries.tsv](raw/queries.tsv), with the residual fixed in the verifier.
