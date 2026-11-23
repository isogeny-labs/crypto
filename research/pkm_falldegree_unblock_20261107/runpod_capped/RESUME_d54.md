# Resume note: Amendment 33 probe `pkm-x-t14-k0-c7-d54` (committed instead of the S3 checkpoint, owner decision 22)

Every number is `[pcc:unverified]`.

- **What runs.** One job, `pkm-x-t14-k0-c7-d54`, stage `cap:14:0:7:54000000000`, on a reserved RunPod CPU pod (cpu5m,
  32 vCPU, 256 GB, about US$2.08/h), launched by `runpod_capped/run.sh` under the frozen Amendment 33 of
  `research/prime_fall_degree_20261027/PREREGISTRATION.md` (owner decisions 20-27).
- **Where its state lives.** Receipts in `runpod_capped/receipts/pkm-x-t14-k0-c7-d54/` (`pod-state.json` holds the run
  and hard deadlines; `start.json` the runner start), results in `runpod_capped/results/pkm-x-t14-k0-c7-d54/collect-<k>/`
  once collected, the ledger row in `runpod_capped/ledger.tsv`. Pod tokens stay in the session's private `RPC_STATE`;
  a fresh session recovers them from the RunPod v2 record (`fetch.sh`).
- **How to resume from a fresh session.** Check out the branch, `export RPC_STATE=$(mktemp -d)`,
  `RPC_BIN=<dir with the four frozen binaries, entry_r2.sh>`, then from `runpod_capped/`: `./fetch.sh pkm-x-t14-k0-c7-d54`
  (check-in, every 15 minutes per owner decision 26; `./watch.sh` loops it). When it reports a collected tree: commit
  and push `results/` and `receipts/`, then `./fetch.sh pkm-x-t14-k0-c7-d54 --delete <pushed commit>`. Past the hard
  deadline `fetch.sh` and `reap.sh` delete the pod regardless.
- **Backstop.** An hourly scheduled task runs `reap.sh` in a fresh session (owner decision 25;
  `runpod_capped/INDEPENDENT_REAPER.md`). Remove it once the pod is confirmed gone.
- **Reading** (Amendment 33 section 1.1): refutes at degree 7 gives `d_F = 7`; degree 7 completed without refuting gives
  `d_F >= 8`; oversize at 54e9, a memory kill or the wall budget gives censored, `d_F >= 7`. Draw 1 needs a new owner
  decision.
- **Budget.** Per-job cap US$65 (worst case about US$64.06 at 30.8 h); study guard US$595, ceiling US$600 (decision 27).
