"""Score E13 (Amendments 17/18) from committed files only. Usage: python3 e13/score_e13.py STUDY_DIR > e13/score.json"""
import json, os, sys
D = sys.argv[1]; L = os.path.join(D, 'e13/local')
def ref(path):
    return {r['draw']: r for r in (json.loads(l) for l in open(os.path.join(D, path)) if l.strip())}
def pe(name):
    f = os.path.join(L, name + '.jsonl')
    if os.path.exists(f):
        return json.loads(open(f).readline())
    return None
out = {'E13b': {'censored': [], 'controls': []}}
cens = [('3-21-7', 'e10/run-r3-B/out/E_3-21-7.jsonl', [0, 1, 3, 4]),
        ('3-24-8', 'e10/run-r3-B/out/E_3-24-8.jsonl', [0, 1, 2, 3]),
        ('3-27-9', 'e10/run-r3-B/out/E_3-27-9.jsonl', [1, 2, 3, 4])]
ok_c = True
for cell, p, ds in cens:
    R = ref(p)
    for d in ds:
        r = pe(f'B_{cell}_d{d}'); h = R[d]['per_degree']
        row = {'cell': cell, 'draw': d, 'hky_par_dim3': h['3']['dim'], 'hky_par_dim4': h['4']['dim'], 'of4': h['4']['of']}
        if r is None:
            row['status'] = 'not completed'; ok_c = False
        else:
            s4 = [s for s in r['scan'] if s['degree'] == 4][0]; s3 = [s for s in r['scan'] if s['degree'] == 3][0]
            row.update(pe_rank3=s3['closure_rank'], pe_rank4=s4['closure_rank'], refuted4=s4['closure_refuted'], saturated4=s4['saturated'])
            row['agree'] = (not s4['closure_refuted']) and s4['saturated'] and s4['closure_rank'] == h['4']['dim'] and s3['closure_rank'] == h['3']['dim']
            row['status'] = 'agree' if row['agree'] else 'DIFFERS'; ok_c &= row['agree']
        out['E13b']['censored'].append(row)
ctls = [('4-12-3', 'e10/run-r3-C/out/E_4-12-3.jsonl', [0, 1, 2, 3]), ('6-10-3', 'e9/run-r3/out/E_6-10-3.jsonl', [1, 2, 3, 4])]
ok_k = True; differ = []
for cell, p, ds in ctls:
    R = ref(p)
    for d in ds:
        r = pe(f'B_ctl_{cell}_d{d}'); rec = R[d].get('d_f')
        row = {'cell': cell, 'draw': d, 'recorded_d_F': rec}
        if r is None:
            row['status'] = 'not completed (memory)' if os.path.exists(os.path.join(L, f'B_ctl_{cell}_d{d}.failed')) else 'missing'; ok_k = False
        else:
            row['pe_closure_degree'] = r.get('closure_degree')
            row['status'] = 'refutes at recorded d_F' if r.get('closure_degree') == rec else 'DIFFERS'
            if row['status'] == 'DIFFERS': ok_k = False; differ.append((cell, d))
        out['E13b']['controls'].append(row)
differ += [(r['cell'], r['draw']) for r in out['E13b']['censored'] if r['status'] == 'DIFFERS']
out['E13b']['reading'] = 'independent agreement' if ok_c and ok_k else 'disagreement'
out['E13b']['draws_that_differ'] = differ
out['E13b']['unmeasured_controls'] = [(r['cell'], r['draw']) for r in out['E13b']['controls'] if r['status'].startswith('not completed') or r['status'] == 'missing']
a = []
for d in [0, 1, 3, 4]:
    r = pe(f'A_3-21-7_d{d}')
    a.append({'draw': d, 'status': 'unmeasured (not completed, memory)' if r is None else r.get('closure_degree')})
meas = [x for x in a if not str(x['status']).startswith('unmeasured')]
out['E13a'] = {'draws': a, 'reading': 'not completed' if len(meas) < 2 else 'see draws'}
st = open(os.path.join(L, 'C/status')).read().strip() if os.path.exists(os.path.join(L, 'C/status')) else 'missing'
out['E13c'] = {'status_file': st, 'reading': 'not completed (gate 0 build exceeded 13 GiB cap; no draw computed)' if st.startswith('gate_failed') else st}
print(json.dumps(out, indent=1))
