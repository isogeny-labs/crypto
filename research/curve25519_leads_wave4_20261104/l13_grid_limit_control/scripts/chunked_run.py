#!/usr/bin/env python3
"""Restart-safe chunked ecbench driver (Addendum C of T11 and T12).

The container restarts every ~40-60 min and ecbench cannot resume a session,
so a frozen spec is split into chunks of ONE target each (all arms, same
rounds/order/timeouts). Each chunk is a complete, audit-valid session.
A chunk whose session.json status is 'complete' is skipped; any other chunk
dir is moved to interrupted/ (session.json renamed session.interrupted.json)
and re-run from scratch. Chunk c uses target_seed = base_target_seed + c and
measurement seed = base_seed + c.
Usage: chunked_run.py <frozen spec.json> <out_dir> <n_chunks> <ecbench binary>
"""
import json, os, shutil, subprocess, sys, time
spec_path, out, n, ecb = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]
base = json.load(open(spec_path))
os.makedirs(os.path.join(out, 'specs'), exist_ok=True)
os.makedirs(os.path.join(out, 'interrupted'), exist_ok=True)
log = open(os.path.join(out, 'driver.log'), 'a')
def L(m): log.write(time.strftime('%Y-%m-%dT%H:%M:%SZ ', time.gmtime()) + m + '\n'); log.flush()
for c in range(n):
    d = os.path.join(out, f'c{c:02d}')
    sj = os.path.join(d, 'session.json')
    if os.path.exists(sj) and json.load(open(sj)).get('status') == 'complete':
        L(f'c{c:02d} complete, skip'); continue
    if os.path.exists(d):
        k = len(os.listdir(os.path.join(out, 'interrupted')))
        dst = os.path.join(out, 'interrupted', f'c{c:02d}_{k}')
        shutil.move(d, dst)
        if os.path.exists(os.path.join(dst, 'session.json')):
            os.rename(os.path.join(dst, 'session.json'), os.path.join(dst, 'session.interrupted.json'))
        L(f'c{c:02d} incomplete -> {dst}')
    s = json.loads(json.dumps(base))
    s['workloads']['targets_per_curve'] = 1
    s['workloads']['target_seed'] = base['workloads']['target_seed'] + c
    s['measurement']['seed'] = base['measurement']['seed'] + c
    s['description'] = base['description'] + f' [chunk {c} of {n}, Addendum C]'
    sp = os.path.join(out, 'specs', f'c{c:02d}.json'); json.dump(s, open(sp, 'w'), indent=1)
    L(f'c{c:02d} start')
    rc = subprocess.call([ecb, 'run', '--wait', '--allow-busy', '--spec', sp, '--out', d],
                         stdout=open(os.path.join(out, f'c{c:02d}.log'), 'w'), stderr=subprocess.STDOUT)
    st = json.load(open(sj)).get('status') if os.path.exists(sj) else None
    L(f'c{c:02d} end rc={rc} status={st}')
L('ALL CHUNKS DONE')
