# Runtime context staging

Run `python3 verify.py` from this directory for read-only verification of the retained local context. It does not recreate missing bytes. `staging-receipt.json` identifies the local directory and complete inventory; `readback.json` binds the original check.

To reproduce staging from the exact retained local inputs, from repository root:

```sh
python3 research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/runtime-context-staging/stage.py research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/runtime-binding-reconciliation/evidence/proposed-context-member-ledger.json research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/runtime-mountpoint-proposal/Dockerfile.runtime.proposed 5d86b44b9f087a430969c40882db60127d4fcfdb5934bb87cc4d2398dbcec21b /var/tmp/new-context-receipt.json
```

Use an unused receipt destination. The script writes a fresh private context, not an image. Missing or changed inputs fail. Existing runtime payload custody is documented in runtime-binding-reconciliation; source paths are host-local. No new archive upload or cloud publication occurred. This directory is not a build admission.
