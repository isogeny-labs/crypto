#!/bin/bash
cd "$(dirname "$0")"
for pu in 257:3000 521:3000 1031:3000 2053:1000 4099:1000; do
  p=${pu%%:*}; u=${pu##*:}
  ./runlog.sh raw/COMMANDS.log "timeout 600 ./l06/target/release/l06 census --p $p --u-count $u --prime-target 120 --draw-cap 40000 --out raw/census_p$p.jsonl 2> raw/census_p$p.log" raw/census_p$p.jsonl raw/census_p$p.log
  echo "p=$p exit=$?" >> raw/exit_codes.txt
done
