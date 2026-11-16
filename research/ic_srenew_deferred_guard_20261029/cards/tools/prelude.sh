# Sourced by every step of the v3 analysis card. Reads b and J from the environment.
set -euo pipefail
export PATH=/tmp/claude-0/awsvenv/bin:$PATH AWS_DEFAULT_REGION=us-east-2
case "$b:$J" in
  24:ic-dg3-p24-20261029) ;;
  28:ic-dg3-p28-20261029) ;;
  32:ic-dg3-p32-20261029) ;;
  *) echo "STOP: b and J do not match the card's table: b=$b J=$J"; exit 1 ;;
esac
SC=/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad
D=$SC/dg3/P$b
SPEC=$SC/wt-dg/backlog/XP-20261029-srenew-deferred-guard-v3/spec-P$b.json
TOOLS=$SC/dg3-card-tools
FB=/dev/shm/isolab-build/ecbench
FBSHA=88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c
REV=be2969cebf30a0cfb79088fd10094324c3efb3bb
S3=s3://isolab-results-104186811093-us-east-2/runs/$J
if [ "$b" = 28 ]; then BASE=srenew; else BASE=walk; fi
echo "PRELUDE_OK b=$b J=$J BASE=$BASE D=$D"
