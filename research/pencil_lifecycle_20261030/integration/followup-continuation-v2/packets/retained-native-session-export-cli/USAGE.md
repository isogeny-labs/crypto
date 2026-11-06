# Retained native session exporter CLI

This program exports an already completed native session. It does not launch a worker or observe a process. The caller must separately obtain and authenticate the expected owner/controller pins. Software/request identity does not authenticate those pins.

Invocation (replace each uppercase value with its actual absolute path or lowercase SHA256):

```text
retained-export-cli --request ABSOLUTE_REQUEST_JSON --request-sha256 REQUEST_SHA256 --binary-sha256 CLI_BINARY_SHA256 --out FRESH_ABSOLUTE_DIRECTORY
```

The strict request has these keys:

```json
{
  "schema": "ecbench.retained-export-cli-request/v1",
  "files": {
    "inventory_request": "ABSOLUTE_STATIC_REQUEST_JSON",
    "inventory_sha256": "STATIC_REQUEST_SHA256",
    "route": "ENROLLED_ROUTE_NAME",
    "host": "ABSOLUTE_HOST_JSON",
    "session": "ABSOLUTE_COMPLETE_SESSION_JSON",
    "preparation": "ABSOLUTE_PREPARATION_JSON",
    "launch": "ABSOLUTE_LAUNCH_JSON",
    "contexts": ["ABSOLUTE_CONTEXT_JSON"],
    "journal": "ABSOLUTE_JOURNAL_JSON",
    "owner": "ABSOLUTE_FRESH_OWNER_JSON",
    "config": "ABSOLUTE_FRESH_CONFIG_JSON",
    "custody": "ABSOLUTE_CONTROLLER_CUSTODY_JSON"
  },
  "expected": {
    "fresh": {
      "config_sha256": "EXPECTED_CONFIG_SHA256",
      "request_sha256": "EXPECTED_OWNER_REQUEST_SHA256",
      "binary_sha256": "EXPECTED_WORKER_BINARY_SHA256",
      "image_sha256": "EXPECTED_WORKER_IMAGE_SHA256",
      "pod_uid": "EXPECTED_POD_UID",
      "job_uid": "EXPECTED_JOB_UID"
    },
    "controller_custody_sha256": "EXPECTED_CONTROLLER_CUSTODY_SHA256"
  }
}
```

Placeholders above are documentation, not valid evidence. The CLI binary hash is distinct from the worker binary hash in expected.fresh. CLI request hash is distinct from static inventory request and owner request hashes.

Success exits zero and emits a JSON invocation receipt on stdout. The fresh output directory contains the unchanged exporter's copied native files, actual-request.json, export-report.json, plus exact cli-request.json and cli-invocation.json. The invocation binds the executable, request and export-report hashes. Owner authentication remains false; worker allocation, total active worker time and exact kernel exit remain null where the observer did not establish them.

Failure exits nonzero and emits JSON on stderr. A partially written output directory may remain and must be retained as failed evidence. Reusing it is rejected. The program performs no retry, overwrite or cleanup of retained evidence. Caller orchestration must preserve stderr and exit status even when an invocation receipt was not produced.
