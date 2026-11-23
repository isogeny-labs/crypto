"""Regular doubled-source free-k fiber and exact incidence components."""
from pathlib import Path
import sys,json,time,gzip,hashlib,itertools,resource,statistics
from rootfinder.binaryfield import BinaryField,divmod_poly,powmod_poly,binary_field_roots
from research.quartic_cell_20261114_v1 import worker as old
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0

def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'predicate/time cap'
 assert v,why

def params(f,z,y,h,a,ya,k):
 ga=f.mul(h,a)^k;gz=f.mul(h,z)^k
 s=f.mul(f.mul(k,f.inv(a)),ya)^f.square(a)^f.mul(a,ga)
 numerator=f.mul(ga,ya)^f.mul(f.square(a),a)^f.mul(f.square(z),z)^f.mul(s,a^z)^f.mul(gz,y)
 r=f.mul(numerator,f.inv(f.square(a^z)))
 return r,s

def quotient(f,A,B,z,y,h,a,ya,k):
 r,s=params(f,z,y,h,a,ya,k);H=old.literal(f,A,B,z,y,h,k,r,s)
 Q,rem=divmod_poly(H,[f.square(a),0,1],f)
 check(not rem and len(Q)==4 and Q[-1]==1,'exact doubled-source quotient')
 check(Q[2]==f.square(h)^h^z,'fixed cubic first coefficient')
 return list(Q)

def spans(f,vs):
 if vs==[1<<i for i in range(len(vs))]:return old.product_spans(f,len(vs))[:3]
 out=[]
 for j in range(1,4):
  values=[]
  for tup in itertools.product(vs,repeat=j):
   p=1
   for x in tup:p=f.mul(p,x)
   values.append(p)
  out.append(old.basis(values))
 return out

def affine(f,A,B,z,y,h,a,ya,vs):
 base=quotient(f,A,B,z,y,h,a,ya,0);cols=[]
 for bit in range(f.degree):cols.append([u^v for u,v in zip(quotient(f,A,B,z,y,h,a,ya,1<<bit),base)])
 W=spans(f,vs)
 def pack(Q):return old.residual(Q[1],W[1])|(old.residual(Q[0],W[2])<<f.degree)
 rhs=pack(base);packed=[pack(c)for c in cols]
 rows=[(sum(((v>>bit)&1)<<i for i,v in enumerate(packed)),(rhs>>bit)&1)for bit in range(2*f.degree)]
 return base,cols,rows,old.solve(rows,f.degree),W

def family_states(sol):
 if not sol['consistent']:return []
 return [sol['particular']^old.xor(v for i,v in enumerate(sol['basis'])if t>>i&1)for t in range(1<<sol['nullity'])]

def components(rows,n):
 # Exact binary incidence, including Frobenius-square effects via full columns.
 adj=[set()for _ in range(n)]
 for m,_ in rows:
  ids=[i for i in range(n)if m>>i&1]
  if ids:
   first=ids[0]
   for i in ids[1:]:adj[first].add(i);adj[i].add(first)
 todo=set(range(n));parts=[]
 while todo:
  start=min(todo);todo.remove(start);comp={start};stack=[start]
  while stack:
   for j in adj[stack.pop()]&todo:todo.remove(j);comp.add(j);stack.append(j)
  parts.append(sorted(comp))
 # Every nonzero row is confined to exactly one component; zero rows retained.
 nonzero=[(m,r)for m,r in rows if m]
 check(all(any(all(not(m>>i&1)or i in part for i in range(n))for part in parts)for m,_ in nonzero),'block-diagonal incidence certificate')
 return dict(columns=n,rows=len(rows),nonzero_rows=len(nonzero),nonzeros=sum(m.bit_count()for m,_ in rows),density=sum(m.bit_count()for m,_ in rows)/(len(rows)*n),active_row_density=sum(m.bit_count()for m,_ in rows)/(max(1,len(nonzero))*n),column_components=parts,component_sizes=[len(c)for c in parts],general_btf_tested=False)

def sparse_solve(rows,n,parts):
 if any(not m and r for m,r in rows):return dict(consistent=False,rank=None,nullity=None,particular=None,basis=[])
 results=[];rank=0;particular=0;kernel=[];consistent=True
 for part in parts:
  mask=sum(1<<i for i in part)
  local=[(sum(((m>>v)&1)<<j for j,v in enumerate(part)),r)for m,r in rows if m&mask]
  sol=old.solve(local,len(part));rank+=sol['rank'];consistent &=sol['consistent']
  def lift(x):return sum(((x>>j)&1)<<v for j,v in enumerate(part))
  if sol['consistent']:particular^=lift(sol['particular'])
  kernel.extend(lift(x)for x in sol['basis'])
 return dict(consistent=consistent,rank=rank,nullity=n-rank,particular=particular if consistent else None,basis=kernel)

