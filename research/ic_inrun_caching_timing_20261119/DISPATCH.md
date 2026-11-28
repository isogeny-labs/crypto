# Dispatch plan (for the lead; nothing here has been run)

Protocol: [`PROTOCOL.md`](PROTOCOL.md). Specs (SHA-256 at drafting):

| spec | sha256 |
|:--|:--|
| `specs/p24a-batch-sparse.json` | `6def46f9df26d7688bd0acae132a65ac478ce61b998bf6ab048a290e78ecae41` |
| `specs/k0-13-descent-template.json` | `f88f59566c33d4db8acf15591a9f6b9b54baaa84b7fbced5576a908e6d27f35a` |

Re-hash before freezing. If you edit a spec, update this table and the
protocol together.

## 0. Pre-launch commands (in order; stop at the first failure)

```sh
cd /home/user/workspace/repos/crypto
D=research/ic_inrun_caching_timing_20261119
BASE=ad2a014c0022cf000845014499a9649b6c2cef2d

# 0a. Local spec sanity, no build: both specs parse as JSON. Do this before any commit.
for f in $D/specs/*.json; do python3 -m json.tool "$f" >/dev/null; done; sha256sum $D/specs/*.json

# 0b. NEW revision = the PR head after the freeze commit and the OLD merge below (code frozen
#     at 0f1a8cd8d, which includes the lanczos kernel-probe fix).
# 0c. OLD revision = ad2a014 + this directory only, built locally from git objects (no checkout,
#     no GitHub write API), then merged into the PR branch so it is reachable and fetchable:
export GIT_INDEX_FILE=$(mktemp); git read-tree $BASE
for f in $(git ls-files $D); do git update-index --add --cacheinfo 100644,$(git rev-parse HEAD:$f),$f; done
OLD_SHA=$(git commit-tree $(git write-tree) -p $BASE -m "Timing baseline: ad2a014 plus the study directory only")
unset GIT_INDEX_FILE
git diff --name-only $BASE $OLD_SHA          # must list only $D/...
git merge --no-ff --no-edit $OLD_SHA         # no content change: the PR already has ad2a014 and $D
git push; NEW_SHA=$(git rev-parse HEAD)
for s in p24a-batch-sparse k0-13-descent-template; do                 # spec bytes identical on both
  for rev in $OLD_SHA $NEW_SHA; do git show $rev:$D/specs/$s.json | sha256sum; done; done
```

Steps 0b and 0c push only to the PR branch (the owner authorised the run on 2026-11-19).

```sh
# 0d. Publication receipt (COST_CONTROLS.md "Before spending"); NEW is the local HEAD.
mkdir -p $D/receipts
python3 taskq/deploy/eks-karpenter/check-experiment-git --repository isogeny-labs/crypto \
  --path $D/PROTOCOL.md --path $D/DISPATCH.md \
  --path $D/specs/p24a-batch-sparse.json --path $D/specs/k0-13-descent-template.json \
  > $D/receipts/publication-new.json
#     OLD has no local checkout: the compare listing + spec hashes of 0c are its receipt.
gh api $R/compare/$BASE...$OLD_SHA > $D/receipts/publication-old-compare.json

# 0e. Optional preregistration (a T0 engineering study needs none under SUCCESS_CRITERIA §2;
#     recommended because the sabotage rule asks for claims to be records):
python3 tools/pcc/pcc.py claim new --kind statement --claim-type data \
  --scope "$D/PROTOCOL.md@$NEW_SHA" \
  --statement "Preregistration: PR #4756 in-run caching timing, protocol v1 frozen at $NEW_SHA (new) and $OLD_SHA (old) before any dispatch; decisions D5s, D5m, D5n, D2b, D4 and S as written there."
#     experiment.protocol/v1 (experiment-protocol skill): needs ../crypto-autoresearcher; then
#     python3 ../crypto-autoresearcher/tools/experiment_protocol.py freeze $D/protocol.json --commit auto --filed-before-affected-data

# 0f. Resume checkpoint (AGENTS.md "Resume checkpoints"): copy the draft, finish it, push, gate.
scripts/checkpoint.sh init
cp $D/RESUME.draft.md .resume/RESUME.md      # then fill in NEW_SHA / OLD_SHA / record paths
scripts/checkpoint.sh push && scripts/checkpoint.sh gate

# 0g. Benchmark records: one per job (records are written once; a rerun gets its own).
for j in O-P24-1 N-P24-1 N-K13-1 O-K13-1 N-P24-2 O-P24-2 O-K13-2 N-K13-2; do
  case $j in *P24*) prob=ic:inrun-caching:P24a; uid=EC1P24Cfph095cb65c0da0 ;;
             *K13*) prob=ic:inrun-caching:K0-13; uid=EC1N13Ce0h1dfa32cdf623 ;; esac
  python3 tools/benchrec.py init --slug "ic-inrun-caching-$(tr A-Z a-z <<<$j)" --problem $prob \
    --curve $uid --entry benchmarks/20261119-ic-inrun-caching
done   # note each printed record path as REC_<job>

# 0h. Budget and capacity (read-only), immediately before EACH dispatch:
aws sts get-caller-identity          # account 104186811093, if the bridge identity is used
scripts/isolab_pool_check.sh --cpu 4 --memory 4Gi --pool cpu-l2-spot --save $D/receipts/<job>/pool.json
#     Record the daily (USD 200) and monthly (USD 1,000) budget headroom; do not dispatch if unknown.
```

