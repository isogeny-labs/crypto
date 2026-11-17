# r29 factor-base attempt terminal capture

Job `n83-fb-variants-r29-20261031-r1` (UID `95f12a4d-a2fa-44d6-ae1d-1f26b6ae67ee`) failed with `BackoffLimitExceeded` at 2026-10-31T09:34:22Z. The bounded logs endpoint returned an empty log and the status endpoint retained no Pods. Its S3 prefix contains 81 objects, including 75 partial content-addressed snapshots.

The snapshot bodies are not available through the current local AWS identity, so the failure cause and any completed shard subset are unresolved. Preserve this as a partial failed attempt. Do not count it as r29 completion or retry with the same attempt prefix. The exact status, logs response, and full results listing are in this directory.
