import time
START=time.monotonic()
import json,random,hashlib
from pathlib import Path
from .common import P,finish
from research.shifted_tangent_20261117_v144b.kernel import packed,at
from research.precision_filtration_20261117_v132b.kernel import actual_family,filtration
from research.delayed_block_20261117_v137b.kernel import coefficients
from research.residue_quotient_20261116_v99b.kernel import basis,apply
from research.odd_coprime_factor_20261116_v119c.kernel import inverse_block,initial
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.delayed_block_20261117_v137b.kernel import solve

def parallel_rank(A,full):
 A=[list(r)for r in A];h=len(A);w=len(A[0]);masks=[full]+[0]*h
 for col in range(w):
  previous=list(masks);masks=[0]*(h+1);masks[h]=previous[h]
  for r in range(h):
   active=previous[r];found=0
   for i in range(r,h):
    selected=active&A[i][col]&(full^found);found|=selected
    if i!=r:
     for j in range(w):delta=(A[r][j]^A[i][j])&selected;A[r][j]^=delta;A[i][j]^=delta
   for i in range(h):
    if i!=r:
     condition=found&A[i][col]
     for j in range(w):A[i][j]^=A[r][j]&condition
   masks[r]|=active&(full^found);masks[r+1]|=found
 assert sum(m.bit_count()for m in masks)==full.bit_count()
 return masks

def plane_rows(F,Ds,full,d=18,ell=16,n=23):
 h=n-ell;f=Arithmetic(2,[0,1]);S,I=inverse_block(f);A=[[0],[0],[full]];B=[[full if a else 0]for a in initial(f)];m=d-1
 def transform(e):
  out=[]
  for row in I:
   a=0
   for j,b in enumerate(row):
    if b:a^=e[j]
   out.append(a)
  return out
 for k in range(1,h+1):
  e=[F[j][k]for j in range(5)]
  for i,a in enumerate(A):
   for j,b in enumerate(B):
    if i+j<5:
     for t in range(k+1):e[i+j]^=at(a,t)&at(b,k-t)
  c=transform(e)
  for V,degree,lo in [(A,2,0),(B,3,2)]:
   for j in range(degree):
    if k<=(degree-j)*m:V[j].append(c[lo+j])
 rows=[[0]*len(Ds)for _ in range(6)]
 for lane,D in enumerate(Ds):
  DA=[[]for _ in range(3)];DB=[[]for _ in range(4)]
  for s in range(h+1):
   e=[full if D[j]>>(ell+s)&1 else 0 for j in range(5)]
   for i in range(3):
    for j in range(4):
     if i+j<5:
      for r in range(s):e[i+j]^=at(DA[i],r)&at(B[j],s-r);e[i+j]^=at(A[i],s-r)&at(DB[j],r)
   dc=transform(e)
   if ell+s>=18:rows[ell+s-18][lane]=dc[1]
   for V,degree,lo in [(DA,2,0),(DB,3,2)]:
    for j in range(degree):
     if ell+s<=(degree-j)*m:V[j].append(dc[lo+j])
 return rows

out={'parallel_rank_oracles':0,'scalar_plane_checks':0,'complete_early_census':False,'original_complete':False};error=None
try:
 data=json.loads((P/'DATA.json').read_bytes());rng=random.Random(data['seed']);full=(1<<64)-1;A=[[sum(((code>>(i*3+j))&1)<<code for code in range(64))for j in range(3)]for i in range(2)];masks=parallel_rank(A,full)
 for code in range(64):assert next(r for r,m in enumerate(masks)if m>>code&1)==solve([code&7,code>>3],0,3)['rank'];out['parallel_rank_oracles']+=1
 full=(1<<256)-1;A=[[rng.getrandbits(256)for _ in range(7)]for _ in range(6)];masks=parallel_rank(A,full)
 for code in range(256):
  rows=[sum(((A[i][j]>>code)&1)<<j for j in range(7))for i in range(6)];assert next(r for r,m in enumerate(masks)if m>>code&1)==solve(rows,0,7)['rank'];out['parallel_rank_oracles']+=1
 cfg=json.loads(Path('research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads(Path('research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];base,cols=actual_family(cfg,chart);K0=filtration(cols,16)['kernel'];K=[int('25bc29bd1e36b336',16)]
 for v in K0:
  if len(basis(K+[v])[0])>len(K):K.append(v)
 assert len(K)==7;Ds=[coefficients(cols,v)for v in K];early=filtration(cols,8)['lower_columns'];piv,_=basis(early);images=[piv[k][1]for k in sorted(piv)];r=len(images);assert r<=data['early_rank_cap']and r==24;N=1<<r;assert (N//8)*(40+7*28+42+35+24)<data['byte_budget'],'static_memory_admission';full=(1<<N)-1;patterns=[]
 for j in range(r):
  run=1<<j;a=((1<<run)-1)<<run;length=2*run
  while length<N:a|=a<<length;length*=2
  patterns.append(a)
 F=[]
 for j in range(6):
  a=[]
  for k in range(8):
   v=full if base[j]>>k&1 else 0
   if j<5:
    for s,b in enumerate(images):
     if b>>(5*k+j)&1:v^=patterns[s]
   a.append(v)
  F.append(a)
 rows=plane_rows(F,Ds,full);assert rows[0][0]==full,'global_unit_plane';nbytes=(N+7)//8;fingerprints=[[hashlib.sha256(v.to_bytes(nbytes,'little')).hexdigest()for v in row]for row in rows]
 for code in [rng.randrange(N)for _ in range(data['scalar_checks'])]:
  b=apply(images,code);scalar=[(base[j]&255)^sum(((b>>(5*k+j))&1)<<k for k in range(8))if j<5 else base[j]&255 for j in range(6)];q=packed(scalar,Ds,18,16,23)['correction_masks'];expected=[q[k-16][1]for k in range(18,24)];actual=[sum(((rows[i][j]>>code)&1)<<j for j in range(7))for i in range(6)];assert actual==expected;out['scalar_plane_checks']+=1
 del F,patterns
 masks=parallel_rank(rows,full);counts={str(i):v.bit_count()for i,v in enumerate(masks)};assert sum(counts.values())==N and counts['0']==0;upper=sum(count*(1<<(63-r-int(rank)))for rank,count in counts.items());out.update(complete_early_census=True,early_rank=r,early_states=N,rank_distribution=counts,conditional_parameter_upper_bound=upper,development_bound_target_met=upper<=data['target_upper_bound'],plane_sha256=fingerprints,intermediate_planes_retained=False,global_original_root_count=None,total_seconds=time.monotonic()-START)
except Exception as exc:error=repr(exc)
if error:out['complete_early_census']=False;out['conditional_parameter_upper_bound']=None;out['global_original_root_count']=None
finish(out,error,START,'Complete bit-plane early-cell rank census with scalar and parallel Gaussian controls','If complete,compile a singularity-safe dynamicK16block and measure total graph/search costs;otherwise repair memory/work admission or the exact failed bit-plane assumption in a new variant. Parameter bounds are not runtimes or complete original root sets.')
