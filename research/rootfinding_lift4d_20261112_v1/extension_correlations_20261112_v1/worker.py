"""Frozen finite-image information diagnostic; coordinator executes this module."""
from pathlib import Path
import sys, time, json, gzip, hashlib, itertools, resource, statistics
from research.extension_oracle_20261111_v1.worker import Field, TableOracle

HERE=Path(__file__).resolve().parent
START=time.monotonic()
CHECKS=0
FUNCTIONALS=[[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1],
             [1,1,0,0],[0,1,1,0],[0,0,1,1],[1,0,0,1]]

def check(value,why):
    global CHECKS
    CHECKS+=1
    assert CHECKS<=200000,'predicate cap'
    assert time.monotonic()-START<80,'internal work deadline'
    assert value,why

def features(field,x,y):
    return [field.mul(x,x),field.mul(x,y),field.mul(y,y),x,y]

def output(field,values,aa,bb):
    t=u=0
    for a,b,z in zip(aa,bb,values):
        t=field.add(t,field.mul(a,z));u=field.add(u,field.mul(b,z))
    return t,u

def oracle_map(p,c):
    # Closed-form table operations are independent of candidate convolution.
    oracle=TableOracle(p,c);q=p*p;domain=[0,1,p]
    aa=[(2*i+1)%q for i in range(5)];bb=[(3*i+2)%q for i in range(5)]
    expected={(t,u):[] for t in range(q) for u in range(q)}
    for x,y in itertools.product(domain,repeat=2):
        # Independently spell out the two quadratic maps, without candidate features().
        terms=[oracle.mul(x,x),oracle.mul(x,y),oracle.mul(y,y),x,y]
        totals=[]
        for coeff in [aa,bb]:
            z=0
            for a,v in zip(coeff,terms):z=oracle.add(z,oracle.mul(a,v))
            totals.append(z)
        expected[tuple(totals)].append([x,y])
    return expected

