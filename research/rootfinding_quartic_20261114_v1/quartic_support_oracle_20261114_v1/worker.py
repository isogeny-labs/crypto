"""Exact quartic-stratum oracle; only the coordinator runs this frozen worker."""
from pathlib import Path
import itertools, json, gzip, hashlib, resource, sys, time
HERE=Path(__file__).resolve().parent
START=time.monotonic(); CHECKS=0

def check(value, why):
    global CHECKS
    CHECKS+=1
    assert CHECKS<=200000, 'predicate cap'
    assert time.monotonic()-START<80, 'work deadline'
    assert value, why

def mul(a,b,n,mod):
    out=0
    while b:
        if b&1:out^=a
        b>>=1;a<<=1
        if a>>n:a^=mod
    return out

def fourth(a,n,mod):
    a=mul(a,a,n,mod);return mul(a,a,n,mod)

def tables(n,mod):
    # Independent coefficient-array convolution and polynomial reduction.
    q=1<<n;out=[]
    for a in range(q):
        row=[]
        for b in range(q):
            v=[0]*(2*n-1)
            for i in range(n):
                for j in range(n):v[i+j]^=((a>>i)&1)*((b>>j)&1)
            for k in range(2*n-2,n-1,-1):
                if v[k]:
                    for j in range(n+1):v[k-n+j]^=(mod>>j)&1
            row.append(sum(v[i]<<i for i in range(n)))
        out.append(row)
    return out

def span(basis):
    values=[0]
    for b in basis:values += [x^b for x in values]
    return sorted(set(values))

def in_span(value,basis):
    pivots={}
    for b in basis:
        while b:
            bit=b.bit_length()-1
            if bit in pivots:b^=pivots[bit]
            else:pivots[bit]=b;break
    while value:
        bit=value.bit_length()-1
        if bit not in pivots:return False
        value^=pivots[bit]
    return True

def product_h(roots,tab):
    h=[1]
    for root in roots:
        out=[0]*(len(h)+1)
        for i,c in enumerate(h):out[i]^=tab[c][root];out[i+1]^=c
        h=out
    return h

def inverse_fourth(c,n,mod):
    # Frobenius squared has period n; works also n=2.
    for _ in range((-2)%n):c=mul(c,c,n,mod)
    return c

def quartic_tuples(a):
    return sorted(set(itertools.permutations([0,a,a,a,a])))

def small_fields():
    records=[];bags=0
    for n,mod in [(2,7),(3,11),(4,19)]:
        q=1<<n;tab=tables(n,mod)
        for a,b in itertools.product(range(q),repeat=2):check(mul(a,b,n,mod)==tab[a][b],'independent arithmetic')
        check(all(any(tab[a][b]==1 for b in range(1,q))for a in range(1,q)),'small quotient is a field')
        bases=[[1<<j for j in range(d)]for d in range(n+1)]
        bases += [[2],[3,1<<(n-1)]]
        for bi,basis in enumerate(bases):
            domain=span(basis);check(len(domain)==1<<len(basis),'independent source basis')
            image=[fourth(v,n,mod)for v in basis];expected={}
            for roots in itertools.combinations_with_replacement(domain,5):
                h=product_h(roots,tab);bags+=1
                check(h[-1]==1 and len(h)==6,'monic oracle product')
                if h[0]==h[2]==h[3]==h[4]==0:
                    expected.setdefault(h[1],[]).extend(sorted(set(itertools.permutations(roots))))
            rows=[]
            for c in range(q):
                a=inverse_fourth(c,n,mod);supported=in_span(c,image)
                actual=quartic_tuples(a)if supported else[]
                wanted=sorted(expected.get(c,[]))
                check(fourth(a,n,mod)==c and supported==(a in domain),'inverse and image criterion')
                check(actual==wanted and len(actual)==len(set(actual)),'complete ordered original source tuples')
                rows.append(dict(c=c,fourth_root=a,supported=supported,ordered_sources=[list(t)for t in actual]))
            records.append(dict(n=n,modulus=mod,basis=basis,domain=domain,image_basis=image,rows=rows,
                                supported_coefficients=len(expected),multiset_oracle=True))
    return records,bags

