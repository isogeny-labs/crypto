"""Target-scoped support shortcut; every H is constructed from norm parameters."""
from pathlib import Path
import hashlib,json
from rootfinder import binaryfield as b
from research.inversion_free_norm_20261115_v34 import compiler
from research.compiled_subspace_20261115_v31.factor import compile_basis
ROOT=Path(__file__).resolve().parents[2]
MOD=0x800000000200000000007
Z=0x622425e96ba633969263b
QY=0x3258275069b18d0854564
PINS={
 'research/k0_zero_cubic_20261115_v43/HISTORICAL_CUBIC_SCIENCE.json':'5a0aec5491ec70c67a7563851fef319e3965b3227109c9522ab94037a14386a9',
 'research/reciprocal_triple_lift_20261115_v40/raw.json':'deae1b65dde0c5653b82194ecd2ccc69438da5050d014f72de82e23baf87642f'}

def verify_certificates():
 data=[]
 for path,digest in PINS.items():
  raw=(ROOT/path).read_bytes()
  if hashlib.sha256(raw).hexdigest()!=digest:raise ValueError('certificate hash mismatch')
  data.append(json.loads(raw))
 cubic,triple=data[0],data[1]['out']['actual']
 assert cubic['complete'] and cubic['supported_norm_count_exact']==0
 assert cubic['field_modulus_hex']==hex(MOD) and cubic['source_dimension']==18
 assert cubic['target']==[hex(Z),hex(QY)]
 assert triple['global_nonzero_reciprocal_triples_complete'] and triple['query']['exact_count']==0 and triple['z']==Z
 return True

def remainder(f,H,lam):
 rem=(0,1)
 for c in lam:rem=b._add(b._squaremod(rem,H,f),tuple(f._raw_mul(c,v)for v in rem))
 return rem

def generic(f,H,lam):
 rem=remainder(f,H,lam)
 for _ in range(3):rem=b._squaremod(rem,H,f)
 return not rem

class Family:
 def __init__(self,f,A,B,z,y,c,d,certified=False):
  assert B and f.square(c)^c==A^z
  assert f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B
  self.f=f;self.ctx=dict(status='ready',z=z,y=y,c=c,d=y^f.mul(z,c));self.d=d
  self.lam=compile_basis(f,[1<<i for i in range(d)])[0]
  self.eligible=(f.degree==83 and f.modulus==MOD and (A,B,z,y,d)==(0,1,Z,QY^Z,18))
  self.certified=bool(certified and self.eligible and verify_certificates())
 def polynomial(self,h,u,v):
  for x in (h,u,v):self.f.validate(x)
  return tuple(compiler.norm_coefficients(self.f,self.ctx,h,u,v))
 def support(self,h,u,v):
  H=self.polynomial(h,u,v)
  if self.certified:
   if not v:return False
   return not remainder(self.f,H,self.lam)
  return generic(self.f,H,self.lam)
