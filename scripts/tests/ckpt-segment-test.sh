#!/usr/bin/env bash
# Offline test of scripts/ckpt-segment.sh against a fake object store and a fake
# job that checkpoints a counter: segment deadline, a signal to the script, a
# restart from S3 on a "new node", and the final exit 0.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
export FAKE_S3_ROOT=$T/s3 CKPT_AWS=$here/fake-aws.sh CKPT_SYNC_SECONDS=1
S=$here/../ckpt-segment.sh
U=s3://b/job
# The job counts to TARGET, committing a generation every step, and on SIGTERM
# commits and exits 143.  It resumes from the newest generation in DREG_CKPT_DIR.
cat > $T/job.sh <<'JOB'
#!/usr/bin/env bash
set -euo pipefail
d=$DREG_CKPT_DIR; target=$1
g=$(ls "$d" 2>/dev/null | grep '^gen-' | sort | tail -1 || true)
n=0; gen=0
if [ -n "$g" ]; then n=$(cat "$d/$g/count"); gen=$((10#${g#gen-})); fi
stop=0; trap 'stop=1' TERM
commit() { gen=$((gen + 1)); gg=$(printf 'gen-%020d' $gen); mkdir -p "$d/$gg"; echo $n > "$d/$gg/count"; echo "{}" > "$d/$gg/checkpoint-manifest.json"; }
while [ $n -lt $target ]; do
  n=$((n + 1)); sleep 0.2; commit
  if [ $stop = 1 ]; then exit 143; fi
done
exit 0
JOB
chmod +x $T/job.sh
# segment 1: a 3 s deadline stops it part-way
rc=0; "$S" $U $T/node1 3 -- $T/job.sh 40 || rc=$?
[ $rc = 75 ] || { echo "FAIL segment 1 exit $rc, wanted 75"; exit 1; }
n1=$(cat $T/node1/ckpt/$(ls $T/node1/ckpt | sort | tail -1)/count)
[ "$n1" -gt 0 ] && [ "$n1" -lt 40 ] || { echo "FAIL segment 1 count $n1"; exit 1; }
# segment 2 on a new node: a signal to the script itself mid-run
"$S" $U $T/node2 60 -- $T/job.sh 40 2>/dev/null & p=$!
sleep 3; kill -TERM $p; rc=0; wait $p || rc=$?
[ $rc = 75 ] || { echo "FAIL segment 2 exit $rc, wanted 75"; exit 1; }
n2=$(cat $T/node2/ckpt/$(ls $T/node2/ckpt | sort | tail -1)/count)
[ "$n2" -gt "$n1" ] || { echo "FAIL segment 2 did not continue: $n1 -> $n2"; exit 1; }
# segment 3 finishes
rc=0; "$S" $U $T/node3 120 -- $T/job.sh 40 2>/dev/null || rc=$?
[ $rc = 0 ] || { echo "FAIL segment 3 exit $rc"; exit 1; }
n3=$(cat $T/node3/ckpt/$(ls $T/node3/ckpt | sort | tail -1)/count)
[ "$n3" = 40 ] || { echo "FAIL final count $n3"; exit 1; }
echo "ckpt-segment: all checks passed ($n1 -> $n2 -> $n3)"
