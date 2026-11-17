# Independent materializer V2 delta review

The two V1 source findings have scoped corrections in published64afb6b895. The V2 preread plan was written before implementation inspection and retains the original V1 findings in ed2db1be72. Reviewer did not author the producer; the proposed fixes were disclosed. [pcc:unverified]

M1: materialize_safety.py4-9 reads the candidate once, hashes those bytes, writes the same bytes and checks the destination against the approved digest. materialize.py116-117 additionally pins the manifest row, and167-168 recheck original bound inputs. The original separate-read counterexample no longer changes the copied bytes after the approved read. Endpoint checks still retain the existing shared-filesystem/transient-change assumptions. [pcc:unverified]

M2: materialize_safety.py11-25 attempts owned session-group cleanup regardless of leader completion, waits with a two-second timeout, probes group absence and records failures. materialize.py49-51 rejects incomplete cleanup, and174-176 preserve failure during final cleanup. These checks rely on the process-group identity owned by the launcher; they do not independently qualify kernel scheduling or replace outer unit/cgroup termination evidence. [pcc:unverified]

Six supplied mock-only controls passed under a10-second external timeout with2-second kill grace: approved single-read candidate, changed candidate rejection, corrupt destination rejection, exited-leader group cleanup, wait-timeout retention and visible-group rejection. Command exit was0; captured unittest output reports6 tests and OK. Tests use in-memory byte objects and fake process/kill functions, import only the small safety helper and do not execute the materializer body. All producer manifest hashes matched after the controls. [pcc:unverified]

Source diff inspection found no changes to pinned source revision, include expansion, vendor checksum/extraction rules, cache separation, work deadline, CPU/RAM/swap/task limits or retry policy. The safety helper is added to the required bound input set. No new blocking finding was identified in this scoped delta. [pcc:unverified]

Concrete preparation.json, tool/path bindings, current cumulative debit and a bounded reviewed outer supervisor remain execution gates. The owner's dollar waiver does not alter the46,800-worker-second ceiling,2026-11-06T04:00Z deadline or previous charges. No actual materializer, compiler, library test or target cohort ran in this review. [pcc:unverified]

Primary evidence: exact published V1 review and V2 materializer/helper/test sources. The prior Cargo source-replacement reference remains applicable; this operational delta introduces no new mathematical mechanism. KB was not loaded and its wrappers were absent in the sparse checkout, as recorded in the preread plan. [pcc:unverified]
