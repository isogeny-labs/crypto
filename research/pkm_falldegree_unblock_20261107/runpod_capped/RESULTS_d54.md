# Amendment 33 probe `pkm-x-t14-k0-c7-d54`: outcome (2026-11-22)

Every number is `[pcc:unverified]`: it is read from the committed receipts and results tree
(`receipts/pkm-x-t14-k0-c7-d54/`, `results/pkm-x-t14-k0-c7-d54/collect-1/`); no validator has re-checked it.

## What ran

Stage `cap:14:0:7:54000000000` (t = 14, draw 0, cap 7, `--max-dense 54e9`) on RunPod pod `46duu2mnh5dyz6`
(EU-RO-1, cpu5m, 32 vCPU / 256 GB, cgroup v1 limit 256,000,000,000 bytes, host `vm.max_map_count` 65,530), frozen image
binaries (layer `a0f7e28e…`), `RAYON_NUM_THREADS = 12`. Runner start 2026-11-21T23:40:51Z, runner end 2026-11-22T06:29:29Z;
collected 06:34Z through two identical downloads; pod deleted through the git gate at 06:36:48Z (commit `49ef8fa0d6`).
Pod lifetime 24,993 s, about US$14.44 (derived: cost per hour x lifetime).

## Outcome under Amendment 33

**NOT_ADMITTED (rule P2e): no row, no reading, no relaunch; it goes to the owner.** Admission rules r3 and r4 fail: the
pilot exited 134 (abort) and wrote no result row (`runs/X-m2-t14-k0-c7.jsonl` is empty).

## Why the pilot aborted

- The run log ends after `tower step 33` (degree 6, B' 10,662,114,798 entries, memory 27,771 MB, peak 64,823 MB) with
  `memory allocation of 262144 bytes failed` and `memory allocation of 456768 bytes failed`, then the abort. It never
  reached step 35, the first degree-7 step, so the new `--max-dense` was never exercised.
- Memory was not exhausted: the container's peak was 68,165,406,720 bytes (63.5 GiB) of a 256,000,000,000-byte limit,
  `memory.failcnt` 0, `oom_kill` 0; the pilot's own peak (VmHWM) 67,971,842,048 bytes.
- The host allowed 65,530 memory mappings (`vm.max_map_count`, recorded by the boot loop). The binary is a static
  executable whose allocator maps each large block separately; step 33 kept 68,581 basis elements, so the mapping count
  is the binding limit. This is the case the fourth review flagged (record `vm.max_map_count`; a failed allocation reads
  NOT_ADMITTED).
- The registered `-m112` runs of 2026-11-16 (`results/SNAPSHOT_20261117/`) passed this point and reached step 35 on
  other hosts, so the limit depends on the host RunPod assigns.

## What it decides

Nothing about `d_F` at t = 14: `d_F >= 7` (Amendment 31) stands unchanged. The cost is recorded in `ledger.tsv`
(study sum US$517.63).

## Open

- A relaunch would need a pre-start check that refuses a host with a low `vm.max_map_count` (a procedure change, by a
  dated amendment and the owner).
- The bench record `benchmarks/records/20261121T2339Z-pkm-fd-rpc-d54.json` could not be closed: `tools/benchrec.py close`
  validates every record and refuses because another record on main
  (`benchmarks/records/20261117T0334Z-hec-multi-production-cold.json`) is malformed. The commit message of `9efe4a6c29`
  says "bench record closed invalid"; that is wrong, the record is still open.
