# Launch card P4: equal-advice BL preprocessing rho vs compact-orbit IC at n = 73 and n = 83

**Protocol:** [`research/equal_advice_wide_20261018/PROTOCOL.md`](../../equal_advice_wide_20261018/PROTOCOL.md) and `run.sh`, frozen by PR #2163. This card changes neither file. The job is [`job.sh`](job.sh): a cgroup-aware preflight, host facts, build, then exactly `THREADS=$(nproc) run.sh n73 n83`.
**Question and rule:** the protocol's per-rung verdicts (`BL_WINS_AT_EQUAL_ADVICE` / `IC_WINS_AT_EQUAL_ADVICE` / `UNRESOLVED`; control failures override). Class: accounting/audit, not an improvement. §5a band pricing is in the claims output.

| | |
|---|---|
| host | RunPod `cpu3g-32-128` (32 vCPU, 128 GB). n = 83 holds about 29 GB resident for both arms (IC 13.9 GiB + BL 15.4 GB); 64 GB (`cpu3c`) is too tight with the build and an unmeasured 32-thread rank stage. CPU pods stop at 32 vCPU. |
| price | **$1.28/h** (queried 2026-10-18 21:13Z) |
| stages | boot 0.15 h · build 0.25 h · n73: BL table walks (40–100 core-h, 2.1–5.2 h at 32 vCPU with SMT efficiency 0.6) + two IC processes (rank stage 18,398 s single-thread each, 0.3–1.7 h for both) · n83: BL walks 1.4–3.5 h + two IC setups 0.2–0.4 h · serving window 3 h |
| expected hours | **7.5 h** to `EA_DONE` (range 5.4–9.0; the protocol's own projection is 5–8.5 h for the two rungs) |
| expected cost | **$9.60** (range $6.90–$11.50); lifetime cap 14 h = **$17.92 at most** |
| isolation | L0 (a shared cloud host), recorded as such by `run.sh` |

## Minimal configuration (the about $12 left for P2 + P4 together)

**Launched 2026-10-18 22:08Z by the parent session:** pod `mhjde0s1ysa5aq`, `cpu3g-32-128` at $1.28/h, 6 h cap, `RUNGS=n83`, booted from this branch at `b91b196ef`.

The n = 83 rung only (`RUNGS=n83`), the rung with the recorded IC claims (root21, K = 3240), on the same `cpu3g-32-128`. Expected 3.5 h (range 2.2–4.6: BL walks 1.4–3.5 h, two IC setups 0.2–0.4 h, boot and build 0.4 h), **$4.50** (range $2.80–$5.90), cap 6 h = $7.68. With the minimal P2 that is $10.30 expected, $14.28 at the caps. n = 73 stays for a later launch: same command with `RUNGS=n73`, about $6.

```sh
python3 launch.py run --job ea-wide --name equal-advice-n83 --branch main \
  --flavors cpu3g --vcpu 32 --disk 80 --max-seconds 21600 --env SERVE_SECONDS=1800 --env RUNGS=n83
```

## Launch (full configuration)

```sh
cd research/dreg_closure_m4l6_20261018/pod
python3 launch.py run --job ea-wide --name equal-advice-n73-n83 --branch main \
  --flavors cpu3g --vcpu 32 --disk 80 --max-seconds 50400 --env SERVE_SECONDS=10800
```

## Watch and fetch

`STATUS`: `EA_PREFLIGHT_OK` (or `REJECTED_CGROUP`, then terminate), `EA_BUILD_DONE`, `EA_PHASE …` mirrors of run.sh's phase lines, `EA_DECISION n73 …`, `EA_DECISION n83 …`, `EA_DONE`.

```sh
P=research/dreg_closure_m4l6_20261018/pod/launch.py
python3 $P fetch ID research/equal_advice_wide_20261018/runs --path ea-wide/runs/
for f in STATUS ea-host.txt ea-run.stderr; do curl -sS https://ID-8000.proxy.runpod.net/$f -o research/equal_advice_wide_20261018/runs/pod-ID-$f; done
python3 $P terminate ID
```

## Known before launch (found in testing; the frozen files are not changed)

1. **`run.sh` exits 143 on success.** Its EXIT trap kills its own process group. `job.sh` counts rc 143 as success only when every rung wrote `assembled/decision.json`, and logs `EA_NOTE` with the raw code.
2. **`run.sh`'s RAM preflight reads `MemTotal`**, the host's RAM in a container. `job.sh` checks the cgroup limit first (≥ 56 GiB) and refuses below it.
3. **The A/A control (±25 %) is noise-sensitive.** On the n = 41 smoke, at about 0.1 ms intervals on a contended 1-core sandbox, it failed in two of three runs (median ratios 0.70–0.76). The verdict was then `UNRESOLVED_CONTROL_FAILED` with positive and null controls passing. At n = 73/83 the online intervals are milliseconds on L0 vCPUs, so a control-failed verdict from noise alone is possible. The rows stay valid, and the protocol's rule decides.
4. The BL walks finish before IC #1 starts, so the session wall is the sum of the stages, not the max.

## Tested before launch (authoring container)

`bash -n`, `shellcheck` clean. The frozen `run.sh n41smoke` ran directly (verdict `BL_WINS_AT_EQUAL_ADVICE`, exit 143 as above). `job.sh` with `RUNGS=n41smoke MIN_CGROUP_GIB=6` ran end to end, exit 0, 2 min warm. The rejection path at the default 56 GiB exits 2 with `ea-rejected.json`. Outputs are in `smoke_local/`.
