# Ordinary binary n23 root/BSGS control campaign

`protocol.json` and `protocol-v2.json` are the two frozen experiment records. The v2 amendment applies only to indices 11..15 after the original XFS free-space gate stopped generation before index 11. The unchanged native 16-cell population and its plan bindings are in `plan.json` and `plan-v2.json`. The original protocol, plan and output for indices 0..10 are retained; the second XFS contains indices 11..15.

The administrative controllers `campaign.py` and `campaign_v2.py` support read-only `check` and bounded `run --max-new-cells N`. Both require pinned inputs, XFS, one-attempt markers, native verify/report/export, complete restricted archive, S3 PUT, and fresh GET/hash before a Git-ready per-cell checkpoint is written. V2 also refuses an already-started v1 index 11. The campaign is terminal, so do not rerun generation. `audit_campaign.py` and `make_experiment_export.py` project the retained evidence without solver execution. `RESULT-01.md`, `AUDIT-01.json`, `TERMINAL-01.json`, `INDEPENDENT-S3-SAMPLE-01.json`, and `experiment-export.json` contain the completed observation and checks.

Exact official export check:

```sh
/home/ec2-user/ml-dataset-work-20261028/protocol-checker-venv/bin/python3 \
  /home/ec2-user/ml-dataset-work-20261028/protocol-tooling-f755085ff951/tools/experiment_protocol.py \
  check-export research/ml_dataset_binary_root_bsgs_20261031/experiment-export.json \
  --protocol research/ml_dataset_binary_root_bsgs_20261031/protocol-v2.json --verify-files
```
