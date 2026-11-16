"""Portable prime-pair math replay; provide --flint for exact primality recheck."""
from pathlib import Path
from fractions import Fraction
import argparse,ctypes,hashlib,json,math,os,resource,time
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def load(folder,digest):
 blob=(R/'research'/folder/'raw.json').read_bytes();assert hashlib.sha256(blob).hexdigest()==digest;d=json.loads(blob);assert d['error']is None;return d['out']
def prove(p,path):
 lib=ctypes.CDLL(str(path));pointer=ctypes.POINTER(ctypes.c_long)
 lib.fmpz_init.argtypes=[pointer];lib.fmpz_init.restype=None;lib.fmpz_clear.argtypes=[pointer];lib.fmpz_clear.restype=None;lib.fmpz_set_str.argtypes=[pointer,ctypes.c_char_p,ctypes.c_int];lib.fmpz_set_str.restype=ctypes.c_int;lib.fmpz_is_prime.argtypes=[pointer];lib.fmpz_is_prime.restype=ctypes.c_int
 value=ctypes.c_long();lib.fmpz_init(ctypes.byref(value))
 try:assert lib.fmpz_set_str(ctypes.byref(value),str(p).encode(),10)==0;assert lib.fmpz_is_prime(ctypes.byref(value))==1
 finally:lib.fmpz_clear(ctypes.byref(value))
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--flint',type=Path,required=True);args=parser.parse_args();os.sched_setaffinity(0,{min(os.sched_getaffinity(0))});resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
 first=load('prime_pair_oracles_20261115_v42','c1ca44d5dc9fd840b9296585bec8aebf2e15a484b0fa4b77f6fccb2a2fd82d45');new=load('prime_pair_adaptive_20261115_v42','86e9f4f9126f6a1d78adf88a35f5244eb8e7d1f5589010ed4c88a8d9cb4cb249');tiny=0
 for old,row in zip(first['controls']['profiles'],new['controls']['profiles']):
  assert old==row;p=row['p'];M=row['M'];assert all(p%d for d in range(2,math.isqrt(p)+1));truth={}
  for a in range(1,M+1):
   for b in range(1,M+1):truth.setdefault((pow(a,-1,p)+pow(b,-1,p))%p,[]).append((a,b))
  digest=hashlib.sha256()
  for t in range(p):digest.update(len(truth.get(t,[])).to_bytes(4,'little'))
  assert digest.hexdigest()==row['count_sha256'];assert row['positive']==[dict(t=t,pairs=[list(x)for x in sorted(truth[t])])for t in sorted(truth)];tiny+=p
 count=0;libhash=hashlib.sha256(args.flint.read_bytes()).hexdigest()
 for old,field in zip(first['actual']['fields'],new['actual']['fields']):
  p=field['p'];M=field['M'];d=field['d'];assert4=4*M**3<p;assert assert4;prove(p,args.flint);H=1<<(d-1)
  assert field['prime_proof']['proven_prime']and field['prime_proof']['integer_hex']==hex(p)
  for oldcase,case in zip(old['cases'],field['cases']):
   t=case['t'];result=case['result'];assert result['complete'];best=Fraction(t,p).limit_denominator(M*M);num=t*best.denominator-p*best.numerator;fraction=[num,best.denominator]if 0<num<=2*M else None;assert fraction==result.get('fraction')
   mode=case['mode']
   if mode=='near_half':
    N,D=fraction;assert(N,D)==(2*H+1,H*(H+1));assert min(2*M//N,M*M//D)==1;want=[(H,H+1),(H+1,H)]
   elif mode=='diagonal':
    N,D=fraction;assert(N,D)==(2,M);assert(4*D+N*N-1)//(N*N)==M==min(2*M//N,M*M//D);want=[(M,M)]
   elif mode=='unit_fraction':
    assert fraction==[1,1]and not oldcase['result']['complete'];want=[(2,2)]
   else:assert mode in['zero','query']and fraction is None;want=[]
   assert result['pairs']==[list(x)for x in want]
   for a,b in want:assert 1<=a<=M and 1<=b<=M and(pow(a,-1,p)+pow(b,-1,p))%p==t
   if mode!='unit_fraction':assert oldcase['result']['pairs']==result['pairs']
   count+=1
 print(json.dumps(dict(independent_small_targets=tiny,exact_proved_fields=[x['name']for x in new['actual']['fields']],complete_interval_controls=count,previous_incomplete_unitcases=5,library_matches_recorded=libhash==new['actual']['fields'][0]['prime_proof']['library_sha256'],seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,qualified_speedup=None)))
if __name__=='__main__':main()
