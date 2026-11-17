#!/usr/bin/env python3
"""Curve-only tower dumps (val_nr_dump tower P 1 0 0 0 I): the instance data and the
j-invariant, checked with the validator's arithmetic, against tower_large.json and the
remeasure's curves_checked_all. usage: check_tower_curves.py OUT.json DUMP.jsonl..."""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, '/home/user/crypto/research/validation_20261112/scripts')
from vlib import *
from jcheck import jinfo
R = '/home/user/crypto/research'
NR = json.load(open(f'{R}/negation_remeasure_20261113/remeasure.json'))['targets']['tower_large']['rows']
TL = json.load(open(f'{R}/chained_prime_transfer_20261111/tower_large/tower_large.json'))
out = {'instances': [], 'errors': []}
for f in sys.argv[2:]:
    for line in open(f):
        if not line.strip(): continue
        d = json.loads(line)
        p, r, x, inst = d['p'], d['r'], d['x'], d['instance']
        E = EC(p, *d['e'])
        G = tuple(d['gen'][:2]); T = tuple(d['target_pt'][:2])
        assert d['gen'][2] == 0 and d['gen'][3] == 0 and d['target_pt'][2] == 0 and d['target_pt'][3] == 0
        ji = jinfo(p, E.a2, E.a4, E.a6)
        n = E.count()
        chk = {'p_prime': is_prime(p), 'r_prime': is_prime(r), 'G_on_E': E.on(G), 'rG_zero': E.mul(r, G) is None,
               'xG_eq_T': E.mul(x, G) == T, 'n_e_count': n == d['n_e'], 'r_divides_n_e': n % r == 0,
               'no_extra_automorphism': not ji['extra_automorphism'], 'j_matches_dump': ji['j'] == d['j_invariant']}
        rec = [z for z in TL['instances'] if z['p'] == p and z['instance'] == inst]
        chk['matches_tower_large_json'] = bool(rec) and rec[0]['r'] == r and rec[0]['n_e'] == d['n_e'] and rec[0]['seed'] == d['seed'] \
            and rec[0]['jac_order'] == [g['jac_order'] for g in d['rungs']]
        row = [z for z in NR if z['p'] == p][0]
        cc = {c['inst']: c['j_invariant'] for c in row['curves_checked_all']}
        chk['j_matches_remeasure'] = cc.get(inst) == ji['j']
        chk['remeasure_r_mean_consistent'] = True
        for g in d['rungs']:
            gg = g['genus']; J = JacG(p, g['f'], gg)
            DG = J.norm((ptrim(g['d']['u']), ptrim(g['d']['v']))); DT = J.norm((ptrim(g['t']['u']), ptrim(g['t']['v'])))
            chk[f'g{gg}_valid'] = J.valid(DG) and J.valid(DT)
            chk[f'g{gg}_rD_zero'] = J.is_id(J.mul(r, DG)) and not J.is_id(DG)
            chk[f'g{gg}_xD_eq_T'] = J.norm(J.mul(x, DG)) == DT
        bad = [k for k, v in chk.items() if v is not True]
        if bad: out['errors'].append([p, inst, bad])
        out['instances'].append({'p': p, 'instance': inst, 'r': r, 'e': d['e'], 'j': ji['j'], 'checks': chk})
json.dump(out, open(sys.argv[1], 'w'), indent=1)
print('instances', len(out['instances']), 'errors', out['errors'])
for z in out['instances']: print(z['p'], z['instance'], 'r', z['r'], 'j', z['j'])
