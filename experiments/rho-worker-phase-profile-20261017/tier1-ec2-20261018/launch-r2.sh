#!/usr/bin/env bash
set -euo pipefail

root=/home/ec2-user/rho-worker-tier1-20261018
cd "$root"
test "$(sha256sum plan-r2.json | cut -d' ' -f1)" = 694468acea0068e0c6d1a3a58301b1dc213ae62c73bb36e61f39542fcf94792b
test "$(sha256sum allocation-r2.json | cut -d' ' -f1)" = 19ac8cda5d1af7ecd8298b26d9e4a55b6dad379c7ef35e0d063abfab2610db1d
test ! -e observer-r2.pid
test ! -e runner-r2.pid
test "$(systemctl is-active rho-hard-stop.timer)" = active
test "$(cat /sys/fs/cgroup/rho.slice/rho-runner.slice/cpuset.cpus.effective)" = 0-15

nohup sudo sh -c 'echo $$ > /home/ec2-user/rho-worker-tier1-20261018/observer-r2.pid; exec taskset -c 0-14 python3 /home/ec2-user/rho-worker-tier1-20261018/watch_affinity.py --runner-pid-file /home/ec2-user/rho-worker-tier1-20261018/runner-r2.pid --ready-file /home/ec2-user/rho-worker-tier1-20261018/observer-r2-ready.json --output /home/ec2-user/rho-worker-tier1-20261018/observer-r2-result.json --interval-s 0.25 --quiet-period-s 5 --startup-timeout-s 90' \
  </dev/null >observer-r2.log 2>&1 &
sleep 2
test -s observer-r2.pid
test -d "/proc/$(cat observer-r2.pid)"

nohup sudo systemd-run --scope --slice=rho-runner.slice \
  -p AllowedCPUs=0-15 -p AllowedMemoryNodes=0 \
  -p CPUAccounting=yes -p MemoryAccounting=yes -p CPUQuota=1600% \
  /usr/bin/taskset -c 0-14 /usr/bin/python3 "$root/start_barrier.py" \
  --runner-pid-file "$root/runner-r2.pid" \
  --ready-file "$root/observer-r2-ready.json" \
  --observer-pid-file "$root/observer-r2.pid" \
  --release-receipt "$root/barrier-r2-release.json" \
  --manifest "$root/plan-r2.json" \
  --queue-root "$root/queue-r2" \
  --runner "$root/source/experiments/rho-worker-phase-profile-20261017/run_tier1_worker_batch.py" \
  </dev/null >runner-r2.log 2>&1 &
echo "launch submitted; observer_pid=$(cat observer-r2.pid)"
