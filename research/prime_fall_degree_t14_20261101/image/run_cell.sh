#!/bin/sh
# run_cell K: one cap-6 system of research/prime_fall_degree_t14_20261101 (Amendment 7 A, 112 GiB request).
# Runs pkm_tower_pilot, then uploads the row and the trace to $ISOLAB_RESULTS_URI. Exit code is the pilot's.
set -u
k=${1:?draw index 0 or 1}
name=T-t14-k$k-c6
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-16}
cd /work || exit 2
/usr/local/bin/pkm_tower_pilot --p 2013265921 --ladder-t none --planted 0 --random 2 --engine tower --kinds kummer \
  --m 2 --controls tower --budget 86400 --max-dense 27900000000 --trace --t-min 14 --max-t 14 --targets "$k" --cap 6 \
  --out "/work/$name.jsonl" 2> "/work/$name.log"
rc=$?
echo "exit $rc" >> "/work/$name.log"
sha256sum "/work/$name.jsonl" "/work/$name.log" /usr/local/bin/pkm_tower_pilot > "/work/$name.sha256" 2>&1
if [ -n "${ISOLAB_RESULTS_URI:-}" ]; then
  for f in "$name.jsonl" "$name.log" "$name.sha256"; do
    [ -f "/work/$f" ] && aws s3 cp --only-show-errors "/work/$f" "${ISOLAB_RESULTS_URI%/}/$f" || echo "upload of $f failed" >&2
  done
fi
exit $rc
