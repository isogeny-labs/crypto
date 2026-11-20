#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Combined signed and Kummer qualification wrapper
[pcc:unverified] T-2093; source-only prospective implementation. No compiler, tests, payload or lock generation. Independent review pending.
#image("flow.svg",width:100%)
== Concrete changes
The signed producer's native launcher, lease heartbeat, supervisor permit, InvocationID/cgroup ownership, cleanup and durable sealing machinery are reused with fresh identities. The worker keeps exact28 signed rows and final completion summary, identity/duplicate coverage, and seven Labels cases.
A separate private source tree stages the49 exact driver inputs and fixture JSONL records. Its lock must be the actual generated and published driver lock; no root-lock substitution or historical-crate replacement. Root, driver binary tests and adapter dependency-library tests all share1800seconds including staging and verification.
Eight exact test invocations share600seconds including cleanup and postchecks. Driver output acceptance requires original/disabled/observed complete-row equality, verified fixture scalar and eight fixed fault results. Forced fallback reconstruction and zero-vector lift have their own result. Four adapter negatives each require one passing exact test. Source and binary hashes remain checked.
== Preserved limits and pending bindings
Total2400seconds,2CPU16GiB zero swap512tasks, one attempt/no retries; original output/workspace bounds retained. The limits may cause failure and imply no completion prediction. Full1260cohort unchanged; qualification is not that cohort.
No admission.json exists. Draft release false; deadline, generated lock path/hash/size, source publication, ledger, lease, fresh cache and release receipts remain unset. Loader rejects absent binding before launcher mutation. Final coordinator-authenticated, independently reviewed published admission is still needed.
== Source provenance and next review
PLAN.md and input-bindings.json identify the three exact predecessor packets. Individual source patches show modifications; panel_acceptance.py and stage_driver.py supply new mechanics. KB unavailable; exact local primary-source fallback, no novelty claim. Cargo dependency package-target resolution, compilation and native branch coverage remain untested. Next is independent source review, then actual lock binding and authorized qualification separately. No old evidence changed.
