#!/usr/bin/env python3
"""j-invariants of the v3 and cover E1 curves from the ORIGINAL committed data
(certs_v3.jsonl group coefficients; pushforward.json coeffs via the validator's
bielliptic model), compared with the j the remeasure recorded."""
import json, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, '/home/user/crypto/research/validation_20261112/scripts')
from jcheck import jinfo
from vlib import bielliptic, EC
R = '/home/user/crypto/research'
NR = json.load(open(f'{R}/negation_remeasure_20261113/remeasure.json'))['targets']
out = {'curves': [], 'errors': []}
certs = [json.loads(l) for l in open(f'{R}/chained_prime_transfer_20261111/certs_v3.jsonl') if l.strip()]
for row in NR['v3']['rows']:
    p = row['p']
    c = [c for c in certs if c['p'] == p and c['arm'] == 'rho_e1'][0]['group']
    ji = jinfo(p, c['a2'], c['a4'], c['a6'])
    rec = row['rho']['automorphism_fold']['j_invariant']
    out['curves'].append({'target': 'v3', 'p': p, 'coeffs': [c['a2'], c['a4'], c['a6']], **ji, 'remeasure_j': rec})
    if ji['j'] != rec: out['errors'].append(['v3 j differs', p, ji['j'], rec])
    if ji['extra_automorphism']: out['errors'].append(['v3 extra automorphism', p])
CP = json.load(open(f'{R}/cover_pushforward_20261112/pushforward.json'))
for row in NR['cover_pushforward']['rows']:
    p = row['p']
    o = [x for x in CP['rows'] if x['p'] == p][0]
    x0, g5, E1, E2 = bielliptic(p, o['coeffs'])
    n = E1.count()
    ji = jinfo(p, E1.a2, E1.a4, E1.a6)
    rec = row['rho']['automorphism_fold']['j_invariant']
    out['curves'].append({'target': 'cover_pushforward', 'p': p, 'coeffs_c': o['coeffs'],
                          'e1': [E1.a2, E1.a4, E1.a6], 'e1_order_count': n, 'remeasure_e1_order': row['e1_order'], **ji, 'remeasure_j': rec})
    if ji['j'] != rec: out['errors'].append(['cover j differs', p, ji['j'], rec])
    if n != row['e1_order']: out['errors'].append(['cover #E1 differs', p, n, row['e1_order']])
    if ji['extra_automorphism']: out['errors'].append(['cover extra automorphism', p])
json.dump(out, open(sys.argv[1], 'w'), indent=1)
print('curves', len(out['curves']), 'errors', out['errors'])
for c in out['curves']: print(c['target'], c['p'], 'j', c['j'], 'remeasure j', c['remeasure_j'], 'extra aut', c['extra_automorphism'])
