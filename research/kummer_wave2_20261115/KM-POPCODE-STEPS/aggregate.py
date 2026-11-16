import json,glob,collections,sys
# collect nu table per (tag) and d; derive sigma and rule-implied d_F
meas={1:{**{t:v for t,v in zip(range(2,12),[3,3,4,4,4,4,5,5,5,5])}},
      2:{t:v for t,v in zip(range(2,12),[4,4,4,4,5,5,5,5,6,6])},
      3:{t:v for t,v in zip(range(2,10),[5,5,5,5,5,5,5,6])}}
reg={(1,2):4,(1,3):8,(1,4):12,(2,3):6,(2,4):10,(3,4):9}
tabs={}
for fn in sorted(glob.glob('raw/gen_*.jsonl')+glob.glob('raw/n[12]_*.jsonl')+glob.glob('raw/f1.jsonl')):
    for l in open(fn):
        r=json.loads(l)
        if r.get('meta'): continue
        tabs.setdefault((r['tag']),{}).setdefault(r['d'],{})[r['s']]=r
out={}
for tag,dd in tabs.items():
    for d,v in dd.items():
        if d<2: continue
        ss=sorted(v); nu=[v[s]['nu'] for s in ss]
        pos=[s-1 for s in ss if v[s]['nu']]
        sg=max(pos) if pos else None
        out.setdefault(tag,{})[d]=dict(smax=ss[-1],dimM=[v[s]['dimM'] for s in ss],nu=nu,sigma=sg)
json.dump(out,open('table.json','w'),indent=1)
for tag,dd in out.items():
    W=int(tag.split('-W')[1][0])
    print(tag,{d:(x['sigma'],x['smax']) for d,x in dd.items()})
