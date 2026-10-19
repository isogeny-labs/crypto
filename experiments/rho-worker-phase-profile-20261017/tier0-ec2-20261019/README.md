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

The frozen [source archive](source-64.tar.gz) SHA-256 is
`f32992105d47d63c78a7ba5bb02021ba6ba150e4c388cec55715b979d697f722`.
It was built from PR #2137 with optional `CHECKIN_EVERY=64`, so the requested
batch width 32 is reachable. The 32 public Q fixtures retain SHA-256
`f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8`.
The replacement host rebuilt the binary with SHA-256
`4882b89d974e27bf9073272cdcc461e1bc33162fd9bb0caf93fd554ae309dce2`.
The [build log](build.log) records the release build. Before isolation, an
[untimed serial q00 wrapper check](q00-serial-smoke.txt) returned
`verified=1`, 128 walkers, and `checkin_every=64`.

The initial wrapper-based [worker plan](worker-plan.json) SHA-256 is
`237a24501204b69208eb4467151eeca088887b4d896bcd326c02edc9de0bd0d3`;
the strict [host manifest](manifest.json) SHA-256 is
`a24f3fb6b703a99d99910238a8c9e8899ddac83d4f89540dd985fadce5de0cfc`.
The [host probe](host-probe.json), [configuration log](configure-host.log),
and [strict preflight](preflight.json) were captured after the isolation
reboot. Preflight passed with no problems: `cpuset.cpus.partition=isolated`
on CPUs `23,47`, zero effective IRQ overlaps, no CPU quota, physical host
detection, NUMA node 1, and 3,800,000 kHz `performance` min/max readbacks
on both benchmark CPUs.

The [wrapper](worker_wrapper.py) checks every worker's exact DP replay and
charged phase sum, and exposes the raw worker JSON to the strict host runner.
The [manifest builder](build_manifest.py) derives 96 cases from the frozen
same-Q plan: one serial/batch comparison, one serial/serial control, and one
batch/batch control per Q. Four repetitions alternate arm order, for 768
arms and 384 pairs. The [independent auditor](analyze.py) checks raw output,
artifact hashes, paired fields, A/A `[0.90, 1.11]`, peak RSS, host preflight,
and a 32-Q cluster bootstrap. The improvement threshold is a 95% lower
bound greater than 1.05 with every arm and pair valid.

Before the full dispatch, one q00 serial/batch pair is run with the same
strict service as an infrastructure calibration. Its result is retained
separately and does not enter the 32-Q speed decision. The complete panel
manifest is frozen before this calibration begins.

The [first calibration archive](calibration-attempt1.tar.gz) is SHA-256
`04c3f4d16aa7fdf6cd907123f59737c82e7aff20eb54b796f9920ddf75db630c`.
Its first serial arm completed the exact worker replay, but the strict runner
marked it invalid for CPU pressure, 81 interrupt counts, four
involuntary switches, and major faults; it did not run the candidate arm.
The [direct serial](direct-noise-diagnostic.json) and [direct batch](direct-batch-noise-diagnostic.json)
diagnostics identify local `LOC`, `IWI`, `RES`, `CAL`, and `TLB` interrupts
with no device IRQ overlap or charged IRQ/softirq CPU time. The Python
wrapper also launches a second process on the measured core. These
diagnostics prompted a separately frozen direct-output attempt; the first
calibration stays invalid.

The revised strict [runner adapter](isolated_worker_runner.py) retains the
generic host preflight, exclusive queue lock, raw ledgers and artifact
checks. Its declared local-noise limits are five local IPIs per second plus
ten per arm, at most 1,000 microseconds of CPU pressure, and at most two
involuntary switches. Device IRQs, CPU-time IRQ/softirq/steal, throttling,
OOM, thermal changes and major faults remain zero-tolerance. The direct
Rust output removes the Python wrapper from the measured cgroup. Every
counter remains in the arm ledger, and the independent auditor recomputes
the limits from raw before/after values.

The first full direct-output dispatch was stopped after 19 arms and 9 pairs;
the [complete partial archive](full-r3-aborted.tar.gz) is SHA-256
`2c999e38d98919e516fdeece34c91dfaf3c959e0c0e80e93536db1dad030a29b`.
Its eighth arm counted one `MCP` event on each logical CPU and was marked
invalid by the listed-source rule; no numeric device IRQ changed. This
partial run has no aggregate. The next frozen policy classifies numeric
`/proc/interrupts` labels as device IRQs and named labels as local
architectural counters, with the same local-count and pressure bounds. It
retains every source delta and still requires zero numeric device IRQs.

The revised [source archive](source-r3.tar.gz) was generated twice with the
same SHA-256 `e369360087a53f44362043ce6f10d5a884f07da09142c0387c70a05708aa5c72`.
Its rebuilt binary SHA-256 is
`d6970ff60594f015fd3331c3d6cff3942d539a7ccad237604cf2ec33d73b79ea`;
the [build log](build-r3.log) and separate [serial](q00-serial-direct-smoke.txt)
and [batch](q00-batch-direct-smoke.txt) exact-output checks are retained.
The revised [worker plan](worker-plan-r3.json) SHA-256 is
`7b36aa4eee850ebff38f85ea849d2f692b3526603b7d31d80e303fe10c9b45f7`;
the full [Tier 0 manifest](manifest-r3.json) SHA-256 is
`e6d77a75e090bd41cb972e09e8337fdd74de7b871a413f609f60de18e2f7ccfd`.
The [full preflight](preflight-r3.json) passed with no problems. The separate
[second calibration manifest](calibration-manifest-r3.json) SHA-256 is
`aec4c0536385c36f061ac689fcb2f18936a17cfdd70c7dba6eb85d733337179c`;
its [preflight](calibration-preflight-r3.json) also passed.
The [complete second calibration archive](calibration-r3-complete.tar.gz)
is SHA-256 `0c5d33005ba814627cba8db5a5aa550e5ebc1b45a1fa5fb3332fd7b2abfc9e99`.
Both arms and their exact-output pair passed the strict service. The serial
arm recorded 66 local interrupt counts, 3 microseconds of CPU pressure,
zero involuntary switches and zero major faults; batch recorded 15, 8,
zero and zero respectively. The full 32-Q panel uses the already frozen
manifest and a fresh serialized queue.

The benchmark CPU is physical core 23 (logical CPUs 23,47) on NUMA node 1.
The [host configuration](configure_host.sh) sets the cgroup partition,
housekeeping slices, IRQ routing, turbo disabled, and fixed requested frequency after booting with
`isolcpus=managed_irq,domain,23,47 nohz_full=23,47 rcu_nocbs=23,47`
and housekeeping `irqaffinity`. Boot isolation, the cgroup v2 isolated partition, housekeeping affinity,
IRQ routing, fixed frequency, memory binding, and per-arm noise are checked
by the repository's `scripts/isolated_bench.py`. A failed preflight or arm
remains in the raw ledger with the aggregate decision unknown.
