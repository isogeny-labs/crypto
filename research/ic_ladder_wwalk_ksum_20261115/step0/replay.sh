#!/bin/bash
# Step 0 (zero new group operations): replay committed k-sum sessions, with the documented command
#   ecbench verify --dir D --replay-all --exit-code --out D.json
# once with the binary built BEFORE the change and once AFTER; the two audits must be count-identical.
#   step0/replay.sh <ecbench> <label (before|after)> <scratch dir>
# The sessions' records.jsonl live in the artifact store (S3, by sha256 from .artifacts.json); they are copied next
# to a scratch copy of each session directory, so nothing in the checkout is hydrated or changed.
set -u
BIN=${1:?ecbench binary}; LABEL=${2:?before|after}; SCR=${3:?scratch dir}
R=$(cd "$(dirname "$0")/../.." && pwd)            # research/
SESS="ic_tableb_ksum_20261115/sessions/P20-r1 ic_tableb_ksum_20261115/sessions/P24-r1 prime_ksum_k5_20261028/test/sessions/P32 prime_ksum_k5_20261028/test/sessions/P36"
export RAYON_NUM_THREADS=1
mkdir -p "$SCR/sessions" "$SCR/$LABEL"
for s in $SESS; do
  n=$(echo $s | sed 's#/sessions##; s#/test##; s#/#-#g')
  d=$SCR/sessions/$n
  if [ ! -f $d/records.jsonl ]; then
    mkdir -p $d && cp -a $R/$s/. $d/
    h=$(jq -r '.files["records.jsonl"].sha256' $R/$s/.artifacts.json)
    aws s3 cp --only-show-errors s3://setec-astronomy-artifacts/store/sha256/${h:0:2}/$h $d/records.jsonl
    [ "$(sha256sum $d/records.jsonl | cut -d' ' -f1)" = "$h" ] || { echo "records hash mismatch for $s"; exit 2; }
  fi
  $BIN verify --dir $d --replay-all --exit-code --out $SCR/$LABEL/$n.json > $SCR/$LABEL/$n.log 2>&1; rc=$?
  echo "$LABEL $n rc=$rc $(tail -1 $SCR/$LABEL/$n.log)"
done
