# Big-node runs: the (4,6) closure cells and PKM m = 3, N = 21 (2026-10-18)

**Status: registered, not yet run.** This directory holds everything needed to
run the two memory-blocked exponent questions named in
[`research/ic_leads_review_20261018/`](../ic_leads_review_20261018/README.md)
on a rented host, under the owner's 50-dollar total budget. The protocol is
frozen in [`PROTOCOL.md`](PROTOCOL.md) (§8 is the budget amendment); the PKM
msolve companion is [`../pkm_m3_n21_msolve_20261018/PROTOCOL.md`](../pkm_m3_n21_msolve_20261018/PROTOCOL.md).
No result is in this directory yet; results land in `draws/` and `runs/` by
the result PR, with the run report, graph and PDF the research-visuals skill
requires.

## What runs

| job | host | cells | cost |
|---|---|---|---|
| `pkm-msolve` | 8 vCPU, 32 GB CPU pod | item-3 msolve on the committed `m3_t7` (N = 21) systems, control `m3_t5` first, then `m3_t8` if time remains | 0.32 $/h, 10 h cap |
| `closure` | any host with ≥ 200 GB cgroup memory | A1 (4,(15,6)), A2 (4,(16,6)) at degree 4, two draws side by side; then C1 (3,(19,7)) draw 0 at degree 5; B cells and C2 not in budget | 1.8–4.2 $/h, 10–12 h cap |

The closure engine is single-threaded; the host is bought for its memory.

## How to launch (RunPod; `RUNPOD_API_KEY` in the environment)

```sh
cd research/dreg_closure_m4l6_20261018/pod
# PKM msolve cell (launched 2026-10-18 05:57Z as pod dq3pb1b1m7qoc8)
python3 launch.py run --job pkm-msolve --name pkm-m3-n21-msolve-20261018 \
  --branch claude/bignode-dreg-pkm-20261018 --vcpu 8 --flavors cpu3g,cpu5g --disk 40 --max-seconds 36000

# Closure cells, cheapest host with 250 GB (2 x RTX 4090, 1.78 $/h; GPUs unused)
python3 launch.py run --job closure --name dreg-closure-m4l6-250g-2x4090-20261018 \
  --branch claude/bignode-dreg-pkm-20261018 --gpu "NVIDIA GeForce RTX 4090" --gpu-count 2 \
  --min-ram-per-gpu 120 --min-vcpu-per-gpu 12 --disk 100 --max-seconds 43200

# or a 256 GB CPU pod when RunPod has capacity (none was available on 2026-10-18 05:45-06:00Z)
python3 launch.py run --job closure --name dreg-closure-m4l6-256g-20261018 \
  --branch claude/bignode-dreg-pkm-20261018 --vcpu 32 --flavors cpu3m,cpu5m --disk 120 --max-seconds 36000
```

`launch.py ls|status ID|stop ID|terminate ID` manage pods. The pod carries no
secret; it fetches `pod/boot.sh` from this branch, builds the examples, runs
the frozen `run.sh`, and serves its study directory read-only at
`https://<pod-id>-8000.proxy.runpod.net/` (`STATUS`, `closure/`, `pkm/`,
`build.log`). It ends itself 45 minutes after its last run or at the cap.

## The one manual step: commit the draws before measurement

The protocol requires the fresh draws to be committed before any closure
measurement. The pod generates them (`STATUS` reads `GEN_DONE`) and then
waits for the tag `draws-m4l6-20261018` on `origin`:

```sh
P=https://<pod-id>-8000.proxy.runpod.net/closure/draws
cd research/dreg_closure_m4l6_20261018 && mkdir -p draws && cd draws
for f in m4-15-6.jsonl m4-15-6.log m4-16-6.jsonl m4-16-6.log sha256.txt; do curl -sS -o $f $P/$f; done
sha256sum -c sha256.txt && cd .. && git add draws && git commit -m "draws: m4 (15,6) and (16,6), seed 20261018" \
  && git push && git tag draws-m4l6-20261018 && git push origin draws-m4l6-20261018
```

The pod polls every two minutes and starts `measure` once the tag exists.

## Fetching results

Copy `closure/runs/` (one `.jsonl` and `.log` per draw, `queue.log`,
`host-*.txt`) and `pkm/runs/` (`*.log`, `*.out`, `*.mem`, `progress.txt`,
`host.txt`) from the pod URL into the two study directories before the pod's
lifetime ends, then write the run report per the research-visuals skill and
score by PROTOCOL.md §4 and the msolve protocol's rule.

## Files

- `PROTOCOL.md`: frozen cells, instrument, decision rule, predictions; §8 budget amendment; §9 PKM scope.
- `run.sh`: `host | gen | measure | control | deg5 [C2]`; reads the cgroup memory limit.
- `pod/boot.sh`, `pod/launch.py`: host plumbing (no research computation).
- `../pkm_m3_n21_msolve_20261018/{PROTOCOL.md,run.sh}`: the msolve companion.


## Status 2026-10-18 13:35Z

Run 20261018T1134Z is complete (`reports/run-20261018T1134Z/`, records in
`runs/`): A1 refuted at degree 4 on both unsatisfiable draws with a clean
control, C1 capped by memory at 169 GB, A2/B/C2 deferred. Reading in
`RESULTS.md`. The PKM msolve rerun is reported in
`../pkm_m3_n21_msolve_20261018/`.
