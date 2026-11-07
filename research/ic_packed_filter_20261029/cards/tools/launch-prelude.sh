# Sourced by every step of the XP-20261029-packed-filter launch card. Reads b (24, 28 or 32) and unit
# (PAIR, r2, rho-r2, nr or nr-r2) from the environment and refuses any other value.
set -euo pipefail
export PATH=/tmp/claude-0/awsvenv/bin:$PATH AWS_DEFAULT_REGION=us-east-2 AWS_REGION=us-east-2
export ISOLAB_BUCKET=isolab-results-104186811093-us-east-2 ISOLAB_CODEBUILD_PROJECT=isolab-experiment-launcher
case "$b" in
  24|28|32) ;;
  *) echo "STOP: b is not 24, 28 or 32: b=$b"; exit 1 ;;
esac
case "$unit" in
  PAIR|r2|rho-r2|nr|nr-r2) ;;
  *) echo "STOP: unit is not PAIR, r2, rho-r2, nr or nr-r2: unit=$unit"; exit 1 ;;
esac
SC=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad
PFD=$SC/pf
TOOLS=$SC/pf-card-tools
WT=/dev/shm/src-pf
SRC=/dev/shm/src-pf
REMOTE=$SRC/taskq/deploy/eks-karpenter/remote
PD=$WT/backlog/XP-20261029-packed-filter
STUDY=$WT/research/ic_packed_filter_20261029
BRANCH=claude/ic-packed-filter-20261029
FREEZE=f870408c1762a52dd133e8f1e0bca181cc2a76f5
IMAGE=104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:32144a29e437376b9bf8b902aef4ba6b9cc723a15f01d296fb7b09eb131b817b
REV=3271a25d02dcef6da148293fb8f9e2149ce59858
FBSHA=21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503
RUNURL=https://github.com/isogeny-labs/crypto/blob/$FREEZE/backlog/XP-20261029-packed-filter/PROTOCOL.md
S3RUNS=s3://$ISOLAB_BUCKET/runs
PREDTAG=""
case "$unit" in
  PAIR) TAGS="P$b P$b-rho"; NEED=2 ;;
  r2) TAGS="P$b-r2"; PREDTAG="P$b"; NEED=1 ;;
  rho-r2) TAGS="P$b-rho-r2"; PREDTAG="P$b-rho"; NEED=1 ;;
  nr) TAGS="P$b-nr"; NEED=1 ;;
  nr-r2) TAGS="P$b-nr-r2"; PREDTAG="P$b-nr"; NEED=1 ;;
esac
job_of() {
  case "$1" in
    "P$b") echo "ic-dgbf-p$b-20261029" ;;
    "P$b-rho") echo "ic-dgbf-rho-p$b-20261029" ;;
    "P$b-r2") echo "ic-dgbf-p$b-20261029-r2" ;;
    "P$b-rho-r2") echo "ic-dgbf-rho-p$b-20261029-r2" ;;
    "P$b-nr") echo "ic-dgbf-p$b-20261029-nr" ;;
    "P$b-nr-r2") echo "ic-dgbf-p$b-20261029-nr-r2" ;;
    *) echo "STOP: bad tag $1" >&2; return 1 ;;
  esac
}
spec_of() {
  case "$1" in
    "P$b"|"P$b-r2"|"P$b-nr"|"P$b-nr-r2") echo "$PD/spec-P$b.json" ;;
    "P$b-rho"|"P$b-rho-r2") echo "$PD/spec-P$b-rho.json" ;;
    *) echo "STOP: bad tag $1" >&2; return 1 ;;
  esac
}
echo "PRELUDE_OK b=$b unit=$unit TAGS=$TAGS PREDTAG=${PREDTAG:-none} NEED_SLOTS=$NEED"
