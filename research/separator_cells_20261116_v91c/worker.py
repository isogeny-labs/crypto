from itertools import product
import json,time,hashlib
from .common import P,finish
from research.separator_images_20261116_v91.kernel import Solver
START=time.monotonic();out={'panels':[],'complete':False,'qualified_speedup':None};error=None
try:
 data=json.load(open(P/'DATA.json'))
 for spec in data['panels']:
  S=Solver(spec['n'],spec['mod'],spec['d']);f=S.f;rows=[]
  for k in data['cell_dimensions']:
   for q in [x for x in spec['queries']if x['kind']=='S3']:
    y=q['y'];u0=q['u']&~((1<<k)-1);a,b,c=S.coefficients(y,u0,q['B']);cols=S.columns(a,b,True)
    for i in range(k):
     h=1<<i;v=f.mul(y,h);cols.extend(S.columns(f.sq(h),v,True));cols.append(f.sq(v))
    prepared=S.prepare(cols);res=c
    while res:
     pivot=res.bit_length()-1
     if pivot not in prepared[0]:break
     res^=prepared[0][pivot][0]
    excluded=S.particular(c,prepared[0])is None;witness=None
    if excluded:
     # A coordinate of the fully reduced linear remainder is a dual witness.
     def reduce(v):
      for pivot in sorted(prepared[0],reverse=True):
       if v>>pivot&1:v^=prepared[0][pivot][0]
      return v
     residual=reduce(c);bit=(residual&-residual).bit_length()-1;witness=sum(((reduce(1<<i)>>bit)&1)<<i for i in range(S.n));assert all((witness&v).bit_count()%2==0 for v in cols)and(witness&c).bit_count()%2==1
    total=0;positive_separators=0
    for t in range(1<<k):
     aa,bb,cc=S.coefficients(y,u0^t,q['B']);xs=S.solve(aa,bb,cc,'stream');total+=len(xs);positive_separators+=bool(xs)
     if S.n<=4:
      literal=[x for x in range(1<<S.d)if f.mul(aa,f.sq(x))^f.mul(bb,x)==cc];assert xs==literal
    assert not excluded or total==0
    rows.append({'dimension':k,'query_id':q['id'],'y':y,'u_base':u0,'span_rank':len(prepared[0]),'columns':len(cols),'certified_empty':excluded,'dual_witness':witness,'pointwise_root_count':total,'positive_separators':positive_separators})
  out['panels'].append({'id':spec['id'],'cells':rows,'excluded':sum(r['certified_empty']for r in rows),'total':len(rows),'rank_min':min(r['span_rank']for r in rows),'rank_max':max(r['span_rank']for r in rows),'unique_scope':'cells overlap acrossdimensions;counts arecertificatecases,not global searchvolume'})
 out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Wholeaffineseparatorcell linearwitnesses verified against complete pointwisefibers;passedcells remainunresolved.','Ifunionrank saturates,try exactbilinear rankconstraints orsmaller disjointprefixcells;chargewholecovercost andretainallpositivebranches;neverclaimglobalpruningfromsamplecells.')
