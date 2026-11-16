#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Combined qualification v4 lifecycle successor
[pcc:unverified] Source-only lifecycle correction; T-2147. No compiler, tests, lock generation or payload.
#image("flow.svg",width:100%)
== Concrete source changes
The v3 worker failed before materialization, compilation, or tests because its
monitor traversed the private target directory before the materializer created
it. V4 permits an absent target only while the command label is `materialize`.
If target exists it must be a real directory; every other command still fails
closed when target is absent. Workspace, log, target, disk, deadline, and
permit checks are unchanged.
== Preserved panel and limits
Exact 28 signed cases, seven Labels cases, 11 native driver calls, forced-fallback and zero-vector probes, and four adapter negatives remain. Combined reservation is 2400 seconds: 1800 build plus 600 execution; 2 CPU / 16 GiB, one attempt without automatic retry. The lock gate preserves 60 seconds total and 45 shared resolver seconds, 1 CPU / 2 GiB, private network and version-only rustc proxy. Both resolutions charge the same deadline.
== Pending admission and evidence
V4 has no release. Independent review, publication, a post-pivot ledger, live
lease, and serial reconciliation remain pending. The prior v3 attempt is fully
charged in campaign accounting and is not retried. No runtime completion or
cost ratio is measured. PDF, SVG, CSV, JSON, and the source patch describe the
prospective lifecycle correction only.
