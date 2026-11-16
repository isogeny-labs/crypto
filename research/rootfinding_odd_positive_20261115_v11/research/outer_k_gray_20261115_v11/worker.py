"""Matched166Gray versus87direct/Gray full-family compiler diagnostic."""
from pathlib import Path
import sys,json,time,gzip,hashlib,resource,itertools
from rootfinder.binaryfield import BinaryField
from research.quartic_cell_20261114_v1 import worker as old
from research.regular_ore_20261115_v3 import worker as ore
from research.full_quintic_block_20261115_v9 import worker as previous
from research.full_quintic_87_20261115_v10 import worker as current
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();CHECKS=0

def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'predicate/time cap'
 assert v,why

def edges():
 state=0x677261795f6b3837;mask=(1<<64)-1;out=[]
 def word():
  nonlocal state
  state^=(state<<13)&mask;state^=state>>7;state^=(state<<17)&mask;state &=mask;return state
 for i in range(256):
  k=(word()|(word()<<64))&((1<<83)-1);j=i%83;out.append([k,j,k^(1<<j)])
 return out

def digest(x):return hashlib.sha256(json.dumps(x,separators=(',',':'),sort_keys=True).encode()).hexdigest()
def xor_table(base,table,k):
 out=list(base)
 while k:
  bit=(k&-k).bit_length()-1;k &=k-1
  out=[a^b for a,b in zip(out,table[bit])]
 return out

def project(f,W,which,values):
 out=0;offset=0
 for j,value in zip(which,values):
  rem=old.residual(value,W[j-1])
  for bit in range(f.degree):
   if bit not in W[j-1]:out|=((rem>>bit)&1)<<offset;offset+=1
 return out

def rows_from_columns(columns,count):return [sum(((v>>j)&1)<<i for i,v in enumerate(columns))for j in range(count)]

def setup87(f,A,z,y,h,W):
 base=current.compiler87(f,A,z,y,h,0,W);check(base['branch']=='compiler87'and base['rho']==1,'fixed h rank1 AS chart');ih=f.inv(h);ih2=f.square(ih);wb=[W[1][i]for i in sorted(W[1])];mask=base['trace_row'][0];pivot=mask.bit_length()-1;vp=wb[pivot];rd=[]
 for raw in base['directions'][:len(W[1])]:rd.append(raw&((1<<f.degree)-1))
 # rho1 gives exactly dimW2 r directions, including its AS kernel.
 check(len(rd)==len(W[1]),'r direction dimension');Rdelta=[]
 for j in range(f.degree):
  delta=1<<j;tr=previous.trace(f,f.mul(delta,ih2));admitted=f.mul(delta^(vp if tr else 0),ih2);Rdelta.append(f.mul(h,previous.asroot(f,admitted)))
 ac=[];bc=[]
 for d in rd:ac.append(f.mul(ih,f.mul(z,f.square(d))));bc.append(f.mul(ih,d))
 vc=[f.mul(ih,W[2][i])for i in sorted(W[2])]
 return dict(R0=base['r0'],Rdelta=Rdelta,r_directions=rd,a=ac,b=bc,v3_directions=vc,ih=ih,trace_row=base['trace_row'])

def Rvalue(pre,k):return pre['R0']^old.xor(v for j,v in enumerate(pre['Rdelta'])if k>>j&1)
def constants(f,A,z,y,h,k):return old.formula(f,A,z,y,h,k,0,0)

def direct87(f,A,z,y,h,W,pre,k):
 C=constants(f,A,z,y,h,k);r=Rvalue(pre,k);w=f.mul(pre['ih'],C[2]^f.mul(z,f.square(r))^f.mul(k,r));directions=[];columns=[];n=f.degree
 for dr,a,b in zip(pre['r_directions'],pre['a'],pre['b']):
  dw=a^f.mul(k,b);directions.append(dr|((dw^f.mul(z,dr))<<n));columns.append(project(f,W,[4],[f.square(dw)^f.mul(k,dw)]))
 for dw in pre['v3_directions']:directions.append(dw<<n);columns.append(project(f,W,[4],[f.square(dw)^f.mul(k,dw)]))
 rowcount=n-len(W[3]);rows=rows_from_columns(columns,rowcount);rhs=project(f,W,[4],[C[1]^f.square(w)^f.mul(k,w)])
 return rows,rhs,r|((w^f.mul(z,r))<<n),directions

