"""Frozen characteristic-two support/trace counterexample component."""
from pathlib import Path
import sys, json, gzip, hashlib, itertools, time, resource

HERE=Path(__file__).resolve().parent
START=time.monotonic(); CHECKS=0
FIELDS=[(2,0b111),(3,0b1011)]

def check(value,why):
    global CHECKS
    CHECKS+=1
    assert CHECKS<=200000,'predicate cap'
    assert time.monotonic()-START<80,'internal deadline'
    assert value,why

def mul(a,b,n,modulus):
    out=0
    while b:
        if b&1: out^=a
        b>>=1; a<<=1
        if a&(1<<n): a^=modulus
    return out

def oracle_table(n,modulus):
    # Separate coefficient-array convolution/reduction, not shift multiplication.
    q=1<<n; table=[]
    for a in range(q):
        row=[]
        for b in range(q):
            coeff=[0]*(2*n-1)
            for i in range(n):
                for j in range(n): coeff[i+j]^=((a>>i)&1)*((b>>j)&1)
            for degree in range(2*n-2,n-1,-1):
                if coeff[degree]:
                    for j in range(n+1): coeff[degree-n+j]^=(modulus>>j)&1
            row.append(sum(coeff[i]<<i for i in range(n)))
        table.append(row)
    return table

def product_h(roots,table):
    out=[1]
    for root in roots:
        nxt=[0]*(len(out)+1)
        for i,c in enumerate(out): nxt[i]^=table[c][root];nxt[i+1]^=c
        out=nxt
    return tuple(out)

def reduction(poly,H,n,modulus):
    out=list(poly)+[0]*max(0,5-len(poly))
    for degree in range(len(out)-1,4,-1):
        lead=out[degree]
        if lead:
            for j in range(5): out[degree-5+j]^=mul(lead,H[j],n,modulus)
    return out[:5]

def ring_product(a,b,H,n,modulus):
    out=[0]*9
    for i,x in enumerate(a):
        for j,y in enumerate(b):
            if x and y: out[i+j]^=mul(x,y,n,modulus)
    return reduction(out,H,n,modulus)

def multiplication_matrix(r,H,n,modulus):
    cols=[reduction([0]*i+list(r),H,n,modulus)for i in range(5)]
    return [[cols[j][i]for j in range(5)]for i in range(5)]

def matrix_product(a,b,table):
    out=[[0]*5 for _ in range(5)]
    for i in range(5):
        for k in range(5):
            if a[i][k]:
                for j in range(5):out[i][j]^=table[a[i][k]][b[k][j]]
    return out

def trace(a):
    out=0
    for i in range(5):out^=a[i][i]
    return out

# Characteristic polynomial coefficients from principal minors. In characteristic
# two every determinant sign is +, so this is distinct from quotient squaring.
MINORS=[[(indices,perm)for indices in itertools.combinations(range(5),k)
          for perm in itertools.permutations(indices)]for k in range(1,6)]
def charpoly(a,table):
    elementary=[]
    for terms in MINORS:
        value=0
        for indices,perm in terms:
            term=1
            for i,j in zip(indices,perm):term=table[term][a[i][j]]
            value^=term
        elementary.append(value)
    return elementary

def field_power(a,power,table):
    out=1
    while power:
        if power&1:out=table[out][a]
        a=table[a][a];power>>=1
    return out

class LazyRow:
    def __init__(self,a,n,modulus):self.a,self.n,self.modulus=a,n,modulus
    def __getitem__(self,b):return mul(self.a,b,self.n,self.modulus)
class LazyProducts:
    def __init__(self,n,modulus):self.n,self.modulus=n,modulus
    def __getitem__(self,a):return LazyRow(a,self.n,self.modulus)

def evaluate_linearized(coeffs,x,n,modulus):
    result=0;power=x
    for coefficient in coeffs:
        result^=mul(coefficient,power,n,modulus)
        power=mul(power,power,n,modulus)
    return result

