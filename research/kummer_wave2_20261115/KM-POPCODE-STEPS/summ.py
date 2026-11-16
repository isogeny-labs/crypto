import json,sys,collections
for fn in sys.argv[1:]:
    rows=[json.loads(l) for l in open(fn)]
    meta=[r for r in rows if r.get('meta')]
    rows=[r for r in rows if not r.get('meta')]
    tab=collections.defaultdict(dict)
    for r in rows: tab[(r['tag'],r['d'])][r['s']]=r
    for (tag,d),v in sorted(tab.items()):
        ss=sorted(v)
        print(fn,tag,'d=%d'%d,'dimM',[v[s]['dimM'] for s in ss],'nu',[v[s]['nu'] for s in ss],'ver',all(v[s]['verified'] for s in ss))
        pos=[s-1 for s in ss if v[s]['nu']]
        if d>=2: print('    sigma(last positive s-1)=',max(pos) if pos else None,'smax',ss[-1])
