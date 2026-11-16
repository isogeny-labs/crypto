"""Displacement-rank preflight of complete natural-coordinate incidence systems."""
from pathlib import Path
import gzip,json,hashlib,resource,sys,time
from research.full_kernel_block_20261115_v26 import worker as prev
b=prev.b;s=b.s;P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def rank(rows):return len(b.reduce_basis(rows))
def transpose_apply(mask,images):return sum(((mask&v).bit_count()%2)<<i for i,v in enumerate(images))
def displace(rows,images,blocks):
 out=[];start=0
 for length,step in blocks:
  for j in range(length):out.append((rows[start+j-step]if step and j>=step else 0)^transpose_apply(rows[start+j],images))
  start+=length
 assert start==len(rows);return out
def controls():
 # Independent full operator application over all2x4binary matrices and16vectors.
 S=[2,3,8,12];calls=0
 for code in range(256):
  rows=[code&15,code>>4];D=displace(rows,S,[(2,1)])
  for x in range(16):
   sx=s.xor(v for i,v in enumerate(S)if x>>i&1);mx=sum(((r&x).bit_count()%2)<<i for i,r in enumerate(rows));left=((mx<<1)&3)^sum(((r&sx).bit_count()%2)<<i for i,r in enumerate(rows));right=sum(((r&x).bit_count()%2)<<i for i,r in enumerate(D));assert left==right;calls+=1
 return dict(matrices=256,full_operator_equalities=calls)
def actual():
 old=json.loads(gzip.decompress((R/'research/full_quintic_block_20261115_v9/raw.json.gz').read_bytes()))['out']['profile']['proofs']['baseline166'];gold=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate'];atlas=json.loads((R/'research/kernel_incidence_20261115_v20/raw.json').read_text())['out']['profile'];D=json.loads((R/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text());f=s.b.BinaryField(83,int(D['extension_field']['modulus_hex'],16));z=int(D['target']['x_hex'],16);mask=(1<<83)-1
 natural=[(1<<i)|(f.mul(z,1<<i)<<83)for i in range(83)]+[1<<(83+i)for i in range(83)]
 base=[(transpose_apply(m,natural),r)for i,(m,r)in enumerate(old['rows'])if 83+35<=i<166 or 166+52<=i<249 or 249+69<=i<332];assert len(base)==93
 assert all(m==0 and r==0 for i,(m,r)in enumerate(old['rows'])if not(83+35<=i<166 or 166+52<=i<249 or 249+69<=i<332))
 rawbase=[m for m,r in old['rows']];piv=b.reduce_basis(rawbase);duals,_=s.nullspace(gold['raw_basis'],166);extra=[]
 for row in duals:
  rem=row
  while rem:
   bit=rem.bit_length()-1
   if bit not in piv:piv[bit]=rem;extra.append((transpose_apply(row,natural),(row&gold['raw_particular']).bit_count()%2));break
   rem^=piv[bit]
 assert len(extra)==4
 one=[f.mul(2,1<<i)for i in range(83)];two=[f.mul(4,1<<i)for i in range(83)];ordinary=one+[v<<83 for v in one];skew=one+[v<<83 for v in two]
 us=[1<<i for i in range(15)]+[1<<34]+atlas['directions'][35:51];assert len(set(us))==32;cases=[];summary={}
 for name,n in [('pre-Gold73',73),('Gold69',69)]:
  selected={v['u']:v['intersection']for v in atlas['records']if v['family']==name};front=base+(extra if n==69 else[]);g=len(front)-93;records=[]
  for u in us:
   power=[f.mul(u,1<<i)for i in range(83)];inc=[(sum(((v>>j)&1)<<i for i,v in enumerate(power)),0)for j in range(52,83)]+[(sum(((v>>j)&1)<<(83+i)for i,v in enumerate(power)),0)for j in range(69,83)];rows=[m for m,r in front+inc]
   normal=displace(rows,ordinary,[(48,1),(31,1),(14,1)]+([(4,0)]if g else[])+[(31,1),(14,1)]);mixed=displace(rows,skew,[(48,2),(31,2),(14,4)]+([(4,0)]if g else[])+[(31,1),(14,2)])
   assert rank(rows)==166-n+rank([m for m,r in selected[u]['rows']])
   record=dict(u=u,rows=len(rows),ordinary_displacement_rank=rank(normal),mixed_displacement_rank=rank(mixed),mixed_outside_AS_rank=rank(mixed[48:]),system_rank=rank(rows));assert record['mixed_outside_AS_rank']<=13;records.append(record)
  summary[name]=dict(cases=32,whole_rank8_prediction=all(r['mixed_displacement_rank']<=8 for r in records),ordinary_ranks=sorted(set(r['ordinary_displacement_rank']for r in records)),mixed_ranks=sorted(set(r['mixed_displacement_rank']for r in records)),outside_AS_ranks=sorted(set(r['mixed_outside_AS_rank']for r in records)));cases.append(dict(family=name,records=records))
 return dict(summary=summary,families=cases,natural_base_rows=base,natural_Gold4_rows=extra,modulus=int(D['extension_field']['modulus_hex'],16),ordinary_column_images=ordinary,mixed_column_images=skew,scope='Full coefficient-span linear matrices only; no support/globalroot or otherfield timing')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<524288
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,controls=out.get('controls'),actual=out.get('actual',{}).get('summary'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),external_reviewer=False,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Complete-system displacement ranks with ordinary and Frobenius-aware frozen shifts.',evidence=ev,next_actions=['Ifwholelowrankfails, isolate ArtinSchreierblock plus provedboundedboundarygenerators; do not inferfullstructurefrommultiplicationalone. Oddbackendandoriginalrootgatesremainopen.'])
 text=json.dumps(result,separators=(',',':'));assert len(text.encode())<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
