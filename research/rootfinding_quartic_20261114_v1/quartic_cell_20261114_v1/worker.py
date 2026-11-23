"""Exact conditional affine/quartic branch admission; coordinator executes."""
from pathlib import Path
import sys,json,time,gzip,hashlib,itertools,resource
from rootfinder.binaryfield import BinaryField
P=Path(__file__).resolve().parent; ROOT=P.parents[1]; START=time.monotonic();CHECKS=0
def check(v,why):
 global CHECKS
 CHECKS+=1
 assert CHECKS<=200000 and time.monotonic()-START<80,'predicate/time cap'
 assert v,why
def xor(values):
 out=0
 for v in values:out^=v
 return out
def basis(values):
 piv={}
 for value in values:
  for p in sorted(piv,reverse=True):
   if value>>p&1:value^=piv[p]
  if value:piv[value.bit_length()-1]=value
 return piv
def residual(value,piv):
 for p in sorted(piv,reverse=True):
  if value>>p&1:value^=piv[p]
 return value
def pmul(a,b,f):
 out=[0]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):out[i+j]^=f.mul(x,y)
 return out
def padd(a,b):return [(a[i]if i<len(a)else 0)^(b[i]if i<len(b)else 0)for i in range(max(len(a),len(b)))]
def formula(f,A,z,y,h,k,r,s):
 h2,k2,r2,s2,z2=[f.square(v)for v in [h,k,r,s,z]];z3=f.mul(z2,z);z4=f.square(z2)
 e1=h2^h^z
 e2=r2^f.mul(h,r)^k^f.mul(h2,A)^f.mul(z,e1)
 e3=f.mul(h,s)^f.mul(k,r)^k2^f.mul(z,e2)
 e4=s2^f.mul(k,s)^f.mul(z,f.mul(k,r))^f.mul(z2,r2)^f.mul(k2,A^z)^f.mul(z2,k)^f.mul(h2,f.square(y))^f.mul(f.mul(h,k),y)^z4
 e5=f.mul(z,s2)^f.mul(z3,r2)^f.mul(k2,y^f.mul(z,A)^z2)^f.mul(z,f.mul(h2,f.square(y)))^f.mul(z4,z)
 return [e5,e4,e3,e2,e1,1]
def literal(f,A,B,z,y,h,k,r,s):
 t=f.mul(f.square(z),z)^f.mul(r,f.square(z))^f.mul(s,z)^f.mul(f.mul(h,z)^k,y)
 F=[t,s,r,1];G=[k,h]
 norm=padd(padd(pmul(F,F,f),pmul(pmul([0,1],F,f),G,f)),pmul([B,0,A,1],pmul(G,G,f),f))
 norm+= [0]*(7-len(norm));out=[0]*6;out[5]=norm[6]
 for i in range(5,0,-1):out[i-1]=norm[i]^f.mul(z,out[i])
 check(norm[0]^f.mul(z,out[0])==0 and out[5]==1,'literal norm monic division')
 return out
def translate(H,b,f):
 out=[0]
 for c in reversed(H):out=padd(pmul(out,[b,1],f),[c])
 return out[:6]
def peval(H,x,f):
 out=0
 for c in reversed(H):out=f.mul(out,x)^c
 return out
def value(base,columns,t):return [base[j]^xor(col[j]for i,col in enumerate(columns)if t>>i&1)for j in range(6)]
def linear_rows(base,columns,pack,width):
 rhs=pack(base);cols=[pack(c)for c in columns]
 return [(sum(((v>>bit)&1)<<i for i,v in enumerate(cols)),(rhs>>bit)&1)for bit in range(width)]
def solve(rows,n):
 check(n<=166 and len(rows)<=500,'linear stage admission')
 piv={};contradiction=None
 for index,(mask,rhs)in enumerate(rows):
  combo=1<<index
  while mask:
   p=mask.bit_length()-1
   if p not in piv:piv[p]=(mask,rhs,combo);break
   m,r,c=piv[p];mask^=m;rhs^=r;combo^=c
  if not mask and rhs and contradiction is None:contradiction=combo
 free=[i for i in range(n)if i not in piv]
 def back(x,homogeneous):
  for p,(mask,rhs,_)in sorted(piv.items()):
   if ((mask&x).bit_count()&1)^(0 if homogeneous else rhs):x^=1<<p
  return x
 particular=None if contradiction is not None else back(0,False)
 kernel=[back(1<<i,True)for i in free]
 result=dict(consistent=contradiction is None,rank=len(piv),nullity=len(free),particular=particular,basis=kernel,contradiction=contradiction,pivots=[list(v)for _,v in sorted(piv.items())])
 verify(rows,n,result)
 return result