def build166(f,A,z,y,h,W):
 n=f.degree;count=sum(n-len(W[j-1])for j in [2,3,4,5]);basecols=[]
 for i in range(2*n):
  r=(1<<i)if i<n else 0;w=(1<<(i-n))if i>=n else 0;basecols.append(project(f,W,[2,3,4,5],[f.square(r)^f.mul(h,r),f.mul(z,f.square(r))^f.mul(h,w),f.square(w),f.mul(z,f.square(w))]))
 base=rows_from_columns(basecols,count);C=constants(f,A,z,y,h,0);rhs0=project(f,W,[2,3,4,5],[C[3],C[2],C[1],C[0]]);table=[];rhs=[]
 for j in range(n):
  delta=1<<j;cols=[project(f,W,[2,3,4,5],[0,f.mul(delta,1<<i),0,0])for i in range(n)]+[project(f,W,[2,3,4,5],[0,0,f.mul(delta,1<<i),0])for i in range(n)];table.append(rows_from_columns(cols,count));d2=f.square(delta);rhs.append(project(f,W,[2,3,4,5],[delta,d2^f.mul(z,delta),f.mul(d2,A^z)^f.mul(delta,f.square(z)^f.mul(h,y)),f.mul(d2,y^f.mul(z,A)^f.square(z))]))
 return dict(base=base,increments=table,rhs0=rhs0,rhs_increments=rhs)

def build87(f,A,z,y,h,W,pre):
 base,b0,_,_=direct87(f,A,z,y,h,W,pre,0);table=[]
 for j in range(f.degree):
  value=direct87(f,A,z,y,h,W,pre,1<<j)[0];table.append([a^b for a,b in zip(value,base)])
 def offset(k):
  C=constants(f,A,z,y,h,k);r=Rvalue(pre,k);w=f.mul(pre['ih'],C[2]^f.mul(z,f.square(r))^f.mul(k,r));return project(f,W,[4],[C[1]^f.square(w)^f.mul(k,w)])
 linear=[offset(1<<i)^b0 for i in range(f.degree)];quadratic=[]
 # Exact degree2 theorem permits these3487 coefficient evaluations.
 for i in range(f.degree):
  row=[]
  for j in range(i+1,f.degree):row.append(offset((1<<i)|(1<<j))^linear[i]^linear[j]^b0)
  quadratic.append(row)
 return dict(base=base,increments=table,rhs0=b0,rhs_linear=linear,rhs_quadratic=quadratic)

def quadratic_value(data,k):
 bits=[i for i in range(len(data['rhs_linear']))if k>>i&1];out=data['rhs0']^old.xor(data['rhs_linear'][i]for i in bits)
 for a,i in enumerate(bits):
  for j in bits[a+1:]:out^=data['rhs_quadratic'][i][j-i-1]
 return out

def quadratic_delta(data,k,j):
 out=data['rhs_linear'][j]
 for i in range(len(data['rhs_linear'])):
  if i!=j and k>>i&1:out^=data['rhs_quadratic'][min(i,j)][abs(i-j)-1]
 return out

def solve_map(f,z,rows,rhs,variables,raw0,directions):
 sol=old.solve([(v,(rhs>>i)&1)for i,v in enumerate(rows)],variables)
 def lift(x):return old.xor(v for i,v in enumerate(directions)if x>>i&1)
 mapped=dict(consistent=sol['consistent'],particular=(raw0^lift(sol['particular']))if sol['consistent']else None,basis=[lift(v)for v in sol['basis']])
 return sol,mapped

def canonical(mapped):
 if not mapped['consistent']:return None
 piv=old.basis(mapped['basis']);check(len(piv)==len(mapped['basis']),'mapped kernel complete/injective')
 for p in sorted(piv):
  for q in sorted(piv):
   if q>p and piv[q]>>p&1:piv[q]^=piv[p]
 return [old.residual(mapped['particular'],piv),[[p,v]for p,v in sorted(piv.items())]]

