# Offline launch reconciliation and native binding roundtrip

`fixture.py` uses invented Job, Pod, Node, policy, and reservation inputs. It
reserves a launch in a temporary native budget ledger, derives a digest-bound
candidate with `scripts/ml_dataset_launch_reconcile.py`, rejects a tampered
reservation digest without changing the ledger, and binds the untampered
candidate through `crypto dataset budget bind-launch`. It then checks that the
ledger has one bound launch and no pending launch. `report.json` records the
observed result and the exact native binary and reconciler hashes.

Run with a native binary that supports `dataset budget reserve-launch` and
`bind-launch`:

```sh
python3 research/ml_dataset_launch_bind_roundtrip_20261031/fixture.py \
  --crypto /path/to/crypto \
  --out /tmp/launch-bind-report.json
```

The test creates a fresh temporary ledger and exits without any Kubernetes or
AWS request. Its grant is local and synthetic; it is not a worker grant for a
real cluster. Temporary paths are included in the native reservation, so
their digest changes on each run. Check the relationships and rejection
assertions rather than comparing the report's digest to a rerun byte for byte.