def verify(rows,n,result):
 for mask,rhs,combo in result['pivots']:
  check(xor(rows[i][0]for i in range(len(rows))if combo>>i&1)==mask and xor(rows[i][1]for i in range(len(rows))if combo>>i&1)==rhs,'pivot row-combination replay')
 c=result['contradiction']
 if c is not None:check(xor(rows[i][0]for i in range(len(rows))if c>>i&1)==0 and xor(rows[i][1]for i in range(len(rows))if c>>i&1)==1,'inconsistency certificate')
 else:check(all(((m&result['particular']).bit_count()&1)==r for m,r in rows),'particular replay')
 check(len(result['basis'])==n-result['rank'],'full kernel dimension')
 check(all(all((m&v).bit_count()%2==0 for m,_ in rows)for v in result['basis']),'kernel replay')
 check(len(basis(result['basis']))==len(result['basis']),'independent kernel')
def map_for(f,A,z,y,h,k):
 base=formula(f,A,z,y,h,k,0,0);cols=[]
 for bit in range(2*f.degree):
  v=formula(f,A,z,y,h,k,(1<<bit)if bit<f.degree else 0,(1<<(bit-f.degree))if bit>=f.degree else 0)
  cols.append([a^b for a,b in zip(v,base)])
 return base,cols
def product_spans(f,d):
 # Every degree0..j(d-1) monomial is a product of j low-prefix monomials.
 result=[]
 for j in range(1,6):
  columns=[]
  for e in range(j*(d-1)+1):
   v=1<<e
   while v.bit_length()>f.degree:v^=f.modulus<<(v.bit_length()-f.degree-1)
   columns.append(v)
  result.append(basis(columns))
 return result
def relaxation(f,d,base,columns):
 spans=product_spans(f,d)
 def pack(H):return sum(residual(H[5-j],spans[j-1])<<((j-1)*f.degree)for j in range(1,6))
 rows=linear_rows(base,columns,pack,5*f.degree);family=solve(rows,2*f.degree)
 return spans,rows,family
def stratum(f,d,base,columns,shift):
 base=translate(base,shift,f);columns=[translate(c,shift,f)for c in columns]
 C=basis([f.square(f.square(1<<i))for i in range(d)])
 def pack(H):return H[4]|(H[3]<<f.degree)|(H[2]<<(2*f.degree))|(H[0]<<(3*f.degree))|(residual(H[1],C)<<(4*f.degree))
 rows=linear_rows(base,columns,pack,5*f.degree);result=solve(rows,len(columns))
 return dict(shift=shift,base=base,columns=columns,rows=rows,result=result,fourth_image_dimension=len(C))
def controls():
 summaries=[];total=0
 for n,modulus in [(3,0b1011),(4,0b10011)]:
  f=BinaryField(n,modulus);q=f.order
  # Independent coefficient-array convolution arithmetic oracle.
  for a,b in itertools.product(range(q),repeat=2):
   bits=[0]*(2*n-1)
   for i in range(n):
    for j in range(n):bits[i+j]^=((a>>i)&1)*((b>>j)&1)
   for i in range(2*n-2,n-1,-1):
    if bits[i]:
     for j in range(n+1):bits[i-n+j]^=(modulus>>j)&1
   check(f.mul(a,b)==sum(bits[i]<<i for i in range(n)),'independent field oracle')
  for d,h,k in itertools.product([1,2],[0,1,4],[0,1]):
   base,columns=map_for(f,0,0,1,h,k);spans,rows,family=relaxation(f,d,base,columns)
   for shift in [0,base[4]]:
    for H in [base]+columns:
     shifted=translate(H,shift,f)
     for x in range(q):check(peval(shifted,x,f)==peval(H,x^shift,f),'independent complete tiny translation evaluation')
   direct=[]
   for r,s in itertools.product(range(q),repeat=2):
    H=literal(f,0,1,0,1,h,k,r,s);t=r|(s<<n)
    check(H==formula(f,0,0,1,h,k,r,s)==value(base,columns,t),'literal/formula/affine exact equality')
    admitted=all(residual(H[5-j],spans[j-1])==0 for j in range(1,6))
    if admitted:direct.append(t)
    total+=1
   if family['consistent']:
    emitted={family['particular']^xor(v for i,v in enumerate(family['basis'])if t>>i&1)for t in range(1<<family['nullity'])}
    check(emitted==set(direct),'complete tiny relaxed family')
    rb=value(base,columns,family['particular']);rc=[[a^b for a,b in zip(value(base,columns,v),base)]for v in family['basis']]
    for shift in [0,rb[4]]:
     branch=stratum(f,d,rb,rc,shift);br=branch['result'];wanted=[]
     for t in range(1<<len(rc)):
      H=value(branch['base'],branch['columns'],t)
      if not any([H[4],H[3],H[2],H[0]]) and residual(H[1],basis([f.square(f.square(1<<i))for i in range(d)]))==0:wanted.append(t)
     got=set()if not br['consistent']else{br['particular']^xor(v for i,v in enumerate(br['basis'])if t>>i&1)for t in range(1<<br['nullity'])}
     check(got==set(wanted),'complete tiny conditional stratum family')
   else:check(not direct,'empty relaxed family certificate')
   summaries.append(dict(n=n,d=d,h=h,k=k,relaxed_rank=family['rank'],consistent=family['consistent']))
 # A one-equation contradiction and tampered proof are checked independently.
 test=solve([(0,1)],1);bad=dict(test,contradiction=0)
 try:verify([(0,1)],1,bad)
 except AssertionError:pass
 else:raise AssertionError('corrupted contradiction admitted')
 return dict(cells=summaries,direct_parameter_states=total,corrupted_certificate_rejected=True)
