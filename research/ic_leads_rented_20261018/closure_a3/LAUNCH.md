# Launch card: decisive A1 count at (m, ℓ) = (4, 6) (closure amendment A3)

**Protocol:** [`research/dreg_closure_m4l6_20261018/PROTOCOL.md`](../../dreg_closure_m4l6_20261018/PROTOCOL.md) §11 (A3), frozen 2026-10-18 before any draw ≥ 251 exists.
**Question:** does every unsatisfiable A1 draw (m = 4, n = 15, ℓ = 6, N = 54, S = −9) refute at degree 4? That is the strong form of "ℓ = 7". So far 2 of 2 have.
**Decision rule:** §11.4. Any saturated non-refuting draw refutes "ℓ = 7" in its strong form. `R + 2 ≥ 14` refuting draws with none saturated is decisive for it (share needing degree ≥ 5 below 15 % at R = 16).

| | |
|---|---|
| host | RunPod CPU `cpu3m`, 32 vCPU, 256 GB (instance `cpu3m-32-256`), secure cloud |
| price | **$1.76/h** (GraphQL `cpuFlavors.specifics`, queried 2026-10-18 21:13Z; re-check with `launch.py prices`) |
| stages | build ≈ 0.2 h · replay gate (32 draws) ≈ 0.05 h · 3,000 fresh draws in 32 shards 2.4–3.6 h · 17 closures (16 unsat + 1 control) 6 at once, 1.2–2.0 h · serving window 3 h |
| expected hours | **5.5 h** to `A3_DONE` (range 4.0–6.0), plus the 3 h serving window only if nobody fetches sooner |
| expected cost | **$9.70** (range $7.00–$10.60) to `A3_DONE`; with the full serving window $15; lifetime cap 9 h = **$15.84 at most** |
| memory | 6 closures × ≤ 39 GB address space (25.8 GB peak RSS each, *recorded*); generation needs < 1 GB per shard |

## Launch (from a checkout of `main` after this PR merges, with `RUNPOD_API_KEY` in the environment)

```sh
cd research/dreg_closure_m4l6_20261018/pod
python3 launch.py prices --instances cpu3m-32-256        # confirm the price
python3 launch.py run --job a3 --name dreg-a3-m4-15-6 --branch main \
  --flavors cpu3m --vcpu 32 --disk 100 --max-seconds 32400 --env SERVE_SECONDS=10800
```

The output line gives the pod id `ID` and `https://ID-8000.proxy.runpod.net/`.

## Watch and fetch

- `curl -s https://ID-8000.proxy.runpod.net/STATUS` shows `BOOT`, `BUILDING`, `GENERATING_A3`, `MEASURING_A3`, then `A3_DONE` or `GEN_A3_FAILED`.
- `GEN_A3_FAILED` with `REPLAY_MISMATCH` in `closure/runs/queue.log` means the gate stopped the run before any fresh draw existed. Terminate the pod and report it; it is not a result.
- After `A3_DONE`, from the repository root:

```sh
P=research/dreg_closure_m4l6_20261018/pod/launch.py
python3 $P fetch ID research/dreg_closure_m4l6_20261018/runs  --path closure/runs/
python3 $P fetch ID research/dreg_closure_m4l6_20261018/draws --path closure/draws/
for f in STATUS boot.log build.log host-quick.txt; do
  curl -sS https://ID-8000.proxy.runpod.net/$f -o research/dreg_closure_m4l6_20261018/runs/pod-ID-$f; done
python3 $P terminate ID
```

  Do not fetch the top level recursively: it links the whole clone (`src/`).
- Commit `draws/a3-m4-15-6.jsonl`, `draws/a3-sha256.txt`, `runs/a3-*`, `runs/queue.log`, `runs/host-*.txt`, then write RESULTS.md §A3 from `runs/a3-tally.tsv` by §11.4, with a run report (research-visuals).

## Tested before launch (authoring container, 4 vCPU)

- `dreg_ladder --shard` reproduces committed draws 0–7 and 40–43 (unsatisfiable draw 42 included) byte-for-byte apart from `secs`: `../tests/shard_replay_20261018/`.
- `run.sh gen-a3 / measure-a3 / tally-a3` end to end on a tiny m = 4 cell (n = 8, ℓ = 3): the merged shards equal the sequential run on draws 20–59; a corrupted committed file stops `gen-a3` with `REPLAY_MISMATCH` (rc 3) and no fresh draws; 3 unsatisfiable closures + 1 control ran 2 at once and tallied: `../tests/a3_steps_20261018/`.
- `pod/boot.sh a3` run locally (the clone replaced by a link to the checkout, rustup and the final `kill 1` stubbed; `A3_END=255`): build, host record, replay gate 4/4 identical, 4 fresh draws (all satisfiable), the control closure ended by the 15 GB host's memory (exit 137, recorded as an allocation failure, as on a real host), tally, `A3_DONE`: `../tests/boot_a3_20261018/`. The full clone itself was not tested here (the checkout is about 8 GB against 2.8 GB free); the first host's boot used the same `git clone`, now `--depth 1`.
