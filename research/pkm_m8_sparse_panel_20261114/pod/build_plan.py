#!/usr/bin/env python3
"""Build pod/plan.json (pkm.m8-panel.plan/v2) in the frozen order of PROTOCOL.md v2.2 §3, and stage the
pod bundle (public files only; hidden.json is never copied).  [pcc:unverified]

usage: python3 pod/build_plan.py [--bundle-dir pod/bundle] [--tgz pod/bundle.tgz]
"""
import argparse, hashlib, json, os, shutil, subprocess, sys, tarfile
HERE = os.path.dirname(os.path.abspath(__file__)); STUDY = os.path.dirname(HERE)
REPO = subprocess.run(['git', 'rev-parse', '--show-toplevel'], capture_output=True, text=True, cwd=STUDY).stdout.strip()
T2 = ['t2r9', 't2r10', 't2r11']; T3 = ['t3r8', 't3r7', 't3r9']; PROBE = 't4r7'
GB60 = 60 * 1024 * 1024; GB30 = 30 * 1024 * 1024
MSOLVE_SHA = 'c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa'

def targets(cell, sub=''):
    t = json.load(open(os.path.join(STUDY, 'cells', cell, sub, 'targets.json')))['targets']
    nat = sorted([x for x in t if x['label'] in ('sat', 'unsat')], key=lambda x: x['draw_index'])
    sats = [x['name'] for x in nat if x['label'] == 'sat']; uns = [x['name'] for x in nat if x['label'] == 'unsat']
    planted = [x['name'] for x in t if x['label'] == 'planted']
    return [x['name'] for x in nat], sats, uns, planted

rows = []
def add(kind, cell, target, solver='msolve', mode='param', threads=8, timeout=7200, cap=GB60, verify=None, **kw):
    r = dict(order=len(rows) + 1, kind=kind, cell=cell, target=target, solver=solver, threads=threads,
             timeout_s=timeout, rss_cap_kb=cap)
    if solver == 'msolve': r['msolve_mode'] = mode
    r['verify'] = (solver == 'msolve' and mode == 'param' and kind in ('aa', 'planted', 'natural', 'probe')) if verify is None else verify
    r.update(kw); rows.append(r)

def canary():
    add('canary', 'ctrl', 'l1canary', mode='gb', threads=8, timeout=120, cap=GB60, system='systems/canary/l1_t3r8.ms')

# ---- t = 2 block -------------------------------------------------------------------------------
add('timer_floor', 'ctrl', 'trivial', mode='gb', threads=1, timeout=60, cap=GB60, system='systems/trivial.ms')
_, s9, u9, _ = targets('t2r9'); AA = s9[0]
add('aa', 't2r9', AA)
for c in T2:
    nat, sats, uns, planted = targets(c)
    add('planted', c, planted[0])
    for n in nat: add('natural', c, n)
    if c == 't2r9':  # m = 8 cross-check pair (second instrument), the only m = 8 gb/native rows (§9a item 8)
        for n in (sats[0], uns[0]):
            add('gb_check', c, n, mode='gb')
            add('native', c, n, solver='native', threads=1, timeout=600, cap=GB30, native_max_degree=12, native_deadline_s=570,
                system_native=f'systems/{c}/{n}.system.json')
    for m in (4, 5, 6):  # cheap equality gates through the same exporter path
        natm, satm, unm, _ = targets(c, f'm{m}')
        for n in (satm[0], unm[0]):
            add('gb_check', f'{c}-m{m}', n, mode='gb', timeout=600, system=f'systems/{c}/m{m}/{n}.ms',
                cell_dir=f'cells/{c}', targets_dir=f'cells/{c}/m{m}')
            add('native', f'{c}-m{m}', n, solver='native', threads=1, timeout=600, cap=GB30, native_max_degree=12,
                native_deadline_s=570, system=f'systems/{c}/m{m}/{n}.ms', system_native=f'systems/{c}/m{m}/{n}.system.json',
                cell_dir=f'cells/{c}', targets_dir=f'cells/{c}/m{m}')
add('aa', 't2r9', AA)
# ---- t = 3 block -------------------------------------------------------------------------------
T3T = {c: targets(c) for c in T3}
def t3(kind, c, n, timeout=21600):
    canary(); add(kind, c, n, timeout=timeout, cap='cgroup0.9')
