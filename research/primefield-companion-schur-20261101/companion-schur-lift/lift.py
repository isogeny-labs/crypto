"""Unexecuted primal/dual Schur adapters; caller supplies a Schur object."""
def reduced_rhs(obj,b):
    n=obj.cut;m=obj.size;assert len(b)==n+m
    obj.m.add('rhs_reduction_calls')
    x=obj._solve(list(b[:n]))
    cx=obj._source(x+[0]*m)[n:]
    return [obj.m.sub(u,v) for u,v in zip(b[n:],cx)]

def lift_primal(obj,b,z):
    n=obj.cut;m=obj.size;assert len(b)==n+m and len(z)==m
    obj.m.add('primal_lift_calls')
    bz=obj._source([0]*n+list(z))[:n]
    head=obj._solve([obj.m.sub(u,v) for u,v in zip(b[:n],bz)])
    return head+list(z)

def lift_dual(obj,v):
    n=obj.cut;m=obj.size;assert len(v)==m
    obj.m.add('dual_lift_calls')
    cv=obj._source([0]*n+list(v),True)[:n]
    head=obj._solve(cv,True)
    return [obj.m.sub(0,u) for u in head]+list(v)
