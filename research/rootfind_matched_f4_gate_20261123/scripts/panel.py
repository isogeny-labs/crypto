#!/usr/bin/env python3
"""Panel generator, brute-force oracle and interleaved runner (PROTOCOL.md).

  panel.py gen <M> <seed...>            -> chart lines on stdout
  panel.py calibrate <outjson>          -> calibration ladder (not scored)
  panel.py panelA <outjson>             -> program B on the frozen NIST charts
  panel.py run <M> <outjsonl>           -> 3-repetition interleaved panel
Environment: PB (program_b binary), F4 (example binary), WORK (scratch dir).
"""
import json, os, random, subprocess, sys, time, hashlib, math
from pathlib import Path

P = 4294967291
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
PB = os.environ.get('PB')
F4 = os.environ.get('F4')
WORK = Path(os.environ.get('WORK', '/tmp'))
COPIES = 5


def chart(seed, M):
    rng = random.Random(f'rootfind_matched_f4_gate_20261123/{seed}/{M}')
    t0, t1, u0, u1 = (rng.randrange(P) for _ in range(4))
    if seed <= 10:  # planted
        x, y, z = (rng.randrange(M + 1) for _ in range(3))
        S, T, U = x + y + z, x*y + x*z + y*z, x*y*z
        t2 = (T - t0*S*S - t1*S) % P
        u2 = (U - u0*S*S - u1*S) % P
    else:
        t2, u2 = rng.randrange(P), rng.randrange(P)
    return dict(id=f's{seed}', p=P, M=M, t=[t0, t1, t2], u=[u0, u1, u2])


def brute(c):
    p, M, (t0, t1, t2), (u0, u1, u2) = c['p'], c['M'], c['t'], c['u']
    out = []
    for x in range(M + 1):
        for y in range(M + 1):
            for z in range(M + 1):
                S = x + y + z
                if (x*y + x*z + y*z - t0*S*S - t1*S - t2) % p == 0 and \
                   (x*y*z - u0*S*S - u1*S - u2) % p == 0:
                    out.append([x, y, z])
    return out


def line(c):
    return ' '.join(map(str, [c['id'], c['p'], c['M'], *c['t'], *c['u']]))


def timed(cmd, timeout):
    t = time.monotonic()
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout,
                           env=dict(os.environ, RAYON_NUM_THREADS='1'))
        return time.monotonic() - t, r.returncode, r.stdout, r.stderr[-400:], False
    except subprocess.TimeoutExpired:
        return time.monotonic() - t, None, '', '', True


def run_b(c, timeout=300):
    f = WORK / f'b_{c["id"]}_{c["M"]}.txt'
    f.write_text('\n'.join(line(c).replace(c['id'], 'chart', 1) for _ in range(COPIES)) + '\n')
    wall, rc, out, err, to = timed([PB, 'x', str(f)], timeout)
    rec = dict(arm='B', wall=wall, rc=rc, timeout=to)
    if rc == 0:
        d = json.loads(out)
        sets = [sorted([int(v) for v in s[1:]] for s in case['solutions']) for case in d['cases']]
        assert all(s == sets[0] for s in sets), 'B copies disagree'
        rec.update(roots=sets[0], complete=all(case['complete'] for case in d['cases']),
                   nodes=d['cases'][0]['nodes'], lll_steps=d['cases'][0]['LLL_steps'])
    else:
        rec.update(complete=False, err=err)
    return rec


def run_f4(c, mode, budget=120, copies=COPIES):
    f = WORK / f'f_{c["id"]}_{c["M"]}.txt'
    f.write_text(line(c) + '\n')
    wall, rc, out, err, to = timed([F4, mode, str(f), str(copies), str(budget)],
                                   copies * budget + 60)
    rec = dict(arm='F4-' + mode, wall=wall, rc=rc, timeout=to, copies=copies)
    if rc == 0:
        d = json.loads(out)
        rec.update(roots=[list(r) for r in d['roots']], complete=d['complete'],
                   solve_seconds=d['solve_seconds'], field_ops=d['field_ops'],
                   solving_degree=d['solving_degree'], max_rows=d['max_rows'],
                   max_cols=d['max_cols'], f4_runs=d['f4_runs'])
    else:
        rec.update(complete=False, err=err)
    return rec


