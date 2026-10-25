#!/usr/bin/env python3
"""parse.py -> table of runs/ (PROTOCOL.md section 5 definitions). Prints markdown rows."""
import re, glob, os
print('| system | arm | stop | exit | cpu s | wall s | peak RSS GiB | rounds | max deg | max productive deg | deg>=8 printed | deg>=8 productive | .out |')
print('|---|---|---|--:|--:|--:|--:|--:|--:|--:|---|---|---|')
for mem in sorted(glob.glob('runs/*.mem')):
    name = os.path.basename(mem)[:-4]; sysn, arm = name.split('__')
    sysn = sysn.replace('pkm_kummer_p2013265921_', '')
    kv = dict(x.split('=') for x in open(mem).read().split()[1:])
    rows = []
    for l in open(f'runs/{name}.log', errors='replace'):
        m = re.match(r'\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x\s*(\d+)\s+[\d.]+%\s+(\d+) new', l)
        if m: rows.append((int(m.group(1)), int(m.group(6))))
    md = max((d for d, n in rows), default=0); mp = max((d for d, n in rows if n > 0), default=0)
    r8 = any(d >= 8 for d, n in rows); p8 = any(d >= 8 and n > 0 for d, n in rows)
    o = open(f'runs/{name}.out').read().strip()[:30].replace('\n', ' ') if os.path.exists(f'runs/{name}.out') else ''
    if kv['stop']=='exit' and int(kv['exit'])<0: kv['stop']='interrupted'
    peak = max(int(kv['ps_peak_rss_kB']), int(kv['ru_maxrss_kB'])) / 1048576
    print(f"| {sysn} | {arm} | {kv['stop']} | {kv['exit']} | {kv['cpu_s']} | {kv['wall_s']} | {peak:.2f} | {len(rows)} | {md} | {mp} | {'yes' if r8 else 'no'} | {'yes' if p8 else 'no'} | `{o}` |")
