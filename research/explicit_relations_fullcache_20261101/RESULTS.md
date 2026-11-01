# Full-cache specialization: completed matched recoveries

The second original large input now has a complete matched comparison. On a
fresh RTX 4090, the uninstrumented full-cache candidate completed the 63-bit
seed202 recovery in 1053.896752 seconds. The identical baseline executable took
1183.823571 seconds before it and 1185.763838 seconds after it: **11.05% lower
cold recovery time** relative to their mean, with **0.164% baseline repeat
spread**. All three certificates match byte-for-byte, each with 344 relations
and 13,733 attempts, fresh CPU replay and scalar+1 rejection.

Both original large inputs now have complete within-host matched comparisons.
The earlier first-large result and repeated small controls are preserved below;
the new second-large set ran on a different host. No clocks are pooled across
hosts or substituted into the original incomplete panel.

| Field bits / seed | Run | Rounds | Baseline b s | Candidate v s | Baseline repeat x s | Candidate / baseline mean |
|---|---|---:|---:|---:|---:|---:|
|52 /101|v2|5|6.623034|5.581687|6.712873|0.832521|
|52 /202|v2|5|4.957987|4.097162|4.937561|0.825690|
|63 /101|v2|1|2591.625752|2329.970949|2591.933570|0.898985|
|63 /202|v3|1|1185.763838|1053.896752|1183.823571|0.889519|

Cold solver clocks include target construction, startup, curve validation, base
construction, relation collection, elimination, in-process replay, serialization
and shutdown. Fresh external CPU replay is recorded separately. Small timing
columns are per-arm medians; their ratio is the median of paired ratios, not a
ratio of those medians. These are L0 descriptions on shared hosts, with one
matched round per large input. [Version 2 paired rows](retained-analysis-v2/ratios.csv)
and [version 3 paired rows](results-v3/ratios.csv) preserve exact values.

For the new set, relation-oracle times were 1183.232501s (x), 1053.243613s (v),
and 1185.126132s (b); linear algebra took 0.107728s, 0.107401s and 0.107395s.
The cold candidate/mean-baseline ratio is 0.8895192034322242; the oracle ratio
is 0.8894291581882858. The candidate saves 130.897 cold seconds on this input.
The A/A relative spread is abs(b-x)/mean(b,x)=0.0016376415359776618.

All version 3 checks passed: 29 driver tests and Clippy, 37,584 host and 37,584 device
arithmetic cases, 315 direct query comparisons, backend/cache-fallback checks,
both service/driver self-tests, six clean sanitizer runs, and exclusion of
profiling kernels/clock instructions from candidate PTX. CUDA again reports
40 registers/56-byte stack for the specialized kernel versus 48 registers/120-byte
stack for the generic kernel, with no spills. Tables had 549,596,520 left rows and
444,762,500 cached right rows. Conservative admission 14,920,341,256 bytes is an
estimate, not measured peak memory.

The new [canonical export](results-v3/export.json) passes schema, referenced-file
hashes and frozen-protocol coverage with zero findings. The frozen native
[analyzer](analyze-v3.rs) was replayed locally on a separate copied result tree;
all five analysis outputs were byte-identical to the worker outputs. Its
[replay record](results-v3/analysis-replay.json) binds those hashes. All three
certificates have SHA256
`5705f40439da5818892a896f70c3e6c7b5dff0702519e33e0d7596aef9b238a4`.

Source 64901ebb2182790f0629ed637b507aea4da48bee was clean, with GPU code and direct
driver dependencies unchanged from the version 2 measured source. The
[version 3 protocol](protocol-v3.json) was frozen before new data. CUDA 13.2,
Rust 1.88.0 and the image digest were pinned; the worker reported an RTX 4090 with
24,564 MiB framebuffer. No local root crypto/plugin build was run.

Pod k1mjc0sf1gpzvq completed, controller exit 0, and deletion is verified.
Request through verified cleanup took 3780.073 seconds. Charging 3840 seconds
conservatively leaves 5160 seconds of the original worker budget; no GPU is
running. [Budget accounting](results-v3/budget.json) retains the timestamps.
The final archive SHA256 is
`5887d994f67ba53061cb9033475143b801c2c0bc063b15fb7c45232d8dd7c149`;
S3 readback was byte-identical. [Archive manifest](results-v3/archive-manifest.json)
identifies every producer file, including PTX retained in the archive.

Historical interruptions and failures remain unchanged:

| Run / stratum | Protocol planned | Actually attempted | Verified complete | Interrupted | Not started |
|---|---:|---:|---:|---:|---:|
|v2 /52-bit|30|30|30|0|0|
|v2 /63-bit|6|4|3|1|2|
|v3 /63-bit seed202|3|3|3|0|0|

Across v2/v3 there are 36 verified recoveries plus one interrupted execution among
37 actual attempts. The two unstarted v2 slots remain recorded. Version 1's
stale-lock preflight failure ran zero recovery cells. The first version 3 launch
was refused with HTTP 402 before allocation; its [receipt](launch-refused-v3.json)
remains preserved and consumed no GPU budget.

Version 2's worker was externally deleted during its second input. The original
frozen whole-panel analyzer still correctly rejects its incomplete coverage.
The separate [retained-set adapter](analyze-retained.rs) and
[custody record](retained-analysis-v2/analysis-custody.json) remain explicitly
unblinded, post-interruption descriptive analysis. Version 2's large export
retains four infrastructure-shortfall coverage findings. The new complete
version 3 export does not rewrite those historical records.

The inspected historical [version 2 report](reports/run-20261101T0412Z/report.pdf)
and [version1 report](reports/run-20261101T0400Z/report.pdf) remain unchanged.
Large logs stay loose because the canonical exports and replay checks name
individual repository paths; policy-selected logs use artifact-store pointers.
Hydrate them before replay:

```sh
python3 tools/artifacts/artifacts.py pull research/explicit_relations_fullcache_20261101/results-v3
rustc --edition=2021 -D warnings -O research/explicit_relations_fullcache_20261101/analyze-v3.rs -o /tmp/fullcache-analyze-v3
cp -a research/explicit_relations_fullcache_20261101/results-v3 /tmp/fullcache-v3-replay
/tmp/fullcache-analyze-v3 /tmp/fullcache-v3-replay
```

The existing committed-state metrics reader discovers the version 3 export and
retains separate version1 preflight and version 2 interruption accounting. The
full exporter previously exceeded its 60-second limit with both changed and
unchanged versions on this workspace; focused committed-state checks cover this
study without claiming that broader runtime check passed.
