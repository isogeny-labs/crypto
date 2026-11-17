import time,json
from research.kernel_lift_solver_20261116_v88b.solver import solve as prior
from research.kernel_trellis_20261116_v89.fiber_solver import solve as generic
from research.twist_three_isogeny_20261116_v86b.curve import add,neg,scale,on

def solve(case,method):
 if method!='fullkernel':return prior(case,method)
 start=time.monotonic();p,A,B=case['p'],case['A'],case['B'];R=[tuple(U)for U in case['R']];Q=tuple(case['Q']);K=[None if U is None else tuple(U)for U in case['K']];assert case['group_order']%2==1 and len(K)==9 and len(set(K))==9 and all(on(U,p,A,B)and scale(3,U,p,A)is None for U in K);assert all(on(U,p,A,B)and neg(U,p)in R for U in R)and on(Q,p,A,B)
 # Characteristic>3: [3] has exactly9 geometric kernel points;9distinct rational points exhaust it.
 assert p>3
 roots,stats=generic(R,Q,case['arity'],lambda U,V:add(U,V,p,A),lambda U:neg(U,p),lambda U:scale(3,U,p,A),K)
 out={'complete':True,'roots':sorted(roots),'exact_root_count':len(roots),'stats':stats,'kernel_setup_and_search_seconds':time.monotonic()-start};trial=json.dumps(out,separators=(',',':'))
 if time.monotonic()-start>=9:out['complete']=False;out['exact_root_count']=None
 return out
