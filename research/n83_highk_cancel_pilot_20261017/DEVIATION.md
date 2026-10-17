# Local resource receipt correction

The frozen pilot plan requested the inherited 15.25 GiB RSS cap. A code audit
after the first child started found that the inherited `one_run` sampler reads
`/proc/<pid>/status`, which is unavailable on this macOS host. Its fallback
`wait4` receipt multiplies `ru_maxrss` by 1,024; macOS already reports that
field in bytes. The recorded `peak_rss_bytes` is therefore not a calibrated
byte measurement here, and the 15.25 GiB RSS guard was not enforced by this
runner. The 900-second wall cap remains active.

The producer and its committed flags are left as executed so the raw receipt
stays intact. The independent checker labels the local resource telemetry
`UNVERIFIED_MAC_RSS` and evaluates arithmetic, factor-base geometry, relation,
scalar, and timing-phase consistency separately. These local timings remain
exploratory regardless of their observed value. A later Tier 1 comparison
must use the frozen cgroup memory and telemetry gate on each cloud host.
