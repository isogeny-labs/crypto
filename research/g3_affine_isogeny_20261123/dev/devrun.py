import sys, json, random, time
sys.path.insert(0,'.')
import population as PO, build as BD
p=int(sys.argv[1]); n=int(sys.argv[2])
rng=random.Random(5)
for f,P in PO.curves(p,n,1000+p):
    for key in PO.stable_keys():
        t=rng.randrange(p)
        while sum(cf*pow(t,i,p) for i,cf in enumerate(f))%p==0: t=rng.randrange(p)
        s,x=PO.label(p,f,key,t,rng)
        res=BD.construct(p,f,sorted(sorted(s_) for s_ in key),t,seed=2)
        tw=[c*(-1)**(6-i) for i,c in enumerate(P)]
        PX=res.get('P_X')
        print(json.dumps({'p':p,'label':s,'status':res['status'],'stable':res.get('n_stable_aronhold_sets'),'signs':res.get('n_row_sign_patterns_ok'),
          'match':('J' if PX==P else 'twist' if PX==tw else None),'t_c':res.get('sec_construct'),'t_n':res.get('sec_count')}))
