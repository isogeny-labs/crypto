"""SAT against F4 on the Riemann–Roch decomposition systems over GF(2^m).

    python3 run_panel.py --out RUNDIR --f4-bin PATH [--m 7,9,11,13] [--ell-offset 0,1]
                         [--planted 4] [--unplanted 4] [--budget 600] [--jobs 3]
                         [--arms incidence,norm] [--solvers f4,sat] [--seed 20261115]

For every cell (m, ell) and draw: one random ell-dimensional subspace V of
GF(2^m) and one target R (planted: a sum of three distinct-abscissa points
over V; unplanted: 4*P for a uniform affine P, redrawn while x_R in V or
R = O).  The ground truth is counted by group arithmetic (ordered triples of
factor-base points summing to R, with and without repeated abscissas).  Both
arms (incidence, degree 2; norm, degree 3) are built from the same draw by
rrsys.buildSystem and written as JSON; the same file goes to bool_f4_file
(F4 over the Boolean ring, --solutions) and to sat_solve.py (CryptoMiniSat).
Every model or solution returned is decoded and verified independently by
rrsys.decodeSolution.  One JSON line per (draw, arm, solver) in results.jsonl.

Wall time is a practicality note (shared 4-vCPU container, RAYON_NUM_THREADS=1,
--jobs concurrent subprocesses); the counted figures (degrees, matrix sizes,
conflicts, solution counts) do not depend on the host.
"""

import argparse
import concurrent.futures
import json
import os
import random
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rrsys  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))


def drawInstance(f, curve, ell, rng, planted):
    span = None
    for _ in range(10000):
        basis, span = rrsys.randomSubspace(f, ell, rng)
        base = []
        for x in sorted(span - {0}):
            p = curve.pointFromX(x)
            if p is not None:
                base.append(p)
                base.append(curve.neg(p))
        if planted:
            xsDistinct = sorted({p[0] for p in base})
            if len(xsDistinct) < 3:
                continue
            xs = rng.sample(xsDistinct, 3)
            pts = []
            for x in xs:
                p = curve.pointFromX(x)
                pts.append(p if rng.random() < 0.5 else curve.neg(p))
            target = curve.add(curve.add(pts[0], pts[1]), pts[2])
        else:
            while True:
                x = rng.randrange(1, 1 << f.m)
                p = curve.pointFromX(x)
                if p is not None:
                    break
            target = curve.mul(p, 4)
        if target is None or target[0] in span:
            continue
        return basis, span, target
    raise RuntimeError('could not draw an instance')


def countTruth(curve, span, target):
    """Ordered factor-base triples summing to target; the base is every affine
    point with x in V, the 2-torsion point (0, 1) included since 0 is in V and
    both polynomial systems admit it (the Rust panel records both counts)."""
    pts = []
    for x in span:
        if x == 0:
            pts.append((0, 1))
            continue
        p = curve.pointFromX(x)
        if p is not None:
            pts.append(p)
            pts.append(curve.neg(p))
    have = set(pts)
    total = distinct = 0
    for p in pts:
        for q in pts:
            s = curve.add(p, q)
            r = curve.add(target, curve.neg(s))
            if r is not None and r in have:
                total += 1
                if len({p[0], q[0], r[0]}) == 3:
                    distinct += 1
    return total, distinct, len(pts)


def runF4(binPath, path, budget):
    env = dict(os.environ, RAYON_NUM_THREADS='1')
    t0 = time.time()
    proc = subprocess.run([binPath, path, str(budget), '--solutions'], capture_output=True,
                          text=True, env=env, timeout=budget * 3 + 60)
    out = {'wall_outer_s': time.time() - t0, 'returncode': proc.returncode}
    try:
        out.update(json.loads(proc.stdout.strip().splitlines()[-1]))
    except Exception:
        out['stderr'] = proc.stderr[-2000:]
    return out