def compare_families(rows,dense,sparse):
 check(dense['consistent']==sparse['consistent'],'dense/sparse consistency')
 if not dense['consistent']:return
 check(dense['rank']==sparse['rank'] and len(sparse['basis'])==dense['nullity'],'dense/sparse dimensions')
 check(all(((m&sparse['particular']).bit_count()&1)==r for m,r in rows),'sparse particular')
 check(all(all((m&v).bit_count()%2==0 for m,_ in rows)for v in sparse['basis']),'sparse kernel')
 check(len(old.basis(sparse['basis']))==dense['nullity'],'sparse kernel independent complete')

def supported(f,Q,vs):
 # Build L_V modulo Q by exact subspace-polynomial recurrence. Scalar lambda
 # values use a sparse linearized polynomial, never materialize 2^d terms.
 R=(0,1);coeff=[1]
 for v in vs:
  power=v;lam=0
  for c in coeff:lam^=f.mul(c,power);power=f.square(power)
  check(lam!=0,'independent source basis')
  sq=powmod_poly(R,2,Q,f);R=tuple(old.padd(sq,[f.mul(lam,x)for x in R]));R=divmod_poly(R,Q,f)[1]
  nxt=[0]*(len(coeff)+1)
  for j,c in enumerate(coeff):nxt[j]^=f.mul(lam,c);nxt[j+1]^=f.square(c)
  coeff=nxt
 return not powmod_poly(R,4,Q,f)

def recover(f,Q,a,vs):
 roots=binary_field_roots(Q,f,max_trials=128)
 if roots['status']!='complete':return dict(status='unknown_factorization',roots=roots,sources=[])
 mult=[];left=tuple(Q)
 for x in roots['roots']:
  check(old.residual(x,old.basis(vs))==0,'recovered source lies in V')
  while len(left)>1:
   q,r=divmod_poly(left,[x,1],f)
   if r:break
   mult.append(x);left=q
 check(left==(1,)and len(mult)==3,'complete multiplicity recovery')
 sources=sorted(set(itertools.permutations(mult+[a,a])))
 return dict(status='complete',roots=roots,multiset=sorted(mult+[a,a]),sources=[list(t)for t in sources])

def points(f,A,B):return [(x,y)for x in range(f.order)for y in range(f.order)if f.square(y)^f.mul(x,y)==f.mul(f.square(x),x)^f.mul(A,f.square(x))^B]

def controls():
 records=[];raw_states=0;positive=0;normcells=0
 for n,mod,vs,A in [(2,7,[2],0),(3,11,[1,2],1),(4,19,[1,2],0)]:
  f=BinaryField(n,mod);V=[old.xor(v for i,v in enumerate(vs)if t>>i&1)for t in range(1<<len(vs))];pts=points(f,A,1)
  targets=[p for p in pts if p[0]not in V]
  if not targets:records.append(dict(n=n,skipped='no target outside frozen V'));continue
  z,y=targets[0];hs=[h for h in range(f.order)if f.square(h)^h^z in V]
  localpos=0;localcells=0
  for a,ya in [p for p in pts if p[0]in V and p[0]!=0][:2]:
   for h in hs[:2]:
    base,cols,rows,sol,W=affine(f,A,1,z,y,h,a,ya,vs);structure=components(rows,n);sp=sparse_solve(rows,n,structure['column_components']);compare_families(rows,sol,sp)
    expected=[];actual=[];supported_ks=[]
    for k in range(f.order):
     r,s=params(f,z,y,h,a,ya,k);Q=quotient(f,A,1,z,y,h,a,ya,k)
     predicted=[base[j]^old.xor(c[j]for bit,c in enumerate(cols)if k>>bit&1)for j in range(4)]
     check(Q==predicted,'complete tiny affine replay')
     admitted=old.residual(Q[1],W[1])==0 and old.residual(Q[0],W[2])==0
     if admitted:expected.append(k)
     # Independent exhaustive source-factor oracle on tiny domain.
     possible=[tuple(t)for t in itertools.combinations_with_replacement(V,3)if old.pmul(old.pmul([t[0],1],[t[1],1],f),[t[2],1],f)==Q]
     check(supported(f,Q,vs)==bool(possible),'nilpotent support versus full multiset oracle')
     if possible and f.mul(h,a)^k:
      localpos+=1;positive+=1;supported_ks.append(k);rec=recover(f,Q,a,vs)
      check(rec['status']=='complete' and rec['multiset']==sorted(list(possible[0])+[a,a]),'complete tiny multiplicity/source recovery')
      check(rec['sources']==[list(t)for t in sorted(set(itertools.permutations(list(possible[0])+[a,a])))],'all ordered tiny source tuples')
     # Exhaust every r,s and check regular double criterion independently.
     if f.mul(h,a)^k:
      target=[]
      for rr,ss in itertools.product(range(f.order),repeat=2):
       H=old.literal(f,A,1,z,y,h,k,rr,ss);t=f.mul(f.square(z),z)^f.mul(rr,f.square(z))^f.mul(ss,z)^f.mul(f.mul(h,z)^k,y)
       Fval=t^f.mul(ss,a)^f.mul(rr,f.square(a))^f.mul(f.square(a),a)
       _,rem=divmod_poly(H,[f.square(a),0,1],f)
       if Fval==f.mul(f.mul(h,a)^k,ya)and not rem:target.append((rr,ss))
       raw_states+=1;check(raw_states<=200000,'raw tiny state cap')
      check(target==[(r,s)],'all r/s regular norm-root derivative oracle')
    check(set(family_states(sol))==set(expected),'complete k coefficient relaxation')
    records.append(dict(n=n,a=a,ya=ya,h=h,z=z,rank=sol['rank'],nullity=sol['nullity'],consistent=sol['consistent'],positive_regular_norm_ks=supported_ks,structure=structure));localcells+=1;normcells+=1
  check(localcells>0,'nonvacuous regular norm cells in each field')
  records.append(dict(n=n,positive_regular_norm_parameters=localpos,norm_cells=localcells))
 check(positive>0,'positive admitted regular norm controls required')
 return dict(records=records,raw_r_s_k_states=raw_states,positive_regular_norm_parameters=positive,norm_cells=normcells,zero_a_branch='unknown',common_factor_branch='unknown',scope='fixed h,a,ya regular norm branch only')