def parity(x):return x.bit_count()&1

def eliminate(rows,nvars):
    # Augmented row bits, rhs at nvars; carry full original-row combinations.
    work=[[mask|(rhs<<nvars),1<<i]for i,(mask,rhs)in enumerate(rows)]
    rank=0;pivots=[]
    for col in range(nvars):
        pivot=next((j for j in range(rank,len(work))if (work[j][0]>>col)&1),None)
        if pivot is None:continue
        work[rank],work[pivot]=work[pivot],work[rank]
        for j in range(len(work)):
            if j!=rank and (work[j][0]>>col)&1:
                work[j][0]^=work[rank][0];work[j][1]^=work[rank][1]
        pivots.append(col);rank+=1
    for row,combination in work:
        if row==(1<<nvars):return dict(consistent=False,contradiction=combination,rank=rank)
    particular=sum(((work[i][0]>>nvars)&1)<<col for i,col in enumerate(pivots))
    free=[j for j in range(nvars)if j not in pivots];kernel=[]
    for j in free:
        v=1<<j
        for i,col in enumerate(pivots):v|=((work[i][0]>>j)&1)<<col
        kernel.append(v)
    return dict(consistent=True,particular=particular,kernel=kernel,rank=rank)

def affine_controls():
    records=[];n=3;mod=11;V=[1,2];image=span([fourth(v,n,mod)for v in V])
    for case in range(35):
        if case<32:
            k=case%9
            digest=hashlib.sha256(('quartic-affine-oracle-v1|'+str(case)).encode()).digest()
            offset=int.from_bytes(digest[:2],'little')&((1<<15)-1)
            columns=[int.from_bytes(hashlib.sha256(digest+bytes([j])).digest()[:2],'little')&((1<<15)-1)for j in range(k)]
        else:
            # Explicit packed-map consistency, image freedom and coefficient-map kernel.
            offset,columns=[(0,[0]),(1<<9,[1,6<<9,0]),(1,[1])][case-32]
            k=len(columns)
        # H coordinates(e1,e2,e3,e4,e5), each3bits. Image e4 is two-dimensional.
        allowed=[c<<(3*3)for c in image]
        checks=[1<<i for i in range(15)if i//3!=3]
        # One orthogonal equation for e4, found independently by exhaustive dual search.
        checks += [w<<9 for w in range(1,8)if all(parity(w&c)==0 for c in image)]
        rows=[(sum(parity(w&column)<<j for j,column in enumerate(columns)),parity(w&offset))for w in checks]
        result=eliminate(rows,k);expected=[]
        for t in range(1<<k):
            coeff=offset
            for j,column in enumerate(columns):
                if(t>>j)&1:coeff^=column
            if coeff in allowed:expected.append(t)
        if case>=32:check(result['consistent'],'explicit packed coefficient map is consistent')
        if result['consistent']:
            actual=sorted(result['particular']^v for v in span(result['kernel']))
            check(actual==expected and len(actual)==1<<len(result['kernel']),'complete affine parameter solution set')
            for t in actual:check(all(parity(mask&t)==rhs for mask,rhs in rows),'particular/kernel equations')
        else:
            combination=result['contradiction'];mask=rhs=0
            for i,(m,b)in enumerate(rows):
                if(combination>>i)&1:mask^=m;rhs^=b
            check(mask==0 and rhs==1 and not expected,'independent contradiction combination')
        records.append(dict(case=case,nvars=k,offset=offset,columns=columns,equations=rows,result=result,
                            direct_parameter_solutions=expected,parameter_count_is_not_root_count=True))
    # Deliberate consistent kernel case and inconsistent cases exercise both contracts.
    for rows,k in [([(0,1)],0), ([(1,0)],2), ([(1,0),(1,1)],1), ([],3)]:
        result=eliminate(rows,k);wanted=[t for t in range(1<<k)if all(parity(m&t)==b for m,b in rows)]
        check(bool(wanted)==result['consistent'],'explicit elimination branch')
        if result['consistent']:check(sorted(result['particular']^v for v in span(result['kernel']))==wanted,'explicit full nullspace')
        else:
            m=b=0
            for i,(x,y)in enumerate(rows):
                if(result['contradiction']>>i)&1:m^=x;b^=y
            check(m==0 and b==1,'explicit contradiction')
        records.append(dict(nvars=k,equations=rows,result=result,direct_parameter_solutions=wanted))
    return records

def actual83():
    n=83;mod=(1<<83)|(1<<45)|(1<<2)|(1<<1)|1
    basis=[1<<i for i in range(18)];image=[fourth(v,n,mod)for v in basis]
    check(image==[1<<(4*i)for i in range(18)]and len(set(image))==18,'actual exact unwrapped image columns')
    mask=sum(image);rows=[]
    coefficients=[0]+image+[2]
    for i in range(44):
        bits=int.from_bytes(hashlib.sha256(('quartic83-v1|'+str(i)).encode()).digest()[:3],'little')&((1<<18)-1)
        coefficients.append(sum(1<<(4*j)for j in range(18)if(bits>>j)&1))
    for c in coefficients:
        a=inverse_fourth(c,n,mod);supported=(c&~mask)==0
        check(fourth(a,n,mod)==c and supported==(a>>18==0),'actual support versus inverse Frobenius membership')
        check(in_span(c,image)==supported,'actual matrix versus explicit image mask')
        roots=quartic_tuples(a)if supported else[]
        check((len(roots)==(1 if c==0 else 5))if supported else not roots,'actual multiplicity/order contract')
        rows.append(dict(c=c,fourth_root=a,supported=supported,ordered_sources=[list(v)for v in roots]))
    return dict(n=n,modulus=mod,domain_basis=basis,image_basis=image,rows=rows,
                image_dimension=18,degree68_ambient_dimension=69,stratum_codimension=51,
                ambient83_codimension=65,global73_bit_pruning_claim=False,
                actual_norm_family_intersected=False,full_field_setup_reverified=False)

def main():
    ident=sys.argv[1];small=[];affine=[];actual=None;bags=0;error=None;status='passed'
    try:small,bags=small_fields();affine=affine_controls();actual=actual83()
    except (AssertionError,ValueError,RuntimeError,ZeroDivisionError,KeyError)as exc:status='inconclusive';error=str(exc)
    raw=dict(schema='quartic-support-oracle/v1',small_fields=small,multiset_products=bags,
             affine_controls=affine,actual83=actual,predicates=CHECKS,error=error)
    blob=gzip.compress(json.dumps(raw,sort_keys=True,separators=(',',':')).encode(),mtime=0)
    with(HERE/'raw-controls.json.gz').open('xb')as f:f.write(blob)
    evidence=dict(small_domains=len(small),small_coefficient_checks=sum(len(v['rows'])for v in small),
                  multiset_products=bags,affine_cases=len(affine),actual83=actual,predicates=CHECKS,
                  predicate_cap=200000,complete_support_oracle=status=='passed',
                  raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),
                  peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                  actual_norm_family_intersected=False,original_root_count_exact=None,
                  complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,
                  novelty_claim=False,error=error)
    result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,
                summary='Exact quartic support and independent tiny affine intersection oracle; no global solve claim.',
                evidence=evidence,next_actions=['Admit the literal actual fixed-cell affine norm map before using the image-coordinate conditions; retain coefficient-map kernels and original curve/auxiliary recovery obligations.'])
    text=json.dumps(result,sort_keys=True,separators=(',',':'))
    if time.monotonic()-START>=84:
        result['status']='inconclusive';evidence['complete_support_oracle']=False;evidence['error']='late serialization';text=json.dumps(result)
    assert time.monotonic()-START<88,'final report deadline'
    print(text)

if __name__=='__main__':main()
