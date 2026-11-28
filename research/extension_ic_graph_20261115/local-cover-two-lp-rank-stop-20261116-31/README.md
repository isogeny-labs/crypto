# Graph IC: stop collection at certified rank

[Report](report.pdf) · [Original native summary](summary.json) · [Flow](flow.jsonl)
· [Frozen protocol](../AMENDMENT_26_RANK_STOP_COLLECTION.md)
· [Exact original custody](local-evidence.json)

Original IC, fixed-cohort two-LP IC and rank-stopped two-LP IC all recover
146169426957327337 from the same public weak-family input, passing native
and additional elliptic replay plus held-out agreement. The copied Rust
qualifier also checks independent affine target recovery, every graph
certificate, exact source-prefix equality and the common descent prefix.
[pcc:unverified]

| Observation | Fixed 2LP | Rank stop |
| --- | ---: | ---: |
| Unique B values | 1018081 | 798720 |
| Verified original sources | 643 | 497 |
| Complete affine rows and replays | 521 | 376 |
| Independent final rank | 371 | 371 |
| Accepted descents / attempted residuals | 2 / 2087 | 2 / 2087 |
| Observed relation subtotal, bitops | 5875912460348 | 4608739628848 |
| Observed LA subtotal, bitops | 291342950896 | 790731766146 |
| Observed primitive subtotal, bitops | 8264537803730 | 7496753787480 |

These are incomplete primitive subtotals. Original IC's observed subtotal
is5086671014334bitops. Rank stopping reduces the observed subtotal against
its fixed-cohort reference, while the added rank checks increase LA work.
No complete-cost ranking, wall-speed ratio or rho result follows.
[pcc:unverified]

The first checkpoint uses369sources at B574464, producing247homogeneous
certificates and independent rank247, below the target369. The second uses
497sources at B798720, producing374certificates and rank369; collection
stops. All621checkpoint certificates replay, their exact independent ranks
match, every checkpoint's counters fit the LA phase, and all376final affine
certificates replay. The final system independently determines the same
actual scalar at rank371. No fallback, fill refusal or changed criterion.
[pcc:unverified]

Source564853c1e5d9708cbe462f5487b8198b63738298 was pushed before the sole
call. Exit0in194.973seconds, one fixture and three workers complete;
one validation/zero warmups, CPU0/8GiB/one claimed thread,60secondsfixture/
900perworker/2800whole,no retries. All230source/precall/preworker/post
hashes pass. Process wall is a practicality observation without A/A
controls or intervals. Source hashes were checked before post-call
publication-documentation edits. [pcc:unverified]

The7398-byte original IC raw,24073-byte original summary, all progress,
flow, argv, status and hashes are exact Git copies. Full505331-byte fixed
raw,621964-byte rank-stop raw and1151925-byte native raw export remain
exact, untruncated and hash-bound under the original local run directory.
Immutable copied binaries/qualifier/wrapper/source and host facts are also
retained there. The native export contains all workers/protocol/fixture/
flow/analysis and BLAKE3 pins to originals. It explicitly claims no
canonical experiment.export/v1 schema conformance because the canonical
skill/schemas are encrypted and unreadable here. [pcc:unverified]

The single final PDF page was visually inspected. Exact PDF/SVG/150dpiPNG
benchmark copies and local originals pass their hashes. A table-layout
clipping issue and prose typo were fixed only in the presentation copy;
the frozen template and initial render remain locally hash-bound. Raw,
criteria, copied qualifier and original summary remain unchanged.
[pcc:unverified]

PCC-20261116-6dce0f records this producer assertion; standing is unverified.
Native/container/PRNG coverage and the actual strongest-rho pricing
contract remain open. No independent signed verdict, complete total,
fastest-external result, admitted leaderboard row or successful S3 upload
is asserted. [pcc:unverified]
