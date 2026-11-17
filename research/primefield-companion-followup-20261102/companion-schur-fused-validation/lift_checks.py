"""Unexecuted full per-context residual, witness and obstruction checks."""
from random import Random
import numpy as np
from kernel import Meter
from audit import AuditedSchur,check_ledger
from lift import reduced_rhs,lift_primal,lift_dual
from certificates import check_residuals,product,pairing
from schur import LeadingBlockObstruction
from schur_reference import oracles,inverse_or_null

def rejects(fn):
    try:fn()
    except AssertionError:return
    raise AssertionError('corrupted certificate accepted')

def fixture(A,S,G,K,a,b,cut,Inv,r,seed,arm):
    N=len(A);m=N-cut;rng=Random(seed);records=[]
    cases=[('zero',[0]*N,[0]*m,[0]*m),('ones',[1]*N,[1]*m,[1]*m),
           ('seeded',[rng.randrange(r) for _ in range(N)],[rng.randrange(r) for _ in range(m)],[rng.randrange(r) for _ in range(m)])]
    _,null=inverse_or_null(np.array(S,dtype=np.int64).T,r)
    if null is not None:
        j=next(i for i,t in enumerate(null) if t);h=[0]*m;h[j]=pow(null[j],-1,r)
        cases.append(('normalized',[0]*cut+h,[0]*m,null))
    for label,rhs,z,v in cases:
        sf,st,stats=oracles(Inv,r);meter=Meter(r)
        obj=AuditedSchur(G,K,a,b,cut,sf,st,meter,dense_reference=A)
        arg=lambda x:x if arm=='candidate' else tuple(x)
        h=reduced_rhs(obj,arg(rhs));x=lift_primal(obj,arg(rhs),arg(z));w=lift_dual(obj,arg(v))
        certificate=check_residuals(A,S,rhs,z,h,x,v,w,r)
        # All three adapters called; check their combined schedule and oracle work.
        expected=check_ledger(obj,2,1,dict(rhs_reduction_calls=1,primal_lift_calls=1,dual_lift_calls=1,
            leading_forward_calls=2,leading_transpose_calls=1,field_additions=m+2*cut))
        assert stats==dict(forward_calls=2,transpose_calls=1,dense_terms=3*cut*cut,coefficient_reductions=3*cut)
        badx=x[:];badx[0]=(badx[0]+1)%r
        badw=w[:];badw[0]=(badw[0]+1)%r
        badh=h[:];badh[0]=(badh[0]+1)%r
        rejects(lambda:check_residuals(A,S,rhs,z,h,badx,v,w,r))
        rejects(lambda:check_residuals(A,S,rhs,z,h,x,v,badw,r))
        rejects(lambda:check_residuals(A,S,rhs,z,badh,x,v,w,r))
        bad=dict(expected);bad['source_forward_calls']+=1;assert bad!=meter.counts
        controls=4
        if label=='normalized':
            assert certificate['normalized_exclusion'] and pairing(w,rhs,r)==1
            assert not any(product(A,w,r,True))
            assert pairing([0]*N,rhs,r)!=1
            assert pairing(w,[(2*t)%r for t in rhs],r)!=1
            controls+=2
        records.append(dict(label=label,rhs=rhs,z=z,v=v,h=h,x=x,w=w,certificate=certificate,
            counts=meter.counts,source_receipts=obj.source_receipts,blocks=meter.blocks,
            oracle_work=stats,mutations_rejected=controls))
    return records

def obstructions(A,G,K,a,b,cut,r,arm):
    N=len(A);m=N-cut;records=[]
    for name,forward,transpose,leading_f,leading_t in [('rhs',0,0,1,0),('primal',1,0,1,0),('dual',0,1,0,1)]:
        sf,st,stats=oracles(None,r);meter=Meter(r);obj=AuditedSchur(G,K,a,b,cut,sf,st,meter,dense_reference=A)
        arg=lambda n:[0]*n if arm=='candidate' else tuple([0]*n)
        try:
            if name=='rhs':reduced_rhs(obj,arg(N))
            elif name=='primal':lift_primal(obj,arg(N),arg(m))
            else:lift_dual(obj,arg(m))
        except LeadingBlockObstruction as e:assert 'undetermined' in str(e)
        else:raise AssertionError('leading obstruction suppressed')
        extras={dict(rhs='rhs_reduction_calls',primal='primal_lift_calls',dual='dual_lift_calls')[name]:1}
        if leading_f:extras['leading_forward_calls']=leading_f
        if leading_t:extras['leading_transpose_calls']=leading_t
        if name=='primal':extras['field_additions']=cut
        check_ledger(obj,forward,transpose,extras)
        assert stats==dict(forward_calls=leading_f,transpose_calls=leading_t,dense_terms=0,coefficient_reductions=0)
        records.append(dict(adapter=name,counts=meter.counts,source_receipts=obj.source_receipts,
            blocks=meter.blocks,oracle_work=stats,full_matrix_rank='undetermined'))
    return records
