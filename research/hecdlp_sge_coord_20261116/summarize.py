import json,collections,sys
rows=[json.loads(l) for l in open('smoke.jsonl')]
rows=[r for r in rows if r.get('kind')=='run']
agg=collections.defaultdict(list)
for r in rows: agg[(r['p'],r['arm'])].append(r)
print('p | arm | n | certified | fallbacks | core_cols in->out | la_mul | la_add | sge_mul | sge_add | rel_mul')
for (p,a),rs in sorted(agg.items()):
    n=len(rs); f=lambda k:sum(x.get(k,0) for x in rs)/n
    co='%d->%s'%(f('core_cols'),('%d'%f('sge_cols_out')) if 'sge_cols_out' in rs[0] else '-')
    print(p,a,n,sum(x['certified'] for x in rs),sum(x['la_fallbacks'] for x in rs),co,
      '%.3g %.3g %.3g %.3g %.3g'%(f('la_mul'),f('la_add'),f('la_sge_mul'),f('la_sge_add'),f('rel_mul')),sep=' | ')