for k in range(4):
    t3('natural', 't3r8', T3T['t3r8'][2][k]); t3('natural', 't3r7', T3T['t3r7'][1][k]); t3('natural', 't3r9', T3T['t3r9'][2][k])
    t3('natural', 't3r8', T3T['t3r8'][1][k]); t3('natural', 't3r7', T3T['t3r7'][2][k]); t3('natural', 't3r9', T3T['t3r9'][1][k])
for c in T3: t3('planted', c, T3T[c][3][0], timeout=3600)
_, sp, _, _ = targets(PROBE); canary(); add('probe', PROBE, sp[0], timeout=7200, cap='cgroup0.9')

plan = {'schema': 'pkm.m8-panel.plan/v2', 'plan_id': 'panelA-20261114-v2.2', 'protocol': 'PKM-M8-PANELA-20261114 v2.2',
        'instrument': 'bin/pkm_m8_panel', 'msolve': {'version': '0.6.5-1build2', 'sha256': MSOLVE_SHA},
        'defaults': {'threads': 8, 'timeout_s': 21600, 'rss_cap_kb': 'cgroup0.9'}, 'master_seed': 20261114,
        'order_note': 't2 block: timer floor, A/A t2r9 SAT1, per cell planted then natural rows in draw order, cross-checks; A/A again; '
                      't3: canary before every call, interleaved t3r8-UNSAT1,t3r7-SAT1,t3r9-UNSAT1,t3r8-SAT1,t3r7-UNSAT1,t3r9-SAT1 then round-robin; '
                      't3 planted; probe last (analysed only if a t=3 natural row completed)',
        'rows': rows}

ap = argparse.ArgumentParser(); ap.add_argument('--bundle-dir', default=os.path.join(HERE, 'bundle')); ap.add_argument('--tgz', default=os.path.join(HERE, 'bundle.tgz'))
a = ap.parse_args()
json.dump(plan, open(os.path.join(HERE, 'plan.json'), 'w'), indent=1); open(os.path.join(HERE, 'plan.json'), 'a').write('\n')
# ---- bundle ------------------------------------------------------------------------------------
bd = a.bundle_dir; shutil.rmtree(bd, ignore_errors=True); os.makedirs(bd)
def cp(src, dst):
    assert os.path.basename(src).lower() != 'hidden.json'
    os.makedirs(os.path.dirname(os.path.join(bd, dst)), exist_ok=True); shutil.copy2(src, os.path.join(bd, dst))
shutil.copy2(os.path.join(HERE, 'plan.json'), os.path.join(bd, 'plan.json'))
cp(os.path.join(REPO, 'target/release/examples/pkm_m8_panel'), 'bin/pkm_m8_panel')
cp(os.path.join(HERE, 'parse_msolve_log.py'), 'tools/parse_msolve_log.py')
cp(os.path.join(STUDY, 'smoke/l1-export/l1/l1_t3r8.ms'), 'systems/canary/l1_t3r8.ms')
os.makedirs(os.path.join(bd, 'systems'), exist_ok=True); open(os.path.join(bd, 'systems/trivial.ms'), 'w').write('x\n10007\nx-1\n')
for c in T2 + T3 + [PROBE]:
    for sub in [''] + ([f'm{m}' for m in (4, 5, 6)] if c in T2 else []):
        d = os.path.join(STUDY, 'cells', c, sub)
        for f in sorted(os.listdir(d)):
            if f.lower() == 'hidden.json' or f == 'gen.log' or os.path.isdir(os.path.join(d, f)): continue
            if f.endswith('.ms') or f.endswith('.system.json'): cp(os.path.join(d, f), os.path.join('systems', c, sub, f))
            else: cp(os.path.join(d, f), os.path.join('cells', c, sub, f))
for root, _, files in os.walk(bd):
    assert not any(f.lower() == 'hidden.json' for f in files), root
with tarfile.open(a.tgz, 'w:gz') as t:
    for root, _, files in sorted(os.walk(bd)):
        for f in sorted(files):
            p = os.path.join(root, f); t.add(p, arcname=os.path.relpath(p, bd))
h = hashlib.sha256(open(a.tgz, 'rb').read()).hexdigest()
print(f'rows={len(rows)} plan={os.path.relpath(os.path.join(HERE, "plan.json"), REPO)} bundle={os.path.relpath(a.tgz, REPO)} sha256={h} bytes={os.path.getsize(a.tgz)}')
from collections import Counter; print(Counter(r['kind'] for r in rows))