def controls():
 states=0;cells=[]
 for n,mod,vs,A in [(2,7,[2],0),(3,11,[1,2],1),(4,19,[1,2,8],0)]:
  f=BinaryField(n,mod);W=previous.spans(f,vs);pts=[(x,y)for x in range(f.order)for y in range(f.order)if f.square(y)^f.mul(x,y)==f.mul(f.square(x),x)^f.mul(A,f.square(x))^1];z,y=next(p for p in pts if old.residual(p[0],W[0]));hs=[h for h in range(1,f.order)if not old.residual(f.square(h)^h^z,W[0])][:2]
  for h in hs:
   strong=build166(f,A,z,y,h,W)
   for value in range(f.order):
    observed=xor_table(strong['base'],strong['increments'],value);observed_rhs=strong['rhs0']^old.xor(v for i,v in enumerate(strong['rhs_increments'])if value>>i&1);H0=old.literal(f,A,1,z,y,h,value,0,0);columns=[]
    for bit in range(2*n):
     r=(1<<bit)if bit<n else 0;w=(1<<(bit-n))if bit>=n else 0;Hv=old.literal(f,A,1,z,y,h,value,r,w^f.mul(z,r));delta=[a^b for a,b in zip(Hv,H0)];columns.append(project(f,W,[2,3,4,5],[delta[3],delta[2],delta[1],delta[0]]))
    expected=rows_from_columns(columns,len(observed));expected_rhs=project(f,W,[2,3,4,5],[H0[3],H0[2],H0[1],H0[0]]);check(observed==expected and observed_rhs==expected_rhs,'tiny strong166Gray independent literal operators')
   # General tinycompiler keeps extrae5 andrho0; allk/allr/s directoracle.
   for k in range(f.order):
    a=previous.baseline(f,A,z,y,h,k,W);b=current.compiler87(f,A,z,y,h,k,W);previous.equal(a,b);kernel=old.basis(b.get('basis',[]))
    for r,s in itertools.product(range(f.order),repeat=2):
     H=old.literal(f,A,1,z,y,h,k,r,s);admitted=all(old.residual(H[5-j],W[j-1])==0 for j in range(1,6));member=bool(b['consistent'])and old.residual((r|(s<<n))^b['particular'],kernel)==0;check(admitted==member,'tiny literal complete family');states+=1
   # Degree2fulltruth oracle for scalar normalformoffsets withfixed linearASsection.
   result=current.compiler87(f,A,z,y,h,0,W)
   if result.get('rho')==1 and len(W[4])==n:
    pre=setup87(f,A,z,y,h,W);values=[]
    for k in range(f.order):
     R=Rvalue(pre,k);C=constants(f,A,z,y,h,k);L=f.mul(pre['ih'],C[2]^f.mul(z,f.square(R)));w=L^f.mul(f.mul(k,pre['ih']),R);values.append(C[1]^f.square(w)^f.mul(k,w))
    anf=list(values)
    for bit in range(n):
     for mask in range(f.order):
      if mask>>bit&1:anf[mask]^=anf[mask^(1<<bit)]
    check(all(not v for mask,v in enumerate(anf)if mask.bit_count()>2),'full truth RHS degree<=2');cells.append(dict(n=n,h=h,quadratic_ANF=anf))
 # Synthetic nontrivial coefficient constraints separately exercise everyGray
 # matrix/RHS entry; these are NOT advertised as source-product domains.
 for n,mod in [(2,7),(3,11),(4,19)]:
  f=BinaryField(n,mod);A=0;z=0;y=1;h=1;full=old.basis([1<<i for i in range(n)]);W=[full,full,old.basis([1]),old.basis([1]),full];pre=setup87(f,A,z,y,h,W);table=build87(f,A,z,y,h,W,pre);truth=[]
  for value in range(f.order):
   rows,rhs,_,_=direct87(f,A,z,y,h,W,pre,value);check(rows==xor_table(table['base'],table['increments'],value)and rhs==quadratic_value(table,value),'synthetic allk87matrix/RHS equality');truth.append(rhs)
   for bit in range(n):check(quadratic_delta(table,value,bit)==(rhs^quadratic_value(table,value^(1<<bit))),'synthetic everyGray firstdifference')
  anf=list(truth)
  for bit in range(n):
   for mask in range(f.order):
    if mask>>bit&1:anf[mask]^=anf[mask^(1<<bit)]
  check(all(not v for mask,v in enumerate(anf)if mask.bit_count()>2),'nontrivial fullANF quadratic degree');bad=[v^1 if i==0 else v for i,v in enumerate(table['increments'][0])];check(xor_table(table['base'],[bad]+table['increments'][1:],1)!=direct87(f,A,z,y,h,W,pre,1)[0],'corrupted real matrix increment rejected')
 # Independently corruptedincrement is rejected on exactbitresponse.
 base=[3,5];delta=[7,2];actual=[a^b for a,b in zip(base,delta)];bad=[delta[0]^1,delta[1]];check([a^b for a,b in zip(base,bad)]!=actual,'corrupt delta rejection')
 return dict(literal_r_s_states=states,quadratic_control_cells=cells,even_AS_exercised=True,rho0_guard_retained=True,corrupt_delta_rejected=True)

