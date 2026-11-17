"""Exact E2-fiber elimination; conditional necessary-moment cell certificates."""
from pathlib import Path
import gzip,hashlib,itertools,json,resource,sys,time
from rootfinder.binaryfield import BinaryField
from research.joint_cubic_20261115_v15 import worker as formula
P=Path(__file__).resolve().parent;R=P.parents[1];START=time.monotonic();CHECKS=0

def require(value,why):
    global CHECKS
    CHECKS+=1
    assert CHECKS<=200000 and time.monotonic()-START<78,'work cap'
    assert value,why

def xor(values):
    out=0
    for v in values:out^=v
    return out

def decomposition(columns):
    piv={};outer=[];kernel=[]
    for i,value in enumerate(columns):
        v=value;t=1<<i
        while v:
            p=v.bit_length()-1
            if p in piv:v^=piv[p][0];t^=piv[p][1]
            else:piv[p]=(v,t);outer.append(1<<i);break
        if not v:kernel.append(t)
    require(len(outer)+len(kernel)==len(columns),'full parameter decomposition')
    require(all(xor(v for i,v in enumerate(columns)if t>>i&1)==0 for t in kernel),'exact coefficient kernel')
    return outer,kernel

def image(f,d,count):
    piv={};generators=[]
    for i,j in itertools.combinations(range(d),2):
        v=sum((f.mul(f.pow(1<<i,q),1<<j)^f.mul(1<<i,f.pow(1<<j,q)))<<(f.degree*k)for k,q in enumerate([2,4,8][:count]));generators.append(v)
        while v:
            p=v.bit_length()-1
            if p in piv:v^=piv[p]
            else:piv[p]=v;break
    return piv,generators

def residue(v,piv):
    for p in sorted(piv,reverse=True):
        if v>>p&1:v^=piv[p]
    return v

def solve(columns,rhs,variables,width):
    rows=[]
    for bit in range(width):rows.append((sum(((v>>bit)&1)<<i for i,v in enumerate(columns)),(rhs>>bit)&1))
    piv={}
    for mask,b in rows:
        while mask:
            p=mask.bit_length()-1
            if p in piv:mask^=piv[p][0];b^=piv[p][1]
            else:piv[p]=(mask,b);break
        if not mask and b:return dict(consistent=False,rank=len(piv),particular=None,basis=[])
    def finish(t,constant):
        for p in sorted(piv):
            mask,b=piv[p]
            if ((mask&t).bit_count()&1)^(b if constant else 0):t^=1<<p
        return t
    particular=finish(0,True);basis=[finish(1<<i,False)for i in range(variables)if i not in piv]
    require(xor(v for i,v in enumerate(columns)if particular>>i&1)==rhs,'particular literal equation')
    require(all(xor(v for i,v in enumerate(columns)if t>>i&1)==0 for t in basis),'kernel literal equation')
    return dict(consistent=True,rank=len(piv),particular=particular,basis=basis)

def mapH(base,cols,t):return [b^xor(col[j]for i,col in enumerate(cols)if t>>i&1)for j,b in enumerate(base)]
def direction(cols,t):return [xor(col[j]for i,col in enumerate(cols)if t>>i&1)for j in range(6)]
def polys(base,cols):
    H=[({0:v}if v else {})for v in base]
    for i,col in enumerate(cols):
        for j,v in enumerate(col):
            if v:H[j][1<<i]=v
    return H

def numeric(f,H):
    e=[0]+list(reversed(H[:5]));s=[1]
    for k in range(1,10):
        v=xor(f.mul(e[j],s[k-j])for j in range(1,min(5,k-1)+1))
        if k<=5 and k%2:v^=e[k]
        s.append(v)
    return [s[k]^f.pow(e[1],k)for k in[3,5,9]]

def evaluate(poly,t):return xor(v for m,v in poly.items()if m&t==m)
def degree(poly):return max((m.bit_count()for m in poly),default=0)