def runSat(path, budget):
    t0 = time.time()
    proc = subprocess.run([sys.executable, '-I', os.path.join(HERE, 'sat_solve.py'), path, str(budget)],
                          capture_output=True, text=True, timeout=budget * 3 + 60)
    out = {'wall_outer_s': time.time() - t0, 'returncode': proc.returncode}
    try:
        out.update(json.loads(proc.stdout.strip().splitlines()[-1]))
    except Exception:
        out['stderr'] = proc.stderr[-2000:]
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--out', required=True)
    ap.add_argument('--f4-bin', required=True)
    ap.add_argument('--m', default='7,9,11,13')
    ap.add_argument('--ell-offset', default='0')
    ap.add_argument('--planted', type=int, default=4)
    ap.add_argument('--unplanted', type=int, default=4)
    ap.add_argument('--budget', type=float, default=600)
    ap.add_argument('--jobs', type=int, default=3)
    ap.add_argument('--arms', default='incidence,norm')
    ap.add_argument('--solvers', default='f4,sat')
    ap.add_argument('--seed', type=int, default=20261115)
    ap.add_argument('--max-vars', type=int, default=128)
    args = ap.parse_args()
    os.makedirs(os.path.join(args.out, 'cells'), exist_ok=True)
    arms = args.arms.split(',')
    solvers = args.solvers.split(',')
    resultsPath = os.path.join(args.out, 'results.jsonl')
    jobs = []
    for m in [int(v) for v in args.m.split(',')]:
        f = rrsys.Gf2m(m)
        curve = rrsys.Curve(f)
        order = curve.order()
        for off in [int(v) for v in args.ell_offset.split(',')]:
            ell = (m + 2) // 3 + off
            rng = random.Random('%d-%d-%d' % (args.seed, m, ell))
            for d in range(args.planted + args.unplanted):
                planted = d < args.planted
                basis, span, target = drawInstance(f, curve, ell, rng, planted)
                total, distinct, nbase = countTruth(curve, span, target)
                head = {'m': m, 'ell': ell, 'draw': d, 'planted': planted, 'order': order,
                        'irreducible': f.poly, 'basis': basis, 'target': list(target),
                        'truth_ordered_total': total, 'truth_ordered_distinct': distinct,
                        'factor_base_points': nbase}
                for arm in arms:
                    polys, nVars, layout = rrsys.buildSystem(arm, f, basis, target)
                    name = 'm%d_l%d_d%d_%s' % (m, ell, d, arm)
                    path = os.path.join(args.out, 'cells', name + '.json')
                    with open(path, 'w') as fh:
                        fh.write(rrsys.exportJson(polys, nVars))
                    info = dict(head, arm=arm, system=name, n_vars=nVars, n_polys=len(polys),
                                degree=max(rrsys.polyDegree(p) for p in polys),
                                terms=sum(len(p) for p in polys), layout=layout)
                    for solver in solvers:
                        if solver == 'f4' and nVars > args.max_vars:
                            continue
                        jobs.append((info, solver, path, f, curve, basis, target, layout, arm))
    print('%d jobs' % len(jobs), flush=True)

    def work(job):
        info, solver, path, f, curve, basis, target, layout, arm = job
        if solver == 'f4':
            res = runF4(args.f4_bin, path, args.budget)
            decided = (not res.get('timed_out', True)) and not res.get('oversize', False)
            sols = res.get('solutions') or []
            verified = 0
            for s in sols:
                asg = 0
                for i in s:
                    asg |= 1 << i
                dec = rrsys.decodeSolution(arm, f, basis, target, layout, asg, curve)
                if dec and dec['sum_ok']:
                    verified += 1
            res['verified_solutions'] = verified
            res['decided'] = decided
        else:
            res = runSat(path, args.budget)
            res['decided'] = res.get('status') in ('sat', 'unsat')
            if res.get('status') == 'sat':
                asg = int(res['model_hex'], 16)
                dec = rrsys.decodeSolution(arm, f, basis, target, layout, asg, curve)
                res['model_verified'] = bool(dec and dec['sum_ok'])
        line = dict(info, solver=solver, result=res)
        line.pop('layout', None)
        return line

    with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as ex, open(resultsPath, 'a') as out:
        for line in ex.map(work, jobs):
            out.write(json.dumps(line) + '\n')
            out.flush()
            r = line['result']
            print('%s %s decided=%s sd=%s dr=%s rows=%s status=%s confl=%s wall=%.1f' % (
                line['system'], line['solver'], r.get('decided'), r.get('solving_degree'),
                r.get('degree_reached'), r.get('matrix_rows_max'), r.get('status'),
                r.get('conflicts'), r.get('wall_outer_s', 0)), flush=True)


if __name__ == '__main__':
    main()
