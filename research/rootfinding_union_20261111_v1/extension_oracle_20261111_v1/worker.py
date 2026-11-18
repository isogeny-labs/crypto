"""Bounded coordinate-field proof oracle; no elliptic chart or speed claim."""
from pathlib import Path
import sys, json, time, itertools, hashlib, resource, gzip

START = time.monotonic()
HERE = Path(__file__).resolve().parent
CHECKS = 0

def check(value, why):
    global CHECKS
    CHECKS += 1
    assert CHECKS <= 200000, 'declared scalar-check cap'
    assert time.monotonic() - START < 80, 'internal work deadline'
    assert value, why

class Field:
    def __init__(self, p, c):
        self.p, self.c, self.q = p, c, p*p
    def coord(self, a): return [a % self.p, a // self.p]
    def enc(self, a): return a[0] % self.p + self.p*(a[1] % self.p)
    def add(self, a, b): return self.enc([x+y for x,y in zip(self.coord(a),self.coord(b))])
    def neg(self, a): return self.enc([-x for x in self.coord(a)])
    def mul(self, a, b):
        # Generic convolution followed by polynomial reduction modulo X^2+c.
        aa, bb = self.coord(a), self.coord(b)
        out = [0,0,0]
        for i in range(2):
            for j in range(2): out[i+j] += aa[i]*bb[j]
        out[0] -= self.c*out.pop()
        return self.enc(out)
    def matrix(self, a):
        columns = [self.coord(self.mul(a,b)) for b in [1,self.p]]
        return [[columns[j][i] for j in range(2)] for i in range(2)]

class TableOracle:
    """Independent pair arithmetic: precomputed direct closed-form field tables."""
    def __init__(self, p, c):
        self.p, self.c, self.q = p,c,p*p
        self.adds = [[((a%p+b%p)%p)+p*((a//p+b//p)%p) for b in range(p*p)] for a in range(p*p)]
        self.muls = [[((a%p*(b%p)-c*(a//p)*(b//p))%p)+p*((a%p*(b//p)+(a//p)*(b%p))%p) for b in range(p*p)] for a in range(p*p)]
        self.negs = [next(b for b in range(p*p) if self.adds[a][b]==0) for a in range(p*p)]
    def add(self,a,b): return self.adds[a][b]
    def mul(self,a,b): return self.muls[a][b]
    def neg(self,a): return self.negs[a]

def determinant(rows):
    # Bareiss exact integer determinant, not floating point.
    a=[row[:] for row in rows]; previous=1; sign=1
    for k in range(len(a)-1):
        pivot=next((j for j in range(k,len(a)) if a[j][k]),None)
        if pivot is None: return 0
        if pivot!=k: a[k],a[pivot]=a[pivot],a[k]; sign=-sign
        value=a[k][k]
        for i in range(k+1,len(a)):
            for j in range(k+1,len(a)):
                numerator=value*a[i][j]-a[i][k]*a[k][j]
                assert numerator%previous==0
                a[i][j]=numerator//previous
        for i in range(k+1,len(a)): a[i][k]=0
        previous=value
    return sign*a[-1][-1]

def bases(f, aa, bb):
    primal=[]
    for i in range(10):
        row=[0]*14; row[i]=1
        a,b=f.matrix(aa[i//2]),f.matrix(bb[i//2])
        for j in range(2): row[10+j]=a[j][i%2]; row[12+j]=b[j][i%2]
        primal.append(row)
    for i in range(10,14):
        row=[0]*14; row[i]=f.p; primal.append(row)
    dual=[]
    for i in range(10):
        row=[0]*14; row[i]=f.p; dual.append(row)
    for i in range(4):
        row=[0]*14; row[10+i]=1
        for j in range(5):
            mat=f.matrix(aa[j] if i<2 else bb[j])
            for k in range(2): row[2*j+k]=mat[i%2][k]
        dual.append(row)
    return primal,dual

def primal_member(row, f, oracle, aa, bb):
    for coefficients, offset in [(aa,10),(bb,12)]:
        total=0
        for i in range(5): total=oracle.add(total,oracle.mul(coefficients[i],f.enc(row[2*i:2*i+2])))
        if total != f.enc(row[offset:offset+2]): return False
    return True

def dual_member(row, f, oracle, aa, bb):
    # Oracle uses field multiplication of basis vectors and ordinary dot products;
    # it never constructs/transposes a multiplication matrix.
    for i in range(5):
        for k,unit in enumerate([1,f.p]):
            av=f.coord(oracle.mul(aa[i],unit)); bv=f.coord(oracle.mul(bb[i],unit))
            rhs=sum(row[10+j]*av[j]+row[12+j]*bv[j] for j in range(2))
            if (row[2*i+k]-rhs)%f.p: return False
    return True

def divide(poly, root, f):
    """Descending monic coefficients; division never uses encoded integer roots."""
    acc=poly[0]; quotient=[acc]
    for coefficient in poly[1:-1]:
        acc=f.add(coefficient,f.mul(acc,root)); quotient.append(acc)
    remainder=f.add(poly[-1],f.mul(acc,root))
    return quotient,remainder

def candidate_support(poly, domain, f):
    remaining=list(poly); roots=[]
    for root in domain:
        while len(remaining)>1:
            quotient,remainder=divide(remaining,root,f)
            if remainder: break
            roots.append(root); remaining=quotient
    if len(remaining)!=1: return []
    assert len(roots)==3
    return sorted(set(itertools.permutations(roots)))

def oracle_cubic(triple, oracle):
    a,b,c=triple
    s=oracle.add(oracle.add(a,b),c)
    t=oracle.add(oracle.add(oracle.mul(a,b),oracle.mul(a,c)),oracle.mul(b,c))
    u=oracle.mul(oracle.mul(a,b),c)
    return (1,oracle.neg(s),t,oracle.neg(u))

def run_field(p,c):
    f=Field(p,c); oracle=TableOracle(p,c)
    check(all((x*x+c)%p for x in range(p)), 'irreducible quadratic over prime field')
    for a,b in itertools.product(range(f.q),repeat=2):
        check(f.mul(a,b)==oracle.mul(a,b), 'independent multiplication')
        check(f.add(a,b)==oracle.add(a,b), 'independent addition')
        m=f.matrix(a); x=f.coord(b)
        check([(sum(m[i][j]*x[j] for j in range(2))%p) for i in range(2)]==f.coord(oracle.mul(a,b)), 'matrix coordinate action')
    aa=[(2*i+1)%f.q for i in range(5)]; bb=[(3*i+2)%f.q for i in range(5)]
    primal,dual=bases(f,aa,bb)
    for rows,power,member in [(primal,4,primal_member),(dual,10,dual_member)]:
        check(abs(determinant(rows))==p**power,'original lattice index')
        check(all(member(row,f,oracle,aa,bb) for row in rows),'original row membership')
        work=[r[:] for r in rows]
        for step in range(32):
            i,j=step%14,(step*3+1)%14
            if i==j: j=(j+1)%14
            if step%4==0: work[i],work[j]=work[j],work[i]
            else: work[i]=[x+(1 if step%2 else -1)*y for x,y in zip(work[i],work[j])]
            check(abs(determinant(work))==p**power and all(member(row,f,oracle,aa,bb) for row in work), 'unimodular basis equality')
            # Increasing then decreasing positive diagonal metrics, exactly inverted.
            for weights in [[1+(j%4) for j in range(14)],[4-(j%4) for j in range(14)]]:
                scaled=[[x*w for x,w in zip(row,weights)] for row in work]
                unscaled=[[x//w for x,w in zip(row,weights)] for row in scaled]
                check(unscaled==work,'metric transport exact; no LLL reducer tested')
        corrupt=[r[:] for r in rows]; corrupt[0]=[2*x for x in corrupt[0]]
        check(all(member(row,f,oracle,aa,bb) for row in corrupt) and abs(determinant(corrupt))!=p**power, 'membership alone cannot certify equal index')
    # Direct affine identity: all 3^5 feature assignments, one declared functional.
    domain=[0,1,p]; affine_checks=0
    gamma=[1,2%p]; delta=[2%p,1]; lam=[]
    for a,b in zip(aa,bb):
        lam.extend([(sum(gamma[j]*f.coord(oracle.mul(a,unit))[j]+delta[j]*f.coord(oracle.mul(b,unit))[j] for j in range(2))%p) for unit in [1,p]])
    a0,b0=p,1; constant=sum(x*y for x,y in zip(gamma,f.coord(a0)))+sum(x*y for x,y in zip(delta,f.coord(b0)))
    for features in itertools.product(domain,repeat=5):
        t,u=a0,b0
        for a,b,x in zip(aa,bb,features): t=oracle.add(t,oracle.mul(a,x)); u=oracle.add(u,oracle.mul(b,x))
        value=constant+sum(lam[2*i+j]*f.coord(x)[j] for i,x in enumerate(features) for j in range(2))-sum(gamma[j]*f.coord(t)[j]+delta[j]*f.coord(u)[j] for j in range(2))
        check(value%p==0,'affine dual annihilation including constants'); affine_checks+=1
    # Independent oracle enumerates exactly 27 original ordered triples once.
    expected={}
    for triple in itertools.product(domain,repeat=3): expected.setdefault(oracle_cubic(triple,oracle),[]).append(triple)
    outputs=[]; supported=0; repeated=0
    for coefficients in itertools.product(range(f.q),repeat=3):
        poly=(1,*coefficients); actual=candidate_support(poly,domain,f); wanted=sorted(expected.get(poly,[]))
        check(actual==wanted,'complete ordered cubic support oracle')
        if wanted:
            supported+=1; repeated+=int(any(len(set(t))<3 for t in wanted))
        outputs.append([list(poly),[list(t) for t in actual]])
    # Wrong-transpose counterexample is concrete and field-specific.
    a=p; matrix=f.matrix(a); gamma=[1,0]
    wrong=[sum(matrix[i][j]*gamma[j] for j in range(2))%p for i in range(2)]
    right=[sum(matrix[j][i]*gamma[j] for j in range(2))%p for i in range(2)]
    check(wrong!=right,'wrong transpose rejected by explicit functional witness')
    cache=dict(p=p,degree=2,modulus=[c,0,1],basis=['1','theta'],coefficients=[aa,bb],offset=[a0,b0],domain=domain)
    def key(x): return hashlib.sha256(json.dumps(x,sort_keys=True,separators=(',',':')).encode()).hexdigest()
    altered=dict(cache,basis=['theta','1']); changed=dict(cache,modulus=[(c+1)%p,0,1])
    check(key(cache)!=key(altered) and key(cache)!=key(changed),'field representation key separation')
    return dict(p=p,degree=2,q=f.q,modulus=[c,0,1],basis=['1','theta'],domain=domain,monic_cubics=f.q**3,supported_polynomials=supported,repeated_supported_polynomials=repeated,ordered_source_triples=27,affine_feature_tuples=affine_checks,basis_transformations=64,wrong_transpose=dict(a=a,gamma=gamma,wrong=wrong,right=right),cache_key=key(cache),root_sets=outputs)

def main():
    ident=sys.argv[1]; rows=[]; status='passed'; error=None
    try:
        for p,c in [(3,1),(5,2)]: rows.append(run_field(p,c))
        # Separate characteristic-two polynomial arithmetic: theta^2+theta+1.
        # The odd-field helper above only represents theta^2+c and is not used.
        product=0
        for i in range(2):
            if (2>>i)&1: product ^= 2<<i
        for degree in range(2,-1,-1):
            if degree>=2 and (product>>degree)&1: product ^= 0b111<<(degree-2)
        check(product==3 and 2*2%4==0,'GF4 scalar integer encoding is not a ring encoding')
    except (AssertionError,ValueError,RuntimeError,ZeroDivisionError) as exc:
        status='inconclusive'; error=str(exc)
    raw=dict(schema='extension-coordinate-oracle/v1',fields=rows,checks=CHECKS,error=error)
    serialized_raw=json.dumps(raw,sort_keys=True,separators=(',',':')).encode()
    blob=gzip.compress(serialized_raw,mtime=0)
    with (HERE/'raw-oracle.json.gz').open('xb') as stream: stream.write(blob)
    evidence=dict(field_summaries=[{k:v for k,v in r.items() if k!='root_sets'} for r in rows],checks=CHECKS,check_cap=200000,basis_transformations=sum(r['basis_transformations'] for r in rows),basis_transform_cap=128,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,complete_coordinate_and_tiny_support_oracle=status=='passed',llL_metric_reduction_tested=False,elliptic_chart_coverage=False,complete_global_root_search=False,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False,error=error)
    result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary='Exact coordinate-field and ordered tiny-domain cubic oracle; no elliptic chart or comparative performance claim.',evidence=evidence,next_actions=['Prove the odd-extension guarded chart and domain coordinate envelopes; benchmark reduction only after complete conditional root contracts are supplied.'])
    text=json.dumps(result,sort_keys=True,separators=(',',':'))
    if time.monotonic()-START>=84:
        result['status']='inconclusive'; evidence['complete_coordinate_and_tiny_support_oracle']=False; evidence['error']='final serialization deadline'; text=json.dumps(result)
    assert time.monotonic()-START<88,'full report acceptance deadline'
    print(text)

if __name__=='__main__': main()
