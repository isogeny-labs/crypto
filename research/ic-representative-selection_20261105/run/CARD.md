# Frozen launch route for the experiment-director card

This is what the director compiles into `tools/task_card.py` cards (AGENTS.md
"Model tiers"; `isolab-eks` references/submission.md "Mechanical task cards").
The main session relays each card verbatim to executor-mechanical; the
executor runs it unchanged and computes nothing.

**Preconditions (main session, before any card):** PROTOCOL.md frozen and
INPUTS.md (A1) committed and pushed; `.resume/RESUME.md` pushed
(`scripts/checkpoint.sh gate` passes); `aws sts get-caller-identity` shows
account 104186811093 for the executor; `check-experiment-git` receipt for the
pushed commit; budget remaining ≥ the batch's share of USD 150.

**Build (once):**

```sh
gh workflow run isolab-experiment.yml --repo aburan28/crypto --ref main \
  -f source_revision=<FROZEN_COMMIT_SHA> -f publish_only=true
# then write its publish receipt as research/ic-representative-selection_20261105/inputs/image.json:
# {"image_uri": "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:<64 hex>",
#  "source_revision": "<FROZEN_COMMIT_SHA>", "ecbench_sha256": "<64 hex>"}
```

**Per job, one at a time, in `inputs/jobs.tsv` order (tier 24 first), then
`inputs/jobs-hold.tsv` after each class's `run/05-hold.sh`:**

```sh
python3 tools/benchrec.py init --slug ic-repsel-<job> --problem ic:representative-selection-20261105 \
  --entry research/ic-representative-selection_20261105
python3 tools/benchrec.py run benchmarks/records/<record>.json -- \
  research/ic-representative-selection_20261105/run/03-launch.sh <job>
research/ic-representative-selection_20261105/run/04-collect.sh <job>
python3 tools/benchrec.py add-raw benchmarks/records/<record>.json \
  research/ic-representative-selection_20261105/sessions/<job>/session/records.jsonl \
  --s3-uri <results_uri from receipts/<job>/result.json>session/records.jsonl
python3 tools/benchrec.py close benchmarks/records/<record>.json --status measured|invalid|censored --summary "<job>: admission and hashes only"
```

Card binds (SHA-256): `run/03-launch.sh`, `run/04-collect.sh`, `run/common.sh`,
the job's spec under `inputs/specs/`, `inputs/image.json`,
`taskq/deploy/eks-karpenter/remote/validate-request` and `launch-remote`.
Environment: `AWS_REGION=us-east-2`,
`ISOLAB_BUCKET=isolab-results-104186811093-us-east-2`,
`ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher` (set by 03-launch.sh);
AWS credentials inherited, never written. `must_not_exist`:
`receipts/<job>`. Client timeout: 3,600 s job + 3,600 s launcher wait + 600 s
= 7,800 s. Outputs: `receipts/<job>/result.json`, `sessions/<job>/`.

The executor never re-submits, renames, changes the instance type or switches
backends; a non-zero exit returns to the main session, which applies the
retry rule of PROTOCOL.md §6 (one `-r2` per job) after reconciling the
original submission.
