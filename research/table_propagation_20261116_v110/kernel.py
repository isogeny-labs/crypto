"""Exact support for one finite-domain XOR relation; no global UNSAT claim."""
from collections import Counter

class WorkCap(Exception):
    pass

class Budget:
    def __init__(self, limit=1000000):
        self.limit, self.used = limit, 0
    def charge(self, n):
        self.used += n
        if self.used > self.limit:
            raise WorkCap('declared tuple/butterfly work cap')

def walsh(values, budget):
    a = list(values)
    step = 1
    while step < len(a):
        budget.charge(len(a)//2)
        for start in range(0,len(a),2*step):
            for j in range(start,start+step):
                x,y = a[j],a[j+step]
                a[j],a[j+step] = x+y,x-y
        step *= 2
    return a

def inverse(values,budget):
    a = walsh(values,budget)
    assert all(x % len(a) == 0 for x in a)
    a = [x//len(a) for x in a]
    assert min(a) >= 0
    return a

def propagate(domains,bits,target,budget):
    """domains[i] contains (distinct token, XOR contribution), not deduplicated images."""
    size = 1 << bits
    assert size <= 4096 and 0 <= target < size
    assert domains and all(len({t for t,v in d}) == len(d) for d in domains)
    hist=[]; transformed=[]
    for domain in domains:
        budget.charge(len(domain))
        h=Counter(v for t,v in domain)
        assert all(0 <= v < size for v in h)
        hist.append(h)
        transformed.append(walsh([h.get(i,0) for i in range(size)],budget))
    prefix=[[1]*size]
    for a in transformed:
        budget.charge(size)
        prefix.append([x*y for x,y in zip(prefix[-1],a)])
    suffix=[[1]*size for _ in range(len(domains)+1)]
    for i in range(len(domains)-1,-1,-1):
        budget.charge(size)
        suffix[i]=[x*y for x,y in zip(transformed[i],suffix[i+1])]
    total=inverse(prefix[-1],budget)[target]
    kept=[]; minima=[]; state_counts=[]; support_histograms=[]
    for i,domain in enumerate(domains):
        budget.charge(size+len(domain))
        others=inverse([x*y for x,y in zip(prefix[i],suffix[i+1])],budget)
        kept.append([(t,v) for t,v in domain if others[target^v] > 0])
        minima.append(min((others[target^v] for t,v in domain),default=0))
        state_counts.append(sum(x>0 for x in others))
        support_histograms.append(dict(sorted(Counter(others[target^v] for t,v in domain).items())))
    return {'kept':kept,'exact_single_relation_tuple_count':total,
            'other_sumset_state_counts':state_counts,'minimum_support_counts':minima,
            'support_count_histograms':support_histograms,'image_cardinalities':[len(h) for h in hist],
            'work_units':budget.used,'single_relation_complete':True,
            'original_complete':False,'original_exact_root_count':None}
