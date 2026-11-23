import json,sys
d=json.load(open(sys.argv[1])); m,t=d['m'],d['t']; k=d.get('k',max(m-2,0))
out=[f"{d['p']} {m} {t} {k} {len(d['polys'])}"]
for P in d['polys']:
    out.append(str(len(P)))
    for e,c in P: out.append(' '.join(map(str,e))+' '+str(c))
open(sys.argv[2],'w').write('\n'.join(out)+'\n')
