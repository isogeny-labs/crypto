"""Complete centroid hcover v-rank census at fixed k0, no source-root claim."""
from pathlib import Path
import hashlib,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.norm_trace_dual_20261115_v37 import backend as a
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def census(out):
 prior=json.loads((R/'research/centroid_cover_20261115_v33/raw.json').read_text());assert prior['error']is None;aa=prior['out']['actual'];domain=aa['domain'];f=BinaryField(83,aa['modulus']);D=a.Dual(f,52,69);B=domain['basis'];h=domain['particular'];N=domain['count'];assert N==1<<18 and domain['all_h_nonzero']
 terms=D.terms(0);rawmul=f._raw_mul;rank=a.rank;hist=[0]*15;digest=hashlib.sha256();invseq=hashlib.sha256();out.update(complete=False,seen=0,total=N,rank_histogram=hist,deficient_witnesses=[],direct_samples=[],field_degree=83,modulus=f.modulus,k=0,particular=h,basis=B,derivative=D.derivative,terms=terms)
 for i in range(N):
  if i:h^=B[(i&-i).bit_length()-1]
  assert h;inv=f.inv(h);assert rawmul(h,inv)==1;factor=rawmul(D.derivative,inv);cols=[rawmul(factor,v)>>31 for v in terms];r=rank(cols);hist[r]+=1;digest.update(h.to_bytes(11,'little')+bytes([r]));invseq.update(h.to_bytes(11,'little')+inv.to_bytes(11,'little'));out['seen']=i+1
  if r!=14 and len(out['deficient_witnesses'])<16:
   direct=D.direct_v_rank(h,0);assert direct==31+r;out['deficient_witnesses'].append(dict(h=h,rank=r,columns=cols,direct_rank=direct))
  if i%4096==0:
   direct=D.direct_v_rank(h,0);assert direct==31+r;out['direct_samples'].append(dict(index=i,h=h,dual_rank=r,direct_rank=direct));assert time.monotonic()-START<78,'census progress deadline'
 assert invseq.hexdigest()=='27fff4d86ea4a4a39d0843ed49c51d162a07b9f81a4408cdaf1c9cc82cb72ad0'
 out.update(complete=True,all_v_full=hist[14]==N,sequence_sha256=digest.hexdigest(),inverse_sequence_sha256=invseq.hexdigest(),E3_rank_min=31+next(j for j,c in enumerate(hist)if c),v_kernel_dimension=38 if hist[14]==N else None,scope='All certifiedh vblock ranks atfixedk0 only; no source-support or originalallroots solve')
def main():
 out={};error=None
 try:census(out)
 except Exception as exc:error=type(exc).__name__+': '+str(exc);out['complete']=False
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<65536
 with(P/'raw.json').open('xb')as f:f.write(raw)
 ev=dict(error=error,complete=out.get('complete'),seen=out.get('seen'),total=out.get('total'),rank_histogram=out.get('rank_histogram'),sequence_sha256=out.get('sequence_sha256'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Complete certified262144h vblock rank census at k0; inverse sequence and direct rank witnesses.',evidence=ev,next_actions=['If allfull, linear E2/E3/E4 cannot reject thesehcharts; use v-first exact fibers and prioritize nonlinear wholecell support. If incomplete, change to Grayupdated projectedmatrix representation, not identicaltimeout retry.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
