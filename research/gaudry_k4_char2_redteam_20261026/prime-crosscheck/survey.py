import json, glob, os, collections
ROOT="/tmp/claude-0/-home-user/e260bf71-4677-5a1e-b0d3-1c3899bb5656/scratchpad/redteam/research/gaudry_quartic_d4_exponent_20261015/sessions"
cnt=collections.Counter()
for f in sorted(glob.glob(ROOT+"/*/records.jsonl")):
    d=os.path.basename(os.path.dirname(f))
    for l in open(f):
        r=json.loads(l)
        cnt[(d.split('-')[0] if '-' in d else d, r['arm'], r['role'], r['outcome']['status'], r['warmup'])]+=1
for k,v in sorted(cnt.items()): print(k,v)
