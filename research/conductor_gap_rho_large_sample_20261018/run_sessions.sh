#!/usr/bin/env bash
# Run every spec as its own ecbench session, up to $SLOTS at once, each on its own
# lock file and CPU (CPUs 25.. ; the host is shared with other jobs, so every run is L0).
set -u
HERE=$(cd "$(dirname "$0")" && pwd); ROOT=$(cd "$HERE/../.." && pwd)
SLOTS=${SLOTS:-7}; BASE_CPU=${BASE_CPU:-25}
ORDER=${ORDER:-"s1a-ell8209-q29 s1b-ell8209-q29 s1c-ell8209-q29 s1d-ell8209-q29 s2-ell131101-q37 m1-ell1211653-q45 m2-ell1138019-q45 m3-ell1609903-q45 l1-pairs00-03 l2-pairs04-07 l3-pairs08-15 l4-pairs16-23"}
i=0
for name in $ORDER; do
  while [ "$(jobs -rp | wc -l)" -ge "$SLOTS" ]; do wait -n; done
  cpu=$((BASE_CPU + i % SLOTS)); i=$((i+1))
  out="$HERE/sessions/$name"; [ -e "$out" ] && { echo "skip $name (exists)"; continue; }
  echo "$(date -u +%FT%TZ) start $name cpu=$cpu"
  ( nice -n 19 taskset -c "$cpu" "$ROOT/target/release/ecbench" run --spec "$HERE/specs/$name.json" --out "$out" \
      --cpus none --lock "/tmp/cg-large-sample-$name.lock" --allow-busy --quiet > "$HERE/sessions/$name.log" 2>&1;
    echo "$(date -u +%FT%TZ) done $name exit=$?" ) &
done
wait
echo "$(date -u +%FT%TZ) all sessions finished"