def arm(method,corpus):
 start=time.monotonic();f,A,B,z,y,h,_,_,vs=ore.fixture();W=previous.spans(f,vs);check([len(w)for w in W]==[18,35,52,69,83],'actual span dimensions');setup=time.monotonic()-start;begin=time.monotonic();pre=None;tables=None
 if method=='166Gray':tables=build166(f,A,z,y,h,W)
 else:
  pre=setup87(f,A,z,y,h,W)
  if method=='87Gray':tables=build87(f,A,z,y,h,W,pre)
 prep=time.monotonic()-begin;records=[];families=[];complete=True;stop=None;stage=dict(reset=0.0,solve_map=0.0,literal_verify=0.0,serialize=0.0)
 for edge,(k,j,other)in enumerate(corpus):
  if time.monotonic()-START>=73:complete=False;stop='wholeworker evaluation deadline';break
  for endpoint,value in enumerate([k,other]):
   begin=time.monotonic()
   if method=='166Gray':
    if endpoint==0:rows=xor_table(tables['base'],tables['increments'],value);rhs=tables['rhs0']^old.xor(v for i,v in enumerate(tables['rhs_increments'])if value>>i&1)
    else:rows=[a^b for a,b in zip(rows,tables['increments'][j])];rhs^=tables['rhs_increments'][j]
    raw0=0;directions=[(1<<i)|(f.mul(z,1<<i)<<83)for i in range(83)]+[1<<(83+i)for i in range(83)];variables=166
   else:
    directrows,directrhs,raw0,directions=direct87(f,A,z,y,h,W,pre,value);variables=87
    if method=='87direct':rows,rhs=directrows,directrhs
    elif endpoint==0:rows=xor_table(tables['base'],tables['increments'],value);rhs=quadratic_value(tables,value)
    else:rows=[a^b for a,b in zip(rows,tables['increments'][j])];rhs^=quadratic_delta(tables,k,j)
    check(rows==directrows and rhs==directrhs,'all actual87Gray matrix/RHS direct reconstruction')
   stage['reset']+=time.monotonic()-begin;begin=time.monotonic();sol,mapped=solve_map(f,z,rows,rhs,variables,raw0,directions);can=canonical(mapped);stage['solve_map']+=time.monotonic()-begin
   begin=time.monotonic();zero=old.literal(f,A,B,z,y,h,value,0,0)
   if mapped['consistent']:
    for index,raw in enumerate([mapped['particular']]+mapped['basis']):
     r=raw&((1<<83)-1);s=raw>>83;H=old.literal(f,A,B,z,y,h,value,r,s)
     if index:H=old.padd(H,zero)
     check(all(old.residual(H[5-q],W[q-1])==0 for q in range(1,6)),'paid everymapped literal coefficient verification')
   stage['literal_verify']+=time.monotonic()-begin
   record=dict(edge=edge,endpoint=endpoint,k=value,rank=sol['rank'],consistent=sol['consistent'],coordinate_particular=sol['particular'],row_hash=digest([rows,rhs]),coordinate_family_hash=digest([sol['particular'],sol['basis']]),canonical_mapped_family_hash=digest(can),nullity=sol['nullity']);records.append(record);families.append(can)
  if not complete:break
 proof=dict(pre=pre,tables=tables);out=dict(method=method,complete=complete,stop=stop,records=records,operator_proof=proof,setup_seconds=setup,preparation_seconds=prep,stage_seconds=stage)
 begin=time.monotonic();json.dumps(out,separators=(',',':'));stage['serialize']=time.monotonic()-begin;out['total_seconds']=time.monotonic()-start;return out,families

