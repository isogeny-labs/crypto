#!/usr/bin/env python3
"""Validator driver for certcheck: per (q, seed) instance, gather the recorded ell/order/k and every recovered log of every
ok record (producer's raw records and, if given, the validator's replay records), call the independent C checker, and
add two negative controls per instance (k_found + 1 and k_found ^ 2^20), which must FAIL the check.
usage: run_certs.py <certcheck binary> <out.jsonl> <dir> [<dir> ...]"""
import json, glob, os, subprocess, sys, collections
binp, outp, dirs = sys.argv[1], sys.argv[2], sys.argv[3:]
inst = collections.defaultdict(lambda: dict(ell=set(), order=set(), k=set(), logs=collections.Counter(), nbad=0, files=0))
for d in dirs:
    for f in sorted(glob.glob(os.path.join(d, 'ic_*_c*.jsonl')) + glob.glob(os.path.join(d, 'aa_*.jsonl'))):
        for l in open(f):
            if not l.startswith('{'): continue
            r = json.loads(l)
            key = (r['q'], r['seed']); I = inst[key]
            if r['rec'] == 'ratios': I['ell'].add(r['ell']); I['order'].add(r['order']); I['files'] += 1
            elif r['rec'] == 'ic':
                I['ell'].add(r['ell']); I['k'].add(r['k'])
                if r['ok']: I['logs'][r['k_found']] += 1
                else: I['nbad'] += 1
out = open(outp, 'w'); allok = True
for (q, seed) in sorted(inst):
    I = inst[(q, seed)]
    assert len(I['ell']) == 1 and len(I['order']) == 1 and len(I['k']) == 1, (q, seed, I)
    ell, order, k = next(iter(I['ell'])), next(iter(I['order'])), next(iter(I['k']))
    logs = list(I['logs'])
    ctl = [(logs[0] + 1) % (1 << 64), logs[0] ^ (1 << 20)] if logs else []
    inp = '\n'.join(str(x) for x in logs + ctl) + '\n'
    p = subprocess.run([binp, str(q), str(seed), str(ell), str(order), str(k), '2000'], input=inp, capture_output=True, text=True)
    r = json.loads(p.stdout.strip().splitlines()[-1]) if p.stdout.strip() else dict(matched=False, err=p.stderr[-300:])
    n_real = len(logs)
    r.update(distinct_ok_logs=n_real, ok_records=sum(I['logs'].values()), failed_attempt_records=I['nbad'], unit_files=I['files'],
             negative_controls_rejected=(r.get('logs_checked', 0) - r.get('logs_ok', 0)) == len(ctl) and r.get('logs_ok', -1) == n_real)
    good = (r.get('matched') and r['order_eq_4ell'] and r['ell_prime'] and r['G_on_curve_order_ell'] and r['order_kills_fresh_points']
            and r['k_matches_record'] and r['logs_ok'] == n_real and r['negative_controls_rejected'] and r['j_not_0_1728'] and not r['j_in_Fq'] and r['j_delta_nonzero'])
    r['pass'] = bool(good); allok &= bool(good)
    out.write(json.dumps(r) + '\n'); out.flush()
    print(q, seed, 'PASS' if good else 'FAIL', 'tries', r.get('tries_until_match'), 'logs', n_real, flush=True)
print('ALL PASS' if allok else 'SOME FAIL', len(inst), 'instances')
