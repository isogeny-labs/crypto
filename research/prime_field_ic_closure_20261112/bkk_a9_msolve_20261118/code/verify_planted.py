"""Positive control: the planted decomposition satisfies every element of each arm's reduced GB."""
import sys, json, itertools, re
sys.path.insert(0, '/dev/shm/bkk/code')
from bkklib import *
AUX = {'D': {}, 'C3': {'u': [0, 1]}, 'C3u': {'u': [0, 1]}, 'C4': {'u1': [0, 1], 'u2': [0, 1, 2]},
       'B4': {'u': [0, 1], 'v': [2, 3]}, 'H4a': {'u': [0, 1, 2]}, 'H4b': {'u': [0, 1]}}
def sums(Ps, idx, p, a):
    out = set()
    for signs in itertools.product([1, -1], repeat=len(idx) - 1):
        S = Ps[idx[0]]
        for s, i in zip(signs, idx[1:]):
            Q = Ps[i] if s == 1 else (Ps[i][0], (-Ps[i][1]) % p)
            S = ec_add(S, Q, p, a)
        if S is not None: out.add(S[0])
    return sorted(out)
def gb_polys(path):
    txt = open(path).read()
    vs = re.search(r'#variable order:\s*(.*)', txt).group(1).replace(' ', '').split(',')
    body = ''.join(l for l in txt.splitlines() if not l.startswith('#'))
    body = body.strip().rstrip(':').strip('[]')
    return vs, [q for q in body.split(',') if q.strip()]
def ev(poly, asg, p):
    expr = poly.replace('^', '**')
    return eval(expr, {}, dict(asg)) % p
for line in open(sys.argv[1]):
    r = json.loads(line)
    inst = make_instance(r['m'], r['bits'], r['idx'], True)
    p, a = inst['p'], inst['a']; Ps = inst['planted_pts']
    vs, polys = gb_polys(f"/dev/shm/bkk/runs/gb/{r['tag']}.gb")
    base = {f'x{i+1}': Ps[i][0] for i in range(r['m'])}
    cands = {v: sums(Ps, idx, p, a) for v, idx in AUX[r['arm']].items()}
    ok = False
    for combo in itertools.product(*cands.values()):
        asg = dict(base); asg.update(dict(zip(cands.keys(), combo)))
        if all(ev(q, asg, p) == 0 for q in polys):
            ok = True; break
    print(r['tag'], 'GB elems', len(polys), 'planted solution satisfies GB:', ok)
