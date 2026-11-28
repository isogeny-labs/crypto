mkdir -p /results/cgroup
cgsnap() {
  for f in memory.max memory.peak memory.events memory/memory.limit_in_bytes memory/memory.max_usage_in_bytes memory/memory.oom_control memory/memory.failcnt; do
    if [ -r "/sys/fs/cgroup/$f" ]; then
      cat "/sys/fs/cgroup/$f" > /tmp/cgsnap.tmp 2>/dev/null && mv -f /tmp/cgsnap.tmp "/results/cgroup/$(echo "$f" | tr / _)"
    fi
  done
  date -u +%FT%TZ > /tmp/cgsnap.tmp && mv -f /tmp/cgsnap.tmp /results/cgroup/snapshot-at
}
( cat /proc/self/cgroup > /results/cgroup/proc-self-cgroup 2>/dev/null
  stat -fc %T /sys/fs/cgroup > /results/cgroup/fstype 2>/dev/null
  n=0
  while :; do
    if [ -e /results/runner_state.json ]; then cgsnap; date -u +%FT%TZ > /results/cgroup/final-at; exit 0; fi
    if [ $((n % 6)) = 0 ]; then cgsnap; fi
    n=$((n + 1)); sleep 5
  done ) &
exec python3 -u -c "import base64;exec(base64.b64decode('<server_pinned.py base64>'))"
