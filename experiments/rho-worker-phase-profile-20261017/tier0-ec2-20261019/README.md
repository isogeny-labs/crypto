# Physical-host worker panel

This panel uses a dedicated EC2 `m5zn.metal` host to test the full rho
worker with physical core ownership. The host is separate from the user's
active research instance. The [replacement allocation](aws-r2-allocation.json)
is `i-06792af749cbdd584` in `us-east-2b`; its scheduled shutdown is
2026-10-19 05:30 UTC, and it is configured to terminate on shutdown.

The first `m5zn.metal` host (`i-047f22d5f7ea716c2`, `us-east-2a`) lost
AWS system reachability during its isolation reboot (`impaired` since
2026-10-19 01:08 UTC); the [CloudWatch metric](aws-system-reachability.json)
reports `StatusCheckFailed_System=1` at 01:10–01:16 UTC. [AWS documents](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/monitoring-system-instance-status-check.html)
that this check can fail temporarily during a bare metal restart, and
[bare metal initialization](https://docs.aws.amazon.com/ec2/latest/instancetypes/ec2-nitro-instances.html)
can take 20 minutes or more. This host was stopped and terminated before
recovery could be assessed. It produced zero benchmark arms; its receipt is
an interrupted setup, not a diagnosed hardware failure.
The [EC2 state receipt](aws-stopping.json) records the subsequent stop
transition. The host was then terminated and its empty root volume deleted.
An attempt to boot the smaller `a1.metal` host with Amazon Linux 2023 was
[rejected by glibc at boot](a1-al2023-boot-failure.txt): that image requires
Graviton 2 or later. It produced zero benchmark arms. The replacement x86
host was launched after the first host released its quota; the panel remains
unmeasured until it passes the strict preflight.

The frozen source archive SHA-256 is
`f32992105d47d63c78a7ba5bb02021ba6ba150e4c388cec55715b979d697f722`.
It was built from PR #2137 with optional `CHECKIN_EVERY=64`, so the requested
batch width 32 is reachable. The 32 public Q fixtures retain SHA-256
`f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8`.
The replacement host rebuilt the binary with SHA-256
`4882b89d974e27bf9073272cdcc461e1bc33162fd9bb0caf93fd554ae309dce2`.
Before isolation, an untimed serial q00 wrapper check returned
`verified=1`, 128 walkers, and `checkin_every=64`.

The [wrapper](worker_wrapper.py) checks every worker's exact DP replay and
charged phase sum, and exposes the raw worker JSON to the strict host runner.
The [manifest builder](build_manifest.py) derives 96 cases from the frozen
same-Q plan: one serial/batch comparison, one serial/serial control, and one
batch/batch control per Q. Four repetitions alternate arm order, for 768
arms and 384 pairs. The [independent auditor](analyze.py) checks raw output,
artifact hashes, paired fields, A/A `[0.90, 1.11]`, peak RSS, host preflight,
and a 32-Q cluster bootstrap. The improvement threshold is a 95% lower
bound greater than 1.05 with every arm and pair valid.

The benchmark CPU is physical core 23 (logical CPUs 23,47) on NUMA node 1.
The [host configuration](configure_host.sh) sets the cgroup partition,
housekeeping slices, IRQ routing, turbo disabled, and fixed requested frequency after booting with
`isolcpus=managed_irq,domain,23,47 nohz_full=23,47 rcu_nocbs=23,47`
and housekeeping `irqaffinity`. Boot isolation, the cgroup v2 isolated partition, housekeeping affinity,
IRQ routing, fixed frequency, memory binding, and per-arm noise are checked
by the repository's `scripts/isolated_bench.py`. A failed preflight or arm
remains in the raw ledger with the aggregate decision unknown.