def profile():
 data=json.loads((ROOT/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());spec=data['extension_field'];records=[]
 for rep in range(3):
  begin=time.monotonic();f=BinaryField(spec['degree'],int(spec['modulus_hex'],16));A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400;a=1;ya=0;vs=[1<<i for i in range(18)]
  check(f.degree==83 and f.square(ya)^f.mul(a,ya)==f.mul(f.square(a),a)^f.mul(A,f.square(a))^B,'actual source/field identity');check(z>=1<<18 and (f.square(h)^h^z)<1<<18,'actual target/admitted h')
  check([int(v,16)for v in data['source_basis_hex']]==vs and int(data.get('source_offset_hex','0'),16)==0,'actual source identity')
  check(B!=0 and a!=z and f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B,'actual target and guards')
  base,cols,rows,sol,W=affine(f,A,B,z,y,h,a,ya,vs);check([len(w)for w in W]==[18,35,52],'actual product spans')
  structure=components(rows,83);sp=sparse_solve(rows,83,structure['column_components']);compare_families(rows,sol,sp)
  candidates=[];complete=True;regular_count=None;support_count=None
  if sol['consistent'] and sol['nullity']>12:complete=False
  else:
   states=family_states(sol);regular_count=0;support_count=0
   for k in states:
    if k==h:continue
    regular_count+=1;Q=quotient(f,A,B,z,y,h,a,ya,k)
    if supported(f,Q,vs):
     support_count+=1;rec=recover(f,Q,a,vs);complete &=rec['status']=='complete';candidates.append(dict(k=k,Q=Q,recovery=rec))
  records.append(dict(rep=rep,seconds=time.monotonic()-begin,family=sol,rows=rows,base=base,columns=cols,structure=structure,regular_candidates=regular_count,supported_parameters=support_count,complete_regular_norm_fiber=complete,candidates=candidates))
 return dict(field_degree=83,h=h,a=a,ya=ya,excluded_common_factor_k=h,records=records,median_seconds=statistics.median(r['seconds']for r in records),repetitions='same process, not cold; field irreducibility cache may be warm',global_original_complete=False,original_root_count_exact=None)

def main():
 ident,mode=sys.argv[1:];assert mode in ['controls','profile'];out={};status='passed';error=None
 try:out=controls()if mode=='controls'else profile()
 except Exception as exc:status='inconclusive';error=type(exc).__name__+': '+str(exc)
 raw=dict(mode=mode,out=out,predicates=CHECKS,error=error);blob=gzip.compress(json.dumps(raw,sort_keys=True,separators=(',',':')).encode(),mtime=0)
 with(P/('raw-'+mode+'.json.gz')).open('xb')as s:s.write(blob)
 evidence=dict(details=out,predicates=CHECKS,error=error,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_component=status=='passed' and (mode=='controls' or all(r['complete_regular_norm_fiber']for r in out.get('records',[]))),complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Regular doubled-source free-k affine fiber and exact incidence-component probe; no global original-root or speedup claim.',evidence=evidence,next_actions=['If dense connected, investigate matched structural SCC/BTF or alternate source-basis ordering; preserve common-factor/zero-a and all other h/a/y branches.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:
  result['status']='inconclusive';evidence['complete_conditional_component']=False;evidence['error']='late serialized report'
  if mode=='profile':
   for record in out.get('records',[]):
    record['complete_regular_norm_fiber']=False;record['regular_candidates']=None;record['supported_parameters']=None
  text=json.dumps(result)
 assert time.monotonic()-START<88,'final reporting deadline';print(text)
if __name__=='__main__':main()
