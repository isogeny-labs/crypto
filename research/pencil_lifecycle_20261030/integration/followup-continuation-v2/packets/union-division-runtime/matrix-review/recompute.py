from pathlib import Path
import json,hashlib,sys
p=Path(sys.argv[1]); summary=json.loads((p/'inputs/summary.json').read_text())
def rows(name):
 xs=[json.loads(s) for s in (p/'inputs'/name).read_text().splitlines() if s.strip()]
 ds={x['p']:x for x in xs};assert len(ds)==len(xs);return ds
theta=rows('theta-models.jsonl');generic=rows('generic-models.jsonl')
models={x['p']:x for x in summary['models']};assert len(models)==len(summary['models'])==21
assert set(models)==set(theta)==set(generic)
out=[]
for prime,m in models.items():
 assert 2<=prime<=2**32 and all(prime%d for d in range(2,__import__('math').isqrt(prime)+1))
 acts=generic[prime]['torsion_actions'];assert len(acts)==len(m['square_scalars'])==5
 t=theta[prime]['selected'];d=t['delta'];r0=t['r0'];assert 0<d<prime and 0<=r0<prime
 for i,a in enumerate(acts):
  mat=a['action']['matrix'];assert len(mat)==4 and all(len(row)==4 for row in mat)
  assert all(type(x)==int and 0<=x<prime for row in mat for x in row)
  square=[[sum(mat[j][k]*mat[k][l] for k in range(4))%prime for l in range(4)] for j in range(4)]
  scalar=m['square_scalars'][i];assert 0<scalar<prime
  assert square==[[scalar if j==l else 0 for l in range(4)] for j in range(4)]
  r=a['root'];assert 0<=r<prime
  out.append({'p':prime,'index':i,'square_scalar':scalar,'normalized_root':((r-r0)*pow(d,-1,prime))%prime})
assert len(out)==105
print(json.dumps({'accepted':True,'models':21,'actions':105,'rows':out},indent=2))
