import json, glob, os, math, collections, csv
BASE="/tmp/claude-0/-home-user/e260bf71-4677-5a1e-b0d3-1c3899bb5656/scratchpad/redteam/research"
PR=BASE+"/gaudry_quartic_d4_exponent_20261015/sessions"
BN=BASE+"/gaudry_k4_char2_crossover_20261015/sessions"
OUT="/tmp/claude-0/-home-user/e260bf71-4677-5a1e-b0d3-1c3899bb5656/scratchpad/redteam/out/prime-crosscheck"

def load(path):
    out=[]
    for l in open(path):
        out.append(json.loads(l))
    return out

def phase(r, name):
    for p in r['phases']:
        if p['name']==name: return p['gae']
    return None

def rows_for(root, sessions_glob=("*",), exclude_substr=("killed",)):
    rows=[]
    for sd in sorted(glob.glob(root+"/*/")):
        sname=os.path.basename(sd.rstrip('/'))
        if any(x in sname for x in exclude_substr): continue
        f=sd+"records.jsonl"
        if not os.path.exists(f): continue
        for r in load(f):
            cs=r['workload']['curve_spec']
            c=r['counters']; d=r.get('detail') or {}
            row=dict(session=sname, arm=r['arm'], role=r['role'], warmup=r['warmup'],
                     status=r['outcome']['status'], matches=r['outcome'].get('matches_target'),
                     kind=cs['kind'], seed=cs.get('seed'), target_index=r['workload']['target_index'],
                     target=tuple(r['workload']['target']), curve=r['workload']['curve']['icv1'],
                     S=r['cost']['s'], GAE=r['cost']['total_gae'],
                     GAE_setup=phase(r,'setup'), GAE_search=phase(r,'search'),
                     GAE_fb=phase(r,'factor_base'), GAE_rel=phase(r,'relations'),
                     GAE_la=phase(r,'linear_algebra'), GAE_ver=phase(r,'verify'),
                     GAE_int=phase(r,'internal_verification'),
                     trials=c.get('targets_tried'), rel=c.get('relations_found'), rows=c.get('matrix_rows'),
                     cols=(r.get('factor_base') or {}).get('columns'),
                     la=(d.get('linear_algebra') or {}).get('name'))
            if cs['kind']=='fp4_torsion': row['p']=cs['p']; row['q']=cs['p']
            else: row['m']=cs['m']; row['q']=2**cs['m']
            if 'p' in cs: row['p']=cs['p']
            rows.append(row)
    return rows

prime=rows_for(PR)
binary=rows_for(BN, exclude_substr=("killed","pilot"))
json.dump({'prime':prime,'binary':binary}, open(OUT+"/records_extracted.json","w"))
print(len(prime), len(binary))
print(collections.Counter((r['session'].split('-')[0], r['arm']) for r in prime))
print(collections.Counter((r['session'].split('-')[0], r['arm']) for r in binary))
