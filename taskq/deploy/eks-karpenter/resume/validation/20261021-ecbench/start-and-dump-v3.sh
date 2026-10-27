set -euo pipefail
cd /work/qualification-v3
export HOME=/work/qualification-v3 TMPDIR=/work/qualification-v3/tmp
/work/tools/setpriv --reuid=1000 --regid=1000 --clear-groups --bounding-set=-all --inh-caps=-all --ambient-caps=-all --no-new-privs /work/tools/isolab-criu-progress /usr/local/bin/ecbench /work/qualification-v3/spec.json /work/qualification-v3/session > run.stdout 2> run.stderr < /dev/null &
payload_pid=$!
printf '%s\n' "$payload_pid" > payload.pid
for attempt in {1..200}; do
  [[ -r /proc/$payload_pid/task/$payload_pid/children ]] || break
  children=$(cat /proc/$payload_pid/task/$payload_pid/children)
  if [[ -n "$children" ]]; then
    kill -STOP "$payload_pid"
    for child in $children; do kill -STOP "$child" || true; done
    break
  fi
  sleep 0.05
done
if [[ -z ${children:-} ]]; then
  cat run.stderr
  echo 'No active native child observed; no dump attempted' >&2
  exit 2
fi
for proc in "$payload_pid" $children; do
  cat "/proc/$proc/status" > "process-$proc-status.txt"
  ls -l "/proc/$proc/fd" > "process-$proc-fds.txt"
done
set +e
LD_LIBRARY_PATH=/work/tools/lib /work/tools/criu dump --tree "$payload_pid" --images-dir /work/qualification-v3/dump --shell-job --leave-stopped --manage-cgroups=ignore --cpu-cap=all --file-locks --log-file dump.log -v4
result=$?
printf '%s\n' "$result" > dump.exit
# Preserve failed dump and stopped workload for inspection; no measurements from this run may be admitted.
tail -35 /work/qualification-v3/dump/dump.log
exit "$result"
