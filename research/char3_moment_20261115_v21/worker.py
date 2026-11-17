"""Characteristic-three polar moment image in a nonwrapping power basis."""
from pathlib import Path
import hashlib,itertools,json,resource,sys,time
P=Path(__file__).resolve().parent;START=time.monotonic();CHECKS=0
def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<500000 and time.monotonic()-START<78,'work cap'
 assert v,why
def reduce(v,piv):
 v=dict(v)
 while v:
  p=max(v)
  if p not in piv:break
  c=v[p]
  for j,a in piv[p].items():
   x=(v.get(j,0)-c*a)%3
   if x:v[j]=x
   else:v.pop(j,None)
 return v
def basis(rows):
 piv={}
 for row in rows:
  v=reduce(row,piv)
  if v:
   p=max(v);inv=1 if v[p]==1 else 2;piv[p]={j:a*inv%3 for j,a in v.items()}
 return piv
def forms(d):
 n=4*d-3;out=[]
 for positive,negative in[(1,3),(5,7),(2,6)]:
  out.append({j:1 if j%8==positive else 2 for j in range(n)if j%8 in[positive,negative]})
 return out
def image(d):
 n=4*d-3;rows=[{4*i:2}for i in range(d)];edges=[]
 for i in range(d):
  for j in range(i+1,d):
   a=3*i+j;b=i+3*j;rows.append({a:1,b:1});edges.append((a,b))
 piv=basis(rows);fs=forms(d);check(all(sum(c*f.get(j,0)for j,c in row.items())%3==0 for f in fs for row in rows),'signed guards annihilate every polar generator')
 check(len(piv)+len(basis(fs))==n,'guards exactly characterize image')
 adjacency=[set()for _ in range(n)]
 for a,b in edges:adjacency[a].add(b);adjacency[b].add(a)
 anchors={4*i for i in range(d)};seen=set();graph_rank=0;unanchored=[]
 for root in range(n):
  if root in seen:continue
  colors={root:0};stack=[root];bip=True
  while stack:
   a=stack.pop()
   for b in adjacency[a]:
    if b not in colors:colors[b]=colors[a]^1;stack.append(b)
    elif colors[b]==colors[a]:bip=False
  comp=set(colors);seen |=comp;anchored=bool(comp&anchors);graph_rank+=len(comp)-(not anchored and bip)
  if not anchored:unanchored.append(dict(vertices=len(comp),bipartite=bip))
 expected=1 if d==1 else 3 if d==2 else 4*d-6
 check(len(piv)==graph_rank==expected,'independent unsigned-graph rank oracle')
 if d>=3:check(len(unanchored)==3 and all(c['bipartite']for c in unanchored),'three connected signed components')
 return piv,dict(d=d,coefficient_dimensions=n,rank=len(piv),codimension=n-len(piv),graph_rank=graph_rank,unanchored_components=unanchored)
def trim(a):
 a=list(a)
 while a and not a[-1]:a.pop()
 return tuple(a)
def add(a,b,scale=1):return trim([((a[i]if i<len(a)else 0)+scale*(b[i]if i<len(b)else 0))%3 for i in range(max(len(a),len(b)))])
def mul(a,b):
 if not a or not b:return ()
 out=[0]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):out[i+j]=(out[i+j]+x*y)%3
 return trim(out)
def power(a,n):
 out=(1,)
 for _ in range(n):out=mul(out,a)
 return out
def dot(poly,form):return sum(c*form.get(i,0)for i,c in enumerate(poly))%3
def controls():
 cases=0;repeated=0
 for d in[1,2]:
  roots=[]
  for t in range(3**d):
   coeff=[]
   for _ in range(d):coeff.append(t%3);t//=3
   roots.append(trim(coeff))
  piv,_=image(d);f=forms(d)
  fourth=[power(x,4)for x in roots];third=[power(x,3)for x in roots]
  for indices in itertools.combinations_with_replacement(range(len(roots)),5):
   E=[(1,)]+[()for _ in range(5)];sum4=()
   for index in indices:
    x=roots[index];sum4=add(sum4,fourth[index])
    for j in range(5,0,-1):E[j]=add(E[j],mul(x,E[j-1]))
   a,b,c,e=E[1:5];value=add(sum4,power(a,4),-1)
   newton=add(add(add((),mul(power(a,2),b),-1),power(b,2),-1),mul(a,c));newton=add(newton,e,-1)
   pair=()
   for i,j in itertools.combinations(indices,2):pair=add(pair,add(mul(third[i],roots[j]),mul(roots[i],third[j])),-1)
   check(value==newton==pair,'independent literal power sums, Newton and pair-polar identities')
   check(not reduce({i:c for i,c in enumerate(value)if c},piv)and all(dot(value,x)==0 for x in f),'complete multiset image/guard check')
   cases+=1;repeated+=len(set(indices))<5
 check(cases==1308,'complete tiny five-source multiset corpus')
 counter=add((2,),power((2,),4),-1);check(counter==(1,),'binary constant-zero guard has a true odd-characteristic counterexample')
 return dict(source_multisets=cases,repeated_multisets=repeated,domain_sizes=[3,9],formal_polynomial_oracle=True,counterexample=dict(roots=[0,0,0,1,1],characteristic=3,Gold4=[1],binary_constant_zero_guard_would_reject=True))
def main():
 out={};error=None
 try:out['controls']=controls();out['ranks']=[image(d)[1]for d in range(1,65)]
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error,checks=CHECKS),sort_keys=True,separators=(',',':')).encode();assert len(raw)<256*1024
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),dimensions_tested=len(out.get('ranks',[])),rank_formula='rank1(d1),rank3(d2),rank4d-6(d>=3)',scope='FormalF3[t]moment image; embeds in anyGF(3^n)when n>=4d-3. No odd-field hard-instance timing or source/curve closure.',worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,novelty_claim=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Exact characteristic3signed Gold4 guards with graph-rank proof controls and complete tiny repeated-root oracles.',evidence=ev,next_actions=['Review primary-source prior art and integrate signed guards with separately proved odd norm/interval routes; retain reduction/wrap handling outside n>=4d-3. No NISTprime proper-subspace inference.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
