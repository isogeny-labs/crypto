#!/usr/bin/env bash
set -euo pipefail

root=/home/ec2-user/rho-worker-tier1-20261018
cd "$root"
test "$(sha256sum plan.json | cut -d' ' -f1)" = 385d94d1a56523d9f8fdac88a623b4cd9cd70adfe76f164f6c1ef2d9ceeebb67
test "$(sha256sum allocation.json | cut -d' ' -f1)" = 39555db18372a7753401d065abe0effa65659782f72afe38bfa98ff6e2bdeef6
test ! -e observer.pid
test ! -e runner.pid
test "$(systemctl is-active rho-hard-stop.timer)" = active
test "$(cat /sys/fs/cgroup/rho.slice/rho-runner.slice/cpuset.cpus.effective)" = 0-15

nohup sudo sh -c 'echo $$ > /home/ec2-user/rho-worker-tier1-20261018/observer.pid; exec taskset -c 0-14 python3 /home/ec2-user/rho-worker-tier1-20261018/watch_affinity.py --runner-pid-file /home/ec2-user/rho-worker-tier1-20261018/runner.pid --ready-file /home/ec2-user/rho-worker-tier1-20261018/observer-ready.json --output /home/ec2-user/rho-worker-tier1-20261018/observer-result.json --interval-s 0.25 --quiet-period-s 5 --startup-timeout-s 90' \
  </dev/null >observer.log 2>&1 &
sleep 2
test -s observer.pid
test -d "/proc/$(cat observer.pid)"

nohup sudo systemd-run --scope --slice=rho-runner.slice \
  -p AllowedCPUs=0-15 -p AllowedMemoryNodes=0 \
  /usr/bin/taskset -c 0-14 /usr/bin/python3 "$root/start_barrier.py" \
  --runner-pid-file "$root/runner.pid" \
  --ready-file "$root/observer-ready.json" \
  --observer-pid-file "$root/observer.pid" \
  --release-receipt "$root/barrier-release.json" \
  --manifest "$root/plan.json" \
  --queue-root "$root/queue" \
  --runner "$root/source/experiments/rho-worker-phase-profile-20261017/run_tier1_worker_batch.py" \
  </dev/null >runner.log 2>&1 &
echo "launch submitted; observer_pid=$(cat observer.pid)"
