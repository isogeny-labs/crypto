# Protocol amendment 1: RunPod CPU pods instead of Iso Lab EKS L2

**Committed 2026-11-12, before any data under `PROTOCOL.md`.** It changes
only what is listed here. Everything else in `PROTOCOL.md` (arms, grid,
order, counted units, controls, acceptance and stop rules, frozen analysis,
falsification tests) stands.

## Why

The owner asked on 2026-11-12 for the run on RunPod rather than Iso Lab
(chat: "1 but on runpod"). The repository's rules allow an explicitly
requested environment (CLAUDE.md, "Preserve explicitly requested
environments").

## Changes

1. **Host.** RunPod Secure Cloud CPU pods, flavor `cpu5g` (General
   Purpose, 4 GiB per vCPU), 4 vCPU, 16 GiB, on-demand (not interruptible),
   image `rust:1.97.0-bookworm` (rustc 1.97.0, the same version as the
   2026-11-12 devcheck). The protocol's "Iso Lab EKS L2 worker (native L2
   admission, `cpu-l2-spot`)" and the stop rule "if the node is not admitted
   at native L2" do not apply. There is no L2 admission on RunPod. Each pod
   records `nproc`, CPU model, cgroup CPU and memory limits, kernel, rustc
   and the binary's SHA-256 in `host.json`. Wall time stays **secondary and
   host-dependent**, as the protocol already says. Counted units are
   host-independent and unaffected.
2. **One pod per seed.** Seeds 1, 2, 3 each run on their own pod at the
   same time, all with the same flavor. Each pod runs every D for its seed,
   in the protocol's rotated arm order `(log2 D + seed) mod 14`. All 14 arms
   of one (D, seed) cell therefore run on the same pod, which keeps the width
   comparison within one host. Comparisons across seeds also cross hosts.
   The width law (analysis 2) uses ratios within a pod, and its spread over
   seeds is reported as min–max as frozen.
3. **A/A per pod.** Each pod runs the A/A pair (`full-bw` at `U192@129` and
   `u64@61`, D = 2¹⁶, seed 1) before its grid, after its D = 2¹⁶ block (the
   midpoint: size 5 of 9), and after its grid. Determinism is checked across
   all nine `U192@129` runs and all nine `u64@61` runs from the three pods:
   counted units must be identical. The wall-time band is computed per pod.
   A width ratio is "not resolved" if it lies inside the band of the pod that
   measured it.
4. **Secondary (`--coeffs wide`).** Each pod runs its seed's 14 arms at
   D = 2¹⁶, right after the midpoint A/A.
5. **Records.** Each run's raw record line goes to `grid.jsonl`, `aa.jsonl`
   or `wide_coeffs.jsonl`, exactly as the harness printed it. If a run exits
   non-zero with no record, a `{"failed":true,...}` line goes there instead,
   as the protocol's launcher does. The script also writes `runs.jsonl` with
   one line per attempt: phase, arm, D, seed, B, word, attempt (1 or 2),
   exit code, validity, outer wall time and a stderr tail. It also writes a
   `skipped` line for each run that the stop rule skips. The repeat rule
   (one repeat of an invalid run, none after a timeout or OOM kill) and the
   per-arm stop rule (a timeout, exit 124, or an OOM kill, exit 137, skips
   the arm's larger D on that pod) run on the pod (`runpod/pod_run.sh`).
6. **Source identity.** The pod builds from a minimal source bundle of the
   tested commit (`runpod/make_bundle.py`: the manifest, every declared
   target, path dependencies and every `include*!`/`#[path]` file, taken from
   `git ls-files`). The bundle's SHA-256 is checked on the pod before the
   build. The commit, the bundle hash, the script hash and the pod ids are in
   `launch.json`. Raw data goes to
   `s3://setec-astronomy-artifacts/lab-results/wide-sparse-la/<stamp>-<sha>/seed<s>/`
   through presigned PUT URLs that expire after 7 days.
7. **Budget.** USD 25 ceiling, as proposed. Planned: three pods at the
   quoted USD 0.184 per pod-hour, about 13 to 16 h each from a local sizing
   probe (D = 2¹⁴ and 2¹⁶, u64@61 and U256@255; growth about ×3.3 per
   doubling of D). That is about USD 9. The probe's timings are planning
   numbers, not data. Each pod has a hard deadline 40 h after launch
   (`DEADLINE_EPOCH`), when it stops and uploads what it has. That caps the
   worst case at 3 × 40 h × 0.184 ≈ USD 22. A pod terminates itself when it
   finishes. If it cannot, the session that launched it terminates it.
   Two retries (a whole-pod relaunch) are allowed within the ceiling.
8. **Restart guard.** If a pod's container restarts, the script uploads
   what it has with status `container_restarted` and stops. It does not
   start the grid again over the uploaded data. Such a pod counts as a
   retry.
