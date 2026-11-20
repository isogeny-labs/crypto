#!/bin/bash
cd /home/user/crypto
SPEC=research/curve25519_autolab_20261104/round-039/profiles/session/spec.json
BASE=/tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/bin-base/ecbench
NEW=/tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/bin-new-ecbench
for pair in "S1-base $BASE" "S2-new $NEW" "S3-new $NEW" "S4-base $BASE"; do
  set -- $pair
  date -u +"%FT%TZ start $1" >> /tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/wallsess/progress.log
  eval "bin=$2"
  $bin run --spec $SPEC --out /tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/wallsess/$1 --wait --quiet >> /tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/wallsess/$1.out 2>&1
  echo "$(date -u +%FT%TZ) done $1 rc=$?" >> /tmp/claude-0/-home-user/e375a883-d595-5f04-8ef9-a57bb8288ce5/scratchpad/wallsess/progress.log
done