def profile():
 path=ROOT/'research/advance/instances/n83-d18-m5-T001-offset0.json';data=json.loads(path.read_text());spec=data['extension_field'];f=BinaryField(spec['degree'],int(spec['modulus_hex'],16));check(f.degree==83,'actual field identity')
 V=[int(v,16)for v in data['source_basis_hex']];check(V==[1<<i for i in range(18)]and int(data.get('source_offset_hex','0'),16)==0,'actual low18 domain')
 A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);qy=int(data['target']['y_hex'],16);y=qy^z
 check(f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B,'negative target on actual curve')
 h=0x4208336d0d441df79c400;k=0
 historical=json.loads((ROOT/'research/advance/n83_five_divisor_profile.json').read_text())['profiles'][0]
 check(int(historical['d1_hex'],16)==h and int(historical['d0_hex'],16)==k,'first historical cell fixed before execution')
 base,columns=map_for(f,A,z,y,h,k);check(base==literal(f,A,B,z,y,h,k,0,0),'actual literal constant')
 for bit in range(166):
  r=(1<<bit)if bit<83 else 0;s=(1<<(bit-83))if bit>=83 else 0
  check(value(base,columns,1<<bit)==literal(f,A,B,z,y,h,k,r,s),'actual literal column replay')
 # Structural proof in PROOF.md establishes no nonlinear terms; replay is an audit.
 spans,rows,family=relaxation(f,18,base,columns);check([len(p)for p in spans]==[18,35,52,69,83],'exact actual product spans')
 check(family['consistent']and family['rank']==93 and family['nullity']==73,'historical relaxed cell rank reproduced')
 rb=value(base,columns,family['particular']);rc=[[a^b for a,b in zip(value(base,columns,v),base)]for v in family['basis']];check(len(rc)<=100,'reduced parameter cap');check(residual(rb[4],spans[0])==0,'fixed translation preserves actual V')
 branches=[stratum(f,18,rb,rc,shift)for shift in [0,rb[4]]]
 return dict(instance=data['instance_id'],field_degree=83,modulus=f.modulus,source_dimension=18,h=h,k=k,raw_parameter_bits=166,relaxed_rows=rows,relaxed_family=family,relaxed_H_base=rb,relaxed_H_columns=rc,branches=branches,first_symmetric_coefficient=rb[4],literal_constant_and_columns=167,structural_affinity_proved=True,enumerated_2_to_73=False,original_root_count_exact=None,original_global_complete=False)
def main():
 ident,mode=sys.argv[1:];assert mode in ['controls','profile'];out={};status='passed';error=None
 try:out=controls()if mode=='controls'else profile()
 except (AssertionError,ValueError,RuntimeError,ZeroDivisionError,KeyError)as exc:status='inconclusive';error=str(exc)
 raw=dict(mode=mode,out=out,predicates=CHECKS,error=error)
 blob=gzip.compress(json.dumps(raw,sort_keys=True,separators=(',',':')).encode(),mtime=0)
 with(P/('raw-'+mode+'.json.gz')).open('xb')as stream:stream.write(blob)
 evidence=dict(details=out,predicates=CHECKS,error=error,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_affine_component=status=='passed',complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Exact conditional quartic Frobenius/affine norm-cell intersection; no original-global-root or speed claim.',evidence=evidence,next_actions=['Inspect proved obstruction or residual coefficient-map kernel before proposing a broader repeated-factor stratum; preserve original source/order/auxiliary/torsion branches.'])
 text=json.dumps(result,separators=(',',':'))
 if time.monotonic()-START>=84:
  result['status']='inconclusive';evidence['complete_conditional_affine_component']=False;evidence['error']='late serialized report';text=json.dumps(result)
 assert time.monotonic()-START<88,'final report deadline';print(text)
if __name__=='__main__':main()
