# Launch card: degree 5 on C1 (m = 3, (19, 7)) above 280 GB (closure amendment A3 §11.3)

**Protocol:** [`research/dreg_closure_m4l6_20261018/PROTOCOL.md`](../../dreg_closure_m4l6_20261018/PROTOCOL.md) §11.3, §4 row C (Q3).
**Question:** does the MutantXL closure refute at degree 5 on the committed m = 3, ℓ = 7 draws that did not refute at 4? The last attempt failed to allocate at 169 GB resident under a 184 GB cap (projection 217 GB).

| | |
|---|---|
| host | RunPod GPU pod **2 × NVIDIA L40** (`--min-ram-per-gpu 140`), used only for its ≈ 286 GB of host RAM; the GPUs stay idle. The closure is single-threaded. |
| price | **$1.64/h** (2 × $0.82, secure; GraphQL `gpuTypes.lowestPrice`, minMemory 143 GB per GPU, stock "Low", 2026-10-18 21:13Z). Fallback: 2 × RTX PRO 6000 Blackwell Server Edition (180 GB per GPU, 360 GB) at $4.98/h. A CPU pod tops out at 256 GB, below the 280 GB gate. |
| gate | `run.sh deg5` runs C1 only when the cgroup limit is ≥ 280 GB (`DEG5_MIN_LIMIT`). If `memoryInGb` in the launch output is below 280, terminate at once (cost ≈ $0.05). |
| stages | build ≈ 0.2 h · C1 draw 0 at degree 5, timeout 12 h: projected 2.5–8 h (*derived*: A1's 341k columns took 24 min; C1 has 760k; work exponent 2.2–2.3 gives ×6, so ≈ 2.4 h, and §8 projected ≥ 7 h) · serving window 3 h |
| expected hours | **5 h** to `C1_DONE` (range 2.7–12.2 h) |
| expected cost | **$8.20** (range $4.40–$20.00); lifetime cap 16 h = **$26.24 at most** |

## Launch

```sh
cd research/dreg_closure_m4l6_20261018/pod
python3 launch.py prices          # GPU rows: confirm L40 price and minMemory
python3 launch.py run --job c1 --name dreg-c1-deg5 --branch main \
  --gpu "NVIDIA L40" --gpu-count 2 --min-ram-per-gpu 140 --min-vcpu-per-gpu 8 \
  --disk 100 --max-seconds 57600 --env SERVE_SECONDS=10800
```

For draws 1–3 too (a later launch only, §11.3), add `--env DEG5_DRAWS=4 --max-seconds 172800`: up to 4 × 12 h, at most $78.

## Watch and fetch

`STATUS` shows `DEG5 C1 draws=1 TMO5=…` then `C1_DONE`. `closure/runs/queue.log` has `deg5-m3-19-7 d0: exit N`: 0 = finished, 134/137 = allocation failure (recorded, not evidence), 124 = timeout.

```sh
P=research/dreg_closure_m4l6_20261018/pod/launch.py
python3 $P fetch ID research/dreg_closure_m4l6_20261018/runs --path closure/runs/
python3 $P terminate ID
```

The new `runs/deg5-m3-19-7.d0.*` overwrite the capped attempt's files. Move those to `runs/attempt1/` first and keep them (§7: a capped run is recorded, not re-run silently).

## Tested before launch

`run.sh deg5` with `DEG5_MIN_LIMIT` refuses below the threshold. The local run on the 15 GB container logs `not attempted` and starts nothing. The binary and the step are the ones that ran C1 on the first host; only the threshold is new.
