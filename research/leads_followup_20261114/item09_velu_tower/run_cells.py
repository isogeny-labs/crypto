import random, sys, json, time, os
from tower_engine import *
from towers import *

ts = [int(x) for x in sys.argv[1].split(',')]
out = sys.argv[2]
dmax = int(sys.argv[3]) if len(sys.argv) > 3 else 6
kinds = sys.argv[4].split(',') if len(sys.argv) > 4 else ['velu', 'kummer']
insts = [int(x) for x in sys.argv[5].split(',')] if len(sys.argv) > 5 else [1, 2]
p = find_prime(); sq = sq_table(p)
def log(s): print(time.strftime('%H:%M:%S'), s, flush=True)
for t in ts:
    for i in insts:
        rng = random.Random(1000 * t + 10 * i)
        E = build_tower(p, t, rng, sq)
        xi, tau, c = velu_data(p, t, E); V = base_V(p, t, E)
        check_tower(p, t, E, xi, tau, c, V)
        inst = unit_instance(p, t, V, rng, sq)
        rec = dict(p=p, t=t, i=i, aux=dict(a0=E['a0'], b0=E['b0'], N=E['N'], Q=E['Q'], R=E['R'], aux_curves_tried=E['aux_curves_tried']),
                   xi=xi, tau=tau, c=c, inst=inst)
        log(f'instance t={t} i={i}: aux a0={E["a0"]} b0={E["b0"]} #E\'={E["N"]} tried={E["aux_curves_tried"]}; E a={inst["a"]} b={inst["b"]} target={inst["target"]} nonunit_rejected={inst["rejected_nonunit"]}')
        for kind in kinds:
            rules = Rules('velu', t, p, xi, tau, c) if kind == 'velu' else Rules('kummer', t, p)
            lfd = None
            for D in range(3, dmax + 1):
                t0 = time.time()
                r = run_cell(rules, p, inst['a'], inst['b'], inst['target'], D, early=True, verify=True, log=log if t >= 7 else None)
                r.update(kind=kind, instance=i)
                log(json.dumps(r))
                rec.setdefault('cells', []).append(r)
                with open(out, 'a') as f:
                    f.write(json.dumps(dict(r, p=p, a=inst['a'], b=inst['b'], target=inst['target'])) + '\n')
                if r['one_in_V']:
                    lfd = D; break
            log(f'RESULT t={t} i={i} {kind} LFD={lfd}')
            with open(out, 'a') as f:
                f.write(json.dumps(dict(summary=True, t=t, i=i, kind=kind, LFD=lfd, p=p)) + '\n')
        json.dump(rec, open(f'instance_t{t}_i{i}.json', 'w'))