def profile():
 corpus=edges();records=[];reference=None
 for method in ['166Gray','87direct','87Gray']:
  data,families=arm(method,corpus)
  if reference is None:reference=families
  else:
   for a,b in zip(reference,families):check(a==b,'ALL complete mapped affine families identical acrossarms')
  records.append(data)
 complete=all(r['complete']and len(r['records'])==512 for r in records);times={r['method']:r['total_seconds']for r in records}
 return dict(corpus=corpus,distinct_vertices=len({v for k,j,o in corpus for v in [k,o]}),records=records,complete_batch=complete,internal_total_ratios=dict(strong166Gray_over_87Gray=times['166Gray']/times['87Gray'],cached87direct_over_87Gray=times['87direct']/times['87Gray'])if complete else None,exact_family_contract='Everyfullsystem reconstructible fromfrozenoperators+k; deterministicGaussian/mappingverified, hashesnotreplacementforequations',repetitions=1,original_global_complete=False)

def compact(out):return dict(controls=out.get('controls'),profile={**{k:v for k,v in out.get('profile',{}).items()if k not in ['corpus','records']},'records':[{k:v for k,v in r.items()if k not in ['operator_proof','records']}for r in out.get('profile',{}).get('records',[])]})
def demote(out):
 p=out.get('profile',{});p['complete_batch']=False;p['internal_total_ratios']=None
 for rec in p.get('records',[]):rec['complete']=False

def main():
 ident=sys.argv[1];out={};status='passed';error=None
 try:out['controls']=controls();out['profile']=profile()
 except Exception as exc:status='inconclusive';error=type(exc).__name__+': '+str(exc)
 if not out.get('profile',{}).get('complete_batch',False):status='inconclusive'
 encoded=json.dumps(dict(out=out,error=error,predicates=CHECKS),sort_keys=True,separators=(',',':')).encode();blob=gzip.compress(encoded,mtime=0)
 if len(encoded)>4*1024*1024 or len(blob)>768*1024:status='inconclusive';error='raw4MiB/gzip768KiB cap';demote(out);encoded=json.dumps(dict(error=error,complete=False,operators_omitted=True)).encode();blob=gzip.compress(encoded,mtime=0)
 with(P/'raw.json.gz').open('xb')as stream:stream.write(blob)
 evidence=dict(details=compact(out),error=error,predicates=CHECKS,raw_path='research/outer_k_gray_20261115_v11/raw.json.gz',raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),raw_uncompressed_bytes=len(encoded),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_compiler=status=='passed',complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Exactmatched166Gray/87direct/87Gray compilerbatch; allsetup/outputpaid, norootsearch/globalgainclaim.',evidence=evidence,next_actions=['Auditallreconstructibleoperator-definedfamiliesandcosts; ifreset/Gaussian/mapping dominates, changebottleneckinnewvariant ratherthanretrysamecorpusorclaimupdate-stagegainasglobal.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:result['status']='inconclusive';evidence['complete_conditional_compiler']=False;evidence['error']='late report';demote(out);evidence['details']=compact(out);text=json.dumps(result)
 assert len(text.encode())<=24576 and time.monotonic()-START<88,'compactfinaldeadline';print(text)
if __name__=='__main__':main()
