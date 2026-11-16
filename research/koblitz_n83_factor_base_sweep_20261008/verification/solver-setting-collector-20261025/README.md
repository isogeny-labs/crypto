# Collect the frozen eight-stratum N83 solver-setting run

[`collect.py`](collect.py) is pinned to GitHub Actions run `40193751812`,
attempt 1, at `a5af8d5023a644e9e3b347af4d8495540c8dea99`. It reads provider
run, job and artifact APIs and never dispatches or restarts a workflow. This
directory is outside the panel workflow's push path filter, so publishing the
collector does not launch a duplicate experiment. The collector is a small
wrapper around the [precommitted independent per-stratum auditor](../solver-setting-panel-20261025/AUDIT.md).

On a host with `gh` authenticated for read access, probe the exact run:

```sh
python3 research/koblitz_n83_factor_base_sweep_20261008/verification/solver-setting-collector-20261025/collect.py --probe
```

`WAITING_FOR_PROVIDER_RUN` or `WAITING_FOR_EIGHT_ARTIFACTS` means re-probe
the same run later. `FAILED_PROVIDER_JOB` preserves a failed producer rather
than converting it into a solver verdict. When the eight exact provider jobs
and artifacts are complete, collect into a **new** directory:

```sh
python3 research/koblitz_n83_factor_base_sweep_20261008/verification/solver-setting-collector-20261025/collect.py \
  --collect --out /tmp/n83-setting-run-40193751812-attempt-1
```

The collector downloads the two exact S3-backed producer ZIPs, the pinned
solver ZIP, all eight setting ZIPs, their provider metadata and run/job
snapshots. It checks the provider SHA-256 digest and byte count for each
download, then executes the precommitted independent auditor for every
stratum. It writes eight per-stratum audits and a `collection.json` with
actual solver statuses, audited arm count and summed solver process wall.
If a download or audit fails, it leaves partial raw evidence in the attempt
directory but writes no collection summary; use a new directory for a retry.
The raw ZIPs may be large, so keep them outside Git and retain their hashes
and provider IDs in any committed result.

`PASS_EIGHT_CENSORED_STRATA` means each of the 40 bounded solver arms
reported `INDETERMINATE`; it is not a factor-base ranking. Any SAT model
requires fresh S3 group-equation replay with a separate worker. UNSAT needs
an independent certificate. Watchdog, OOM and malformed outcomes retain
their own statuses. Complete cold index-calculus runtime and the selected
minimum remain unset pending natural relations, rank, linear algebra,
individual logarithm and matched rho.
