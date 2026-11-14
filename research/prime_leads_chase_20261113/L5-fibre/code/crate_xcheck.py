"""Generate tensor-structured requests at tiny primes, run the crate binary, and compare
its counts against independent Python counts (direct point enumeration on E, and via the
Velu 2-isogeny on E'). Usage: crate_xcheck.py BIN OUTDIR bits... """
import json, os, random, subprocess, sys, time, itertools
sys.path.insert(0, os.path.dirname(__file__))
from ec import find_curve, fibre_points, crate_image_x, velu2, in_G, INF

BIN, OUT = sys.argv[1], sys.argv[2]
os.makedirs(OUT, exist_ok=True)
rows = []


def counts_direct(E, F, targets, m):
    cnt = {}
    for tup in itertools.product(F, repeat=m):
        s = INF
        for P in tup:
            s = E.add(s, P)
        cnt[s] = cnt.get(s, 0) + 1
    return [cnt.get(R, 0) for R in targets]


for spec in sys.argv[3:]:
    bits, m, mode = spec.split(':')
    bits, m = int(bits), int(m)
    C = None
    for seed in range(1000):
        try:
            C = find_curve(bits, seed * 7919 + bits, cofactors=(2, 4))
        except Exception:
            continue
        if C['p'] > 3:
            break
    E, p, r = C['E'], C['p'], C['r']
    rng = random.Random(bits * 31 + m)
    # image set: random x' values, keep those with nonempty fibre; at most ~ 3 images
    S, F = [], []
    cand = list(range(p))
    rng.shuffle(cand)
    for xp in cand:
        pts = fibre_points(C, xp)
        if pts:
            S.append(xp)
            F.extend(pts)
        if len(S) >= 2:
            break
    # independent membership check via crate formula on all of G
    G = [P for x in range(p) for P in E.lift(x) if in_G(E, r, P)]
    F2 = [P for P in G if P[0] != C['t'] and crate_image_x(E, C['t'], P[0]) in set(S)]
    assert sorted(F) == sorted(F2), (F, F2)
    targets = [INF] + rng.sample(G, min(3, len(G)))
    py = counts_direct(E, F, targets, m)
    # Velu side: base S' = phi(F) on E', target phi(R)
    E2, phi = velu2(C)
    Fp = [phi(P) for P in F]
    assert all(E2.on(Q) for Q in Fp)
    py2 = counts_direct(E2, Fp, [phi(R) for R in targets], m)
    req = {"schema": "structured-prime-ic-request/v1", "p": str(p), "a": str(C['a']), "b": str(C['b']),
           "r": str(r), "cofactor": str(C['h']), "generator": [str(C['g'][0]), str(C['g'][1])],
           "kernel_x": str(C['t']), "image_x": [str(s) for s in S], "arity": m,
           "targets": [None if R is INF else [str(R[0]), str(R[1])] for R in targets],
           "membership": mode, "fixture_seed": 1,
           "steps": 4_000_000_000_000, "vector": 200_000_000, "bits": 100_000_000}
    fn = os.path.join(OUT, f'req_{bits}_{m}_{mode}.json')
    json.dump(req, open(fn, 'w'))
    t0 = time.time()
    try:
        res = subprocess.run(['prlimit', '--as=3221225472', '--', 'timeout', '600', BIN, fn],
                             capture_output=True, text=True)
        wall = time.time() - t0
        if res.returncode != 0:
            rows.append(dict(bits=bits, p=p, r=r, h=C['h'], m=m, mode=mode, B=len(F), status='fail',
                             rc=res.returncode, err=res.stderr[-300:], wall=wall, py=py, py_velu=py2))
            print(rows[-1], flush=True)
            continue
        rec = json.loads(res.stdout)
        open(os.path.join(OUT, f'rec_{bits}_{m}_{mode}.json'), 'w').write(res.stdout)
        crate = [int(a['count']) for a in rec['semantic']['answers']]
        d = rec['diagnostics']
        rows.append(dict(bits=bits, p=p, r=r, h=C['h'], m=m, mode=mode, B=len(F), S=len(S), crate=crate, py=py,
                         py_velu=py2, match=(crate == py == py2), wall=round(wall, 2),
                         setup=d['setup_charged_steps'], query_steps=[q['charged_steps'] for q in d['query_diagnostics']],
                         edges=d['graph_edges'], backend=d['last_backend'],
                         msg_nodes=(d['membership_message'] or {}).get('nonterminal_nodes')))
    except Exception as e:  # noqa
        rows.append(dict(bits=bits, m=m, mode=mode, status='error', err=str(e)))
    print(rows[-1], flush=True)
json.dump(rows, open(os.path.join(OUT, 'xcheck.json'), 'w'), indent=1)
