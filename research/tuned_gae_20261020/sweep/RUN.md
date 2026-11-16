# gae-reference sweep run record

Commands run, in order (working directory /home/user/crypto):

1. mkdir -p research/tuned_gae_20261020/sweep
   exit code: 0
2. ./target/release/gae-reference --from-calibration > research/tuned_gae_20261020/sweep/gae_reference.jsonl 2> research/tuned_gae_20261020/sweep/gae_reference.stderr
   exit code: 0 (recorded in research/tuned_gae_20261020/sweep/exit_code.txt)
3. wc -l research/tuned_gae_20261020/sweep/gae_reference.jsonl
   tail -20 research/tuned_gae_20261020/sweep/gae_reference.stderr
   grep "model name" /proc/cpuinfo | head -1
4. python3 -c "import json;[print(d['curve'],d['family'],d['field_bits'],round(d['ns_per_add_reference'],1),round(d['ns_per_add_tuned'],1),round(d['ratio_reference_over_tuned'],2),d['verified_additions']) for d in map(json.loads,open('research/tuned_gae_20261020/sweep/gae_reference.jsonl'))]"

Timestamps (UTC, date -u):
- before step 1: Tue Oct 20 16:31:29 UTC 2026
- after step 2 completed: Tue Oct 20 16:32:15 UTC 2026
- after step 3-4 completed: Tue Oct 20 16:32:19 UTC 2026

Host line: vm (Linux vm 6.18.44-fc-v139 #1 SMP PREEMPT_DYNAMIC @0 x86_64 x86_64 x86_64 GNU/Linux)
CPU: Intel(R) Xeon(R) Processor @ 2.80GHz

## Field speedups, tables and Table B re-pricing (same host)

5. ./target/release/gae-reference field --from-calibration > research/tuned_gae_20261020/sweep/field.jsonl 2> research/tuned_gae_20261020/sweep/field.stderr
   exit code: 0 (field_exit_code.txt); 33 lines; 44 s; started 2026-10-20 16:50 UTC
6. ./target/release/gae-reference retable
   writes docs/ic/calibration_tuned.json (25 rows) and src/cryptanalysis/ecbench/full_prices_tuned.json (8 rows)
7. ./target/release/gae-reference reprice-ledger
   writes research/tuned_gae_20261020/reprice/table_b.json; legacy S reproduced for every recipe at 1e-9
