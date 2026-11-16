import sys,json,time,resource,os
from pathlib import Path
START=time.monotonic()
from .kernel import bundle
from research.image_cells_20261115_v67.algebra import BField
Path(sys.argv[3]).write_text('post-exec imports complete\n');d=json.loads(Path(sys.argv[1]).read_text());method=sys.argv[2];assert method in d['methods'];f=BField(d['n'],d['modulus']);begin=time.monotonic();rows=bundle(d['records'],f,method,d['dimension']);elapsed=time.monotonic()-begin
for r,row in zip(d['records'],rows):
 assert row['support']==r['support_truth']
 if method!='support':assert row['image_polynomial']==d['expected_labels'][r['name']]['image_polynomial']
out={'complete':True,'method':method,'rows':rows,'bundle_seconds':elapsed,'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'cpu_affinity':sorted(os.sched_getaffinity(0)),'exact_root_count':None,'scope':'exact support/label diagnostics; no original root-set computation'};trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=16:out['complete']=False;out['reason']='late bundle'
print(json.dumps(out,separators=(',',':')))
