#!/bin/bash
# Starts the Phase B driver fully detached (survives the caller). Refuses on any failed check.
# Exit: 0 started; 2 setup problem; 3 disk; 4 binary sha; 5 driver already alive; 1 driver did not stay up.
STUDY=/home/user/crypto/research/volcano_ecdlp_census_followup_20261023
ECB=/home/user/crypto/target/release/ecbench
ECB_SHA=ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35
BRANCH=claude/volcano-census-followup-20261023
TRAILER_FILE=/tmp/claude-0/-home-user/a7557d77-b66e-59ae-b32e-04991c48472e/scratchpad/followup/commit_trailer.txt
MINFREE_MIB=1024
cd "$STUDY" || exit 2
got=$(sha256sum "$ECB" | cut -d' ' -f1)
if [ "$got" != "$ECB_SHA" ]; then echo "REFUSED binary sha256 $got is not $ECB_SHA"; exit 4; fi
echo "binary sha256 ok $got"
free=$(df -Pm "$STUDY" | awk 'NR==2{print $4}')
if [ "$free" -lt $MINFREE_MIB ]; then echo "REFUSED free disk $free MiB is below $MINFREE_MIB MiB"; exit 3; fi
echo "free disk $free MiB"
if [ "$(git rev-parse --abbrev-ref HEAD)" != "$BRANCH" ]; then echo "REFUSED branch is $(git rev-parse --abbrev-ref HEAD), not $BRANCH"; exit 2; fi
if [ ! -s "$TRAILER_FILE" ] || [ "$(wc -l < "$TRAILER_FILE")" != 2 ]; then echo "REFUSED trailer file $TRAILER_FILE missing or not 2 lines"; exit 2; fi
if [ -f runs/driver.pid ]; then
  pid=$(cat runs/driver.pid 2>/dev/null)
  case "$pid" in ''|*[!0-9]*) ;; *)
    if kill -0 "$pid" 2>/dev/null && tr '\0' ' ' < /proc/$pid/cmdline 2>/dev/null | grep -q run_phase_b.sh; then
      echo "REFUSED driver already alive pid=$pid"; exit 5; fi;;
  esac
  rm -f runs/driver.pid
fi
mkdir -p runs
COMMIT_TRAILER="$(cat "$TRAILER_FILE")" setsid nohup bash scripts/run_phase_b.sh >> runs/phase_b.log 2>&1 < /dev/null &
for i in $(seq 1 30); do
  sleep 1
  if [ -f runs/driver.pid ]; then
    pid=$(cat runs/driver.pid)
    if kill -0 "$pid" 2>/dev/null && tr '\0' ' ' < /proc/$pid/cmdline 2>/dev/null | grep -q run_phase_b.sh; then
      echo "DRIVER_STARTED pid=$pid"; exit 0
    fi
  fi
done
echo "DRIVER_NOT_UP after 30 s; last lines of runs/phase_b.log:"; tail -n 5 runs/phase_b.log
exit 1
