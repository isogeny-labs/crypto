# Sourced by every step of the XP-20261029-packed-filter analysis card. Reads b (24, 28 or 32), wall (first, r2, nr or nr-r2)
# and counted (first, r2 or existing) from the environment and refuses any other value.
set -euo pipefail
export PATH=/tmp/claude-0/awsvenv/bin:$PATH AWS_DEFAULT_REGION=us-east-2 AWS_REGION=us-east-2
export ISOLAB_BUCKET=isolab-results-104186811093-us-east-2
case "$b" in
  24|28|32) ;;
  *) echo "STOP: b is not 24, 28 or 32: b=$b"; exit 1 ;;
esac
case "$wall" in
  first|r2|nr|nr-r2) ;;
  *) echo "STOP: wall is not first, r2, nr or nr-r2: wall=$wall"; exit 1 ;;
esac
case "$counted" in
  first|r2|existing) ;;
  *) echo "STOP: counted is not first, r2 or existing: counted=$counted"; exit 1 ;;
esac
SC=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad
WT=/dev/shm/src-pf
PD=$WT/backlog/XP-20261029-packed-filter
STUDY=$WT/research/ic_packed_filter_20261029
TOOLS=$SC/pf-analysis-tools
D=$STUDY/P$b
case "$wall" in
  first) JW=ic-dgbf-p$b-20261029; WK=wall ;;
  r2) JW=ic-dgbf-p$b-20261029-r2; WK=wall ;;
  nr) JW=ic-dgbf-p$b-20261029-nr; WK=wall-nr ;;
  nr-r2) JW=ic-dgbf-p$b-20261029-nr-r2; WK=wall-nr ;;
esac
case "$counted" in
  first) JC=ic-dgbf-rho-p$b-20261029; DO_COUNTED=yes ;;
  r2) JC=ic-dgbf-rho-p$b-20261029-r2; DO_COUNTED=yes ;;
  existing) JC=none; DO_COUNTED=no ;;
esac
DW=$D/$WK
DC=$D/counted
if [ "$DO_COUNTED" = yes ]; then ANC=$DC/analysis; else ANC=$DW/analysis/counted-ref; fi
SPECW=$PD/spec-P$b.json
SPECC=$PD/spec-P$b-rho.json
FB=/dev/shm/pf-bundle/ecbench
FBSHA=21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503
REV=3271a25d02dcef6da148293fb8f9e2149ce59858
S3RUNS=s3://$ISOLAB_BUCKET/runs
S3W=$S3RUNS/$JW
S3C=$S3RUNS/$JC
WALL_PAIRS="dgb:dgbf dgb:dgb-aa dgbf-edge:dgbf dgb:dgbf-edge dgb-aa:dgbf"
COUNTED_PAIRS="rho-neg:rho-neg-aa rho-neg:dgb rho-neg:dgbf"
echo "PRELUDE_OK b=$b wall=$wall counted=$counted JW=$JW JC=$JC DW=$DW DC=$DC DO_COUNTED=$DO_COUNTED"
