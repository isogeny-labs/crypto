# Independent replay of a hosted solver-setting stratum

The [auditor](audit_artifact.py) is committed before the hosted result. For
each completed stratum, keep the unmodified current provider ZIP, its SHA-256
and API metadata, the run's provider jobs JSON, the exact earlier t1_m0 or
t2_m0 source ZIP and metadata, and the pinned Linux solver preflight ZIP.
The current provider artifact name must be
`n83-solver-setting-<stratum>-<run_id>-<attempt>`. Do not re-zip extracted
files. The two source provider ZIP IDs and SHA-256 digests, plus the pinned
solver ZIP, are in [`protocol.json`](protocol.json).

```sh
python3 audit_artifact.py \
  --stratum hash_5_1_t1_m0 \
  --artifact-zip /path/to/current-provider.zip \
  --expected-zip-sha256 CURRENT_PROVIDER_ZIP_SHA256 \
  --metadata /path/to/current-provider-metadata.json \
  --jobs /path/to/current-provider-jobs.json \
  --source-zip /path/to/prior-t1_m0-provider.zip \
  --source-metadata /path/to/prior-t1_m0-provider-metadata.json \
  --solver-zip /path/to/pinned-preflight-provider.zip
```

The auditor binds the terminal provider job and each successful setup/search
step to the current run/head, checks the earlier S3-backed producer audit and
exact source ZIP, replays formula and solver hashes, validates the positive
wide-XOR smoke, and independently parses each retained solver stdout,
stderr, status and conflict count. It checks the frozen configuration order
and flags and sums process wall from raw receipts. A SAT result remains
`PENDING_INDEPENDENT_GROUP_REPLAY` until a separately built worker,
fresh S3 archives and the original curve-group equation verify its model.
An UNSAT result remains pending a separate certificate. All five settings
run after SAT or UNSAT; watchdog, OOM and malformed processes stop a stratum
and retain their own statuses. Five `INDETERMINATE` arms
yield only `PASS_FIVE_CENSORED_SETTINGS_REPLAY`; no total-runtime winner is
selected from a censored setting screen.
