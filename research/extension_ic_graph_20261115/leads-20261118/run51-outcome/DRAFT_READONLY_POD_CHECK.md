# DRAFT: read-only Run51 pod check (not executed; no compute)

Status: draft, for a session or operator whose Runpod credential works. This is
not an experiment and allocates nothing. It only reads.

1. `GET https://rest.runpod.io/v1/pods/7zki4wnz56fzad`
   - 404: pod deleted. Record `{pod_id, get_http: 404, checked_at}` as
     `run51-absence.json` in a new lane dir; Run51 originals then exist only on
     the dispatching host, if anywhere.
   - 200: record `desiredStatus`, `costPerHr`, `createdAt`, `lastStartedAt`
     verbatim. Stop and hand to the owner: deciding to capture originals via
     the protocol's authorized capture route and then delete is an owner
     decision (this lane may not stop or delete).
2. `GET https://rest.runpod.io/v1/pods` and list every pod whose name or env
   marks it as ic-owned (`ic-runpod-runs`, `unit-deferred`, `deferred-pair`),
   with `desiredStatus` and `costPerHr`.
3. If the original archive is later recovered: verify `bundle-tar.sha256`,
   `pre-call.sha256`, `post-call.sha256`, then run the frozen analyzer
   (`examples/jv_sieve_unit_deferred_check.rs`, build hash in
   `validation/unit51-build.sha256`) unchanged, exactly as line 106 of
   `run-unit-deferred-task.sh`; do not rerun any worker.

[pcc:unverified]
