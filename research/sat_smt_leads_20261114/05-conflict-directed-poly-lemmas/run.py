import sys, json, time
from pdp5 import *
n=13; BB={2:470,3:1600,4:9500}
rows=[]; val=[]
out=open('rows.jsonl','a')
ls=[int(x) for x in sys.argv[1:]] or [2,3,4]
for l in ls:
    for idx in (0,4,1,5,2,6,3,7):
        planted = idx<4
        inst=make_instance(n,l,idx,planted); sysd=build_system(inst)
        bf=brute_force(inst)
        sols=solution_assignment(inst,sysd,inst['sol']) if planted else []
        if planted: assert sols and all(all(eval_anf(e,a)==0 for e in sysd['eqs']) for a in sols)
        counts={}
        for arm,k in (('A0',0),('A1',0),('A2',8),('A3',8),('A2',12),('A3',12)):
            r=run_arm(inst,sysd,arm,B=BB[l],k=k or 8,replay_counts=counts.get(k),time_cap=150)
            if arm=='A2': counts[k]=r['lemma_counts']
            model=r.pop('model'); lem=r.pop('_lemmas')
            chk=dict(bf_count=bf, status_matches_bf=(r['status']=='TIMEOUT' or (r['status']=='SAT')==(bf>0)))
            if model:
                alg,lifts,xs=check_model(inst,sysd,model); chk.update(model_alg_ok=alg,model_lifts=lifts)
            if planted: chk['lemmas_sound_on_planted']=all(eval_anf(e,a)==0 for e in lem for a in sols)
            row=dict(n=n,l=l,idx=idx,seed=inst['seed'],planted=planted,B=BB[l],k=k,**r,**chk)
            out.write(json.dumps(row)+'\n'); out.flush()
            print(l,idx,arm,k,r['status'],r['conflicts'],r['rounds'],r['lemmas'],round(r['wall'],1),chk,flush=True)
