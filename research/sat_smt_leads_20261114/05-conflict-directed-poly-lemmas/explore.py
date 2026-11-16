import sys, time
from pdp5 import *
for n in (11,13,17,19):
    F=Field(n); assert F.irreducible_check(), n
n=int(sys.argv[1]); l=int(sys.argv[2])
for planted in (True, False):
    inst=make_instance(n,l,900,planted)
    E=inst['E']; assert E.on(inst['R'])
    sysd=build_system(inst)
    print('n',n,'l',l,'planted',planted,'N',sysd['N'],'eqs',len(sysd['eqs']),'monos',len(sysd['monos']))
    if planted:
        sols=solution_assignment(inst,sysd,inst['sol'])
        assert sols and all(all(eval_anf(e,a)==0 for e in sysd['eqs']) for a in sols)
    r=run_arm(inst,sysd,'A0',B=10**9)
    print(' A0',r['status'],r['conflicts'],r['decisions'],round(r['wall'],2),'nv',r['nvars'])
    if r['model']: print('  model check',check_model(inst,sysd,r['model'])[:2])
    t=time.time(); g=LemmaGen(sysd); new=g.generate(list(range(sysd['N'])))
    print(' static lemmas',g.last_stats,'ops',g.ops,round(time.time()-t,1))
    if planted and new:
        print('  sound',all(eval_anf(e,a)==0 for e in new for a in sols), 'lens',sorted(len(e) for e in new)[:10])
    g=LemmaGen(sysd); new=g.generate(list(range(8))); print(' local k=8 (first vars)',g.last_stats)
