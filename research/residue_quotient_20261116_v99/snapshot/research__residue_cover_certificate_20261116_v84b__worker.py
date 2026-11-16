from pathlib import Path
import json,time,random,math
from .common import P,finish
from research.residue_lift_sieve_20261116_v84.kernel import mul,divrem,code,echelon
START=time.monotonic()
def certificate(cols,bits):
 piv={};kernel=[]
 for j,v in enumerate(cols):
  mask=1<<j
  while v:
   i=v.bit_length()-1
   if i in piv:v^=piv[i][0];mask^=piv[i][1]
   else:piv[i]=(v,mask);break
  if not v:kernel.append(mask)
 def lift(v):
  ans=0
  while v:
   i=v.bit_length()-1
   if i not in piv:raise ValueError('not surjective')
   v^=piv[i][0];ans^=piv[i][1]
  return ans
 return[ lift(1<<i)for i in range(bits)],kernel

def apply(cols,v):
 out=0
 while v:
  b=v&-v;out^=cols[b.bit_length()-1];v^=b
 return out

def main():
 out={'toy_cases':0,'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
  for _ in range(80):
   cols=[rng.randrange(64)for _ in range(8)]
   if len(echelon(cols))!=6:continue
   R,K=certificate(cols,6);assert len(K)==2 and len(echelon(K))==2
   A={rng.randrange(8)for _ in range(5)};B={rng.randrange(8)for _ in range(5)};base=rng.randrange(64);exact={v for v in range(256)if(apply(cols,v)^base)&7 in A and(apply(cols,v)^base)>>3 in B};got=set()
   for a in A:
    for b in B:
     y=a|(b<<3);v=apply(R,y^base)
     for t in range(4):got.add(v^apply(K,t))
   assert got==exact;out['toy_cases']+=1
  rows=[[int(x,16)for x in s.split()]for s in Path(d['matrix']).read_text().splitlines()[1:]];base=rows[0];hc=rows[1:];assert len(hc)==73 and base[4]>>17==1 and all(v[4]==0 for v in hc);M=d['modulus'];cols=[0]*76;offset=0;counts=[]
  for s,m in d['residue_fields']:
   mask=(1<<(4*s))-1
   for j,v in enumerate(hc):cols[j]^=(code(v,s,m)&mask)<<offset
   for j in range(3):cols[73+j]^=divrem(mul(1<<j,M),m)[1]<<offset
   offset+=4*s;counts.append(math.comb((1<<s)+4,5)//(1<<s))
  assert offset==72;R,K=certificate(cols,72);assert len(K)==4 and len(echelon(K))==4
  assert all(apply(cols,v)==1<<i for i,v in enumerate(R));assert all(apply(cols,v)==0 for v in K)
  out.update({'observation_bits':72,'parameter_bits':76,'right_inverse':R,'kernel_basis':K,'allowed_counts_at_fixed_trace':counts,'exact_first7_cover_size':16*math.prod(counts),'original_lift_pairs':1<<76,'coverage':'all first7 residue-compatible lifted parameter states,not full original roots;no birthday filtering','complete':True})
 except Exception as e:error=repr(e)
 finish(out,error,START,'Exact affine table-coordinate cover with explicit right inverse/kernel and exhaustive toy all-solutions checks.','Use the complete-cover parametrization in an exact bounded table-XOR solver; add residue constraints and full support verification, preserving every solution and all chart obligations.')
if __name__=='__main__':main()
