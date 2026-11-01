#!/usr/bin/env python3
"""S1 sweep driver: applies the l rule of the protocol (section 3) for one (cell, config) with ecbench sessions on this branch's build.
usage: sweep.py <cell> <cfg: A|B|C|D|lab> <lane> [lmax]
Known values come from every finished T1_* session of the same cell (spec.json: curve a,n and target_seed) holding 8/8 verified records of that arm; an arm whose
records are all present but not all verified counts as failed (dearer).  Guard l (3, 4; WDSat only) are first run as one-target trials (timeout 3600 s); a trial whose
whole run is >= 3x the best 8-target median seen is 'trial-excluded' (dearer, never a best point); otherwise the full 8-target session is run."""
import json, os, sys, glob, time, subprocess, statistics as st
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import make_specs as M
from s1lib import ir, ECB
B = '/Volumes/SSD990-2/s1-build/s1b-20261101'; INF = float('inf')
PFX = os.environ.get('PFX', 'T1'); RUNSESS = 'runsess2.sh' if PFX == 'T2' else 'runsess.sh'
LMAX_BUILT = 15   # l >= 16 binaries exceed the 4 GB static-allocation limit and do not load
def key_of(runp):
    s = json.load(open(runp + '/spec.json')); c = s['workloads']['curves'][0]
    return (c['a'], c['n'], s['workloads']['target_seed'])
def finished(name):
    lg = f'{B}/runs/{PFX}_{name}.log'
    return os.path.exists(lg) and any(l.startswith('exit ') for l in open(lg))
def finished2(pf, name):
    lg = (f'/Volumes/SSD990-2/s1-build/runs/{pf}_{name}.log' if pf == 'S1' else f'{B}/runs/{pf}_{name}.log')
    return os.path.exists(lg) and any(l.startswith('exit ') for l in open(lg))
def scan(cell, cfg):
    a, n = M.CELLS[cell]; seed = M.SEED.get(cell, 202610111901); full = {}; trial = {}
    for d in sorted(glob.glob(f'{B}/runs/T?_*') + glob.glob('/Volumes/SSD990-2/s1-build/runs/S1_*')):
        if d.endswith('_tm') or d.endswith('.log') or not os.path.isdir(d): continue
        name = os.path.basename(d)[3:]; pf = os.path.basename(d)[:2]
        if pf == 'S1' and ('_AA' in name or name.startswith('T')): continue
        if not os.path.exists(d + '/records.jsonl') or not finished2(pf, name): continue
        try:
            if key_of(d) != (a, n, seed): continue
        except Exception: continue
        recs = [json.loads(l) for l in open(d + '/records.jsonl')]
        tms = sorted(glob.glob(d + '_tm/*.time'))
        if len(tms) != len(recs): continue
        by = {}
        for i, r in enumerate(recs):
            orc = r['method']['params']['oracle']; l = int(r['method']['params']['factor_base'].split('=')[1])
            if 'inherited-f4' in orc: arm = ('lab', l); extra = 0
            else:
                kv = dict(x.split('=') for x in orc.split(',')[1:] if '=' in x)
                arm = ({(1, 1): 'A', (0, 0): 'B', (1, 0): 'C', (0, 1): 'D'}[(int(kv['xg']), int(kv['sym']))], l)
                extra = (r.get('solver') or {}).get('extra', {}).get('wdsat_ir', 0)
            ok = r['outcome']['status'] == 'verified'
            by.setdefault(arm, []).append((ok, ir(tms[i]) + extra - ECB))
        for arm, v in by.items():
            if len(recs) and len(v) == len(recs) // len(set(r['method']['params']['factor_base'] + r['method']['params']['oracle'] for r in recs)) or True:
                pass
            if len(v) == 8 and all(o for o, _ in v): full[arm] = st.median(w for _, w in v)
            elif len(v) == 8: full[arm] = INF
            elif len(v) == 1: trial[arm] = v[0][1] if v[0][0] else INF
    return full, trial
def needs(curve, l0, guard, lmin, lmax):
    if l0 not in curve: return [l0]
    best = min(curve.values()); need = []
    for d in (-1, 1):
        l = l0 + d; worse = 0
        while lmin <= l <= lmax:
            if l not in curve: need.append(l); break
            worse = worse + 1 if curve[l] > best else 0
            if worse >= 2: break
            l += d
    for g in guard:
        if lmin <= g <= lmax and g not in curve: need.append(g)
    return sorted(set(need))
def run(name, spec):
    M.dump(f'{B}/specs', name, spec)
    subprocess.run(['bash', '-c', f'cd {B}; LANE={LANE} TMPDIR={B}/tmp ./{RUNSESS} {name}'], check=False)
if __name__ == '__main__':
    cell, cfg, LANE = sys.argv[1], sys.argv[2], sys.argv[3]
    a, n = M.CELLS[cell]; lmax = min(n - 1, LMAX_BUILT) if len(sys.argv) <= 4 else int(sys.argv[4])
    lab = cfg == 'lab'; l0 = (n - 1) // 2 if lab else n - 4; guard = () if lab else (3, 4)
    it = 0
    while True:
        full, trial = scan(cell, cfg)
        curve = {l: v for (c, l), v in full.items() if c == cfg}
        # trial-excluded guard l
        best = min(curve.values()) if curve else INF
        for (c, l), v in trial.items():
            if c == cfg and l not in curve and best < INF and v >= 3 * best: curve[l] = INF
        # in-flight sessions for this cell: wait
        inflight = [d for d in glob.glob(f'{B}/runs/T?_*.log') if not finished2(os.path.basename(d)[:2], os.path.basename(d)[3:-4])]
        mine = []
        for lg in inflight:
            nm = os.path.basename(lg)[3:-4]
            try:
                if key_of(f'{B}/runs/{os.path.basename(lg)[:2]}_{nm}') == (a, n, M.SEED.get(cell, 202610111901)) and (not nm.startswith('DR') or nm.startswith(('DR_' + cfg + '_', 'DRT_' + cfg + '_'))): mine.append(nm)
            except Exception: pass
        if mine: time.sleep(30); continue
        need = needs(curve, l0, guard, 3, lmax)
        print(time.strftime('%T'), cell, cfg, 'curve', {l: f'{v:.3g}' for l, v in sorted(curve.items())}, 'need', need, flush=True)
        if not need: break
        it += 1
        tr = [l for l in need if (l in guard or os.environ.get('TRIAL_ALL')) and (cfg, l) not in trial and l not in curve]
        full_l = [l for l in need if l not in tr]
        if tr:
            wl = [(cfg, l) for l in tr] if not lab else []
            run(f'DRT_{cfg}_{cell}_{it}', M.spec(f'S1 trial {cell} {cfg}', cell, wl, [], targets=1, timeout=3600))
            continue
        wl = [] if lab else [(cfg, l) for l in full_l]; ll = full_l if lab else []
        # a guard l with a surviving trial goes to the full list
        run(f'DR_{cfg}_{cell}_{it}', M.spec(f'S1 sweep {cell} {cfg} it{it}', cell, wl, ll, timeout=14400))
    full, _ = scan(cell, cfg); curve = {l: v for (c, l), v in full.items() if c == cfg}
    print('DONE', cell, cfg, 'best l', min(curve, key=curve.get), f'{min(curve.values()):.3g}', flush=True)