## 1. Dispatch inputs (8 jobs, sequential: dispatch the next only after the previous reaches a terminal state)

All jobs use workflow `isolab-experiment.yml`, `--ref main`, and the inputs
`instance_type=c8a.2xlarge`, `seconds=4800`, `publish_only=false`,
`size_sweep=false`, `norm_branch_census=false` and `resumable=false`
(defaults). `checkpoint_seconds` and `max_restores` are unused.

| # | job | source_revision | spec_path |
|--:|:--|:--|:--|
| 1 | O-P24-1 | `<OLD_SHA>` | `research/ic_inrun_caching_timing_20261119/specs/p24a-batch-sparse.json` |
| 2 | N-P24-1 | `<NEW_SHA>` | `research/ic_inrun_caching_timing_20261119/specs/p24a-batch-sparse.json` |
| 3 | N-K13-1 | `<NEW_SHA>` | `research/ic_inrun_caching_timing_20261119/specs/k0-13-descent-template.json` |
| 4 | O-K13-1 | `<OLD_SHA>` | `research/ic_inrun_caching_timing_20261119/specs/k0-13-descent-template.json` |
| 5 | N-P24-2 | `<NEW_SHA>` | `…/specs/p24a-batch-sparse.json` |
| 6 | O-P24-2 | `<OLD_SHA>` | `…/specs/p24a-batch-sparse.json` |
| 7 | O-K13-2 | `<OLD_SHA>` | `…/specs/k0-13-descent-template.json` |
| 8 | N-K13-2 | `<NEW_SHA>` | `…/specs/k0-13-descent-template.json` |

```sh
python3 tools/benchrec.py run "$REC_O_P24_1" -- \
  gh workflow run isolab-experiment.yml --repo isogeny-labs/crypto --ref main \
    -f source_revision="$OLD_SHA" \
    -f spec_path=research/ic_inrun_caching_timing_20261119/specs/p24a-batch-sparse.json \
    -f instance_type=c8a.2xlarge -f seconds=4800
# Then record the run id: gh run list --workflow isolab-experiment.yml --repo isogeny-labs/crypto -L 3 --json databaseId,createdAt,headSha
# The Job name is gh-<RUN_ID>-<ATTEMPT>. Keep it in $D/receipts/<job>/dispatch.json.
```

Before dispatch 1, confirm that `isolab-experiment.yml` on `main` is still
the version read on 2026-11-19 (commit-image support, `spec_path` read at
`source_revision`).

**Afterwards (per job):** fetch the private S3 receipt and session. Verify
the Job/Pod UIDs, `build.source_revision`, `ecbench_sha256`, the L2 gate,
replay, A/A admission and the payload hashes. Then run
`tools/benchrec.py add-raw` on the session archive, using `--s3-uri` for the
isolab prefix, and `tools/benchrec.py close`, with status `measured`,
`invalid` or `censored`. Push the checkpoint again.

## 2. Cost estimate and ceiling

These are estimates, `[pcc:unverified]`. COST_CONTROLS.md gives budgets,
not instance prices. The rates below are assumed list prices, to be checked
against the AWS price list before launch.

| item | per job | basis | 8 jobs |
|:--|--:|:--|--:|
| c8a.2xlarge worker, worst case | ≤ USD 0.67 | 4,800 s deadline + 300 s provisioning, at an assumed on-demand USD 0.47/h (Spot is lower) | ≤ USD 5.4 |
| c8a.2xlarge worker, expected | ≈ USD 0.12–0.25 | sessions of about 15–30 min (240 / 120 children, mostly under 3 s) at Spot | ≈ USD 1–2 |
| CodeBuild launcher (general1.small, assumed USD 0.005/min), worst case | ≤ USD 0.70 | 140 min cap | ≤ USD 5.6 |
| ECR image, S3 results and transfer | < USD 0.05 | a few hundred MB | < USD 0.4 |
| GitHub build job | 0 | hosted runner (ARC pools are at 0) | 0 |
| **total** | | | **worst ≈ USD 11.4; expected ≈ USD 3** |

**Proposed ceiling: USD 15 for the whole study.** It covers the 8 jobs, at
most 2 re-dispatches (each a new experiment with its own record, about USD
1.4 worst case each), idle time, failures and interruptions. When the charged
plus reserved spend reaches USD 15, or the remainder is unknown, stop.
Concurrency is 1 job (4 vCPU / 4 GiB on `cpu-l2-spot`). The original absolute
deadline is the dispatch date + 3 days. Retry allowance: 2 jobs in total.

## 3. Expected job count and wall

8 jobs and 8 workflow runs. Each run takes about 20–40 min to build, plus at
most 80 min for the experiment. Run sequentially, that is about 8–14 h
end to end. Each rebuild of an identical commit produces a new image. The
`ecbench_sha256` of the two builds of one revision should match. If they do
not, report it as an unreproducible build: it is not a failure of the
measurement.
