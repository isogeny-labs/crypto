# Independent materializer source review

Disposition: two source revisions are needed before execution admission. The preread plan was frozen before producer inspection; the implementation was read only after root verified publication of357f73fe0eb945c9406fab542def431de51ee64c. Reviewer did not author the materializer. Source SHA256:d6151c45c1c00691a3ad9f446a8d68051ac6cb511b5e0c6011b241ffacb901ef. No materializer, compiler or scientific payload was executed. [pcc:unverified]

## M1 — Bind the bytes actually copied

At materialize.py114 the approved candidate is hashed, then read again for copying. `row(pencil,IMM)` records the second read's digest, and line163 checks against that newly generated digest. A source trace is: approved A passes the first hash; the file changes to B; B is copied and entered in IMM; final comparison accepts B. This is an unexecuted source counterexample, not an observed race. The initial bound JSON inputs are likewise consumed after their first hash and are not rechecked at completion. Read/hash the same consumed bytes, assert the destination equals the pinned candidate digest, and retain final checks of original bindings. [pcc:unverified]

## M2 — Bound owned-child cleanup

At materialize.py49 and171, ACTIVE.wait() has no timeout. Lines48/170 only signal the process group while its leader is running; an exited leader can leave descendants unexamined. A cleanup stall therefore can outlast the worker's own work guard, while the worker cannot substantiate descendant cleanup. Use remaining-time-bounded reaping and explicit retained cleanup status; the outer supervisor must still confirm service/cgroup termination. No child process was launched during this review. [pcc:unverified]

## Components consistent on source inspection

The full source revision is pinned, selected Git entries must be ordinary blobs, and source totals are bounded. Vendor archives must match lockfile checksums; traversal, links and duplicate members are rejected, and new per-file checksum metadata is produced. New workspace/output creation is exclusive, checkpoints require output ownership, and fresh destination hashes are recorded instead of relabeling historical manifests. Initial compiled cache contents are recorded separately from immutable source inputs; that does not independently establish compiled-cache provenance. Live CPU/RAM/swap/task checks and disk/source/vendor/cache caps are present. [pcc:unverified]

The primary Cargo source-replacement documentation requires equivalent source contents and explains directory-source checksum metadata: https://doc.rust-lang.org/cargo/reference/source-replacement.html. This supports checking the archive and unpacked-file bindings, while leaving concrete copied-byte validation to this implementation. [pcc:unverified]

## Admission still to bind

The packet supplies a proposal, not an executable preparation.json. Root must publish/review the concrete admission, resolved source/output paths, current lease and cumulative debit, plus a bounded outer supervisor and exact tool identities. The reviewed tool manifest binds Python and Rust tools but does not bind git/findmnt; record those tools or an explicit trusted-host premise. The owner's dollar waiver leaves46,800 worker-seconds and the2026-11-06T04:00Z deadline unchanged; it does not erase prior charges. [pcc:unverified]

V1 remains immutable. Record any correction in a successor with new hashes and obtain delta review. No statement here establishes successful materialization, runtime qualification or a new target result. [pcc:unverified]