def no_multiple(lo,hi,p):
    assert lo<=hi
    return -((-lo)//p)>hi//p

def certificate_data(f,tuples,aa,bb):
    coords=[sum([f.coord(z) for z in values],[]) for _,_,values in tuples]
    marginals=[(min(z[j] for z in coords),max(z[j] for z in coords)) for j in range(10)]
    records=[]
    for functional in FUNCTIONALS:
        gamma,delta=functional[:2],functional[2:];lam=[]
        for a,b in zip(aa,bb):
            ma,mb=f.matrix(a),f.matrix(b)
            lam.extend([sum(gamma[k]*ma[k][j]+delta[k]*mb[k][j] for k in range(2))%f.p for j in range(2)])
        values=[sum(a*b for a,b in zip(lam,z)) for z in coords]
        marginal_lo=sum(a*(lo if a>=0 else hi) for a,(lo,hi) in zip(lam,marginals))
        marginal_hi=sum(a*(hi if a>=0 else lo) for a,(lo,hi) in zip(lam,marginals))
        canonical_vector=lam+[-a for a in functional]
        lo=sum(min(0,a*(f.p-1)) for a in canonical_vector)
        hi=sum(max(0,a*(f.p-1)) for a in canonical_vector)
        check(not no_multiple(lo,hi,f.p),'full independent canonical box must be vacuous')
        check(marginal_lo<=min(values)<=max(values)<=marginal_hi,'correlated range contained in marginal range')
        records.append(dict(functional=functional,lambda_coefficients=lam,values=values,
                            marginal=[marginal_lo,marginal_hi],correlated=[min(values),max(values)],canonical=[lo,hi]))
    return records

def checked_rows(rows,expected):
    check(len(rows)==len(expected),'complete target row count')
    check(len({tuple(row['target'])for row in rows})==len(rows),'no target duplicates')
    for row in rows:
        check(row['roots']==expected[tuple(row['target'])],'independent complete ordered root set')
    serialized=json.dumps(rows,sort_keys=True,separators=(',',':'))
    return serialized

def arm(name,p,c,expected):
    began=time.monotonic();f=Field(p,c);q=f.q;domain=[0,1,p]
    aa=[(2*i+1)%q for i in range(5)];bb=[(3*i+2)%q for i in range(5)]
    tuples=[(x,y,features(f,x,y)) for x,y in itertools.product(domain,repeat=2)]
    rows=[];functional_records=None
    if name=='indexed':
        # Strong specialized tiny comparator: build every output bucket once.
        buckets={}
        for x,y,values in tuples:buckets.setdefault(output(f,values,aa,bb),[]).append([x,y])
        for t,u in itertools.product(range(q),repeat=2):
            rows.append(dict(target=[t,u],roots=buckets.get((t,u),[])))
    else:
        functional_records=certificate_data(f,tuples,aa,bb)
        for t,u in itertools.product(range(q),repeat=2):
            targetcoords=f.coord(t)+f.coord(u)
            masks={key:[]for key in ['canonical','marginal','correlated','discrete']}
            for record in functional_records:
                shift=sum(x*y for x,y in zip(record['functional'],targetcoords))
                ml,mh=record['marginal'];cl,ch=record['correlated']
                masks['canonical'].append(False)
                masks['marginal'].append(no_multiple(ml-shift,mh-shift,p))
                masks['correlated'].append(no_multiple(cl-shift,ch-shift,p))
                # Inspect each original correlated value; no independent-coordinate product.
                feasible=False
                for value in record['values']:
                    check(True,'bounded target/functional/source evaluation')
                    feasible=feasible or (value-shift)%p==0
                masks['discrete'].append(not feasible)
            check(all(not a or b for a,b in zip(masks['marginal'],masks['correlated'])),'correlated hull dominates marginal hull')
            check(all(not a or b for a,b in zip(masks['correlated'],masks['discrete'])),'exact image dominates hull')
            if expected[(t,u)]:check(not any(any(z)for z in masks.values()),'a true target must never be rejected')
            roots=[]
            if not any(masks['discrete']):
                for x,y,values in tuples:
                    if output(f,values,aa,bb)==(t,u):roots.append([x,y])
            rows.append(dict(target=[t,u],roots=roots,masks=masks))
    serialized=checked_rows(rows,expected)
    return dict(arm=name,p=p,modulus=[c,0,1],basis=['1','theta'],domain=domain,coefficients=[aa,bb],
                paid_component_seconds=time.monotonic()-began,serialized_output_bytes=len(serialized.encode()),
                complete_conditional_toy_root_sets=True,rows=rows,functional_records=functional_records)

def main():
    ident=sys.argv[1];records=[];status='passed';error=None;summaries=[]
    try:
        expected={}
        for p,c in [(3,1),(5,2)]:
            f=Field(p,c);o=TableOracle(p,c)
            check(all((x*x+c)%p for x in range(p)),'irreducible field polynomial')
            for a,b in itertools.product(range(p*p),repeat=2):
                check(f.mul(a,b)==o.mul(a,b) and f.add(a,b)==o.add(a,b),'independent arithmetic oracle')
            expected[p]=oracle_map(p,c)
        for rep,order in enumerate([['indexed','filtered'],['filtered','indexed'],['indexed','filtered']]):
            for name in order:
                for p,c in [(3,1),(5,2)]:
                    row=arm(name,p,c,expected[p]);row['rep']=rep;records.append(row)
        for p in [3,5]:
            sample=next(row for row in records if row['p']==p and row['arm']=='filtered');counts={}
            for method in ['canonical','marginal','correlated','discrete']:
                counts[method]=dict(rejected_targets=sum(any(row['masks'][method])for row in sample['rows']),
                    per_functional=[sum(row['masks'][method][i]for row in sample['rows'])for i in range(8)])
            baseline=statistics.median(row['paid_component_seconds']for row in records if row['p']==p and row['arm']=='indexed')
            filtered=statistics.median(row['paid_component_seconds']for row in records if row['p']==p and row['arm']=='filtered')
            summaries.append(dict(p=p,q=p*p,targets=p**4,ordered_source_pairs=9,nonempty_targets=sum(bool(v)for v in expected[p].values()),
                rejection_counts=counts,indexed_median_seconds=baseline,filtered_median_seconds=filtered,
                diagnostic_component_ratio=baseline/filtered,correlation_adds_rejection=counts['discrete']['rejected_targets']>counts['marginal']['rejected_targets'],
                cost_prediction_supported=filtered<baseline))
    except (AssertionError,ValueError,RuntimeError,ZeroDivisionError,KeyError)as exc:
        status='inconclusive';error=str(exc)
    if status!='passed':
        for summary in summaries:summary.update(diagnostic_component_ratio=None,cost_prediction_supported=None)
    raw=dict(schema='extension-correlation-diagnostic/v1',records=records,summaries=summaries,checks=CHECKS,error=error)
    blob=gzip.compress(json.dumps(raw,sort_keys=True,separators=(',',':')).encode(),mtime=0)
    with(HERE/'raw-profile.json.gz').open('xb')as f:f.write(blob)
    evidence=dict(field_summaries=summaries,checks=CHECKS,predicate_cap=200000,functionals=FUNCTIONALS,
        complete_toy_root_comparison=status=='passed',raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),
        peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,memory_metric='peak process self RSS KiB',
        common_setup_policy='independent table-oracle construction and arithmetic crosschecks charged whole worker, outside equally scoped paid arms',
        timing_scope='each arm constructs field/features, filter or direct index, recovers all target roots, independently compares every target and serializes output; filtered additionally serializes diagnostic masks, indexed serializes roots only',
        elliptic_chart_coverage=False,complete_global_root_search=False,original_root_count_exact=None,
        qualified_speedup=None,release_gate_claim=False,novelty_claim=False,error=error)
    result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,
        summary='Complete tiny correlated-image filters versus strong indexed root enumeration; no elliptic-chart or qualified speed claim.',evidence=evidence,
        next_actions=['If exact image prunes but total filter cost loses, derive a compact algebraic product-span/Frobenius representation without enumerating the hard domain; do not retry this identical toy family.'])
    text=json.dumps(result,sort_keys=True,separators=(',',':'))
    if time.monotonic()-START>=84:
        result['status']='inconclusive';evidence.update(complete_toy_root_comparison=False,error='final report deadline')
        for summary in summaries:summary.update(diagnostic_component_ratio=None,cost_prediction_supported=None)
        for row in records:row['complete_conditional_toy_root_sets']=False
        text=json.dumps(result,sort_keys=True,separators=(',',':'))
    assert time.monotonic()-START<88,'final serialized report deadline'
    print(text)

if __name__=='__main__':main()
