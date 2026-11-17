# Fixed BenchLock controls: independent retained-output review

The admitted compiler and native helper both exited zero. Independent raw-output recomputation found each of C1–C6 and the terminal marker exactly once, in prescribed order C1,C2,C3,C4,C6,C5. Nine probes match their source assertions: three contention failures with errno 11 and six successful acquisitions. Four distinct child-ready PIDs and four zero-status child reaps are present; C2 confirms CLOEXEC. No native rerun occurred.

The admission-bound compiler, libc rlib, helper source, extracted primitive and producer manifest match current bytes. The prior independent source-review manifest matches its admission binding. The executable matches binary-binding.json, which binds the exact admission digest. Compiler/run commands retain their 120-second/60-second outer caps and 5-second kill grace. Native run stderr is empty. All producer manifest entries were also rechecked.

C1 checks ordinary ownership/drop; C2 close-only inheritance; C3 parent release while child retains a descriptor; C4 child nonowner release-method call plus close while parent remains owner; C6 early-return Drop; C5 cross-thread owner Drop. These observations agree with the previously reviewed source assertions. Output markers alone are not substituted for their probe records.

Scope remains an extracted production primitive with an unexercised signal stub. wait=true, injected non-WouldBlock errors, and complete Rust child-field destruction are not tested. This packet does not establish which interleaving or error caused the historical GPU failures. Root owns subsequent integrated tests. Reviewer authored an earlier close-only mechanism diagnostic, not this patch/helper or execution.

Exact input/output hashes and counts are in result.json; recompute.py records this read-only analysis. Execution source: /var/tmp/pencil-benchlock-controls-execution-20261031-v2. Producer source: /var/tmp/pencil-benchlock-controls-20261031-v2. Prospective review: /var/tmp/pencil-benchlock-independent-review-20261031-v2.