def actual83_control():
    n=83;modulus=(1<<83)|(1<<45)|(1<<2)|(1<<1)|1
    alpha=2;H=(0,alpha,0,0,0,1);a=alpha
    for _ in range(81):a=mul(a,a,n,modulus)
    check(mul(mul(a,a,n,modulus),mul(a,a,n,modulus),n,modulus)==alpha,'supplied factor a fourth equals alpha')
    check(a>>18!=0,'a outside low18 source span')
    # L_0=X; L_(i+1)=L_i^2+L_i(b_i)*L_i for b_i=alpha^i.
    coeffs=[1];r=[0,1,0,0,0];betas=[];ra=a
    for index in range(18):
        beta=evaluate_linearized(coeffs,1<<index,n,modulus)
        check(beta!=0,'independent next source basis element')
        betas.append(beta)
        square=ring_product(r,r,H,n,modulus)
        r=[x^mul(beta,y,n,modulus)for x,y in zip(square,r)]
        nxt=[0]*(len(coeffs)+1)
        for j,coefficient in enumerate(coeffs):
            nxt[j]^=mul(beta,coefficient,n,modulus)
            nxt[j+1]^=mul(coefficient,coefficient,n,modulus)
        coeffs=nxt
        ra=mul(ra,ra,n,modulus)^mul(beta,ra,n,modulus)
    check(evaluate_linearized(coeffs,a,n,modulus)==ra and ra!=0,'unsupported repeated factor linearized value')
    r8=r
    for _ in range(3):r8=ring_product(r8,r8,H,n,modulus)
    check(any(r8),'actual83 nilpotent support rejects supplied H')
    table=LazyProducts(n,modulus);matrix=multiplication_matrix(r,H,n,modulus)
    e=charpoly(matrix,table);m2=matrix_product(matrix,matrix,table)
    m3=matrix_product(m2,matrix,table);m5=matrix_product(m2,m3,table)
    odd=[trace(matrix),trace(m3),trace(m5)]
    ra4=mul(mul(ra,ra,n,modulus),mul(ra,ra,n,modulus),n,modulus)
    check(e==[0,0,0,ra4,0]and not any(odd),'actual83 oddtrace false positive and exact expected characteristic polynomial')
    return dict(degree=n,modulus=modulus,domain='span_F2(1,alpha,...,alpha^17)',
                H=list(H),factor_root=a,factor_multiplicity=4,zero_root_multiplicity=1,
                first_four_coefficient_degree_bounds_pass=True,polynomial_lift_q=0,
                actual_73_parameter_norm_family_membership_claim=False,
                linearized_coefficients=coeffs,annihilator_basis_steps=18,
                linearized_value_at_factor=ra,R8=r8,charpoly=e,odd_traces=odd,
                exact_support=False,odd_shortcut_false_positive=True,
                enumeration_of_2_to_18_domain=False,full_field_setup_reverified=False)

def cases(n,table):
    q=1<<n
    if n==2:return [tuple(v)+(1,)for v in itertools.product(range(q),repeat=5)]
    # Fixed source-independent coefficient rule, no favorable-case selection.
    result=set()
    for index in range(64):
        digest=hashlib.sha256(('binary-trace-gf8-v1|'+str(index)).encode()).digest()
        result.add(tuple(v%q for v in digest[:5])+(1,))
    # Explicit supported and repeated-root controls supplement the 64 samples.
    for domain in [[0],[0,1]]:
        result.update(product_h(t,table)for t in itertools.product(domain,repeat=5))
    result.add(product_h([0,1,1,2,2],table))
    return sorted(result)

