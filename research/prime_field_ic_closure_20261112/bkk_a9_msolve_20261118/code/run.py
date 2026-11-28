"""Run msolve on every presentation arm of the A9 comparison.  [pcc:unverified]
usage: run.py --m 3 --bits 12 14 --n 3 [--planted] [--workers 2] [--timeout 900] [--out runs/x.jsonl]
"""
import argparse, json, os, re, subprocess, sys, time, hashlib
from concurrent.futures import ThreadPoolExecutor
sys.path.insert(0, '/dev/shm/bkk/code')
from bkklib import *

ROOT = '/dev/shm/bkk/runs'
ROUND = re.compile(r'^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s')
FINAL = re.compile(r'reduce final basis\s+(\d+) x (\d+)')

def parse_log(txt):
    rounds = [tuple(int(g) for g in m.groups()) for m in map(ROUND.match, txt.splitlines()) if m]
    fin = FINAL.search(txt)
    degs = [r[0] for r in rounds]
    mats = [r[3] * r[4] for r in rounds]
    return dict(n_rounds=len(rounds), Dmax=max(degs) if degs else None,
                Msum=sum(mats), Mmax=max(mats) if mats else 0,
                rows_max=max((r[3] for r in rounds), default=0), cols_max=max((r[4] for r in rounds), default=0),
                final=[int(fin.group(1)), int(fin.group(2))] if fin else None,
                rounds=[[r[0], r[3], r[4]] for r in rounds])

def run_one(inst, arm, timeout, mem_kb, la=1):
    m = inst['m']
    eqs, order = semaev_part(arm, m, inst['p'], inst['a'], inst['b'], inst['xR'])
    txt = msolve_text(eqs, inst['roots'], m, order, inst['p'])
    tag = f"m{m}_b{inst['bits']}_i{inst['idx']}_{'P' if inst['planted'] else 'R'}_{arm}"
    os.makedirs(f'{ROOT}/in', exist_ok=True); os.makedirs(f'{ROOT}/log', exist_ok=True)
    fin, fout, flog = f'{ROOT}/in/{tag}.ms', f'{ROOT}/in/{tag}.gb', f'{ROOT}/log/{tag}.log'
    open(fin, 'w').write(txt)
    cmd = f'ulimit -v {mem_kb}; exec timeout {timeout} msolve -v 2 -l {la} -g 2 -t 1 -f {fin} -o {fout}'
    t0 = time.perf_counter()
    pr = subprocess.run(['bash', '-c', cmd], stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    T = time.perf_counter() - t0
    log = pr.stdout.decode(errors='replace')
    open(flog, 'w').write(log)
    rec = dict(la=la, tag=tag, m=m, bits=inst['bits'], idx=inst['idx'], planted=inst['planted'], arm=arm,
               p=inst['p'], L=inst['L'], nvars=len(order), order=order, rc=pr.returncode, T=T,
               input_sha256=hashlib.sha256(txt.encode()).hexdigest())
    rec['status'] = 'ok' if pr.returncode == 0 else ('timeout' if pr.returncode == 124 else 'fail')
    rec.update(parse_log(log))
    gb = open(fout).read() if os.path.exists(fout) else ''
    body = re.sub(r'\s', '', ''.join(l for l in gb.splitlines() if not l.startswith('#')))
    rec['gb_is_one'] = body.strip('[]:,') == '1'
    rec['gb_nelems'] = body.count(',') + 1 if body else 0
    rec['gb_len'] = len(gb)
    rec['gb_body_head'] = body[:120]
    if inst['planted'] and os.path.exists(fout):
        os.makedirs(f'{ROOT}/gb', exist_ok=True); os.replace(fout, f'{ROOT}/gb/{tag}.gb')
    if os.path.exists(fout): os.remove(fout)
    os.remove(fin)
    return rec

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--m', type=int, required=True)
    ap.add_argument('--bits', type=int, nargs='+', required=True)
    ap.add_argument('--n', type=int, default=3)
    ap.add_argument('--planted', action='store_true')
    ap.add_argument('--arms', nargs='*')
    ap.add_argument('--workers', type=int, default=2)
    ap.add_argument('--timeout', type=int, default=900)
    ap.add_argument('--mem_kb', type=int, default=6_000_000)
    ap.add_argument('--out', required=True)
    ap.add_argument('--la', type=int, default=1)
    a = ap.parse_args()
    arms = a.arms or ARMS[a.m]
    dead = {}  # arm -> True once it censored on >=2 instances of a cell
    with open(a.out, 'a') as fo:
        for bits in a.bits:
            insts = [make_instance(a.m, bits, i, a.planted) for i in range(a.n)]
            jobs = []
            for inst in insts:
                k = inst['idx'] % len(arms)
                for arm in arms[k:] + arms[:k]:
                    if not dead.get(arm):
                        jobs.append((inst, arm))
            with ThreadPoolExecutor(a.workers) as ex:
                recs = list(ex.map(lambda j: run_one(j[0], j[1], a.timeout, a.mem_kb, a.la), jobs))
            for r in recs:
                r.pop('rounds_full', None)
                fo.write(json.dumps(r) + '\n'); fo.flush()
                print(r['tag'], r['status'], f"T={r['T']:.2f}", 'Dmax', r['Dmax'], 'Msum', r['Msum'], 'one', r['gb_is_one'], flush=True)
            for arm in arms:
                bad = sum(1 for r in recs if r['arm'] == arm and r['status'] != 'ok')
                if bad >= 2:
                    dead[arm] = True
            for arm in arms:
                if dead.get(arm):
                    print('censored from next bits on:', arm, flush=True)

if __name__ == '__main__':
    main()
