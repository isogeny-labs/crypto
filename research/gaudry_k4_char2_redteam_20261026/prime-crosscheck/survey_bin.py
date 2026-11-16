import json, glob, os, collections
ROOT="/tmp/claude-0/-home-user/e260bf71-4677-5a1e-b0d3-1c3899bb5656/scratchpad/redteam/research/gaudry_k4_char2_crossover_20261015"
for f in sorted(glob.glob(ROOT+"/sessions/*/records.jsonl")+glob.glob(ROOT+"/sessions*/*.attempt*/records.jsonl")):
    d=os.path.basename(os.path.dirname(f))
    c=collections.Counter()
    for l in open(f):
        r=json.loads(l)
        c[(r['arm'], r['role'], r['outcome']['status'], r['warmup'], str(r['workload']['curve_spec'].get('kind')), str(r['workload']['curve_spec'].get('m', r['workload']['curve_spec'].get('field_bits'))))]+=1
    print(f.split('/research/')[1], dict(c))
