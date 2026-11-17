# Six-hour continuation: conditional operational guard review

The new authorization starts at C0=2026-10-30T05:17:54Z (1793337474), expires at D=11:17:54Z (1793359074), and permits21600 cumulative active-worker seconds. The earlier allowance and failed run remain separate historical evidence.

With reserve R=900 seconds, G=08:10:24Z (1793347824):
2G-C0+R = 1793359074 = D.
The conservative per-epoch span G-C0 is10350 seconds; twice that plus900 equals21600.

The approved client SHAac7d57132617775679770dc55b6f8b3acc83f4454e31b025a1f35e9b915abc21 and cloud/supervisor/io source hashes still match the prior reviewed package locally. This review did not query live deployment or bind a new request.

The actual cloud source wraps AWS/Kubernetes calls in timeout180s with kill-after5s (cloud.rs20-45). Fresh creation can involve four sequential Kubernetes calls after the remaining-time calculation (cloud.rs670-704), totaling740 seconds;60 seconds termination grace gives800, leaving100 seconds of the900 reserve. The existing120-second fresh subtraction is not credited as extra slack.

The supervisor checks absoluteG between operations (supervisor.rs628-635). Synchronous verification and CRIU rely additionally on Pod active-deadline enforcement. Computation is completed/reaped before native replay; verified replay precedes fresh measurement. Restore requires the previous instance terminated (cloud.rs802-826). These facts support serial scientific payloads within a request, but cross-request seriality must still be observed by the owner.

Acceptance is conditional: ordinary local/control and relevant scheduling delays must fit the remaining100 seconds, and Kubernetes deadlines/grace must operate normally. Relative Pod/Job deadlines do not prove termination byD under arbitrary scheduling stalls, host suspension or broken enforcement. This is not a hard-real-time theorem.

If all scientific intervals lie inside[C0,D] and at most one scientific payload is active, their summed active-worker duration cannot exceed21600 seconds. This includes computation, restores, native replay and fresh measurement. Idle allocations and controller operations remain separately reported and are not asserted to equal active-worker duration.

Every subsequent request must retain originalC0,D and the cumulative21600 budget; it must not reset the start clock. Bind G no later than08:10:24, or independently review a stricter remaining-budget guard. Reconcile actual prior attempt/recovery/replay/fresh intervals and termination before another request; missing observations are not zero spend. Preserve all failures and no-success rows.

Scientific protocol admission, resource caps, outcome/replay checks and timing qualification remain independent. No cloud actions, trial execution, request submission or old-record edits were performed. Parent-reported prior-worker termination is not an independently observed fact in this review.
