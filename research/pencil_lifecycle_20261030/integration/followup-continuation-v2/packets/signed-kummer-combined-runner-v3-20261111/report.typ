#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Combined qualification v3 successor
[pcc:unverified] Source-only prospective implementation; T-2116. No compiler, tests, lock generation or payload.
#image("flow.svg",width:100%)
== Concrete source changes
Adds the omitted driver-controls.rs, authenticates emitted trace headers, and compiles the adapter through its standalone workspace with its own actual lock.
The 50-file inventory includes the exact pre-existing driver-controls.rs. The worker stages independent actual locks at the driver and adapter workspace roots. Adapter compilation selects its own manifest and reporting feature. Trace acceptance compares published revision, staged pruned source hash, workload, run ID and complete native header. The producer's source_verified Boolean remains only a scalar-result check.
The deliberate first header-write fault has an empty prefix and position zero. Its identity is explicitly unavailable; all other emitted qualification traces require the expected header. Two source-written identity rejection controls are present but unexecuted.
== Preserved panel and limits
Exact 28 signed cases, seven Labels cases, 11 native driver calls, forced-fallback and zero-vector probes, and four adapter negatives remain. Combined reservation is 2400 seconds: 1800 build plus 600 execution; 2 CPU / 16 GiB, one attempt without automatic retry. The lock gate preserves 60 seconds total and 45 shared resolver seconds, 1 CPU / 2 GiB, private network and version-only rustc proxy. Both resolutions charge the same deadline.
== Pending admission and evidence
Actual driver and adapter locks are null. Source publication, lock publication, deadline, fresh cache, owner, ledger and independent review are pending. The stale campaign constant is corrected from 43200 to the user-authorized 129600 seconds, with stop deadline 2026-11-11 at 12:00 UTC. Actual prior and remaining worker time remain unbound until the final ledger. No runtime completion or cost ratio is measured. PDF, SVG, CSV, JSON and source patches represent the proposed construction only. Earlier evidence remains unchanged.


V3 corrects F2-R1: parse the actual Rust Record envelope, require integer sequence zero, then inspect event.kind and event.data. Identity rejection controls now construct this same envelope. V2 remains preserved as reviewed evidence; source review is pending.