def run_field(n,modulus):
    q=1<<n;table=oracle_table(n,modulus)
    for a,b in itertools.product(range(q),repeat=2):check(mul(a,b,n,modulus)==table[a][b],'independent field multiplication')
    check(all(field_power(a,q-1,table)==1 for a in range(1,q)),'finite field nonzero elements')
    rows=[];summaries=[];hs=cases(n,table)
    for domain in [[0],[0,1]]:
        wanted={}
        for roots in itertools.product(domain,repeat=5):wanted.setdefault(product_h(roots,table),[]).append(list(roots))
        accepted=odd_false=trace1_auto=four_checked=0
        for H in hs:
            # L_{0}=X and L_{0,1}=X^2+X are squarefree linearized polynomials.
            r=[0,1,0,0,0]if len(domain)==1 else[0,1,1,0,0]
            r8=r
            for _ in range(3):r8=ring_product(r8,r8,H,n,modulus)
            support=not any(r8);expected=H in wanted
            check(support==expected,'full repeated-root support versus original ordered roots')
            matrix=multiplication_matrix(r,H,n,modulus);e=charpoly(matrix,table)
            m2=matrix_product(matrix,matrix,table);m3=matrix_product(m2,matrix,table)
            m4=matrix_product(m2,m2,table);m5=matrix_product(m4,matrix,table)
            s1,s2,s3,s4,s5=[trace(x)for x in [matrix,m2,m3,m4,m5]]
            check(s1==e[0]and s2==table[s1][s1]and s4==table[s2][s2],'even Frobenius traces')
            check(s3==(table[e[0]][s2]^table[e[1]][s1]^e[2]),'Newton identity degree three')
            check(s5==(table[e[0]][s4]^table[e[1]][s3]^table[e[2]][s2]^table[e[3]][s1]^e[4]),'Newton identity degree five')
            check((not any(e))==support,'full characteristic polynomial nilpotence')
            odd=(s1==s3==s5==0)
            check(not odd or e[0]==e[2]==e[4]==0,'odd trace relaxation parity coefficients')
            if s1==0:
                four_checked+=1
                check((e[1]==s3==e[3]==s5==0)==support,'four exact residuals when trace1 zero')
            if H[4]in domain:
                trace1_auto+=1;check(s1==0,'trace L_D(X) equals L_D(first coefficient)')
            accepted+=support;odd_false+=odd and not support
            rows.append(dict(H=list(H),domain=domain,R8=r8,charpoly=e,odd_traces=[s1,s3,s5],
                             exact_support=support,original_ordered_roots=wanted.get(H,[]),
                             odd_shortcut_false_positive=odd and not support))
        summaries.append(dict(domain=domain,quintics=len(hs),supported_quintics=accepted,
                              original_source_tuples=sum(len(v)for h,v in wanted.items()if h in hs),
                              odd_shortcut_false_positives=odd_false,trace1_automatic_cases=trace1_auto,
                              four_residual_cases=four_checked))
    # This particular supplied repeated-root quintic must refute odd traces.
    H=product_h([0,1,1,2,2],table)
    witness=next(row for row in rows if tuple(row['H'])==H and row['domain']==[0])
    check(witness['odd_shortcut_false_positive']and not witness['exact_support'],'mandatory repeated-root counterexample')
    return dict(degree=n,modulus=modulus,q=q,monic_input_count=len(hs),summaries=summaries,
                counterexample=witness,records=rows)

def main():
    ident=sys.argv[1];fields=[];actual=None;status='passed';error=None
    try:
        for n,modulus in FIELDS:fields.append(run_field(n,modulus))
        actual=actual83_control()
    except (AssertionError,ValueError,RuntimeError,ZeroDivisionError,StopIteration)as exc:
        status='inconclusive';error=str(exc)
    raw=dict(schema='binary-trace-support/v1',fields=fields,actual83=actual,predicates=CHECKS,error=error)
    blob=gzip.compress(json.dumps(raw,sort_keys=True,separators=(',',':')).encode(),mtime=0)
    with(HERE/'raw-controls.json.gz').open('xb')as f:f.write(blob)
    evidence=dict(field_summaries=[{k:v for k,v in row.items()if k!='records'}for row in fields],
                  predicates=CHECKS,predicate_cap=200000,raw_sha256=hashlib.sha256(blob).hexdigest(),
                  raw_bytes=len(blob),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                  memory_metric='individual process self RSS KiB',complete_support_component=status=='passed',
                  odd_trace_shortcut_claim=False,actual_GF2_83_executed=actual is not None,
                  actual83_supplied_counterexample=actual,novelty_claim=False,
                  complete_global_root_search=False,original_root_count_exact=None,
                  qualified_speedup=None,release_gate_claim=False,error=error)
    result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,
                summary='Exact binary support versus odd-trace relaxation; repeated-root counterexamples retained.',
                evidence=evidence,next_actions=['Use the exact four residuals only after proving trace1 zero; test symbolic whole-cell pruning on a fresh fixed binary family, not independent odd-trace feasibility. No runtime advantage of characteristic-polynomial extraction is established.'])
    text=json.dumps(result,sort_keys=True,separators=(',',':'))
    if time.monotonic()-START>=84:
        result['status']='inconclusive';evidence['complete_support_component']=False
        evidence['error']='final serialized report deadline';text=json.dumps(result)
    assert time.monotonic()-START<88,'report deadline'
    print(text)

if __name__=='__main__':main()
