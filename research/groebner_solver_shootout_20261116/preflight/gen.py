import sys; sys.path.insert(0,'/home/user/crypto')
from groebner_compare import semaev as S
cells=[(3,v,n,l,a) for v in ('plain','symmetrised','symmetrised-e') for (n,l,a) in ((11,4,0),(13,5,0),(15,5,1),(17,6,0))]+[(4,'plain',n,l,a) for (n,l,a) in ((9,3,1),(11,3,0),(13,4,0))]
for m,v,n,l,a in cells:
    inst,ans=S.generate(m,n,l,variant=v,a=a,b=1,seed=1,index=0)
    nv=inst['semaev']['layout']['nvars']; name=f"m{m}-{v}-n{n}-l{l}-a{a}"
    open(name+'.ms','w').write(S.to_msolve(inst['equations'],nv))
    open(name+'.dimacs','w').write(S.to_dimacs_xor(inst['equations'],nv)[0])
    print(name,nv,len(inst['equations']),flush=True)
