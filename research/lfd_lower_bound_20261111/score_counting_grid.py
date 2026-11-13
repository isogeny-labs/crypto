#!/usr/bin/env python3
"""Score out/lead_A2/grid.jsonl against the preregistered predictions of LEAD-A-selector-counting-law."""
import json, collections
pred = {  # (type,k,n,e): (A, B)  with A in {'F','I','?'} (feasible/infeasible/ambiguous)
    ('R', 8, 17, 2): ('I', 'F'), ('R', 9, 22, 2): ('I', 'F'), ('R', 10, 26, 2): ('I', 'F'), ('R', 10, 27, 2): ('I', 'F'),
    ('R', 8, 18, 2): ('?', 'F'), ('R', 11, 23, 3): ('?', 'F'), ('R', 12, 28, 3): ('?', 'F'), ('T', 13, 25, 4): ('F', 'I'),
}
controls = {('R', 11, 22, 3): 'I', ('R', 12, 26, 3): 'I', ('R', 13, 25, 3): 'I', ('R', 13, 25, 4): 'F'}
obs = collections.defaultdict(list)
for l in open('out/lead_A2/grid.jsonl'):
    d = json.loads(l); r = d['result']; key = (d['type'], d['k'], d['n'], d['e'])
    em = r.get('e_min'); obs[key].append('F' if em is not None and em <= d['e'] else 'I')
def cell(key):
    v = obs[key]; return v[0] if len(set(v)) == 1 else 'mixed'
a_hit = a_n = b_hit = b_n = 0
print(f"{'cell':<16} {'obs':<9} {'A':<3} {'B':<3}")
for key, (A, B) in pred.items():
    o = cell(key); print(f"{str(key):<16} {o+' '+''.join(obs[key]):<9} {A:<3} {B:<3}")
    if o == 'mixed': continue
    if A != '?': a_n += 1; a_hit += (A == o)
    b_n += 1; b_hit += (B == o)
print('controls:', {str(k): (cell(k), v) for k, v in controls.items()})
print(f"Model A: {a_hit}/{a_n} non-ambiguous predictions hold; Model B: {b_hit}/{b_n}")
bw = b_hit >= 7 and a_hit <= 2; aw = a_hit >= 4 and b_hit <= 4
print('verdict:', 'B wins' if bw else 'A wins' if aw else 'neither model wins by the preregistered rule')
