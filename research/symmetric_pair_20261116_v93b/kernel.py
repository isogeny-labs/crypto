from research.disjoint_separator_20261116_v92.kernel import Prefix

def solve(spec):
 n,mod,d,u,B=spec['n'],spec['mod'],spec['d'],spec['u'],spec['B'];S=Prefix(n,mod,d);f=S.f
 if B==1 and n>4*(d-1)and u in[0,1]:
  roots=[(1,1)]if u==0 else[(0,1),(1,0)]
  return {'complete':True,'roots':roots,'exact_root_count':len(roots),'stats':{'shortcut':'polynomial-degree','coefficient_states':0}}
 e=min(2*d-1,n);cols=[];v=f.sq(u)
 for _ in range(d):cols.append(v);v=S.timesT(S.timesT(v))
 E=Prefix(n,mod,e);cols.extend(E.columns(1,u,True));rows,ker=S.prepare(cols);base=S.particular(B,rows);stats={'rank':len(rows),'nullity':len(ker),'coefficient_states':0}
 if base is None:return {'complete':True,'roots':[],'exact_root_count':0,'stats':stats}
 if len(ker)>spec['fiber_cap_bits']:return {'complete':False,'roots':[],'exact_root_count':None,'stats':stats,'reason':'coefficient-fiber cap'}
 states=[base]
 for w in ker:states +=[x^w for x in states]
 out=set()
 for z in states:
  s=z&((1<<d)-1);t=z>>d;stats['coefficient_states']+=1
  for x in S.solve(1,s,t,'stream'):out.add((x,x^s))
 return {'complete':True,'roots':sorted(out),'exact_root_count':len(out),'stats':stats}
