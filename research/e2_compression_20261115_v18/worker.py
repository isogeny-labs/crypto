"""Fresh-process matched conditional-cell compiler comparison; no global gate."""
from pathlib import Path
import gzip,hashlib,json,math,os,resource,subprocess,sys,time
START=time.monotonic();P=Path(__file__).resolve().parent;R=P.parents[1]
METHODS=['reference','cached','lean','compressed']
def child(method):
    import importlib
    package='research.e2_certificate_20261115_v17.worker'if method=='reference'else'research.e2_compression_20261115_v18.'+method
    m=importlib.import_module(package)
    from rootfinder.binaryfield import BinaryField
    controls=m.controls();cert_controls=m.certificate_controls()
    cert=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate']
    f=BinaryField(83,0x800000000200000000007);base=cert['base'];cols=cert['columns'];outer,W=m.decomposition([col[3]for col in cols]);cells=[];canonical=[]
    assert len(outer)==34 and len(W)==35
    for index in range(32):
        gray=index^(index>>1);value=m.xor(v for i,v in enumerate(outer[:5])if gray>>i&1)
        cell,_=m.one_cell(f,base,cols,18,value,W)
        assert cell['complete_moment_cell'],'incomplete cell'
        canonical.append({k:cell[k]for k in['E2','linear','syndrome','remaining_bits','complete_moment_cell','moment_count_exact']})
        cells.append(cell)
    normalized=json.dumps(canonical,sort_keys=True,separators=(',',':')).encode()
    result=dict(method=method,controls=controls,certificate_controls=cert_controls,E2_rank=len(outer),inner_bits=len(W),cells=cells,normalized_sha256=hashlib.sha256(normalized).hexdigest(),inner_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_conditional_cells=True,complete_global_roots=False)
    data=json.dumps(result,separators=(',',':'));assert len(data.encode())<1024*1024 and time.monotonic()-START<20
    print(data)
def parent():
    reports=[];error=None
    orders=[METHODS,list(reversed(METHODS)),['cached','compressed','reference','lean']]
    try:
        for repetition,order in enumerate(orders):
            for method in order:
                remaining=78-(time.monotonic()-START)
                if remaining<20:raise TimeoutError('reserve output budget before next cold child')
                begin=time.monotonic();run=subprocess.run([sys.executable,'-m','research.e2_compression_20261115_v18.worker','--child',method],cwd=R,capture_output=True,text=True,timeout=min(20,remaining))
                elapsed=time.monotonic()-begin
                if run.returncode:raise RuntimeError('child '+method+' failed: '+run.stderr[-1500:])
                if len(run.stdout.encode())>1024*1024:raise RuntimeError('child stdout cap')
                report=json.loads(run.stdout);report['repetition']=repetition;report['end_to_end_seconds']=elapsed;reports.append(report)
        assert len(reports)==12
        assert len({r['normalized_sha256']for r in reports})==1,'conditional result mismatch'
        assert all(r['controls']==reports[0]['controls']and r['certificate_controls']==reports[0]['certificate_controls']for r in reports)
    except Exception as exc:error=type(exc).__name__+': '+str(exc)
    metrics={}
    for method in METHODS:
        rs=[r for r in reports if r['method']==method];times=[r['end_to_end_seconds']for r in rs]
        if times:
            metrics[method]=dict(cold_process_repetitions=len(times),seconds=times,geomean_seconds=math.exp(sum(math.log(t)for t in times)/len(times)),aa_spread=max(times)/min(times)-1,peak_rss_kib=max(r['peak_rss_kib']for r in rs),rejected_cells=[sum(c['rejected_supported_norm_cell']for c in r['cells'])for r in rs],independent_equations=sorted({c.get('polynomial_certificate',{}).get('independent_equations',-1)for r in rs for c in r['cells']}))
    ratios={}
    if len(reports)==12:
        for base in METHODS[:-1]:ratios[base+'_over_compressed']=metrics[base]['geomean_seconds']/metrics['compressed']['geomean_seconds']
    raw=json.dumps(dict(reports=reports,metrics=metrics,component_ratios=ratios,error=error),sort_keys=True,separators=(',',':')).encode();blob=gzip.compress(raw,mtime=0)
    assert len(raw)<8*1024*1024 and len(blob)<512*1024
    with(P/'raw.json.gz').open('xb')as stream:stream.write(blob)
    evidence=dict(metrics=metrics,component_ratios=ratios,error=error,raw_bytes=len(blob),raw_sha256=hashlib.sha256(blob).hexdigest(),worker_seconds=time.monotonic()-START,parent_peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,child_peak_rss_kib=max((r['peak_rss_kib']for r in reports),default=0),memory_scope='Sequential child RSS and parent RSS separately, not a cgroup simultaneous high-water mark',external_reviewer=False,complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,scope='Matched32conditionalE2cells, freshprocess repetitions, exposed calibration only; no genuine F4/F5/F6 baseline')
    result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='Matched fresh-process conditional compiler comparison: cache, avoid unused prelinear g9, and compress equivalent quadratic equations before certificate solving.',evidence=evidence,next_actions=['Choose measured bottleneck versus the strongest cached/lean scalar comparator; design a fresh guarded or batched outer-cell mechanism. Preserve rank-deficient and exceptional contexts; no component ratio certifies the full all-roots release gate.'])
    text=json.dumps(result,separators=(',',':'))
    if time.monotonic()-START>=84:result['status']='inconclusive';evidence['error']='late report';evidence['component_ratios']={};text=json.dumps(result,separators=(',',':'))
    assert len(text.encode())<8192 and time.monotonic()-START<88;print(text)
if __name__=='__main__':
    if len(sys.argv)==3 and sys.argv[1]=='--child':child(sys.argv[2])
    else:parent()
