from pathlib import Path
from math import isqrt
import json,importlib.util
from jets import normalized
root=Path(__file__).resolve().parent;c=json.loads((root.parent/'round52-p192/input.json').read_text());p,r,a,b=c['p'],c['r'],c['a'],c['b']
s=importlib.util.spec_from_file_location('curve52',root.parent/'round52-p192/model.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
points=[c['G'],c['Q'],m.mul(2,c['G'],p,a)];rows=[]
for P in points:
 v,ops=normalized(r,P[0],a,b,p);assert v[0]==0 and v[1]!=0;kappa=4*P[1]*P[1]*v[1]*v[1]%p;assert kappa
 rows.append({'point':P,'psi_value':v[0],'psi_derivative':v[1],'kappa':kappa,'recurrence_counts':ops})
d=16*5*2389;assert (r-1)%d==0;L=(r-1)//d
ceilroot=lambda n:isqrt(n)+(isqrt(n)**2<n)
out={'all_checks_pass':True,'p':p,'r':r,'a':a,'b':b,'normalization_cases':rows,'auxiliary_input_candidate':{'d':d,'divides_r_minus_one':True,'L':L,'sqrtL_ceil':ceilroot(L),'sqrtd_ceil':ceilroot(d),'action_queries_to_construct_xdG':d-1,'status':'admissible divisor only; numerator construction and generator/setup costs remain; no P192 Cheon execution'},'counts_scope':'uniform dual operations plus memo entries; coefficient setup and normalization outside counts','P192_regularized_numerator':'not constructed'}
(root/'P192-certificate.json').write_text(json.dumps(out,indent=2)+'\n');print('P192_NORMALIZATION_PASS',len(rows),'memo counts',[z['recurrence_counts']['memoized_indices'] for z in rows],'d',d)
