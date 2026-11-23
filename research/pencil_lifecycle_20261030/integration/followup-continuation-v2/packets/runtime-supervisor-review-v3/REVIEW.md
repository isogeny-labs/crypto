# Narrow v3 supervisor source review

The additive v3 signal-handling change addresses the remaining R2 launch-ownership gap at source scope. The handler sets pending_signal without raising inside Popen; unreaped_client is set before launch; the assigned handle is retained before a deferred signal is raised; pending signals are checked in the running loop and before new noncleanup commands. Constructor failures with uncertain ownership leave the cleanup guard closed. No signal mask is inherited by the child. The prior v2 findings and source review remain retained.

Verdict: no blocking discrepancy found in this narrow delta. All v3 manifest entries match. This is not an execution admission or runtime correctness result. No Docker, container, cloud or reviewed executable was launched. Independent fixed software controls for these failure paths remain useful before any operational claim.

The effective-before-start pids gate is still unresolved and qualification_complete=false remains appropriate. The /opt archive is still a prospective scope extension. Final image/client/build bindings, route, resource envelope and prospective admission remain required. No lifecycle completion or hard wall-time guarantee follows from this source review.

Producer manifest a24a692f67a9fe758ae21a756c6869e57cc7fe501973112c6ee200d45dba3b50; supervise.py61497a2ba145a7f0c03ddb9030c382f4edb43a78cfca0dc150fd8364fc370f59. Review uses the continued frozen v1/v2 source-only plan and exact correction.patch. Producer files unchanged.
