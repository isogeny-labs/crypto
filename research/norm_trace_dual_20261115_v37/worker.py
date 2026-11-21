"""Independent complete small-field image and trace-pairing rank oracle."""
from pathlib import Path
import hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.norm_trace_dual_20261115_v37 import backend as a
from research.norm_block_reuse_20261115_v36 import worker as old
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic()
def naive(f,x,y):return old.old.naive(f,x,y)
def independent_trace(f,x):
 t=0
 for _ in range(f.degree):t^=x;x=naive(f,x,x)
 assert t in(0,1);return t
def controls():
 systems=images=ann=zero=defect=0;profiles=[]
 for n,m in[(2,7),(3,11),(4,19)]:
  f=BinaryField(n,m);q=f.order
  for d in range(1,min(3,n)+1):
   w3=min(n,3*(d-1)+1);w4=min(n,4*(d-1)+1);D=a.Dual(f,w3,w4)
   for width in range(n+1):
    vals=[f.mul(1<<j,f.inv(D.derivative))for j in range(n-width)]
    assert a.rank(vals)==n-width
    for mu in vals:
     for i in range(width):assert independent_trace(f,naive(f,mu,1<<i))==0;ann+=1
   for v in range(q):assert independent_trace(f,naive(f,v,f.inv(D.derivative)))==v>>(n-1)
   histogram={}
   for h,k in itertools.product(range(q),repeat=2):
    image={(naive(f,h,v)>>w3)|(((naive(f,v,v)^naive(f,k,v))>>w4)<<(n-w3))for v in range(q)};images+=q;assert len(image)&(len(image)-1)==0
    literal_rank=len(image).bit_length()-1;predicted=D.predicted_v_rank(h,k);assert predicted==literal_rank==D.direct_v_rank(h,k)
    histogram[str(literal_rank)]=histogram.get(str(literal_rank),0)+1;systems+=1;zero+=h==0;defect+=literal_rank<2*n-w3-w4
   profiles.append(dict(n=n,d=d,w3=w3,w4=w4,histogram=histogram))
 return dict(complete_systems=systems,literal_v_values=images,annihilator_trace_checks=ann,h0_systems=zero,rank_deficient_systems=defect,profiles=profiles)
def actual():
 prior=json.loads((R/'research/norm_block_reuse_20261115_v36/raw.json').read_text());assert prior['error']is None;data=prior['out']['actual'];f=BinaryField(83,data['modulus']);D=a.Dual(f,52,69);rows=[]
 for h,k in itertools.product(data['hs'],data['ks']):
  columns=D.columns(h,k);dualrank=a.rank(columns);vrank=D.direct_v_rank(h,k);assert vrank==31+dualrank
  rows.append(dict(h=h,k=k,dual_rank=dualrank,v_rank=vrank,dual_columns=columns))
 return dict(modulus=f.modulus,derivative=D.derivative,mu=D.mu,mu_half=D.half,w3=52,w4=69,records=rows,all_v_full=all(r['v_rank']==45 for r in rows),scope='Exact vblock rank only; allroot support/wholecell pruning unproved')
def main():
 out={};error=None
 try:out['controls']=controls();out['actual']=actual()
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode();assert len(raw)<131072
 with(P/'raw.json').open('xb')as f:f.write(raw)
 aa=out.get('actual',{});ev=dict(error=error,controls=out.get('controls'),actual_cells=len(aa.get('records',[])),all_v_full=aa.get('all_v_full'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,external_reviewer=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Trace-dual14variable rank equivalence with complete tiny image/annihilator controls and128actual vblocks.',evidence=ev,next_actions=['If full selected ranks hold, freeze completehcover census at fixedk0; then v-first exact fibers with fallback, or move to nonlinear support because linear coefficient conditions cannot prune thesecharts.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<8192 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
