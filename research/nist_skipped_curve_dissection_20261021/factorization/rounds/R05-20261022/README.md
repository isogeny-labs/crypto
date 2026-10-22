# R05: local GNFS sieving in progress

**Interim status:** the two native lattice-sieving workers were running at
the timestamp in [OBSERVATION.json](OBSERVATION.json). This is a dated
observation, not a claim that the process remains alive indefinitely.
The native log reports raw relations; these are not independently
deduplicated or sufficient to establish a factorization.

The exact P-256 block-876 residual is still 444 bits. R04 supplies a
[validated selected polynomial and portable job](../R04-20261022/README.md),
so this allocation starts sieving directly. The mathematical total remains
eight complete integers and thirteen resolved timeout fields. No new factor
is claimed by this interim report.

The [frozen protocol](PROTOCOL.md) allows one local 3600-second native
allocation, outer 3660 seconds plus 30 seconds grace, two threads, and
1 GiB maximum output per file. This protects limited workspace disk;
collection may stop before the configured 46,000,000-relation target.
That target is not a measured count, runtime estimate or guarantee.
No paid or remote machine was launched.

Native source files were copied into an immutable external snapshot and
each blob hash was compared with the frozen commit. An initial broad Git
archive export failed on missing promisor objects before any native control
or production run; the failure is retained. The version-2 receipt control
passes on a small known input, without claiming additional GNFS coverage.

See the [interim report](reports/run-20261022T1027Z/REPORT.md) and
[inspected interim PDF](reports/run-20261022T1027Z/report.pdf). Full raw
archive, final native status, prime/product replay and final report remain
**pending until this actual worker terminates**. The local raw directory
is recorded in the observation. The distributed backlog card is separate
and remains draft/unqueued; this local allocation is real.
