# Fresh N83 high-K wide3 factor-base comparison on cgroup v1 Pods

This panel tests whether the replayed K4300 and K4405 wide3 root indices improve
one-target online time over the previously admitted K3300 wide3 control. The
exact curve is `EC1N83Ckb1a1haacb69ee9f37`, with subgroup order
`8569786107849059`; the canonical curve, generator, source and factor-base
records are in the three candidate manifests resolved by `arms.py`.

| Arm | Candidate ID | Actual points | Folded columns | Exact root request |
| --- | --- | ---: | ---: | --- |
| K3300 wide3 | `IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586` | 547,800 | 3,300 | fixed30, 29-bit state ID |
| K4300 wide3 | `IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77` | 713,800 | 4,300 | load75 id30, 30-bit state ID |
| K4405 wide3 | `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7` | 731,230 | 4,405 | load75 id30, 30-bit state ID |

`generate_fresh.py` fixes 16 public points with domain
`n83-highk-wide3-cgroupv1-fresh16-20261019-v1` and scalar rule
`1 + LE(SHA256(domain || u32le(index))) mod (r-1)`. The public-point file
SHA-256 is `ba421011d9596321f60049cff34d5178b20c34aac4e095ef19cf34c5b7fec294`;
the sealed answer sidecar SHA-256 is
`57c1d77e49d89b78aae1441abd132b406087ff7bc77bc5fd4c06ce2aee2d281a`.
`audit_fresh.py` checked all 16 for subgroup membership and uniqueness and
found zero overlap with 31,536 distinct points in the prior sealed fixture
inventory, including all 16 points in the stopped cgroup-v2-only diagnostic.

Two distinct, concurrent Secure Cloud RunPod CPU Pods each receive one fixed
eight-point stratum. Require at least four visible CPUs, 24 GiB child RSS
headroom plus 256 MiB, 3 GiB free disk, and distinct machine IDs. Use the
4-vCPU/32-GiB memory-optimized CPU class if its live price is at most $0.28
per Pod hour; the diagnostic K4405 child peaked at 22.35 GB RSS under the
same 24-GiB child cap. Record the cgroup v1 memory limit at preflight;
freeze the actual allocation, price, image, source commit and two Pod IDs
before starting. The six stopped prior allocations cost an estimated $0.657459;
`prior_attempts.json` reserves $0.70 against the approved **$5 combined cap**.
The new pair has a $4.30 hard spend cap and a 26,100-second allocation cap;
each producer stops before 25,200 seconds. At the $0.28 per-Pod hourly ceiling,
the pair's maximum is $4.06 and the conservative campaign total is $4.76.
The controller stops both Pods before either cap. Keep every partial receipt.
No API key is sent to a Pod or stored in the artifacts.

For each point, rotate the three arms, alternate IC/rho order, and run three
same-point rho calibration children before and after each pair. Execute every
IC child in a fresh process with cold index/base construction, then record its
target-dependent interval separately. The online interval starts at first
target query and ends at scalar recovery/check; target-independent startup,
base and index construction are outside that interval. Charge all attempts
and target-dependent phases; record cold process time as a separate metric.
The per-child limits are 1,200 seconds for K3300, 1,500 seconds for K4300/
K4405, and 120 seconds for rho and calibration. A 24-GiB RSS cap, zero
throttling/steal/OOM, and at most 10% before/after median calibration drift
gate each pair. Timeout, missing data and noise remain explicit rows.

The prior runner read cgroup v2 paths on Pods exposing cgroup v1, so its
completed producer evidence is preserved as a capped diagnostic and its Tier 1
status remains `NOT_ADMITTED`. This runner reads v1 `cpu.stat`,
`cpu.cfs_{quota,period}_us`, `memory.{usage,max_usage,limit}_in_bytes`,
`memory.failcnt`, and `memory.oom_control` before and after each child. It
retains the raw strings and checks the normalized gate fields during replay.
Using v1 `memory.failcnt` as the OOM-event gate is conservative: a zero delta
establishes no memory-limit denial. Any missing counter, throttle, limit
denial, OOM kill, steal, or failed calibration blocks Tier 1 admission.

`run.py` freezes 192 ordered events per Pod. `runpod_runner.py` binds the
allocation and source commit; `verify.py` independently replays the base,
four-point witness, recovered scalar, exclusive phases, resource telemetry,
same-point rho and calibration for every child. `runpod_provider.py` proves
distinct overlapping provider allocations. `aggregate_runpod.py` hashes the
raw archives and reruns both strata before selection. Admit the 16-point
matrix only if both Tier 1 strata pass. `select.py` applies all three exact
paired sign-flip tests at Bonferroni `0.05/3` separately to log online ratios
and cold process differences. A selected online candidate must beat both
other arms in the prescribed direction at that threshold; otherwise its
status is `BOUNDED_UNKNOWN` and its selected ID is `null`. Report per-point
same-target rho/IC online ratios with the observed Tier 1 envelope.

This is an `a=1` challenger panel. The primary `a=0` factor-base search and
the live wider `a=1` extended holdout retain their own frozen protocols and
selection status. A high-K panel winner enters a subsequent one-target
holdout and cross-family comparison; this panel alone does not select an
absolute N83 factor base.
