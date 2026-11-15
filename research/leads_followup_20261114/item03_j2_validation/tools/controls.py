#!/usr/bin/env python3
"""Negative controls for the validator's own tools. usage: controls.py <J2 dir> <replay dir> <certcheck binary> <out.txt>"""
import json, sys, os, copy, subprocess
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import compare
J2, RP, CC, OUT = sys.argv[1:5]
out = []
def log(*a): out.append(' '.join(str(x) for x in a)); print(out[-1])
a = compare.recs(os.path.join(J2, 'heldout', 'ic_2097091_4_c1.60.jsonl')); b = compare.recs(os.path.join(RP, 'ic_2097091_4_c1.60.jsonl'))
log('comparator on a real pair (producer vs replay):', 'IDENTICAL' if not compare.diff(a, b)[3] else 'DIFFERS')
def tweak(f):
    t = copy.deepcopy(b); f(t); return compare.diff(a, t)[3]
def e(t): t[1]['relations.mul'] += 1
log('control relations.mul + 1 ->', tweak(e)[:1])
def e(t): t[2]['k_found'] += 1
log('control k_found + 1 ->', tweak(e)[:1])
def e(t): t[3]['linear_algebra.ladd'] -= 1
log('control linear_algebra.ladd - 1 ->', tweak(e)[:1])
log('control dropped record ->', compare.diff(a, b[:-1])[3][:1])
def e(t): t[1]['relations.wall'] += 5; t[0]['load1'] = 99; t[1]['s.controls_wall'] = 7
log('control timing-only change must be ignored ->', tweak(e))
# certcheck controls on q = 65479 seed 1
rs = compare.recs(os.path.join(J2, 'units', 'ic_65479_1_c2.00.jsonl')); rat = [r for r in rs if r['rec'] == 'ratios'][0]; ic = [r for r in rs if r['rec'] == 'ic'][0]
def run(ell, order, k, logs, tries='400'):
    p = subprocess.run([CC, '65479', '1', str(ell), str(order), str(k), tries], input='\n'.join(str(x) for x in logs) + '\n', capture_output=True, text=True)
    return json.loads(p.stdout.strip().splitlines()[-1])
r = run(rat['ell'], rat['order'], ic['k'], [ic['k_found'], ic['k_found'] + 1, ic['k_found'] ^ (1 << 20)])
log('certcheck true record: matched', r['matched'], 'tries', r['tries_until_match'], 'k_matches_record', r['k_matches_record'], 'logs_ok/checked', r['logs_ok'], '/', r['logs_checked'], '(expect 1/3: the +1 and xor logs are rejected)')
r = run(rat['ell'] + 2, rat['order'], ic['k'], [ic['k_found']]); log('control wrong ell ->', 'matched', r['matched'])
r = run(rat['ell'], rat['order'] + 4, ic['k'], [ic['k_found']]); log('control wrong order ->', 'matched', r['matched'])
r = run(rat['ell'], rat['order'], ic['k'] + 1, [ic['k_found']]); log('control wrong planted k ->', 'k_matches_record', r['k_matches_record'])
open(OUT, 'w').write('\n'.join(out) + '\n')
