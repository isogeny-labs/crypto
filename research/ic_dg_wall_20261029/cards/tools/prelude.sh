# Sourced by every step of the dg-wall analysis card. Reads b from the environment.
set -euo pipefail
export PATH=/tmp/claude-0/awsvenv/bin:$PATH AWS_DEFAULT_REGION=us-east-2 AWS_REGION=us-east-2
case "$b" in
  24|28|32) ;;
  *) echo "STOP: b is not 24, 28 or 32: b=$b"; exit 1 ;;
esac
SC=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad
PD=$SC/wt-wall/backlog/XP-20261029-dg-wall
TOOLS=$SC/dgw-analysis-tools
D=/dev/shm/dgw-analysis/P$b
JW=ic-dgw-p$b-20261029
JC=ic-dgw-rho-p$b-20261029
DW=$D/wall
DC=$D/counted
SPECW=$PD/spec-P$b.json
SPECC=$PD/spec-P$b-rho.json
FB=/dev/shm/isolab-build/ecbench
FBSHA=88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c
REV=be2969cebf30a0cfb79088fd10094324c3efb3bb
S3W=s3://isolab-results-104186811093-us-east-2/runs/$JW
S3C=s3://isolab-results-104186811093-us-east-2/runs/$JC
WALL_PAIRS="srenew:dg srenew:srenew-aa walk:srenew walk:dg walk:dg-small srenew:dg-small"
if [ "$b" = 28 ]; then
  COUNTED_PAIRS="rho-neg:rho-neg-aa rho-neg:srenew rho-neg:dg srenew:dg"
else
  COUNTED_PAIRS="rho-neg:rho-neg-aa rho-neg:srenew rho-neg:dg"
fi
echo "PRELUDE_OK b=$b JW=$JW JC=$JC D=$D"