def calibrate(out):
    rows = []
    alive = {'ideal': True, 'visit': True}
    for M in [4, 6, 8, 10, 12, 16, 24, 32, 48, 64, 96, 128]:
        for mode in ('ideal', 'visit'):
            if not alive[mode]:
                continue
            per = []
            for seed in (9001, 9002, 9003):
                c = chart(seed, M)
                c['id'] = f'cal{seed}'
                r = run_f4(c, mode, budget=120, copies=1)
                r.update(M=M, seed=seed)
                rows.append(r)
                print(json.dumps({k: r.get(k) for k in ('arm', 'M', 'seed', 'wall', 'complete', 'field_ops')}), flush=True)
                per.append(r['wall'] if r['complete'] else math.inf)
            if sorted(per)[1] == math.inf or sorted(per)[1] > 120:
                alive[mode] = False
        Path(out).write_text(json.dumps(rows))
        if not any(alive.values()):
            break
    Path(out).write_text(json.dumps(rows))


def panelA(out):
    src = ROOT / 'research/rootfinding_checkpoint_20261109/code/research/dual_ladder_20261109_v3/planted_inputs.json'
    data = json.loads(src.read_text())
    f = WORK / 'panelA.txt'
    lines = []
    for c in data['cases']:
        pub = c['public']
        lines.append(' '.join([pub['field'], pub['p'], pub['M'], *pub['T'], *pub['U']]))
    f.write_text('\n'.join(lines) + '\n')
    reps = []
    for r in range(3):
        wall, rc, outp, err, to = timed([PB, 'x', str(f)], 120)
        rec = dict(rep=r, wall=wall, rc=rc, timeout=to)
        if rc == 0:
            d = json.loads(outp)
            rec['cases'] = []
            for case, c in zip(d['cases'], data['cases']):
                got = sorted(sorted(int(v) for v in s[1:]) for s in case['solutions'])
                exp = sorted(int(v) for v in c['expected']['residual_roots'])
                rec['cases'].append(dict(field=case['field'], complete=case['complete'],
                                         n_ordered=len(case['solutions']),
                                         expected_multiset_found=all(g == exp for g in got) and len(got) > 0,
                                         nodes=case['nodes'], M_bits=int(c['public']['M']).bit_length(),
                                         p_bits=int(c['public']['p']).bit_length()))
        else:
            rec['err'] = err
        reps.append(rec)
        print(json.dumps(rec)[:400], flush=True)
    Path(out).write_text(json.dumps(dict(input_sha256=hashlib.sha256(src.read_bytes()).hexdigest(), reps=reps), indent=1))


def run(M, out, use_ideal):
    charts = [chart(s, M) for s in range(1, 21)]
    oracle = {c['id']: brute(c) for c in charts}
    fo = open(out, 'a')  # gzip -9 afterwards (64 KB data-file rule)
    for rep in (1, 2, 3):
        order = charts[:]
        random.Random(100 + rep).shuffle(order)
        for c in order:
            seq = [('B', None), ('F4', 'visit')] + ([('F4', 'ideal')] if use_ideal else []) + [('F4', 'visit'), ('B', None)]
            seen = {}
            for arm, mode in seq:
                r = run_b(c) if arm == 'B' else run_f4(c, mode)
                key = r['arm']
                r['aa'] = key in seen
                seen[key] = True
                r.update(rep=rep, id=c['id'], M=M, planted=int(c['id'][1:]) <= 10,
                         oracle_match=(r.get('roots') == oracle[c['id']]), n_roots=len(oracle[c['id']]))
                fo.write(json.dumps(r) + '\n')
                fo.flush()
                print(rep, c['id'], key, round(r['wall'], 3), r.get('complete'), r['oracle_match'], flush=True)


if __name__ == '__main__':
    cmd = sys.argv[1]
    if cmd == 'gen':
        for s in sys.argv[3:]:
            print(line(chart(int(s), int(sys.argv[2]))))
    elif cmd == 'calibrate':
        calibrate(sys.argv[2])
    elif cmd == 'panelA':
        panelA(sys.argv[2])
    elif cmd == 'run':
        run(int(sys.argv[2]), sys.argv[3], sys.argv[4] == 'ideal')