def one_cell(f,base,cols,d,outer_value,W,small=False):
    begin=time.monotonic();cellbase=mapH(base,cols,outer_value);inner=[direction(cols,t)for t in W]
    require(all(col[3]==0 for col in inner),'E2 fixed on complete cell')
    moments=formula.compile_moments(f,polys(cellbase,inner));require(all(degree(p)<=1 for p in moments[:2]),'first two moments affine on E2 fiber')
    piv35,_=image(f,d,2);pack=lambda vals:sum(v<<(f.degree*i)for i,v in enumerate(vals))
    rhs=residue(pack([p.get(0,0)for p in moments[:2]]),piv35)
    columns=[residue(pack([p.get(1<<i,0)for p in moments[:2]]),piv35)for i in range(len(W))]
    sol=solve(columns,rhs,len(W),2*f.degree)
    out=dict(outer_parameter=outer_value,E2=cellbase[3],inner_bits=len(W),linear=sol,covered_parameter_points=1<<len(W),complete_moment_cell=False,moment_count_exact=None,supported_norm_count_exact=None,models=[],complete_original_roots=False)
    if not sol['consistent']:
        out.update(complete_moment_cell=True,moment_count_exact=0,rejected_supported_norm_cell=True,enumerated=0);return out,[]
    nullity=len(sol['basis']);out['remaining_bits']=nullity
    if nullity>16:out.update(reason='postlinear nullity cap',enumerated=0,rejected_supported_norm_cell=False);return out,[]
    hbase=mapH(cellbase,inner,sol['particular']);hcols=[direction(inner,t)for t in sol['basis']]
    final=formula.compile_moments(f,polys(hbase,hcols));require(all(degree(p)<=2 for p in final),'all remaining moments at most quadratic')
    piv,_=image(f,d,3);syndrome={}
    for mask in set().union(*(p.keys()for p in final)):
        v=residue(pack([p.get(mask,0)for p in final]),piv)
        if v:syndrome[mask]=v
    require(degree(syndrome)<=2,'quadratic full joint syndrome')
    digest=hashlib.sha256();survivors=[];count=0
    for t in range(1<<nullity):
        v=evaluate(syndrome,t);digest.update(v.to_bytes((3*f.degree+7)//8,'little'))
        if not v:
            count+=1;old=outer_value^xor(w for i,w in enumerate(W)if sol['particular']>>i&1)^xor(W[j]for i,basis in enumerate(sol['basis'])if t>>i&1 for j in range(len(W))if basis>>j&1)
            if small:survivors.append(old)
            if len(out['models'])<32:out['models'].append(dict(parameter=old,H=mapH(hbase,hcols,t)))
        if t<4 or small:
            h=mapH(hbase,hcols,t);require([evaluate(p,t)for p in final]==numeric(f,h),'independent generic Newton residual replay')
            require(v==residue(pack(numeric(f,h)),piv),'complete tiny or first4 actual numeric projection')
    out.update(complete_moment_cell=True,moment_count_exact=count,rejected_supported_norm_cell=count==0,enumerated=1<<nullity,syndrome_degree=degree(syndrome),syndrome_terms=len(syndrome),syndrome=sorted(syndrome.items()),truth_sha256=digest.hexdigest(),models_truncated=count>32,seconds=time.monotonic()-begin)
    return out,survivors

def controls():
    cases=[]
    for n,mod in[(2,7),(3,11),(4,19)]:
        f=BinaryField(n,mod);base=[0,0,0,0,0,1];cols=[[((i+1)*(j+2))%f.order if j<4 else 0 for j in range(6)]for i in range(4)]
        outer,W=decomposition([col[3]for col in cols]);_,generators=image(f,2,3);direct=[]
        # Independent image-membership oracle exhausts generator span, not Gaussian reduction.
        span={xor(v for i,v in enumerate(generators)if t>>i&1)for t in range(1<<len(generators))}
        for t in range(16):
            vals=numeric(f,mapH(base,cols,t));packed=sum(v<<(n*i)for i,v in enumerate(vals))
            if packed in span:direct.append(t)
        found=[]
        for t in range(1<<len(outer)):
            cell,survivors=one_cell(f,base,cols,2,xor(v for i,v in enumerate(outer)if t>>i&1),W,True);require(cell['complete_moment_cell'],'tiny cell complete');found+=survivors
        require(sorted(found)==direct and len(found)==len(set(found))and 0 in found,'complete independent tiny original parameter sets')
        cases.append(dict(n=n,assignments=16,E2_rank=len(outer),inner_bits=len(W),moment_models=len(found)))
    return cases

def actual():
    cert=json.loads(gzip.decompress((R/'research/joint_cubic_20261115_v15/raw.json.gz').read_bytes()))['out']['actual']['family_certificate'];base=cert['base'];cols=cert['columns'];f=BinaryField(83,0x800000000200000000007)
    require(len(cols)==69,'certified family width');outer,W=decomposition([col[3]for col in cols]);require(len(outer)<=35 and len(W)>=34,'nonwrapping E2 span bound')
    choices=[0]+[v for v in outer[:7]];cells=[]
    for t in choices:cells.append(one_cell(f,base,cols,18,t,W)[0])
    return dict(parameter_bits=69,E2_rank=len(outer),inner_bits=len(W),outer_basis=outer,inner_basis=W,cells=cells,all_E2_cells=1<<len(outer),tested_E2_cells=len(cells),rejected_cells=sum(c['rejected_supported_norm_cell']for c in cells),coverage_scope='Only explicitly fixed E2 cells of one exposed h/k chart; no global source or original-chain completion.')

def main():
    out={};error=None
    try:out['controls']=controls();out['actual']=actual()
    except Exception as exc:error=type(exc).__name__+': '+str(exc)
    status='passed'if error is None and all(c['complete_moment_cell']for c in out.get('actual',{}).get('cells',[]))else'inconclusive'
    raw=json.dumps(dict(out=out,error=error,checks=CHECKS),sort_keys=True,separators=(',',':')).encode();blob=gzip.compress(raw,mtime=0);assert len(raw)<512*1024 and len(blob)<128*1024
    with(P/'raw.json.gz').open('xb')as stream:stream.write(blob)
    actual_summary=out.get('actual',{});compact={k:v for k,v in actual_summary.items()if k not in['inner_basis','outer_basis','cells']}
    compact['cells']=[{k:v for k,v in c.items()if k in['E2','remaining_bits','enumerated','complete_moment_cell','moment_count_exact','rejected_supported_norm_cell','seconds','syndrome_terms']}for c in actual_summary.get('cells',[])]
    evidence=dict(controls=out.get('controls'),actual=compact,error=error,checks=CHECKS,peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,worker_seconds=time.monotonic()-START,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),external_reviewer=False,complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False)
    result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status=status,summary='Exact E2-fiber linear elimination followed by bounded complete quadratic-moment cell enumeration; conditional necessary-condition certificates only.',evidence=evidence,next_actions=['If cells close cheaply, derive whole-outer-cell polynomial inconsistency certificates or a stronger batching rule; do not mistake eight cells for2^E2rank coverage. If postlinear nullity cap fails, change the chosen coefficient projection before retry.'])
    text=json.dumps(result,separators=(',',':'))
    if time.monotonic()-START>=84:result['status']='inconclusive';evidence['actual']={};evidence['error']='late report';text=json.dumps(result,separators=(',',':'))
    assert len(text.encode())<8192 and time.monotonic()-START<88;print(text)
if __name__=='__main__':main()
