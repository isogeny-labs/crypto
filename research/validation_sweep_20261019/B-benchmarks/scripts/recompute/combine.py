#!/usr/bin/env python3
# Runs each row script (python3 -I) and writes the SWB-RECOMPUTE-02 JSONL, one object per row.
import json, subprocess, sys

R = '/home/user/crypto'
SD = R + '/research/validation_sweep_20261019/B-benchmarks/scripts/recompute'
OUT = R + '/research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-02.jsonl'
rows = []
for n in range(9, 17):
    res = subprocess.run([sys.executable, '-I', f'{SD}/{n}.py'], capture_output=True, text=True, cwd=R)
    if res.returncode != 0:
        raise SystemExit(f'row {n} failed: {res.stderr[-800:]}')
    rows.append(json.loads(res.stdout.strip().splitlines()[-1]))
with open(OUT, 'w', encoding='utf-8') as f:
    for r in rows:
        f.write(json.dumps(r, ensure_ascii=False) + '\n')
for r in rows:
    c = {}
    for cl in r['claims']:
        c[cl['status']] = c.get(cl['status'], 0) + 1
    print(r['row_no'], ' '.join(f'{k}={v}' for k, v in sorted(c.items())), 'identical=' + r['identical']['status'])
print('wrote', OUT)
